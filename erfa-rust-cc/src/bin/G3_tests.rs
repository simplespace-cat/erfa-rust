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

fn t_aper13(status: &mut i32) {
    let ut11 = 2456165.5;
    let ut12 = 0.401182685;
    let mut astrom: H1::eraASTROM = Default::default();
    astrom.along = 1.234;

    unsafe {
        H1::eraAper13(ut11, ut12, &mut astrom);
    }

    vvd(
        astrom.eral,
        3.316236661789694933,
        1e-12,
        "eraAper13",
        "pmt",
        status,
    );
}

fn t_apio(status: &mut i32) {
    let sp = -3.01974337e-11;
    let theta = 3.14540971;
    let elong = -0.527800806;
    let phi = -1.2345856;
    let hm = 2738.0;
    let xp = 2.47230737e-7;
    let yp = 1.82640464e-6;
    let refa = 0.000201418779;
    let refb = -2.36140831e-7;
    let mut astrom: H1::eraASTROM = Default::default();

    unsafe {
        H1::eraApio(sp, theta, elong, phi, hm, xp, yp, refa, refb, &mut astrom);
    }

    vvd(
        astrom.along,
        -0.5278008060295995734,
        1e-12,
        "eraApio",
        "along",
        status,
    );
    vvd(
        astrom.xpl,
        0.1133427418130752958e-5,
        1e-17,
        "eraApio",
        "xpl",
        status,
    );
    vvd(
        astrom.ypl,
        0.1453347595780646207e-5,
        1e-17,
        "eraApio",
        "ypl",
        status,
    );
    vvd(
        astrom.sphi,
        -0.9440115679003211329,
        1e-12,
        "eraApio",
        "sphi",
        status,
    );
    vvd(
        astrom.cphi,
        0.3299123514971474711,
        1e-12,
        "eraApio",
        "cphi",
        status,
    );
    vvd(
        astrom.diurab,
        0.5135843661699913529e-6,
        1e-12,
        "eraApio",
        "diurab",
        status,
    );
    vvd(
        astrom.eral,
        2.617608903970400427,
        1e-12,
        "eraApio",
        "eral",
        status,
    );
    vvd(
        astrom.refa,
        0.2014187790000000000e-3,
        1e-15,
        "eraApio",
        "refa",
        status,
    );
    vvd(
        astrom.refb,
        -0.2361408310000000000e-6,
        1e-18,
        "eraApio",
        "refb",
        status,
    );
}

fn t_apio13(status: &mut i32) {
    let utc1 = 2456384.5;
    let utc2 = 0.969254051;
    let dut1 = 0.1550675;
    let elong = -0.527800806;
    let phi = -1.2345856;
    let hm = 2738.0;
    let xp = 2.47230737e-7;
    let yp = 1.82640464e-6;
    let phpa = 731.0;
    let tc = 12.8;
    let rh = 0.59;
    let wl = 0.55;
    let mut astrom: H1::eraASTROM = Default::default();

    let j = unsafe {
        H1::eraApio13(
            utc1,
            utc2,
            dut1,
            elong,
            phi,
            hm,
            xp,
            yp,
            phpa,
            tc,
            rh,
            wl,
            &mut astrom,
        )
    };

    vvd(
        astrom.along,
        -0.5278008060295995733,
        1e-12,
        "eraApio13",
        "along",
        status,
    );
    vvd(
        astrom.xpl,
        0.1133427418130752958e-5,
        1e-17,
        "eraApio13",
        "xpl",
        status,
    );
    vvd(
        astrom.ypl,
        0.1453347595780646207e-5,
        1e-17,
        "eraApio13",
        "ypl",
        status,
    );
    vvd(
        astrom.sphi,
        -0.9440115679003211329,
        1e-12,
        "eraApio13",
        "sphi",
        status,
    );
    vvd(
        astrom.cphi,
        0.3299123514971474711,
        1e-12,
        "eraApio13",
        "cphi",
        status,
    );
    vvd(
        astrom.diurab,
        0.5135843661699913529e-6,
        1e-12,
        "eraApio13",
        "diurab",
        status,
    );
    vvd(
        astrom.eral,
        2.617608909189664000,
        1e-12,
        "eraApio13",
        "eral",
        status,
    );
    vvd(
        astrom.refa,
        0.2014187785940396921e-3,
        1e-15,
        "eraApio13",
        "refa",
        status,
    );
    vvd(
        astrom.refb,
        -0.2361408314943696227e-6,
        1e-18,
        "eraApio13",
        "refb",
        status,
    );
    viv(j, 0, "eraApio13", "j", status);
}

fn t_atcc13(status: &mut i32) {
    let rc = 2.71;
    let dc = 0.174;
    let pr = 1e-5;
    let pd = 5e-6;
    let px = 0.1;
    let rv = 55.0;
    let date1 = 2456165.5;
    let date2 = 0.401182685;
    let mut ra = 0.0;
    let mut da = 0.0;

    unsafe {
        H1::eraAtcc13(rc, dc, pr, pd, px, rv, date1, date2, &mut ra, &mut da);
    }

    vvd(ra, 2.710126504531372384, 1e-12, "eraAtcc13", "ra", status);
    vvd(da, 0.1740632537628350152, 1e-12, "eraAtcc13", "da", status);
}

fn t_atccq(status: &mut i32) {
    let date1 = 2456165.5;
    let date2 = 0.401182685;
    let mut eo = 0.0;
    let rc = 2.71;
    let dc = 0.174;
    let pr = 1e-5;
    let pd = 5e-6;
    let px = 0.1;
    let rv = 55.0;
    let mut ra = 0.0;
    let mut da = 0.0;
    let mut astrom: H1::eraASTROM = Default::default();

    unsafe {
        H1::eraApci13(date1, date2, &mut astrom, &mut eo);
    }

    unsafe {
        H1::eraAtccq(
            rc,
            dc,
            pr,
            pd,
            px,
            rv,
            &mut astrom as *mut H1::eraASTROM,
            &mut ra,
            &mut da,
        );
    }

    vvd(ra, 2.710126504531372384, 1e-12, "eraAtccq", "ra", status);
    vvd(da, 0.1740632537628350152, 1e-12, "eraAtccq", "da", status);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        unsafe {
            VERBOSE = true;
        }
    }

    let mut status = 0;

    t_aper13(&mut status);
    t_apio(&mut status);
    t_apio13(&mut status);
    t_atcc13(&mut status);
    t_atccq(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
