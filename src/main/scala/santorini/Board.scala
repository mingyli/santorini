package santorini

object Board {
  val Width = 5

  // val grid = Array.ofDim[Tile](Width, Width)
  // for (i <- 0 until Width; j <- 0 until Width)
  //   grid(i)(j) = new Tile()
  val grid = Array.fill[Tile](Width, Width)(new Tile())

  override def toString: String = grid.map(_.mkString).mkString("\n")
}
