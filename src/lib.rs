use lalrpop_util::lalrpop_mod;

pub mod ast;
pub mod eval;

lalrpop_mod!(pub while_grammar);
