#![allow(dead_code)]

use erfa_rust::G15_safe;
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

fn t_fad03(status: &mut i32) {
    let result = G15_safe::eraFad03_safe(0.80).expect("eraFad03_safe");
    vvd(
        result,
        1.946709205396925672,
        1e-12,
        "eraFad03_safe",
        "",
        status,
    );
}

fn t_fae03(status: &mut i32) {
    let result = G15_safe::eraFae03_safe(0.80).expect("eraFae03_safe");
    vvd(
        result,
        1.744713738913081846,
        1e-12,
        "eraFae03_safe",
        "",
        status,
    );
}

fn t_faf03(status: &mut i32) {
    let result = G15_safe::eraFaf03_safe(0.80).expect("eraFaf03_safe");
    vvd(
        result,
        0.2597711366745499518,
        1e-12,
        "eraFaf03_safe",
        "",
        status,
    );
}

fn t_faju03(status: &mut i32) {
    let result = G15_safe::eraFaju03_safe(0.80).expect("eraFaju03_safe");
    vvd(
        result,
        5.275711665202481138,
        1e-12,
        "eraFaju03_safe",
        "",
        status,
    );
}

fn t_fal03(status: &mut i32) {
    let result = G15_safe::eraFal03_safe(0.80).expect("eraFal03_safe");
    vvd(
        result,
        5.132369751108684150,
        1e-12,
        "eraFal03_safe",
        "",
        status,
    );
}

fn t_falp03(status: &mut i32) {
    let result = G15_safe::eraFalp03_safe(0.80).expect("eraFalp03_safe");
    vvd(
        result,
        6.226797973505507345,
        1e-12,
        "eraFalp03_safe",
        "",
        status,
    );
}

fn t_fama03(status: &mut i32) {
    let result = G15_safe::eraFama03_safe(0.80).expect("eraFama03_safe");
    vvd(
        result,
        3.275506840277781492,
        1e-12,
        "eraFama03_safe",
        "",
        status,
    );
}

fn t_fame03(status: &mut i32) {
    let result = G15_safe::eraFame03_safe(0.80).expect("eraFame03_safe");
    vvd(
        result,
        5.417338184297289661,
        1e-12,
        "eraFame03_safe",
        "",
        status,
    );
}

fn t_fane03(status: &mut i32) {
    let result = G15_safe::eraFane03_safe(0.80).expect("eraFane03_safe");
    vvd(
        result,
        2.079343830860413523,
        1e-12,
        "eraFane03_safe",
        "",
        status,
    );
}

fn t_faom03(status: &mut i32) {
    let result = G15_safe::eraFaom03_safe(0.80).expect("eraFaom03_safe");
    vvd(
        result,
        -5.973618440951302183,
        1e-12,
        "eraFaom03_safe",
        "",
        status,
    );
}

fn t_fapa03(status: &mut i32) {
    let result = G15_safe::eraFapa03_safe(0.80).expect("eraFapa03_safe");
    vvd(
        result,
        0.1950884762240000000e-1,
        1e-12,
        "eraFapa03_safe",
        "",
        status,
    );
}

fn t_fasa03(status: &mut i32) {
    let result = G15_safe::eraFasa03_safe(0.80).expect("eraFasa03_safe");
    vvd(
        result,
        5.371574539440827046,
        1e-12,
        "eraFasa03_safe",
        "",
        status,
    );
}

fn t_faur03(status: &mut i32) {
    let result = G15_safe::eraFaur03_safe(0.80).expect("eraFaur03_safe");
    vvd(
        result,
        5.180636450180413523,
        1e-12,
        "eraFaur03_safe",
        "",
        status,
    );
}

fn t_fave03(status: &mut i32) {
    let result = G15_safe::eraFave03_safe(0.80).expect("eraFave03_safe");
    vvd(
        result,
        3.424900460533758000,
        1e-12,
        "eraFave03_safe",
        "",
        status,
    );
}

fn t_fk5hip(status: &mut i32) {
    let (r5h, s5h) = G15_safe::eraFk5hip_safe().expect("eraFk5hip_safe");

    vvd(
        r5h[0][0],
        0.9999999999999928638,
        1e-14,
        "eraFk5hip_safe",
        "11",
        status,
    );
    vvd(
        r5h[0][1],
        0.1110223351022919694e-6,
        1e-17,
        "eraFk5hip_safe",
        "12",
        status,
    );
    vvd(
        r5h[0][2],
        0.4411803962536558154e-7,
        1e-17,
        "eraFk5hip_safe",
        "13",
        status,
    );
    vvd(
        r5h[1][0],
        -0.1110223308458746430e-6,
        1e-17,
        "eraFk5hip_safe",
        "21",
        status,
    );
    vvd(
        r5h[1][1],
        0.9999999999999891830,
        1e-14,
        "eraFk5hip_safe",
        "22",
        status,
    );
    vvd(
        r5h[1][2],
        -0.9647792498984142358e-7,
        1e-17,
        "eraFk5hip_safe",
        "23",
        status,
    );
    vvd(
        r5h[2][0],
        -0.4411805033656962252e-7,
        1e-17,
        "eraFk5hip_safe",
        "31",
        status,
    );
    vvd(
        r5h[2][1],
        0.9647792009175314354e-7,
        1e-17,
        "eraFk5hip_safe",
        "32",
        status,
    );
    vvd(
        r5h[2][2],
        0.9999999999999943728,
        1e-14,
        "eraFk5hip_safe",
        "33",
        status,
    );
    vvd(
        s5h[0],
        -0.1454441043328607981e-8,
        1e-17,
        "eraFk5hip_safe",
        "s1",
        status,
    );
    vvd(
        s5h[1],
        0.2908882086657215962e-8,
        1e-17,
        "eraFk5hip_safe",
        "s2",
        status,
    );
    vvd(
        s5h[2],
        0.3393695767766751955e-8,
        1e-17,
        "eraFk5hip_safe",
        "s3",
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

    t_fad03(&mut status);
    t_fae03(&mut status);
    t_faf03(&mut status);
    t_faju03(&mut status);
    t_fal03(&mut status);
    t_falp03(&mut status);
    t_fama03(&mut status);
    t_fame03(&mut status);
    t_fane03(&mut status);
    t_faom03(&mut status);
    t_fapa03(&mut status);
    t_fasa03(&mut status);
    t_faur03(&mut status);
    t_fave03(&mut status);
    t_fk5hip(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
