fn student_index(student: &str) -> usize {
    match student {
        "Alice" => 0,
        "Bob" => 1,
        "Charlie" => 2,
        "David" => 3,
        "Eve" => 4,
        "Fred" => 5,
        "Ginny" => 6,
        "Harriet" => 7,
        "Ileana" => 8,
        "Joseph" => 9,
        "Kincaid" => 10,
        "Larry" => 11,
        _ => panic!("unknown student"),
    }
}

fn char_to_plant(c: char) -> &'static str {
    match c {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => panic!("unknown plant"),
    }
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let idx = student_index(student);
    diagram
        .lines()
        .flat_map(|line| line.chars().skip(idx * 2).take(2).map(char_to_plant))
        .collect()
}
