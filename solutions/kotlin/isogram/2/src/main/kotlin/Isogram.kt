object Isogram {

    fun isIsogram(input: String): Boolean {
        var letters = mutableSetOf<Char>()

        for(char in input.lowercase()) {
            if(char.isLetter() && !letters.add(char)) return false
        }

        return true
    }
}
