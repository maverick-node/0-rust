#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 0 || self.v == 1 {
            return None;
        }

        let current = *self;

        if self.v % 2 == 0 {
            self.v /= 2;
        } else {
            self.v = 3 * self.v + 1;
        }

        Some(current)
    }
}

pub fn collatz(n: u64) -> usize {
    if n == 0 {
        return 0;
    }
    Collatz::new(n).count()
}
