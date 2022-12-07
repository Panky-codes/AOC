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

// Move the helpers to a library
int main(int argc, char** argv) {
	std::string line;
	std::ifstream file{helpers::input_file()};

	int ans1 = 0;
	int ans2 = 0;
	constexpr size_t max_size = 70000000;
	constexpr size_t req_space = 30000000;
	size_t total_size = 0;
	size_t free_space = 0;
	std::map<std::string, size_t> folder_size{};
	std::vector<std::string> cur_dir;
	std::vector<size_t> part2_candidates{};

	while (std::getline(file, line)) {
		helpers::Tokens_t token = helpers::splitstr(line);
		if (token[0] == "$" && token[1] == "cd") {
			if (token[2] == "..") {
				cur_dir.pop_back();
			} else {
				cur_dir.push_back(token[2]);
			}
			continue;
		}
		if (token[0] == "$" && token[1] == "ls") continue;

		if (token[0] != "dir") {
			std::string tmp1 = "";
			size_t file_size = std::stoi(token[0]);
			total_size += file_size;
			for (int i = 1; i < cur_dir.size(); ++i) {
				tmp1 += "/" + cur_dir[i];

				folder_size[tmp1] += file_size;
			}
		}
	}

	free_space = max_size - total_size;
	for (auto v : folder_size) {
		if (v.second <= 100000) ans1 += v.second;
		if (free_space + v.second >= req_space)
			part2_candidates.push_back(v.second);
	}

	std::cout << "First: " << ans1 << " Count:" << folder_size.size()
		  << " Total size: " << total_size << '\n';
	std::cout << "Second: "
		  << *std::min_element(part2_candidates.begin(),
				       part2_candidates.end())
		  << '\n';
}
