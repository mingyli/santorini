package santorini

object Board {
  val BoardWidth = 5

  private val _grid = Array.ofDim[Tile](BoardWidth, BoardWidth)
  for (i <- 0 until BoardWidth; j <- 0 until BoardWidth)
    _grid(i)(j) = new Tile()

  override def toString: String = _grid.map(_.mkString).mkString("\n")
}
