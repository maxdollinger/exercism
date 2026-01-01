fun transcribeToRna(dna: String): String = buildString {
    dna.forEach { 
        val nuc = when(it) {
            'G' -> 'C'
            'C' -> 'G'
            'T' -> 'A'
            'A' -> 'U'
            else -> throw IllegalArgumentException("Unknown nucleodite")
        }
        append(nuc)
    }
}