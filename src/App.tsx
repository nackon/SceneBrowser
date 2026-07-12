import { useEffect, useMemo, useState } from 'react';
import { Sidebar } from './components/Sidebar';
import { VideoGrid } from './components/VideoGrid';
import { Settings } from './components/Settings';
import { FilterBar, type FilterMode, type SortField, type SortDirection } from './components/FilterBar';
import { useVideoStore } from './store/videoStore';
import { useVideos } from './hooks/useVideos';
import { checkFFmpeg, getFavoriteCount } from './services/commands';
import './App.css';

type View = 'videos' | 'settings';

const isSortField = (value: string | null): value is SortField =>
  value === 'filename' || value === 'date';

const isSortDirection = (value: string | null): value is SortDirection =>
  value === 'asc' || value === 'desc';

function App() {
  const { videos, selectedFolder, isLoading, error } = useVideoStore();
  const [ffmpegError, setFFmpegError] = useState<string | null>(null);
  const [currentView, setCurrentView] = useState<View>('videos');
  const [filterMode, setFilterMode] = useState<FilterMode>(() => {
    const saved = localStorage.getItem('video_filter_mode');
    return (saved as FilterMode) || 'all';
  });
  const [favoriteCount, setFavoriteCount] = useState(0);
  const [sortField, setSortField] = useState<SortField>(() => {
    const saved = localStorage.getItem('video_sort_field');
    return isSortField(saved) ? saved : 'filename';
  });
  const [sortDirection, setSortDirection] = useState<SortDirection>(() => {
    const saved = localStorage.getItem('video_sort_direction');
    return isSortDirection(saved) ? saved : 'asc';
  });

  // Check FFmpeg availability on startup
  useEffect(() => {
    checkFFmpeg().catch((err) => {
      const errorMsg = typeof err === 'string' ? err : String(err);
      setFFmpegError(errorMsg);
      console.error('FFmpeg check failed:', errorMsg);
    });
  }, []);

  // Fetch videos when selected folder changes
  useVideos(selectedFolder);

  // Update favorite count when folder or videos change
  useEffect(() => {
    if (selectedFolder !== null) {
      getFavoriteCount(selectedFolder)
        .then(setFavoriteCount)
        .catch((err) => console.error('Failed to get favorite count:', err));
    }
  }, [selectedFolder, videos]);

  // Save filter mode to localStorage
  const handleFilterModeChange = (mode: FilterMode) => {
    setFilterMode(mode);
    localStorage.setItem('video_filter_mode', mode);
  };

  // Save sort field to localStorage
  const handleSortFieldChange = (field: SortField) => {
    setSortField(field);
    localStorage.setItem('video_sort_field', field);
  };

  // Toggle and save sort direction to localStorage
  const handleSortDirectionToggle = () => {
    const next = sortDirection === 'asc' ? 'desc' : 'asc';
    setSortDirection(next);
    localStorage.setItem('video_sort_direction', next);
  };

  // Update favorite count when a video's favorite status changes
  const handleFavoriteToggled = () => {
    if (selectedFolder !== null) {
      getFavoriteCount(selectedFolder)
        .then(setFavoriteCount)
        .catch((err) => console.error('Failed to get favorite count:', err));
    }
  };

  // Filter by filter mode, then sort by the selected field/direction.
  // Combined into one memo (rather than a `filteredVideos` intermediate) so the
  // sort only recomputes when its actual inputs change, not on every render.
  const sortedVideos = useMemo(() => {
    const filtered = filterMode === 'favorites'
      ? videos.filter((v) => v.is_favorite === 1)
      : videos;

    const directionMultiplier = sortDirection === 'asc' ? 1 : -1;

    if (sortField === 'filename') {
      return [...filtered].sort(
        (a, b) => a.filename.localeCompare(b.filename) * directionMultiplier
      );
    }

    // Precompute each timestamp once instead of re-parsing per comparison.
    return filtered
      .map((video) => ({ video, timestamp: new Date(video.created_at).getTime() }))
      .sort((a, b) => (a.timestamp - b.timestamp) * directionMultiplier)
      .map((entry) => entry.video);
  }, [videos, filterMode, sortField, sortDirection]);

  return (
    <div className="app">
      <Sidebar />
      <main className="main-content">
        <div className="top-bar">
          <button
            className={`nav-button ${currentView === 'videos' ? 'active' : ''}`}
            onClick={() => setCurrentView('videos')}
          >
            Videos
          </button>
          <button
            className={`nav-button ${currentView === 'settings' ? 'active' : ''}`}
            onClick={() => setCurrentView('settings')}
          >
            Settings
          </button>
        </div>

        {currentView === 'videos' ? (
          <>
            {selectedFolder !== null && videos.length > 0 && (
              <FilterBar
                filterMode={filterMode}
                onFilterModeChange={handleFilterModeChange}
                favoriteCount={favoriteCount}
                totalCount={videos.length}
                sortField={sortField}
                onSortFieldChange={handleSortFieldChange}
                sortDirection={sortDirection}
                onSortDirectionToggle={handleSortDirectionToggle}
              />
            )}
            {ffmpegError && (
              <div className="error-message" style={{ backgroundColor: '#ff4444', color: 'white', padding: '16px', margin: '16px', borderRadius: '8px' }}>
                <strong>FFmpeg Not Found</strong>
                <p style={{ marginTop: '8px' }}>
                  SceneBrowser requires FFmpeg to generate video thumbnails and extract metadata.
                </p>
                <p style={{ marginTop: '8px' }}>
                  <strong>To install FFmpeg:</strong>
                </p>
                <pre style={{ backgroundColor: 'rgba(0,0,0,0.2)', padding: '8px', borderRadius: '4px', marginTop: '8px' }}>
                  brew install ffmpeg
                </pre>
                <p style={{ marginTop: '8px', fontSize: '14px' }}>
                  After installing FFmpeg, please restart the application.
                </p>
              </div>
            )}
            {error && (
              <div className="error-message">
                <strong>Error:</strong> {error}
              </div>
            )}
            {isLoading ? (
              <div className="loading">
                <div className="spinner"></div>
                <p>Loading videos...</p>
              </div>
            ) : videos.length === 0 ? (
              <div className="empty-state">
                <p>No videos found</p>
                <p className="hint">Add a folder and scan to get started</p>
              </div>
            ) : sortedVideos.length === 0 ? (
              <div className="empty-state">
                <p>No favorite videos</p>
                <p className="hint">Click the ★ button on videos to add them to favorites</p>
              </div>
            ) : (
              <VideoGrid
                videos={sortedVideos}
                selectedFolder={selectedFolder}
                onFavoriteToggled={handleFavoriteToggled}
              />
            )}
          </>
        ) : (
          <Settings />
        )}
      </main>
    </div>
  );
}

export default App;
