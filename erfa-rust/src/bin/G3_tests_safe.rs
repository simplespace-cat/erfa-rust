#![allow(dead_code)]

use erfa_rust::G1_safe;
use erfa_rust::G3_safe;
use erfa_rust::H1_safe;
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
    let mut astrom: H1_safe::eraASTROM = Default::default();
    astrom.along = 1.234;

    let result = G3_safe::eraAper13_safe(ut11, ut12, &mut astrom);

    match result {
        Ok(()) => {
            vvd(
                astrom.eral,
                3.316236661789694933,
                1e-12,
                "eraAper13_safe",
                "eral",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraAper13_safe failed: unexpected error");
        }
    }
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
    let mut astrom: H1_safe::eraASTROM = Default::default();

    let result = G3_safe::eraApio_safe(sp, theta, elong, phi, hm, xp, yp, refa, refb, &mut astrom);

    match result {
        Ok(()) => {
            vvd(
                astrom.along,
                -0.5278008060295995734,
                1e-12,
                "eraApio_safe",
                "along",
                status,
            );
            vvd(
                astrom.xpl,
                0.1133427418130752958e-5,
                1e-17,
                "eraApio_safe",
                "xpl",
                status,
            );
            vvd(
                astrom.ypl,
                0.1453347595780646207e-5,
                1e-17,
                "eraApio_safe",
                "ypl",
                status,
            );
            vvd(
                astrom.sphi,
                -0.9440115679003211329,
                1e-12,
                "eraApio_safe",
                "sphi",
                status,
            );
            vvd(
                astrom.cphi,
                0.3299123514971474711,
                1e-12,
                "eraApio_safe",
                "cphi",
                status,
            );
            vvd(
                astrom.diurab,
                0.5135843661699913529e-6,
                1e-12,
                "eraApio_safe",
                "diurab",
                status,
            );
            vvd(
                astrom.eral,
                2.617608903970400427,
                1e-12,
                "eraApio_safe",
                "eral",
                status,
            );
            vvd(
                astrom.refa,
                0.2014187790000000000e-3,
                1e-15,
                "eraApio_safe",
                "refa",
                status,
            );
            vvd(
                astrom.refb,
                -0.2361408310000000000e-6,
                1e-18,
                "eraApio_safe",
                "refb",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraApio_safe failed: unexpected error");
        }
    }
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
    let mut astrom: H1_safe::eraASTROM = Default::default();

    let result = G3_safe::eraApio13_safe(
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
    );

    match result {
        Ok(j) => {
            vvd(
                astrom.along,
                -0.5278008060295995733,
                1e-12,
                "eraApio13_safe",
                "along",
                status,
            );
            vvd(
                astrom.xpl,
                0.1133427418130752958e-5,
                1e-17,
                "eraApio13_safe",
                "xpl",
                status,
            );
            vvd(
                astrom.ypl,
                0.1453347595780646207e-5,
                1e-17,
                "eraApio13_safe",
                "ypl",
                status,
            );
            vvd(
                astrom.sphi,
                -0.9440115679003211329,
                1e-12,
                "eraApio13_safe",
                "sphi",
                status,
            );
            vvd(
                astrom.cphi,
                0.3299123514971474711,
                1e-12,
                "eraApio13_safe",
                "cphi",
                status,
            );
            vvd(
                astrom.diurab,
                0.5135843661699913529e-6,
                1e-12,
                "eraApio13_safe",
                "diurab",
                status,
            );
            vvd(
                astrom.eral,
                2.617608909189664000,
                1e-12,
                "eraApio13_safe",
                "eral",
                status,
            );
            vvd(
                astrom.refa,
                0.2014187785940396921e-3,
                1e-15,
                "eraApio13_safe",
                "refa",
                status,
            );
            vvd(
                astrom.refb,
                -0.2361408314943696227e-6,
                1e-18,
                "eraApio13_safe",
                "refb",
                status,
            );
            viv(j, 0, "eraApio13_safe", "j", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraApio13_safe failed: unexpected error");
        }
    }
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

    let result = G3_safe::eraAtcc13_safe(rc, dc, pr, pd, px, rv, date1, date2);

    match result {
        Ok((ra, da)) => {
            vvd(
                ra,
                2.710126504531372384,
                1e-12,
                "eraAtcc13_safe",
                "ra",
                status,
            );
            vvd(
                da,
                0.1740632537628350152,
                1e-12,
                "eraAtcc13_safe",
                "da",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraAtcc13_safe failed: unexpected error");
        }
    }
}

fn t_atccq(status: &mut i32) {
    let date1 = 2456165.5;
    let date2 = 0.401182685;
    let rc = 2.71;
    let dc = 0.174;
    let pr = 1e-5;
    let pd = 5e-6;
    let px = 0.1;
    let rv = 55.0;
    let mut astrom: H1_safe::eraASTROM = Default::default();

    let result_apci = G1_safe::eraApci13_safe(date1, date2, &mut astrom);

    match result_apci {
        Ok(_eo) => {
            let result = G3_safe::eraAtccq_safe(rc, dc, pr, pd, px, rv, &astrom);

            match result {
                Ok((ra, da)) => {
                    vvd(
                        ra,
                        2.710126504531372384,
                        1e-12,
                        "eraAtccq_safe",
                        "ra",
                        status,
                    );
                    vvd(
                        da,
                        0.1740632537628350152,
                        1e-12,
                        "eraAtccq_safe",
                        "da",
                        status,
                    );
                }
                Err(_) => {
                    *status = 1;
                    println!("eraAtccq_safe failed: unexpected error");
                }
            }
        }
        Err(_) => {
            *status = 1;
            println!("eraApci13_safe failed in t_atccq setup");
        }
    }
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
