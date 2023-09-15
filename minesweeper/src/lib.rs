pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.len() == 0 {
        return vec![];
    }
    let (n, m) = (minefield.len(), minefield[0].len());
    minefield
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '*' => '*',
                    _ => match (i.saturating_sub(1)..=(usize::min(i + 1, n.saturating_sub(1))))
                        .flat_map(|x| {
                            (j.saturating_sub(1)..=(usize::min(j + 1, m.saturating_sub(1))))
                                .map(move |y| (x, y))
                        })
                        .filter(|&(x, y)| !(x == i && y == j))
                        .filter(|&(x, y)| minefield[x].chars().nth(y).unwrap_or(' ') == '*')
                        .count() as u8
                    {
                        0 => ' ',
                        n => (n + b'0') as char,
                    },
                })
                .collect::<String>()
        })
        .collect()
}
