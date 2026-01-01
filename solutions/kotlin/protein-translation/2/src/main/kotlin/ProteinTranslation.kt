fun translate(rna: String?): List<String> {
    if (rna == null) return emptyList()
    
    var aminoAcids = mutableListOf<String>()
    for(codon in rna.chunked(3)) {
        val acid = when(codon) {
            "AUG" -> "Methionine"
            "UUU", "UUC" -> "Phenylalanine"
            "UUA", "UUG" -> "Leucine"
            "UCU", "UCC", "UCA", "UCG" -> "Serine"
            "UAU", "UAC" -> "Tyrosine"
            "UGU", "UGC" -> "Cysteine"
            "UGG" -> "Tryptophan"
            "UAA", "UAG", "UGA" -> break
            else -> throw IllegalArgumentException("Invalid Codon")
        }

        aminoAcids.add(acid)
    }

    return aminoAcids
}
