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

fn t_eceq06(status: &mut i32) {
    let date1 = 2456165.5;
    let date2 = 0.401182685;
    let dl = 5.1;
    let db = -0.9;
    let mut dr = 0.0;
    let mut dd = 0.0;

    unsafe {
        H1::eraEceq06(date1, date2, dl, db, &mut dr, &mut dd);
    }

    vvd(dr, 5.533459733613627767, 1e-14, "eraEceq06", "dr", status);
    vvd(dd, -1.246542932554480576, 1e-14, "eraEceq06", "dd", status);
}

fn t_ecm06(status: &mut i32) {
    let date1 = 2456165.5;
    let date2 = 0.401182685;
    let mut rm = [[0.0; 3]; 3];

    unsafe {
        H1::eraEcm06(date1, date2, &mut rm as *mut [[f64; 3]; 3] as *mut f64);
    }

    vvd(
        rm[0][0],
        0.9999952427708701137,
        1e-14,
        "eraEcm06",
        "rm11",
        status,
    );
    vvd(
        rm[0][1],
        -0.2829062057663042347e-2,
        1e-14,
        "eraEcm06",
        "rm12",
        status,
    );
    vvd(
        rm[0][2],
        -0.1229163741100017629e-2,
        1e-14,
        "eraEcm06",
        "rm13",
        status,
    );
    vvd(
        rm[1][0],
        0.3084546876908653562e-2,
        1e-14,
        "eraEcm06",
        "rm21",
        status,
    );
    vvd(
        rm[1][1],
        0.9174891871550392514,
        1e-14,
        "eraEcm06",
        "rm22",
        status,
    );
    vvd(
        rm[1][2],
        0.3977487611849338124,
        1e-14,
        "eraEcm06",
        "rm23",
        status,
    );
    vvd(
        rm[2][0],
        0.2488512951527405928e-5,
        1e-14,
        "eraEcm06",
        "rm31",
        status,
    );
    vvd(
        rm[2][1],
        -0.3977506604161195467,
        1e-14,
        "eraEcm06",
        "rm32",
        status,
    );
    vvd(
        rm[2][2],
        0.9174935488232863071,
        1e-14,
        "eraEcm06",
        "rm33",
        status,
    );
}

fn t_ee00(status: &mut i32) {
    let epsa = 0.4090789763356509900;
    let dpsi = -0.9630909107115582393e-5;

    let ee = unsafe { H1::eraEe00(2400000.5, 53736.0, epsa, dpsi) };

    vvd(ee, -0.8834193235367965479e-5, 1e-18, "eraEe00", "", status);
}

fn t_ee00a(status: &mut i32) {
    let ee = unsafe { H1::eraEe00a(2400000.5, 53736.0) };
    vvd(ee, -0.8834192459222588227e-5, 1e-18, "eraEe00a", "", status);
}

fn t_ee00b(status: &mut i32) {
    let ee = unsafe { H1::eraEe00b(2400000.5, 53736.0) };
    vvd(ee, -0.8835700060003032831e-5, 1e-18, "eraEe00b", "", status);
}

fn t_ee06a(status: &mut i32) {
    let ee = unsafe { H1::eraEe06a(2400000.5, 53736.0) };
    vvd(ee, -0.8834195072043790156e-5, 1e-15, "eraEe06a", "", status);
}

fn t_eect00(status: &mut i32) {
    let eect = unsafe { H1::eraEect00(2400000.5, 53736.0) };
    vvd(
        eect,
        0.2046085004885125264e-8,
        1e-20,
        "eraEect00",
        "",
        status,
    );
}

fn t_eform(status: &mut i32) {
    let mut j: i32;
    let mut a = 0.0;
    let mut f = 0.0;

    j = unsafe { H1::eraEform(0, &mut a, &mut f) };
    viv(j, -1, "eraEform", "j0", status);

    j = unsafe { H1::eraEform(H1::ERFA_WGS84, &mut a, &mut f) };
    viv(j, 0, "eraEform", "j1", status);
    vvd(a, 6378137.0, 1e-10, "eraEform", "a1", status);
    vvd(f, 0.3352810664747480720e-2, 1e-18, "eraEform", "f1", status);

    j = unsafe { H1::eraEform(H1::ERFA_GRS80, &mut a, &mut f) };
    viv(j, 0, "eraEform", "j2", status);
    vvd(a, 6378137.0, 1e-10, "eraEform", "a2", status);
    vvd(f, 0.3352810681182318935e-2, 1e-18, "eraEform", "f2", status);

    j = unsafe { H1::eraEform(H1::ERFA_WGS72, &mut a, &mut f) };
    viv(j, 0, "eraEform", "j2", status);
    vvd(a, 6378135.0, 1e-10, "eraEform", "a3", status);
    vvd(f, 0.3352779454167504862e-2, 1e-18, "eraEform", "f3", status);

    j = unsafe { H1::eraEform(4, &mut a, &mut f) };
    viv(j, -1, "eraEform", "j3", status);
}

fn t_eo06a(status: &mut i32) {
    let eo = unsafe { H1::eraEo06a(2400000.5, 53736.0) };
    vvd(eo, -0.1332882371941833644e-2, 1e-15, "eraEo06a", "", status);
}

fn t_eors(status: &mut i32) {
    let mut rnpb = [[0.0; 3]; 3];
    let s: f64;

    rnpb[0][0] = 0.9999989440476103608;
    rnpb[0][1] = -0.1332881761240011518e-2;
    rnpb[0][2] = -0.5790767434730085097e-3;
    rnpb[1][0] = 0.1332858254308954453e-2;
    rnpb[1][1] = 0.9999991109044505944;
    rnpb[1][2] = -0.4097782710401555759e-4;
    rnpb[2][0] = 0.5791308472168153320e-3;
    rnpb[2][1] = 0.4020595661593994396e-4;
    rnpb[2][2] = 0.9999998314954572365;

    s = -0.1220040848472271978e-7;

    let eo = unsafe { H1::eraEors(&mut rnpb as *mut [[f64; 3]; 3] as *mut f64, s) };
    vvd(eo, -0.1332882715130744606e-2, 1e-14, "eraEors", "", status);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        unsafe {
            VERBOSE = true;
        }
    }

    let mut status = 0;

    t_eceq06(&mut status);
    t_ecm06(&mut status);
    t_ee00(&mut status);
    t_ee00a(&mut status);
    t_ee00b(&mut status);
    t_ee06a(&mut status);
    t_eect00(&mut status);
    t_eform(&mut status);
    t_eo06a(&mut status);
    t_eors(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
