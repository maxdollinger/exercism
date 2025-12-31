fun reverse(input: String): String {
    var reversed = StringBuilder()
    for ( i in input.length -1 downTo 0) {
        reversed.append(input[i])
    }
    return reversed.toString()
}
