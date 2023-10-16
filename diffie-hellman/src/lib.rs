// use rand; // Is this considered secure????

pub fn private_key(p: u64) -> u64 {
    // TODO - fix this absolutely HORRIBLE impl!!!!
    p / 2
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    g.exp(a) % p
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    b_pub.exp(a) % p
}

pub struct Modulo {
    n: u64,
    modulo: u64,
}

impl Modulo {
    fn new(n: u64, modulo: u64) -> Modulo {
        Modulo { n, modulo }
    }
}

pub trait Exp {
    fn exp(&self, e: u64) -> Self;
}

impl Exp for u64 {
    fn exp(&self, e: u64) -> u64 {
        let mut rep: Vec<bool> = vec![];
        let mut ex = e;
        while ex > 0 {
            rep.push(ex % 2 == 1);
            ex /= 2;
        }
        rep.iter()
            .fold((*self, 1), |a: (u64, u64), elt: &bool| {
                (a.0 * a.0, if *elt { a.0 * a.1 } else { a.1 })
            })
            .1
    }
}

impl Exp for Modulo {
    fn exp(&self, e: u64) -> u64 {
        let mut rep: Vec<bool> = vec![];
        let mut ex = e;
        while ex > 0 {
            rep.push(ex % 2 == 1);
            ex /= 2;
        }
        rep.iter()
            .fold((*self, 1), |a: (u64, u64), elt: &bool| {
                (a.0 * a.0, if *elt { a.0 * a.1 } else { a.1 })
            })
            .1
    }
}
