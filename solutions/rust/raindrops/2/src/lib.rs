pub fn raindrops(n: u32) -> String {
    let mut result: String = String::new();

    if n.is_multiple_of(3) {
        result += "Pling";
    }
    if n.is_multiple_of(5) {
        result += "Plang";
    }
    if n.is_multiple_of(7) {
        result += "Plong";
    }

    if result.is_empty() {
        result = n.to_string();
    }

    result
}
