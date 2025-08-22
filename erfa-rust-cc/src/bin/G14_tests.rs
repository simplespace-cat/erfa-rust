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

fn t_eqec06(status: &mut i32) {
    let date1 = 1234.5;
    let date2 = 2440000.5;
    let dr = 1.234;
    let dd = 0.987;
    let mut dl = 0.0;
    let mut db = 0.0;

    unsafe {
        H1::eraEqec06(date1, date2, dr, dd, &mut dl, &mut db);
    }

    vvd(dl, 1.342509918994654619, 1e-14, "eraEqec06", "dl", status);
    vvd(db, 0.5926215259704608132, 1e-14, "eraEqec06", "db", status);
}

fn t_eqeq94(status: &mut i32) {
    let eqeq = unsafe { H1::eraEqeq94(2400000.5, 41234.0) };
    vvd(
        eqeq,
        0.5357758254609256894e-4,
        1e-17,
        "eraEqeq94",
        "",
        status,
    );
}

fn t_era00(status: &mut i32) {
    let era00 = unsafe { H1::eraEra00(2400000.5, 54388.0) };
    vvd(era00, 0.4022837240028158102, 1e-12, "eraEra00", "", status);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        unsafe {
            VERBOSE = true;
        }
    }

    let mut status = 0;

    t_eqec06(&mut status);
    t_eqeq94(&mut status);
    t_era00(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
