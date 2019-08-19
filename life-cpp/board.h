#pragma once

#include <array>
#include <memory>
#include "optional.h"
#include <vector>

class Grid {
public:
	Grid(size_t width, size_t height) :
		m_width(width),
		m_height(height),
		m_grid(width * height)
	{
	}

	[[nodiscard]] size_t width() const
	{
		return m_width;
	}

	[[nodiscard]] size_t height() const
	{
		return m_height;
	}

	[[nodiscard]] stdx::optional<bool> at(ssize_t x, ssize_t y) const;

	void set(ssize_t x, ssize_t y, bool val);

	[[nodiscard]] size_t count_live_neighbors(ssize_t x, ssize_t y) const;

private:
	[[nodiscard]] stdx::optional<size_t> index(ssize_t x, ssize_t y) const;

	[[nodiscard]] std::array<stdx::optional<size_t>, 8> neighbors(ssize_t x, ssize_t y) const;

	size_t m_width;
	size_t m_height;
	std::vector<bool> m_grid;
};

class LifeBoard {
public:
	explicit LifeBoard(Grid &&initial) :
		m_current(initial),
		m_next(m_current.width(), m_current.height())
	{
	}

	const Grid* tick();

private:
	static bool will_live(bool is_live, size_t num_neighbors);

	void update();

	Grid m_current;
	Grid m_next;
};
