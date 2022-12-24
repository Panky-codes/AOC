#ifndef HELPERS_H_
#define HELPERS_H_

#include <algorithm>
#include <fstream>
#include <functional>
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

static inline bool found_str(std::string& str, const char* substr) {
	return str.find(substr) != std::string::npos;
}

static inline std::string input_file() {
#ifdef DEMO
	return "input";
#else
	return "input_full";
#endif
}

template <typename T, typename V>
static std::vector<std::vector<T>> file_to_matrix(
    std::ifstream& file,
    std::function<T(V)> lambda = [](V in) -> T { return (T)in; }) {
	std::vector<std::vector<T>> in;

	int i = 0;
	std::string line;
	while (std::getline(file, line)) {
		in.push_back({});
		for (auto v : line) {
			in[i].push_back(lambda(v));
		}
		++i;
	}
	return in;
}
struct PairHash {
	template <typename T1, typename T2>
	auto operator()(const std::pair<T1, T2>& p) const -> size_t {
		return std::hash<T1>{}(p.first) ^ std::hash<T2>{}(p.second);
	}
};

}  // namespace helpers

#endif
