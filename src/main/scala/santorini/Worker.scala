package santorini

class Worker(position: (Int, Int) = (-1, -1), color: String = Console.GREEN) {
  def position_= (newPosition: (Int, Int)): Unit = {
    println(s"Setting old position ${position} to new position ${newPosition}")
  }

  override def toString: String = s"${color}Worker${position}${Console.RESET}" 
}
