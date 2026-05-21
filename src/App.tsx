import { useEffect, useState } from 'react';
import { Sidebar } from './components/Sidebar';
import { VideoGrid } from './components/VideoGrid';
import { Settings } from './components/Settings';
import { FilterBar, type FilterMode } from './components/FilterBar';
import { useVideoStore } from './store/videoStore';
import { useVideos } from './hooks/useVideos';
import { checkFFmpeg, getFavoriteCount } from './services/commands';
import './App.css';

type View = 'videos' | 'settings';

function App() {
  const { videos, selectedFolder, isLoading, error } = useVideoStore();
  const [ffmpegError, setFFmpegError] = useState<string | null>(null);
  const [currentView, setCurrentView] = useState<View>('videos');
  const [filterMode, setFilterMode] = useState<FilterMode>(() => {
    const saved = localStorage.getItem('video_filter_mode');
    return (saved as FilterMode) || 'all';
  });
  const [favoriteCount, setFavoriteCount] = useState(0);

  // Check FFmpeg availability on startup
  useEffect(() => {
    checkFFmpeg().catch((err) => {
      const errorMsg = typeof err === 'string' ? err : String(err);
      setFFmpegError(errorMsg);
      console.error('FFmpeg check failed:', errorMsg);
    });
  }, []);

  // Fetch videos when selected folder changes
  useVideos(selectedFolder, 100);

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

  // Filter videos based on filter mode
  const filteredVideos = filterMode === 'favorites'
    ? videos.filter(v => v.is_favorite === 1)
    : videos;

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
            ) : filteredVideos.length === 0 ? (
              <div className="empty-state">
                <p>No favorite videos</p>
                <p className="hint">Click the ★ button on videos to add them to favorites</p>
              </div>
            ) : (
              <VideoGrid videos={filteredVideos} selectedFolder={selectedFolder} />
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
