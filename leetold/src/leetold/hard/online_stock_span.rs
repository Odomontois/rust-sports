#[derive(Debug, Clone, Copy)]
struct Item {
    idx: i32,
    price: i32,
}

#[derive(Default)]
struct StockSpanner {
    idx: i32,
    prices: Vec<Item>,
}

impl StockSpanner {
    fn new() -> Self {
        Self::default()
    }

    fn next(&mut self, price: i32) -> i32 {
        let drop = self.prices.iter().rev().take_while(|i| i.price <= price).count();
        self.prices.drain(self.prices.len() - drop..);
        let prev = self.prices.last().map_or(0, |x| x.idx);
        self.idx += 1;
        self.prices.push(Item { idx: self.idx, price });
        self.idx - prev
    }
}
