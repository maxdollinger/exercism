object Isogram {

    fun isIsogram(input: String): Boolean {
        var seen = 0
        for(c in input.lowercase()) {
            if(c.isLetter() && seen and (1 shl c.code) != 0) return false
            seen = seen or (1 shl c.code)
        }
        return true
    }
}
