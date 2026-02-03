pub fn build_proverb(list: &[&str]) -> String {
    let mut output = String::new();

    if list.is_empty() {
        return output;
    }

    let mut i = 0;
    let len = list.len() - 1;
    while i < len {
        output += format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]).as_str();
        i += 1;
    }

    output += format!("And all for the want of a {}.", list[0]).as_str();

    output
}
