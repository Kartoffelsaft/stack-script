use super::super::runner::{Executable, StackValue, ExecuteErr, StackExecuteErr};
use either::Either::{Left, Right};

impl Executable for super::Block<'_> {
    fn execute(&self) -> Result<(), ExecuteErr> {
        let _ = self.execute_with_stack(&mut Vec::new());
        Ok(())
    }

    fn execute_with_stack(&self, stack: &mut Vec<StackValue>) -> Result<(), StackExecuteErr> {
        use super::RefinedToken::*;
        for op in &self.operations { match op {
            LangType(_) => unimplemented!(),
            LangTypeCast(_) => unimplemented!(),
            Number(Left(n)) => stack.push(StackValue::ISize(*n)),
            Number(Right(n)) => stack.push(StackValue::Float64(*n)),
            Keyword(w) => w.execute_with_stack(stack)?,
            Call(f) => self.user_defs.get(f).map_or_else(
                || panic!("{} called, but not defined", f),
                |real_func| {real_func.execute_with_stack(stack)},
            )?,
            LooseBlock(b) => b.execute_with_stack(stack)?,
        }}

        Ok(())
    }
}

impl Executable for super::RefinedStandardKeyword {
    fn execute(&self) -> Result<(), ExecuteErr> {
        Err(ExecuteErr::RequiresStack)
    }

    fn execute_with_stack(&self, stack: &mut Vec<StackValue>) -> Result<(), StackExecuteErr> {
        use super::RefinedStandardKeyword::*;
        let mut pop = || {
            stack.pop().ok_or(StackExecuteErr::EmptyStackPop)
        };
        let push_vals = match self {
            Stdin => unimplemented!(),
            Stdout => {print!("{}", pop()?); vec![]},
            Add => vec![pop()? + pop()?],
            Mul => vec![pop()? * pop()?],
            Sub => vec![pop()? - pop()?],
            Div => vec![pop()? / pop()?],
            Copy => {let x = pop()?; vec![x.clone(), x]}
        };

        // clippy thinks this is redundant, but it doesn't realize this
        // is a workaround for converting a &mut argument into a move arg.
        // like, why does append take &mut and just erases the values? that
        // sounds like it should really be a move, but no one seems to
        // question it, so I must just be insane or dumb
        #[allow(clippy::redundant_closure_call)]
        (move |mut v| stack.append(&mut v))(push_vals);

        Ok(())
    }
}
