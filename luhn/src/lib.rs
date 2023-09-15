/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut invalid = false;
    let digitize = |c: char| -> Option<u32> {
        match c {
            ' ' => None,
            _ => {
                let d = c.to_digit(10);
                if d.is_none() {
                    invalid = true;
                }
                d
            }
        }
    };

    let digits = code.chars().filter_map(digitize).collect::<Vec<u32>>();
    if invalid || digits.len() <= 1 {
        return false;
    }

    digits
        .into_iter()
        .rev()
        .enumerate()
        .map(summand)
        .sum::<u32>()
        % 10
        == 0
}

fn summand(i_d: (usize, u32)) -> u32 {
    let (i, d) = i_d;
    match i % 2 {
        0 => d,
        _ => {
            let e = 2 * d;
            e / 10 + e % 10
        }
    }
}
