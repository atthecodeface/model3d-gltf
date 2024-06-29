//a Imports
use std::marker::PhantomData;

use crate::Indexable;

//a ODUse
//tp ODUse
#[derive(Default, Clone)]
pub enum ODUse<T>
where
    T: Clone,
{
    #[default]
    Unknown,
    NotRequired,
    Required,
    Use(T),
}

//ip Debug for ODUse<T>
impl<T> std::fmt::Debug for ODUse<T>
where
    T: Clone + std::fmt::Debug,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Unknown => write!(fmt, "?"),
            Self::NotRequired => write!(fmt, "<unused>"),
            Self::Required => write!(fmt, "<required>"),
            Self::Use(t) => t.fmt(fmt),
        }
    }
}

//ip ODUse
impl<T> ODUse<T>
where
    T: Clone,
{
    //ap is_required
    /// Return true if it is required (whether specified with a use or not)
    #[allow(dead_code)]
    pub fn is_required(&self) -> bool {
        !matches!(self, ODUse::Unknown | ODUse::NotRequired)
    }

    //mp set_use
    /// Set the use to be something - it must already be required
    #[track_caller]
    pub fn set_use(&mut self, data: T) {
        assert!(
            matches!(self, ODUse::Required),
            "Must already be Required to set a use"
        );
        *self = ODUse::Use(data);
    }

    //ap data
    /// Return Some reference to the data, if it has had its use set; else None
    #[track_caller]
    pub fn data(&self) -> Option<&T> {
        match self {
            ODUse::Use(x) => Some(x),
            _ => None,
        }
    }
}

//a ODUses
//tp ODUses
pub struct ODUses<Index, T>
where
    Index: Indexable,
    T: Clone,
{
    unknown: ODUse<T>,
    uses: Vec<ODUse<T>>,
    phantom: PhantomData<Index>,
}

//ip Debug for ODUses<Index, T>
impl<Index, T> std::fmt::Debug for ODUses<Index, T>
where
    Index: Indexable,
    T: Clone + std::fmt::Debug,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "ODUses [")?;
        let mut first = true;
        for u in &self.uses {
            if !first {
                write!(fmt, ", ")?;
            }
            write!(fmt, "{u:?}")?;
            first = false;
        }
        write!(fmt, "]")
    }
}

//ip ODUses<Index, T>
impl<Index, T> ODUses<Index, T>
where
    Index: Indexable,
    T: Clone,
{
    //cp new
    pub fn new() -> Self {
        let unknown = ODUse::Unknown;
        let uses = vec![];
        Self {
            unknown,
            uses,
            phantom: PhantomData,
        }
    }

    //ap is_required
    #[allow(dead_code)]
    pub fn is_required(&self, i: Index) -> bool {
        let i = i.as_usize();
        if i >= self.uses.len() {
            false
        } else {
            self.uses[i].is_required()
        }
    }

    //mp set_required
    pub fn set_required(&mut self, i: Index) {
        let i = i.as_usize();
        while i >= self.uses.len() {
            self.uses.push(ODUse::Unknown);
        }
        self.uses[i] = ODUse::Required;
    }

    //mp complete_uses
    pub fn complete_uses(&mut self) {
        for x in self.uses.iter_mut() {
            if let ODUse::Unknown = x {
                *x = ODUse::NotRequired;
            }
        }
    }
    pub fn iter_required(&self) -> impl Iterator<Item = (Index, &ODUse<T>)> {
        self.uses
            .iter()
            .enumerate()
            .filter(|(_n, x)| x.is_required())
            .map(|(n, x)| (n.into(), x))
    }
    pub fn iter_mut_required(&mut self) -> impl Iterator<Item = (Index, &mut ODUse<T>)> {
        self.uses
            .iter_mut()
            .enumerate()
            .filter(|(_n, x)| x.is_required())
            .map(|(n, x)| (n.into(), x))
    }
}

impl<Index, T> std::ops::Index<Index> for ODUses<Index, T>
where
    Index: Indexable,
    T: Clone,
{
    type Output = ODUse<T>;
    fn index(&self, index: Index) -> &Self::Output {
        let index = index.as_usize();
        if index > self.uses.len() {
            &self.unknown
        } else {
            &self.uses[index]
        }
    }
}
