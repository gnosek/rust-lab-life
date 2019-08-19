#pragma once

#include <memory>
#include "optional.h"

namespace ffi {
extern "C" {

enum CellValue {
	Dead = 0,
	Alive = 1,
	OutOfBounds = 2
};

struct Grid;
struct LifeBoard;

Grid *grid_new(int width, int height);
void grid_delete(Grid *grid);
unsigned int grid_width(const Grid *grid);
unsigned int grid_height(const Grid *grid);
CellValue grid_at(const Grid *grid, int x, int y);
void grid_set(Grid *grid, int x, int y, bool val);
unsigned int grid_count_live_neighbors(const Grid *grid, int x, int y);

LifeBoard *board_new(Grid *initial_grid);
void board_delete(LifeBoard *board);
const Grid *board_tick(LifeBoard *board);
}
}

class LifeBoard;

class Grid {
public:
	Grid(int width, int height):
		m_grid(ffi::grid_new(width, height)),
		m_owned(true)
	{}

	explicit Grid(ffi::Grid* grid) noexcept:
		m_grid(grid),
		m_owned(false)
	{}

	Grid(Grid&& rhs) noexcept:
		m_grid(rhs.m_grid),
		m_owned(rhs.m_owned)
	{
		rhs.m_grid = nullptr;
		rhs.m_owned = false;
	}


	~Grid() {
		if(m_owned)
		{
			ffi::grid_delete(m_grid);
		}
	}

	[[nodiscard]] size_t width() const
	{
		return ffi::grid_width(m_grid);
	}

	[[nodiscard]] size_t height() const
	{
		return ffi::grid_height(m_grid);
	}

	[[nodiscard]] stdx::optional<bool> at(ssize_t x, ssize_t y) const
	{
		switch(ffi::grid_at(m_grid, x, y))
		{
		case ffi::CellValue::Alive:
			return stdx::make_optional<bool>(true);
		case ffi::CellValue::Dead:
			return stdx::make_optional<bool>(false);
		default:
			return stdx::nullopt;
		}
	}

	void set(ssize_t x, ssize_t y, bool val)
	{
		ffi::grid_set(m_grid, x, y, val);
	}

	[[nodiscard]] size_t count_live_neighbors(ssize_t x, ssize_t y) const
	{
		return ffi::grid_count_live_neighbors(m_grid, x, y);
	}

private:
	ffi::Grid* m_grid;
	bool m_owned;
	friend class LifeBoard;

	ffi::Grid* take_grid()
	{
		auto grid = m_grid;
		m_grid = nullptr;
		m_owned = false;

		return grid;
	}
};

class LifeBoard {
public:
	explicit LifeBoard(Grid &&initial) :
		m_board(ffi::board_new(initial.take_grid())),
		m_grid(nullptr)
	{
	}

	~LifeBoard()
	{
		ffi::board_delete(m_board);
	}

	const Grid *tick()
	{
		const ffi::Grid* next = ffi::board_tick(m_board);
		m_grid = std::make_unique<const Grid>(const_cast<ffi::Grid*>(next));
		return m_grid.get();
	}

private:
	ffi::LifeBoard* m_board;
	std::unique_ptr<const Grid> m_grid;
};
