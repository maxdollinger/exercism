object EliudsEggs {

    fun eggCount(number: Int): Int = (0 until Int.SIZE_BITS).sumOf { number shr it and 1 }
}
