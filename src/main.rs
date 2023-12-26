use num_traits::*;
/*
struct ExtendedInt {
    integer: Option<u32>,
    root: Option<(u32, u32)>,
}
 */

enum ExtendedInt {
    Int(u32),
    Root((u32, u32)),
    IExpr((Box<ExtendedInt>, Box<ExtendedInt>, Operation)),
}
enum Operation {
    Mul,
    Ident,
}

struct Expr {
    a: ExtendedInt,
    b: ExtendedInt,
    oper: Operation,
}

fn eval(expr: Expr) -> ExtendedInt {
    match expr {
        Expr {
            a: _,
            b: _,
            oper: Operation::Mul,
        } => ExtendedInt::Int(777),
        _ => ExtendedInt::Int(666),
    }
}

fn main() {
    let _1 = ExtendedInt::Int(1);
    let _2 = ExtendedInt::Int(2);
    let e = Expr {
        a: _1,
        b: _2,
        oper: Operation::Mul,
    };
    let r = eval(e);
}
