object Bob {
    fun hey(input: String): String {
        var isQuestion = input.trim().endsWith("?")
        var isSilence = input.isBlank()
        var isYelling = input.any { it.isLetter() } && input.all { !it.isLetter() || it.isUpperCase() }
        
        return when {
            isQuestion && isYelling -> "Calm down, I know what I'm doing!"
            isQuestion -> "Sure."
            isYelling -> "Whoa, chill out!"
            isSilence -> "Fine. Be that way!"
            else -> "Whatever."
        }
        
    }
}
