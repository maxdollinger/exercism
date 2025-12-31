object Isogram {

    fun isIsogram(input: String): Boolean {
        var letters = input.lowercase().filter { it.isLetter() }
        return letters.length == letters.toSet().size
    }
}
