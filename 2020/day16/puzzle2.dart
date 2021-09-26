import 'dart:collection';
import 'dart:io';

void addToIndexClassMap(
    Map<int, Set<String>> indexClassMap, int ind, String key) {
  indexClassMap.putIfAbsent(ind, () => new Set<String>());
  indexClassMap[ind].add(key);
}

void puzzle2(final List<List<int>> validTickets,
    final Map<String, List<int>> rulesMap, List<int> yourTickets) {
  var probRules = new Map<String, List<int>>();
  var indexClassMap = new Map<int, Set<String>>();
  var col_size = validTickets.length;
  print(rulesMap.length);

  for (var row in validTickets) {
    int ind = 0;
    for (var ele in row) {
      rulesMap.forEach((key, valList) {
        if (valList.contains(ele)) {
          probRules.putIfAbsent(key, () => List<int>.filled(row.length, 0));
          if (++probRules[key][ind] == col_size) {
            addToIndexClassMap(indexClassMap, ind, key);
          }
        }
      });
      ind++;
    }
  }
  var sortedindexMap = SplayTreeMap.from(
      indexClassMap,
      (key1, key2) =>
          indexClassMap[key1].length.compareTo(indexClassMap[key2].length));

  var keys = sortedindexMap.keys.toList();

  // fold operation
  var mul = 1;
  for (var i = 0; i < keys.length - 1; ++i) {
    Set<String> s =
        sortedindexMap[keys[i + 1]].difference(sortedindexMap[keys[i]]);
    if (s.first.contains("departure")) {
      mul *= yourTickets[keys[i + 1]];
    }
  }
  print(mul);
}

void main() {
  var file = File("input.txt").readAsStringSync();
  RegExp reg_rule = RegExp(r"(([\w ]+): ((\d*)-(\d*))) or ((\d*)-(\d*))");
  var rulesMap = Map<String, List<int>>();
  Set<int> allRules = {};
  List<int> invalidTickets = [];
  List<int> yourTickets = [];
  List<List<int>> validTickets = [];
  int lineBreak = 0;
  int row = 0;
  for (var line in file.split("\n")) {
    validTickets.add([]);
    if (line == "") {
      lineBreak++;
      continue;
    }
    // Find the rules
    if (lineBreak == 0) {
      var matches = reg_rule.allMatches(line);
      var match = matches.elementAt(0);
      int r1 = int.tryParse(match.group(4) ?? "") ?? 0;
      int r2 = int.tryParse(match.group(5) ?? "") ?? 0;
      int r3 = int.tryParse(match.group(7) ?? "") ?? 0;
      int r4 = int.tryParse(match.group(8) ?? "") ?? 0;
      var list1 = [for (int i = r1; i <= r2; i += 1) i];
      var list2 = [for (int i = r3; i <= r4; i += 1) i];
      list1.addAll(list2);
      allRules.addAll(list1);
      rulesMap[match.group(2)] = list1;
    }
    if (lineBreak == 1) {
      if (line == "your ticket:") continue;
      var nums = line.split(",").map((strnum) => int.parse(strnum));
      yourTickets.addAll(nums);
    }
    // Find the invalid tickets
    if (lineBreak == 2) {
      if (line == "nearby tickets:") continue;
      var nums = line.split(",").map((strnum) => int.parse(strnum));
      for (var num in nums) {
        if (!allRules.contains(num)) {
          invalidTickets.add(num);
        } else {
          validTickets[row].add(num);
        }
      }
    }
    row++;
  }
  validTickets
      .removeWhere((row) => row.length == 0 || row.length < rulesMap.length);
  puzzle2(validTickets, rulesMap, yourTickets);
}
