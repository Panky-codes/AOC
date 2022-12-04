#include <algorithm>
#include <array>
#include <cstring>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <map>
#include <sstream>
#include <string>
#include <string_view>
#include <tuple>
#include <vector>

std::tuple<int, int, int, int> parse_ranges(std::string& line) {
	// Life would have been much easier with good support of std::ranges :/
	int val = 0;
	std::stringstream ss(line);
	std::string range1, range2;
	auto range1_sub = line.substr(0, line.find(","));
	auto range2_sub = line.substr(line.find(",") + 1, line.size());

	auto start_range1_sub = range1_sub.substr(0, range1_sub.find("-"));
	auto end_range1_sub =
	    range1_sub.substr(range1_sub.find("-") + 1, range1_sub.size());

	auto start_range2_sub = range2_sub.substr(0, range2_sub.find("-"));
	auto end_range2_sub =
	    range2_sub.substr(range2_sub.find("-") + 1, range2_sub.size());

	int start_range1 = std::stoi(start_range1_sub);
	int end_range1 = std::stoi(end_range1_sub);

	int start_range2 = std::stoi(start_range2_sub);
	int end_range2 = std::stoi(end_range2_sub);

	return {start_range1, end_range1, start_range2, end_range2};
}

int part1(std::string& line) {
	auto [start_range1, end_range1, start_range2, end_range2] =
	    parse_ranges(line);

	if (start_range1 >= start_range2 && end_range1 <= end_range2) return 1;

	if (start_range2 >= start_range1 && end_range2 <= end_range1) return 1;
	return 0;
}

int part2(std::string& line) {
	auto [start_range1, end_range1, start_range2, end_range2] =
	    parse_ranges(line);

	if (start_range1 >= start_range2 && start_range1 <= end_range2) return 1;

	if (start_range2 >= start_range1 && start_range2 <= end_range1) return 1;


	return 0;
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

	while (std::getline(file, line)) {
		sum1 += part1(line);
		sum2 += part2(line);
	}

	std::cout << "First: " << sum1 << '\n';
	std::cout << "Second: " << sum2 << '\n';
}
