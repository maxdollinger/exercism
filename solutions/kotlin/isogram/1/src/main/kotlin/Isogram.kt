object Isogram {

    fun isIsogram(input: String): Boolean {
        var letters = 0

        for(char in input) {
            if (char.isLetter()) {
                var n = char.lowercaseChar() - 'a'
                if (letters and (1 shl n) != 0) return false
                letters = letters or (1 shl n)
            }
        }

        return true
    }
}
