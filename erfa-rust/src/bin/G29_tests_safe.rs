#![allow(dead_code)]

use erfa_rust::G29_safe::*;
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

fn t_s00(status: &mut i32) {
    let x = 0.5791308486706011000e-3;
    let y = 0.4020579816732961219e-4;

    let s = eraS00_safe(2400000.5, 53736.0, x, y).unwrap();

    vvd(s, -0.1220036263270905693e-7, 1e-18, "eraS00", "", status);
}

fn t_s00a(status: &mut i32) {
    let s = eraS00a_safe(2400000.5, 52541.0).unwrap();
    vvd(s, -0.1340684448919163584e-7, 1e-18, "eraS00a", "", status);
}

fn t_s00b(status: &mut i32) {
    let s = eraS00b_safe(2400000.5, 52541.0).unwrap();
    vvd(s, -0.1340695782951026584e-7, 1e-18, "eraS00b", "", status);
}

fn t_s2c(status: &mut i32) {
    let c = eraS2c_safe(3.0123, -0.999).unwrap();

    vvd(c[0], -0.5366267667260523906, 1e-12, "eraS2c", "1", status);
    vvd(c[1], 0.0697711109765145365, 1e-12, "eraS2c", "2", status);
    vvd(c[2], -0.8409302618566214041, 1e-12, "eraS2c", "3", status);
}

fn t_s2p(status: &mut i32) {
    let p = eraS2p_safe(-3.21, 0.123, 0.456).unwrap();

    vvd(p[0], -0.4514964673880165228, 1e-12, "eraS2p", "x", status);
    vvd(p[1], 0.0309339427734258688, 1e-12, "eraS2p", "y", status);
    vvd(p[2], 0.0559466810510877933, 1e-12, "eraS2p", "z", status);
}

fn t_s2pv(status: &mut i32) {
    let pv = eraS2pv_safe(-3.21, 0.123, 0.456, -7.8e-6, 9.01e-6, -1.23e-5).unwrap();

    vvd(
        pv[0][0],
        -0.4514964673880165228,
        1e-12,
        "eraS2pv",
        "x",
        status,
    );
    vvd(
        pv[0][1],
        0.0309339427734258688,
        1e-12,
        "eraS2pv",
        "y",
        status,
    );
    vvd(
        pv[0][2],
        0.0559466810510877933,
        1e-12,
        "eraS2pv",
        "z",
        status,
    );
    vvd(
        pv[1][0],
        0.1292270850663260170e-4,
        1e-16,
        "eraS2pv",
        "vx",
        status,
    );
    vvd(
        pv[1][1],
        0.2652814182060691422e-5,
        1e-16,
        "eraS2pv",
        "vy",
        status,
    );
    vvd(
        pv[1][2],
        0.2568431853930292259e-5,
        1e-16,
        "eraS2pv",
        "vz",
        status,
    );
}

fn t_s2xpv(status: &mut i32) {
    let s1 = 2.0;
    let s2 = 3.0;
    let pv = [[0.3, 1.2, -2.5], [0.5, 2.3, -0.4]];

    let spv = eraS2xpv_safe(s1, s2, &pv).unwrap();

    vvd(spv[0][0], 0.6, 1e-12, "eraS2xpv", "p1", status);
    vvd(spv[0][1], 2.4, 1e-12, "eraS2xpv", "p2", status);
    vvd(spv[0][2], -5.0, 1e-12, "eraS2xpv", "p3", status);
    vvd(spv[1][0], 1.5, 1e-12, "eraS2xpv", "v1", status);
    vvd(spv[1][1], 6.9, 1e-12, "eraS2xpv", "v2", status);
    vvd(spv[1][2], -1.2, 1e-12, "eraS2xpv", "v3", status);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        unsafe {
            VERBOSE = true;
        }
    }

    let mut status = 0;

    t_s00a(&mut status);
    t_s00b(&mut status);
    t_s00(&mut status);
    t_s2c(&mut status);
    t_s2p(&mut status);
    t_s2pv(&mut status);
    t_s2xpv(&mut status);

    if status != 0 {
        println!("G29_safe validation failed!");
    } else {
        println!("G29_safe validation successful");
    }
    std::process::exit(status);
}
