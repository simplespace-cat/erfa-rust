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

fn t_fk425(status: &mut i32) {
    let r1950 = 0.07626899753879587532;
    let d1950 = -1.137405378399605780;
    let dr1950 = 0.1973749217849087460e-4;
    let dd1950 = 0.5659714913272723189e-5;
    let p1950 = 0.134;
    let v1950 = 8.7;

    let mut r2000 = 0.0;
    let mut d2000 = 0.0;
    let mut dr2000 = 0.0;
    let mut dd2000 = 0.0;
    let mut p2000 = 0.0;
    let mut v2000 = 0.0;

    unsafe {
        H1::eraFk425(
            r1950,
            d1950,
            dr1950,
            dd1950,
            p1950,
            v1950,
            &mut r2000,
            &mut d2000,
            &mut dr2000,
            &mut dd2000,
            &mut p2000,
            &mut v2000,
        );
    }

    vvd(
        r2000,
        0.08757989933556446040,
        1e-14,
        "eraFk425",
        "r2000",
        status,
    );
    vvd(
        d2000,
        -1.132279113042091895,
        1e-12,
        "eraFk425",
        "d2000",
        status,
    );
    vvd(
        dr2000,
        0.1953670614474396139e-4,
        1e-17,
        "eraFk425",
        "dr2000",
        status,
    );
    vvd(
        dd2000,
        0.5637686678659640164e-5,
        1e-18,
        "eraFk425",
        "dd2000",
        status,
    );
    vvd(
        p2000,
        0.1339919950582767871,
        1e-13,
        "eraFk425",
        "p2000",
        status,
    );
    vvd(
        v2000,
        8.736999669183529069,
        1e-12,
        "eraFk425",
        "v2000",
        status,
    );
}

fn t_fk45z(status: &mut i32) {
    let r1950 = 0.01602284975382960982;
    let d1950 = -0.1164347929099906024;
    let bepoch = 1954.677617625256806;

    let mut r2000 = 0.0;
    let mut d2000 = 0.0;

    unsafe {
        H1::eraFk45z(r1950, d1950, bepoch, &mut r2000, &mut d2000);
    }

    vvd(
        r2000,
        0.02719295911606862303,
        1e-15,
        "eraFk45z",
        "r2000",
        status,
    );
    vvd(
        d2000,
        -0.1115766001565926892,
        1e-13,
        "eraFk45z",
        "d2000",
        status,
    );
}

fn t_fk524(status: &mut i32) {
    let r2000 = 0.8723503576487275595;
    let d2000 = -0.7517076365138887672;
    let dr2000 = 0.2019447755430472323e-4;
    let dd2000 = 0.3541563940505160433e-5;
    let p2000 = 0.1559;
    let v2000 = 86.87;

    let mut r1950 = 0.0;
    let mut d1950 = 0.0;
    let mut dr1950 = 0.0;
    let mut dd1950 = 0.0;
    let mut p1950 = 0.0;
    let mut v1950 = 0.0;

    unsafe {
        H1::eraFk524(
            r2000,
            d2000,
            dr2000,
            dd2000,
            p2000,
            v2000,
            &mut r1950,
            &mut d1950,
            &mut dr1950,
            &mut dd1950,
            &mut p1950,
            &mut v1950,
        );
    }

    vvd(
        r1950,
        0.8636359659799603487,
        1e-13,
        "eraFk524",
        "r1950",
        status,
    );
    vvd(
        d1950,
        -0.7550281733160843059,
        1e-13,
        "eraFk524",
        "d1950",
        status,
    );
    vvd(
        dr1950,
        0.2023628192747172486e-4,
        1e-17,
        "eraFk524",
        "dr1950",
        status,
    );
    vvd(
        dd1950,
        0.3624459754935334718e-5,
        1e-18,
        "eraFk524",
        "dd1950",
        status,
    );
    vvd(
        p1950,
        0.1560079963299390241,
        1e-13,
        "eraFk524",
        "p1950",
        status,
    );
    vvd(
        v1950,
        86.79606353469163751,
        1e-11,
        "eraFk524",
        "v1950",
        status,
    );
}

fn t_fk52h(status: &mut i32) {
    let r5 = 1.76779433;
    let d5 = -0.2917517103;
    let dr5 = -1.91851572e-7;
    let dd5 = -5.8468475e-6;
    let px5 = 0.379210;
    let rv5 = -7.6;

    let mut rh = 0.0;
    let mut dh = 0.0;
    let mut drh = 0.0;
    let mut ddh = 0.0;
    let mut pxh = 0.0;
    let mut rvh = 0.0;

    unsafe {
        H1::eraFk52h(
            r5, d5, dr5, dd5, px5, rv5, &mut rh, &mut dh, &mut drh, &mut ddh, &mut pxh, &mut rvh,
        );
    }

    vvd(rh, 1.767794226299947632, 1e-14, "eraFk52h", "ra", status);
    vvd(dh, -0.2917516070530391757, 1e-14, "eraFk52h", "dec", status);
    vvd(
        drh,
        -0.1961874125605721270e-6,
        1e-19,
        "eraFk52h",
        "dr5",
        status,
    );
    vvd(
        ddh,
        -0.58459905176693911e-5,
        1e-19,
        "eraFk52h",
        "dd5",
        status,
    );
    vvd(pxh, 0.37921, 1e-14, "eraFk52h", "px", status);
    vvd(rvh, -7.6000000940000254, 1e-11, "eraFk52h", "rv", status);
}

fn t_fk54z(status: &mut i32) {
    let r2000 = 0.02719026625066316119;
    let d2000 = -0.1115815170738754813;
    let bepoch = 1954.677308160316374;

    let mut r1950 = 0.0;
    let mut d1950 = 0.0;
    let mut dr1950 = 0.0;
    let mut dd1950 = 0.0;

    unsafe {
        H1::eraFk54z(
            r2000,
            d2000,
            bepoch,
            &mut r1950,
            &mut d1950,
            &mut dr1950,
            &mut dd1950,
        );
    }

    vvd(
        r1950,
        0.01602015588390065476,
        1e-14,
        "eraFk54z",
        "r1950",
        status,
    );
    vvd(
        d1950,
        -0.1164397101110765346,
        1e-13,
        "eraFk54z",
        "d1950",
        status,
    );
    vvd(
        dr1950,
        -0.1175712648471090704e-7,
        1e-20,
        "eraFk54z",
        "dr1950",
        status,
    );
    vvd(
        dd1950,
        0.2108109051316431056e-7,
        1e-20,
        "eraFk54z",
        "dd1950",
        status,
    );
}

fn t_fk5hz(status: &mut i32) {
    let r5 = 1.76779433;
    let d5 = -0.2917517103;

    let mut rh = 0.0;
    let mut dh = 0.0;

    unsafe {
        H1::eraFk5hz(r5, d5, 2400000.5, 54479.0, &mut rh, &mut dh);
    }

    vvd(rh, 1.767794191464423978, 1e-12, "eraFk5hz", "ra", status);
    vvd(dh, -0.2917516001679884419, 1e-12, "eraFk5hz", "dec", status);
}

fn t_fw2m(status: &mut i32) {
    let gamb = -0.2243387670997992368e-5;
    let phib = 0.4091014602391312982;
    let psi = -0.9501954178013015092e-3;
    let eps = 0.4091014316587367472;
    let mut r = [[0.0; 3]; 3];

    unsafe {
        H1::eraFw2m(
            gamb,
            phib,
            psi,
            eps,
            &mut r as *mut [[f64; 3]; 3] as *mut f64,
        );
    }

    vvd(
        r[0][0],
        0.9999995505176007047,
        1e-12,
        "eraFw2m",
        "11",
        status,
    );
    vvd(
        r[0][1],
        0.8695404617348192957e-3,
        1e-12,
        "eraFw2m",
        "12",
        status,
    );
    vvd(
        r[0][2],
        0.3779735201865582571e-3,
        1e-12,
        "eraFw2m",
        "13",
        status,
    );
    vvd(
        r[1][0],
        -0.8695404723772016038e-3,
        1e-12,
        "eraFw2m",
        "21",
        status,
    );
    vvd(
        r[1][1],
        0.9999996219496027161,
        1e-12,
        "eraFw2m",
        "22",
        status,
    );
    vvd(
        r[1][2],
        -0.1361752496887100026e-6,
        1e-12,
        "eraFw2m",
        "23",
        status,
    );
    vvd(
        r[2][0],
        -0.3779734957034082790e-3,
        1e-12,
        "eraFw2m",
        "31",
        status,
    );
    vvd(
        r[2][1],
        -0.1924880848087615651e-6,
        1e-12,
        "eraFw2m",
        "32",
        status,
    );
    vvd(
        r[2][2],
        0.9999999285679971958,
        1e-12,
        "eraFw2m",
        "33",
        status,
    );
}

fn t_fw2xy(status: &mut i32) {
    let gamb = -0.2243387670997992368e-5;
    let phib = 0.4091014602391312982;
    let psi = -0.9501954178013015092e-3;
    let eps = 0.4091014316587367472;
    let mut x = 0.0;
    let mut y = 0.0;

    unsafe {
        H1::eraFw2xy(gamb, phib, psi, eps, &mut x, &mut y);
    }

    vvd(x, -0.3779734957034082790e-3, 1e-14, "eraFw2xy", "x", status);
    vvd(y, -0.1924880848087615651e-6, 1e-14, "eraFw2xy", "y", status);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        unsafe {
            VERBOSE = true;
        }
    }

    let mut status = 0;

    t_fk425(&mut status);
    t_fk45z(&mut status);
    t_fk524(&mut status);
    t_fk52h(&mut status);
    t_fk54z(&mut status);
    t_fk5hz(&mut status);
    t_fw2m(&mut status);
    t_fw2xy(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
