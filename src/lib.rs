pub mod ops;
pub mod circuit;

#[cfg(test)]
mod tests {
    use super::*;
    use ops::*;
    use ops::Node::*;

    #[test]
    fn it_works() {
        let a = Node::val(false);
        let b = Node::val(true);
        let and = And(a, b);
        assert_eq!(and.eval_no_vars(), Some(false))
    }
    #[test]
    fn no_vars() {
        let a = Node::var("hi");
        let b = Node::val(true);
        let and = And(a, b);
        assert_eq!(and.eval_no_vars(), None)
    }

    #[test]
    fn table_gen() {
        let a = Node::var("a");
        let b = Node::var("b");
        let and = Box::new(And(a, b));
        let c = Node::var("c");
        let imp = Imply(and, c);
        let table = TruthTable::new(&imp).unwrap();
        println!("{}", imp);
        println!("{}", table);
    }

    #[test]
    fn empty_table() {
        let a = Node::val(true);
        let b = Node::val(false);
        let and = Box::new(And(a, b));
        let c = Node::val(false);
        let imp = Imply(and, c);
        let table = TruthTable::new(&imp).unwrap();
        assert_eq!(table.results.len(), 1);
    }

    #[test]
    fn from_table() {
        let a = Node::var("a");
        let b = Node::var("b");
        let and = Box::new(And(a, b));
        let c = Node::var("c");
        let imp = Box::new(Imply(and, c));
        let d = Node::var("d");
        let e = Xor(imp, d);
        let table = TruthTable::new(&e).unwrap();
        let expr1 = Node::from_table(&table, true);
        let table1 = TruthTable::new(&expr1).unwrap();
        let expr2 = Node::from_table(&table, false);
        let table2 = TruthTable::new(&expr2).unwrap();
        println!("OG:  {}", e);
        //println!("{}", table);
        println!("SOP: {}", expr1);
        //println!("{}", table1);
        println!("POS: {}", expr2);
        //println!("{}", table2);
        assert_eq!(table, table1);
        assert_eq!(table, table2);
    }
    #[test]
    fn nand_from_sop() {
        let a = Node::var("a");
        let b = Node::var("b");
        let and = Box::new(And(a, b));
        let c = Node::var("c");
        let imp = Box::new(Imply(and, c));
        let table = TruthTable::new(imp.as_ref()).unwrap();
        let expr1 = Node::from_table(&table, true);
        let table1 = TruthTable::new(expr1.as_ref()).unwrap();
        println!("{}", imp);
        println!("{}", expr1.to_nand());
        assert_eq!(table, table1);
    }
    #[test]
    fn nand_direct() {
        let a = Node::var("a");
        let b = Node::var("b");
        let and = Box::new(And(a, b));
        let c = Node::var("c");
        let xor = Box::new(Xor(and, c));
        let expr1 = xor.to_nand();
        println!("{}", xor);
        println!("{}", expr1);
        let table = TruthTable::new(xor.as_ref());
        let table1 = TruthTable::new(expr1.as_ref());
        assert_eq!(table, table1);
    }
}
