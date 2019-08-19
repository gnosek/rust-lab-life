#include <cstdio>
#include <chrono>
#include <thread>

#ifdef USE_FFI

#include "ffi-import.h"

#else
#include "board.h"
#endif

#include "display.h"

using namespace std::literals::chrono_literals;

Grid glider(size_t width, size_t height)
{
	Grid grid(width, height);

	grid.set(2, 1, true);
	grid.set(3, 2, true);
	grid.set(1, 3, true);
	grid.set(2, 3, true);
	grid.set(3, 3, true);

	return grid;
}

int main()
{
	auto grid = glider(20, 20);
	show_grid(grid);

	LifeBoard board(std::move(grid));

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wmissing-noreturn"
	while(true)
	{
		std::this_thread::sleep_for(500ms);

		const auto new_grid = board.tick();
		show_grid(*new_grid);
	}
#pragma clang diagnostic pop
}
