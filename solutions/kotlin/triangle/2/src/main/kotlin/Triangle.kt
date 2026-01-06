class Triangle<T : Number>(val a: T, val b: T, val c: T) {
    init {
        val x = a.toDouble()
        val y = b.toDouble()
        val z = c.toDouble()
        require(x > 0 && y > 0 && z > 0) { "All sides must be positive" }
        require(x + y >= z && y + z >= x && x + z >= y) { 
            "Triangle inequality violated" 
        }
    }

    val isEquilateral: Boolean = a == b && b == c
    val isIsosceles: Boolean = a == b || a == c || b == c
    val isScalene: Boolean = !isEquilateral && !isIsosceles
}
