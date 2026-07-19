import { Grid, CellComponentProps } from 'react-window';
import { AutoSizer } from 'react-virtualized-auto-sizer';
import { VideoCard } from './VideoCard';
import type { Video } from '../types/video';
import './VideoGrid.css';

const CARD_WIDTH = 400;
const CARD_HEIGHT = 320;
const GUTTER = 16;

interface VideoGridProps {
  videos: Video[];
  selectedFolder: number | null;
  onFavoriteToggled?: () => void;
}

export function VideoGrid({ videos, selectedFolder, onFavoriteToggled }: VideoGridProps) {
  return (
    <div className="video-grid-container">
      <AutoSizer
        renderProp={({ height, width }) => {
          if (!height || !width) {
            return null;
          }
          const columnCount = Math.max(1, Math.floor(width / (CARD_WIDTH + GUTTER)));
          const rowCount = Math.ceil(videos.length / columnCount);

          return (
            <Grid
              columnCount={columnCount}
              columnWidth={CARD_WIDTH + GUTTER}
              defaultHeight={height}
              defaultWidth={width}
              rowCount={rowCount}
              rowHeight={CARD_HEIGHT + GUTTER}
              cellComponent={Cell}
              cellProps={{ videos, columnCount, selectedFolder, onFavoriteToggled }}
            />
          );
        }}
      />
    </div>
  );
}

interface CellData {
  videos: Video[];
  columnCount: number;
  selectedFolder: number | null;
  onFavoriteToggled?: () => void;
}

type CellProps = CellComponentProps<CellData>;

function Cell({ columnIndex, rowIndex, style, videos, columnCount, selectedFolder, onFavoriteToggled }: CellProps) {
  const index = rowIndex * (columnCount || 1) + columnIndex;
  const video = videos ? videos[index] : undefined;

  if (!video) {
    return null;
  }

  return (
    <div style={{ ...style, padding: `${GUTTER / 2}px` }}>
      {/* key forces a remount when the video at this recycled cell position changes,
          preventing stale favorite/thumbnail state from leaking across videos. */}
      <VideoCard key={video.id} video={video} folderId={selectedFolder} onFavoriteToggled={onFavoriteToggled} />
    </div>
  );
}
