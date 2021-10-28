#![allow(dead_code, unused_variables)]

/// We start solving a very important problem:
///
///     Designing a language with only 3-letter types
///
#[derive(Clone, Debug, PartialEq)]
enum Val {
    Boo(bool), // 👻
    Int(u64),
    Nil,
    Rat(f64),
    Txt(String),
}

#[derive(Clone, Debug)]
enum Exp {
    Var(Val),
    Add(Box<Exp>, Box<Exp>),
    Sub(Box<Exp>, Box<Exp>),
    Mul(Box<Exp>, Box<Exp>),
    Div(Box<Exp>, Box<Exp>),
}

// Type-checked and error-handled `Var` expression
fn type_checked_var(var: Val) -> Option<Val> {
    Some(var)
}

// Type-checked and error-handled `Add` expression
fn type_checked_add(val1: Val, val2: Val) -> Option<Val> {
    match (val1, val2) {
        (Val::Int(val1), Val::Int(val2)) => Some(Val::Int(val1 + val2)),
        (Val::Int(val1), Val::Rat(val2)) => Some(Val::Rat(val1 as f64 + val2)),
        (Val::Rat(val1), Val::Int(val2)) => Some(Val::Rat(val1 + val2 as f64)),
        (Val::Rat(val1), Val::Rat(val2)) => Some(Val::Rat(val1 + val2)),
        _ => None,
    }
}

// Type-checked and error-handled `Sub` expression
fn type_checked_sub(val1: Val, val2: Val) -> Option<Val> {
    match (val1, val2) {
        (Val::Int(val1), Val::Int(val2)) => Some(Val::Int(val1 - val2)),
        (Val::Int(val1), Val::Rat(val2)) => Some(Val::Rat(val1 as f64 - val2)),
        (Val::Rat(val1), Val::Int(val2)) => Some(Val::Rat(val1 - val2 as f64)),
        (Val::Rat(val1), Val::Rat(val2)) => Some(Val::Rat(val1 - val2)),
        _ => None,
    }
}

// Type-checked and error-handled `Mul` expression
fn type_checked_mul(val1: Val, val2: Val) -> Option<Val> {
    match (val1, val2) {
        (Val::Int(val1), Val::Int(val2)) => Some(Val::Int(val1 * val2)),
        (Val::Int(val1), Val::Rat(val2)) => Some(Val::Rat(val1 as f64 * val2)),
        (Val::Rat(val1), Val::Int(val2)) => Some(Val::Rat(val1 * val2 as f64)),
        (Val::Rat(val1), Val::Rat(val2)) => Some(Val::Rat(val1 * val2)),
        _ => None,
    }
}

// Type-checked and error-handled `Div` expression
fn type_checked_div(val1: Val, val2: Val) -> Option<Val> {
    match (val1, val2) {
        (Val::Int(val1), Val::Int(val2)) => Some(Val::Int(val1 / val2)),
        (Val::Int(val1), Val::Rat(val2)) => Some(Val::Rat(val1 as f64 / val2)),
        (Val::Rat(val1), Val::Int(val2)) => Some(Val::Rat(val1 / val2 as f64)),
        (Val::Rat(val1), Val::Rat(val2)) => Some(Val::Rat(val1 / val2)),
        _ => None,
    }
}

// Interpreter
fn eval(expr: Exp) -> Option<Val> {
    match expr {
        Exp::Var(var) => type_checked_var(var),
        Exp::Add(exp1, exp2) => type_checked_add(eval(*exp1)?, eval(*exp2)?),
        Exp::Sub(exp1, exp2) => type_checked_sub(eval(*exp1)?, eval(*exp2)?),
        Exp::Mul(exp1, exp2) => type_checked_mul(eval(*exp1)?, eval(*exp2)?),
        Exp::Div(exp1, exp2) => type_checked_div(eval(*exp1)?, eval(*exp2)?),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ival(x: u64) -> Box<Exp> {
        Box::new(Exp::Var(Val::Int(x)))
    }

    fn fval(x: f64) -> Box<Exp> {
        Box::new(Exp::Var(Val::Rat(x)))
    }

    #[test]
    fn test_eval() {
        let exp1 = eval(Exp::Add(ival(10), fval(20.0)));
        let exp2 = eval(Exp::Sub(ival(10), fval(20.0)));
        let exp3 = eval(Exp::Mul(ival(10), fval(20.0)));
        let exp4 = eval(Exp::Div(ival(10), fval(20.0)));

        // SAFETY: We know that this works!
        assert_eq!(exp1.unwrap(), Val::Rat(30.0));
        assert_eq!(exp2.unwrap(), Val::Rat(-10.0));
        assert_eq!(exp3.unwrap(), Val::Rat(200.0));
        assert_eq!(exp4.unwrap(), Val::Rat(0.5));
    }
}

fn main() {
    println!("Hello, world!");
}
