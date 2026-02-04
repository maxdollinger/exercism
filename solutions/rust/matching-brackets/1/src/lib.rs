pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = vec![];

    for c in string.chars() {
        match c {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            ')' | ']' | '}' if stack.pop() != Some(c) => return false,
            _ => continue,
        }
    }

    stack.is_empty()
}
