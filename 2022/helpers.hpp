#ifndef HELPERS_H_
#define HELPERS_H_

#include <algorithm>
#include <fstream>
#include <sstream>
#include <string>
#include <string_view>
#include <vector>

namespace helpers {
using Tokens_t = std::vector<std::string>;

static Tokens_t splitstr(std::string& str, std::string deli = " ") {
	int start = 0;
	int end = str.find(deli);
	Tokens_t token;
	while (end != -1) {
		token.push_back(str.substr(start, end - start));
		start = end + deli.size();
		end = str.find(deli, start);
	}
	token.push_back(str.substr(start, end - start));

	return token;
}

static inline bool found_str(std::string& str, std::string& substr) {
	return str.find(substr) != std::string::npos;
}

static inline std::string input_file() {
#ifdef DEMO
	return "input";
#else
	return "input_full";
#endif
}

template <typename T>
static std::vector<std::vector<T>> file_to_matrix(std::ifstream& file) {
	std::vector<std::vector<T>> in;

	int i = 0;
	std::string line;
	while (std::getline(file, line)) {
		in.push_back({});
		for (auto v : line) {
			if constexpr (std::is_integral<T>())
				in[i].push_back(std::stoi(std::string{v}));
			else
				in[i].push_back(v);
		}
		++i;
	}
	return in;
}

}  // namespace helpers

#endif