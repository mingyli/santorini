package santorini

import org.scalatest.WordSpec

class TileSpec extends WordSpec {
  "A Tile" when {
    "is height 0" should {
      "be height 0" in {
        val tile = new Tile()
        assert(tile.height === 0)
      }
      "increase height on build" in {
        val tile = new Tile()
        tile.build()
        assert(tile.height === 1)
      }
    }
    "is height 3" should {
      "be domed on build" in {
        val tile = new Tile()
        tile.height = 3
        tile.build()
        assert(tile.domed === true)
      }
    }
    "is domed" should {
      "throw Exception on build" in {
        val tile = new Tile()
        tile.domed = true
        assertThrows[Exception] {
          tile.build()
        }
      }
    }
  }
}
