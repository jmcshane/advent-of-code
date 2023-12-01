val input = sc.textFile("aoc-1.txt")
input.map(line => line.filter(_.isDigit)).map(line => line.take(1).concat(line.takeRight(1)).toInt).collect().sum
