#![allow(dead_code)]

use erfa_rust::G12_safe;
use libc::{c_char, snprintf};
use std::ffi::CString;

static mut VERBOSE: bool = false;

fn format_g(val: f64, precision: usize) -> String {
    let mut buffer = vec![0u8; 512];
    let format_str = format!("%.{}g", precision);
    let c_format_str = CString::new(format_str).expect("format CString");
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
    let epb = G12_safe::eraEpb_safe(2415019.8135, 30103.18648).expect("eraEpb_safe");
    vvd(epb, 1982.418424159278580, 1e-12, "eraEpb_safe", "", status);
}

fn t_epb2jd(status: &mut i32) {
    let epb = 1957.3;
    let (djm0, djm) = G12_safe::eraEpb2jd_safe(epb).expect("eraEpb2jd_safe");

    vvd(djm0, 2400000.5, 1e-9, "eraEpb2jd_safe", "djm0", status);
    vvd(djm, 35948.1915101513, 1e-9, "eraEpb2jd_safe", "mjd", status);
}

fn t_epj(status: &mut i32) {
    let epj = G12_safe::eraEpj_safe(2451545.0, -7392.5).expect("eraEpj_safe");
    vvd(epj, 1979.760438056125941, 1e-12, "eraEpj_safe", "", status);
}

fn t_epj2jd(status: &mut i32) {
    let epj = 1996.8;
    let (djm0, djm) = G12_safe::eraEpj2jd_safe(epj).expect("eraEpj2jd_safe");

    vvd(djm0, 2400000.5, 1e-9, "eraEpj2jd_safe", "djm0", status);
    vvd(djm, 50375.7, 1e-9, "eraEpj2jd_safe", "mjd", status);
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
