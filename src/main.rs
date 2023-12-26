pub enum TResult {
    Int(i64),
    IR(Box<Expr>),
}

macro_rules! impl_arith {
    ($trait:ident, $func:ident) => {
        impl std::ops::$trait<TResult> for TResult {
            type Output = TResult;

            fn $func(self, other: TResult) -> TResult {
                match (self, other) {
                    (TResult::Int(a), TResult::Int(b)) => TResult::Int(a.$func(b)),
                    (_, TResult::IR(_)) => todo!(),
                    (TResult::IR(_), _) => todo!(),
                }
            }
        }
    };
}

impl_arith!(Add, add);
impl_arith!(Sub, sub);
impl_arith!(Mul, mul);

enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    LiteralInt(TResult),
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
fn literal(n: TResult) -> Expr {
    Expr::LiteralInt(n)
}

pub enum ExprFrame<A> {
    Add(A, A),
    Sub(A, A),
    Mul(A, A),
    LiteralInt(TResult),
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
            Expr::LiteralInt(x) => ExprFrame::LiteralInt(x),
        }
    }
}
fn eval(e: &Expr) -> TResult {
    use TResult::*;
    e.collapse_frames(|frame| match frame {
        ExprFrame::Add(Int(a), Int(b)) => TResult::Int(a + b),
        ExprFrame::Sub(Int(a), Int(b)) => TResult::Int(a - b),
        ExprFrame::Mul(Int(a), Int(b)) => TResult::Int(a * b),
        ExprFrame::LiteralInt(Int(x)) => TResult::Int(x),

        ExprFrame::Add(IR(_), _) => todo!(),
        ExprFrame::Sub(IR(_), _) => todo!(),
        ExprFrame::Mul(IR(_), _) => todo!(),

        ExprFrame::Add(_, IR(_)) => todo!(),
        ExprFrame::Sub(_, IR(_)) => todo!(),
        ExprFrame::Mul(_, IR(_)) => todo!(),

        ExprFrame::LiteralInt(IR(_)) => todo!(),
    })
}

fn main() {
    let _1 = TResult::Int(1);
    let _2 = TResult::Int(2);
    let _3 = TResult::Int(3);
    let __1 = TResult::Int(1);
    let expr = add(
        multiply(subtract(literal(_1), literal(_2)), literal(_3)),
        literal(__1),
    );
}
/*

*/
