#include <algorithm>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <sstream>
#include <string>
#include <string_view>
#include <tuple>
#include <vector>

#include "../helpers.hpp"

void fill_down_matrix(const std::vector<std::vector<int>>& in,
		      std::vector<std::vector<int>>& down) {
	for (int j = 0; j < in[0].size(); ++j) {
		int max = in[in.size() - 1][j];
		for (int i = in.size() - 1; i >= 0; --i) {
			down[i][j] = max;
			if (in[i][j] > max) {
				max = in[i][j];
			}
		}
	}
}

void fill_up_matrix(const std::vector<std::vector<int>>& in,
		    std::vector<std::vector<int>>& up) {
	for (int j = 0; j < in[0].size(); ++j) {
		int max = in[0][j];
		for (int i = 0; i < in.size(); ++i) {
			up[i][j] = max;
			if (in[i][j] > max) {
				max = in[i][j];
			}
		}
	}
}

void fill_left_matrix(const std::vector<std::vector<int>>& in,
		      std::vector<std::vector<int>>& left) {
	for (int i = 0; i < in.size(); ++i) {
		int max = in[i][in.size() - 1];
		for (int j = in[0].size() - 1; j >= 0; --j) {
			left[i][j] = max;
			if (in[i][j] > max) {
				max = in[i][j];
			}
		}
	}
}

void fill_right_matrix(const std::vector<std::vector<int>>& in,
		       std::vector<std::vector<int>>& right) {
	for (int i = 0; i < in.size(); ++i) {
		int max = in[i][0];
		for (int j = 0; j < in[0].size(); ++j) {
			right[i][j] = max;
			if (in[i][j] > max) {
				max = in[i][j];
			}
		}
	}
}

int part1(std::vector<std::vector<int>>& in) {
	std::vector<std::vector<int>> right(in.size(),
					    std::vector<int>(in[0].size()));
	std::vector<std::vector<int>> left(in.size(),
					   std::vector<int>(in[0].size()));
	std::vector<std::vector<int>> up(in.size(),
					 std::vector<int>(in[0].size()));
	std::vector<std::vector<int>> down(in.size(),
					   std::vector<int>(in[0].size()));
	int vis = 0;

	fill_right_matrix(in, right);
	fill_left_matrix(in, left);
	fill_up_matrix(in, up);
	fill_down_matrix(in, down);

	for (int i = 1; i < in.size() - 1; ++i) {
		for (int j = 1; j < in[0].size() - 1; ++j) {
			if (in[i][j] > right[i][j] || in[i][j] > left[i][j] ||
			    in[i][j] > down[i][j] || in[i][j] > up[i][j]) {
				++vis;
			}
		}
	}

	return vis + 2 * in.size() + 2 * (in[0].size()) - 4;
}

int part2(std::vector<std::vector<int>>& in) {
	int max_vis = 0;
	std::vector<std::tuple<int, int>> mov{{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
	const auto row_len = in.size();
	const auto col_len = in[0].size();

	for (int i = 1; i < in.size() - 1; ++i) {
		for (int j = 1; j < in[0].size() - 1; ++j) {
			int mul = 1;
			for (auto [r, c] : mov) {
				int row = i + r;
				int col = j + c;
				int vis = 0;
				while (((0 <= row) && (row < row_len)) &&
				       ((0 <= col) && (col < col_len))) {
					vis++;
					if (in[i][j] > in[row][col]) {
					} else
						break;

					row += r;
					col += c;
				}
				mul *= vis;
			}
			max_vis = std::max(max_vis, mul);
		}
	}

	return max_vis;
}

int main(int argc, char** argv) {
	std::ifstream file{helpers::input_file()};

	int ans1 = 0;
	int ans2 = 0;

	auto in = helpers::file_to_matrix<int, char>(
	    file, [](char v) -> int { return int(v) - 48; });
	// Part 1 could be easily solved with the approach used in part2
	// But keeping it as is.
	std::cout << "First: " << part1(in) << '\n';
	std::cout << "Second: " << part2(in) << '\n';
}
