#![allow(dead_code)]

use erfa_rust_cc::H1;
use libc::{c_char, snprintf};
use std::ffi::CString;

static mut VERBOSE: bool = false;

fn format_g(val: f64, precision: usize) -> String {
    let mut buffer = vec![0u8; 512];
    let format_str = format!("%.{}g", precision);
    let c_format_str = match CString::new(format_str) {
        Ok(s) => s,
        Err(e) => {
            panic!("FATAL: Failed to create CString for formatting. This should be impossible. Error: {:?}", e);
        }
    };
    unsafe {
        snprintf(
            buffer.as_mut_ptr() as *mut c_char,
            buffer.len(),
            c_format_str.as_ptr(),
            val,
        );
    }
    String::from_utf8_lossy(&buffer)
        .trim_end_matches('\0')
        .to_string()
}

fn viv(ival: i32, ivalok: i32, func: &str, test: &str, status: &mut i32) {
    if ival != ivalok {
        *status = 1;
        println!("{} failed: {} want {} got {}", func, test, ivalok, ival);
    } else if unsafe { VERBOSE } {
        println!("{} passed: {} want {} got {}", func, test, ivalok, ival);
    }
}

fn vvd(val: f64, valok: f64, dval: f64, func: &str, test: &str, status: &mut i32) {
    let a = val - valok;
    if a != 0.0 && a.abs() > dval.abs() {
        let f = (valok / a).abs();
        *status = 1;
        println!(
            "{} failed: {} want {} got {} (1/{})",
            func,
            test,
            format_g(valok, 20),
            format_g(val, 20),
            format_g(f, 3)
        );
    } else if unsafe { VERBOSE } {
        println!(
            "{} passed: {} want {} got {}",
            func,
            test,
            format_g(valok, 20),
            format_g(val, 20)
        );
    }
}

fn t_refco(status: &mut i32) {
    let phpa = 800.0;
    let tc = 10.0;
    let rh = 0.9;
    let wl = 0.4;
    let mut refa: f64 = 0.0;
    let mut refb: f64 = 0.0;

    unsafe {
        H1::eraRefco(
            phpa,
            tc,
            rh,
            wl,
            &mut refa as *mut f64,
            &mut refb as *mut f64,
        );
    }

    vvd(
        refa,
        0.2264949956241415009e-3,
        1e-15,
        "eraRefco",
        "refa",
        status,
    );
    vvd(
        refb,
        -0.2598658261729343970e-6,
        1e-18,
        "eraRefco",
        "refb",
        status,
    );
}

fn t_rm2v(status: &mut i32) {
    let mut r = [[0.0, -0.8, -0.6], [0.8, -0.36, 0.48], [0.6, 0.48, -0.64]];
    let mut w = [0.0; 3];

    unsafe {
        H1::eraRm2v(
            &mut r as *mut [[f64; 3]; 3] as *mut f64,
            &mut w as *mut [f64; 3] as *mut f64,
        );
    }

    vvd(w[0], 0.0, 1e-12, "eraRm2v", "1", status);
    vvd(w[1], 1.413716694115406957, 1e-12, "eraRm2v", "2", status);
    vvd(w[2], -1.884955592153875943, 1e-12, "eraRm2v", "3", status);
}

fn t_rv2m(status: &mut i32) {
    let mut w = [0.0, 1.41371669, -1.88495559];
    let mut r = [[0.0; 3]; 3];

    unsafe {
        H1::eraRv2m(
            &mut w as *mut [f64; 3] as *mut f64,
            &mut r as *mut [[f64; 3]; 3] as *mut f64,
        );
    }

    vvd(
        r[0][0],
        -0.7071067782221119905,
        1e-14,
        "eraRv2m",
        "11",
        status,
    );
    vvd(
        r[0][1],
        -0.5656854276809129651,
        1e-14,
        "eraRv2m",
        "12",
        status,
    );
    vvd(
        r[0][2],
        -0.4242640700104211225,
        1e-14,
        "eraRv2m",
        "13",
        status,
    );
    vvd(
        r[1][0],
        0.5656854276809129651,
        1e-14,
        "eraRv2m",
        "21",
        status,
    );
    vvd(
        r[1][1],
        -0.0925483394532274246,
        1e-14,
        "eraRv2m",
        "22",
        status,
    );
    vvd(
        r[1][2],
        -0.8194112531408833269,
        1e-14,
        "eraRv2m",
        "23",
        status,
    );
    vvd(
        r[2][0],
        0.4242640700104211225,
        1e-14,
        "eraRv2m",
        "31",
        status,
    );
    vvd(
        r[2][1],
        -0.8194112531408833269,
        1e-14,
        "eraRv2m",
        "32",
        status,
    );
    vvd(
        r[2][2],
        0.3854415612311154341,
        1e-14,
        "eraRv2m",
        "33",
        status,
    );
}

fn t_rx(status: &mut i32) {
    let phi = 0.3456789;
    let mut r = [[2.0, 3.0, 2.0], [3.0, 2.0, 3.0], [3.0, 4.0, 5.0]];

    unsafe {
        H1::eraRx(phi, &mut r as *mut [[f64; 3]; 3] as *mut f64);
    }

    vvd(r[0][0], 2.0, 0.0, "eraRx", "11", status);
    vvd(r[0][1], 3.0, 0.0, "eraRx", "12", status);
    vvd(r[0][2], 2.0, 0.0, "eraRx", "13", status);
    vvd(r[1][0], 3.839043388235612460, 1e-12, "eraRx", "21", status);
    vvd(r[1][1], 3.237033249594111899, 1e-12, "eraRx", "22", status);
    vvd(r[1][2], 4.516714379005982719, 1e-12, "eraRx", "23", status);
    vvd(r[2][0], 1.806030415924501684, 1e-12, "eraRx", "31", status);
    vvd(r[2][1], 3.085711545336372503, 1e-12, "eraRx", "32", status);
    vvd(r[2][2], 3.687721683977873065, 1e-12, "eraRx", "33", status);
}

fn t_rxp(status: &mut i32) {
    let mut r = [[2.0, 3.0, 2.0], [3.0, 2.0, 3.0], [3.0, 4.0, 5.0]];
    let mut p = [0.2, 1.5, 0.1];
    let mut rp = [0.0; 3];

    unsafe {
        H1::eraRxp(
            &mut r as *mut [[f64; 3]; 3] as *mut f64,
            &mut p as *mut [f64; 3] as *mut f64,
            &mut rp as *mut [f64; 3] as *mut f64,
        );
    }

    vvd(rp[0], 5.1, 1e-12, "eraRxp", "1", status);
    vvd(rp[1], 3.9, 1e-12, "eraRxp", "2", status);
    vvd(rp[2], 7.1, 1e-12, "eraRxp", "3", status);
}

fn t_rxpv(status: &mut i32) {
    let mut r = [[2.0, 3.0, 2.0], [3.0, 2.0, 3.0], [3.0, 4.0, 5.0]];
    let mut pv = [[0.2, 1.5, 0.1], [1.5, 0.2, 0.1]];
    let mut rpv = [[0.0; 3]; 2];

    unsafe {
        H1::eraRxpv(
            &mut r as *mut [[f64; 3]; 3] as *mut f64,
            &mut pv as *mut [[f64; 3]; 2] as *mut f64,
            &mut rpv as *mut [[f64; 3]; 2] as *mut f64,
        );
    }

    vvd(rpv[0][0], 5.1, 1e-12, "eraRxpv", "11", status);
    vvd(rpv[1][0], 3.8, 1e-12, "eraRxpv", "12", status);
    vvd(rpv[0][1], 3.9, 1e-12, "eraRxpv", "21", status);
    vvd(rpv[1][1], 5.2, 1e-12, "eraRxpv", "22", status);
    vvd(rpv[0][2], 7.1, 1e-12, "eraRxpv", "31", status);
    vvd(rpv[1][2], 5.8, 1e-12, "eraRxpv", "32", status);
}

fn t_rxr(status: &mut i32) {
    let mut a = [[2.0, 3.0, 2.0], [3.0, 2.0, 3.0], [3.0, 4.0, 5.0]];
    let mut b = [[1.0, 2.0, 2.0], [4.0, 1.0, 1.0], [3.0, 0.0, 1.0]];
    let mut atb = [[0.0; 3]; 3];

    unsafe {
        H1::eraRxr(
            &mut a as *mut [[f64; 3]; 3] as *mut f64,
            &mut b as *mut [[f64; 3]; 3] as *mut f64,
            &mut atb as *mut [[f64; 3]; 3] as *mut f64,
        );
    }

    vvd(atb[0][0], 20.0, 1e-12, "eraRxr", "11", status);
    vvd(atb[0][1], 7.0, 1e-12, "eraRxr", "12", status);
    vvd(atb[0][2], 9.0, 1e-12, "eraRxr", "13", status);
    vvd(atb[1][0], 20.0, 1e-12, "eraRxr", "21", status);
    vvd(atb[1][1], 8.0, 1e-12, "eraRxr", "22", status);
    vvd(atb[1][2], 11.0, 1e-12, "eraRxr", "23", status);
    vvd(atb[2][0], 34.0, 1e-12, "eraRxr", "31", status);
    vvd(atb[2][1], 10.0, 1e-12, "eraRxr", "32", status);
    vvd(atb[2][2], 15.0, 1e-12, "eraRxr", "33", status);
}

fn t_ry(status: &mut i32) {
    let theta = 0.3456789;
    let mut r = [[2.0, 3.0, 2.0], [3.0, 2.0, 3.0], [3.0, 4.0, 5.0]];

    unsafe {
        H1::eraRy(theta, &mut r as *mut [[f64; 3]; 3] as *mut f64);
    }

    vvd(r[0][0], 0.8651847818978159930, 1e-12, "eraRy", "11", status);
    vvd(r[0][1], 1.467194920539316554, 1e-12, "eraRy", "12", status);
    vvd(r[0][2], 0.1875137911274457342, 1e-12, "eraRy", "13", status);
    vvd(r[1][0], 3.0, 1e-12, "eraRy", "21", status);
    vvd(r[1][1], 2.0, 1e-12, "eraRy", "22", status);
    vvd(r[1][2], 3.0, 1e-12, "eraRy", "23", status);
    vvd(r[2][0], 3.500207892850427330, 1e-12, "eraRy", "31", status);
    vvd(r[2][1], 4.779889022262298150, 1e-12, "eraRy", "32", status);
    vvd(r[2][2], 5.381899160903798712, 1e-12, "eraRy", "33", status);
}

fn t_rz(status: &mut i32) {
    let psi = 0.3456789;
    let mut r = [[2.0, 3.0, 2.0], [3.0, 2.0, 3.0], [3.0, 4.0, 5.0]];

    unsafe {
        H1::eraRz(psi, &mut r as *mut [[f64; 3]; 3] as *mut f64);
    }

    vvd(r[0][0], 2.898197754208926769, 1e-12, "eraRz", "11", status);
    vvd(r[0][1], 3.500207892850427330, 1e-12, "eraRz", "12", status);
    vvd(r[0][2], 2.898197754208926769, 1e-12, "eraRz", "13", status);
    vvd(r[1][0], 2.144865911309686813, 1e-12, "eraRz", "21", status);
    vvd(r[1][1], 0.865184781897815993, 1e-12, "eraRz", "22", status);
    vvd(r[1][2], 2.144865911309686813, 1e-12, "eraRz", "23", status);
    vvd(r[2][0], 3.0, 1e-12, "eraRz", "31", status);
    vvd(r[2][1], 4.0, 1e-12, "eraRz", "32", status);
    vvd(r[2][2], 5.0, 1e-12, "eraRz", "33", status);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        unsafe {
            VERBOSE = true;
        }
    }

    let mut status = 0;

    t_refco(&mut status);
    t_rm2v(&mut status);
    t_rv2m(&mut status);
    t_rx(&mut status);
    t_rxp(&mut status);
    t_rxpv(&mut status);
    t_rxr(&mut status);
    t_ry(&mut status);
    t_rz(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
