#![allow(dead_code)]

use erfa_rust::G14_safe;
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

    let result = G14_safe::eraEqec06_safe(date1, date2, dr, dd);
    match result {
        Ok((dl, db)) => {
            vvd(
                dl,
                1.342509918994654619,
                1e-14,
                "eraEqec06_safe",
                "dl",
                status,
            );
            vvd(
                db,
                0.5926215259704608132,
                1e-14,
                "eraEqec06_safe",
                "db",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraEqec06_safe failed: unexpected error");
        }
    }
}

fn t_eqeq94(status: &mut i32) {
    let result = G14_safe::eraEqeq94_safe(2400000.5, 41234.0);
    match result {
        Ok(eqeq) => {
            vvd(
                eqeq,
                0.5357758254609256894e-4,
                1e-17,
                "eraEqeq94_safe",
                "",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraEqeq94_safe failed: unexpected error");
        }
    }
}

fn t_era00(status: &mut i32) {
    let result = G14_safe::eraEra00_safe(2400000.5, 54388.0);
    match result {
        Ok(era00) => {
            vvd(
                era00,
                0.4022837240028158102,
                1e-12,
                "eraEra00_safe",
                "",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraEra00_safe failed: unexpected error");
        }
    }
}

fn t_version(status: &mut i32) {
    let version = G14_safe::eraVersion_safe();
    if unsafe { VERBOSE } {
        println!("eraVersion_safe: {}", version);
    }

    let major = G14_safe::eraVersionMajor_safe();
    let minor = G14_safe::eraVersionMinor_safe();
    let micro = G14_safe::eraVersionMicro_safe();

    if unsafe { VERBOSE } {
        println!("Version components: {}.{}.{}", major, minor, micro);
    }

    if major < 0 {
        *status = 1;
        println!(
            "eraVersionMajor_safe failed: invalid major version {}",
            major
        );
    }

    let sofa_version = G14_safe::eraSofaVersion_safe();
    if sofa_version != "unknown" {
        *status = 1;
        println!(
            "eraSofaVersion_safe failed: expected 'unknown', got '{}'",
            sofa_version
        );
    } else if unsafe { VERBOSE } {
        println!("eraSofaVersion_safe passed: {}", sofa_version);
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
