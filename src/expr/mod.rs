pub mod number;

use std::fmt;

trait Expr: fmt::Display {
    fn reduce(&self) -> Option<Box<Expr>>;
}
