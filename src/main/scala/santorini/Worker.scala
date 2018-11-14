package santorini

class Worker(var position: (Int, Int) = (-1, -1),
             val color: String = Console.GREEN) {

  @throws(classOf[Exception])
  def move(direction: Direction): Unit = {
    val (drow, dcol) = direction.d
    var (row, col) = position
    row += drow
    col += dcol
    if (Board.inBounds(row, col))
      position = (row, col)
    else
      throw new Exception(s"Cannot move worker to ($row, $col)")
  }

  override def toString: String = s"${color}Worker${position}${Console.RESET}"
}
