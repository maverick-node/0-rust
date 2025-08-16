#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Self { numbers }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        self.numbers.last().copied()
    }

    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().copied().max()
    }
    pub fn highest_three(&self) -> Vec<u32> {
        let mut sorted = self.numbers.to_vec();
        sorted.sort(); 
        sorted.reverse();
        sorted[0..3].to_vec()
    }
}
