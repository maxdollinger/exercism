pub fn raindrops(n: u32) -> String {
    let mut results: Vec<&str> = Vec::new();

    if n.is_multiple_of(3) {
        results.push("Pling");
    }
    if n.is_multiple_of(5) {
        results.push("Plang");
    }
    if n.is_multiple_of(7) {
        results.push("Plong");
    }

    if results.is_empty() {
        n.to_string()
    } else {
        results.join("")
    }
}
