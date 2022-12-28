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

static std::map<int, std::map<int, unsigned int>>
    m{};  // 1 for rock, 2 for sand
	  //
static std::vector<std::pair<int, int>> dir{
    {0, 1}, {-1, 1}, {1, 1}};  // d, l, r

void parse_map(std::string& line) {
	helpers::Tokens_t token = helpers::splitstr(line, " -> ");
	int in = 0;
	//
	for (in = 0; in < token.size() - 1; ++in) {
		helpers::Tokens_t from = helpers::splitstr(token[in], ",");
		auto from_major = std::stoi(from[0]);
		auto from_minor = std::stoi(from[1]);
		helpers::Tokens_t to = helpers::splitstr(token[in + 1], ",");
		auto to_major = std::stoi(to[0]);
		auto to_minor = std::stoi(to[1]);

		auto col_start = std::min(from_major, to_major);
		auto col_end = std::max(from_major, to_major);

		auto row_start = std::min(from_minor, to_minor);
		auto row_end = std::max(from_minor, to_minor);

		for (auto col = col_start; col <= col_end; ++col)
			for (auto row = row_start; row <= row_end; ++row)
				m[col][row] = 1;
	}
}

int floor_val(int part) {
	std::vector<int> val;
	for (auto i : m) {
		for (auto e : i.second) val.push_back(e.first);
	}
	auto max_ele = *std::max_element(val.begin(), val.end());

	return (part == 1) ? max_ele : (max_ele + 2);
}

void print_map(int part) {
	for (auto e = 0; e <= floor_val(part); e++) {
		for (auto i = 480; i < 520; i++) {
			if (m.contains(i) && m[i].contains(e))
				std::cout << ((m[i][e] == 1) ? "#" : "o");
			else
				std::cout << ".";
		}
		std::cout << std::endl;
	}
}

int part1() {
	auto floor = floor_val(1);
	auto steps = 0;

	while (1) {
		std::pair<int, int> pos = {500, 0};
		bool found_spot = false;

		while (!found_spot) {
			auto counter = 0;
			for (auto [dx, dy] : dir) {
				auto tmp_pos = pos;
				tmp_pos.first += dx;
				tmp_pos.second += dy;

				if (tmp_pos.second > floor) {
					goto exit;
				}

				if (m.contains(tmp_pos.first) &&
				    m[tmp_pos.first].contains(tmp_pos.second)) {
					++counter;
					continue;
				} else {
					pos = tmp_pos;
					break;
				}
			}
			if (counter == dir.size()) {
				m[pos.first][pos.second] = 2;
				found_spot = true;
				++steps;
			}
		}
	}
exit:
	return steps;
}

int part2() {
	auto floor = floor_val(2);
	auto steps = 0;

	while (1) {
		std::pair<int, int> pos = {500, 0};
		bool found_spot = false;

		if (m.contains(pos.first) && m[pos.first].contains(pos.second))
			goto exit;

		while (!found_spot) {
			auto counter = 0;
			for (auto [dx, dy] : dir) {
				auto tmp_pos = pos;
				tmp_pos.first += dx;
				tmp_pos.second += dy;

				// lazily fill the bottom
				if (tmp_pos.second == floor) {
					m[tmp_pos.first][tmp_pos.second] = 1;
				}

				if (m.contains(tmp_pos.first) &&
				    m[tmp_pos.first].contains(tmp_pos.second)) {
					++counter;
					continue;
				} else {
					pos = tmp_pos;
					break;
				}
			}
			if (counter == dir.size()) {
				m[pos.first][pos.second] = 2;
				found_spot = true;
				++steps;
			}
		}
		print_map(2);
	}
exit:
	return steps;
}

int main(int argc, char** argv) {
	std::string line;
	std::ifstream file{helpers::input_file()};
	std::map<int, std::map<int, unsigned int>> tmp_m;

	if (!file.good()) {
		std::cerr << "Input file not found" << std::endl;
		exit(1);
	}

	while (std::getline(file, line)) {
		parse_map(line);
	}

	tmp_m =  m;

	std::cout << "First: " << part1() << '\n';
	m = tmp_m;
	std::cout << "Second: " << part2() << '\n';
}
