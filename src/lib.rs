//use num_complex::Complex64;

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

    return reverse;
}

pub fn reverse_bit_order(signal: &mut [f64], bits: u64) {
    let n = signal.len();
    for i in 0..(n) {
        let j = bit_reverse(i, bits);
        if i < j {
            signal.swap(i + 1, j + 1);
            //let tmp = signal[i + 1];
            //signal[i + 1] = signal[j + 1];
            //signal[j + 1] = tmp;
        }
    }
}
