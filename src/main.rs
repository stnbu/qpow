use std::fmt;

enum Expr<'a> {
    Int(u32),
    Operation(Box<Oper<'a>>),
}

impl fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{i}"),
            Self::Operation(op) => write!(f, "{op}"),
        }
    }
}

enum Oper<'a> {
    Add(&'a Expr<'a>, &'a Expr<'a>),
}

impl fmt::Display for Oper<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add(a, b) => {
                write!(f, "{a} + {b}")
            }
        }
    }
}

fn main() {
    let x = Expr::Int(3);
    let y = Expr::Int(4);
    let z = add(&x, &y);
    println!("{z}");
}

fn add<'t>(a: &'t Expr<'t>, b: &'t Expr<'t>) -> Expr<'t> {
    Expr::Operation(Box::new(Oper::Add(&a, &b)))
}
