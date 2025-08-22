#![allow(dead_code)]
#![allow(non_snake_case)]

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

fn t_g2icrs(status: &mut i32) {
    let mut dr = 0.0;
    let mut dd = 0.0;

    let dl = 5.5850536063818546461558105;
    let db = -0.7853981633974483096156608;
    unsafe { H1::eraG2icrs(dl, db, &mut dr, &mut dd) };
    vvd(
        dr,
        5.9338074302227188048671,
        1e-14,
        "eraG2icrs",
        "R",
        status,
    );
    vvd(
        dd,
        -1.1784870613579944551541,
        1e-14,
        "eraG2icrs",
        "D",
        status,
    );
}

fn t_gc2gd(status: &mut i32) {
    let mut xyz = [2e6, 3e6, 5.244e6];
    let mut e = 0.0;
    let mut p = 0.0;
    let mut h = 0.0;
    let mut j: i32;

    j = unsafe { H1::eraGc2gd(0, xyz.as_mut_ptr(), &mut e, &mut p, &mut h) };
    viv(j, -1, "eraGc2gd", "j0", status);

    j = unsafe { H1::eraGc2gd(H1::ERFA_WGS84, xyz.as_mut_ptr(), &mut e, &mut p, &mut h) };
    viv(j, 0, "eraGc2gd", "j1", status);
    vvd(e, 0.9827937232473290680, 1e-14, "eraGc2gd", "e1", status);
    vvd(p, 0.97160184819075459, 1e-14, "eraGc2gd", "p1", status);
    vvd(h, 331.4172461426059892, 1e-8, "eraGc2gd", "h1", status);

    j = unsafe { H1::eraGc2gd(H1::ERFA_GRS80, xyz.as_mut_ptr(), &mut e, &mut p, &mut h) };
    viv(j, 0, "eraGc2gd", "j2", status);
    vvd(e, 0.9827937232473290680, 1e-14, "eraGc2gd", "e2", status);
    vvd(p, 0.97160184820607853, 1e-14, "eraGc2gd", "p2", status);
    vvd(h, 331.41731754844348, 1e-8, "eraGc2gd", "h2", status);

    j = unsafe { H1::eraGc2gd(H1::ERFA_WGS72, xyz.as_mut_ptr(), &mut e, &mut p, &mut h) };
    viv(j, 0, "eraGc2gd", "j3", status);
    vvd(e, 0.9827937232473290680, 1e-14, "eraGc2gd", "e3", status);
    vvd(p, 0.9716018181101511937, 1e-14, "eraGc2gd", "p3", status);
    vvd(h, 333.2770726130318123, 1e-8, "eraGc2gd", "h3", status);

    j = unsafe { H1::eraGc2gd(4, xyz.as_mut_ptr(), &mut e, &mut p, &mut h) };
    viv(j, -1, "eraGc2gd", "j4", status);
}

fn t_gc2gde(status: &mut i32) {
    let a = 6378136.0;
    let f = 0.0033528;
    let mut xyz = [2e6, 3e6, 5.244e6];
    let mut e = 0.0;
    let mut p = 0.0;
    let mut h = 0.0;

    let j = unsafe { H1::eraGc2gde(a, f, xyz.as_mut_ptr(), &mut e, &mut p, &mut h) };
    viv(j, 0, "eraGc2gde", "j", status);
    vvd(e, 0.9827937232473290680, 1e-14, "eraGc2gde", "e", status);
    vvd(p, 0.9716018377570411532, 1e-14, "eraGc2gde", "p", status);
    vvd(h, 332.36862495764397, 1e-8, "eraGc2gde", "h", status);
}

fn t_gd2gc(status: &mut i32) {
    let e = 3.1;
    let p = -0.5;
    let h = 2500.0;
    let mut xyz = [0.0; 3];
    let mut j: i32;

    j = unsafe { H1::eraGd2gc(0, e, p, h, xyz.as_mut_ptr()) };
    viv(j, -1, "eraGd2gc", "j0", status);

    j = unsafe { H1::eraGd2gc(H1::ERFA_WGS84, e, p, h, xyz.as_mut_ptr()) };
    viv(j, 0, "eraGd2gc", "j1", status);
    vvd(xyz[0], -5599000.5577049947, 1e-7, "eraGd2gc", "1/1", status);
    vvd(xyz[1], 233011.67223479203, 1e-7, "eraGd2gc", "2/1", status);
    vvd(xyz[2], -3040909.4706983363, 1e-7, "eraGd2gc", "3/1", status);

    j = unsafe { H1::eraGd2gc(H1::ERFA_GRS80, e, p, h, xyz.as_mut_ptr()) };
    viv(j, 0, "eraGd2gc", "j2", status);
    vvd(xyz[0], -5599000.5577260984, 1e-7, "eraGd2gc", "1/2", status);
    vvd(
        xyz[1],
        233011.6722356702949,
        1e-7,
        "eraGd2gc",
        "2/2",
        status,
    );
    vvd(xyz[2], -3040909.4706095476, 1e-7, "eraGd2gc", "3/2", status);

    j = unsafe { H1::eraGd2gc(H1::ERFA_WGS72, e, p, h, xyz.as_mut_ptr()) };
    viv(j, 0, "eraGd2gc", "j3", status);
    vvd(xyz[0], -5598998.7626301490, 1e-7, "eraGd2gc", "1/3", status);
    vvd(
        xyz[1],
        233011.5975297822211,
        1e-7,
        "eraGd2gc",
        "2/3",
        status,
    );
    vvd(xyz[2], -3040908.6861467111, 1e-7, "eraGd2gc", "3/3", status);

    j = unsafe { H1::eraGd2gc(4, e, p, h, xyz.as_mut_ptr()) };
    viv(j, -1, "eraGd2gc", "j4", status);
}

fn t_gd2gce(status: &mut i32) {
    let a = 6378136.0;
    let f = 0.0033528;
    let e = 3.1;
    let p = -0.5;
    let h = 2500.0;
    let mut xyz = [0.0; 3];

    let j = unsafe { H1::eraGd2gce(a, f, e, p, h, xyz.as_mut_ptr()) };
    viv(j, 0, "eraGd2gce", "j", status);
    vvd(xyz[0], -5598999.6665116328, 1e-7, "eraGd2gce", "1", status);
    vvd(xyz[1], 233011.6351463057189, 1e-7, "eraGd2gce", "2", status);
    vvd(xyz[2], -3040909.0517314132, 1e-7, "eraGd2gce", "3", status);
}

fn t_gmst00(status: &mut i32) {
    let theta = unsafe { H1::eraGmst00(2400000.5, 53736.0, 2400000.5, 53736.0) };
    vvd(theta, 1.754174972210740592, 1e-12, "eraGmst00", "", status);
}

fn t_gmst06(status: &mut i32) {
    let theta = unsafe { H1::eraGmst06(2400000.5, 53736.0, 2400000.5, 53736.0) };
    vvd(theta, 1.754174971870091203, 1e-12, "eraGmst06", "", status);
}

fn t_gmst82(status: &mut i32) {
    let theta = unsafe { H1::eraGmst82(2400000.5, 53736.0) };
    vvd(theta, 1.754174981860675096, 1e-12, "eraGmst82", "", status);
}

fn t_gst00a(status: &mut i32) {
    let theta = unsafe { H1::eraGst00a(2400000.5, 53736.0, 2400000.5, 53736.0) };
    vvd(theta, 1.754166138018281369, 1e-12, "eraGst00a", "", status);
}

fn t_gst00b(status: &mut i32) {
    let theta = unsafe { H1::eraGst00b(2400000.5, 53736.0) };
    vvd(theta, 1.754166136510680589, 1e-12, "eraGst00b", "", status);
}

fn t_gst06(status: &mut i32) {
    let mut rnpb = [
        [
            0.9999989440476103608,
            -0.1332881761240011518e-2,
            -0.5790767434730085097e-3,
        ],
        [
            0.1332858254308954453e-2,
            0.9999991109044505944,
            -0.4097782710401555759e-4,
        ],
        [
            0.5791308472168153320e-3,
            0.4020595661593994396e-4,
            0.9999998314954572365,
        ],
    ];
    let theta = unsafe {
        H1::eraGst06(
            2400000.5,
            53736.0,
            2400000.5,
            53736.0,
            &mut rnpb as *mut [[f64; 3]; 3] as *mut f64,
        )
    };
    vvd(theta, 1.754166138018167568, 1e-12, "eraGst06", "", status);
}

fn t_gst06a(status: &mut i32) {
    let theta = unsafe { H1::eraGst06a(2400000.5, 53736.0, 2400000.5, 53736.0) };
    vvd(theta, 1.754166137675019159, 1e-12, "eraGst06a", "", status);
}

fn t_gst94(status: &mut i32) {
    let theta = unsafe { H1::eraGst94(2400000.5, 53736.0) };
    vvd(theta, 1.754166136020645203, 1e-12, "eraGst94", "", status);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        unsafe {
            VERBOSE = true;
        }
    }

    let mut status = 0;

    t_g2icrs(&mut status);
    t_gc2gd(&mut status);
    t_gc2gde(&mut status);
    t_gd2gc(&mut status);
    t_gd2gce(&mut status);
    t_gmst00(&mut status);
    t_gmst06(&mut status);
    t_gmst82(&mut status);
    t_gst00a(&mut status);
    t_gst00b(&mut status);
    t_gst06(&mut status);
    t_gst06a(&mut status);
    t_gst94(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
