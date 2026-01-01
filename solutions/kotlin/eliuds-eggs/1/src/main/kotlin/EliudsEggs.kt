object EliudsEggs {

    fun eggCount(number: Int): Int{
        var cnt = 0
        var n = number
        while (n > 0) {
            cnt += n and 1
            n = n shr 1
        }

        return cnt
    }
}
