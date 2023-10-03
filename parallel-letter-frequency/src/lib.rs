use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let pool = ThreadPoolBuilder::new()
        .num_threads(worker_count)
        .build()
        .unwrap();

    pool.install(|| {
        input
            .par_iter()
            .map(|&line| _frequency(line))
            .reduce(HashMap::new, |mut acc, map| {
                for (c, &count) in map.iter() {
                    *acc.entry(*c).or_insert(0) += count;
                }
                acc
            })
    })
}

fn _frequency(input: &str) -> HashMap<char, usize> {
    input
        .chars()
        .filter(|c| c.is_alphabetic())
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
            acc
        })
}
