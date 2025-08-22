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

fn t_dtdb(status: &mut i32) {
    let dtdb = unsafe { H1::eraDtdb(2448939.5, 0.123, 0.76543, 5.0123, 5525.242, 3190.0) };

    vvd(
        dtdb,
        -0.1280368005936998991e-2,
        1e-15,
        "eraDtdb",
        "",
        status,
    );
}

fn t_dtf2d(status: &mut i32) {
    let mut u1: f64 = 0.0;
    let mut u2: f64 = 0.0;

    let c_scale = CString::new("UTC").expect("CString::new failed for a literal string");

    let j = unsafe {
        H1::eraDtf2d(
            c_scale.as_ptr(),
            1994,
            6,
            30,
            23,
            59,
            60.13599,
            &mut u1,
            &mut u2,
        )
    };

    vvd(u1 + u2, 2449534.49999, 1e-6, "eraDtf2d", "u", status);
    viv(j, 0, "eraDtf2d", "j", status);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        unsafe {
            VERBOSE = true;
        }
    }

    let mut status = 0;

    t_dtdb(&mut status);
    t_dtf2d(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
