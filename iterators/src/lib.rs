#[derive(Copy, Clone)]
pub struct Collatz {
    pub n: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 1 {
            return None;
        }

        let current = *self;

        if self.n % 2 == 0 {
            self.n /= 2;
        } else {
            self.n = self.n * 3 + 1;
        }

        Some(current)
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Self { n }
    }
}

pub fn collatz(n: u64) -> usize {
    if n <= 1 {
        return 0;
    }

    Collatz::new(n).count()
}