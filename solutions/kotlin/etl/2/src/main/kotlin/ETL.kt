object ETL {
    fun transform(source: Map<Int, Collection<Char>>): Map<Char, Int> = buildMap {
        source.forEach { (v, chars) ->
            chars.forEach { c -> put(c.lowercaseChar(), v)}
        }
    }
}
