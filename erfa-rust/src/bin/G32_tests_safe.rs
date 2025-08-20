#![allow(dead_code)]

use erfa_rust::G29_safe::*;
use erfa_rust::G32_safe::*;
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

fn t_taitt(status: &mut i32) {
    let ((t1, t2), j) = eraTaitt_safe(2453750.5, 0.892482639).unwrap();
    vvd(t1, 2453750.5, 1e-6, "eraTaitt", "t1", status);
    vvd(t2, 0.892855139, 1e-12, "eraTaitt", "t2", status);
    viv(j, 0, "eraTaitt", "j", status);
}

fn t_taiut1(status: &mut i32) {
    let ((u1, u2), j) = eraTaiut1_safe(2453750.5, 0.892482639, -32.6659).unwrap();
    vvd(u1, 2453750.5, 1e-6, "eraTaiut1", "u1", status);
    vvd(u2, 0.8921045614537037037, 1e-12, "eraTaiut1", "u2", status);
    viv(j, 0, "eraTaiut1", "j", status);
}

fn t_taiutc(status: &mut i32) {
    let ((u1, u2), j) = eraTaiutc_safe(2453750.5, 0.892482639).unwrap();
    vvd(u1, 2453750.5, 1e-6, "eraTaiutc", "u1", status);
    vvd(u2, 0.8921006945555555556, 1e-12, "eraTaiutc", "u2", status);
    viv(j, 0, "eraTaiutc", "j", status);
}

fn t_tcbtdb(status: &mut i32) {
    let ((b1, b2), j) = eraTcbtdb_safe(2453750.5, 0.893019599).unwrap();
    vvd(b1, 2453750.5, 1e-6, "eraTcbtdb", "b1", status);
    vvd(b2, 0.8928551362746343397, 1e-12, "eraTcbtdb", "b2", status);
    viv(j, 0, "eraTcbtdb", "j", status);
}

fn t_tcgtt(status: &mut i32) {
    let ((t1, t2), j) = eraTcgtt_safe(2453750.5, 0.892862531).unwrap();
    vvd(t1, 2453750.5, 1e-6, "eraTcgtt", "t1", status);
    vvd(t2, 0.8928551387488816828, 1e-12, "eraTcgtt", "t2", status);
    viv(j, 0, "eraTcgtt", "j", status);
}

fn t_tdbtcb(status: &mut i32) {
    let ((b1, b2), j) = eraTdbtcb_safe(2453750.5, 0.892855137).unwrap();
    vvd(b1, 2453750.5, 1e-6, "eraTdbtcb", "b1", status);
    vvd(b2, 0.8930195997253656716, 1e-12, "eraTdbtcb", "b2", status);
    viv(j, 0, "eraTdbtcb", "j", status);
}

fn t_tdbtt(status: &mut i32) {
    let ((t1, t2), j) = eraTdbtt_safe(2453750.5, 0.892855137, -0.000201).unwrap();
    vvd(t1, 2453750.5, 1e-6, "eraTdbtt", "t1", status);
    vvd(t2, 0.8928551393263888889, 1e-12, "eraTdbtt", "t2", status);
    viv(j, 0, "eraTdbtt", "j", status);
}

fn t_tf2a(status: &mut i32) {
    let (a, j) = eraTf2a_safe('+', 4, 58, 20.2).unwrap();
    vvd(a, 1.301739278189537429, 1e-12, "eraTf2a", "a", status);
    viv(j, 0, "eraTf2a", "j", status);
}

fn t_tf2d(status: &mut i32) {
    let (d, j) = eraTf2d_safe(' ', 23, 55, 10.9).unwrap();
    vvd(d, 0.9966539351851851852, 1e-12, "eraTf2d", "d", status);
    viv(j, 0, "eraTf2d", "j", status);
}

fn t_tpors(status: &mut i32) {
    let ((az1, bz1), (az2, bz2), n) = eraTpors_safe(-0.03, 0.07, 1.3, 1.5).unwrap();
    vvd(az1, 1.736621577783208748, 1e-13, "eraTpors", "az1", status);
    vvd(bz1, 1.436736561844090323, 1e-13, "eraTpors", "bz1", status);
    vvd(az2, 4.004971075806584490, 1e-13, "eraTpors", "az2", status);
    vvd(bz2, 1.565084088476417917, 1e-13, "eraTpors", "bz2", status);
    viv(n, 2, "eraTpors", "n", status);
}

fn t_tporv(status: &mut i32) {
    let v = eraS2c_safe(1.3, 1.5).unwrap();
    let (vz1, vz2, n) = eraTporv_safe(-0.03, 0.07, &v).unwrap();
    vvd(
        vz1[0],
        -0.02206252822366888610,
        1e-15,
        "eraTporv",
        "x1",
        status,
    );
    vvd(
        vz1[1],
        0.1318251060359645016,
        1e-14,
        "eraTporv",
        "y1",
        status,
    );
    vvd(
        vz1[2],
        0.9910274397144543895,
        1e-14,
        "eraTporv",
        "z1",
        status,
    );
    vvd(
        vz2[0],
        -0.003712211763801968173,
        1e-16,
        "eraTporv",
        "x2",
        status,
    );
    vvd(
        vz2[1],
        -0.004341519956299836813,
        1e-16,
        "eraTporv",
        "y2",
        status,
    );
    vvd(
        vz2[2],
        0.9999836852110587012,
        1e-14,
        "eraTporv",
        "z2",
        status,
    );
    viv(n, 2, "eraTporv", "n", status);
}

fn t_tpsts(status: &mut i32) {
    let (ra, dec) = eraTpsts_safe(-0.03, 0.07, 2.3, 1.5).unwrap();
    vvd(ra, 0.7596127167359629775, 1e-14, "eraTpsts", "ra", status);
    vvd(dec, 1.540864645109263028, 1e-13, "eraTpsts", "dec", status);
}

fn t_tpstv(status: &mut i32) {
    let vz = eraS2c_safe(2.3, 1.5).unwrap();
    let v = eraTpstv_safe(-0.03, 0.07, &vz).unwrap();
    vvd(v[0], 0.02170030454907376677, 1e-15, "eraTpstv", "x", status);
    vvd(v[1], 0.02060909590535367447, 1e-15, "eraTpstv", "y", status);
    vvd(v[2], 0.9995520806583523804, 1e-14, "eraTpstv", "z", status);
}

fn t_tpxes(status: &mut i32) {
    let ((xi, eta), j) = eraTpxes_safe(1.3, 1.55, 2.3, 1.5).unwrap();
    vvd(xi, -0.01753200983236980595, 1e-15, "eraTpxes", "xi", status);
    vvd(
        eta,
        0.05962940005778712891,
        1e-15,
        "eraTpxes",
        "eta",
        status,
    );
    viv(j, 0, "eraTpxes", "j", status);
}

fn t_tpxev(status: &mut i32) {
    let v = eraS2c_safe(1.3, 1.55).unwrap();
    let vz = eraS2c_safe(2.3, 1.5).unwrap();
    let ((xi, eta), j) = eraTpxev_safe(&v, &vz).unwrap();
    vvd(xi, -0.01753200983236980595, 1e-15, "eraTpxev", "xi", status);
    vvd(
        eta,
        0.05962940005778712891,
        1e-15,
        "eraTpxev",
        "eta",
        status,
    );
    viv(j, 0, "eraTpxev", "j", status);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        unsafe {
            VERBOSE = true;
        }
    }

    let mut status = 0;

    t_taitt(&mut status);
    t_taiut1(&mut status);
    t_taiutc(&mut status);
    t_tcbtdb(&mut status);
    t_tcgtt(&mut status);
    t_tdbtcb(&mut status);
    t_tdbtt(&mut status);
    t_tf2a(&mut status);
    t_tf2d(&mut status);
    t_tpors(&mut status);
    t_tporv(&mut status);
    t_tpsts(&mut status);
    t_tpstv(&mut status);
    t_tpxes(&mut status);
    t_tpxev(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
