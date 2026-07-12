import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { FilterBar } from './FilterBar';

describe('FilterBar', () => {
  it('renders both filter buttons', () => {
    const onFilterModeChange = vi.fn();

    render(
      <FilterBar
        filterMode="all"
        onFilterModeChange={onFilterModeChange}
        favoriteCount={5}
        totalCount={20}
        searchQuery=""
        onSearchQueryChange={vi.fn()}
      />
    );

    expect(screen.getByText('All Videos')).toBeInTheDocument();
    expect(screen.getByText('Favorites')).toBeInTheDocument();
  });

  it('displays correct counts', () => {
    const onFilterModeChange = vi.fn();

    render(
      <FilterBar
        filterMode="all"
        onFilterModeChange={onFilterModeChange}
        favoriteCount={5}
        totalCount={20}
        searchQuery=""
        onSearchQueryChange={vi.fn()}
      />
    );

    const allButton = screen.getByText('All Videos').closest('button');
    const favoritesButton = screen.getByText('Favorites').closest('button');

    expect(allButton?.textContent).toContain('20');
    expect(favoritesButton?.textContent).toContain('5');
  });

  it('marks "all" button as active when filterMode is "all"', () => {
    const onFilterModeChange = vi.fn();

    render(
      <FilterBar
        filterMode="all"
        onFilterModeChange={onFilterModeChange}
        favoriteCount={5}
        totalCount={20}
        searchQuery=""
        onSearchQueryChange={vi.fn()}
      />
    );

    const allButton = screen.getByText('All Videos').closest('button');
    const favoritesButton = screen.getByText('Favorites').closest('button');

    expect(allButton).toHaveClass('active');
    expect(favoritesButton).not.toHaveClass('active');
  });

  it('marks "favorites" button as active when filterMode is "favorites"', () => {
    const onFilterModeChange = vi.fn();

    render(
      <FilterBar
        filterMode="favorites"
        onFilterModeChange={onFilterModeChange}
        favoriteCount={5}
        totalCount={20}
        searchQuery=""
        onSearchQueryChange={vi.fn()}
      />
    );

    const allButton = screen.getByText('All Videos').closest('button');
    const favoritesButton = screen.getByText('Favorites').closest('button');

    expect(allButton).not.toHaveClass('active');
    expect(favoritesButton).toHaveClass('active');
  });

  it('calls onFilterModeChange with "all" when All Videos button is clicked', async () => {
    const user = userEvent.setup();
    const onFilterModeChange = vi.fn();

    render(
      <FilterBar
        filterMode="favorites"
        onFilterModeChange={onFilterModeChange}
        favoriteCount={5}
        totalCount={20}
        searchQuery=""
        onSearchQueryChange={vi.fn()}
      />
    );

    const allButton = screen.getByText('All Videos');
    await user.click(allButton);

    expect(onFilterModeChange).toHaveBeenCalledWith('all');
  });

  it('calls onFilterModeChange with "favorites" when Favorites button is clicked', async () => {
    const user = userEvent.setup();
    const onFilterModeChange = vi.fn();

    render(
      <FilterBar
        filterMode="all"
        onFilterModeChange={onFilterModeChange}
        favoriteCount={5}
        totalCount={20}
        searchQuery=""
        onSearchQueryChange={vi.fn()}
      />
    );

    const favoritesButton = screen.getByText('Favorites');
    await user.click(favoritesButton);

    expect(onFilterModeChange).toHaveBeenCalledWith('favorites');
  });

  it('displays correct icons', () => {
    const onFilterModeChange = vi.fn();

    render(
      <FilterBar
        filterMode="all"
        onFilterModeChange={onFilterModeChange}
        favoriteCount={5}
        totalCount={20}
        searchQuery=""
        onSearchQueryChange={vi.fn()}
      />
    );

    const allButton = screen.getByText('All Videos').closest('button');
    const favoritesButton = screen.getByText('Favorites').closest('button');

    expect(allButton?.textContent).toContain('🎬');
    expect(favoritesButton?.textContent).toContain('★');
  });

  it('handles zero favorites', () => {
    const onFilterModeChange = vi.fn();

    render(
      <FilterBar
        filterMode="all"
        onFilterModeChange={onFilterModeChange}
        favoriteCount={0}
        totalCount={20}
        searchQuery=""
        onSearchQueryChange={vi.fn()}
      />
    );

    const favoritesButton = screen.getByText('Favorites').closest('button');
    expect(favoritesButton?.textContent).toContain('0');
  });

  it('handles zero total videos', () => {
    const onFilterModeChange = vi.fn();

    render(
      <FilterBar
        filterMode="all"
        onFilterModeChange={onFilterModeChange}
        favoriteCount={0}
        totalCount={0}
        searchQuery=""
        onSearchQueryChange={vi.fn()}
      />
    );

    const allButton = screen.getByText('All Videos').closest('button');
    expect(allButton?.textContent).toContain('0');
  });

  it('handles when all videos are favorites', () => {
    const onFilterModeChange = vi.fn();

    render(
      <FilterBar
        filterMode="all"
        onFilterModeChange={onFilterModeChange}
        favoriteCount={10}
        totalCount={10}
        searchQuery=""
        onSearchQueryChange={vi.fn()}
      />
    );

    const allButton = screen.getByText('All Videos').closest('button');
    const favoritesButton = screen.getByText('Favorites').closest('button');

    expect(allButton?.textContent).toContain('10');
    expect(favoritesButton?.textContent).toContain('10');
  });

  describe('search box', () => {
    it('renders the search input with the current query', () => {
      const onFilterModeChange = vi.fn();
      const onSearchQueryChange = vi.fn();

      render(
        <FilterBar
          filterMode="all"
          onFilterModeChange={onFilterModeChange}
          favoriteCount={5}
          totalCount={20}
          searchQuery="beach"
          onSearchQueryChange={onSearchQueryChange}
        />
      );

      expect(screen.getByPlaceholderText('Search by filename...')).toHaveValue('beach');
    });

    it('calls onSearchQueryChange incrementally as the user types', async () => {
      const user = userEvent.setup();
      const onFilterModeChange = vi.fn();
      const onSearchQueryChange = vi.fn();

      render(
        <FilterBar
          filterMode="all"
          onFilterModeChange={onFilterModeChange}
          favoriteCount={5}
          totalCount={20}
          searchQuery=""
          onSearchQueryChange={onSearchQueryChange}
        />
      );

      const input = screen.getByPlaceholderText('Search by filename...');
      await user.type(input, 'cat');

      expect(onSearchQueryChange).toHaveBeenCalledTimes(3);
      expect(onSearchQueryChange).toHaveBeenNthCalledWith(1, 'c');
      expect(onSearchQueryChange).toHaveBeenNthCalledWith(2, 'a');
      expect(onSearchQueryChange).toHaveBeenNthCalledWith(3, 't');
    });

    it('does not render a clear button when the query is empty', () => {
      const onFilterModeChange = vi.fn();
      const onSearchQueryChange = vi.fn();

      render(
        <FilterBar
          filterMode="all"
          onFilterModeChange={onFilterModeChange}
          favoriteCount={5}
          totalCount={20}
          searchQuery=""
          onSearchQueryChange={onSearchQueryChange}
        />
      );

      expect(screen.queryByLabelText('Clear search')).not.toBeInTheDocument();
    });

    it('clears the query when the clear button is clicked', async () => {
      const user = userEvent.setup();
      const onFilterModeChange = vi.fn();
      const onSearchQueryChange = vi.fn();

      render(
        <FilterBar
          filterMode="all"
          onFilterModeChange={onFilterModeChange}
          favoriteCount={5}
          totalCount={20}
          searchQuery="beach"
          onSearchQueryChange={onSearchQueryChange}
        />
      );

      await user.click(screen.getByLabelText('Clear search'));

      expect(onSearchQueryChange).toHaveBeenCalledWith('');
    });
  });
});
