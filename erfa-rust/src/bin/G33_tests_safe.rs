#![allow(dead_code)]

use erfa_rust::G33_safe::*;
use libc::{c_char, snprintf};
use std::ffi::CString;

static mut VERBOSE: bool = false;

fn format_g(val: f64, precision: usize) -> String {
    let mut buffer = vec![0u8; 512];
    let fmt = format!("%.{}g", precision);
    let cfmt = CString::new(fmt).unwrap();
    unsafe {
        snprintf(
            buffer.as_mut_ptr() as *mut c_char,
            buffer.len(),
            cfmt.as_ptr(),
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

fn t_tr(status: &mut i32) {
    let r: [[f64; 3]; 3] = [[2.0, 3.0, 2.0], [3.0, 2.0, 3.0], [3.0, 4.0, 5.0]];
    let rt = eraTr_safe(&r).unwrap();
    vvd(rt[0][0], 2.0, 0.0, "eraTr_safe", "11", status);
    vvd(rt[0][1], 3.0, 0.0, "eraTr_safe", "12", status);
    vvd(rt[0][2], 3.0, 0.0, "eraTr_safe", "13", status);
    vvd(rt[1][0], 3.0, 0.0, "eraTr_safe", "21", status);
    vvd(rt[1][1], 2.0, 0.0, "eraTr_safe", "22", status);
    vvd(rt[1][2], 4.0, 0.0, "eraTr_safe", "23", status);
    vvd(rt[2][0], 2.0, 0.0, "eraTr_safe", "31", status);
    vvd(rt[2][1], 3.0, 0.0, "eraTr_safe", "32", status);
    vvd(rt[2][2], 5.0, 0.0, "eraTr_safe", "33", status);
}

fn t_trxp(status: &mut i32) {
    let r: [[f64; 3]; 3] = [[2.0, 3.0, 2.0], [3.0, 2.0, 3.0], [3.0, 4.0, 5.0]];
    let p = [0.2, 1.5, 0.1];
    let trp = eraTrxp_safe(&r, &p).unwrap();
    vvd(trp[0], 5.2, 1e-12, "eraTrxp_safe", "1", status);
    vvd(trp[1], 4.0, 1e-12, "eraTrxp_safe", "2", status);
    vvd(trp[2], 5.4, 1e-12, "eraTrxp_safe", "3", status);
}

fn t_trxpv(status: &mut i32) {
    let r: [[f64; 3]; 3] = [[2.0, 3.0, 2.0], [3.0, 2.0, 3.0], [3.0, 4.0, 5.0]];
    let pv = [[0.2, 1.5, 0.1], [1.5, 0.2, 0.1]];
    let trpv = eraTrxpv_safe(&r, &pv).unwrap();
    vvd(trpv[0][0], 5.2, 1e-12, "eraTrxpv_safe", "p1", status);
    vvd(trpv[0][1], 4.0, 1e-12, "eraTrxpv_safe", "p2", status);
    vvd(trpv[0][2], 5.4, 1e-12, "eraTrxpv_safe", "p3", status);
    vvd(trpv[1][0], 3.9, 1e-12, "eraTrxpv_safe", "v1", status);
    vvd(trpv[1][1], 5.3, 1e-12, "eraTrxpv_safe", "v2", status);
    vvd(trpv[1][2], 4.1, 1e-12, "eraTrxpv_safe", "v3", status);
}

fn t_tttai(status: &mut i32) {
    let ((a1, a2), j) = eraTttai_safe(2453750.5, 0.892482639).unwrap();
    vvd(a1, 2453750.5, 1e-6, "eraTttai_safe", "a1", status);
    vvd(a2, 0.892110139, 1e-12, "eraTttai_safe", "a2", status);
    viv(j, 0, "eraTttai_safe", "j", status);
}

fn t_tttcg(status: &mut i32) {
    let ((g1, g2), j) = eraTttcg_safe(2453750.5, 0.892482639).unwrap();
    vvd(g1, 2453750.5, 1e-6, "eraTttcg_safe", "g1", status);
    vvd(
        g2,
        0.8924900312508587113,
        1e-12,
        "eraTttcg_safe",
        "g2",
        status,
    );
    viv(j, 0, "eraTttcg_safe", "j", status);
}

fn t_tttdb(status: &mut i32) {
    let ((b1, b2), j) = eraTttdb_safe(2453750.5, 0.892855139, -0.000201).unwrap();
    vvd(b1, 2453750.5, 1e-6, "eraTttdb_safe", "b1", status);
    vvd(
        b2,
        0.8928551366736111111,
        1e-12,
        "eraTttdb_safe",
        "b2",
        status,
    );
    viv(j, 0, "eraTttdb_safe", "j", status);
}

fn t_ttut1(status: &mut i32) {
    let ((u1, u2), j) = eraTtut1_safe(2453750.5, 0.892855139, 64.8499).unwrap();
    vvd(u1, 2453750.5, 1e-6, "eraTtut1_safe", "u1", status);
    vvd(
        u2,
        0.8921045614537037037,
        1e-12,
        "eraTtut1_safe",
        "u2",
        status,
    );
    viv(j, 0, "eraTtut1_safe", "j", status);
}

fn t_ut1tai(status: &mut i32) {
    let ((a1, a2), j) = eraUt1tai_safe(2453750.5, 0.892104561, -32.6659).unwrap();
    vvd(a1, 2453750.5, 1e-6, "eraUt1tai_safe", "a1", status);
    vvd(
        a2,
        0.8924826385462962963,
        1e-12,
        "eraUt1tai_safe",
        "a2",
        status,
    );
    viv(j, 0, "eraUt1tai_safe", "j", status);
}

fn t_ut1tt(status: &mut i32) {
    let ((t1, t2), j) = eraUt1tt_safe(2453750.5, 0.892104561, 64.8499).unwrap();
    vvd(t1, 2453750.5, 1e-6, "eraUt1tt_safe", "t1", status);
    vvd(
        t2,
        0.8928551385462962963,
        1e-12,
        "eraUt1tt_safe",
        "t2",
        status,
    );
    viv(j, 0, "eraUt1tt_safe", "j", status);
}

fn t_ut1utc(status: &mut i32) {
    let ((u1, u2), j) = eraUt1utc_safe(2453750.5, 0.892104561, 0.3341).unwrap();
    vvd(u1, 2453750.5, 1e-6, "eraUt1utc_safe", "u1", status);
    vvd(
        u2,
        0.8921006941018518519,
        1e-12,
        "eraUt1utc_safe",
        "u2",
        status,
    );
    viv(j, 0, "eraUt1utc_safe", "j", status);
}

fn t_utctai(status: &mut i32) {
    let ((u1, u2), j) = eraUtctai_safe(2453750.5, 0.892100694).unwrap();
    vvd(u1, 2453750.5, 1e-6, "eraUtctai_safe", "u1", status);
    vvd(
        u2,
        0.8924826384444444444,
        1e-12,
        "eraUtctai_safe",
        "u2",
        status,
    );
    viv(j, 0, "eraUtctai_safe", "j", status);
}

fn t_utcut1(status: &mut i32) {
    let ((u1, u2), j) = eraUtcut1_safe(2453750.5, 0.892100694, 0.3341).unwrap();
    vvd(u1, 2453750.5, 1e-6, "eraUtcut1_safe", "u1", status);
    vvd(
        u2,
        0.8921045608981481481,
        1e-12,
        "eraUtcut1_safe",
        "u2",
        status,
    );
    viv(j, 0, "eraUtcut1_safe", "j", status);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        unsafe {
            VERBOSE = true;
        }
    }

    let mut status = 0;

    t_tr(&mut status);
    t_trxp(&mut status);
    t_trxpv(&mut status);
    t_tttai(&mut status);
    t_tttcg(&mut status);
    t_tttdb(&mut status);
    t_ttut1(&mut status);
    t_ut1tai(&mut status);
    t_ut1tt(&mut status);
    t_ut1utc(&mut status);
    t_utctai(&mut status);
    t_utcut1(&mut status);

    if status != 0 {
        println!("G33_safe validation failed!");
    } else {
        println!("G33_safe validation successful");
    }
    std::process::exit(status);
}
