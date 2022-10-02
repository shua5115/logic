use super::Node;
use super::Node::*;
use super::TruthTable;
use std::collections::BTreeSet;

impl Node {
    pub fn from_table(table: &TruthTable, sum_of_products: bool) -> Box<Self> {
        type BoolRow = (Vec<Option<bool>>, usize);

        let len = table.vars.len();
        if len == 0 { return Node::val(false); }
        let bool_row = |s: usize| -> BoolRow {
            let mut r = Vec::with_capacity(len);
            let mut i: usize = len - 1;
            loop {
                r.push(Some(
                    (s & (1 << i)) != 0
                ));
                if i == 0 { break; }
                i -= 1;
            }
            (r, 0)
        };

        fn bool_row_eq(a: &BoolRow, b: &BoolRow) -> bool {
            if a.1 != b.1 || a.0.len() != b.0.len() { return false; }
            for p in a.0.iter().zip(b.0.iter()) {
                match p {
                    (None, Some(_)) | (Some(_), None) => { return false; }
                    (Some(va), Some(vb)) if *va != *vb => { return false; }
                    _ => {}
                }
            }
            true
        }

        // checks if two BoolRows have only a single difference in their
        // non-None entries. If so, it returns a new BoolRow without that difference.
        // Otherwise, returns None. 
        fn single_change(a: &BoolRow, b: &BoolRow) -> Option<BoolRow> {
            if a.1 != b.1 || a.0.len() != b.0.len() || a.0.len() == 0 { return None; }
            let len = a.0.len();
            let mut diff_index: Option<usize> = None;
            let mut num_null: usize = 0;
            for i in 0..len {
                match (a.0[i], b.0[i]) {
                    (None, Some(_)) | (Some(_), None) => { return None; }
                    (None, None) => { num_null += 1; }
                    (Some(va), Some(vb)) if va != vb => {
                        if diff_index == None {
                            diff_index = Some(i);
                        } else { return None; }
                    }
                    _ => {}
                }
            }
            if num_null == len - 1 { return None; }
            if let Some(d) = diff_index {
                let mut out = a.0.clone();
                out[d] = None;
                return Some((out, a.1 + 1));
            }
            None
        }

        fn bool_row_to_node<'a, I>(arr: &Vec<Option<bool>>, mut name_iter: I, sum_of_products: bool) -> Box<Node>
        where I:Iterator<Item = &'a &'a str>
        {
            let mut head: Option<Box<Node>> = None;
            for b in arr {
                let name = name_iter.next();
                if name == None { return Box::new(Value(false)) }
                match b {
                    Some(v) => {
                        let val = if sum_of_products == *v {
                            Box::new(Variable(String::from(*name.unwrap())))
                        } else {
                            Box::new(Node::Not(Box::new(Variable(String::from(*name.unwrap())))))
                        };
                        if let Some(prev) = head {
                            head = if sum_of_products {
                                Some(Box::new(And(prev, val)))
                            } else {
                                Some(Box::new(Or(prev, val)))
                            };
                        } else {
                            head = Some(val);
                        }
                    }
                    _ => {}
                }
            }
            match head {
                Some(n) => n,
                _ => {
                    println!("Head should not be None at this point.");
                    Box::new(Value(false))
                }
            }
        }

        let mut true_results: usize = 0;
        
        let mut source = Vec::new(); // all possibilities
        let mut simple: Vec<BoolRow> = Vec::new(); // the final expression, as BoolRows

        for res in &table.results {
            if res.1 { true_results += 1; }
            if res.1 == sum_of_products { source.push(bool_row(res.0)) }
        }
        if true_results == 0 || true_results == table.vars.len() {
            return Box::new(Value(true_results > 0));
        }

        loop {
            let mut changed: BTreeSet<usize> = BTreeSet::new(); // all indexes of source which were simplified
            let mut candidates: Vec<BoolRow> = Vec::new(); // all simplified BoolRows to continue to next iteration
            // changed.clear();
            let size = source.len();
            for i in 0..size {
                for j in (i+1)..size {
                    let a = source.get(i).unwrap();
                    let b = source.get(j).unwrap();
                    let c = single_change(a, b);
                    if let Some(br) = c {
                        changed.insert(i);
                        changed.insert(j);
                        let mut dupe = false;
                        for obr in &candidates {
                            if bool_row_eq(&br, obr) {
                                dupe = true;
                                break;
                            }
                        }
                        if !dupe {
                            candidates.push(br);
                        }
                    }
                }
            }
            // taking advantage of the fact that all changed indices are sorted
            let mut exclude_iter = changed.iter();
            let mut exclude = exclude_iter.next();
            // moving all unchanged values from source into to simple
            for (i, v) in source.into_iter().enumerate() {
                match exclude {
                    Some(x) if i == *x => {
                        exclude = exclude_iter.next();
                    }
                    _ => {
                        simple.push(v);
                    }
                }
            }
            source = candidates;
            if changed.len() == 0 { break; } // then all values were moved
        }
        // TODO: construct tree from simple
        let var_names = &table.vars;
        let mut head: Option<Box<Node>> = None;
        for br in simple {
            let elem = bool_row_to_node(&br.0, var_names.iter(), sum_of_products);
            if let None = head {
                head = Some(elem);
            } else if let Some(h) = head {
                if sum_of_products {
                    head = Some(Box::new(Or(h, elem)));
                } else {
                    head = Some(Box::new(And(h, elem)));
                }
            }
        }
        match head {
            Some(h) => h,
            _ => Box::new(Value(false))
        }
    }
}