import kotlin.math.hypot

object Darts {

    fun score(x: Number, y: Number): Int {
        var dist = hypot(x.toDouble(), y.toDouble())

        return when {
            dist > 10 -> 0
            dist > 5 -> 1
            dist > 1 -> 5
            else -> 10
        }
    }
}
