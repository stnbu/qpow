enum Expr<'a> {
    Int(u32),
    Operation(Box<Oper<'a>>),
}

enum Oper<'a> {
    Add(&'a Expr<'a>, &'a Expr<'a>),
}

fn main() {
    let x = Expr::Int(3);
    let y = Expr::Int(4);
    let blah = Oper::Add(&x, &y);
    let z = Expr::Operation(Box::new(blah));
    let z = Oper::Add(&z, &x);
}

fn add<'t>(a: &'t Expr<'t>, b: &'t Expr<'t>) -> Expr<'t> {
    Expr::Operation(Box::new(Oper::Add(&a, &b)))
}
