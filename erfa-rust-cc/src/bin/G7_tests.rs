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

fn t_c2i00a(status: &mut i32) {
    let mut rc2i = [[0.0; 3]; 3];

    unsafe {
        H1::eraC2i00a(
            2400000.5,
            53736.0,
            &mut rc2i as *mut [[f64; 3]; 3] as *mut f64,
        );
    }

    vvd(
        rc2i[0][0],
        0.9999998323037165557,
        1e-12,
        "eraC2i00a",
        "11",
        status,
    );
    vvd(
        rc2i[0][1],
        0.5581526348992140183e-9,
        1e-12,
        "eraC2i00a",
        "12",
        status,
    );
    vvd(
        rc2i[0][2],
        -0.5791308477073443415e-3,
        1e-12,
        "eraC2i00a",
        "13",
        status,
    );
    vvd(
        rc2i[1][0],
        -0.2384266227870752452e-7,
        1e-12,
        "eraC2i00a",
        "21",
        status,
    );
    vvd(
        rc2i[1][1],
        0.9999999991917405258,
        1e-12,
        "eraC2i00a",
        "22",
        status,
    );
    vvd(
        rc2i[1][2],
        -0.4020594955028209745e-4,
        1e-12,
        "eraC2i00a",
        "23",
        status,
    );
    vvd(
        rc2i[2][0],
        0.5791308472168152904e-3,
        1e-12,
        "eraC2i00a",
        "31",
        status,
    );
    vvd(
        rc2i[2][1],
        0.4020595661591500259e-4,
        1e-12,
        "eraC2i00a",
        "32",
        status,
    );
    vvd(
        rc2i[2][2],
        0.9999998314954572304,
        1e-12,
        "eraC2i00a",
        "33",
        status,
    );
}

fn t_c2i00b(status: &mut i32) {
    let mut rc2i = [[0.0; 3]; 3];

    unsafe {
        H1::eraC2i00b(
            2400000.5,
            53736.0,
            &mut rc2i as *mut [[f64; 3]; 3] as *mut f64,
        );
    }

    vvd(
        rc2i[0][0],
        0.9999998323040954356,
        1e-12,
        "eraC2i00b",
        "11",
        status,
    );
    vvd(
        rc2i[0][1],
        0.5581526349131823372e-9,
        1e-12,
        "eraC2i00b",
        "12",
        status,
    );
    vvd(
        rc2i[0][2],
        -0.5791301934855394005e-3,
        1e-12,
        "eraC2i00b",
        "13",
        status,
    );
    vvd(
        rc2i[1][0],
        -0.2384239285499175543e-7,
        1e-12,
        "eraC2i00b",
        "21",
        status,
    );
    vvd(
        rc2i[1][1],
        0.9999999991917574043,
        1e-12,
        "eraC2i00b",
        "22",
        status,
    );
    vvd(
        rc2i[1][2],
        -0.4020552974819030066e-4,
        1e-12,
        "eraC2i00b",
        "23",
        status,
    );
    vvd(
        rc2i[2][0],
        0.5791301929950208873e-3,
        1e-12,
        "eraC2i00b",
        "31",
        status,
    );
    vvd(
        rc2i[2][1],
        0.4020553681373720832e-4,
        1e-12,
        "eraC2i00b",
        "32",
        status,
    );
    vvd(
        rc2i[2][2],
        0.9999998314958529887,
        1e-12,
        "eraC2i00b",
        "33",
        status,
    );
}

fn t_c2i06a(status: &mut i32) {
    let mut rc2i = [[0.0; 3]; 3];

    unsafe {
        H1::eraC2i06a(
            2400000.5,
            53736.0,
            &mut rc2i as *mut [[f64; 3]; 3] as *mut f64,
        );
    }

    vvd(
        rc2i[0][0],
        0.9999998323037159379,
        1e-12,
        "eraC2i06a",
        "11",
        status,
    );
    vvd(
        rc2i[0][1],
        0.5581121329587613787e-9,
        1e-12,
        "eraC2i06a",
        "12",
        status,
    );
    vvd(
        rc2i[0][2],
        -0.5791308487740529749e-3,
        1e-12,
        "eraC2i06a",
        "13",
        status,
    );
    vvd(
        rc2i[1][0],
        -0.2384253169452306581e-7,
        1e-12,
        "eraC2i06a",
        "21",
        status,
    );
    vvd(
        rc2i[1][1],
        0.9999999991917467827,
        1e-12,
        "eraC2i06a",
        "22",
        status,
    );
    vvd(
        rc2i[1][2],
        -0.4020579392895682558e-4,
        1e-12,
        "eraC2i06a",
        "23",
        status,
    );
    vvd(
        rc2i[2][0],
        0.5791308482835292617e-3,
        1e-12,
        "eraC2i06a",
        "31",
        status,
    );
    vvd(
        rc2i[2][1],
        0.4020580099454020310e-4,
        1e-12,
        "eraC2i06a",
        "32",
        status,
    );
    vvd(
        rc2i[2][2],
        0.9999998314954628695,
        1e-12,
        "eraC2i06a",
        "33",
        status,
    );
}

fn t_c2ibpn(status: &mut i32) {
    let mut rbpn = [
        [
            9.999962358680738e-1,
            -2.516417057665452e-3,
            -1.093569785342370e-3,
        ],
        [
            2.516462370370876e-3,
            9.999968329010883e-1,
            4.006159587358310e-5,
        ],
        [
            1.093465510215479e-3,
            -4.281337229063151e-5,
            9.999994012499173e-1,
        ],
    ];
    let mut rc2i = [[0.0; 3]; 3];

    unsafe {
        H1::eraC2ibpn(
            2400000.5,
            50123.9999,
            &mut rbpn as *mut [[f64; 3]; 3] as *mut f64,
            &mut rc2i as *mut [[f64; 3]; 3] as *mut f64,
        );
    }

    vvd(
        rc2i[0][0],
        0.9999994021664089977,
        1e-12,
        "eraC2ibpn",
        "11",
        status,
    );
    vvd(
        rc2i[0][1],
        -0.3869195948017503664e-8,
        1e-12,
        "eraC2ibpn",
        "12",
        status,
    );
    vvd(
        rc2i[0][2],
        -0.1093465511383285076e-2,
        1e-12,
        "eraC2ibpn",
        "13",
        status,
    );
    vvd(
        rc2i[1][0],
        0.5068413965715446111e-7,
        1e-12,
        "eraC2ibpn",
        "21",
        status,
    );
    vvd(
        rc2i[1][1],
        0.9999999990835075686,
        1e-12,
        "eraC2ibpn",
        "22",
        status,
    );
    vvd(
        rc2i[1][2],
        0.4281334246452708915e-4,
        1e-12,
        "eraC2ibpn",
        "23",
        status,
    );
    vvd(
        rc2i[2][0],
        0.1093465510215479000e-2,
        1e-12,
        "eraC2ibpn",
        "31",
        status,
    );
    vvd(
        rc2i[2][1],
        -0.4281337229063151000e-4,
        1e-12,
        "eraC2ibpn",
        "32",
        status,
    );
    vvd(
        rc2i[2][2],
        0.9999994012499173103,
        1e-12,
        "eraC2ibpn",
        "33",
        status,
    );
}

fn t_c2ixy(status: &mut i32) {
    let x = 0.5791308486706011000e-3;
    let y = 0.4020579816732961219e-4;
    let mut rc2i = [[0.0; 3]; 3];

    unsafe {
        H1::eraC2ixy(
            2400000.5,
            53736.0,
            x,
            y,
            &mut rc2i as *mut [[f64; 3]; 3] as *mut f64,
        );
    }

    vvd(
        rc2i[0][0],
        0.9999998323037157138,
        1e-12,
        "eraC2ixy",
        "11",
        status,
    );
    vvd(
        rc2i[0][1],
        0.5581526349032241205e-9,
        1e-12,
        "eraC2ixy",
        "12",
        status,
    );
    vvd(
        rc2i[0][2],
        -0.5791308491611263745e-3,
        1e-12,
        "eraC2ixy",
        "13",
        status,
    );
    vvd(
        rc2i[1][0],
        -0.2384257057469842953e-7,
        1e-12,
        "eraC2ixy",
        "21",
        status,
    );
    vvd(
        rc2i[1][1],
        0.9999999991917468964,
        1e-12,
        "eraC2ixy",
        "22",
        status,
    );
    vvd(
        rc2i[1][2],
        -0.4020579110172324363e-4,
        1e-12,
        "eraC2ixy",
        "23",
        status,
    );
    vvd(
        rc2i[2][0],
        0.5791308486706011000e-3,
        1e-12,
        "eraC2ixy",
        "31",
        status,
    );
    vvd(
        rc2i[2][1],
        0.4020579816732961219e-4,
        1e-12,
        "eraC2ixy",
        "32",
        status,
    );
    vvd(
        rc2i[2][2],
        0.9999998314954627590,
        1e-12,
        "eraC2ixy",
        "33",
        status,
    );
}

fn t_c2ixys(status: &mut i32) {
    let x = 0.5791308486706011000e-3;
    let y = 0.4020579816732961219e-4;
    let s = -0.1220040848472271978e-7;
    let mut rc2i = [[0.0; 3]; 3];

    unsafe {
        H1::eraC2ixys(x, y, s, &mut rc2i as *mut [[f64; 3]; 3] as *mut f64);
    }

    vvd(
        rc2i[0][0],
        0.9999998323037157138,
        1e-12,
        "eraC2ixys",
        "11",
        status,
    );
    vvd(
        rc2i[0][1],
        0.5581984869168499149e-9,
        1e-12,
        "eraC2ixys",
        "12",
        status,
    );
    vvd(
        rc2i[0][2],
        -0.5791308491611282180e-3,
        1e-12,
        "eraC2ixys",
        "13",
        status,
    );
    vvd(
        rc2i[1][0],
        -0.2384261642670440317e-7,
        1e-12,
        "eraC2ixys",
        "21",
        status,
    );
    vvd(
        rc2i[1][1],
        0.9999999991917468964,
        1e-12,
        "eraC2ixys",
        "22",
        status,
    );
    vvd(
        rc2i[1][2],
        -0.4020579110169668931e-4,
        1e-12,
        "eraC2ixys",
        "23",
        status,
    );
    vvd(
        rc2i[2][0],
        0.5791308486706011000e-3,
        1e-12,
        "eraC2ixys",
        "31",
        status,
    );
    vvd(
        rc2i[2][1],
        0.4020579816732961219e-4,
        1e-12,
        "eraC2ixys",
        "32",
        status,
    );
    vvd(
        rc2i[2][2],
        0.9999998314954627590,
        1e-12,
        "eraC2ixys",
        "33",
        status,
    );
}

fn t_c2s(status: &mut i32) {
    let mut p = [100.0, -50.0, 25.0];
    let mut theta = 0.0;
    let mut phi = 0.0;

    unsafe {
        H1::eraC2s(&mut p as *mut [f64; 3] as *mut f64, &mut theta, &mut phi);
    }

    vvd(
        theta,
        -0.4636476090008061162,
        1e-14,
        "eraC2s",
        "theta",
        status,
    );
    vvd(phi, 0.2199879773954594463, 1e-14, "eraC2s", "phi", status);
}

fn t_c2t00a(status: &mut i32) {
    let tta = 2400000.5;
    let ttb = 53736.0;
    let uta = 2400000.5;
    let utb = 53736.0;
    let xp = 2.55060238e-7;
    let yp = 1.860359247e-6;
    let mut rc2t = [[0.0; 3]; 3];

    unsafe {
        H1::eraC2t00a(
            tta,
            ttb,
            uta,
            utb,
            xp,
            yp,
            &mut rc2t as *mut [[f64; 3]; 3] as *mut f64,
        );
    }

    vvd(
        rc2t[0][0],
        -0.1810332128307182668,
        1e-12,
        "eraC2t00a",
        "11",
        status,
    );
    vvd(
        rc2t[0][1],
        0.9834769806938457836,
        1e-12,
        "eraC2t00a",
        "12",
        status,
    );
    vvd(
        rc2t[0][2],
        0.6555535638688341725e-4,
        1e-12,
        "eraC2t00a",
        "13",
        status,
    );
    vvd(
        rc2t[1][0],
        -0.9834768134135984552,
        1e-12,
        "eraC2t00a",
        "21",
        status,
    );
    vvd(
        rc2t[1][1],
        -0.1810332203649520727,
        1e-12,
        "eraC2t00a",
        "22",
        status,
    );
    vvd(
        rc2t[1][2],
        0.5749801116141056317e-3,
        1e-12,
        "eraC2t00a",
        "23",
        status,
    );
    vvd(
        rc2t[2][0],
        0.5773474014081406921e-3,
        1e-12,
        "eraC2t00a",
        "31",
        status,
    );
    vvd(
        rc2t[2][1],
        0.3961832391770163647e-4,
        1e-12,
        "eraC2t00a",
        "32",
        status,
    );
    vvd(
        rc2t[2][2],
        0.9999998325501692289,
        1e-12,
        "eraC2t00a",
        "33",
        status,
    );
}

fn t_c2t00b(status: &mut i32) {
    let tta = 2400000.5;
    let ttb = 53736.0;
    let uta = 2400000.5;
    let utb = 53736.0;
    let xp = 2.55060238e-7;
    let yp = 1.860359247e-6;
    let mut rc2t = [[0.0; 3]; 3];

    unsafe {
        H1::eraC2t00b(
            tta,
            ttb,
            uta,
            utb,
            xp,
            yp,
            &mut rc2t as *mut [[f64; 3]; 3] as *mut f64,
        );
    }

    vvd(
        rc2t[0][0],
        -0.1810332128439678965,
        1e-12,
        "eraC2t00b",
        "11",
        status,
    );
    vvd(
        rc2t[0][1],
        0.9834769806913872359,
        1e-12,
        "eraC2t00b",
        "12",
        status,
    );
    vvd(
        rc2t[0][2],
        0.6555565082458415611e-4,
        1e-12,
        "eraC2t00b",
        "13",
        status,
    );
    vvd(
        rc2t[1][0],
        -0.9834768134115435923,
        1e-12,
        "eraC2t00b",
        "21",
        status,
    );
    vvd(
        rc2t[1][1],
        -0.1810332203784001946,
        1e-12,
        "eraC2t00b",
        "22",
        status,
    );
    vvd(
        rc2t[1][2],
        0.5749793922030017230e-3,
        1e-12,
        "eraC2t00b",
        "23",
        status,
    );
    vvd(
        rc2t[2][0],
        0.5773467471863534901e-3,
        1e-12,
        "eraC2t00b",
        "31",
        status,
    );
    vvd(
        rc2t[2][1],
        0.3961790411549945020e-4,
        1e-12,
        "eraC2t00b",
        "32",
        status,
    );
    vvd(
        rc2t[2][2],
        0.9999998325505635738,
        1e-12,
        "eraC2t00b",
        "33",
        status,
    );
}

fn t_c2t06a(status: &mut i32) {
    let tta = 2400000.5;
    let ttb = 53736.0;
    let uta = 2400000.5;
    let utb = 53736.0;
    let xp = 2.55060238e-7;
    let yp = 1.860359247e-6;
    let mut rc2t = [[0.0; 3]; 3];

    unsafe {
        H1::eraC2t06a(
            tta,
            ttb,
            uta,
            utb,
            xp,
            yp,
            &mut rc2t as *mut [[f64; 3]; 3] as *mut f64,
        );
    }

    vvd(
        rc2t[0][0],
        -0.1810332128305897282,
        1e-12,
        "eraC2t06a",
        "11",
        status,
    );
    vvd(
        rc2t[0][1],
        0.9834769806938592296,
        1e-12,
        "eraC2t06a",
        "12",
        status,
    );
    vvd(
        rc2t[0][2],
        0.6555550962998436505e-4,
        1e-12,
        "eraC2t06a",
        "13",
        status,
    );
    vvd(
        rc2t[1][0],
        -0.9834768134136214897,
        1e-12,
        "eraC2t06a",
        "21",
        status,
    );
    vvd(
        rc2t[1][1],
        -0.1810332203649130832,
        1e-12,
        "eraC2t06a",
        "22",
        status,
    );
    vvd(
        rc2t[1][2],
        0.5749800844905594110e-3,
        1e-12,
        "eraC2t06a",
        "23",
        status,
    );
    vvd(
        rc2t[2][0],
        0.5773474024748545878e-3,
        1e-12,
        "eraC2t06a",
        "31",
        status,
    );
    vvd(
        rc2t[2][1],
        0.3961816829632690581e-4,
        1e-12,
        "eraC2t06a",
        "32",
        status,
    );
    vvd(
        rc2t[2][2],
        0.9999998325501747785,
        1e-12,
        "eraC2t06a",
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

    t_c2i00a(&mut status);
    t_c2i00b(&mut status);
    t_c2i06a(&mut status);
    t_c2ibpn(&mut status);
    t_c2ixy(&mut status);
    t_c2ixys(&mut status);
    t_c2s(&mut status);
    t_c2t00a(&mut status);
    t_c2t00b(&mut status);
    t_c2t06a(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
