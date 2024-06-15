pub trait Named: Sized {
    fn is_name(&self, name: &str) -> bool;
    fn get_named(s: &[Self], name: &str) -> Option<usize> {
        for i in 0..s.len() {
            if s[i].is_name(name) {
                return Some(i);
            }
        }
        if let Ok(n) = str::parse::<usize>(name) {
            if n < s.len() {
                return Some(n);
            }
        }
        None
    }
}
