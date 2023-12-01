val digits = List("one", "two", "three", "four", "five", "six", "seven", "eight", "nine")

def matchDigit(input:String):String = {
  val digitMatch = digits.filter(digit => input.contains(digit))
  if (digitMatch.length > 0) {
    (digits.indexOf(digitMatch(0)) + 1).toString
  } else {
    input
  }
}

def findFirstDigit(input:String) : Int = {
  input.foldLeft("")(( str: String, op : Char) => {
     if (str.length > 0 && str.forall(Character.isDigit(_))) {
          str
     } else if (op.isDigit) {
          op.toString
     } else {
          matchDigit(str + op)
     }
     }).toInt
}

val input = sc.textFile("aoc-1.txt")
input.map(line => findFirstDigit(line) * 10 + findFirstDigit(line.reverse)).collect().sum
