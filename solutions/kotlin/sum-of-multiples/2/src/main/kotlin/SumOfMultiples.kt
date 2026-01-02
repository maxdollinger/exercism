object SumOfMultiples {

    fun sum(factors: Set<Int>, limit: Int): Int {
        var divisors = factors.filter{ it > 0 }
        var sum = 0
        for(n in 1 until limit) {
            if(divisors.any{ n % it == 0 }) sum += n
        }

        return sum
    }
}
