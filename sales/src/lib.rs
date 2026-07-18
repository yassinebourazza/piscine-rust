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
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some(item) = s.products.iter().find(|x| x.0 == ele) {
            self.items.push(item.clone());
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.items.iter().map(|x| x.1).collect();

        let total: f32 = prices.iter().sum();

        let mut free = 0.0;

        let mut sorted = prices.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for group in sorted.chunks(3) {
            if group.len() == 3 {
                free += group[0];
            }
        }

        let ratio = (total - free) / total;

        prices.iter_mut().for_each(|x| {
            *x = ((*x * ratio) * 100.0).round() / 100.0;
        });

        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        self.receipt = prices.clone();
        return prices
    }
}