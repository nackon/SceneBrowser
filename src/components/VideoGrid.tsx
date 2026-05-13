import { Grid, CellComponentProps } from 'react-window';
import { AutoSizer } from 'react-virtualized-auto-sizer';
import { VideoCard } from './VideoCard';
import type { Video } from '../types/video';
import './VideoGrid.css';

const CARD_WIDTH = 360;
const CARD_HEIGHT = 280;
const GUTTER = 16;

interface VideoGridProps {
  videos: Video[];
}

export function VideoGrid({ videos }: VideoGridProps) {
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
              cellProps={{ videos, columnCount }}
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
}

type CellProps = CellComponentProps<CellData>;

function Cell({ columnIndex, rowIndex, style, videos, columnCount }: CellProps) {
  const index = rowIndex * (columnCount || 1) + columnIndex;
  const video = videos ? videos[index] : undefined;

  if (!video) {
    return null;
  }

  return (
    <div style={{ ...style, padding: `${GUTTER / 2}px` }}>
      <VideoCard video={video} />
    </div>
  );
}
