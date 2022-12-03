#include <algorithm>
#include <array>
#include <cstring>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <sstream>
#include <string>
#include <vector>

int calculate_prio(char c) {
	if (std::islower(c)) return c - 96;
	return c - 38;
}

void remove_duplicates(std::string& str) {
	std::set<char> s;

	for (auto c : str) {
		s.insert(c);
	}

	str.erase();

	for (auto c : s) {
		str += c;
	}
}

int part1(std::string& line) {
	auto str1 = line.substr(0, line.size() / 2);
	auto str2 = line.substr(line.size() / 2);

	std::map<char, int> item_count{};
	int val = 0;

	for (auto c : str1) {
		item_count[c] += 1;
	}
	for (auto c : str2) {
		if (!item_count.contains(c)) {
			continue;
		}
		val = calculate_prio(c);
		break;
	}
	return val;
}

int part2(std::string& lines) {
	int val = 0;
	std::stringstream ss(lines);
	std::string line;
	std::map<char, int> item_count{};

	while (std::getline(ss, line)) {
		remove_duplicates(line);
		for (auto c : line) {
			item_count[c] += 1;
			if (item_count[c] == 3) {
				val = calculate_prio(c);
			}
		}
	}
	return val;
}

int main(int argc, char** argv) {
	if (argc != 2) {
		std::cerr << "missing input file\n";
		exit(1);
	}

	std::string line;
	std::ifstream file(argv[1]);
	int sum1 = 0;
	int sum2 = 0;
	int line_count = 0;
	std::string three_line_in;

	while (std::getline(file, line)) {
		sum1 += part1(line);

		three_line_in += line;
		three_line_in += "\n";
		++line_count;
		if (line_count == 3) {
			sum2 += part2(three_line_in);
			three_line_in.erase();
			line_count = 0;
		}
	}

	std::cout << "First: " << sum1 << '\n';
	std::cout << "Second: " << sum2 << '\n';
}
