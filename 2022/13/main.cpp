#include <algorithm>
#include <fstream>
#include <iostream>
#include <iterator>
#include <map>
#include <nlohmann/json.hpp>
#include <set>
#include <sstream>
#include <string>
#include <string_view>
#include <tuple>
#include <typeinfo>
#include <vector>

#include "../helpers.hpp"

using json = nlohmann::json;

int part1() { return 0; }

int cmp(int l, int r) {
	if (r > l) {
		return 1;
	}

	if (r < l) {
		return -1;
	}

	return 0;
}

int compare(json& l1, json& l2) {
	long unsigned int in = 0;
	auto res = 0;
	for (in = 0; in < std::min(l1.size(), l2.size()); ++in) {
		auto v = l1[in];
		if (v.is_array() && l2[in].is_array()) {
			if ((res = compare(v, l2[in]))) {
				break;
			}
		}
		if (v.is_array() && l2[in].is_number()) {
			json j = json::array_t(std::vector<json>{l2[in]});
			if ((res = compare(v, j))) {
				break;
			}
		}
		if (v.is_number() && l2[in].is_array()) {
			json j = json::array_t(std::vector<json>{v});

			if ((res = compare(j, l2[in]))) {
				break;
			}
		}
		if (v.is_number() && l2[in].is_number()) {
			if ((res = cmp(v, l2[in]))) {
				break;
			}
		}
	}

	if (!res && (l1.size() == in) && (in < l2.size())) res = 1;
	if (!res && (l2.size() == in) && (in < l1.size())) res = -1;

	return res;
}

int part2() { return 0; }

int main(int argc, char** argv) {
	std::string line;
	std::ifstream file{helpers::input_file()};
	unsigned long p1 = 0;

	if (!file.good()) {
		std::cerr << "Input file not found" << std::endl;
		exit(1);
	}

	int i = 0;
	int pair = 1;
	std::string prev = "";
	std::vector<json> l{};

	while (std::getline(file, line)) {
		if (line == "") {
			i = 0;
			pair++;
			continue;
		}

		if (i) {
			json l1 = json::parse(prev);
			json l2 = json::parse(line);
			if (compare(l1, l2) > 0) {
				p1 += pair;
			}
			l.push_back(l1);
			l.push_back(l2);
			continue;
		}

		prev = line;
		++i;
	}

	l.push_back(json::parse("[[2]]"));
	l.push_back(json::parse("[[6]]"));

	std::sort(l.begin(), l.end(), [](json l1, json l2) {
		return compare(l1, l2) == 1 ? true : false;
	});

	std::cout << "First: " << p1 << '\n';

	unsigned long index = 1;
	unsigned long p2 = 1;

	for (auto v : l) {
		if (v == json::parse("[[2]]")) p2 *= index;
		if (v == json::parse("[[6]]")) p2 *= index;
		++index;
	}

	std::cout << "Second: " << p2 << '\n';
}
