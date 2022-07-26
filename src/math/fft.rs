use super::{complex::Complex, misc::next_power_of_2, poly::Polynomial};

/// Fast Fourier Transform (FFT): we use the Cooley-Tukey algorithm, which in
/// this implementation will require zero-padding of the polynomial coefficients
/// to the next power of 2 (zero-padding is not optimal, but this is the
/// simplified version I wanted to implement for now). That is, a polynomial
/// with 5 coefficients `[0, 1, 3, 5, 0]` is going to be zero-padded to have 8
/// coefficients, that is, `[0, 1, 3, 5, 0, 0, 0, 0]`.
///
/// `fft` returns a vector consisting of the evaluation of `p` at each
/// root-of-unity.
pub fn fft(mut p: Polynomial<f32>) -> Vec<Complex<f32>> {
    // Zero padding, if necessary
    let n2 = next_power_of_2(p.coeff.len());
    if n2 != p.coeff.len() {
        p.set_degree_bound(n2 - 1);
    }
    let v = fft_recursive(Complex::from_real_vec(p.coeff));
    for cpx in &v {
        println!("{:?}", cpx);
    }
    v
}

fn fft_recursive(mut v: Vec<Complex<f32>>) -> Vec<Complex<f32>> {
    let n = v.len();
    if n == 1 {
        return v;
    }

    let root_n = Complex::root_of_unity(n);
    let mut omega = Complex::new(1.0, 0.0);

    // Initialize and create the even and odd indexed split of the given vector
    let mut v_even = Vec::new();
    let mut v_odd = Vec::new();
    v.iter().enumerate().for_each(|(idx, a)| {
        if idx % 2 == 0 {
            v_even.push(a.clone());
        } else {
            v_odd.push(a.clone());
        }
    });

    // Divide and conquer recursively
    let y_even = fft_recursive(v_even);
    let y_odd = fft_recursive(v_odd);

    for j in 0..n/2 {
        let t = omega.clone() * y_odd[j].clone();
        v[j] = y_even[j].clone() + t.clone();
        v[j + n/2] = y_even[j].clone() - t;
        omega = root_n.clone() * omega.clone();
    }
    v
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::math::poly::Polynomial;

    fn check_result(result: Vec<Complex<f32>>, expected: Vec<Complex<f32>>) {
        let eps = 1.0e-6;
        for (r, e) in result.iter().zip(expected) {
            assert!((r.re - e.re).abs() < eps);
            assert!((r.im - e.im).abs() < eps);
        }
    }

    #[test]
    fn _fft() {
        let p = Polynomial::new(vec![0.0, 1.0, 3.0, 7.0]);
        let expected = vec![
            Complex::new(11.0, 0.0),
            Complex::new(-3.0, 6.0),
            Complex::new(-5.0, 0.0),
            Complex::new(-3.0, -6.0),
        ];
        check_result(fft(p), expected);

        let p = Polynomial::new(vec![1.0, 3.0, 4.0, 6.0, 7.0, 8.0, 0.0, 0.0]);
        let expected = vec![
            Complex::new(29.0, 0.0),
            Complex::new(-13.778_174, -4.707_106),
            Complex::new(4.0, -5.0),
            Complex::new(1.778_174, 3.292_893),
            Complex::new(-5.0, 0.0),
            Complex::new(1.778_174, -3.292_893),
            Complex::new(4.0, 5.0),
            Complex::new(-13.778_174, 4.707_106_7),
        ];
        check_result(fft(p), expected);

        let p = Polynomial::new(vec![0.0, 1.0, 3.0, 7.0, 8.0]);
        let expected = vec![
            Complex::new(19.0, 0.0),
            Complex::new(-12.242_641, -8.656_854),
            Complex::new(5.0, 6.0),
            Complex::new(-3.757_359, -2.656_854),
            Complex::new(3.0, 0.0),
            Complex::new(-3.757_359, 2.656_854),
            Complex::new(5.0, -6.0),
            Complex::new(-12.242_64, 8.656_854),
        ];
        check_result(fft(p), expected);
    }
}
