package santorini

sealed abstract class Direction(val drow: Int, val dcol: Int) {
  def d: (Int, Int) = (drow, dcol)
}

case object North extends Direction(-1, 0)
case object Northeast extends Direction(-1, 1)
case object East extends Direction(0, 1)
case object Southeast extends Direction(1, 1)
case object South extends Direction(1, 0)
case object Southwest extends Direction(1, -1)
case object West extends Direction(0, -1)
case object Northwest extends Direction(-1, -1)
