#![allow(dead_code)]

use erfa_rust::G23_safe;
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

fn t_nut00b(status: &mut i32) {
    let result = G23_safe::eraNut00b_safe(2400000.5, 53736.0);

    match result {
        Ok((dpsi, deps)) => {
            vvd(
                dpsi,
                -0.9632552291148362783e-5,
                1e-13,
                "eraNut00b_safe",
                "dpsi",
                status,
            );
            vvd(
                deps,
                0.4063197106621159367e-4,
                1e-13,
                "eraNut00b_safe",
                "deps",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraNut00b_safe failed: unexpected error");
        }
    }
}

fn t_nut06a(status: &mut i32) {
    let result = G23_safe::eraNut06a_safe(2400000.5, 53736.0);

    match result {
        Ok((dpsi, deps)) => {
            vvd(
                dpsi,
                -0.9630912025820308797e-5,
                1e-13,
                "eraNut06a_safe",
                "dpsi",
                status,
            );
            vvd(
                deps,
                0.4063238496887249798e-4,
                1e-13,
                "eraNut06a_safe",
                "deps",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraNut06a_safe failed: unexpected error");
        }
    }
}

fn t_nut80(status: &mut i32) {
    let result = G23_safe::eraNut80_safe(2400000.5, 53736.0);

    match result {
        Ok((dpsi, deps)) => {
            vvd(
                dpsi,
                -0.9643658353226563966e-5,
                1e-13,
                "eraNut80_safe",
                "dpsi",
                status,
            );
            vvd(
                deps,
                0.4060051006879713322e-4,
                1e-13,
                "eraNut80_safe",
                "deps",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraNut80_safe failed: unexpected error");
        }
    }
}

fn t_nutm80(status: &mut i32) {
    let result = G23_safe::eraNutm80_safe(2400000.5, 53736.0);

    match result {
        Ok(rmatn) => {
            vvd(
                rmatn[0][0],
                0.9999999999534999268,
                1e-12,
                "eraNutm80_safe",
                "11",
                status,
            );
            vvd(
                rmatn[0][1],
                0.8847935789636432161e-5,
                1e-12,
                "eraNutm80_safe",
                "12",
                status,
            );
            vvd(
                rmatn[0][2],
                0.3835906502164019142e-5,
                1e-12,
                "eraNutm80_safe",
                "13",
                status,
            );
            vvd(
                rmatn[1][0],
                -0.8847780042583435924e-5,
                1e-12,
                "eraNutm80_safe",
                "21",
                status,
            );
            vvd(
                rmatn[1][1],
                0.9999999991366569963,
                1e-12,
                "eraNutm80_safe",
                "22",
                status,
            );
            vvd(
                rmatn[1][2],
                -0.4060052702727130809e-4,
                1e-12,
                "eraNutm80_safe",
                "23",
                status,
            );
            vvd(
                rmatn[2][0],
                -0.3836265729708478796e-5,
                1e-12,
                "eraNutm80_safe",
                "31",
                status,
            );
            vvd(
                rmatn[2][1],
                0.4060049308612638555e-4,
                1e-12,
                "eraNutm80_safe",
                "32",
                status,
            );
            vvd(
                rmatn[2][2],
                0.9999999991684415129,
                1e-12,
                "eraNutm80_safe",
                "33",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraNutm80_safe failed: unexpected error");
        }
    }
}

fn t_obl06(status: &mut i32) {
    let result = G23_safe::eraObl06_safe(2400000.5, 54388.0);

    match result {
        Ok(obl) => {
            vvd(
                obl,
                0.4090749229387258204,
                1e-14,
                "eraObl06_safe",
                "",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraObl06_safe failed: unexpected error");
        }
    }
}

fn t_obl80(status: &mut i32) {
    let result = G23_safe::eraObl80_safe(2400000.5, 54388.0);

    match result {
        Ok(obl) => {
            vvd(
                obl,
                0.4090751347643816218,
                1e-14,
                "eraObl80_safe",
                "",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraObl80_safe failed: unexpected error");
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

    t_nut00b(&mut status);
    t_nut06a(&mut status);
    t_nut80(&mut status);
    t_nutm80(&mut status);
    t_obl06(&mut status);
    t_obl80(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
