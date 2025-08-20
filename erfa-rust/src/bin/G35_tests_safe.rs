#![allow(dead_code)]

use erfa_rust::G35_safe::*;
use libc::{c_char, snprintf};
use std::ffi::CString;

static mut VERBOSE: bool = false;

fn format_g(val: f64, precision: usize) -> String {
    let mut buffer = vec![0u8; 512];
    let format_str = format!("%.{}g", precision);
    let c_format_str = CString::new(format_str).unwrap();
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

fn t_xys00a(status: &mut i32) {
    let (x, y, s) = eraXys00a_safe(2400000.5, 53736.0).unwrap();

    vvd(x, 0.5791308472168152904e-3, 1e-14, "eraXys00a", "x", status);
    vvd(y, 0.4020595661591500259e-4, 1e-15, "eraXys00a", "y", status);
    vvd(
        s,
        -0.1220040848471549623e-7,
        1e-18,
        "eraXys00a",
        "s",
        status,
    );
}

fn t_xys00b(status: &mut i32) {
    let (x, y, s) = eraXys00b_safe(2400000.5, 53736.0).unwrap();

    vvd(x, 0.5791301929950208873e-3, 1e-14, "eraXys00b", "x", status);
    vvd(y, 0.4020553681373720832e-4, 1e-15, "eraXys00b", "y", status);
    vvd(
        s,
        -0.1220027377285083189e-7,
        1e-18,
        "eraXys00b",
        "s",
        status,
    );
}

fn t_xys06a(status: &mut i32) {
    let (x, y, s) = eraXys06a_safe(2400000.5, 53736.0).unwrap();

    vvd(x, 0.5791308482835292617e-3, 1e-14, "eraXys06a", "x", status);
    vvd(y, 0.4020580099454020310e-4, 1e-15, "eraXys06a", "y", status);
    vvd(
        s,
        -0.1220032294164579896e-7,
        1e-18,
        "eraXys06a",
        "s",
        status,
    );
}

fn t_zp(status: &mut i32) {
    let p = eraZp_safe();

    vvd(p[0], 0.0, 0.0, "eraZp", "1", status);
    vvd(p[1], 0.0, 0.0, "eraZp", "2", status);
    vvd(p[2], 0.0, 0.0, "eraZp", "3", status);
}

fn t_zpv(status: &mut i32) {
    let pv = eraZpv_safe();

    vvd(pv[0][0], 0.0, 0.0, "eraZpv", "p1", status);
    vvd(pv[0][1], 0.0, 0.0, "eraZpv", "p2", status);
    vvd(pv[0][2], 0.0, 0.0, "eraZpv", "p3", status);
    vvd(pv[1][0], 0.0, 0.0, "eraZpv", "v1", status);
    vvd(pv[1][1], 0.0, 0.0, "eraZpv", "v2", status);
    vvd(pv[1][2], 0.0, 0.0, "eraZpv", "v3", status);
}

fn t_zr(status: &mut i32) {
    let r = eraZr_safe();

    vvd(r[0][0], 0.0, 0.0, "eraZr", "00", status);
    vvd(r[1][0], 0.0, 0.0, "eraZr", "01", status);
    vvd(r[2][0], 0.0, 0.0, "eraZr", "02", status);
    vvd(r[0][1], 0.0, 0.0, "eraZr", "10", status);
    vvd(r[1][1], 0.0, 0.0, "eraZr", "11", status);
    vvd(r[2][1], 0.0, 0.0, "eraZr", "12", status);
    vvd(r[0][2], 0.0, 0.0, "eraZr", "20", status);
    vvd(r[1][2], 0.0, 0.0, "eraZr", "21", status);
    vvd(r[2][2], 0.0, 0.0, "eraZr", "22", status);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        unsafe {
            VERBOSE = true;
        }
    }

    let mut status = 0;

    t_xys00a(&mut status);
    t_xys00b(&mut status);
    t_xys06a(&mut status);
    t_zp(&mut status);
    t_zpv(&mut status);
    t_zr(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
