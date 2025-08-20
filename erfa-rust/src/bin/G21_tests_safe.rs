#![allow(dead_code)]

use erfa_rust::G21_safe;
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

fn t_moon98(status: &mut i32) {
    let result = G21_safe::eraMoon98_safe(2400000.5, 43999.9);

    match result {
        Ok(pv) => {
            vvd(
                pv[0][0],
                -0.2601295959971044180e-2,
                1e-11,
                "eraMoon98_safe",
                "x 4",
                status,
            );
            vvd(
                pv[0][1],
                0.6139750944302742189e-3,
                1e-11,
                "eraMoon98_safe",
                "y 4",
                status,
            );
            vvd(
                pv[0][2],
                0.2640794528229828909e-3,
                1e-11,
                "eraMoon98_safe",
                "z 4",
                status,
            );
            vvd(
                pv[1][0],
                -0.1244321506649895021e-3,
                1e-11,
                "eraMoon98_safe",
                "xd 4",
                status,
            );
            vvd(
                pv[1][1],
                -0.5219076942678119398e-3,
                1e-11,
                "eraMoon98_safe",
                "yd 4",
                status,
            );
            vvd(
                pv[1][2],
                -0.1716132214378462047e-3,
                1e-11,
                "eraMoon98_safe",
                "zd 4",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraMoon98_safe failed: unexpected error");
        }
    }
}

fn t_num00a(status: &mut i32) {
    let result = G21_safe::eraNum00a_safe(2400000.5, 53736.0);

    match result {
        Ok(rmatn) => {
            vvd(
                rmatn[0][0],
                0.9999999999536227949,
                1e-12,
                "eraNum00a_safe",
                "11",
                status,
            );
            vvd(
                rmatn[0][1],
                0.8836238544090873336e-5,
                1e-12,
                "eraNum00a_safe",
                "12",
                status,
            );
            vvd(
                rmatn[0][2],
                0.3830835237722400669e-5,
                1e-12,
                "eraNum00a_safe",
                "13",
                status,
            );
            vvd(
                rmatn[1][0],
                -0.8836082880798569274e-5,
                1e-12,
                "eraNum00a_safe",
                "21",
                status,
            );
            vvd(
                rmatn[1][1],
                0.9999999991354655028,
                1e-12,
                "eraNum00a_safe",
                "22",
                status,
            );
            vvd(
                rmatn[1][2],
                -0.4063240865362499850e-4,
                1e-12,
                "eraNum00a_safe",
                "23",
                status,
            );
            vvd(
                rmatn[2][0],
                -0.3831194272065995866e-5,
                1e-12,
                "eraNum00a_safe",
                "31",
                status,
            );
            vvd(
                rmatn[2][1],
                0.4063237480216291775e-4,
                1e-12,
                "eraNum00a_safe",
                "32",
                status,
            );
            vvd(
                rmatn[2][2],
                0.9999999991671660338,
                1e-12,
                "eraNum00a_safe",
                "33",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraNum00a_safe failed: unexpected error");
        }
    }
}

fn t_num00b(status: &mut i32) {
    let result = G21_safe::eraNum00b_safe(2400000.5, 53736.0);

    match result {
        Ok(rmatn) => {
            vvd(
                rmatn[0][0],
                0.9999999999536069682,
                1e-12,
                "eraNum00b_safe",
                "11",
                status,
            );
            vvd(
                rmatn[0][1],
                0.8837746144871248011e-5,
                1e-12,
                "eraNum00b_safe",
                "12",
                status,
            );
            vvd(
                rmatn[0][2],
                0.3831488838252202945e-5,
                1e-12,
                "eraNum00b_safe",
                "13",
                status,
            );
            vvd(
                rmatn[1][0],
                -0.8837590456632304720e-5,
                1e-12,
                "eraNum00b_safe",
                "21",
                status,
            );
            vvd(
                rmatn[1][1],
                0.9999999991354692733,
                1e-12,
                "eraNum00b_safe",
                "22",
                status,
            );
            vvd(
                rmatn[1][2],
                -0.4063198798559591654e-4,
                1e-12,
                "eraNum00b_safe",
                "23",
                status,
            );
            vvd(
                rmatn[2][0],
                -0.3831847930134941271e-5,
                1e-12,
                "eraNum00b_safe",
                "31",
                status,
            );
            vvd(
                rmatn[2][1],
                0.4063195412258168380e-4,
                1e-12,
                "eraNum00b_safe",
                "32",
                status,
            );
            vvd(
                rmatn[2][2],
                0.9999999991671806225,
                1e-12,
                "eraNum00b_safe",
                "33",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraNum00b_safe failed: unexpected error");
        }
    }
}

fn t_num06a(status: &mut i32) {
    let result = G21_safe::eraNum06a_safe(2400000.5, 53736.0);

    match result {
        Ok(rmatn) => {
            vvd(
                rmatn[0][0],
                0.9999999999536227668,
                1e-12,
                "eraNum06a_safe",
                "11",
                status,
            );
            vvd(
                rmatn[0][1],
                0.8836241998111535233e-5,
                1e-12,
                "eraNum06a_safe",
                "12",
                status,
            );
            vvd(
                rmatn[0][2],
                0.3830834608415287707e-5,
                1e-12,
                "eraNum06a_safe",
                "13",
                status,
            );
            vvd(
                rmatn[1][0],
                -0.8836086334870740138e-5,
                1e-12,
                "eraNum06a_safe",
                "21",
                status,
            );
            vvd(
                rmatn[1][1],
                0.9999999991354657474,
                1e-12,
                "eraNum06a_safe",
                "22",
                status,
            );
            vvd(
                rmatn[1][2],
                -0.4063240188248455065e-4,
                1e-12,
                "eraNum06a_safe",
                "23",
                status,
            );
            vvd(
                rmatn[2][0],
                -0.3831193642839398128e-5,
                1e-12,
                "eraNum06a_safe",
                "31",
                status,
            );
            vvd(
                rmatn[2][1],
                0.4063236803101479770e-4,
                1e-12,
                "eraNum06a_safe",
                "32",
                status,
            );
            vvd(
                rmatn[2][2],
                0.9999999991671663114,
                1e-12,
                "eraNum06a_safe",
                "33",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraNum06a_safe failed: unexpected error");
        }
    }
}

fn t_numat(status: &mut i32) {
    let epsa = 0.4090789763356509900;
    let dpsi = -0.9630909107115582393e-5;
    let deps = 0.4063239174001678826e-4;

    let result = G21_safe::eraNumat_safe(epsa, dpsi, deps);

    match result {
        Ok(rmatn) => {
            vvd(
                rmatn[0][0],
                0.9999999999536227949,
                1e-12,
                "eraNumat_safe",
                "11",
                status,
            );
            vvd(
                rmatn[0][1],
                0.8836239320236250577e-5,
                1e-12,
                "eraNumat_safe",
                "12",
                status,
            );
            vvd(
                rmatn[0][2],
                0.3830833447458251908e-5,
                1e-12,
                "eraNumat_safe",
                "13",
                status,
            );
            vvd(
                rmatn[1][0],
                -0.8836083657016688588e-5,
                1e-12,
                "eraNumat_safe",
                "21",
                status,
            );
            vvd(
                rmatn[1][1],
                0.9999999991354654959,
                1e-12,
                "eraNumat_safe",
                "22",
                status,
            );
            vvd(
                rmatn[1][2],
                -0.4063240865361857698e-4,
                1e-12,
                "eraNumat_safe",
                "23",
                status,
            );
            vvd(
                rmatn[2][0],
                -0.3831192481833385226e-5,
                1e-12,
                "eraNumat_safe",
                "31",
                status,
            );
            vvd(
                rmatn[2][1],
                0.4063237480216934159e-4,
                1e-12,
                "eraNumat_safe",
                "32",
                status,
            );
            vvd(
                rmatn[2][2],
                0.9999999991671660407,
                1e-12,
                "eraNumat_safe",
                "33",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraNumat_safe failed: unexpected error");
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

    t_moon98(&mut status);
    t_num00a(&mut status);
    t_num00b(&mut status);
    t_num06a(&mut status);
    t_numat(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
