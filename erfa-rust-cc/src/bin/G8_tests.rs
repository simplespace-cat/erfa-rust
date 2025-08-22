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

fn t_c2tcio(status: &mut i32) {
    let rc2i = [
        [
            0.9999998323037164738,
            0.5581526271714303683e-9,
            -0.5791308477073443903e-3,
        ],
        [
            -0.2384266227524722273e-7,
            0.9999999991917404296,
            -0.4020594955030704125e-4,
        ],
        [
            0.5791308472168153320e-3,
            0.4020595661593994396e-4,
            0.9999998314954572365,
        ],
    ];
    let era = 1.75283325530307;
    let rpom = [
        [
            0.9999999999999674705,
            -0.1367174580728847031e-10,
            0.2550602379999972723e-6,
        ],
        [
            0.1414624947957029721e-10,
            0.9999999999982694954,
            -0.1860359246998866338e-5,
        ],
        [
            -0.2550602379741215275e-6,
            0.1860359247002413923e-5,
            0.9999999999982369658,
        ],
    ];
    let mut rc2t = [[0.0; 3]; 3];

    unsafe {
        H1::eraC2tcio(
            &rc2i as *const _ as *const f64,
            era,
            &rpom as *const _ as *const f64,
            &mut rc2t as *mut _ as *mut f64,
        );
    }

    vvd(
        rc2t[0][0],
        -0.1810332128307110439,
        1e-12,
        "eraC2tcio",
        "11",
        status,
    );
    vvd(
        rc2t[0][1],
        0.9834769806938470149,
        1e-12,
        "eraC2tcio",
        "12",
        status,
    );
    vvd(
        rc2t[0][2],
        0.6555535638685466874e-4,
        1e-12,
        "eraC2tcio",
        "13",
        status,
    );
    vvd(
        rc2t[1][0],
        -0.9834768134135996657,
        1e-12,
        "eraC2tcio",
        "21",
        status,
    );
    vvd(
        rc2t[1][1],
        -0.1810332203649448367,
        1e-12,
        "eraC2tcio",
        "22",
        status,
    );
    vvd(
        rc2t[1][2],
        0.5749801116141106528e-3,
        1e-12,
        "eraC2tcio",
        "23",
        status,
    );
    vvd(
        rc2t[2][0],
        0.5773474014081407076e-3,
        1e-12,
        "eraC2tcio",
        "31",
        status,
    );
    vvd(
        rc2t[2][1],
        0.3961832391772658944e-4,
        1e-12,
        "eraC2tcio",
        "32",
        status,
    );
    vvd(
        rc2t[2][2],
        0.9999998325501691969,
        1e-12,
        "eraC2tcio",
        "33",
        status,
    );
}

fn t_c2teqx(status: &mut i32) {
    let rbpn = [
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
    let gst = 1.754166138040730516;
    let rpom = [
        [
            0.9999999999999674705,
            -0.1367174580728847031e-10,
            0.2550602379999972723e-6,
        ],
        [
            0.1414624947957029721e-10,
            0.9999999999982694954,
            -0.1860359246998866338e-5,
        ],
        [
            -0.2550602379741215275e-6,
            0.1860359247002413923e-5,
            0.9999999999982369658,
        ],
    ];
    let mut rc2t = [[0.0; 3]; 3];

    unsafe {
        H1::eraC2teqx(
            &rbpn as *const _ as *const f64,
            gst,
            &rpom as *const _ as *const f64,
            &mut rc2t as *mut _ as *mut f64,
        );
    }

    vvd(
        rc2t[0][0],
        -0.1810332128528685730,
        1e-12,
        "eraC2teqx",
        "11",
        status,
    );
    vvd(
        rc2t[0][1],
        0.9834769806897685071,
        1e-12,
        "eraC2teqx",
        "12",
        status,
    );
    vvd(
        rc2t[0][2],
        0.6555535639982634449e-4,
        1e-12,
        "eraC2teqx",
        "13",
        status,
    );
    vvd(
        rc2t[1][0],
        -0.9834768134095211257,
        1e-12,
        "eraC2teqx",
        "21",
        status,
    );
    vvd(
        rc2t[1][1],
        -0.1810332203871023800,
        1e-12,
        "eraC2teqx",
        "22",
        status,
    );
    vvd(
        rc2t[1][2],
        0.5749801116126438962e-3,
        1e-12,
        "eraC2teqx",
        "23",
        status,
    );
    vvd(
        rc2t[2][0],
        0.5773474014081539467e-3,
        1e-12,
        "eraC2teqx",
        "31",
        status,
    );
    vvd(
        rc2t[2][1],
        0.3961832391768640871e-4,
        1e-12,
        "eraC2teqx",
        "32",
        status,
    );
    vvd(
        rc2t[2][2],
        0.9999998325501691969,
        1e-12,
        "eraC2teqx",
        "33",
        status,
    );
}

fn t_c2tpe(status: &mut i32) {
    let tta = 2400000.5;
    let ttb = 53736.0;
    let uta = 2400000.5;
    let utb = 53736.0;
    let dpsi = -0.9630909107115582393e-5;
    let deps = 0.4090789763356509900;
    let xp = 2.55060238e-7;
    let yp = 1.860359247e-6;
    let mut rc2t = [[0.0; 3]; 3];

    unsafe {
        H1::eraC2tpe(
            tta,
            ttb,
            uta,
            utb,
            dpsi,
            deps,
            xp,
            yp,
            &mut rc2t as *mut _ as *mut f64,
        );
    }

    vvd(
        rc2t[0][0],
        -0.1813677995763029394,
        1e-12,
        "eraC2tpe",
        "11",
        status,
    );
    vvd(
        rc2t[0][1],
        0.9023482206891683275,
        1e-12,
        "eraC2tpe",
        "12",
        status,
    );
    vvd(
        rc2t[0][2],
        -0.3909902938641085751,
        1e-12,
        "eraC2tpe",
        "13",
        status,
    );
    vvd(
        rc2t[1][0],
        -0.9834147641476804807,
        1e-12,
        "eraC2tpe",
        "21",
        status,
    );
    vvd(
        rc2t[1][1],
        -0.1659883635434995121,
        1e-12,
        "eraC2tpe",
        "22",
        status,
    );
    vvd(
        rc2t[1][2],
        0.7309763898042819705e-1,
        1e-12,
        "eraC2tpe",
        "23",
        status,
    );
    vvd(
        rc2t[2][0],
        0.1059685430673215247e-2,
        1e-12,
        "eraC2tpe",
        "31",
        status,
    );
    vvd(
        rc2t[2][1],
        0.3977631855605078674,
        1e-12,
        "eraC2tpe",
        "32",
        status,
    );
    vvd(
        rc2t[2][2],
        0.9174875068792735362,
        1e-12,
        "eraC2tpe",
        "33",
        status,
    );
}

fn t_c2txy(status: &mut i32) {
    let tta = 2400000.5;
    let ttb = 53736.0;
    let uta = 2400000.5;
    let utb = 53736.0;
    let x = 0.5791308486706011000e-3;
    let y = 0.4020579816732961219e-4;
    let xp = 2.55060238e-7;
    let yp = 1.860359247e-6;
    let mut rc2t = [[0.0; 3]; 3];

    unsafe {
        H1::eraC2txy(
            tta,
            ttb,
            uta,
            utb,
            x,
            y,
            xp,
            yp,
            &mut rc2t as *mut _ as *mut f64,
        );
    }

    vvd(
        rc2t[0][0],
        -0.1810332128306279253,
        1e-12,
        "eraC2txy",
        "11",
        status,
    );
    vvd(
        rc2t[0][1],
        0.9834769806938520084,
        1e-12,
        "eraC2txy",
        "12",
        status,
    );
    vvd(
        rc2t[0][2],
        0.6555551248057665829e-4,
        1e-12,
        "eraC2txy",
        "13",
        status,
    );
    vvd(
        rc2t[1][0],
        -0.9834768134136142314,
        1e-12,
        "eraC2txy",
        "21",
        status,
    );
    vvd(
        rc2t[1][1],
        -0.1810332203649529312,
        1e-12,
        "eraC2txy",
        "22",
        status,
    );
    vvd(
        rc2t[1][2],
        0.5749800843594139912e-3,
        1e-12,
        "eraC2txy",
        "23",
        status,
    );
    vvd(
        rc2t[2][0],
        0.5773474028619264494e-3,
        1e-12,
        "eraC2txy",
        "31",
        status,
    );
    vvd(
        rc2t[2][1],
        0.3961816546911624260e-4,
        1e-12,
        "eraC2txy",
        "32",
        status,
    );
    vvd(
        rc2t[2][2],
        0.9999998325501746670,
        1e-12,
        "eraC2txy",
        "33",
        status,
    );
}

fn t_cal2jd(status: &mut i32) {
    let mut djm0 = 0.0;
    let mut djm = 0.0;
    let j = unsafe { H1::eraCal2jd(2003, 6, 1, &mut djm0, &mut djm) };

    vvd(djm0, 2400000.5, 0.0, "eraCal2jd", "djm0", status);
    vvd(djm, 52791.0, 0.0, "eraCal2jd", "djm", status);
    viv(j, 0, "eraCal2jd", "j", status);
}

fn t_cp(status: &mut i32) {
    let p = [0.3, 1.2, -2.5];
    let mut c = [0.0; 3];

    unsafe {
        H1::eraCp(&p as *const _ as *const f64, &mut c as *mut _ as *mut f64);
    }

    vvd(c[0], 0.3, 0.0, "eraCp", "1", status);
    vvd(c[1], 1.2, 0.0, "eraCp", "2", status);
    vvd(c[2], -2.5, 0.0, "eraCp", "3", status);
}

fn t_cpv(status: &mut i32) {
    let pv = [[0.3, 1.2, -2.5], [-0.5, 3.1, 0.9]];
    let mut c = [[0.0; 3]; 2];

    unsafe {
        H1::eraCpv(&pv as *const _ as *const f64, &mut c as *mut _ as *mut f64);
    }

    vvd(c[0][0], 0.3, 0.0, "eraCpv", "p1", status);
    vvd(c[0][1], 1.2, 0.0, "eraCpv", "p2", status);
    vvd(c[0][2], -2.5, 0.0, "eraCpv", "p3", status);
    vvd(c[1][0], -0.5, 0.0, "eraCpv", "v1", status);
    vvd(c[1][1], 3.1, 0.0, "eraCpv", "v2", status);
    vvd(c[1][2], 0.9, 0.0, "eraCpv", "v3", status);
}

fn t_cr(status: &mut i32) {
    let r = [[2.0, 3.0, 2.0], [3.0, 2.0, 3.0], [3.0, 4.0, 5.0]];
    let mut c = [[0.0; 3]; 3];

    unsafe {
        H1::eraCr(&r as *const _ as *const f64, &mut c as *mut _ as *mut f64);
    }

    vvd(c[0][0], 2.0, 0.0, "eraCr", "11", status);
    vvd(c[0][1], 3.0, 0.0, "eraCr", "12", status);
    vvd(c[0][2], 2.0, 0.0, "eraCr", "13", status);
    vvd(c[1][0], 3.0, 0.0, "eraCr", "21", status);
    vvd(c[1][1], 2.0, 0.0, "eraCr", "22", status);
    vvd(c[1][2], 3.0, 0.0, "eraCr", "23", status);
    vvd(c[2][0], 3.0, 0.0, "eraCr", "31", status);
    vvd(c[2][1], 4.0, 0.0, "eraCr", "32", status);
    vvd(c[2][2], 5.0, 0.0, "eraCr", "33", status);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        unsafe {
            VERBOSE = true;
        }
    }

    let mut status = 0;

    t_c2tcio(&mut status);
    t_c2teqx(&mut status);
    t_c2tpe(&mut status);
    t_c2txy(&mut status);
    t_cal2jd(&mut status);
    t_cp(&mut status);
    t_cpv(&mut status);
    t_cr(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
