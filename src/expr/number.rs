use super::Expr;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Number(f64);

impl super::Expr for Number {
    fn reduce(&self) -> Option<Box<Expr>> {
        None
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let &Number(n) = self;
        write!(f, "{}", n)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use expr::Expr;

    #[test]
    fn number_display() {
        let num = Number(123.0);
        assert_eq!("123", format!("{}", num));
    }

    #[test]
    fn number_expr_reduce() {
        let num = Number(123.0);
        if let Some(_) = num.reduce() {
            assert!(false, "Number cannot reduce")
        }
    }
}
