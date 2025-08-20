#![allow(dead_code)]

use erfa_rust::G18_safe;
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

fn t_h2fk5(status: &mut i32) {
    let rh = 1.767794352;
    let dh = -0.2917512594;
    let drh = -2.76413026e-6;
    let ddh = -5.92994449e-6;
    let pxh = 0.379210;
    let rvh = -7.6;

    let result = G18_safe::eraH2fk5_safe(rh, dh, drh, ddh, pxh, rvh);

    match result {
        Ok((r5, d5, dr5, dd5, px5, rv5)) => {
            vvd(
                r5,
                1.767794455700065506,
                1e-13,
                "eraH2fk5_safe",
                "ra",
                status,
            );
            vvd(
                d5,
                -0.2917513626469638890,
                1e-13,
                "eraH2fk5_safe",
                "dec",
                status,
            );
            vvd(
                dr5,
                -0.27597945024511204e-5,
                1e-18,
                "eraH2fk5_safe",
                "dr5",
                status,
            );
            vvd(
                dd5,
                -0.59308014093262838e-5,
                1e-18,
                "eraH2fk5_safe",
                "dd5",
                status,
            );
            vvd(px5, 0.37921, 1e-13, "eraH2fk5_safe", "px", status);
            vvd(
                rv5,
                -7.6000001309071126,
                1e-11,
                "eraH2fk5_safe",
                "rv",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraH2fk5_safe failed: unexpected error");
        }
    }
}

fn t_hd2ae(status: &mut i32) {
    let h = 1.1;
    let d = 1.2;
    let p = 0.3;

    let result = G18_safe::eraHd2ae_safe(h, d, p);

    match result {
        Ok((a, e)) => {
            vvd(a, 5.916889243730066194, 1e-13, "eraHd2ae_safe", "a", status);
            vvd(
                e,
                0.4472186304990486228,
                1e-14,
                "eraHd2ae_safe",
                "e",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraHd2ae_safe failed: unexpected error");
        }
    }
}

fn t_hd2pa(status: &mut i32) {
    let h = 1.1;
    let d = 1.2;
    let p = 0.3;

    let result = G18_safe::eraHd2pa_safe(h, d, p);

    match result {
        Ok(q) => {
            vvd(q, 1.906227428001995580, 1e-13, "eraHd2pa_safe", "q", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraHd2pa_safe failed: unexpected error");
        }
    }
}

fn t_hfk5z(status: &mut i32) {
    let rh = 1.767794352;
    let dh = -0.2917512594;

    let result = G18_safe::eraHfk5z_safe(rh, dh, 2400000.5, 54479.0);

    match result {
        Ok((r5, d5, dr5, dd5)) => {
            vvd(
                r5,
                1.767794490535581026,
                1e-13,
                "eraHfk5z_safe",
                "ra",
                status,
            );
            vvd(
                d5,
                -0.2917513695320114258,
                1e-14,
                "eraHfk5z_safe",
                "dec",
                status,
            );
            vvd(
                dr5,
                0.4335890983539243029e-8,
                1e-22,
                "eraHfk5z_safe",
                "dr5",
                status,
            );
            vvd(
                dd5,
                -0.8569648841237745902e-9,
                1e-23,
                "eraHfk5z_safe",
                "dd5",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraHfk5z_safe failed: unexpected error");
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        unsafe {
            VERBOSE = true;
        }
    }

    let mut status = 0;

    t_h2fk5(&mut status);
    t_hd2ae(&mut status);
    t_hd2pa(&mut status);
    t_hfk5z(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
