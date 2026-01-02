object SumOfMultiples {

    fun sum(factors: Set<Int>, limit: Int): Int {
        var numbers = mutableSetOf<Int>()
        var sum = 0
        for(f in factors) {
            if(f < 1 || f >= limit) continue
            val upper = if(limit % f == 0) limit / f - 1 else limit / f
            for(m in 1..upper) {
                val num = m * f
                if(numbers.add(num)) sum += num
            }
        }

        return sum
    }
}
