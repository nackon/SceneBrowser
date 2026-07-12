import './FilterBar.css';

export type FilterMode = 'all' | 'favorites';

interface FilterBarProps {
  filterMode: FilterMode;
  onFilterModeChange: (mode: FilterMode) => void;
  favoriteCount: number;
  totalCount: number;
  searchQuery: string;
  onSearchQueryChange: (query: string) => void;
}

export function FilterBar({
  filterMode,
  onFilterModeChange,
  favoriteCount,
  totalCount,
  searchQuery,
  onSearchQueryChange,
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
      <div className="search-box">
        <span className="search-icon">🔍</span>
        <input
          type="text"
          className="search-input"
          placeholder="Search by filename..."
          value={searchQuery}
          onChange={(e) => onSearchQueryChange(e.target.value)}
          aria-label="Search by filename"
        />
        {searchQuery && (
          <button
            type="button"
            className="search-clear"
            onClick={() => onSearchQueryChange('')}
            aria-label="Clear search"
          >
            ✕
          </button>
        )}
      </div>
    </div>
  );
}
