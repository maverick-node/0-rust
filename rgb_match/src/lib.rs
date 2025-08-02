#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        self.r = match self.r {
           1=> if r == first => second,
           r if r ==second => first,
           r => r
        };
           self.b = match self.b {
           b if b == first => second,
           b if b ==second => first,
           b => b
        };
            self.g = match self.g {
           g if g == first => second,
           g if g ==second => first,
           g => g
        };
            self.a = match self.a {
           a if a == first => second,
           a if a ==second => first,
           a => a
        };

      self
    }
}
