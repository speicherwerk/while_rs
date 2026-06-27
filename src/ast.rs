/// The abstract syntac tree
#[derive(Clone, Debug)]
pub enum Statement {
    /// An assignment
    ///
    /// Id = Id + Num
    Assignment {
        assignee: String,
        base: String,
        delta: i32,
    },
    /// A sequence of two statements
    ///
    /// Statement ; Statement
    Sequence(Box<Statement>, Box<Statement>),
    /// A while loop that evaluates its body while its head doesn't evaluate to 0
    ///
    /// WHILE Id { Statement }
    While { head: String, body: Box<Statement> },
    /// An if statement that evaluates its body if its head doesn't evaluate to zero
    ///
    /// IF Id { Statement }
    If { head: String, body: Box<Statement> },
    /// An if-else statement that evaluates the if-branche's body if the head doesn't evaluate to
    /// zero and the else-branche's body otherwise.
    ///
    /// IF Id { Statement } ELSE { Statement }
    IfElse {
        head: String,
        body: Box<Statement>,
        else_body: Box<Statement>,
    },
    /// Stores a user-input number from standard in into a binding.
    ///
    /// Id = IN
    In(String),
    /// Prints a binding's value onto standard out.
    ///
    /// OUT Id
    Out(String),
}
