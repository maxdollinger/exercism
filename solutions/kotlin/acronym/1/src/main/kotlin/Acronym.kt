object Acronym {
    fun generate(phrase: String): String = buildString {
        var prevIsLetter = false
    
        for (c in phrase) {
            when {
                c == '\'' -> prevIsLetter = true
                c.isLetter() && !prevIsLetter -> {
                    append(c.uppercaseChar())
                    prevIsLetter = true
                }
                else -> prevIsLetter = c.isLetter()
            }
        }
    }
}
