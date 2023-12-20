enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    LiteralInt(i64),
}
fn add(a: Expr, b: Expr) -> Expr {
    Expr::Add(Box::new(a), Box::new(b))
}
fn subtract(a: Expr, b: Expr) -> Expr {
    Expr::Sub(Box::new(a), Box::new(b))
}
fn multiply(a: Expr, b: Expr) -> Expr {
    Expr::Mul(Box::new(a), Box::new(b))
}
fn literal(n: i64) -> Expr {
    Expr::LiteralInt(n)
}

pub enum ExprFrame<A> {
    Add(A, A),
    Sub(A, A),
    Mul(A, A),
    LiteralInt(i64),
}
use recursion::*;
impl MappableFrame for ExprFrame<PartiallyApplied> {
    type Frame<X> = ExprFrame<X>;
    fn map_frame<A, B>(input: Self::Frame<A>, mut f: impl FnMut(A) -> B) -> Self::Frame<B> {
        match input {
            ExprFrame::Add(a, b) => ExprFrame::Add(f(a), f(b)),
            ExprFrame::Sub(a, b) => ExprFrame::Sub(f(a), f(b)),
            ExprFrame::Mul(a, b) => ExprFrame::Mul(f(a), f(b)),
            ExprFrame::LiteralInt(x) => ExprFrame::LiteralInt(x),
        }
    }
}
impl<'a> Collapsible for &'a Expr {
    type FrameToken = ExprFrame<PartiallyApplied>;
    fn into_frame(self) -> <Self::FrameToken as MappableFrame>::Frame<Self> {
        match self {
            Expr::Add(a, b) => ExprFrame::Add(a, b),
            Expr::Sub(a, b) => ExprFrame::Sub(a, b),
            Expr::Mul(a, b) => ExprFrame::Mul(a, b),
            Expr::LiteralInt(x) => ExprFrame::LiteralInt(*x),
        }
    }
}
fn eval(e: &Expr) -> i64 {
    e.collapse_frames(|frame| match frame {
        ExprFrame::Add(a, b) => a + b,
        ExprFrame::Sub(a, b) => a - b,
        ExprFrame::Mul(a, b) => a * b,
        ExprFrame::LiteralInt(x) => x,
    })
}

fn main() {
    let expr = add(
        multiply(subtract(literal(1), literal(2)), literal(3)),
        literal(1),
    );
    assert_eq!(eval(&expr), -2);
}
/*

*/
