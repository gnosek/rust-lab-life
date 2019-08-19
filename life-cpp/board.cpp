#include "board.h"

#include <iostream>

stdx::optional<bool> Grid::at(ssize_t x, ssize_t y) const
{
	auto idx = index(x, y);
	if(idx != stdx::nullopt)
	{
		return m_grid[*idx];
	}
	return stdx::nullopt;
}

stdx::optional<size_t> Grid::index(ssize_t x, ssize_t y) const
{
	if(x < 0 || x >= width())
	{
		return stdx::nullopt;
	}

	if(y < 0 || y >= height())
	{
		return stdx::nullopt;
	}

	return stdx::optional<size_t>(y * height() + x);
}

void Grid::set(ssize_t x, ssize_t y, bool val)
{
	auto idx = index(x, y);
	if(idx != stdx::nullopt)
	{
		m_grid[*idx] = val;
	}
}

std::array<stdx::optional<size_t>, 8> Grid::neighbors(ssize_t x, ssize_t y) const
{
	return {
		index(x - 1, y - 1),
		index(x - 1, y),
		index(x - 1, y + 1),
		index(x, y - 1),
		index(x, y + 1),
		index(x + 1, y - 1),
		index(x + 1, y),
		index(x + 1, y + 1)
	};
}

size_t Grid::count_live_neighbors(ssize_t x, ssize_t y) const
{
	size_t count = 0;

	for(auto idx : neighbors(x, y))
	{
		if(idx != stdx::nullopt)
		{
			count += m_grid[*idx];
		}
	}

	return count;
}

bool LifeBoard::will_live(bool is_live, size_t num_neighbors)
{
	if(is_live)
	{
		return num_neighbors == 2 || num_neighbors == 3;
	}
	else
	{
		return num_neighbors == 3;
	}
}

void LifeBoard::update()
{
	for(size_t y = 0; y < m_current.height(); ++y)
	{
		for(size_t x = 0; x < m_current.width(); ++x)
		{
			bool is_live = m_current.at(x, y).value_or(false);
			size_t num_neighbors = m_current.count_live_neighbors(x, y);

			m_next.set(x, y, will_live(is_live, num_neighbors));
		}
	}
}

const Grid *LifeBoard::tick()
{
	std::cout << "C++ board tick" << std::endl;
	update();
	std::swap(m_current, m_next);

	return &m_current;
}
