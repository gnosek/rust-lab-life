#pragma once

#include <iostream>

template<class G>
void show_grid(const G &grid)
{
//	std::cout << "\e[H\e[J";
	for(size_t x = 0; x < grid.width(); ++x)
	{
		std::cout << '-';
	}
	std::cout << '\n';
	for(size_t y = 0; y < grid.height(); ++y)
	{
		for(size_t x = 0; x < grid.width(); ++x)
		{
			if(grid.at(x, y).value_or(false))
			{
				std::cout << '#';
			}
			else
			{
				std::cout << '.';
			}
		}
		std::cout << std::endl;
	}

}