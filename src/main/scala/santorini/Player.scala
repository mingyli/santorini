package santorini

import collection.mutable
import collection.breakOut

class Player(color: String) {
  private val numWorkers = 2
  val workers: Set[Worker] = (for (i <- 1 to numWorkers) yield new Worker(color = color))(breakOut)

  def winCondition(): Boolean = {
    true
  }
}
