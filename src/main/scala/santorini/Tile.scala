package santorini

class Tile(height: Int = 0) {
  private val Dome: Int = -1

  override def toString: String = {
    Console.BLUE + height
  }
}
