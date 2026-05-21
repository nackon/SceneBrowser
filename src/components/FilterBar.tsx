import './FilterBar.css';

export type FilterMode = 'all' | 'favorites';

interface FilterBarProps {
  filterMode: FilterMode;
  onFilterModeChange: (mode: FilterMode) => void;
  favoriteCount: number;
  totalCount: number;
}

export function FilterBar({
  filterMode,
  onFilterModeChange,
  favoriteCount,
  totalCount,
}: FilterBarProps) {
  return (
    <div className="filter-bar">
      <div className="filter-buttons">
        <button
          className={`filter-button ${filterMode === 'all' ? 'active' : ''}`}
          onClick={() => onFilterModeChange('all')}
        >
          <span className="filter-icon">🎬</span>
          <span className="filter-label">All Videos</span>
          <span className="filter-count">{totalCount}</span>
        </button>
        <button
          className={`filter-button ${filterMode === 'favorites' ? 'active' : ''}`}
          onClick={() => onFilterModeChange('favorites')}
        >
          <span className="filter-icon">★</span>
          <span className="filter-label">Favorites</span>
          <span className="filter-count">{favoriteCount}</span>
        </button>
      </div>
    </div>
  );
}
