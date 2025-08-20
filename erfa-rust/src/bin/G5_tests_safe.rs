#![allow(dead_code)]
#![allow(non_snake_case)]

use erfa_rust::G3_safe;
use erfa_rust::G5_safe;
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

fn t_atoc13(status: &mut i32) {
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

    let ob1_r = 2.710085107986886201;
    let ob2_r = 0.1717653435758265198;
    let result = G5_safe::eraAtoc13_safe(
        "R", ob1_r, ob2_r, utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, wl,
    );

    match result {
        Ok((rc, dc, eo, j)) => {
            vvd(
                rc,
                2.709956744659136129,
                1e-12,
                "eraAtoc13_safe",
                "R/rc",
                status,
            );
            vvd(
                dc,
                0.1741696500898471362,
                1e-12,
                "eraAtoc13_safe",
                "R/dc",
                status,
            );
            if eo.is_nan() || eo.is_infinite() {
                *status = 1;
                println!("eraAtoc13_safe failed: invalid eo value");
            }
            viv(j, 0, "eraAtoc13_safe", "R/j", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraAtoc13_safe failed: unexpected error for R");
        }
    }

    let ob1_h = -0.09247619879782006106;
    let ob2_h = 0.1717653435758265198;
    let result = G5_safe::eraAtoc13_safe(
        "H", ob1_h, ob2_h, utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, wl,
    );

    match result {
        Ok((rc, dc, eo, j)) => {
            vvd(
                rc,
                2.709956744659734086,
                1e-12,
                "eraAtoc13_safe",
                "H/rc",
                status,
            );
            vvd(
                dc,
                0.1741696500898471362,
                1e-12,
                "eraAtoc13_safe",
                "H/dc",
                status,
            );
            if eo.is_nan() || eo.is_infinite() {
                *status = 1;
                println!("eraAtoc13_safe failed: invalid eo value");
            }
            viv(j, 0, "eraAtoc13_safe", "H/j", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraAtoc13_safe failed: unexpected error for H");
        }
    }

    let ob1_a = 0.09233952224794989993;
    let ob2_a = 1.407758704513722461;
    let result = G5_safe::eraAtoc13_safe(
        "A", ob1_a, ob2_a, utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, wl,
    );

    match result {
        Ok((rc, dc, eo, j)) => {
            vvd(
                rc,
                2.709956744659734086,
                1e-12,
                "eraAtoc13_safe",
                "A/rc",
                status,
            );
            vvd(
                dc,
                0.1741696500898471366,
                1e-12,
                "eraAtoc13_safe",
                "A/dc",
                status,
            );
            if eo.is_nan() || eo.is_infinite() {
                *status = 1;
                println!("eraAtoc13_safe failed: invalid eo value");
            }
            viv(j, 0, "eraAtoc13_safe", "A/j", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraAtoc13_safe failed: unexpected error for A");
        }
    }
}

fn t_atoi13(status: &mut i32) {
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

    let ob1_r = 2.710085107986886201;
    let ob2_r = 0.1717653435758265198;
    let result = G5_safe::eraAtoi13_safe(
        "R", ob1_r, ob2_r, utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, wl,
    );

    match result {
        Ok((ri, di, j)) => {
            vvd(
                ri,
                2.710121574447540810,
                1e-12,
                "eraAtoi13_safe",
                "R/ri",
                status,
            );
            vvd(
                di,
                0.1729371839116608778,
                1e-12,
                "eraAtoi13_safe",
                "R/di",
                status,
            );
            viv(j, 0, "eraAtoi13_safe", "R/J", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraAtoi13_safe failed: unexpected error for R");
        }
    }

    let ob1_h = -0.09247619879782006106;
    let ob2_h = 0.1717653435758265198;
    let result = G5_safe::eraAtoi13_safe(
        "H", ob1_h, ob2_h, utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, wl,
    );

    match result {
        Ok((ri, di, j)) => {
            vvd(
                ri,
                2.710121574448138676,
                1e-12,
                "eraAtoi13_safe",
                "H/ri",
                status,
            );
            vvd(
                di,
                0.1729371839116608778,
                1e-12,
                "eraAtoi13_safe",
                "H/di",
                status,
            );
            viv(j, 0, "eraAtoi13_safe", "H/J", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraAtoi13_safe failed: unexpected error for H");
        }
    }

    let ob1_a = 0.09233952224794989993;
    let ob2_a = 1.407758704513722461;
    let result = G5_safe::eraAtoi13_safe(
        "A", ob1_a, ob2_a, utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, wl,
    );

    match result {
        Ok((ri, di, j)) => {
            vvd(
                ri,
                2.710121574448138676,
                1e-12,
                "eraAtoi13_safe",
                "A/ri",
                status,
            );
            vvd(
                di,
                0.1729371839116608781,
                1e-12,
                "eraAtoi13_safe",
                "A/di",
                status,
            );
            viv(j, 0, "eraAtoi13_safe", "A/J", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraAtoi13_safe failed: unexpected error for A");
        }
    }
}

fn t_atoiq(status: &mut i32) {
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

    let mut astrom = H1_safe::eraASTROM::default();
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
        Ok(_) => {
            let ob1_r = 2.710085107986886201;
            let ob2_r = 0.1717653435758265198;
            let result = G5_safe::eraAtoiq_safe("R", ob1_r, ob2_r, &astrom);

            match result {
                Ok((ri, di)) => {
                    vvd(
                        ri,
                        2.710121574447540810,
                        1e-12,
                        "eraAtoiq_safe",
                        "R/ri",
                        status,
                    );
                    vvd(
                        di,
                        0.17293718391166087785,
                        1e-12,
                        "eraAtoiq_safe",
                        "R/di",
                        status,
                    );
                }
                Err(_) => {
                    *status = 1;
                    println!("eraAtoiq_safe failed: unexpected error for R");
                }
            }

            let ob1_h = -0.09247619879782006106;
            let ob2_h = 0.1717653435758265198;
            let result = G5_safe::eraAtoiq_safe("H", ob1_h, ob2_h, &astrom);

            match result {
                Ok((ri, di)) => {
                    vvd(
                        ri,
                        2.710121574448138676,
                        1e-12,
                        "eraAtoiq_safe",
                        "H/ri",
                        status,
                    );
                    vvd(
                        di,
                        0.1729371839116608778,
                        1e-12,
                        "eraAtoiq_safe",
                        "H/di",
                        status,
                    );
                }
                Err(_) => {
                    *status = 1;
                    println!("eraAtoiq_safe failed: unexpected error for H");
                }
            }

            let ob1_a = 0.09233952224794989993;
            let ob2_a = 1.407758704513722461;
            let result = G5_safe::eraAtoiq_safe("A", ob1_a, ob2_a, &astrom);

            match result {
                Ok((ri, di)) => {
                    vvd(
                        ri,
                        2.710121574448138676,
                        1e-12,
                        "eraAtoiq_safe",
                        "A/ri",
                        status,
                    );
                    vvd(
                        di,
                        0.1729371839116608781,
                        1e-12,
                        "eraAtoiq_safe",
                        "A/di",
                        status,
                    );
                }
                Err(_) => {
                    *status = 1;
                    println!("eraAtoiq_safe failed: unexpected error for A");
                }
            }
        }
        Err(_) => {
            *status = 1;
            println!("eraApio13_safe failed: unexpected error");
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

    t_atoc13(&mut status);
    t_atoi13(&mut status);
    t_atoiq(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
