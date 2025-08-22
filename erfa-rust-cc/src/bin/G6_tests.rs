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

fn t_bi00(status: &mut i32) {
    let mut dpsibi = 0.0;
    let mut depsbi = 0.0;
    let mut dra = 0.0;

    unsafe { H1::eraBi00(&mut dpsibi, &mut depsbi, &mut dra) };

    vvd(
        dpsibi,
        -0.2025309152835086613e-6,
        1e-12,
        "eraBi00",
        "dpsibi",
        status,
    );
    vvd(
        depsbi,
        -0.3306041454222147847e-7,
        1e-12,
        "eraBi00",
        "depsbi",
        status,
    );
    vvd(
        dra,
        -0.7078279744199225506e-7,
        1e-12,
        "eraBi00",
        "dra",
        status,
    );
}

fn t_bp00(status: &mut i32) {
    let mut rb = [[0.0; 3]; 3];
    let mut rp = [[0.0; 3]; 3];
    let mut rbp = [[0.0; 3]; 3];

    unsafe {
        H1::eraBp00(
            2400000.5,
            50123.9999,
            &mut rb as *mut [[f64; 3]; 3] as *mut f64,
            &mut rp as *mut [[f64; 3]; 3] as *mut f64,
            &mut rbp as *mut [[f64; 3]; 3] as *mut f64,
        )
    };

    vvd(
        rb[0][0],
        0.9999999999999942498,
        1e-12,
        "eraBp00",
        "rb11",
        status,
    );
    vvd(
        rb[0][1],
        -0.7078279744199196626e-7,
        1e-16,
        "eraBp00",
        "rb12",
        status,
    );
    vvd(
        rb[0][2],
        0.8056217146976134152e-7,
        1e-16,
        "eraBp00",
        "rb13",
        status,
    );
    vvd(
        rb[1][0],
        0.7078279477857337206e-7,
        1e-16,
        "eraBp00",
        "rb21",
        status,
    );
    vvd(
        rb[1][1],
        0.9999999999999969484,
        1e-12,
        "eraBp00",
        "rb22",
        status,
    );
    vvd(
        rb[1][2],
        0.3306041454222136517e-7,
        1e-16,
        "eraBp00",
        "rb23",
        status,
    );
    vvd(
        rb[2][0],
        -0.8056217380986972157e-7,
        1e-16,
        "eraBp00",
        "rb31",
        status,
    );
    vvd(
        rb[2][1],
        -0.3306040883980552500e-7,
        1e-16,
        "eraBp00",
        "rb32",
        status,
    );
    vvd(
        rb[2][2],
        0.9999999999999962084,
        1e-12,
        "eraBp00",
        "rb33",
        status,
    );

    vvd(
        rp[0][0],
        0.9999995504864048241,
        1e-12,
        "eraBp00",
        "rp11",
        status,
    );
    vvd(
        rp[0][1],
        0.8696113836207084411e-3,
        1e-14,
        "eraBp00",
        "rp12",
        status,
    );
    vvd(
        rp[0][2],
        0.3778928813389333402e-3,
        1e-14,
        "eraBp00",
        "rp13",
        status,
    );
    vvd(
        rp[1][0],
        -0.8696113818227265968e-3,
        1e-14,
        "eraBp00",
        "rp21",
        status,
    );
    vvd(
        rp[1][1],
        0.9999996218879365258,
        1e-12,
        "eraBp00",
        "rp22",
        status,
    );
    vvd(
        rp[1][2],
        -0.1690679263009242066e-6,
        1e-14,
        "eraBp00",
        "rp23",
        status,
    );
    vvd(
        rp[2][0],
        -0.3778928854764695214e-3,
        1e-14,
        "eraBp00",
        "rp31",
        status,
    );
    vvd(
        rp[2][1],
        -0.1595521004195286491e-6,
        1e-14,
        "eraBp00",
        "rp32",
        status,
    );
    vvd(
        rp[2][2],
        0.9999999285984682756,
        1e-12,
        "eraBp00",
        "rp33",
        status,
    );

    vvd(
        rbp[0][0],
        0.9999995505175087260,
        1e-12,
        "eraBp00",
        "rbp11",
        status,
    );
    vvd(
        rbp[0][1],
        0.8695405883617884705e-3,
        1e-14,
        "eraBp00",
        "rbp12",
        status,
    );
    vvd(
        rbp[0][2],
        0.3779734722239007105e-3,
        1e-14,
        "eraBp00",
        "rbp13",
        status,
    );
    vvd(
        rbp[1][0],
        -0.8695405990410863719e-3,
        1e-14,
        "eraBp00",
        "rbp21",
        status,
    );
    vvd(
        rbp[1][1],
        0.9999996219494925900,
        1e-12,
        "eraBp00",
        "rbp22",
        status,
    );
    vvd(
        rbp[1][2],
        -0.1360775820404982209e-6,
        1e-14,
        "eraBp00",
        "rbp23",
        status,
    );
    vvd(
        rbp[2][0],
        -0.3779734476558184991e-3,
        1e-14,
        "eraBp00",
        "rbp31",
        status,
    );
    vvd(
        rbp[2][1],
        -0.1925857585832024058e-6,
        1e-14,
        "eraBp00",
        "rbp32",
        status,
    );
    vvd(
        rbp[2][2],
        0.9999999285680153377,
        1e-12,
        "eraBp00",
        "rbp33",
        status,
    );
}

fn t_bp06(status: &mut i32) {
    let mut rb = [[0.0; 3]; 3];
    let mut rp = [[0.0; 3]; 3];
    let mut rbp = [[0.0; 3]; 3];

    unsafe {
        H1::eraBp06(
            2400000.5,
            50123.9999,
            &mut rb as *mut [[f64; 3]; 3] as *mut f64,
            &mut rp as *mut [[f64; 3]; 3] as *mut f64,
            &mut rbp as *mut [[f64; 3]; 3] as *mut f64,
        )
    };

    vvd(
        rb[0][0],
        0.9999999999999942497,
        1e-12,
        "eraBp06",
        "rb11",
        status,
    );
    vvd(
        rb[0][1],
        -0.7078368960971557145e-7,
        1e-14,
        "eraBp06",
        "rb12",
        status,
    );
    vvd(
        rb[0][2],
        0.8056213977613185606e-7,
        1e-14,
        "eraBp06",
        "rb13",
        status,
    );
    vvd(
        rb[1][0],
        0.7078368694637674333e-7,
        1e-14,
        "eraBp06",
        "rb21",
        status,
    );
    vvd(
        rb[1][1],
        0.9999999999999969484,
        1e-12,
        "eraBp06",
        "rb22",
        status,
    );
    vvd(
        rb[1][2],
        0.3305943742989134124e-7,
        1e-14,
        "eraBp06",
        "rb23",
        status,
    );
    vvd(
        rb[2][0],
        -0.8056214211620056792e-7,
        1e-14,
        "eraBp06",
        "rb31",
        status,
    );
    vvd(
        rb[2][1],
        -0.3305943172740586950e-7,
        1e-14,
        "eraBp06",
        "rb32",
        status,
    );
    vvd(
        rb[2][2],
        0.9999999999999962084,
        1e-12,
        "eraBp06",
        "rb33",
        status,
    );

    vvd(
        rp[0][0],
        0.9999995504864960278,
        1e-12,
        "eraBp06",
        "rp11",
        status,
    );
    vvd(
        rp[0][1],
        0.8696112578855404832e-3,
        1e-14,
        "eraBp06",
        "rp12",
        status,
    );
    vvd(
        rp[0][2],
        0.3778929293341390127e-3,
        1e-14,
        "eraBp06",
        "rp13",
        status,
    );
    vvd(
        rp[1][0],
        -0.8696112560510186244e-3,
        1e-14,
        "eraBp06",
        "rp21",
        status,
    );
    vvd(
        rp[1][1],
        0.9999996218880458820,
        1e-12,
        "eraBp06",
        "rp22",
        status,
    );
    vvd(
        rp[1][2],
        -0.1691646168941896285e-6,
        1e-14,
        "eraBp06",
        "rp23",
        status,
    );
    vvd(
        rp[2][0],
        -0.3778929335557603418e-3,
        1e-14,
        "eraBp06",
        "rp31",
        status,
    );
    vvd(
        rp[2][1],
        -0.1594554040786495076e-6,
        1e-14,
        "eraBp06",
        "rp32",
        status,
    );
    vvd(
        rp[2][2],
        0.9999999285984501222,
        1e-12,
        "eraBp06",
        "rp33",
        status,
    );

    vvd(
        rbp[0][0],
        0.9999995505176007047,
        1e-12,
        "eraBp06",
        "rbp11",
        status,
    );
    vvd(
        rbp[0][1],
        0.8695404617348208406e-3,
        1e-14,
        "eraBp06",
        "rbp12",
        status,
    );
    vvd(
        rbp[0][2],
        0.3779735201865589104e-3,
        1e-14,
        "eraBp06",
        "rbp13",
        status,
    );
    vvd(
        rbp[1][0],
        -0.8695404723772031414e-3,
        1e-14,
        "eraBp06",
        "rbp21",
        status,
    );
    vvd(
        rbp[1][1],
        0.9999996219496027161,
        1e-12,
        "eraBp06",
        "rbp22",
        status,
    );
    vvd(
        rbp[1][2],
        -0.1361752497080270143e-6,
        1e-14,
        "eraBp06",
        "rbp23",
        status,
    );
    vvd(
        rbp[2][0],
        -0.3779734957034089490e-3,
        1e-14,
        "eraBp06",
        "rbp31",
        status,
    );
    vvd(
        rbp[2][1],
        -0.1924880847894457113e-6,
        1e-14,
        "eraBp06",
        "rbp32",
        status,
    );
    vvd(
        rbp[2][2],
        0.9999999285679971958,
        1e-12,
        "eraBp06",
        "rbp33",
        status,
    );
}

fn t_bpn2xy(status: &mut i32) {
    let mut rbpn = [
        [
            9.999962358680738e-1,
            -2.516417057665452e-3,
            -1.093569785342370e-3,
        ],
        [
            2.516462370370876e-3,
            9.999968329010883e-1,
            4.006159587358310e-5,
        ],
        [
            1.093465510215479e-3,
            -4.281337229063151e-5,
            9.999994012499173e-1,
        ],
    ];
    let mut x = 0.0;
    let mut y = 0.0;

    unsafe { H1::eraBpn2xy(&mut rbpn as *mut [[f64; 3]; 3] as *mut f64, &mut x, &mut y) };

    vvd(x, 1.093465510215479e-3, 1e-12, "eraBpn2xy", "x", status);
    vvd(y, -4.281337229063151e-5, 1e-12, "eraBpn2xy", "y", status);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        unsafe {
            VERBOSE = true;
        }
    }

    let mut status = 0;

    t_bi00(&mut status);
    t_bp00(&mut status);
    t_bp06(&mut status);
    t_bpn2xy(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
