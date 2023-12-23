enum Expr {
    Pow(Box<Expr>, Box<Expr>),
    Int(u32),
}
fn pow(a: Expr, b: Expr) -> Expr {
    Expr::Pow(Box::new(a), Box::new(b))
}
fn int(n: u32) -> Expr {
    Expr::Int(n)
}

pub enum ExprFrame<A> {
    Pow(A, A),
    Int(u32),
}
use recursion::*;
impl MappableFrame for ExprFrame<PartiallyApplied> {
    type Frame<X> = ExprFrame<X>;
    fn map_frame<A, B>(input: Self::Frame<A>, mut f: impl FnMut(A) -> B) -> Self::Frame<B> {
        match input {
            ExprFrame::Pow(a, b) => ExprFrame::Pow(f(a), f(b)),
            ExprFrame::Int(x) => ExprFrame::Int(x),
        }
    }
}
impl<'a> Collapsible for &'a Expr {
    type FrameToken = ExprFrame<PartiallyApplied>;
    fn into_frame(self) -> <Self::FrameToken as MappableFrame>::Frame<Self> {
        match self {
            Expr::Pow(a, b) => ExprFrame::Pow(a, b),
            Expr::Int(x) => ExprFrame::Int(*x),
        }
    }
}
fn eval(e: &Expr) -> u32 {
    e.collapse_frames(|frame: ExprFrame<u32>| match frame {
        ExprFrame::Pow(a, b) => a.pow(b),
        ExprFrame::Int(x) => x,
    })
}

fn main() {
    let expr = pow(int(2), int(2));
    assert_eq!(eval(&expr), 4);
}
/*

*/