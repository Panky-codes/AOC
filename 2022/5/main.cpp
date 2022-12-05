#include <algorithm>
#include <array>
#include <cctype>
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

std::vector<std::vector<char>> input = {{'z', 'n'}, {'m', 'c', 'd'}, {'p'}};
std::vector<std::vector<char>> input_full = {
    {'s', 'l', 'w'},
    {'j', 't', 'n', 'q'},
    {'s', 'c', 'h', 'f', 'j'},
    {'t', 'r', 'm', 'w', 'n', 'g', 'b'},
    {'t', 'r', 'l', 's', 'd', 'h', 'q', 'b'},
    {'m', 'j', 'b', 'v', 'f', 'h', 'r', 'l'},
    {'d', 'w', 'r', 'n', 'j', 'm'},
    {'b', 'z', 't', 'f', 'h', 'n', 'd', 'j'},
    {'h', 'l', 'q', 'n', 'b', 'f', 't'},
};

void print_vec(std::vector<std::vector<char>>& in){
	for(auto v : in) {
		std::cout << (char)toupper(v.back());
	}
	std::cout << std::endl;
}

std::tuple<int, int, int> parse_line(std::string& line) {
	std::vector<int> actions{};
	std::string c;
	std::stringstream ss(line);

	while (std::getline(ss, c, ' ')) {
		actions.push_back(std::stoi(c));
	}

	return {actions[0], actions[1], actions[2]};
}

void part1(std::string& line, std::vector<std::vector<char>>& in) {
	auto [nr, from, to] = parse_line(line);

	for (int i = 0; i < nr; ++i) {
		char c = in[from - 1].back();

		in[from - 1].pop_back();
		in[to - 1].push_back(c);
	}
}

void part2(std::string& line, std::vector<std::vector<char>>& in) {
	auto [nr, from, to] = parse_line(line);

	in[to - 1].insert(in[to - 1].end(), in[from - 1].end() - nr,
			  in[from - 1].end());

	in[from - 1].erase(in[from - 1].end() - nr, in[from - 1].end());
}

int main(int argc, char** argv) {
	if (argc != 2) {
		std::cerr << "missing input file\n";
		exit(1);
	}

	std::string line;
	std::ifstream file(argv[1]);
	std::vector<std::vector<char>> in1 = input_full;
	std::vector<std::vector<char>> in2 = input_full;

	while (std::getline(file, line)) {
		part1(line, in1);
		part2(line, in2);
	}

	std::cout << "First: ";
	print_vec(in1);

	std::cout << "Second: ";
	print_vec(in2);
}
