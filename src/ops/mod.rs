#[derive(Debug)]
pub enum Op {
    Value(bool),
    Variable(String),
    Not(Box<Op>),
    And(Box<Op>, Box<Op>),
    Or(Box<Op>, Box<Op>),
    Nand(Box<Op>, Box<Op>),
    Nor(Box<Op>, Box<Op>),
    Xor(Box<Op>, Box<Op>),
    Eq(Box<Op>, Box<Op>),
    Imply(Box<Op>, Box<Op>)
}