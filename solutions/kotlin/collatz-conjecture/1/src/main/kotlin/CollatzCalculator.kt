object CollatzCalculator {
    fun computeStepCount(start: Int): Int {
        if (start <= 0) {
            throw IllegalArgumentException("Only positive integers are allowed")
        }
        
        var steps = 0
        var n = start
        while (n > 1) {
            n = if (n % 2 == 0) n / 2 else n * 3 + 1
            steps++
        }
        
        return steps
    }
}
