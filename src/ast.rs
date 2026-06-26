#[derive(Debug)]
pub enum Statement {
    Assignment {
        assignee: String,
        base: String,
        delta: i32,
    },
    Sequence(Box<Statement>, Box<Statement>),
    While {
        head: String,
        body: Box<Statement>,
    },
    If {
        head: String,
        body: Box<Statement>,
    },
    IfElse {
        head: String,
        body: Box<Statement>,
        else_body: Box<Statement>,
    },
    In(String),
    Out(String),
}
