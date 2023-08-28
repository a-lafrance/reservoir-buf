use std::marker::PhantomData;

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

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Handle<T>(usize, PhantomData<T>);

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
