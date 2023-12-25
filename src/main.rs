#![feature(int_roundings)]
use itertools::Itertools;
use num_rational::{Ratio, Rational};
use prime_factorization::Factorization;
use recursion::*;
use std::fmt;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Num {
    Rational(Ratio<u32>),
    Root(Ratio<u32>, u32),
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

pub fn root_factors(base: u32, root: u32) -> Vec<Num> {
    use Num::*;
    let base_factorization = Factorization::run(base).factors;
    let root_factorization = Factorization::run(root).factors;
    let mut result = vec![];
    for root_factor in root_factorization.iter() {
        for base_factor in base_factorization.iter().sorted().unique() {
            let count = base_factorization
                .iter()
                .filter(|&n| n == base_factor)
                .count() as u32;
            let remainder = count % root_factor;
            let quotient = count.div_floor(*root_factor);
            if quotient != 0 {
                let n = Rational(Ratio::new(quotient * base_factor, 1));
                result.push(n);
            }
            if remainder != 0 {
                let n = Root(Ratio::new(remainder * base_factor, 1), *root_factor);
                result.push(n);
            }
        }
    }
    result
}

fn eval(e: &Expr) -> Num {
    use Num::*;
    e.collapse_frames(|frame: ExprFrame<Num>| match frame {
        ExprFrame::Pow(a, b) => match (a, b) {
            (Rational(x), Rational(y)) => Rational(x.pow(*y.numer() as i32)), // TODO
            (Root(base, root), Rational(pow)) => {
                // 2^(1/2) ^ 4 --> root=2, pow=4
                if pow.numer() % root == 0 {
                    let int_pow = pow.numer().div_floor(root) as i32;
                    Rational(base.pow(int_pow))
                } else {
                    todo!();
                }
            }
            _ => Rational(Ratio::new(1, 1)),
        },
        ExprFrame::Literal(x) => x,
    })
}

fn main() {
    for (m, n) in &[(12, 3), (12, 2), (12, 6), (2, 12), (1441440, 1441440)] {
        let r = root_factors(*m, *n);
        println!("{r:?}");
    }
}
