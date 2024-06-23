pub trait Named: Sized {
    type Index: Sized + From<usize>;
    fn is_name(&self, name: &str) -> bool;
    fn get_named(s: &[Self], name: &str) -> Option<Self::Index> {
        for (i, sn) in s.iter().enumerate() {
            if sn.is_name(name) {
                return Some(i.into());
            }
        }
        if let Ok(n) = str::parse::<usize>(name) {
            if n < s.len() {
                return Some(n.into());
            }
        }
        None
    }
}
