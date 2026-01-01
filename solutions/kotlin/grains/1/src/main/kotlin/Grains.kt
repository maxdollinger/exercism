import java.math.BigInteger

object Board {

    fun getGrainCountForSquare(number: Int): BigInteger {
        require(number in 1..64) { "square must be between 1 and 64" }
        return BigInteger.TWO.pow(number - 1)
    }

    fun getTotalGrainCount(): BigInteger {
        return BigInteger.TWO.pow(64) - BigInteger.ONE
    }
}
