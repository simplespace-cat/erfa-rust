#![allow(dead_code)]

use erfa_rust::G30_safe::*;
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

fn t_s06(status: &mut i32) {
    let x = 0.5791308486706011000e-3;
    let y = 0.4020579816732961219e-4;
    let s = eraS06_safe(2400000.5, 53736.0, x, y).unwrap();
    vvd(s, -0.1220032213076463117e-7, 1e-18, "eraS06", "", status);
}

fn t_s06a(status: &mut i32) {
    let s = eraS06a_safe(2400000.5, 52541.0).unwrap();
    vvd(s, -0.1340680437291812383e-7, 1e-18, "eraS06a", "", status);
}

fn t_sepp(status: &mut i32) {
    let a = [1.0, 0.1, 0.2];
    let b = [-3.0, 1e-3, 0.2];
    let s = eraSepp_safe(&a, &b).unwrap();
    vvd(s, 2.860391919024660768, 1e-12, "eraSepp", "", status);
}

fn t_seps(status: &mut i32) {
    let al = 1.0;
    let ap = 0.1;
    let bl = 0.2;
    let bp = -3.0;
    let s = eraSeps_safe(al, ap, bl, bp).unwrap();
    vvd(s, 2.346722016996998842, 1e-14, "eraSeps", "", status);
}

fn t_sp00(status: &mut i32) {
    let result = eraSp00_safe(2400000.5, 52541.0).unwrap();
    vvd(
        result,
        -0.6216698469981019309e-11,
        1e-12,
        "eraSp00",
        "",
        status,
    );
}

fn t_starpm(status: &mut i32) {
    let ra1 = 0.01686756;
    let dec1 = -1.093989828;
    let pmr1 = -1.78323516e-5;
    let pmd1 = 2.336024047e-6;
    let px1 = 0.74723;
    let rv1 = -21.6;

    let ((ra2, dec2, pmr2, pmd2, px2, rv2), j) = eraStarpm_safe(
        ra1, dec1, pmr1, pmd1, px1, rv1, 2400000.5, 50083.0, 2400000.5, 53736.0,
    )
    .unwrap();

    vvd(
        ra2,
        0.01668919069414256149,
        1e-13,
        "eraStarpm",
        "ra",
        status,
    );
    vvd(
        dec2,
        -1.093966454217127897,
        1e-13,
        "eraStarpm",
        "dec",
        status,
    );
    vvd(
        pmr2,
        -0.1783662682153176524e-4,
        1e-17,
        "eraStarpm",
        "pmr",
        status,
    );
    vvd(
        pmd2,
        0.2338092915983989595e-5,
        1e-17,
        "eraStarpm",
        "pmd",
        status,
    );
    vvd(px2, 0.7473533835317719243, 1e-13, "eraStarpm", "px", status);
    vvd(rv2, -21.59905170476417175, 1e-11, "eraStarpm", "rv", status);
    viv(j, 0, "eraStarpm", "j", status);
}

fn t_starpv(status: &mut i32) {
    let ra = 0.01686756;
    let dec = -1.093989828;
    let pmr = -1.78323516e-5;
    let pmd = 2.336024047e-6;
    let px = 0.74723;
    let rv = -21.6;

    let (pv, j) = eraStarpv_safe(ra, dec, pmr, pmd, px, rv).unwrap();

    vvd(
        pv[0][0],
        126668.5912743160601,
        1e-10,
        "eraStarpv",
        "11",
        status,
    );
    vvd(
        pv[0][1],
        2136.792716839935195,
        1e-12,
        "eraStarpv",
        "12",
        status,
    );
    vvd(
        pv[0][2],
        -245251.2339876830091,
        1e-10,
        "eraStarpv",
        "13",
        status,
    );
    vvd(
        pv[1][0],
        -0.4051854008955659551e-2,
        1e-13,
        "eraStarpv",
        "21",
        status,
    );
    vvd(
        pv[1][1],
        -0.6253919754414777970e-2,
        1e-15,
        "eraStarpv",
        "22",
        status,
    );
    vvd(
        pv[1][2],
        0.1189353714588109341e-1,
        1e-13,
        "eraStarpv",
        "23",
        status,
    );
    viv(j, 0, "eraStarpv", "j", status);
}

fn t_sxp(status: &mut i32) {
    let s = 2.0;
    let p = [0.3, 1.2, -2.5];
    let sp = eraSxp_safe(s, &p).unwrap();

    vvd(sp[0], 0.6, 0.0, "eraSxp", "1", status);
    vvd(sp[1], 2.4, 0.0, "eraSxp", "2", status);
    vvd(sp[2], -5.0, 0.0, "eraSxp", "3", status);
}

fn t_sxpv(status: &mut i32) {
    let s = 2.0;
    let pv = [[0.3, 1.2, -2.5], [0.5, 3.2, -0.7]];
    let spv = eraSxpv_safe(s, &pv).unwrap();

    vvd(spv[0][0], 0.6, 0.0, "eraSxpv", "p1", status);
    vvd(spv[0][1], 2.4, 0.0, "eraSxpv", "p2", status);
    vvd(spv[0][2], -5.0, 0.0, "eraSxpv", "p3", status);
    vvd(spv[1][0], 1.0, 0.0, "eraSxpv", "v1", status);
    vvd(spv[1][1], 6.4, 0.0, "eraSxpv", "v2", status);
    vvd(spv[1][2], -1.4, 0.0, "eraSxpv", "v3", status);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        unsafe {
            VERBOSE = true;
        }
    }

    let mut status = 0;

    t_s06a(&mut status);
    t_s06(&mut status);
    t_sepp(&mut status);
    t_seps(&mut status);
    t_sp00(&mut status);
    t_starpm(&mut status);
    t_starpv(&mut status);
    t_sxp(&mut status);
    t_sxpv(&mut status);

    if status != 0 {
        println!("G30_safe validation failed!");
    } else {
        println!("G30_safe validation successful");
    }
    std::process::exit(status);
}
