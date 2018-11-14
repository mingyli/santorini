package santorini

object Board {
  val Width = 5
  private val grid = Array.fill[Tile](Width, Width)(new Tile())

  def apply(row: Int, col: Int): Tile = grid(row)(col)

  def update(row: Int, col: Int, tile: Tile): Unit = {
    grid(row)(col) = tile
  }

  def inBounds(row: Int, col: Int): Boolean =
    row >= 0 && row < Width && col >= 0 && col < Width

  override def toString: String = {
    val lines = grid map (_ mkString " ")
    lines mkString "\n"
  }
}
