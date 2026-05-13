import { useEffect } from 'react';
import { useVideoStore } from '../store/videoStore';
import { getVideos } from '../services/commands';

/**
 * Custom hook to fetch videos for the selected folder
 */
export function useVideos(folderId: number | null, limit = 100) {
  const { setVideos, setIsLoading, setError } = useVideoStore();

  useEffect(() => {
    let cancelled = false;

    async function fetchVideos() {
      setIsLoading(true);
      setError(null);

      try {
        const videos = await getVideos(folderId, limit, 0);
        if (!cancelled) {
          setVideos(videos);
        }
      } catch (error) {
        if (!cancelled) {
          console.error('Failed to fetch videos:', error);
          setError(error instanceof Error ? error.message : String(error));
        }
      } finally {
        if (!cancelled) {
          setIsLoading(false);
        }
      }
    }

    fetchVideos();

    return () => {
      cancelled = true;
    };
  }, [folderId, limit, setVideos, setIsLoading, setError]);
}
