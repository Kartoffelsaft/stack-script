use std::ops::{Add, Sub, Mul, Div};
use std::fmt;

#[derive(Debug, Clone)]
pub enum StackValue {
    Float64(f64),
    ISize(isize),
}

#[derive(Debug)]
pub enum ExecuteErr {
    RequiresStack
}

#[derive(Debug)]
pub enum StackExecuteErr {
    EmptyStackPop
}

pub trait Executable {
    fn execute(&self) -> Result<(), ExecuteErr>;
    fn execute_with_stack(&self, stack: &mut Vec<StackValue>) -> Result<(), StackExecuteErr>;
}

impl Add for StackValue {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        use StackValue::*;
        match (self, rhs) {
            (Float64(l), Float64(r)) => Float64(l + r),
            (ISize(l), ISize(r)) => ISize(l + r),
            _ => unimplemented!(),
        }
    }
}

impl Sub for StackValue {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        use StackValue::*;
        match (self, rhs) {
            (Float64(l), Float64(r)) => Float64(l - r),
            (ISize(l), ISize(r)) => ISize(l - r),
            _ => unimplemented!(),
        }
    }
}

impl Mul for StackValue {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        use StackValue::*;
        match (self, rhs) {
            (Float64(l), Float64(r)) => Float64(l * r),
            (ISize(l), ISize(r)) => ISize(l * r),
            _ => unimplemented!(),
        }
    }
}

impl Div for StackValue {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        use StackValue::*;
        match (self, rhs) {
            (Float64(l), Float64(r)) => Float64(l / r),
            (ISize(l), ISize(r)) => ISize(l / r),
            _ => unimplemented!(),
        }
    }
}

impl fmt::Display for StackValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use StackValue::*;
        match self {
            Float64(v) => v.fmt(f),
            ISize(v) => v.fmt(f),
        }
    }
}
