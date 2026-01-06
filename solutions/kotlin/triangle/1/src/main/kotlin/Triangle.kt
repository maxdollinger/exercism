class Triangle<T : Number>(val x: T, val y: T, val z: T) {

    val a = x.toDouble()
    val b = y.toDouble()
    val c = z.toDouble()

    init {
        require(a > 0 && b > 0 && c > 0) { "All sides must be positive" }
        require(a + b >= c && b + c >= a && a + c >= b) { 
            "Triangle inequality violated" 
        }
    }

    val isEquilateral: Boolean = a == b && b == c
    val isIsosceles: Boolean = a == b || a == c || b == c
    val isScalene: Boolean = !isEquilateral && !isIsosceles
}
