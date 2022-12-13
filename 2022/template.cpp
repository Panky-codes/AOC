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

int part1() {
	return 0;
}

int part2() {
	return 0;
}

int main(int argc, char** argv) {
	std::string line;
	std::ifstream file{helpers::input_file()};

	if (!file.good()) {
		std::cerr << "Input file not found" << std::endl;
		exit(1);
	}

	while (std::getline(file, line)) {
		helpers::Tokens_t token = helpers::splitstr(line);
	}

	std::cout << "First: " << part1() << '\n';
	std::cout << "Second: " << part2() << '\n';
}
