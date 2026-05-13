import { useEffect, useState } from 'react';
import { Sidebar } from './components/Sidebar';
import { VideoGrid } from './components/VideoGrid';
import { useVideoStore } from './store/videoStore';
import { useVideos } from './hooks/useVideos';
import { checkFFmpeg } from './services/commands';
import './App.css';

function App() {
  const { videos, selectedFolder, isLoading, error } = useVideoStore();
  const [ffmpegError, setFFmpegError] = useState<string | null>(null);

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

  return (
    <div className="app">
      <Sidebar />
      <main className="main-content">
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
        ) : (
          <VideoGrid videos={videos} />
        )}
      </main>
    </div>
  );
}

export default App;
