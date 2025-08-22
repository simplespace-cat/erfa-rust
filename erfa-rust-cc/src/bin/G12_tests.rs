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

fn t_epb(status: &mut i32) {
    let epb = unsafe { H1::eraEpb(2415019.8135, 30103.18648) };
    vvd(epb, 1982.418424159278580, 1e-12, "eraEpb", "", status);
}

fn t_epb2jd(status: &mut i32) {
    let epb = 1957.3;
    let mut djm0 = 0.0;
    let mut djm = 0.0;

    unsafe {
        H1::eraEpb2jd(epb, &mut djm0, &mut djm);
    }

    vvd(djm0, 2400000.5, 1e-9, "eraEpb2jd", "djm0", status);
    vvd(djm, 35948.1915101513, 1e-9, "eraEpb2jd", "mjd", status);
}

fn t_epj(status: &mut i32) {
    let epj = unsafe { H1::eraEpj(2451545.0, -7392.5) };
    vvd(epj, 1979.760438056125941, 1e-12, "eraEpj", "", status);
}

fn t_epj2jd(status: &mut i32) {
    let epj = 1996.8;
    let mut djm0 = 0.0;
    let mut djm = 0.0;

    unsafe {
        H1::eraEpj2jd(epj, &mut djm0, &mut djm);
    }

    vvd(djm0, 2400000.5, 1e-9, "eraEpj2jd", "djm0", status);
    vvd(djm, 50375.7, 1e-9, "eraEpj2jd", "mjd", status);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        unsafe {
            VERBOSE = true;
        }
    }

    let mut status = 0;

    t_epb(&mut status);
    t_epb2jd(&mut status);
    t_epj(&mut status);
    t_epj2jd(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
