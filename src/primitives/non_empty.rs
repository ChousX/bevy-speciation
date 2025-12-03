#[derive(Clone, Debug)]
pub struct NonEmpty<T> {
    first: T,
    rest: Vec<T>,
}

impl<T> NonEmpty<T> {
    pub fn new(first: T) -> Self {
        Self {
            first,
            rest: Vec::new(),
        }
    }

    pub fn from_vec(mut vec: Vec<T>) -> Option<Self> {
        if vec.is_empty() {
            None
        } else {
            let first = vec.remove(0);
            Some(Self { first, rest: vec })
        }
    }

    pub fn push(&mut self, item: T) {
        self.rest.push(item);
    }

    pub fn first(&self) -> &T {
        &self.first
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        std::iter::once(&self.first).chain(self.rest.iter())
    }

    pub fn len(&self) -> usize {
        1 + self.rest.len()
    }
}
