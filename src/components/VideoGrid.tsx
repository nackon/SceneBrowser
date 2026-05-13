import { FixedSizeGrid as Grid } from 'react-window';
import AutoSizer from 'react-virtualized-auto-sizer';
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
      <AutoSizer>
        {({ height, width }) => {
          const columnCount = Math.max(1, Math.floor(width / (CARD_WIDTH + GUTTER)));
          const rowCount = Math.ceil(videos.length / columnCount);

          return (
            <Grid
              columnCount={columnCount}
              columnWidth={CARD_WIDTH + GUTTER}
              height={height}
              rowCount={rowCount}
              rowHeight={CARD_HEIGHT + GUTTER}
              width={width}
              itemData={{ videos, columnCount }}
            >
              {Cell}
            </Grid>
          );
        }}
      </AutoSizer>
    </div>
  );
}

interface CellProps {
  columnIndex: number;
  rowIndex: number;
  style: React.CSSProperties;
  data: {
    videos: Video[];
    columnCount: number;
  };
}

function Cell({ columnIndex, rowIndex, style, data }: CellProps) {
  const { videos, columnCount } = data;
  const index = rowIndex * columnCount + columnIndex;
  const video = videos[index];

  if (!video) {
    return null;
  }

  return (
    <div style={{ ...style, padding: `${GUTTER / 2}px` }}>
      <VideoCard video={video} />
    </div>
  );
}
