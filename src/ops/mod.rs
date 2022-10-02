mod simplify;

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
pub enum Node {
    Value(bool),
    Variable(String),
    Not(Box<Node>),
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),
    Nand(Box<Node>, Box<Node>),
    Nor(Box<Node>, Box<Node>),
    Xor(Box<Node>, Box<Node>),
    Eq(Box<Node>, Box<Node>),
    Imply(Box<Node>, Box<Node>)
}

#[derive(PartialEq, Debug)]
pub struct TruthTable<'a> {
    pub vars: BTreeSet<&'a str>,
    pub results: Vec<(usize, bool)>
}

use crate::ops::Node::*;

impl Node {
    pub fn var(name: &str) -> Box<Self> {
        Box::new(Variable(name.to_owned()))
    }
    pub fn val(b: bool) -> Box<Self> {
        Box::new(Value(b))
    }
    pub fn eval(&self, vars: &BTreeMap<&str, bool>) -> bool {
        match self {
            Value(b) => *b,
            Variable(name) => *vars.get(name.as_str()).unwrap_or(&false),
            Not(n) => !n.eval(vars),
            And(a, b) => a.eval(vars) && b.eval(vars),
            Or(a, b) => a.eval(vars) || b.eval(vars),
            Nand(a, b) => !(a.eval(vars) && b.eval(vars)),
            Nor(a, b) => !(a.eval(vars) || b.eval(vars)),
            Xor(a, b) => a.eval(vars) != b.eval(vars),
            Eq(a, b) => a.eval(vars) == b.eval(vars),
            Imply(a, b) => !a.eval(vars) || b.eval(vars)
        }
    }

    pub fn eval_no_vars(&self) -> Option<bool> {
        let vars = self.collect_vars();
        if vars.len() > 0 { return None; }
        let varmap = BTreeMap::<&str, bool>::new();
        Some(self.eval(&varmap))
    }

	pub fn children(&self) -> (Option<&Node>, Option<&Node>) {
		match self {
			Not(n) => (Some(n.as_ref()), None),
			And(a, b) | Or(a, b) |
			Nand(a, b) | Nor(a, b) |
			Xor(a, b) | Eq(a, b) |
            Imply(a, b)
			=> (Some(a.as_ref()), Some(b.as_ref())),
			_ => (None, None)
		}
	}

    pub fn is_binary(&self) -> bool {
        match self {
            Value(_) => false,
            Variable(_) => false,
            Not(_) => false,
            _ => true
        }
    }
    pub fn is_unary(&self) -> bool {
        match self {
            Not(_) => true,
            _ => false
        }
    }

    pub fn is_value(&self) -> bool {
        match self {
            Value(_) => true,
            Variable(_) => true,
            _ => false
        }
    }
    pub fn collect_vars(&self) -> BTreeSet<&str> {
        fn find_vars<'a>(n: &'a Node, v: &mut BTreeSet<&'a str>) {
            if let Variable(name) = n {
                v.insert(name.as_str());
            }
            match n.children() {
                (Some(a), None) => find_vars(a, v),
                (Some(a), Some(b)) => {
                    find_vars(a, v);
                    find_vars(b, v);
                }
                _ => {}
            }
        }
        let mut v: BTreeSet<&str> = BTreeSet::new();
        find_vars(self, &mut v);
        v
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        match self {
            Self::Value(arg0) => Self::Value(*arg0),
            Self::Variable(arg0) => Self::Variable(arg0.clone()),
            Self::Not(arg0) => Self::Not(arg0.clone()),
            Self::And(arg0, arg1) => Self::And(arg0.clone(), arg1.clone()),
            Self::Or(arg0, arg1) => Self::Or(arg0.clone(), arg1.clone()),
            Self::Nand(arg0, arg1) => Self::Nand(arg0.clone(), arg1.clone()),
            Self::Nor(arg0, arg1) => Self::Nor(arg0.clone(), arg1.clone()),
            Self::Xor(arg0, arg1) => Self::Xor(arg0.clone(), arg1.clone()),
            Self::Eq(arg0, arg1) => Self::Eq(arg0.clone(), arg1.clone()),
            Self::Imply(arg0, arg1) => Self::Imply(arg0.clone(), arg1.clone()),
        }
    }
}

impl From<Box<Node>> for Node {
    fn from(b: Box<Node>) -> Self {
        match *b {
            Value(v) => Value(v),
            Variable(name) => Variable(name.clone()),
            Not(a) => Not(a),
            And(a, b) => And(a, b),
            Or(a, b) => Or(a, b),
            Nand(a, b) => Nand(a, b),
            Nor(a, b) => Nor(a, b),
            Xor(a, b) => Xor(a, b),
            Eq(a, b) => Eq(a, b),
            Imply(a, b) => Imply(a, b),
        }
    }
}

impl From<&TruthTable<'_>> for Node {
    fn from(t: &TruthTable) -> Self {
        Node::from(Node::from_table(t, true))
    }
}

impl From<bool> for Node {
    fn from(b: bool) -> Self {
        Value(b)
    }
}

impl From<bool> for Box<Node> {
    fn from(b: bool) -> Self {
        Box::new(Node::from(b))
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            Not(_) => ("!"),
            And(_, _) => ("&"),
            Or(_, _) => ("|"),
            Nand(_, _) => ("~&"),
            Nor(_, _) => ("~|"),
            Xor(_, _) => ("~="),
            Eq(_, _) => ("="),
            Imply(_, _) => ("->"),
            _ => ""
        };
        match self.children() {
            (Some(a), Some(b)) => {
                write!(f, "(")?;
                a.fmt(f)?;
                write!(f, " {} ", op)?;
                b.fmt(f)?;
                write!(f, ")")?;
            }
            (Some(a), None) => {
                write!(f, "{}", op)?;
                a.fmt(f)?;
            }
            _ => {
                match self {
                    Value(v) => {
                        write!(f, "{}", *v)?;
                    }
                    Variable(name) => {
                        write!(f, "{}", name)?;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

impl<'a> TruthTable<'a> {
    /// Creates a truth table from a node tree by evaluating every variable combination.
    /// Returns None if there are too many variables
    pub fn new(n: &'a Node) -> Option<Self> {
        let vars = n.collect_vars();
        let var_count = vars.len() as u32;
        let combos = usize::checked_shl(1, var_count);
        let combos = match combos {
            Some(n) => n,
            None => { return None; }
        };
        let mut results: Vec<(usize, bool)> = Vec::new();
        let mut bits: usize = 0;
        let mut state: BTreeMap<&str, bool> = BTreeMap::new();

        while bits < combos {
            for (i, s) in vars.iter().rev().enumerate() {
                state.insert(*s, (bits & (1 << i)) != 0);
            }
            results.push((bits, n.eval(&state)));
            bits += 1;
        }
        Some(TruthTable{vars, results})
    }
}

impl std::fmt::Display for TruthTable<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fn bool_to_bin_str(res: bool) -> char {
            match res {true => '1', false => '0'}
        }
        let len = self.vars.len();
        let mut varlens = Vec::with_capacity(len);
        for s in &self.vars {
            varlens.push(s.len());
        }
        for s in &self.vars {
            write!(f, "{} | ", s)?;
        }
        writeln!(f, "Results")?;
        for (state, res) in self.results.iter() {
            for i in (0..len).rev() {
                write!(f, "{}", bool_to_bin_str((state & (1 << i)) != 0))?;
                for _ in 0..(varlens[i] + 2) {
                    write!(f, " ")?;
                }
            }
            writeln!(f, "{}", bool_to_bin_str(*res))?;
        }
        Ok(())
    }
}

