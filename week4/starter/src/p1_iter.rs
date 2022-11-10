//! P1: Cartesian product iterator
//!
//! To get experience with traits and generics, you will implement a new kind
//! of iterator: cartesian product. The product of two iterators is the set
//! of all pairs of items from each iterator. For example:
//!
//! [1, 2] x [3, 4]  =  [(1, 3), (1, 4), (2, 3), (2, 4)]
//!
//! Your task is to design all the structs, traits, and impls that are needed
//! to make this code snippet possible:
//!
//! ```ignore
//! let h1 = hashset![1, 2];
//! let h2 = hashset![3, 4];
//! let product =
//!   h1.into_iter()
//!   .cartesian_product(h2.into_iter())
//!   .collect::<HashSet<_>>();
//! ```
//!
//! That is, there should be a method `cartesian_product` which can be called
//! on *any* iterator, such as the one produced by `HashSet::into_iter`. This method
//! returns a structure that implements the `Iterator` trait, allowing one to call
//! methods like `collect`.
//!
//! The snippet above is provided as a unit test, which you can run with
//! `cargo test product`. The test will not compile until you build the API.
//!
//! To get you started, I would read Rust's documentation on how to implement an iterator:
//! https://doc.rust-lang.org/std/iter/index.html#implementing-iterator

pub struct CPIter<A, B> {
    xs: Vec<A>,
    ys: Vec<B>,
    n: usize
}
impl<A : Clone, B: Clone> Iterator for CPIter<A, B> {
    type Item = (A, B);
    fn next(&mut self) -> Option<(A, B)> {
        if self.n < self.xs.len() * self.ys.len() {
            let i = self.n / self.ys.len();
            let j = self.n % self.ys.len();
            self.n += 1;
            Some((self.xs[i].clone(), self.ys[j].clone()))
        } else {
            None
        }
    }
}

pub trait CartesianProduct: Iterator {
    fn cartesian_product<Them: Iterator>(self, them: Them) -> CPIter<Self::Item, Them::Item> where
        Self::Item : Clone,
        Them::Item : Clone;
}
impl<T: Iterator> CartesianProduct for T {
    fn cartesian_product<Them: Iterator>(self, them: Them) -> CPIter<Self::Item, Them::Item> where
    Self::Item : Clone,
    Them::Item : Clone {
        CPIter {
            xs: self.collect(),
            ys: them.collect(),
            n: 0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use maplit::hashset;
    use std::collections::HashSet;

    #[test]
    fn cartesian_product_test() {
        let h1 = hashset![1, 2];
        let h2 = hashset![3, 4];
        let product = h1.into_iter().cartesian_product(h2.into_iter());
        assert_eq!(
            product.collect::<HashSet<_>>(),
            hashset![(1, 3), (1, 4), (2, 3), (2, 4)]
        )
    }
}
