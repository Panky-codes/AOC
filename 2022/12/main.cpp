#include <algorithm>
#include <fstream>
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <sstream>
#include <string>
#include <vector>

#include "../helpers.hpp"

static std::vector<std::vector<int>> matrix{};
static std::vector<std::vector<char>> matrix_orig{};
static int row_len;
static int col_len;

void starting_pos(std::queue<std::tuple<std::pair<int, int>, int>>& q,
		  int part) {
	int r = 0, c = 0;
	int tmp_r = 0;
	for (auto& a : matrix) {
		int tmp_c = 0;
		for (auto& v : a) {
			if ((part == 1 && matrix_orig[tmp_r][tmp_c] == 'S') ||
			    (part == 2 && matrix[tmp_r][tmp_c] == 1)) {
				q.push({{tmp_r, tmp_c}, 0});
			};
			++tmp_c;
		}
		++tmp_r;
	}
}

int bfs(std::queue<std::tuple<std::pair<int, int>, int>>& q) {
	std::set<std::pair<int, int>> visited{};
	std::vector<std::pair<int, int>> dir{{1, 0}, {-1, 0}, {0, 1}, {0, -1}};

	// BFS from S to E
	while (1) {
		auto [pos, dist] = q.front();
		int start_r = pos.first;
		int start_c = pos.second;

		if (visited.contains(pos)) {
			q.pop();
			continue;
		}

		if (matrix_orig[start_r][start_c] == 'E') {
			return dist;
		}

		visited.insert(pos);
		q.pop();

		// get neighbours
		for (auto d : dir) {
			auto rr = start_r + d.first;
			auto cc = start_c + d.second;

			if (((rr >= 0) && (rr < row_len)) &&
			    ((cc >= 0) && (cc < col_len)) &&
			    (matrix[rr][cc] - matrix[start_r][start_c] <= 1)) {
				q.push({{rr, cc}, dist + 1});
			}
		}
	}
	return -1;
}
int part1() {
	std::queue<std::tuple<std::pair<int, int>, int>> q{};

	starting_pos(q, 1);
	return bfs(q);
}

int part2() {
	std::queue<std::tuple<std::pair<int, int>, int>> q{};

	starting_pos(q, 2);
	return bfs(q);
}

int main(int argc, char** argv) {
	std::string line;
	std::ifstream file{helpers::input_file()};
	std::ifstream file1{helpers::input_file()};

	auto conv_lambda = [](char v) -> int {
		if (v == 'S') v = 'a';
		if (v == 'E') v = 'z';

		return (int)v - (int)'a' + 1;
	};

	if (!file.good()) {
		std::cerr << "Input file not found" << std::endl;
		exit(1);
	}

	matrix = helpers::file_to_matrix<int, char>(file, conv_lambda);
	matrix_orig = helpers::file_to_matrix<char, char>(file1);
	row_len = matrix.size();
	col_len = matrix[0].size();

	std::cout << "First: " << part1() << '\n';
	std::cout << "Second: " << part2() << '\n';
}
