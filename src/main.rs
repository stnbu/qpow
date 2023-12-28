use std::fmt;

use num_integer::Roots;

enum Expr<'a> {
    Int(u32),
    Root(u32, u32),
    Op(Box<Operation<'a>>),
}

impl fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{i}"),
            Self::Root(m, p) => write!(f, "{m}^(1/{p})"),
            Self::Op(op) => write!(f, "({op})"),
        }
    }
}

enum Operation<'a> {
    Add(&'a Expr<'a>, &'a Expr<'a>),
    Mul(&'a Expr<'a>, &'a Expr<'a>),
}

impl fmt::Display for Operation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add(a, b) => {
                write!(f, "{a} + {b}")
            }
            Self::Mul(a, b) => {
                write!(f, "{a} × {b}")
            }
        }
    }
}

fn main() {
    let three = Expr::Int(3);
    let four = Expr::Int(4);
    let _ = add(&three, &four);

    let root2 = Expr::Root(2, 2);
    let wat = add(&three, &root2);

    let root3 = Expr::Root(3, 2);
    let wot = add(&three, &root3);

    let xx = mul(&wat, &wot);
    println!("{xx}");
}

fn add<'t>(a: &'t Expr<'t>, b: &'t Expr<'t>) -> Expr<'t> {
    use Expr::*;
    match (a, b) {
        (Int(a), Int(b)) => Int(a + b),
        (a, b) => Op(Box::new(Operation::Add(&a, &b))),
    }
}

fn mul<'t>(a: &'t Expr<'t>, b: &'t Expr<'t>) -> Expr<'t> {
    use Expr::*;
    match (a, b) {
        (Int(x), Int(y)) => Int(x * y),
        _ => Op(Box::new(Operation::Mul(&a, &b))),
    }
}
