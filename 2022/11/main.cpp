#include <algorithm>
#include <cassert>
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

struct monkey_attr_t {
	std::vector<unsigned long> items{};
	int op;	      // 0 for mul, 1 for add
	int op_with;  // neg to indicate old
	int div;
	int true_monkey;
	int false_monkey;
	size_t counter;
};

size_t modulo(std::vector<monkey_attr_t> monkeys) {
	auto mod = 1;
	for (auto m : monkeys) mod *= m.div;
	return mod;
}

void fill_data_struct(std::string& t, int monkey_idx,
		      std::vector<monkey_attr_t>& monkeys) {
	if (helpers::found_str(t, "Monkey")) {
		assert(monkey_idx == std::stoi(&t[t.size() - 2]));
	}

	if (helpers::found_str(t, "Starting")) {
		helpers::Tokens_t token_colon = helpers::splitstr(t, ":");
		helpers::Tokens_t token_comma =
		    helpers::splitstr(token_colon[1], ",");

		for (auto l : token_comma)
			monkeys[monkey_idx].items.push_back(std::stoi(l));
	}

	if (helpers::found_str(t, "Operation")) {
		helpers::Tokens_t token_space = helpers::splitstr(t, " ");
		monkeys[monkey_idx].op =
		    (token_space[token_space.size() - 2] == "*") ? 0 : 1;

		monkeys[monkey_idx].op_with =
		    (token_space[token_space.size() - 1] == "old")
			? -1
			: std::stoi(token_space[token_space.size() - 1]);
	}

	if (helpers::found_str(t, "Test")) {
		helpers::Tokens_t token_space = helpers::splitstr(t, " ");

		monkeys[monkey_idx].div =
		    std::stoi(token_space[token_space.size() - 1]);
	}

	if (helpers::found_str(t, "true")) {
		helpers::Tokens_t token_space = helpers::splitstr(t, " ");

		monkeys[monkey_idx].true_monkey =
		    std::stoi(token_space[token_space.size() - 1]);
	}

	if (helpers::found_str(t, "false")) {
		helpers::Tokens_t token_space = helpers::splitstr(t, " ");

		monkeys[monkey_idx].false_monkey =
		    std::stoi(token_space[token_space.size() - 1]);
	}
}

unsigned long part(int nr_rounds, int part,
		   std::vector<monkey_attr_t> monkeys) {
	auto mod = modulo(monkeys);

	for (int i = 0; i < nr_rounds; ++i) {
		for (auto& v : monkeys) {
			auto tmp = v.items;
			for (auto w : tmp) {
				v.counter += 1;
				auto op_with = (v.op_with < 0) ? w : v.op_with;

				auto worry_level =
				    v.op ? (w + op_with) : (w * op_with);

				if (part == 1)
					worry_level /= 3;
				else
					worry_level = worry_level % mod;

				((worry_level % v.div) == 0)
				    ? monkeys[v.true_monkey].items.push_back(
					  worry_level)
				    : monkeys[v.false_monkey].items.push_back(
					  worry_level);
				v.items.erase(v.items.begin());
			}
		}
	}

	std::vector<unsigned long> count;
	for (auto& v : monkeys) {
		count.push_back(v.counter);
	}
	std::sort(count.begin(), count.end(), std::greater<unsigned int>());

	return (count[0] * count[1]);
}

int main(int argc, char** argv) {
	std::string line;
	std::ifstream file{helpers::input_file()};
	std::vector<monkey_attr_t> monkeys;
	int monkey_idx = 0;
	monkeys.push_back({});

	if (!file.good()) {
		std::cerr << "Input file not found" << std::endl;
		exit(1);
	}

	while (std::getline(file, line)) {
		helpers::Tokens_t token = helpers::splitstr(line, "\n");
		// Parse info
		for (auto t : token) {
			if (t == "") {
				monkey_idx++;
				monkeys.push_back({});
				continue;
			}
			fill_data_struct(t, monkey_idx, monkeys);
		}
	}

	printf("First %lu \n", part(20, 1, monkeys));
	printf("Second %lu \n", part(10000, 2, monkeys));
}
