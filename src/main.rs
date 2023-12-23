#![feature(int_roundings)]
use recursion::*;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Num {
    Int(u32),
    Root(u32, u32),
}

enum Expr {
    Pow(Box<Expr>, Box<Expr>),
    Literal(Num),
}
fn pow(a: Expr, b: Expr) -> Expr {
    Expr::Pow(Box::new(a), Box::new(b))
}
fn literal(n: Num) -> Expr {
    Expr::Literal(n)
}

pub enum ExprFrame<A> {
    Pow(A, A),
    Literal(Num),
}

impl MappableFrame for ExprFrame<PartiallyApplied> {
    type Frame<X> = ExprFrame<X>;
    fn map_frame<A, B>(input: Self::Frame<A>, mut f: impl FnMut(A) -> B) -> Self::Frame<B> {
        match input {
            ExprFrame::Pow(a, b) => ExprFrame::Pow(f(a), f(b)),
            ExprFrame::Literal(x) => ExprFrame::Literal(x),
        }
    }
}
impl<'a> Collapsible for &'a Expr {
    type FrameToken = ExprFrame<PartiallyApplied>;
    fn into_frame(self) -> <Self::FrameToken as MappableFrame>::Frame<Self> {
        match self {
            Expr::Pow(a, b) => ExprFrame::Pow(a, b),
            Expr::Literal(x) => ExprFrame::Literal(*x),
        }
    }
}
fn eval(e: &Expr) -> Num {
    use Num::*;
    e.collapse_frames(|frame: ExprFrame<Num>| match frame {
        ExprFrame::Pow(a, b) => match (a, b) {
            (Int(x), Int(y)) => Int(x.pow(y)),
            (Root(base, root), Int(pow)) => {
                // 2^(1/2) ^ 4 --> root=2, pow=4
                if pow % root == 0 {
                    let int_pow = pow.div_floor(root);
                    Int(base.pow(int_pow))
                } else {
                    todo!();
                }
            }
            _ => Int(1),
        },
        ExprFrame::Literal(x) => x,
    })
}

fn main() {
    use Num::*;
    let expr = pow(literal(Int(2)), literal(Int(2)));
    assert_eq!(eval(&expr), Int(4));
    let expr = pow(literal(Root(2, 2)), literal(Int(2)));
    assert_eq!(eval(&expr), Int(2));
}
