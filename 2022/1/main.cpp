#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

int main() {
	std::string line;
	std::ifstream file("input");
	std::vector<int> elves_cal;
	int sum = 0;

	while (std::getline(file, line)) {
		if (!line.empty()) {
			sum += std::stoi(line);
		} else {
			elves_cal.push_back(sum);
			sum = 0;
		}
	}
	std::cout << "First: "
		  << *std::max_element(elves_cal.begin(), elves_cal.end())
		  << '\n';

	std::sort(elves_cal.begin(), elves_cal.end(), std::greater<int>());

	auto it = elves_cal.begin();

	std::cout << "Second: " << (*it + *(it + 1) + *(it + 2)) << '\n';
}
