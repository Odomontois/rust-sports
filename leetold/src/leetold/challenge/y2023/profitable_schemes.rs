use std::collections::HashMap;

pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
    let schemes = group.len() as u8;
    Schemes {
        group,
        profit,
        cache: HashMap::new(),
    }
    .calc(SchemeParams {
        people: n as u8,
        profit: min_profit as u8,
        schemes,
    })
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct SchemeParams {
    people: u8,
    profit: u8,
    schemes: u8,
}

struct Schemes {
    group: Vec<i32>,
    profit: Vec<i32>,
    cache: HashMap<SchemeParams, i32>,
}

impl Schemes {
    fn calc(&mut self, params: SchemeParams) -> i32 {
        if params.schemes == 0 {
            return if params.profit == 0 { 1 } else { 0 };
        }
        if params.people == 0 {
            return 0;
        }
        if let Some(&count) = self.cache.get(&params) {
            return count;
        }

        let mut count = self.calc(SchemeParams {
            schemes: params.schemes - 1,
            ..params
        });
        let i = params.schemes as usize - 1;

        if params.people >= self.group[i] as u8 {
            count += self.calc(SchemeParams {
                people: params.people - self.group[i] as u8,
                profit: params.profit.saturating_sub(self.profit[i] as u8),
                schemes: params.schemes - 1,
            });
            count %= 1_000_000_007;
        }

        self.cache.insert(params, count);
        count
    }
}
