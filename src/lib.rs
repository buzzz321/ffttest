use num_complex::Complex64;

use std::f64::consts::PI;

//bit reverse algo from https://www.katjaas.nl/bitreversal/bitreversal.html
pub fn bit_reverse(n: usize, bit_width: u64) -> usize {
    let mut reverse = n; // reverse will store the bit-reversed pattern
    let mask = 1 << bit_width; // find N: shift left 1 by the number of bits
    let mut input = n;

    for _i in 1..bit_width {
        input >>= 1;
        reverse <<= 1;
        reverse |= input & 1; // give LSB of n to nrev
    }
    reverse &= mask - 1; // clear all bits more significant than N-1

    reverse
}

pub fn reverse_bit_order<Type, const SIZE: usize>(signal: &mut [Type; SIZE], bits: u64) {
    let n = signal.len();
    for i in 0..(n) {
        let j = bit_reverse(i, bits);
        if i < j {
            signal.swap(i, j);
        }
    }
}

pub fn fftiter<const SIZE: usize>(out_fft: &mut [Complex64; SIZE]) {
    #[allow(non_snake_case)]
    let N = out_fft.len();
    let order = N.ilog2() as u64;

    reverse_bit_order(out_fft, order);

    let mut n1: usize;
    let mut n2 = 1;

    // _i is the depth butter flies in the fft, so for 8 inputs we have depth of 3 (2^3)
    for _i in 0..(order) {
        n1 = n2;
        n2 *= 2;
        let step_angle = -2.0 * PI / (n2 as f64);
        let mut angle = 0.0;

        for j in 0..n1 {
            // j will select odd or even
            let factors = Complex64::new(0.0, angle).exp();
            angle += step_angle;

            for k in (j..N).step_by(n2) {
                let tmp = out_fft[k];
                out_fft[k] += factors * out_fft[k + n1];
                out_fft[k + n1] = tmp - factors * out_fft[k + n1]; // n/2 mirrored path
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bit_reverse;
    use crate::fftiter;
    use crate::reverse_bit_order;
    use num_complex::Complex;

    #[test]
    fn bit_reverse_works() {
        let result = bit_reverse(4, 3);
        assert_eq!(result, 1);
    }

    #[test]
    fn reverse_bit_order_test() {
        let mut uat: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        let length = uat.len();
        let order = length.ilog2() as u64;

        reverse_bit_order(&mut uat, order);

        assert_eq!(uat, [1, 5, 3, 7, 2, 6, 4, 8]);
    }

    #[test]
    fn fftiter_test() {
        const N: usize = 8;

        let mut signal: [Complex<f64>; N] = [
            num_complex::Complex::new(1.0, 0.0),
            num_complex::Complex::new(2.0, 0.0),
            num_complex::Complex::new(3.0, 0.0),
            num_complex::Complex::new(4.0, 0.0),
            num_complex::Complex::new(5.0, 0.0),
            num_complex::Complex::new(6.0, 0.0),
            num_complex::Complex::new(7.0, 0.0),
            num_complex::Complex::new(8.0, 0.0),
        ];

        fftiter(&mut signal);
        let ans = [
            Complex { re: 36.0, im: 0.0 },
            Complex {
                re: -4.0,
                im: 9.65685424949238,
            },
            Complex { re: -4.0, im: 4.0 },
            Complex {
                re: -4.0,
                im: 1.6568542494923797,
            },
            Complex { re: -4.0, im: 0.0 },
            Complex {
                re: -3.9999999999999996,
                im: -1.6568542494923797,
            },
            Complex {
                re: -3.9999999999999996,
                im: -4.0,
            },
            Complex {
                re: -3.9999999999999987,
                im: -9.65685424949238,
            },
        ];
        assert_eq!(signal, ans);
        println!("{:?}", signal);
    }
}
