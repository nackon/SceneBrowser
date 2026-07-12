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
        sortField="filename"
        onSortFieldChange={vi.fn()}
        sortDirection="asc"
        onSortDirectionToggle={vi.fn()}
        favoriteCount={5}
        totalCount={20}
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
        sortField="filename"
        onSortFieldChange={vi.fn()}
        sortDirection="asc"
        onSortDirectionToggle={vi.fn()}
        favoriteCount={5}
        totalCount={20}
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
        sortField="filename"
        onSortFieldChange={vi.fn()}
        sortDirection="asc"
        onSortDirectionToggle={vi.fn()}
        favoriteCount={5}
        totalCount={20}
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
        sortField="filename"
        onSortFieldChange={vi.fn()}
        sortDirection="asc"
        onSortDirectionToggle={vi.fn()}
        favoriteCount={5}
        totalCount={20}
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
        sortField="filename"
        onSortFieldChange={vi.fn()}
        sortDirection="asc"
        onSortDirectionToggle={vi.fn()}
        favoriteCount={5}
        totalCount={20}
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
        sortField="filename"
        onSortFieldChange={vi.fn()}
        sortDirection="asc"
        onSortDirectionToggle={vi.fn()}
        favoriteCount={5}
        totalCount={20}
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
        sortField="filename"
        onSortFieldChange={vi.fn()}
        sortDirection="asc"
        onSortDirectionToggle={vi.fn()}
        favoriteCount={5}
        totalCount={20}
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
        sortField="filename"
        onSortFieldChange={vi.fn()}
        sortDirection="asc"
        onSortDirectionToggle={vi.fn()}
        favoriteCount={0}
        totalCount={20}
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
        sortField="filename"
        onSortFieldChange={vi.fn()}
        sortDirection="asc"
        onSortDirectionToggle={vi.fn()}
        favoriteCount={0}
        totalCount={0}
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
        sortField="filename"
        onSortFieldChange={vi.fn()}
        sortDirection="asc"
        onSortDirectionToggle={vi.fn()}
        favoriteCount={10}
        totalCount={10}
      />
    );

    const allButton = screen.getByText('All Videos').closest('button');
    const favoritesButton = screen.getByText('Favorites').closest('button');

    expect(allButton?.textContent).toContain('10');
    expect(favoritesButton?.textContent).toContain('10');
  });

  it('renders the sort field select with the current value', () => {
    render(
      <FilterBar
        filterMode="all"
        onFilterModeChange={vi.fn()}
        sortField="date"
        onSortFieldChange={vi.fn()}
        sortDirection="asc"
        onSortDirectionToggle={vi.fn()}
        favoriteCount={5}
        totalCount={20}
      />
    );

    expect(screen.getByLabelText('Sort by')).toHaveValue('date');
  });

  it('calls onSortFieldChange when a different sort field is selected', async () => {
    const user = userEvent.setup();
    const onSortFieldChange = vi.fn();

    render(
      <FilterBar
        filterMode="all"
        onFilterModeChange={vi.fn()}
        sortField="filename"
        onSortFieldChange={onSortFieldChange}
        sortDirection="asc"
        onSortDirectionToggle={vi.fn()}
        favoriteCount={5}
        totalCount={20}
      />
    );

    await user.selectOptions(screen.getByLabelText('Sort by'), 'date');

    expect(onSortFieldChange).toHaveBeenCalledWith('date');
  });

  it('calls onSortDirectionToggle when the direction button is clicked', async () => {
    const user = userEvent.setup();
    const onSortDirectionToggle = vi.fn();

    render(
      <FilterBar
        filterMode="all"
        onFilterModeChange={vi.fn()}
        sortField="filename"
        onSortFieldChange={vi.fn()}
        sortDirection="asc"
        onSortDirectionToggle={onSortDirectionToggle}
        favoriteCount={5}
        totalCount={20}
      />
    );

    await user.click(screen.getByLabelText('昇順'));

    expect(onSortDirectionToggle).toHaveBeenCalled();
  });
});
