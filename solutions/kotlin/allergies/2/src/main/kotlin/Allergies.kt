class Allergies(private val score: Int) {
    fun getList(): List<Allergen> = Allergen.entries.filter { isAllergicTo(it) }
    
    fun isAllergicTo(allergen: Allergen): Boolean = score and allergen.score != 0
}