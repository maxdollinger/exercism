pub fn annotate(garden: &[&str]) -> Vec<String> {
    let rows: Vec<&[u8]> = garden.iter().map(|row| row.as_bytes()).collect();

    rows.iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(|(col_idx, &cell)| {
                    if cell == b'*' {
                        '*'
                    } else {
                        let count = count_adjacent_flowers(&rows, row_idx, col_idx);
                        if count == 0 {
                            ' '
                        } else {
                            char::from_digit(count as u32, 10).unwrap()
                        }
                    }
                })
                .collect()
        })
        .collect()
}

fn count_adjacent_flowers(rows: &[&[u8]], row: usize, col: usize) -> u8 {
    const DIRECTIONS: &[(i32, i32)] = &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    DIRECTIONS
        .iter()
        .map(|&(dr, dc)| {
            let r = (row as i32 + dr) as usize;
            let c = (col as i32 + dc) as usize;
            (r, c)
        })
        .map(|(r, c)| rows.get(r).and_then(|row| row.get(c)))
        .filter(|&cell| match cell {
            None => false,
            Some(&c) => c == b'*',
        })
        .count() as u8
}
