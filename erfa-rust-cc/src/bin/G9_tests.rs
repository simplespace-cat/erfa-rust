#![allow(dead_code)]

use erfa_rust_cc::H1;
use libc::snprintf;
use std::ffi::{c_char, CString};

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
    let mut iy = 0;
    let mut im = 0;
    let mut id = 0;
    let mut ihmsf = [0i32; 4];

    let scale = CString::new("UTC").unwrap();

    let j = unsafe {
        H1::eraD2dtf(
            scale.as_ptr(),
            5,
            2400000.5,
            49533.99999,
            &mut iy,
            &mut im,
            &mut id,
            &mut ihmsf as *mut [i32; 4] as *mut i32,
        )
    };

    viv(iy, 1994, "eraD2dtf", "y", status);
    viv(im, 6, "eraD2dtf", "mo", status);
    viv(id, 30, "eraD2dtf", "d", status);
    viv(ihmsf[0], 23, "eraD2dtf", "h", status);
    viv(ihmsf[1], 59, "eraD2dtf", "m", status);
    viv(ihmsf[2], 60, "eraD2dtf", "s", status);
    viv(ihmsf[3], 13599, "eraD2dtf", "f", status);
    viv(j, 0, "eraD2dtf", "j", status);
}

fn t_d2tf(status: &mut i32) {
    let mut ihmsf = [0i32; 4];
    let mut s: c_char = 0;

    unsafe {
        H1::eraD2tf(
            4,
            -0.987654321,
            &mut s,
            &mut ihmsf as *mut [i32; 4] as *mut i32,
        );
    }

    viv(s as i32, '-' as i32, "eraD2tf", "s", status);

    viv(ihmsf[0], 23, "eraD2tf", "0", status);
    viv(ihmsf[1], 42, "eraD2tf", "1", status);
    viv(ihmsf[2], 13, "eraD2tf", "2", status);
    viv(ihmsf[3], 3333, "eraD2tf", "3", status);
}

fn t_dat(status: &mut i32) {
    let mut deltat = 0.0;

    let j1 = unsafe { H1::eraDat(2003, 6, 1, 0.0, &mut deltat) };
    vvd(deltat, 32.0, 0.0, "eraDat", "d1", status);
    viv(j1, 0, "eraDat", "j1", status);

    let j2 = unsafe { H1::eraDat(2008, 1, 17, 0.0, &mut deltat) };
    vvd(deltat, 33.0, 0.0, "eraDat", "d2", status);
    viv(j2, 0, "eraDat", "j2", status);

    let j3 = unsafe { H1::eraDat(2017, 9, 1, 0.0, &mut deltat) };
    vvd(deltat, 37.0, 0.0, "eraDat", "d3", status);
    viv(j3, 0, "eraDat", "j3", status);
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
