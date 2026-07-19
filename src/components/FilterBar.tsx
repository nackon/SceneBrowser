import { ArrowUp, ArrowDown } from 'lucide-react';
import './FilterBar.css';

export type FilterMode = 'all' | 'favorites';
export type SortField = 'filename' | 'date';
export type SortDirection = 'asc' | 'desc';

const isSortField = (value: string): value is SortField =>
  value === 'filename' || value === 'date';

interface FilterBarProps {
  filterMode: FilterMode;
  onFilterModeChange: (mode: FilterMode) => void;
  favoriteCount: number;
  totalCount: number;
  searchQuery: string;
  onSearchQueryChange: (query: string) => void;
  sortField: SortField;
  onSortFieldChange: (field: SortField) => void;
  sortDirection: SortDirection;
  onSortDirectionToggle: () => void;
}

export function FilterBar({
  filterMode,
  onFilterModeChange,
  favoriteCount,
  totalCount,
  searchQuery,
  onSearchQueryChange,
  sortField,
  onSortFieldChange,
  sortDirection,
  onSortDirectionToggle,
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
      <div className="sort-controls">
        <select
          className="sort-select"
          value={sortField}
          onChange={(e) => {
            if (isSortField(e.target.value)) {
              onSortFieldChange(e.target.value);
            }
          }}
          aria-label="Sort by"
        >
          <option value="filename">ファイル名</option>
          <option value="date">日時</option>
        </select>
        <button
          className="sort-direction-button"
          onClick={onSortDirectionToggle}
          title={sortDirection === 'asc' ? '昇順' : '降順'}
          aria-label={sortDirection === 'asc' ? '昇順' : '降順'}
        >
          {sortDirection === 'asc' ? <ArrowUp size={16} /> : <ArrowDown size={16} />}
        </button>
      </div>
    </div>
  );
}
