#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(xs: &[T], ys: &[T]) -> Comparison {
    use Comparison::*;

    if xs == ys {
        return Equal;
    }

    let (smaller, larger) = if xs.len() < ys.len() {
        (xs, ys)
    } else {
        (ys, xs)
    };

    if sublist_impl(smaller, larger) {
        return if xs.len() < ys.len() {
            Sublist
        } else {
            Superlist
        };
    }

    Unequal
}

fn sublist_impl<T: PartialEq>(small: &[T], large: &[T]) -> bool {
    small.is_empty() || large.windows(small.len()).any(|window| window == small)
}
