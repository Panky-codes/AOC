import 'dart:io';

void main() {
  var file = File("input.txt").readAsStringSync();
  RegExp reg_rule = RegExp(r"(([\w ]+): ((\d*)-(\d*))) or ((\d*)-(\d*))");
  Set<int> allRules = {};
  List<int> invalidTickets = [];
  int lineBreak = 0;
  for (var line in file.split("\n")) {
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
    }
    // Find the invalid tickets
    if (lineBreak == 2) {
      if (line == "nearby tickets:") continue;
      var nums = line.split(",").map((strnum) => int.parse(strnum));
      for (var num in nums) {
        if (!allRules.contains(num)) {
          invalidTickets.add(num);
        }
      }
    }
  }
  int sum = invalidTickets.fold(0, (previous, next) => previous + next);
  print(sum);
}
