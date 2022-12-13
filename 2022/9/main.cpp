#include <algorithm>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <sstream>
#include <string>
#include <string_view>
#include <tuple>
#include <unordered_map>
#include <unordered_set>
#include <vector>

#include "../helpers.hpp"

static std::unordered_map<std::string, std::pair<int, int>> dir_map{
    {"R", {0, 1}}, {"L", {0, -1}}, {"U", {1, 0}}, {"D", {-1, 0}}};

static std::pair<int, int> head_loc{0, 0};  // Row, Col
static std::unordered_set<std::pair<int, int>, helpers::PairHash> tail_set_p1{};
static std::unordered_set<std::pair<int, int>, helpers::PairHash> tail_set_p2{};

void move_tail(std::pair<int, int>& tail_loc,
	       const std::pair<int, int>& head_loc) {
	auto row_diff = head_loc.first - tail_loc.first;
	auto col_diff = head_loc.second - tail_loc.second;

	if (std::abs(row_diff) <= 1 && std::abs(col_diff) <= 1) return;

	if (!row_diff) {
		tail_loc.second += ((col_diff < 0) ? -1 : 1);
	} else if (!col_diff) {
		tail_loc.first += ((row_diff < 0) ? -1 : 1);
	} else {
		tail_loc.first += ((row_diff < 0) ? -1 : 1);
		tail_loc.second += ((col_diff < 0) ? -1 : 1);
	}
}

int part1() { return tail_set_p1.size(); }

int part2() { return tail_set_p2.size(); }

int main(int argc, char** argv) {
	std::string line;
	std::ifstream file{helpers::input_file()};
	static std::array<std::pair<int, int>, 9> tail_loc_arr{};

	if (!file.good()) {
		std::cerr << "Input file not found" << std::endl;
		exit(1);
	}

	while (std::getline(file, line)) {
		helpers::Tokens_t token = helpers::splitstr(line);
		auto dist = std::stoi(token[1]);

		for (int i = 0; i < dist; ++i) {
			head_loc.first += dir_map[token[0]].first;
			head_loc.second += dir_map[token[0]].second;

			move_tail(tail_loc_arr[0], head_loc);
			tail_set_p1.insert(tail_loc_arr[0]);

			for (auto i = 1; i < 9; ++i) {
				move_tail(tail_loc_arr[i], tail_loc_arr[i - 1]);
			}
			tail_set_p2.insert(tail_loc_arr[8]);
		}
	}

	std::cout << "First: " << part1() << '\n';
	std::cout << "Second: " << part2() << '\n';
}
