import { Sidebar } from './components/Sidebar';
import { VideoGrid } from './components/VideoGrid';
import { useVideoStore } from './store/videoStore';
import { useVideos } from './hooks/useVideos';
import './App.css';

function App() {
  const { videos, selectedFolder, isLoading, error } = useVideoStore();

  // Fetch videos when selected folder changes
  useVideos(selectedFolder, 100);

  return (
    <div className="app">
      <Sidebar />
      <main className="main-content">
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
        ) : (
          <VideoGrid videos={videos} />
        )}
      </main>
    </div>
  );
}

export default App;
