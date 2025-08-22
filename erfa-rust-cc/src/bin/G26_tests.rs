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

fn t_pn06(status: &mut i32) {
    let mut epsa: f64 = 0.0;
    let mut rb = [[0.0; 3]; 3];
    let mut rp = [[0.0; 3]; 3];
    let mut rbp = [[0.0; 3]; 3];
    let mut rn = [[0.0; 3]; 3];
    let mut rbpn = [[0.0; 3]; 3];

    let dpsi = -0.9632552291149335877e-5;
    let deps = 0.4063197106621141414e-4;

    unsafe {
        H1::eraPn06(
            2400000.5,
            53736.0,
            dpsi,
            deps,
            &mut epsa as *mut f64,
            &mut rb as *mut [[f64; 3]; 3] as *mut f64,
            &mut rp as *mut [[f64; 3]; 3] as *mut f64,
            &mut rbp as *mut [[f64; 3]; 3] as *mut f64,
            &mut rn as *mut [[f64; 3]; 3] as *mut f64,
            &mut rbpn as *mut [[f64; 3]; 3] as *mut f64,
        );
    }

    vvd(
        epsa,
        0.4090789763356509926,
        1e-12,
        "eraPn06",
        "epsa",
        status,
    );
    vvd(
        rb[0][0],
        0.9999999999999942497,
        1e-12,
        "eraPn06",
        "rb11",
        status,
    );
    vvd(
        rb[0][1],
        -0.7078368960971557145e-7,
        1e-14,
        "eraPn06",
        "rb12",
        status,
    );
    vvd(
        rb[0][2],
        0.8056213977613185606e-7,
        1e-14,
        "eraPn06",
        "rb13",
        status,
    );
    vvd(
        rb[1][0],
        0.7078368694637674333e-7,
        1e-14,
        "eraPn06",
        "rb21",
        status,
    );
    vvd(
        rb[1][1],
        0.9999999999999969484,
        1e-12,
        "eraPn06",
        "rb22",
        status,
    );
    vvd(
        rb[1][2],
        0.3305943742989134124e-7,
        1e-14,
        "eraPn06",
        "rb23",
        status,
    );
    vvd(
        rb[2][0],
        -0.8056214211620056792e-7,
        1e-14,
        "eraPn06",
        "rb31",
        status,
    );
    vvd(
        rb[2][1],
        -0.3305943172740586950e-7,
        1e-14,
        "eraPn06",
        "rb32",
        status,
    );
    vvd(
        rb[2][2],
        0.9999999999999962084,
        1e-12,
        "eraPn06",
        "rb33",
        status,
    );
    vvd(
        rp[0][0],
        0.9999989300536854831,
        1e-12,
        "eraPn06",
        "rp11",
        status,
    );
    vvd(
        rp[0][1],
        -0.1341646886204443795e-2,
        1e-14,
        "eraPn06",
        "rp12",
        status,
    );
    vvd(
        rp[0][2],
        -0.5829880933488627759e-3,
        1e-14,
        "eraPn06",
        "rp13",
        status,
    );
    vvd(
        rp[1][0],
        0.1341646890569782183e-2,
        1e-14,
        "eraPn06",
        "rp21",
        status,
    );
    vvd(
        rp[1][1],
        0.9999990999913319321,
        1e-12,
        "eraPn06",
        "rp22",
        status,
    );
    vvd(
        rp[1][2],
        -0.3835944216374477457e-6,
        1e-14,
        "eraPn06",
        "rp23",
        status,
    );
    vvd(
        rp[2][0],
        0.5829880833027867368e-3,
        1e-14,
        "eraPn06",
        "rp31",
        status,
    );
    vvd(
        rp[2][1],
        -0.3985701514686976112e-6,
        1e-14,
        "eraPn06",
        "rp32",
        status,
    );
    vvd(
        rp[2][2],
        0.9999998300623534950,
        1e-12,
        "eraPn06",
        "rp33",
        status,
    );
    vvd(
        rbp[0][0],
        0.9999989300056797893,
        1e-12,
        "eraPn06",
        "rbp11",
        status,
    );
    vvd(
        rbp[0][1],
        -0.1341717650545059598e-2,
        1e-14,
        "eraPn06",
        "rbp12",
        status,
    );
    vvd(
        rbp[0][2],
        -0.5829075756493728856e-3,
        1e-14,
        "eraPn06",
        "rbp13",
        status,
    );
    vvd(
        rbp[1][0],
        0.1341717674223918101e-2,
        1e-14,
        "eraPn06",
        "rbp21",
        status,
    );
    vvd(
        rbp[1][1],
        0.9999990998963748448,
        1e-12,
        "eraPn06",
        "rbp22",
        status,
    );
    vvd(
        rbp[1][2],
        -0.3504269280170069029e-6,
        1e-14,
        "eraPn06",
        "rbp23",
        status,
    );
    vvd(
        rbp[2][0],
        0.5829075211461454599e-3,
        1e-14,
        "eraPn06",
        "rbp31",
        status,
    );
    vvd(
        rbp[2][1],
        -0.4316708436255949093e-6,
        1e-14,
        "eraPn06",
        "rbp32",
        status,
    );
    vvd(
        rbp[2][2],
        0.9999998301093032943,
        1e-12,
        "eraPn06",
        "rbp33",
        status,
    );
    vvd(
        rn[0][0],
        0.9999999999536069682,
        1e-12,
        "eraPn06",
        "rn11",
        status,
    );
    vvd(
        rn[0][1],
        0.8837746921149881914e-5,
        1e-14,
        "eraPn06",
        "rn12",
        status,
    );
    vvd(
        rn[0][2],
        0.3831487047682968703e-5,
        1e-14,
        "eraPn06",
        "rn13",
        status,
    );
    vvd(
        rn[1][0],
        -0.8837591232983692340e-5,
        1e-14,
        "eraPn06",
        "rn21",
        status,
    );
    vvd(
        rn[1][1],
        0.9999999991354692664,
        1e-12,
        "eraPn06",
        "rn22",
        status,
    );
    vvd(
        rn[1][2],
        -0.4063198798558931215e-4,
        1e-14,
        "eraPn06",
        "rn23",
        status,
    );
    vvd(
        rn[2][0],
        -0.3831846139597250235e-5,
        1e-14,
        "eraPn06",
        "rn31",
        status,
    );
    vvd(
        rn[2][1],
        0.4063195412258792914e-4,
        1e-14,
        "eraPn06",
        "rn32",
        status,
    );
    vvd(
        rn[2][2],
        0.9999999991671806293,
        1e-12,
        "eraPn06",
        "rn33",
        status,
    );
    vvd(
        rbpn[0][0],
        0.9999989440504506688,
        1e-12,
        "eraPn06",
        "rbpn11",
        status,
    );
    vvd(
        rbpn[0][1],
        -0.1332879913170492655e-2,
        1e-14,
        "eraPn06",
        "rbpn12",
        status,
    );
    vvd(
        rbpn[0][2],
        -0.5790760923225655753e-3,
        1e-14,
        "eraPn06",
        "rbpn13",
        status,
    );
    vvd(
        rbpn[1][0],
        0.1332856406595754748e-2,
        1e-14,
        "eraPn06",
        "rbpn21",
        status,
    );
    vvd(
        rbpn[1][1],
        0.9999991109069366795,
        1e-12,
        "eraPn06",
        "rbpn22",
        status,
    );
    vvd(
        rbpn[1][2],
        -0.4097725651142641812e-4,
        1e-14,
        "eraPn06",
        "rbpn23",
        status,
    );
    vvd(
        rbpn[2][0],
        0.5791301952321296716e-3,
        1e-14,
        "eraPn06",
        "rbpn31",
        status,
    );
    vvd(
        rbpn[2][1],
        0.4020538796195230577e-4,
        1e-14,
        "eraPn06",
        "rbpn32",
        status,
    );
    vvd(
        rbpn[2][2],
        0.9999998314958576778,
        1e-12,
        "eraPn06",
        "rbpn33",
        status,
    );
}

fn t_pn06a(status: &mut i32) {
    let mut dpsi: f64 = 0.0;
    let mut deps: f64 = 0.0;
    let mut epsa: f64 = 0.0;
    let mut rb = [[0.0; 3]; 3];
    let mut rp = [[0.0; 3]; 3];
    let mut rbp = [[0.0; 3]; 3];
    let mut rn = [[0.0; 3]; 3];
    let mut rbpn = [[0.0; 3]; 3];

    unsafe {
        H1::eraPn06a(
            2400000.5,
            53736.0,
            &mut dpsi as *mut f64,
            &mut deps as *mut f64,
            &mut epsa as *mut f64,
            &mut rb as *mut [[f64; 3]; 3] as *mut f64,
            &mut rp as *mut [[f64; 3]; 3] as *mut f64,
            &mut rbp as *mut [[f64; 3]; 3] as *mut f64,
            &mut rn as *mut [[f64; 3]; 3] as *mut f64,
            &mut rbpn as *mut [[f64; 3]; 3] as *mut f64,
        );
    }

    vvd(
        dpsi,
        -0.9630912025820308797e-5,
        1e-12,
        "eraPn06a",
        "dpsi",
        status,
    );
    vvd(
        deps,
        0.4063238496887249798e-4,
        1e-12,
        "eraPn06a",
        "deps",
        status,
    );
    vvd(
        epsa,
        0.4090789763356509926,
        1e-12,
        "eraPn06a",
        "epsa",
        status,
    );
    vvd(
        rb[0][0],
        0.9999999999999942497,
        1e-12,
        "eraPn06a",
        "rb11",
        status,
    );
    vvd(
        rb[0][1],
        -0.7078368960971557145e-7,
        1e-14,
        "eraPn06a",
        "rb12",
        status,
    );
    vvd(
        rb[0][2],
        0.8056213977613185606e-7,
        1e-14,
        "eraPn06a",
        "rb13",
        status,
    );
    vvd(
        rb[1][0],
        0.7078368694637674333e-7,
        1e-14,
        "eraPn06a",
        "rb21",
        status,
    );
    vvd(
        rb[1][1],
        0.9999999999999969484,
        1e-12,
        "eraPn06a",
        "rb22",
        status,
    );
    vvd(
        rb[1][2],
        0.3305943742989134124e-7,
        1e-14,
        "eraPn06a",
        "rb23",
        status,
    );
    vvd(
        rb[2][0],
        -0.8056214211620056792e-7,
        1e-14,
        "eraPn06a",
        "rb31",
        status,
    );
    vvd(
        rb[2][1],
        -0.3305943172740586950e-7,
        1e-14,
        "eraPn06a",
        "rb32",
        status,
    );
    vvd(
        rb[2][2],
        0.9999999999999962084,
        1e-12,
        "eraPn06a",
        "rb33",
        status,
    );
    vvd(
        rp[0][0],
        0.9999989300536854831,
        1e-12,
        "eraPn06a",
        "rp11",
        status,
    );
    vvd(
        rp[0][1],
        -0.1341646886204443795e-2,
        1e-14,
        "eraPn06a",
        "rp12",
        status,
    );
    vvd(
        rp[0][2],
        -0.5829880933488627759e-3,
        1e-14,
        "eraPn06a",
        "rp13",
        status,
    );
    vvd(
        rp[1][0],
        0.1341646890569782183e-2,
        1e-14,
        "eraPn06a",
        "rp21",
        status,
    );
    vvd(
        rp[1][1],
        0.9999990999913319321,
        1e-12,
        "eraPn06a",
        "rp22",
        status,
    );
    vvd(
        rp[1][2],
        -0.3835944216374477457e-6,
        1e-14,
        "eraPn06a",
        "rp23",
        status,
    );
    vvd(
        rp[2][0],
        0.5829880833027867368e-3,
        1e-14,
        "eraPn06a",
        "rp31",
        status,
    );
    vvd(
        rp[2][1],
        -0.3985701514686976112e-6,
        1e-14,
        "eraPn06a",
        "rp32",
        status,
    );
    vvd(
        rp[2][2],
        0.9999998300623534950,
        1e-12,
        "eraPn06a",
        "rp33",
        status,
    );
    vvd(
        rbp[0][0],
        0.9999989300056797893,
        1e-12,
        "eraPn06a",
        "rbp11",
        status,
    );
    vvd(
        rbp[0][1],
        -0.1341717650545059598e-2,
        1e-14,
        "eraPn06a",
        "rbp12",
        status,
    );
    vvd(
        rbp[0][2],
        -0.5829075756493728856e-3,
        1e-14,
        "eraPn06a",
        "rbp13",
        status,
    );
    vvd(
        rbp[1][0],
        0.1341717674223918101e-2,
        1e-14,
        "eraPn06a",
        "rbp21",
        status,
    );
    vvd(
        rbp[1][1],
        0.9999990998963748448,
        1e-12,
        "eraPn06a",
        "rbp22",
        status,
    );
    vvd(
        rbp[1][2],
        -0.3504269280170069029e-6,
        1e-14,
        "eraPn06a",
        "rbp23",
        status,
    );
    vvd(
        rbp[2][0],
        0.5829075211461454599e-3,
        1e-14,
        "eraPn06a",
        "rbp31",
        status,
    );
    vvd(
        rbp[2][1],
        -0.4316708436255949093e-6,
        1e-14,
        "eraPn06a",
        "rbp32",
        status,
    );
    vvd(
        rbp[2][2],
        0.9999998301093032943,
        1e-12,
        "eraPn06a",
        "rbp33",
        status,
    );
    vvd(
        rn[0][0],
        0.9999999999536227668,
        1e-12,
        "eraPn06a",
        "rn11",
        status,
    );
    vvd(
        rn[0][1],
        0.8836241998111535233e-5,
        1e-14,
        "eraPn06a",
        "rn12",
        status,
    );
    vvd(
        rn[0][2],
        0.3830834608415287707e-5,
        1e-14,
        "eraPn06a",
        "rn13",
        status,
    );
    vvd(
        rn[1][0],
        -0.8836086334870740138e-5,
        1e-14,
        "eraPn06a",
        "rn21",
        status,
    );
    vvd(
        rn[1][1],
        0.9999999991354657474,
        1e-12,
        "eraPn06a",
        "rn22",
        status,
    );
    vvd(
        rn[1][2],
        -0.4063240188248455065e-4,
        1e-14,
        "eraPn06a",
        "rn23",
        status,
    );
    vvd(
        rn[2][0],
        -0.3831193642839398128e-5,
        1e-14,
        "eraPn06a",
        "rn31",
        status,
    );
    vvd(
        rn[2][1],
        0.4063236803101479770e-4,
        1e-14,
        "eraPn06a",
        "rn32",
        status,
    );
    vvd(
        rn[2][2],
        0.9999999991671663114,
        1e-12,
        "eraPn06a",
        "rn33",
        status,
    );
    vvd(
        rbpn[0][0],
        0.9999989440480669738,
        1e-12,
        "eraPn06a",
        "rbpn11",
        status,
    );
    vvd(
        rbpn[0][1],
        -0.1332881418091915973e-2,
        1e-14,
        "eraPn06a",
        "rbpn12",
        status,
    );
    vvd(
        rbpn[0][2],
        -0.5790767447612042565e-3,
        1e-14,
        "eraPn06a",
        "rbpn13",
        status,
    );
    vvd(
        rbpn[1][0],
        0.1332857911250989133e-2,
        1e-14,
        "eraPn06a",
        "rbpn21",
        status,
    );
    vvd(
        rbpn[1][1],
        0.9999991109049141908,
        1e-12,
        "eraPn06a",
        "rbpn22",
        status,
    );
    vvd(
        rbpn[1][2],
        -0.4097767128546784878e-4,
        1e-14,
        "eraPn06a",
        "rbpn23",
        status,
    );
    vvd(
        rbpn[2][0],
        0.5791308482835292617e-3,
        1e-14,
        "eraPn06a",
        "rbpn31",
        status,
    );
    vvd(
        rbpn[2][1],
        0.4020580099454020310e-4,
        1e-14,
        "eraPn06a",
        "rbpn32",
        status,
    );
    vvd(
        rbpn[2][2],
        0.9999998314954628695,
        1e-12,
        "eraPn06a",
        "rbpn33",
        status,
    );
}

fn t_pnm00a(status: &mut i32) {
    let mut rbpn = [[0.0; 3]; 3];

    unsafe {
        H1::eraPnm00a(
            2400000.5,
            50123.9999,
            &mut rbpn as *mut [[f64; 3]; 3] as *mut f64,
        );
    }

    vvd(
        rbpn[0][0],
        0.9999995832793134257,
        1e-12,
        "eraPnm00a",
        "11",
        status,
    );
    vvd(
        rbpn[0][1],
        0.8372384254137809439e-3,
        1e-14,
        "eraPnm00a",
        "12",
        status,
    );
    vvd(
        rbpn[0][2],
        0.3639684306407150645e-3,
        1e-14,
        "eraPnm00a",
        "13",
        status,
    );
    vvd(
        rbpn[1][0],
        -0.8372535226570394543e-3,
        1e-14,
        "eraPnm00a",
        "21",
        status,
    );
    vvd(
        rbpn[1][1],
        0.9999996486491582471,
        1e-12,
        "eraPnm00a",
        "22",
        status,
    );
    vvd(
        rbpn[1][2],
        0.4132915262664072381e-4,
        1e-14,
        "eraPnm00a",
        "23",
        status,
    );
    vvd(
        rbpn[2][0],
        -0.3639337004054317729e-3,
        1e-14,
        "eraPnm00a",
        "31",
        status,
    );
    vvd(
        rbpn[2][1],
        -0.4163386925461775873e-4,
        1e-14,
        "eraPnm00a",
        "32",
        status,
    );
    vvd(
        rbpn[2][2],
        0.9999999329094390695,
        1e-12,
        "eraPnm00a",
        "33",
        status,
    );
}

fn t_pnm00b(status: &mut i32) {
    let mut rbpn = [[0.0; 3]; 3];

    unsafe {
        H1::eraPnm00b(
            2400000.5,
            50123.9999,
            &mut rbpn as *mut [[f64; 3]; 3] as *mut f64,
        );
    }

    vvd(
        rbpn[0][0],
        0.9999995832776208280,
        1e-12,
        "eraPnm00b",
        "11",
        status,
    );
    vvd(
        rbpn[0][1],
        0.8372401264429654837e-3,
        1e-14,
        "eraPnm00b",
        "12",
        status,
    );
    vvd(
        rbpn[0][2],
        0.3639691681450271771e-3,
        1e-14,
        "eraPnm00b",
        "13",
        status,
    );
    vvd(
        rbpn[1][0],
        -0.8372552234147137424e-3,
        1e-14,
        "eraPnm00b",
        "21",
        status,
    );
    vvd(
        rbpn[1][1],
        0.9999996486477686123,
        1e-12,
        "eraPnm00b",
        "22",
        status,
    );
    vvd(
        rbpn[1][2],
        0.4132832190946052890e-4,
        1e-14,
        "eraPnm00b",
        "23",
        status,
    );
    vvd(
        rbpn[2][0],
        -0.3639344385341866407e-3,
        1e-14,
        "eraPnm00b",
        "31",
        status,
    );
    vvd(
        rbpn[2][1],
        -0.4163303977421522785e-4,
        1e-14,
        "eraPnm00b",
        "32",
        status,
    );
    vvd(
        rbpn[2][2],
        0.9999999329092049734,
        1e-12,
        "eraPnm00b",
        "33",
        status,
    );
}

fn t_pnm06a(status: &mut i32) {
    let mut rbpn = [[0.0; 3]; 3];

    unsafe {
        H1::eraPnm06a(
            2400000.5,
            50123.9999,
            &mut rbpn as *mut [[f64; 3]; 3] as *mut f64,
        );
    }

    vvd(
        rbpn[0][0],
        0.9999995832794205484,
        1e-12,
        "eraPnm06a",
        "11",
        status,
    );
    vvd(
        rbpn[0][1],
        0.8372382772630962111e-3,
        1e-14,
        "eraPnm06a",
        "12",
        status,
    );
    vvd(
        rbpn[0][2],
        0.3639684771140623099e-3,
        1e-14,
        "eraPnm06a",
        "13",
        status,
    );
    vvd(
        rbpn[1][0],
        -0.8372533744743683605e-3,
        1e-14,
        "eraPnm06a",
        "21",
        status,
    );
    vvd(
        rbpn[1][1],
        0.9999996486492861646,
        1e-12,
        "eraPnm06a",
        "22",
        status,
    );
    vvd(
        rbpn[1][2],
        0.4132905944611019498e-4,
        1e-14,
        "eraPnm06a",
        "23",
        status,
    );
    vvd(
        rbpn[2][0],
        -0.3639337469629464969e-3,
        1e-14,
        "eraPnm06a",
        "31",
        status,
    );
    vvd(
        rbpn[2][1],
        -0.4163377605910663999e-4,
        1e-14,
        "eraPnm06a",
        "32",
        status,
    );
    vvd(
        rbpn[2][2],
        0.9999999329094260057,
        1e-12,
        "eraPnm06a",
        "33",
        status,
    );
}

fn t_pnm80(status: &mut i32) {
    let mut rmatpn = [[0.0; 3]; 3];

    unsafe {
        H1::eraPnm80(
            2400000.5,
            50123.9999,
            &mut rmatpn as *mut [[f64; 3]; 3] as *mut f64,
        );
    }

    vvd(
        rmatpn[0][0],
        0.9999995831934611169,
        1e-12,
        "eraPnm80",
        "11",
        status,
    );
    vvd(
        rmatpn[0][1],
        0.8373654045728124011e-3,
        1e-14,
        "eraPnm80",
        "12",
        status,
    );
    vvd(
        rmatpn[0][2],
        0.3639121916933106191e-3,
        1e-14,
        "eraPnm80",
        "13",
        status,
    );
    vvd(
        rmatpn[1][0],
        -0.8373804896118301316e-3,
        1e-14,
        "eraPnm80",
        "21",
        status,
    );
    vvd(
        rmatpn[1][1],
        0.9999996485439674092,
        1e-12,
        "eraPnm80",
        "22",
        status,
    );
    vvd(
        rmatpn[1][2],
        0.4130202510421549752e-4,
        1e-14,
        "eraPnm80",
        "23",
        status,
    );
    vvd(
        rmatpn[2][0],
        -0.3638774789072144473e-3,
        1e-14,
        "eraPnm80",
        "31",
        status,
    );
    vvd(
        rmatpn[2][1],
        -0.4160674085851722359e-4,
        1e-14,
        "eraPnm80",
        "32",
        status,
    );
    vvd(
        rmatpn[2][2],
        0.9999999329310274805,
        1e-12,
        "eraPnm80",
        "33",
        status,
    );
}

fn t_pom00(status: &mut i32) {
    let mut rpom = [[0.0; 3]; 3];
    let xp = 2.55060238e-7;
    let yp = 1.860359247e-6;
    let sp = -0.1367174580728891460e-10;

    unsafe {
        H1::eraPom00(xp, yp, sp, &mut rpom as *mut [[f64; 3]; 3] as *mut f64);
    }

    vvd(
        rpom[0][0],
        0.9999999999999674721,
        1e-12,
        "eraPom00",
        "11",
        status,
    );
    vvd(
        rpom[0][1],
        -0.1367174580728846989e-10,
        1e-16,
        "eraPom00",
        "12",
        status,
    );
    vvd(
        rpom[0][2],
        0.2550602379999972345e-6,
        1e-16,
        "eraPom00",
        "13",
        status,
    );
    vvd(
        rpom[1][0],
        0.1414624947957029801e-10,
        1e-16,
        "eraPom00",
        "21",
        status,
    );
    vvd(
        rpom[1][1],
        0.9999999999982695317,
        1e-12,
        "eraPom00",
        "22",
        status,
    );
    vvd(
        rpom[1][2],
        -0.1860359246998866389e-5,
        1e-16,
        "eraPom00",
        "23",
        status,
    );
    vvd(
        rpom[2][0],
        -0.2550602379741215021e-6,
        1e-16,
        "eraPom00",
        "31",
        status,
    );
    vvd(
        rpom[2][1],
        0.1860359247002414021e-5,
        1e-16,
        "eraPom00",
        "32",
        status,
    );
    vvd(
        rpom[2][2],
        0.9999999999982370039,
        1e-12,
        "eraPom00",
        "33",
        status,
    );
}

fn t_ppp(status: &mut i32) {
    let mut a = [2.0, 2.0, 3.0];
    let mut b = [1.0, 3.0, 4.0];
    let mut apb = [0.0; 3];

    unsafe {
        H1::eraPpp(a.as_mut_ptr(), b.as_mut_ptr(), apb.as_mut_ptr());
    }

    vvd(apb[0], 3.0, 1e-12, "eraPpp", "0", status);
    vvd(apb[1], 5.0, 1e-12, "eraPpp", "1", status);
    vvd(apb[2], 7.0, 1e-12, "eraPpp", "2", status);
}

fn t_ppsp(status: &mut i32) {
    let mut a = [2.0, 2.0, 3.0];
    let s = 5.0;
    let mut b = [1.0, 3.0, 4.0];
    let mut apsb = [0.0; 3];

    unsafe {
        H1::eraPpsp(a.as_mut_ptr(), s, b.as_mut_ptr(), apsb.as_mut_ptr());
    }

    vvd(apsb[0], 7.0, 1e-12, "eraPpsp", "0", status);
    vvd(apsb[1], 17.0, 1e-12, "eraPpsp", "1", status);
    vvd(apsb[2], 23.0, 1e-12, "eraPpsp", "2", status);
}

fn t_pr00(status: &mut i32) {
    let mut dpsipr: f64 = 0.0;
    let mut depspr: f64 = 0.0;

    unsafe {
        H1::eraPr00(
            2400000.5,
            53736.0,
            &mut dpsipr as *mut f64,
            &mut depspr as *mut f64,
        );
    }

    vvd(
        dpsipr,
        -0.8716465172668347629e-7,
        1e-22,
        "eraPr00",
        "dpsipr",
        status,
    );
    vvd(
        depspr,
        -0.7342018386722813087e-8,
        1e-22,
        "eraPr00",
        "depspr",
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

    t_pn06a(&mut status);
    t_pn06(&mut status);
    t_pnm00a(&mut status);
    t_pnm00b(&mut status);
    t_pnm06a(&mut status);
    t_pnm80(&mut status);
    t_pom00(&mut status);
    t_ppp(&mut status);
    t_ppsp(&mut status);
    t_pr00(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
