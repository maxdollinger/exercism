class Allergies(val score: Int) {
    private val allergenScores = mapOf(
        Allergen.EGGS to 1,
        Allergen.PEANUTS to 2,
        Allergen.SHELLFISH to 4,
        Allergen.STRAWBERRIES to 8,
        Allergen.TOMATOES to 16,
        Allergen.CHOCOLATE to 32,
        Allergen.POLLEN to 64,
        Allergen.CATS to 128
    )
    
    fun getList(): List<Allergen> = buildList {
        for ((allergen, value) in allergenScores) {
            if (score and value != 0) {
                add(allergen)
            }
        }
    }
    
    fun isAllergicTo(allergen: Allergen): Boolean =
        allergenScores[allergen]?.let { value ->
            score and value != 0
        } ?: false
}