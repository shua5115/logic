pub struct TruthTable {
    pub vars: std::collections::BTreeSet<String>,
    pub results: Vec<(usize, bool)>
}
