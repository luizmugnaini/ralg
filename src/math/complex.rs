//! An implementation of complex numbers
use crate::math::num::{Num, Zero};
use crate::zero_impl;
use core::ops::{Add, Mul, Neg, Sub};

/// Complex number
#[derive(Clone, Debug)]
pub struct Complex<T: Clone + Num> {
    /// Real part
    pub re: T,

    /// Imaginary part
    pub im: T,
}

impl<T: Clone + Num> Complex<T> {
    /// Create a new complex number
    pub fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }

    /// Imaginary unit
    pub fn i() -> Self {
        Complex::new(T::zero(), T::one())
    }

    /// Norm. Given a complex number `x == a + bi`, returns `a * a + b * b`
    pub fn norm(x: Self) -> T {
        x.re.clone() * x.re.clone() + x.im.clone() * x.im
    }

    /// Given a real number `re`, returns the complex number `re + 0 * i`
    pub fn from_real(re: T) -> Self {
        Complex::new(re, T::zero())
    }

    /// Given a vector of real numbers transforms into a vector of complex
    /// number with zero imaginary part.
    /// Example:
    /// ```
    /// use ralg::math::complex::Complex;
    ///
    /// let vec_re = vec![0.1, 4.0, -6.0];
    /// let vec_cpx = Complex::from_real_vec(vec_re);
    /// assert_eq!(vec_cpx, vec![
    ///     Complex::new(0.1, 0.0),
    ///     Complex::new(4.0, 0.0),
    ///     Complex::new(-6.0, 0.0)]
    /// );
    /// ```
    pub fn from_real_vec(vec_re: Vec<T>) -> Vec<Complex<T>> {
        let mut cpx_vec = Vec::new();
        for re in vec_re {
            cpx_vec.push(Complex::from_real(re));
        }
        cpx_vec
    }
}

/// Unfortunately, we still split the cases for `f32` and `f64`, in the future
/// I'll maybe add a `Float` type that will generalize this. I'm aware of the
/// `num` crate, but my goal is to build everything from scratch.
impl Complex<f32> {
    /// Given polar coordinates `r` (radius) and `theta` (angle in radians),
    /// returns the corresponding complex number.
    pub fn from_polar(r: f32, theta: f32) -> Self {
        Complex::new(r * theta.cos(), r * theta.sin())
    }

    /// Compute the exponential function of `self`, returning the corresponding
    /// complex number.
    pub fn exp(self) -> Self {
        Complex::from_polar(self.re.exp(), self.im)
    }

    /// Returns the `n`th root of unity.
    pub fn root_of_unity(n: usize) -> Self {
        // e^{theta i} = cos(theta) + sin(theta) * i
        let theta: f32 = -2.0 * std::f32::consts::PI / n as f32;
        Complex::new(theta.cos(), theta.sin())
    }
}

impl Complex<f64> {
    /// Given polar coordinates `r` (radius) and `theta` (angle in radians),
    /// returns the corresponding complex number.
    pub fn from_polar_f64(r: f64, theta: f64) -> Self {
        Complex::new(r * theta.cos(), r * theta.sin())
    }

    /// Compute the exponential function of `self`, returning the corresponding
    /// complex number.
    pub fn exp_f64(self) -> Self {
        Complex::from_polar_f64(self.re.exp(), self.im)
    }

    /// Returns the `n`th root of unity.
    pub fn root_of_unity_f64(n: usize) -> Self {
        // e^{theta i} = cos(theta) + sin(theta) * i
        let theta: f64 = 2.0 * std::f64::consts::PI / n as f64;
        Complex::new(theta.cos(), theta.sin())
    }
}

impl<T: Clone + Num + Neg<Output = T>> Complex<T> {
    /// Complex conjugate of a given complex number.
    /// Example:
    /// ```
    /// use ralg::math::complex::Complex;
    ///
    /// let z = Complex::new(4, 5);
    /// assert_eq!(z.conj(), Complex::new(4, -5));
    /// ```
    pub fn conj(self) -> Self {
        Complex::new(self.re, -self.im)
    }
}

impl<T: Clone + Num> PartialEq for Complex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.re == other.re && self.im == other.im
    }
}

impl<T: Clone + Num> Add for Complex<T> {
    type Output = Self;

    /// Addition of complex numbers.
    /// Example:
    /// ```
    /// use ralg::math::complex::Complex;
    ///
    /// let z1 = Complex::new(3.3, 20.0);
    /// let z2 = Complex::new(4.4, -5.0);
    /// assert_eq!(z1 + z2, Complex::new(7.7, 15.0));
    /// ```
    fn add(self, rhs: Self) -> Self {
        Complex::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl<T: Clone + Num> Add<T> for Complex<T> {
    type Output = Self;

    /// Addition of a complex number and real number.
    /// Example:
    /// ```
    /// use ralg::math::complex::Complex;
    ///
    /// let z = Complex::new(3.3, 20.0);
    /// assert_eq!(z + 5.0, Complex::new(8.3, 20.0));
    /// ```
    fn add(self, rhs: T) -> Self {
        Complex::new(self.re + rhs, self.im)
    }
}

zero_impl!(Complex<usize>, Complex::new(0, 0));
zero_impl!(Complex<u8>, Complex::new(0, 0));
zero_impl!(Complex<u16>, Complex::new(0, 0));
zero_impl!(Complex<u32>, Complex::new(0, 0));
zero_impl!(Complex<u128>, Complex::new(0, 0));

zero_impl!(Complex<isize>, Complex::new(0, 0));
zero_impl!(Complex<i8>, Complex::new(0, 0));
zero_impl!(Complex<i16>, Complex::new(0, 0));
zero_impl!(Complex<i32>, Complex::new(0, 0));
zero_impl!(Complex<i64>, Complex::new(0, 0));
zero_impl!(Complex<i128>, Complex::new(0, 0));

zero_impl!(Complex<f32>, Complex::new(0.0, 0.0));
zero_impl!(Complex<f64>, Complex::new(0.0, 0.0));

impl<T: Copy + Num + Sub<T, Output = T>> Sub for Complex<T> {
    type Output = Self;

    /// Subtraction between complex numbers.
    /// Example:
    /// ```
    /// use ralg::math::complex::Complex;
    ///
    /// let z1 = Complex::new(3, 4);
    /// let z2 = Complex::new(7, 5);
    /// assert_eq!(z1 - z2, Complex::new(-4, -1));
    /// ```
    fn sub(self, rhs: Self) -> Self {
        Complex::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl<T: Copy + Num + Sub<T, Output = T>> Sub<T> for Complex<T> {
    type Output = Self;

    /// Subtraction of complex numbers by real numbers.
    /// Example:
    /// ```
    /// use ralg::math::complex::Complex;
    ///
    /// let z = Complex::new(3.0, 4.3);
    /// assert_eq!(z - 2.5, Complex::new(0.5, 4.3));
    /// ```
    fn sub(self, rhs: T) -> Self {
        Complex::new(self.re - rhs, self.im)
    }
}

impl<T: Copy + Num + Neg<Output = T>> Neg for Complex<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Complex::new(-self.re, -self.im)
    }
}

impl<T: Copy + Num + Sub<T, Output = T>> Mul for Complex<T> {
    type Output = Self;

    /// Multiplication of complex numbers.
    /// Example:
    /// ```
    /// use ralg::math::complex::Complex;
    ///
    /// let z1 = Complex::new(3, 5);
    /// let z2 = Complex::new(6, 2);
    /// assert_eq!(z1 * z2, Complex::new(8, 36));
    ///
    /// let z = Complex::new(6, 7);
    /// assert_eq!(z.clone() * z, Complex::new(-13, 84));
    /// ```
    fn mul(self, rhs: Self) -> Self {
        Complex::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

impl<T: Copy + Num> Mul<T> for Complex<T> {
    type Output = Self;

    /// Multiplication of a complex number by a real number.
    /// Example:
    /// ```
    /// use ralg::math::complex::Complex;
    ///
    /// let z = Complex::new(6, -2);
    /// assert_eq!(z * -4, Complex::new(-24, 8));
    /// ```
    fn mul(self, rhs: T) -> Self {
        Complex::new(self.re * rhs, self.im * rhs)
    }
}
