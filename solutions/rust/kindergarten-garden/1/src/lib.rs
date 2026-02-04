const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let mut lines = diagram.lines();
    let row1 = lines.next().unwrap();
    let row2 = lines.next().unwrap();

    row1.chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .zip(row2.chars().collect::<Vec<_>>().chunks(2))
        .map(|(row1, row2)| [row1[0], row1[1], row2[0], row2[1]])
        .nth(STUDENTS.iter().position(|&s| s == student).unwrap())
        .unwrap()
        .iter()
        .map(|c| match c {
            'G' => "grass",
            'C' => "clover",
            'R' => "radishes",
            'V' => "violets",
            _ => unreachable!(),
        })
        .collect()
}
