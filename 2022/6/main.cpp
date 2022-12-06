#include <algorithm>
#include <array>
#include <fstream>
#include <iostream>
#include <string>

template <typename T, std::size_t len>
bool check_unique(std::array<T, len>& a) {
	for (int i = 0; i < len; i++) {
		for (int j = i + 1; j < len; ++j) {
			if (a[i] == a[j]) return false;
		}
	}
	return true;
}

template <std::size_t len>
int solve(std::string& line) {
	std::array<char, len> recent = {0, 0, 0, 0};
	int count = 0;
	bool starting = true;
	for (auto c : line.substr(0, len)) {
		recent[count++] = c;
	}

	for (auto c : line.substr(len, line.size())) {
		if (check_unique(recent)) {
			return count++;	 // as it is zero based
		}
		recent[count++ % len] = c;
	}
	return 0;
}

int main(int argc, char** argv) {
	if (argc != 2) {
		std::cerr << "missing input file\n";
		exit(1);
	}

	std::string line;
	std::ifstream file(argv[1]);
	int loc1 = 0;
	int loc2 = 0;

	while (std::getline(file, line)) {
		loc1 = solve<4>(line);
		loc2 = solve<14>(line);
	}

	std::cout << "First: " << loc1 << '\n';
	std::cout << "Second: " << loc2 << '\n';
}
