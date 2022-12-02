#include <algorithm>
#include <array>
#include <fstream>
#include <iostream>
#include <map>
#include <string>
#include <vector>

int main() {
	std::string line;
	std::ifstream file("input_full");
	int elves_points[3][3] = {{3, 0, 6}, {6, 3, 0}, {0, 6, 3}};
	std::map<std::string, int> elves_map{{"A", 0}, {"B", 1}, {"C", 2},
					     {"X", 0}, {"Y", 1}, {"Z", 2}};
	// For part 2
	// rock 1, paper 2, scissors 3
	int elves_points_decrypted[3][3] = {{3, 1, 2}, {1, 2, 3}, {2, 3, 1}};
	std::map<std::string, int> elves_map_decrypted{
	    {"X", 0}, {"Y", 3}, {"Z", 6}};

	int sum1 = 0;
	int sum2 = 0;

	while (std::getline(file, line)) {
		sum1 += elves_points[elves_map[line.substr(2)]]
				    [elves_map[line.substr(0, 1)]] +
			elves_map[line.substr(2)] + 1;

		sum2 += elves_points_decrypted[elves_map[line.substr(2)]]
					      [elves_map[line.substr(0, 1)]] +
			elves_map_decrypted[line.substr(2)];
	}
	std::cout << "First: " << sum1 << '\n';

	std::cout << "Second: " << sum2 << '\n';
}
