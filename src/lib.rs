mod ops;

#[cfg(test)]
mod tests {
    use super::*;
    use ops::*;

    #[test]
    fn it_works() {
        let a = Op::Value(false);
        println!("{:?}", a);
    }
}
