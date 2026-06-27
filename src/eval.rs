use std::{
    collections::HashMap,
    io::{Write, stdin, stdout},
};

use crate::ast::*;

/// The context for evaluating a [Statement].
#[derive(Clone, Debug, Default)]
pub struct Context {
    /// The bindings this context holds.
    pub bindings: HashMap<String, i32>,
}

impl Statement {
    /// Evaluates this statements in the given context.
    pub fn eval(&self, ctx: &mut Context) {
        match self {
            Self::Assignment {
                assignee,
                base,
                delta,
            } => {
                let base_value = *ctx.bindings.entry(base.clone()).or_insert(0);
                *ctx.bindings.entry(assignee.clone()).or_insert(0) = base_value + delta;
            }
            Self::Sequence(left, right) => {
                left.eval(ctx);
                right.eval(ctx);
            }
            Self::While { head, body } => {
                if !ctx.bindings.contains_key(head) {
                    ctx.bindings.insert(head.clone(), 0);
                }
                while ctx.bindings[head] != 0 {
                    body.eval(ctx);
                }
            }
            Self::If { head, body } => {
                if *ctx.bindings.entry(head.clone()).or_insert(0) != 0 {
                    body.eval(ctx);
                }
            }
            Self::IfElse {
                head,
                body,
                else_body,
            } => {
                if *ctx.bindings.entry(head.clone()).or_insert(0) != 0 {
                    body.eval(ctx);
                } else {
                    else_body.eval(ctx);
                }
            }
            Self::In(id) => {
                print!("{id}: ");
                let _ = stdout().flush();
                let mut input = String::new();
                loop {
                    input.clear();
                    match stdin().read_line(&mut input) {
                        Err(e) => {
                            eprintln!("Error reading user input from stdin: {e}. Try again …");
                            continue;
                        }
                        _ => {}
                    }
                    match input.trim().parse::<i32>() {
                        Ok(i) => {
                            ctx.bindings.insert(id.clone(), i);
                            break;
                        }
                        Err(e) => eprintln!("Please try again ({e})"),
                    }
                }
            }
            Self::Out(id) => println!(
                "{id} = {}",
                ctx.bindings.get(id).copied().unwrap_or_default()
            ),
        }
    }
}
