#![allow(dead_code)]

use erfa_rust_cc::H1;
use libc::{c_char, c_int, snprintf};
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

fn t_prec76(status: &mut i32) {
    let ep01: f64 = 2400000.5;
    let ep02: f64 = 33282.0;
    let ep11: f64 = 2400000.5;
    let ep12: f64 = 51544.0;
    let mut zeta: f64 = 0.0;
    let mut z: f64 = 0.0;
    let mut theta: f64 = 0.0;

    unsafe {
        H1::eraPrec76(ep01, ep02, ep11, ep12, &mut zeta, &mut z, &mut theta);
    }

    vvd(
        zeta,
        0.5588961642000161243e-2,
        1e-12,
        "eraPrec76",
        "zeta",
        status,
    );
    vvd(z, 0.5589922365870680624e-2, 1e-12, "eraPrec76", "z", status);
    vvd(
        theta,
        0.4858945471687296760e-2,
        1e-12,
        "eraPrec76",
        "theta",
        status,
    );
}

fn t_pv2p(status: &mut i32) {
    let mut pv = [[0.0; 3]; 2];
    let mut p = [0.0; 3];

    pv[0][0] = 0.3;
    pv[0][1] = 1.2;
    pv[0][2] = -2.5;
    pv[1][0] = -0.5;
    pv[1][1] = 3.1;
    pv[1][2] = 0.9;

    unsafe {
        H1::eraPv2p(&mut pv as *mut _ as *mut f64, &mut p as *mut _ as *mut f64);
    }

    vvd(p[0], 0.3, 0.0, "eraPv2p", "1", status);
    vvd(p[1], 1.2, 0.0, "eraPv2p", "2", status);
    vvd(p[2], -2.5, 0.0, "eraPv2p", "3", status);
}

fn t_pv2s(status: &mut i32) {
    let mut pv = [[0.0; 3]; 2];
    let mut theta: f64 = 0.0;
    let mut phi: f64 = 0.0;
    let mut r: f64 = 0.0;
    let mut td: f64 = 0.0;
    let mut pd: f64 = 0.0;
    let mut rd: f64 = 0.0;

    pv[0][0] = -0.4514964673880165;
    pv[0][1] = 0.03093394277342585;
    pv[0][2] = 0.05594668105108779;
    pv[1][0] = 1.292270850663260e-5;
    pv[1][1] = 2.652814182060692e-6;
    pv[1][2] = 2.568431853930293e-6;

    unsafe {
        H1::eraPv2s(
            &mut pv as *mut _ as *mut f64,
            &mut theta,
            &mut phi,
            &mut r,
            &mut td,
            &mut pd,
            &mut rd,
        );
    }

    vvd(
        theta,
        3.073185307179586515,
        1e-12,
        "eraPv2s",
        "theta",
        status,
    );
    vvd(phi, 0.1229999999999999992, 1e-12, "eraPv2s", "phi", status);
    vvd(r, 0.4559999999999999757, 1e-12, "eraPv2s", "r", status);
    vvd(
        td,
        -0.7800000000000000364e-5,
        1e-16,
        "eraPv2s",
        "td",
        status,
    );
    vvd(pd, 0.9010000000000001639e-5, 1e-16, "eraPv2s", "pd", status);
    vvd(
        rd,
        -0.1229999999999999832e-4,
        1e-16,
        "eraPv2s",
        "rd",
        status,
    );
}

fn t_pvdpv(status: &mut i32) {
    let mut a = [[0.0; 3]; 2];
    let mut b = [[0.0; 3]; 2];
    let mut adb = [0.0; 2];

    a[0][0] = 2.0;
    a[0][1] = 2.0;
    a[0][2] = 3.0;
    a[1][0] = 6.0;
    a[1][1] = 0.0;
    a[1][2] = 4.0;
    b[0][0] = 1.0;
    b[0][1] = 3.0;
    b[0][2] = 4.0;
    b[1][0] = 0.0;
    b[1][1] = 2.0;
    b[1][2] = 8.0;

    unsafe {
        H1::eraPvdpv(
            &mut a as *mut _ as *mut f64,
            &mut b as *mut _ as *mut f64,
            &mut adb as *mut _ as *mut f64,
        );
    }

    vvd(adb[0], 20.0, 1e-12, "eraPvdpv", "1", status);
    vvd(adb[1], 50.0, 1e-12, "eraPvdpv", "2", status);
}

fn t_pvm(status: &mut i32) {
    let mut pv = [[0.0; 3]; 2];
    let mut r: f64 = 0.0;
    let mut s: f64 = 0.0;

    pv[0][0] = 0.3;
    pv[0][1] = 1.2;
    pv[0][2] = -2.5;
    pv[1][0] = 0.45;
    pv[1][1] = -0.25;
    pv[1][2] = 1.1;

    unsafe {
        H1::eraPvm(&mut pv as *mut _ as *mut f64, &mut r, &mut s);
    }

    vvd(r, 2.789265136196270604, 1e-12, "eraPvm", "r", status);
    vvd(s, 1.214495780149111922, 1e-12, "eraPvm", "s", status);
}

fn t_pvmpv(status: &mut i32) {
    let mut a = [[0.0; 3]; 2];
    let mut b = [[0.0; 3]; 2];
    let mut amb = [[0.0; 3]; 2];

    a[0][0] = 2.0;
    a[0][1] = 2.0;
    a[0][2] = 3.0;
    a[1][0] = 5.0;
    a[1][1] = 6.0;
    a[1][2] = 3.0;
    b[0][0] = 1.0;
    b[0][1] = 3.0;
    b[0][2] = 4.0;
    b[1][0] = 3.0;
    b[1][1] = 2.0;
    b[1][2] = 1.0;

    unsafe {
        H1::eraPvmpv(
            &mut a as *mut _ as *mut f64,
            &mut b as *mut _ as *mut f64,
            &mut amb as *mut _ as *mut f64,
        );
    }

    vvd(amb[0][0], 1.0, 1e-12, "eraPvmpv", "11", status);
    vvd(amb[0][1], -1.0, 1e-12, "eraPvmpv", "21", status);
    vvd(amb[0][2], -1.0, 1e-12, "eraPvmpv", "31", status);
    vvd(amb[1][0], 2.0, 1e-12, "eraPvmpv", "12", status);
    vvd(amb[1][1], 4.0, 1e-12, "eraPvmpv", "22", status);
    vvd(amb[1][2], 2.0, 1e-12, "eraPvmpv", "32", status);
}

fn t_pvppv(status: &mut i32) {
    let mut a = [[0.0; 3]; 2];
    let mut b = [[0.0; 3]; 2];
    let mut apb = [[0.0; 3]; 2];

    a[0][0] = 2.0;
    a[0][1] = 2.0;
    a[0][2] = 3.0;
    a[1][0] = 5.0;
    a[1][1] = 6.0;
    a[1][2] = 3.0;
    b[0][0] = 1.0;
    b[0][1] = 3.0;
    b[0][2] = 4.0;
    b[1][0] = 3.0;
    b[1][1] = 2.0;
    b[1][2] = 1.0;

    unsafe {
        H1::eraPvppv(
            &mut a as *mut _ as *mut f64,
            &mut b as *mut _ as *mut f64,
            &mut apb as *mut _ as *mut f64,
        );
    }

    vvd(apb[0][0], 3.0, 1e-12, "eraPvppv", "p1", status);
    vvd(apb[0][1], 5.0, 1e-12, "eraPvppv", "p2", status);
    vvd(apb[0][2], 7.0, 1e-12, "eraPvppv", "p3", status);
    vvd(apb[1][0], 8.0, 1e-12, "eraPvppv", "v1", status);
    vvd(apb[1][1], 8.0, 1e-12, "eraPvppv", "v2", status);
    vvd(apb[1][2], 4.0, 1e-12, "eraPvppv", "v3", status);
}

fn t_pvstar(status: &mut i32) {
    let mut pv = [[0.0; 3]; 2];
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    let mut pmr: f64 = 0.0;
    let mut pmd: f64 = 0.0;
    let mut px: f64 = 0.0;
    let mut rv: f64 = 0.0;

    pv[0][0] = 126668.5912743160601;
    pv[0][1] = 2136.792716839935195;
    pv[0][2] = -245251.2339876830091;
    pv[1][0] = -0.4051854035740712739e-2;
    pv[1][1] = -0.6253919754866173866e-2;
    pv[1][2] = 0.1189353719774107189e-1;

    let j: c_int = unsafe {
        H1::eraPvstar(
            &mut pv as *mut _ as *mut f64,
            &mut ra,
            &mut dec,
            &mut pmr,
            &mut pmd,
            &mut px,
            &mut rv,
        )
    };

    vvd(ra, 0.1686756e-1, 1e-12, "eraPvstar", "ra", status);
    vvd(dec, -1.093989828, 1e-12, "eraPvstar", "dec", status);
    vvd(
        pmr,
        -0.1783235160000472788e-4,
        1e-16,
        "eraPvstar",
        "pmr",
        status,
    );
    vvd(
        pmd,
        0.2336024047000619347e-5,
        1e-16,
        "eraPvstar",
        "pmd",
        status,
    );
    vvd(px, 0.74723, 1e-12, "eraPvstar", "px", status);
    vvd(rv, -21.60000010107306010, 1e-11, "eraPvstar", "rv", status);
    viv(j, 0, "eraPvstar", "j", status);
}

fn t_pvtob(status: &mut i32) {
    let elong: f64 = 2.0;
    let phi: f64 = 0.5;
    let hm: f64 = 3000.0;
    let xp: f64 = 1e-6;
    let yp: f64 = -0.5e-6;
    let sp: f64 = 1e-8;
    let theta: f64 = 5.0;
    let mut pv = [[0.0; 3]; 2];

    unsafe {
        H1::eraPvtob(
            elong,
            phi,
            hm,
            xp,
            yp,
            sp,
            theta,
            &mut pv as *mut _ as *mut f64,
        );
    }

    vvd(
        pv[0][0],
        4225081.367071159207,
        1e-5,
        "eraPvtob",
        "p(1)",
        status,
    );
    vvd(
        pv[0][1],
        3681943.215856198144,
        1e-5,
        "eraPvtob",
        "p(2)",
        status,
    );
    vvd(
        pv[0][2],
        3041149.399241260785,
        1e-5,
        "eraPvtob",
        "p(3)",
        status,
    );
    vvd(
        pv[1][0],
        -268.4915389365998787,
        1e-9,
        "eraPvtob",
        "v(1)",
        status,
    );
    vvd(
        pv[1][1],
        308.0977983288903123,
        1e-9,
        "eraPvtob",
        "v(2)",
        status,
    );
    vvd(pv[1][2], 0.0, 0.0, "eraPvtob", "v(3)", status);
}

fn t_pvu(status: &mut i32) {
    let mut pv = [[0.0; 3]; 2];
    let mut upv = [[0.0; 3]; 2];

    pv[0][0] = 126668.5912743160734;
    pv[0][1] = 2136.792716839935565;
    pv[0][2] = -245251.2339876830229;
    pv[1][0] = -0.4051854035740713039e-2;
    pv[1][1] = -0.6253919754866175788e-2;
    pv[1][2] = 0.1189353719774107615e-1;

    unsafe {
        H1::eraPvu(
            2920.0,
            &mut pv as *mut _ as *mut f64,
            &mut upv as *mut _ as *mut f64,
        );
    }

    vvd(
        upv[0][0],
        126656.7598605317105,
        1e-6,
        "eraPvu",
        "p1",
        status,
    );
    vvd(
        upv[0][1],
        2118.531271155726332,
        1e-8,
        "eraPvu",
        "p2",
        status,
    );
    vvd(
        upv[0][2],
        -245216.5048590656190,
        1e-6,
        "eraPvu",
        "p3",
        status,
    );
    vvd(
        upv[1][0],
        -0.4051854035740713039e-2,
        1e-12,
        "eraPvu",
        "v1",
        status,
    );
    vvd(
        upv[1][1],
        -0.6253919754866175788e-2,
        1e-12,
        "eraPvu",
        "v2",
        status,
    );
    vvd(
        upv[1][2],
        0.1189353719774107615e-1,
        1e-12,
        "eraPvu",
        "v3",
        status,
    );
}

fn t_pvup(status: &mut i32) {
    let mut pv = [[0.0; 3]; 2];
    let mut p = [0.0; 3];

    pv[0][0] = 126668.5912743160734;
    pv[0][1] = 2136.792716839935565;
    pv[0][2] = -245251.2339876830229;
    pv[1][0] = -0.4051854035740713039e-2;
    pv[1][1] = -0.6253919754866175788e-2;
    pv[1][2] = 0.1189353719774107615e-1;

    unsafe {
        H1::eraPvup(
            2920.0,
            &mut pv as *mut _ as *mut f64,
            &mut p as *mut _ as *mut f64,
        );
    }

    vvd(p[0], 126656.7598605317105, 1e-6, "eraPvup", "1", status);
    vvd(p[1], 2118.531271155726332, 1e-8, "eraPvup", "2", status);
    vvd(p[2], -245216.5048590656190, 1e-6, "eraPvup", "3", status);
}

fn t_pvxpv(status: &mut i32) {
    let mut a = [[0.0; 3]; 2];
    let mut b = [[0.0; 3]; 2];
    let mut axb = [[0.0; 3]; 2];

    a[0][0] = 2.0;
    a[0][1] = 2.0;
    a[0][2] = 3.0;
    a[1][0] = 6.0;
    a[1][1] = 0.0;
    a[1][2] = 4.0;
    b[0][0] = 1.0;
    b[0][1] = 3.0;
    b[0][2] = 4.0;
    b[1][0] = 0.0;
    b[1][1] = 2.0;
    b[1][2] = 8.0;

    unsafe {
        H1::eraPvxpv(
            &mut a as *mut _ as *mut f64,
            &mut b as *mut _ as *mut f64,
            &mut axb as *mut _ as *mut f64,
        );
    }

    vvd(axb[0][0], -1.0, 1e-12, "eraPvxpv", "p1", status);
    vvd(axb[0][1], -5.0, 1e-12, "eraPvxpv", "p2", status);
    vvd(axb[0][2], 4.0, 1e-12, "eraPvxpv", "p3", status);
    vvd(axb[1][0], -2.0, 1e-12, "eraPvxpv", "v1", status);
    vvd(axb[1][1], -36.0, 1e-12, "eraPvxpv", "v2", status);
    vvd(axb[1][2], 22.0, 1e-12, "eraPvxpv", "v3", status);
}

fn t_pxp(status: &mut i32) {
    let mut a = [0.0; 3];
    let mut b = [0.0; 3];
    let mut axb = [0.0; 3];

    a[0] = 2.0;
    a[1] = 2.0;
    a[2] = 3.0;
    b[0] = 1.0;
    b[1] = 3.0;
    b[2] = 4.0;

    unsafe {
        H1::eraPxp(
            &mut a as *mut _ as *mut f64,
            &mut b as *mut _ as *mut f64,
            &mut axb as *mut _ as *mut f64,
        );
    }

    vvd(axb[0], -1.0, 1e-12, "eraPxp", "1", status);
    vvd(axb[1], -5.0, 1e-12, "eraPxp", "2", status);
    vvd(axb[2], 4.0, 1e-12, "eraPxp", "3", status);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        unsafe {
            VERBOSE = true;
        }
    }

    let mut status = 0;

    t_prec76(&mut status);
    t_pv2p(&mut status);
    t_pv2s(&mut status);
    t_pvdpv(&mut status);
    t_pvm(&mut status);
    t_pvmpv(&mut status);
    t_pvppv(&mut status);
    t_pvstar(&mut status);
    t_pvtob(&mut status);
    t_pvu(&mut status);
    t_pvup(&mut status);
    t_pvxpv(&mut status);
    t_pxp(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
