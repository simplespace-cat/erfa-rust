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

fn t_epv00(status: &mut i32) {
    let mut pvh = [[0.0; 3]; 2];
    let mut pvb = [[0.0; 3]; 2];

    let j = unsafe {
        H1::eraEpv00(
            2400000.5,
            53411.52501161,
            &mut pvh as *mut [[f64; 3]; 2] as *mut f64,
            &mut pvb as *mut [[f64; 3]; 2] as *mut f64,
        )
    };

    vvd(
        pvh[0][0],
        -0.7757238809297706813,
        1e-14,
        "eraEpv00",
        "ph(x)",
        status,
    );
    vvd(
        pvh[0][1],
        0.5598052241363340596,
        1e-14,
        "eraEpv00",
        "ph(y)",
        status,
    );
    vvd(
        pvh[0][2],
        0.2426998466481686993,
        1e-14,
        "eraEpv00",
        "ph(z)",
        status,
    );

    vvd(
        pvh[1][0],
        -0.1091891824147313846e-1,
        1e-15,
        "eraEpv00",
        "vh(x)",
        status,
    );
    vvd(
        pvh[1][1],
        -0.1247187268440845008e-1,
        1e-15,
        "eraEpv00",
        "vh(y)",
        status,
    );
    vvd(
        pvh[1][2],
        -0.5407569418065039061e-2,
        1e-15,
        "eraEpv00",
        "vh(z)",
        status,
    );

    vvd(
        pvb[0][0],
        -0.7714104440491111971,
        1e-14,
        "eraEpv00",
        "pb(x)",
        status,
    );
    vvd(
        pvb[0][1],
        0.5598412061824171323,
        1e-14,
        "eraEpv00",
        "pb(y)",
        status,
    );
    vvd(
        pvb[0][2],
        0.2425996277722452400,
        1e-14,
        "eraEpv00",
        "pb(z)",
        status,
    );

    vvd(
        pvb[1][0],
        -0.1091874268116823295e-1,
        1e-15,
        "eraEpv00",
        "vb(x)",
        status,
    );
    vvd(
        pvb[1][1],
        -0.1246525461732861538e-1,
        1e-15,
        "eraEpv00",
        "vb(y)",
        status,
    );
    vvd(
        pvb[1][2],
        -0.5404773180966231279e-2,
        1e-15,
        "eraEpv00",
        "vb(z)",
        status,
    );

    viv(j, 0, "eraEpv00", "j", status);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        if args.len() > 1 {
            unsafe {
                VERBOSE = true;
            }
        }
    }

    let mut status = 0;

    t_epv00(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
