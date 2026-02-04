pub fn build_proverb(list: &[&str]) -> String {
    let mut output = list
        .windows(2)
        .map(|pair| format!("For want of a {} the {} was lost.\n", pair[0], pair[1]))
        .collect::<String>();

    if let Some(first) = list.first() {
        output.push_str(&format!("And all for the want of a {}.", first));
    }

    output
}
