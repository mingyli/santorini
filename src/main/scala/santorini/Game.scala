package santorini

object Game extends App {
  def printState(): Unit = {
    println(Board)
    println()
  }

  override def main(args: Array[String]): Unit = {
    println("PLAYING SANTORINI")

    val player1 = new Player(Console.BLUE)
    val player2 = new Player(Console.RED)
    println(player1.workers)

    val worker1 = player1.workers.head
    worker1.position = (1, 1)
    val worker2 = player2.workers.head
    worker2.position = (2, 2)

    player1.workers foreach println
    player2.workers foreach println

    println(s"Board width: ${Board.Width}")
    printState
    println(s"Board width: ${Board.Width}")
    Board(2, 2).domed = true
    Board(1, 1).build
    printState()
    Board(1, 1).build
    printState()
    Board(1, 1).build
    printState()
    Board(1, 1).build
    printState()
  }
}
