use std::f64::consts::PI;

use lazy_static::lazy_static;

lazy_static! {
    static ref AL: f64 = 3.14f64.ln();
    static ref BL: f64 = (22.0f64 / 7.0).ln();
    static ref BA: f64 = *BL - *AL;
}
fn calc(p: f64) -> f64 {
    ((0.5 + (*BA * p).exp() / 2.0).ln() / p + *AL).exp()
}

fn search() -> f64 {
    let (mut high, mut low) = (10000e0, 1.0);
    while high - low > 1e-6 {
        let mid = (high + low) / 2.0;
        if calc(mid) > PI {
            high = mid;
        } else {
            low = mid;
        }
    }
    (high + low) / 2.0
}

fn add_funds1(mut a: Account, add: u64) -> Account {
    a.funds += add;
    a
}

struct Account {
    name: String,
    funds: u64,
}

fn add_funds<'a>(a: &'a mut Account, add: u64) {
    a.funds += add;
}

struct Funds<'a> {
    account: &'a mut Account,
}

impl<'a> Funds<'a> {
    fn add(self, add: u64) {
        self.account.funds += add;
    }
}
fn funds<'a>(account: &'a mut Account) -> Funds<'a> {
    Funds { account }
}

struct Transfer<'a> {
    to: &'a mut Account,
    amount: u64,
}

impl<'a> Transfer<'a> {
    fn commit(self) {
        self.to.funds += self.amount;
    }
}

fn transfer<'a, 'b>(from: &'a mut Account, to: &'b mut Account, amount: u64) -> Transfer<'b> {
    from.funds -= amount;
    Transfer { to, amount }
}

#[allow(unused_variables)]
fn foo1() {
    let name = "Oleg".to_string();
    let a = Account { name, funds: 25 };
    let a1 = add_funds1(a, 5);

    let a2: Account = add_funds1(a1, 5);
}

fn foo() {
    let mut a = Account { name: "Oleg".to_string(), funds: 1000 };
    let mut b = Account { name: "Vasya".to_string(), funds: 1000 };

    let t = transfer(&mut a, &mut b, 10);

    funds(&mut a).add(5);

    t.commit();

    funds(&mut b).add(5);
}
