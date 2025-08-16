#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    items: Vec<(String,f32)>,
    receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart{
    items: Vec::new(),
    receipt: Vec::new(),
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        for i in &s.products {
            if i.0== ele {
                self.items.push(i.clone());
                self.receipt.push(i.1.into());
            }
        }
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        self.receipt.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = self.receipt.len();
        let cal= len / 3;
        if cal > 0 {
            let total_price: f32 = self.receipt.iter().sum();
            let free_price: f32 = self.receipt[..cal].iter().sum();
            let dprice: f32 = total_price - free_price;

            let ratio = dprice / total_price;
            self.receipt.iter_mut().for_each(|price| {
                *price = (*price * ratio * 100.0).round() / 100.0;
            });
        }
        self.receipt.clone()
    
    }
}