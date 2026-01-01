object ETL {
    fun transform(source: Map<Int, Collection<Char>>): Map<Char, Int> {
        var m = mutableMapOf<Char, Int>()
        for ((value,chars) in source) {
            for (c in chars) {
                m[c.lowercaseChar()] = value
            }
        }

        return m
    }
}
