//0 nanoseconds by owayss
/// Check a Luhn checksum.
/// This is from https://exercism.org/tracks/rust/exercises/luhn/solutions/17cfd8003c244132af50dea839e16ff9
pub fn is_valid(code: &str) -> bool {
    // Lookup table for the increment to the checksum. The first 10 numbers are for the digits
    // at even-numbered positions, simply adding the digit. The second set of 10 are for the
    // odd-numbered positions, that double the digit and remove 9 if the result is greater than 9.
    const INCR_TABLE: [u32; 20] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, // normal digits
        0, 2, 4, 6, 8, 1, 3, 5, 7, 9, // doubled digits
    ];
    code.chars() // loop over the string
        .rev() // but start from the right so we double the correct digits
        .filter(|&c| c != ' ') // remove spaces
        .try_fold((0, 0), |(l, s), c| {
            // at each step, update the length and the checksum,
            c.to_digit(10) // error out on non-digit characters
                .ok_or("Invalid character in input string")
                .map(|d| (l + 1, s + INCR_TABLE[(d + 10 * (l & 1)) as usize]))
        })
        .map(|(len, sum)| sum % 10 == 0 && len > 1) // check the checksum and length
        .unwrap_or(false) // return the result, or `false` on error
}

/// Check a Luhn checksum.
pub fn is_valid_orig(code: &str) -> bool {
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
