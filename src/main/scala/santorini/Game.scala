package santorini

object Game extends App {
  println("PLAYING SANTORINI")

  var player = new Player(Console.BLUE)
  println(player.workers)
  player.workers foreach println

  println(s"Board width: ${Board.Width}")
  println(Board)
}
