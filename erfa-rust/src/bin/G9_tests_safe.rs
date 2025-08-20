#![allow(dead_code)]

use erfa_rust::G9_safe;
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

fn t_d2dtf(status: &mut i32) {
    let result = G9_safe::eraD2dtf_safe("UTC", 5, 2400000.5, 49533.99999);

    match result {
        Ok(((iy, im, id), ihmsf, j)) => {
            viv(iy, 1994, "eraD2dtf_safe", "y", status);
            viv(im, 6, "eraD2dtf_safe", "mo", status);
            viv(id, 30, "eraD2dtf_safe", "d", status);
            viv(ihmsf[0], 23, "eraD2dtf_safe", "h", status);
            viv(ihmsf[1], 59, "eraD2dtf_safe", "m", status);
            viv(ihmsf[2], 60, "eraD2dtf_safe", "s", status);
            viv(ihmsf[3], 13599, "eraD2dtf_safe", "f", status);
            viv(j, 0, "eraD2dtf_safe", "j", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraD2dtf_safe failed: unexpected error");
        }
    }
}

fn t_d2tf(status: &mut i32) {
    let result = G9_safe::eraD2tf_safe(4, -0.987654321);

    match result {
        Ok((s, ihmsf)) => {
            viv(s as i32, '-' as i32, "eraD2tf_safe", "s", status);
            viv(ihmsf[0], 23, "eraD2tf_safe", "0", status);
            viv(ihmsf[1], 42, "eraD2tf_safe", "1", status);
            viv(ihmsf[2], 13, "eraD2tf_safe", "2", status);
            viv(ihmsf[3], 3333, "eraD2tf_safe", "3", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraD2tf_safe failed: unexpected error");
        }
    }
}

fn t_dat(status: &mut i32) {
    let result1 = G9_safe::eraDat_safe(2003, 6, 1, 0.0);
    match result1 {
        Ok((deltat, j)) => {
            vvd(deltat, 32.0, 0.0, "eraDat_safe", "d1", status);
            viv(j, 0, "eraDat_safe", "j1", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraDat_safe failed: unexpected error for 2003-06-01");
        }
    }

    let result2 = G9_safe::eraDat_safe(2008, 1, 17, 0.0);
    match result2 {
        Ok((deltat, j)) => {
            vvd(deltat, 33.0, 0.0, "eraDat_safe", "d2", status);
            viv(j, 0, "eraDat_safe", "j2", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraDat_safe failed: unexpected error for 2008-01-17");
        }
    }

    let result3 = G9_safe::eraDat_safe(2017, 9, 1, 0.0);
    match result3 {
        Ok((deltat, j)) => {
            vvd(deltat, 37.0, 0.0, "eraDat_safe", "d3", status);
            viv(j, 0, "eraDat_safe", "j3", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraDat_safe failed: unexpected error for 2017-09-01");
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

    t_d2dtf(&mut status);
    t_d2tf(&mut status);
    t_dat(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
