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

fn t_moon98(status: &mut i32) {
    let mut pv = [[0.0; 3]; 2];

    unsafe {
        H1::eraMoon98(
            2400000.5,
            43999.9,
            &mut pv as *mut [[f64; 3]; 2] as *mut f64,
        );
    }

    vvd(
        pv[0][0],
        -0.2601295959971044180e-2,
        1e-11,
        "eraMoon98",
        "x 4",
        status,
    );
    vvd(
        pv[0][1],
        0.6139750944302742189e-3,
        1e-11,
        "eraMoon98",
        "y 4",
        status,
    );
    vvd(
        pv[0][2],
        0.2640794528229828909e-3,
        1e-11,
        "eraMoon98",
        "z 4",
        status,
    );
    vvd(
        pv[1][0],
        -0.1244321506649895021e-3,
        1e-11,
        "eraMoon98",
        "xd 4",
        status,
    );
    vvd(
        pv[1][1],
        -0.5219076942678119398e-3,
        1e-11,
        "eraMoon98",
        "yd 4",
        status,
    );
    vvd(
        pv[1][2],
        -0.1716132214378462047e-3,
        1e-11,
        "eraMoon98",
        "zd 4",
        status,
    );
}

fn t_num00a(status: &mut i32) {
    let mut rmatn = [[0.0; 3]; 3];

    unsafe {
        H1::eraNum00a(
            2400000.5,
            53736.0,
            &mut rmatn as *mut [[f64; 3]; 3] as *mut f64,
        );
    }

    vvd(
        rmatn[0][0],
        0.9999999999536227949,
        1e-12,
        "eraNum00a",
        "11",
        status,
    );
    vvd(
        rmatn[0][1],
        0.8836238544090873336e-5,
        1e-12,
        "eraNum00a",
        "12",
        status,
    );
    vvd(
        rmatn[0][2],
        0.3830835237722400669e-5,
        1e-12,
        "eraNum00a",
        "13",
        status,
    );
    vvd(
        rmatn[1][0],
        -0.8836082880798569274e-5,
        1e-12,
        "eraNum00a",
        "21",
        status,
    );
    vvd(
        rmatn[1][1],
        0.9999999991354655028,
        1e-12,
        "eraNum00a",
        "22",
        status,
    );
    vvd(
        rmatn[1][2],
        -0.4063240865362499850e-4,
        1e-12,
        "eraNum00a",
        "23",
        status,
    );
    vvd(
        rmatn[2][0],
        -0.3831194272065995866e-5,
        1e-12,
        "eraNum00a",
        "31",
        status,
    );
    vvd(
        rmatn[2][1],
        0.4063237480216291775e-4,
        1e-12,
        "eraNum00a",
        "32",
        status,
    );
    vvd(
        rmatn[2][2],
        0.9999999991671660338,
        1e-12,
        "eraNum00a",
        "33",
        status,
    );
}

fn t_num00b(status: &mut i32) {
    let mut rmatn = [[0.0; 3]; 3];

    unsafe {
        H1::eraNum00b(
            2400000.5,
            53736.0,
            &mut rmatn as *mut [[f64; 3]; 3] as *mut f64,
        );
    }

    vvd(
        rmatn[0][0],
        0.9999999999536069682,
        1e-12,
        "eraNum00b",
        "11",
        status,
    );
    vvd(
        rmatn[0][1],
        0.8837746144871248011e-5,
        1e-12,
        "eraNum00b",
        "12",
        status,
    );
    vvd(
        rmatn[0][2],
        0.3831488838252202945e-5,
        1e-12,
        "eraNum00b",
        "13",
        status,
    );
    vvd(
        rmatn[1][0],
        -0.8837590456632304720e-5,
        1e-12,
        "eraNum00b",
        "21",
        status,
    );
    vvd(
        rmatn[1][1],
        0.9999999991354692733,
        1e-12,
        "eraNum00b",
        "22",
        status,
    );
    vvd(
        rmatn[1][2],
        -0.4063198798559591654e-4,
        1e-12,
        "eraNum00b",
        "23",
        status,
    );
    vvd(
        rmatn[2][0],
        -0.3831847930134941271e-5,
        1e-12,
        "eraNum00b",
        "31",
        status,
    );
    vvd(
        rmatn[2][1],
        0.4063195412258168380e-4,
        1e-12,
        "eraNum00b",
        "32",
        status,
    );
    vvd(
        rmatn[2][2],
        0.9999999991671806225,
        1e-12,
        "eraNum00b",
        "33",
        status,
    );
}

fn t_num06a(status: &mut i32) {
    let mut rmatn = [[0.0; 3]; 3];

    unsafe {
        H1::eraNum06a(
            2400000.5,
            53736.0,
            &mut rmatn as *mut [[f64; 3]; 3] as *mut f64,
        );
    }

    vvd(
        rmatn[0][0],
        0.9999999999536227668,
        1e-12,
        "eraNum06a",
        "11",
        status,
    );
    vvd(
        rmatn[0][1],
        0.8836241998111535233e-5,
        1e-12,
        "eraNum06a",
        "12",
        status,
    );
    vvd(
        rmatn[0][2],
        0.3830834608415287707e-5,
        1e-12,
        "eraNum06a",
        "13",
        status,
    );
    vvd(
        rmatn[1][0],
        -0.8836086334870740138e-5,
        1e-12,
        "eraNum06a",
        "21",
        status,
    );
    vvd(
        rmatn[1][1],
        0.9999999991354657474,
        1e-12,
        "eraNum06a",
        "22",
        status,
    );
    vvd(
        rmatn[1][2],
        -0.4063240188248455065e-4,
        1e-12,
        "eraNum06a",
        "23",
        status,
    );
    vvd(
        rmatn[2][0],
        -0.3831193642839398128e-5,
        1e-12,
        "eraNum06a",
        "31",
        status,
    );
    vvd(
        rmatn[2][1],
        0.4063236803101479770e-4,
        1e-12,
        "eraNum06a",
        "32",
        status,
    );
    vvd(
        rmatn[2][2],
        0.9999999991671663114,
        1e-12,
        "eraNum06a",
        "33",
        status,
    );
}

fn t_numat(status: &mut i32) {
    let epsa = 0.4090789763356509900;
    let dpsi = -0.9630909107115582393e-5;
    let deps = 0.4063239174001678826e-4;
    let mut rmatn = [[0.0; 3]; 3];

    unsafe {
        H1::eraNumat(
            epsa,
            dpsi,
            deps,
            &mut rmatn as *mut [[f64; 3]; 3] as *mut f64,
        );
    }

    vvd(
        rmatn[0][0],
        0.9999999999536227949,
        1e-12,
        "eraNumat",
        "11",
        status,
    );
    vvd(
        rmatn[0][1],
        0.8836239320236250577e-5,
        1e-12,
        "eraNumat",
        "12",
        status,
    );
    vvd(
        rmatn[0][2],
        0.3830833447458251908e-5,
        1e-12,
        "eraNumat",
        "13",
        status,
    );
    vvd(
        rmatn[1][0],
        -0.8836083657016688588e-5,
        1e-12,
        "eraNumat",
        "21",
        status,
    );
    vvd(
        rmatn[1][1],
        0.9999999991354654959,
        1e-12,
        "eraNumat",
        "22",
        status,
    );
    vvd(
        rmatn[1][2],
        -0.4063240865361857698e-4,
        1e-12,
        "eraNumat",
        "23",
        status,
    );
    vvd(
        rmatn[2][0],
        -0.3831192481833385226e-5,
        1e-12,
        "eraNumat",
        "31",
        status,
    );
    vvd(
        rmatn[2][1],
        0.4063237480216934159e-4,
        1e-12,
        "eraNumat",
        "32",
        status,
    );
    vvd(
        rmatn[2][2],
        0.9999999991671660407,
        1e-12,
        "eraNumat",
        "33",
        status,
    );
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
