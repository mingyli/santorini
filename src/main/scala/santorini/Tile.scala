package santorini

class Tile() {
  var height: Int = 0
  var domed: Boolean = false

  @throws(classOf[Exception])
  def build(): Unit = {
    if (domed)
      throw new Exception("Tile cannot be built on")
    else if (height == 3)
      domed = true
    else
      height += 1
  }

  override def toString: String = {
    if (domed)
      s"${Console.MAGENTA}$height${Console.RESET}"
    else
      s"$height"
  }
}
