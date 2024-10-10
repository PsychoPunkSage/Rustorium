use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CustomSet<T> {
    data: Vec<T>,
}

impl<T: Clone + PartialEq + Debug + Ord> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut vecc = Vec::new();
        for i in input {
            if !vecc.contains(i) {
                vecc.push(i.clone());
            }
        }
        vecc.sort_by(|a, b| a.cmp(&b));

        CustomSet { data: vecc }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.data.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if self.contains(&element) {
            return;
        }
        self.data.push(element);
        self.data.sort_by(|a, b| a.cmp(&b));
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.data.iter().all(|val| other.contains(val))
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        !self.data.iter().any(|val| other.contains(val))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let mut res = Self::new(&[]);
        self.data.iter().for_each(|val| {
            if other.contains(val) {
                res.add(val.clone());
            }
        });
        res
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let mut res = Self::new(&[]);
        self.data.iter().for_each(|val| {
            if !other.contains(val) {
                res.add(val.clone());
            }
        });
        res
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut res = self.clone();
        other.data.iter().for_each(|val| {
            if !res.contains(val) {
                res.add(val.clone());
            }
        });
        res
    }
}
