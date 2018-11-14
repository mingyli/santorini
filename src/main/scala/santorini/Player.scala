package santorini

import collection.mutable
import collection.breakOut

class Player(color: String = Console.BLUE) {
  val NumWorkers = 2

  val workers: Set[Worker] =
    Iterator.fill(NumWorkers)(new Worker(color = color)).toSet

}
