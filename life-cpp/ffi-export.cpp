#include "board.h"
#include <iostream>

extern "C" {

enum CellValue {
	Dead = 0,
	Alive = 1,
	OutOfBounds = 2
};

Grid *grid_new(int width, int height)
{
	return new Grid(width, height);
}

void grid_delete(Grid *grid)
{
	delete grid;
}

unsigned int grid_width(const Grid *grid)
{
	return grid->width();
}

unsigned int grid_height(const Grid *grid)
{
	return grid->height();
}

CellValue grid_at(const Grid *grid, int x, int y)
{
	auto val = grid->at(x, y);
	if(val != stdx::nullopt)
	{
		if(val.value())
		{
			return CellValue::Alive;
		}
		else
		{
			return CellValue::Dead;
		}
	}
	else
	{
		return CellValue::OutOfBounds;
	}
}

void grid_set(Grid *grid, int x, int y, bool val)
{
	grid->set(x, y, val);
}

unsigned int grid_count_live_neighbors(const Grid *grid, int x, int y)
{
	return grid->count_live_neighbors(x, y);
}

LifeBoard *board_new(Grid *initial_grid)
{
	return new LifeBoard(std::move(*initial_grid));
}

void board_delete(LifeBoard *board)
{
	delete board;
}

const Grid *board_tick(LifeBoard *board)
{
	return board->tick();
}
}
