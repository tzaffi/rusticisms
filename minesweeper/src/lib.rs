pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let n = minefield.len();
    let mut annotated = Vec::with_capacity(n);

    if n == 0 {
        return annotated;
    }

    let m = minefield[0].len();
    for (i, row) in minefield.iter().enumerate() {
        annotated.push(String::with_capacity(m));
        for (j, c) in row.chars().enumerate() {
            if c == '*' {
                annotated[i].push('*');
                continue;
            }
            let mut mines: u8 = 0;
            for x in (i.saturating_sub(1))..=(usize::min(i + 1, n.saturating_sub(1))) {
                for y in (j.saturating_sub(1))..=(usize::min(j + 1, m.saturating_sub(1))) {
                    if x == i && y == j {
                        continue;
                    }
                    if minefield[x].chars().nth(y).unwrap_or(' ') == '*' {
                        mines += 1;
                    }
                }
            }
            annotated[i].push(if mines == 0 {
                ' '
            } else {
                (mines + 48) as char
            });
        }
    }
    annotated
}
