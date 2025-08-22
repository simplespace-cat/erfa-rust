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

fn t_icrs2g(status: &mut i32) {
    let dr = 5.9338074302227188048671087;
    let dd = -1.1784870613579944551540570;
    let mut dl = 0.0;
    let mut db = 0.0;

    unsafe {
        H1::eraIcrs2g(dr, dd, &mut dl, &mut db);
    }

    vvd(
        dl,
        5.5850536063818546461558,
        1e-14,
        "eraIcrs2g",
        "L",
        status,
    );
    vvd(
        db,
        -0.7853981633974483096157,
        1e-14,
        "eraIcrs2g",
        "B",
        status,
    );
}

fn t_ir(status: &mut i32) {
    let mut r = [[2.0, 3.0, 2.0], [3.0, 2.0, 3.0], [3.0, 4.0, 5.0]];

    unsafe {
        H1::eraIr(&mut r as *mut [[f64; 3]; 3] as *mut f64);
    }

    vvd(r[0][0], 1.0, 0.0, "eraIr", "11", status);
    vvd(r[0][1], 0.0, 0.0, "eraIr", "12", status);
    vvd(r[0][2], 0.0, 0.0, "eraIr", "13", status);
    vvd(r[1][0], 0.0, 0.0, "eraIr", "21", status);
    vvd(r[1][1], 1.0, 0.0, "eraIr", "22", status);
    vvd(r[1][2], 0.0, 0.0, "eraIr", "23", status);
    vvd(r[2][0], 0.0, 0.0, "eraIr", "31", status);
    vvd(r[2][1], 0.0, 0.0, "eraIr", "32", status);
    vvd(r[2][2], 1.0, 0.0, "eraIr", "33", status);
}

fn t_jd2cal(status: &mut i32) {
    let dj1 = 2400000.5;
    let dj2 = 50123.9999;
    let mut iy = 0;
    let mut im = 0;
    let mut id = 0;
    let mut fd = 0.0;

    let j = unsafe { H1::eraJd2cal(dj1, dj2, &mut iy, &mut im, &mut id, &mut fd) };

    viv(iy, 1996, "eraJd2cal", "y", status);
    viv(im, 2, "eraJd2cal", "m", status);
    viv(id, 10, "eraJd2cal", "d", status);
    vvd(fd, 0.9999, 1e-7, "eraJd2cal", "fd", status);
    viv(j, 0, "eraJd2cal", "j", status);
}

fn t_jdcalf(status: &mut i32) {
    let dj1 = 2400000.5;
    let dj2 = 50123.9999;
    let mut iydmf = [0; 4];

    let j = unsafe { H1::eraJdcalf(4, dj1, dj2, &mut iydmf as *mut [i32; 4] as *mut i32) };

    viv(iydmf[0], 1996, "eraJdcalf", "y", status);
    viv(iydmf[1], 2, "eraJdcalf", "m", status);
    viv(iydmf[2], 10, "eraJdcalf", "d", status);
    viv(iydmf[3], 9999, "eraJdcalf", "f", status);
    viv(j, 0, "eraJdcalf", "j", status);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        unsafe {
            VERBOSE = true;
        }
    }

    let mut status = 0;

    t_icrs2g(&mut status);
    t_ir(&mut status);
    t_jd2cal(&mut status);
    t_jdcalf(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
