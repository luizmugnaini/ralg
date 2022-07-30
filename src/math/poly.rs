//! Polynomials in coefficient representation
use crate::math::num::Num;
use core::ops::{Add, Mul, Sub};
use itertools::{
    EitherOrBoth::{Both, Left, Right},
    Itertools,
};
use std::cmp;

/// Polynomial representation using coefficients
#[derive(Debug, PartialEq, Clone)]
pub struct Polynomial<T: Num + Copy> {
    /// Vector containing the coefficients of a polynomial, the indices
    /// correspond to the variable's power.
    ///
    /// For instance, the polynomial
    /// `let p = Polynomial::new(vec![4, 3, 2, 9]);`
    /// Corresponds to the polynomial
    /// `p(x) = 4 + 3 x + 2 x^2 + 9 x^3`
    /// in mathematical notation.
    pub coeff: Vec<T>,
}

impl<T: Num + Copy> Polynomial<T> {
    /// Create a new polynomial out ou a vector containing its coefficients.
    /// Example: the vector `c = vec![0, 1, 5, 6]` generates the polynomial
    /// `x^1 + 5 x^2 + 6 x^3`.
    pub fn new(coeff: Vec<T>) -> Self {
        Polynomial { coeff }
    }

    /// Degree-bound is always greater than or equal to the actuall degree of
    /// the polynomial, we define it as the length - 1 of the coefficient
    /// vector. Notice however that, if the coefficient vector is empty or
    /// consists entirely of zeros, the degree is infinte, hence `usize::MAX`
    /// is returned.
    pub fn degree_bound(&self) -> usize {
        let n = self.coeff.len();
        if n == 0 || self.coeff.iter().all(|&a| a == T::zero()) {
            usize::MAX
        } else {
            n - 1
        }
    }

    /// Degree of the polynomial. Returns the index of the last non-zero
    /// coefficient. If the vector of coefficients is empty or is entirely
    /// composed of zeros, `usize::MAX` is returned: the degree of the
    /// polynomial is infinite.
    pub fn degree(&self) -> usize {
        match self
            .coeff
            .iter()
            .enumerate()
            .rev()
            .find(|(_, a)| a != &&T::zero())
        {
            Some((idx, _)) => idx,
            None => usize::MAX,
        }
    }

    /// Evaluate the polynomial at a given point `x` of the domain.
    ///
    /// We use the Horner's method, which is O(n)
    pub fn eval(&self, x: T) -> T {
        let l = self.coeff.len();
        if l == 0 {
            // Empty polynomials are identically zero
            return T::zero();
        }

        (0..self.coeff.len() - 1)
            .rev()
            .fold(self.coeff[l - 1], |acc, idx| self.coeff[idx] + x * acc)
    }

    /// Reduces the coefficient representation of a given polynomial. That is,
    /// we truncate the collection of zero valued coefficients from the end of
    /// the `coeff` vector.
    ///
    /// Example:
    /// ```
    /// use ralg::math::poly::Polynomial;
    ///
    /// let mut p = Polynomial::new(vec![1, 0, 2, 4, 0, 0]);
    /// p.reduce();
    /// assert_eq!(p, Polynomial::new(vec![1, 0, 2, 4]));
    ///
    /// let mut q = Polynomial::new(vec![0, 0, 0]);
    /// q.reduce();
    /// assert_eq!(q, Polynomial::new(vec![]));
    /// ```
    pub fn reduce(&mut self) {
        match self
            .coeff
            .iter()
            .enumerate()
            .rev()
            .find(|(_, a)| a != &&T::zero())
        {
            // Truncate the vector of coefficients so that the last non-zero
            // value is spared
            Some((last_nonzero, _)) => {
                self.coeff.truncate(last_nonzero + 1);
            }

            // If no index was found, the coefficients are either empty or all
            // zeros, in both cases we should return a polynomial with an empty
            // vector of coefficients
            None => {
                self.coeff = Vec::new();
            }
        }
    }

    /// Set degree-bound: Given a polynomial `p` and a degree bound `n`:
    /// If `n > p.degree_bound()`, we add `n - p.degree_bound()` zeros to the
    /// tail of the coefficient vector `p.coeff`, otherwise, we do nothing (we
    /// do not truncate coefficients to decrease the degree).
    ///
    /// Example:
    /// ```
    /// use ralg::math::poly::Polynomial;
    ///
    /// let mut p = Polynomial::new(vec![5, 2, 0, 1]);
    /// assert_eq!(p.degree_bound(), 3);
    ///
    /// p.set_degree_bound(6);
    /// assert_eq!(p, Polynomial::new(vec![5, 2, 0, 1, 0, 0, 0]));
    /// assert_eq!(p.degree_bound(), 6);
    /// ```
    pub fn set_degree_bound(&mut self, n: usize) {
        let add_to_len = n.saturating_sub(self.degree_bound());
        self.coeff.append(&mut vec![T::zero(); add_to_len]);
    }
}

impl<T: Num + Copy> Add for Polynomial<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut coeff =
            vec![T::zero(); cmp::max(self.coeff.len(), rhs.coeff.len())];

        for (idx, p) in
            self.coeff.iter().zip_longest(rhs.coeff.iter()).enumerate()
        {
            match p {
                Both(a, b) => coeff[idx] = *a + *b,
                Left(a) => coeff[idx] = *a,
                Right(b) => coeff[idx] = *b,
            };
        }
        Polynomial::new(coeff)
    }
}

impl<T: Num + Copy> Sub for Polynomial<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut coeff =
            vec![T::zero(); cmp::max(self.coeff.len(), rhs.coeff.len())];

        for (idx, p) in
            self.coeff.iter().zip_longest(rhs.coeff.iter()).enumerate()
        {
            match p {
                Both(a, b) => coeff[idx] = *a - *b,
                Left(a) => coeff[idx] = *a,
                Right(b) => coeff[idx] = *b,
            };
        }
        Polynomial::new(coeff)
    }
}

/// This multiplication is O(n^2), which is not great
/// TODO: multiplication in O(n log(n)) using FFT
impl<T: Num + Copy> Mul for Polynomial<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let ls = (self.coeff.len(), rhs.coeff.len());
        if ls.0 * ls.1 == 0 {
            // The product of a polynomial by a zero polynomial is always zero
            return Polynomial::new(vec![]);
        }

        let mut coeff = vec![T::zero(); ls.0 + ls.1 - 1];
        for i in 0..ls.0 {
            for j in 0..ls.1 {
                coeff[i + j] = coeff[i + j] + self.coeff[i] * rhs.coeff[j];
            }
        }
        Polynomial::new(coeff)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let p = Polynomial::new(vec![0, 1, 5, 6]);
        assert_eq!(p.coeff, vec![0, 1, 5, 6]);
    }

    #[test]
    fn deg() {
        // Zero polynomials
        let p: Polynomial<i32> = Polynomial::new(vec![]);
        assert_eq!(p.degree_bound(), usize::MAX);
        assert_eq!(p.degree(), usize::MAX);

        let p: Polynomial<f32> = Polynomial::new(vec![0.0, 0.0, 0.0, 0.0]);
        assert_eq!(p.degree_bound(), usize::MAX);
        assert_eq!(p.degree(), usize::MAX);

        let p = Polynomial::new(vec![5, 2, 0, 1]);
        assert_eq!(p.degree_bound(), 3);

        // Degree bound >= degree
        let p = Polynomial::new(vec![0, 3, 4, 0]);
        assert_eq!(p.degree_bound(), 3);
        assert_eq!(p.degree(), 2);

        let q = Polynomial::new(vec![0.1, 0.0, 0.0, 0.0]);
        assert_eq!(q.degree_bound(), 3);
        assert_eq!(q.degree(), 0);

        // Degree bound == degree
        let p = Polynomial::new(vec![1, 5, 0, 6, 7]);
        assert_eq!(p.degree_bound(), 4);
        assert_eq!(p.degree(), 4);
    }

    #[test]
    fn eval() {
        let p = Polynomial::new(vec![]);
        assert_eq!(p.eval(3.0), 0.0);

        let q = Polynomial::new(vec![2, 3, 5, 6]);
        assert_eq!(q.eval(0), 2);
        assert_eq!(q.eval(1), 16);
        assert_eq!(q.eval(-1), -2);
        assert_eq!(q.eval(2), 76);
        assert_eq!(q.eval(4), 478);
    }

    #[test]
    fn add() {
        // Sum of zero polynomials is zero
        let p: Polynomial<f32> = Polynomial::new(vec![]);
        let q: Polynomial<f32> = Polynomial::new(vec![]);
        assert_eq!(p + q, Polynomial::new(vec![]));

        let p = Polynomial::new(vec![1, 2, 4]);
        let q = Polynomial::new(vec![5, -2, 17]);
        assert_eq!(p.clone() + q.clone(), Polynomial::new(vec![6, 0, 21]));
        assert_eq!(q + p, Polynomial::new(vec![6, 0, 21]));
    }

    #[test]
    fn mul() {
        // Zero polynomials
        let p: Polynomial<i32> = Polynomial::new(vec![]);
        let q: Polynomial<i32> = Polynomial::new(vec![]);
        assert_eq!(p * q, Polynomial::new(vec![]));

        let p = Polynomial::new(vec![3.4, 9.0, 1.1]);
        let q: Polynomial<f32> = Polynomial::new(vec![]);
        assert_eq!(p * q, Polynomial::new(vec![]));

        // Same length
        let p = Polynomial::new(vec![0, 3, 5]);
        let q = Polynomial::new(vec![4, 7, 8]);
        assert_eq!(
            p.clone() * q.clone(),
            Polynomial::new(vec![0, 12, 41, 59, 40])
        );
        assert_eq!(q * p, Polynomial::new(vec![0, 12, 41, 59, 40]));

        // Different length
        let p = Polynomial::new(vec![5, 0, 10, 6]);
        let q = Polynomial::new(vec![1, 2, 4]);
        assert_eq!(
            p.clone() * q.clone(),
            Polynomial::new(vec![5, 10, 30, 26, 52, 24]),
        );
        assert_eq!(q * p, Polynomial::new(vec![5, 10, 30, 26, 52, 24]));
    }

    #[test]
    fn reduce() {
        let mut p = Polynomial::new(vec![1, 0, 0]);
        p.reduce();
        assert_eq!(p, Polynomial::new(vec![1]));

        let mut p = Polynomial::new(vec![1.0, 0.0, 4.5, 0.0]);
        p.reduce();
        assert_eq!(p, Polynomial::new(vec![1.0, 0.0, 4.5]));

        let mut p = Polynomial::new(vec![0.0, 0.0, 0.0, 0.0]);
        p.reduce();
        assert_eq!(p, Polynomial::new(vec![]));
    }
}
