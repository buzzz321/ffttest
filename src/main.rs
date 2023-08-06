use ffttest::fftiter;
use std::f64::consts::PI;

use num_complex::Complex64;
const N: usize = 8;

// copied from rosetta code go version.
fn fft(signal: &mut [f64], out_fft: &mut [Complex64], n: usize, offset: usize) {
    if n == 1 {
        out_fft[0].re = signal[0];
        out_fft[0].im = 0.0;
        return;
    }
    fft(signal, out_fft, n / 2, 2 * offset); //get even indexes
    fft(
        &mut signal[offset..],
        &mut out_fft[(n / 2)..],
        n / 2,
        2 * offset,
    ); // get odd indexes
       //println!("-> siglen {}, fftlen {}", signal.len(), out_fft.len());
    let mut k = 0;
    //println!("{:?}", signal);
    while k < (n / 2) {
        let tf =
            Complex64::from_polar(1.0, -2.0 * PI * (k as f64) / (n as f64)) * out_fft[k + (n / 2)];
        let tmp = out_fft[k];
        out_fft[k] = tmp + tf;
        out_fft[k + n / 2] = tmp - tf;

        k += 0x1;
    }
}

fn main() {
    let mut signal = [1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0];
    let mut out_fft: [Complex64; N] = [Complex64::new(0.0, 0.0); N];

    fft(&mut signal, &mut out_fft, N, 1);
    for val in out_fft {
        println!("The FFT is {:?}", val);
    }

    for (idx, item) in signal.iter().enumerate() {
        out_fft[idx] = Complex64::new(*item, 0.0);
    }

    println!("{:?}", signal);

    fftiter(&mut out_fft);
    for val in out_fft {
        println!("The FFT is {:?}", val);
    }
}
