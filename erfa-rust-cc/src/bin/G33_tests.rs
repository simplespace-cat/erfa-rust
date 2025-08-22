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

fn t_tr(status: &mut i32) {
    let mut r: [[f64; 3]; 3] = [[2.0, 3.0, 2.0], [3.0, 2.0, 3.0], [3.0, 4.0, 5.0]];
    let mut rt = [[0.0; 3]; 3];

    unsafe {
        H1::eraTr(&mut r as *mut _ as *mut f64, &mut rt as *mut _ as *mut f64);
    }

    vvd(rt[0][0], 2.0, 0.0, "eraTr", "11", status);
    vvd(rt[0][1], 3.0, 0.0, "eraTr", "12", status);
    vvd(rt[0][2], 3.0, 0.0, "eraTr", "13", status);
    vvd(rt[1][0], 3.0, 0.0, "eraTr", "21", status);
    vvd(rt[1][1], 2.0, 0.0, "eraTr", "22", status);
    vvd(rt[1][2], 4.0, 0.0, "eraTr", "23", status);
    vvd(rt[2][0], 2.0, 0.0, "eraTr", "31", status);
    vvd(rt[2][1], 3.0, 0.0, "eraTr", "32", status);
    vvd(rt[2][2], 5.0, 0.0, "eraTr", "33", status);
}

fn t_trxp(status: &mut i32) {
    let mut r: [[f64; 3]; 3] = [[2.0, 3.0, 2.0], [3.0, 2.0, 3.0], [3.0, 4.0, 5.0]];
    let mut p: [f64; 3] = [0.2, 1.5, 0.1];
    let mut trp = [0.0; 3];

    unsafe {
        H1::eraTrxp(
            &mut r as *mut _ as *mut f64,
            &mut p as *mut _ as *mut f64,
            &mut trp as *mut _ as *mut f64,
        );
    }

    vvd(trp[0], 5.2, 1e-12, "eraTrxp", "1", status);
    vvd(trp[1], 4.0, 1e-12, "eraTrxp", "2", status);
    vvd(trp[2], 5.4, 1e-12, "eraTrxp", "3", status);
}

fn t_trxpv(status: &mut i32) {
    let mut r: [[f64; 3]; 3] = [[2.0, 3.0, 2.0], [3.0, 2.0, 3.0], [3.0, 4.0, 5.0]];
    let mut pv: [[f64; 3]; 2] = [[0.2, 1.5, 0.1], [1.5, 0.2, 0.1]];
    let mut trpv = [[0.0; 3]; 2];

    unsafe {
        H1::eraTrxpv(
            &mut r as *mut _ as *mut f64,
            &mut pv as *mut _ as *mut f64,
            &mut trpv as *mut _ as *mut f64,
        );
    }

    vvd(trpv[0][0], 5.2, 1e-12, "eraTrxpv", "p1", status);
    vvd(trpv[0][1], 4.0, 1e-12, "eraTrxpv", "p1", status);
    vvd(trpv[0][2], 5.4, 1e-12, "eraTrxpv", "p1", status);
    vvd(trpv[1][0], 3.9, 1e-12, "eraTrxpv", "v1", status);
    vvd(trpv[1][1], 5.3, 1e-12, "eraTrxpv", "v2", status);
    vvd(trpv[1][2], 4.1, 1e-12, "eraTrxpv", "v3", status);
}

fn t_tttai(status: &mut i32) {
    let mut a1 = 0.0;
    let mut a2 = 0.0;

    let j = unsafe { H1::eraTttai(2453750.5, 0.892482639, &mut a1, &mut a2) };

    vvd(a1, 2453750.5, 1e-6, "eraTttai", "a1", status);
    vvd(a2, 0.892110139, 1e-12, "eraTttai", "a2", status);
    viv(j, 0, "eraTttai", "j", status);
}

fn t_tttcg(status: &mut i32) {
    let mut g1 = 0.0;
    let mut g2 = 0.0;

    let j = unsafe { H1::eraTttcg(2453750.5, 0.892482639, &mut g1, &mut g2) };

    vvd(g1, 2453750.5, 1e-6, "eraTttcg", "g1", status);
    vvd(g2, 0.8924900312508587113, 1e-12, "eraTttcg", "g2", status);
    viv(j, 0, "eraTttcg", "j", status);
}

fn t_tttdb(status: &mut i32) {
    let mut b1 = 0.0;
    let mut b2 = 0.0;

    let j = unsafe { H1::eraTttdb(2453750.5, 0.892855139, -0.000201, &mut b1, &mut b2) };

    vvd(b1, 2453750.5, 1e-6, "eraTttdb", "b1", status);
    vvd(b2, 0.8928551366736111111, 1e-12, "eraTttdb", "b2", status);
    viv(j, 0, "eraTttdb", "j", status);
}

fn t_ttut1(status: &mut i32) {
    let mut u1 = 0.0;
    let mut u2 = 0.0;

    let j = unsafe { H1::eraTtut1(2453750.5, 0.892855139, 64.8499, &mut u1, &mut u2) };

    vvd(u1, 2453750.5, 1e-6, "eraTtut1", "u1", status);
    vvd(u2, 0.8921045614537037037, 1e-12, "eraTtut1", "u2", status);
    viv(j, 0, "eraTtut1", "j", status);
}

fn t_ut1tai(status: &mut i32) {
    let mut a1 = 0.0;
    let mut a2 = 0.0;

    let j = unsafe { H1::eraUt1tai(2453750.5, 0.892104561, -32.6659, &mut a1, &mut a2) };

    vvd(a1, 2453750.5, 1e-6, "eraUt1tai", "a1", status);
    vvd(a2, 0.8924826385462962963, 1e-12, "eraUt1tai", "a2", status);
    viv(j, 0, "eraUt1tai", "j", status);
}

fn t_ut1tt(status: &mut i32) {
    let mut t1 = 0.0;
    let mut t2 = 0.0;

    let j = unsafe { H1::eraUt1tt(2453750.5, 0.892104561, 64.8499, &mut t1, &mut t2) };

    vvd(t1, 2453750.5, 1e-6, "eraUt1tt", "t1", status);
    vvd(t2, 0.8928551385462962963, 1e-12, "eraUt1tt", "t2", status);
    viv(j, 0, "eraUt1tt", "j", status);
}

fn t_ut1utc(status: &mut i32) {
    let mut u1 = 0.0;
    let mut u2 = 0.0;

    let j = unsafe { H1::eraUt1utc(2453750.5, 0.892104561, 0.3341, &mut u1, &mut u2) };

    vvd(u1, 2453750.5, 1e-6, "eraUt1utc", "u1", status);
    vvd(u2, 0.8921006941018518519, 1e-12, "eraUt1utc", "u2", status);
    viv(j, 0, "eraUt1utc", "j", status);
}

fn t_utctai(status: &mut i32) {
    let mut u1 = 0.0;
    let mut u2 = 0.0;

    let j = unsafe { H1::eraUtctai(2453750.5, 0.892100694, &mut u1, &mut u2) };

    vvd(u1, 2453750.5, 1e-6, "eraUtctai", "u1", status);
    vvd(u2, 0.8924826384444444444, 1e-12, "eraUtctai", "u2", status);
    viv(j, 0, "eraUtctai", "j", status);
}

fn t_utcut1(status: &mut i32) {
    let mut u1 = 0.0;
    let mut u2 = 0.0;

    let j = unsafe { H1::eraUtcut1(2453750.5, 0.892100694, 0.3341, &mut u1, &mut u2) };

    vvd(u1, 2453750.5, 1e-6, "eraUtcut1", "u1", status);
    vvd(u2, 0.8921045608981481481, 1e-12, "eraUtcut1", "u2", status);
    viv(j, 0, "eraUtcut1", "j", status);
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
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
