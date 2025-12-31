object CollatzCalculator {
    fun computeStepCount(start: Int): Int {
        require(start >= 1) { "Must provide a positive integer"}

        var steps = 0
        var n = start
        while (n != 1) {
            n = if (n % 2 == 0) n / 2 else n * 3 + 1
            steps++
        }

        return steps
    }
}
