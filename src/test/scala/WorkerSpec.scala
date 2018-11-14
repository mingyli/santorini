package santorini

import org.scalatest.WordSpec

class WorkerSpec extends WordSpec {
  "A Worker" when {
    "moves in bounds" can {
      "move north" in {
        val worker = new Worker(position = (2, 2))
        worker.move(North)
        assert(worker.position === (1, 2))
      }
      "move east" in {
        val worker = new Worker(position = (2, 2))
        worker.move(East)
        assert(worker.position === (2, 3))
      }
      "move south" in {
        val worker = new Worker(position = (2, 2))
        worker.move(South)
        assert(worker.position === (3, 2))
      }
      "move west" in {
        val worker = new Worker(position = (2, 2))
        worker.move(West)
        assert(worker.position === (2, 1))
      }
    }
    "moves out of bounds" should {
      "throw Exception if it moves north" in {
        val worker1 = new Worker(position = (0, 2))
        assertThrows[Exception] {
          worker1.move(North)
        }
      }
      "throw Exception if it moves east" in {
        val worker2 = new Worker(position = (2, 4))
        assertThrows[Exception] {
          worker2.move(East)
        }
      }
      "throw Exception if it moves south" in {
        val worker3 = new Worker(position = (4, 2))
        assertThrows[Exception] {
          worker3.move(South)
        }
      }
      "throw Exception if it moves west" in {
        val worker4 = new Worker(position = (2, 0))
        assertThrows[Exception] {
          worker4.move(West)
        }
      }
    }
  }
}
