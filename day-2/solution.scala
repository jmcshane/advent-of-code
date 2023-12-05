import scala.collection.mutable.{Map => MutableMap}

@SerialVersionUID(100L)
class Draw(var count: Int, var color: String) extends Serializable {}

@SerialVersionUID(100L)
class Game(var index: Int, var draws: List[List[Draw]]) extends Serializable {}

val colors = List("red", "blue", "green")
def getFirstIntFromString(line: String) : Int = {
  ("""\d+""".r findAllIn line).toList(0).toInt
}

def getFirstColorFromString(line: String) : String = {
  (colors.mkString("|").r findAllIn line).toList(0)
}

def buildRestrictionMap(input: String) : MutableMap[String,Int]= {
  input.split(",").foldLeft(scala.collection.mutable.Map[String,Int]()) {
    (map, restriction) => map += (colors.mkString("|").r findAllIn restriction).toList(0) -> getFirstIntFromString(restriction)
  }
}

def getDraw(line: String) : List[Draw] = {
  line.split(",").map(input => new Draw(getFirstIntFromString(input), getFirstColorFromString(input))).toList
}

def buildDraws(line: String) : List[List[Draw]] = {
  line.split(";").map(input => getDraw(input)).toList
}

def buildGame(line: String) : Game = {
  val split = line.split(":")
  val index = getFirstIntFromString(split(0))
  val draws = buildDraws(split(1))
  new Game(index, draws)
}

def isFeasible(draw: List[Draw], rest : MutableMap[String, Int]) : Boolean = {
  draw.filter(selection => selection.count > rest(selection.color)).length == 0
}

def ifFeasibleReturnIndex(game: Game, rest : MutableMap[String, Int]): Int = {
  game.draws.filter(draw => !isFeasible(draw, rest)).length match {
    case 0 => game.index
    case _ => 0
  }
}

def drawCubePower(gameDraw : List[List[Draw]]) : Int = {
  gameDraw.foldLeft(MutableMap("green"->1, "red" -> 1, "blue" -> 1))((x, y) => {
    y.foreach(d => x.update(d.color, Math.max(x(d.color), d.count)))
    x
  }).foldLeft(1)((x, y) => x * y._2)
}

val restriction = sys.env.get("RESTRICTION").get
val restrictionMap = buildRestrictionMap(restriction)
sc.textFile(sys.env.get("INPUT").get)
  .map(line => buildGame(line))
  .aggregate(0)((sum, game) => sum + ifFeasibleReturnIndex(game,restrictionMap),(a,b) => a+b)

sc.textFile(sys.env.get("INPUT").get).map(line => buildGame(line)).aggregate(0)((sum, game) => sum + drawCubePower(game.draws),(a,b) => a+b)

