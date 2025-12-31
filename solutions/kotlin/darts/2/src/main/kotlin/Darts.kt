import kotlin.math.hypot

object Darts {

    fun score(x: Number, y: Number): Int {
        var dist = hypot(x.toDouble(), y.toDouble())

        return when(dist) {
            in 0f..1f -> 10
            in 1f..5f  -> 5
            in 5f..10f -> 1
            else -> 0
        }
    }
}
