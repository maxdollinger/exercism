class Dna(val dna: String) {

    init {
        require(dna.all { it in "ACGT" }) { "Invalid DNA sequence, only A C G T allowed"}
    }

    val nucleotideCounts: Map<Char, Int>
        get() = "ACGT".associateWith { nuc -> dna.count { it == nuc } }
}
