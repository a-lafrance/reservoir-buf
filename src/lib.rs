use std::marker::PhantomData;
use std::fmt::{self, Formatter};

// TODO: would probably be nice to be able to set an upper bound on the size of the buffer

// TODO: would probably be better to manage the allocation directly rather than delegating to Vec.
// I think the finer control would be nice and would possibly make the interface nicer (like the sizes
// being in terms of elements rather than bytes is kind of wonky)

/* --- Reservoir --- */

#[derive(Clone)]
pub struct Reservoir<T>(Vec<T>);

impl<T> Reservoir<T> {
    pub const SMALL_CAPACITY: usize = 64;
    pub const MEDIUM_CAPACITY: usize = 256;
    pub const LARGE_CAPACITY: usize = 1024;

    pub fn new(capacity: usize) -> Self {
        Reservoir(Vec::with_capacity(capacity))
    }

    pub fn get(&self, handle: Handle<T>) -> &T {
        &self.0[handle.0]
    }

    pub fn get_mut(&mut self, handle: Handle<T>) -> &mut T {
        &mut self.0[handle.0]
    }

    pub fn insert(&mut self, data: T) -> Handle<T> {
        self.0.push(data);
        Handle(self.0.len() - 1, PhantomData)
    }
}

impl<T> Default for Reservoir<T> {
    fn default() -> Self {
        Reservoir::new(Self::MEDIUM_CAPACITY)
    }
}

/* --- Handle --- */

pub struct Handle<T>(usize, PhantomData<T>);

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Handle(self.0, self.1)
    }
}

impl<T> Copy for Handle<T> {}

impl<T> fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Handle({})", self.0)
    }
}

// TODO: manually implement Hash, Eq, Ord, PartialEq, PartialOrd
// note to self: they need to be manually implemented because their
// derive macros don't understand that this type doesn't actually
// depend on its generic type for any of their impls.

/* --- Tests --- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check() {
        let mut r: Reservoir<String> = Reservoir::default();
        let h = r.insert(String::from("hello"));
        assert_eq!("hello", r.get(h));

        r.get_mut(h).push('!');
        assert_eq!("hello!", r.get(h));
    }
}
