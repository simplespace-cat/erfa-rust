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

fn t_pmat00(status: &mut i32) {
    let mut rbp = [[0.0; 3]; 3];

    unsafe {
        H1::eraPmat00(
            2400000.5,
            50123.9999,
            &mut rbp as *mut [[f64; 3]; 3] as *mut f64,
        );
    }

    vvd(
        rbp[0][0],
        0.9999995505175087260,
        1e-12,
        "eraPmat00",
        "11",
        status,
    );
    vvd(
        rbp[0][1],
        0.8695405883617884705e-3,
        1e-14,
        "eraPmat00",
        "12",
        status,
    );
    vvd(
        rbp[0][2],
        0.3779734722239007105e-3,
        1e-14,
        "eraPmat00",
        "13",
        status,
    );
    vvd(
        rbp[1][0],
        -0.8695405990410863719e-3,
        1e-14,
        "eraPmat00",
        "21",
        status,
    );
    vvd(
        rbp[1][1],
        0.9999996219494925900,
        1e-12,
        "eraPmat00",
        "22",
        status,
    );
    vvd(
        rbp[1][2],
        -0.1360775820404982209e-6,
        1e-14,
        "eraPmat00",
        "23",
        status,
    );
    vvd(
        rbp[2][0],
        -0.3779734476558184991e-3,
        1e-14,
        "eraPmat00",
        "31",
        status,
    );
    vvd(
        rbp[2][1],
        -0.1925857585832024058e-6,
        1e-14,
        "eraPmat00",
        "32",
        status,
    );
    vvd(
        rbp[2][2],
        0.9999999285680153377,
        1e-12,
        "eraPmat00",
        "33",
        status,
    );
}

fn t_pmat06(status: &mut i32) {
    let mut rbp = [[0.0; 3]; 3];

    unsafe {
        H1::eraPmat06(
            2400000.5,
            50123.9999,
            &mut rbp as *mut [[f64; 3]; 3] as *mut f64,
        );
    }

    vvd(
        rbp[0][0],
        0.9999995505176007047,
        1e-12,
        "eraPmat06",
        "11",
        status,
    );
    vvd(
        rbp[0][1],
        0.8695404617348208406e-3,
        1e-14,
        "eraPmat06",
        "12",
        status,
    );
    vvd(
        rbp[0][2],
        0.3779735201865589104e-3,
        1e-14,
        "eraPmat06",
        "13",
        status,
    );
    vvd(
        rbp[1][0],
        -0.8695404723772031414e-3,
        1e-14,
        "eraPmat06",
        "21",
        status,
    );
    vvd(
        rbp[1][1],
        0.9999996219496027161,
        1e-12,
        "eraPmat06",
        "22",
        status,
    );
    vvd(
        rbp[1][2],
        -0.1361752497080270143e-6,
        1e-14,
        "eraPmat06",
        "23",
        status,
    );
    vvd(
        rbp[2][0],
        -0.3779734957034089490e-3,
        1e-14,
        "eraPmat06",
        "31",
        status,
    );
    vvd(
        rbp[2][1],
        -0.1924880847894457113e-6,
        1e-14,
        "eraPmat06",
        "32",
        status,
    );
    vvd(
        rbp[2][2],
        0.9999999285679971958,
        1e-12,
        "eraPmat06",
        "33",
        status,
    );
}

fn t_pmat76(status: &mut i32) {
    let mut rmatp = [[0.0; 3]; 3];

    unsafe {
        H1::eraPmat76(
            2400000.5,
            50123.9999,
            &mut rmatp as *mut [[f64; 3]; 3] as *mut f64,
        );
    }

    vvd(
        rmatp[0][0],
        0.9999995504328350733,
        1e-12,
        "eraPmat76",
        "11",
        status,
    );
    vvd(
        rmatp[0][1],
        0.8696632209480960785e-3,
        1e-14,
        "eraPmat76",
        "12",
        status,
    );
    vvd(
        rmatp[0][2],
        0.3779153474959888345e-3,
        1e-14,
        "eraPmat76",
        "13",
        status,
    );
    vvd(
        rmatp[1][0],
        -0.8696632209485112192e-3,
        1e-14,
        "eraPmat76",
        "21",
        status,
    );
    vvd(
        rmatp[1][1],
        0.9999996218428560614,
        1e-12,
        "eraPmat76",
        "22",
        status,
    );
    vvd(
        rmatp[1][2],
        -0.1643284776111886407e-6,
        1e-14,
        "eraPmat76",
        "23",
        status,
    );
    vvd(
        rmatp[2][0],
        -0.3779153474950335077e-3,
        1e-14,
        "eraPmat76",
        "31",
        status,
    );
    vvd(
        rmatp[2][1],
        -0.1643306746147366896e-6,
        1e-14,
        "eraPmat76",
        "32",
        status,
    );
    vvd(
        rmatp[2][2],
        0.9999999285899790119,
        1e-12,
        "eraPmat76",
        "33",
        status,
    );
}

fn t_pmp(status: &mut i32) {
    let mut a = [2.0, 2.0, 3.0];
    let mut b = [1.0, 3.0, 4.0];
    let mut amb = [0.0; 3];

    unsafe {
        H1::eraPmp(
            &mut a as *mut [f64; 3] as *mut f64,
            &mut b as *mut [f64; 3] as *mut f64,
            &mut amb as *mut [f64; 3] as *mut f64,
        );
    }

    vvd(amb[0], 1.0, 1e-12, "eraPmp", "0", status);
    vvd(amb[1], -1.0, 1e-12, "eraPmp", "1", status);
    vvd(amb[2], -1.0, 1e-12, "eraPmp", "2", status);
}

fn t_pmpx(status: &mut i32) {
    let rc = 1.234;
    let dc = 0.789;
    let pr = 1e-5;
    let pd = -2e-5;
    let px = 1e-2;
    let rv = 10.0;
    let pmt = 8.75;
    let mut pob = [0.9, 0.4, 0.1];
    let mut pco = [0.0; 3];

    unsafe {
        H1::eraPmpx(
            rc,
            dc,
            pr,
            pd,
            px,
            rv,
            pmt,
            &mut pob as *mut [f64; 3] as *mut f64,
            &mut pco as *mut [f64; 3] as *mut f64,
        );
    }

    vvd(pco[0], 0.2328137623960308438, 1e-12, "eraPmpx", "1", status);
    vvd(pco[1], 0.6651097085397855328, 1e-12, "eraPmpx", "2", status);
    vvd(pco[2], 0.7095257765896359837, 1e-12, "eraPmpx", "3", status);
}

fn t_pmsafe(status: &mut i32) {
    let ra1 = 1.234;
    let dec1 = 0.789;
    let pmr1 = 1e-5;
    let pmd1 = -2e-5;
    let px1 = 1e-2;
    let rv1 = 10.0;
    let ep1a = 2400000.5;
    let ep1b = 48348.5625;
    let ep2a = 2400000.5;
    let ep2b = 51544.5;

    let mut ra2 = 0.0;
    let mut dec2 = 0.0;
    let mut pmr2 = 0.0;
    let mut pmd2 = 0.0;
    let mut px2 = 0.0;
    let mut rv2 = 0.0;

    let j = unsafe {
        H1::eraPmsafe(
            ra1,
            dec1,
            pmr1,
            pmd1,
            px1,
            rv1,
            ep1a,
            ep1b,
            ep2a,
            ep2b,
            &mut ra2 as *mut f64,
            &mut dec2 as *mut f64,
            &mut pmr2 as *mut f64,
            &mut pmd2 as *mut f64,
            &mut px2 as *mut f64,
            &mut rv2 as *mut f64,
        )
    };

    vvd(ra2, 1.234087484501017061, 1e-12, "eraPmsafe", "ra2", status);
    vvd(
        dec2,
        0.7888249982450468567,
        1e-12,
        "eraPmsafe",
        "dec2",
        status,
    );
    vvd(
        pmr2,
        0.9996457663586073988e-5,
        1e-12,
        "eraPmsafe",
        "pmr2",
        status,
    );
    vvd(
        pmd2,
        -0.2000040085106754565e-4,
        1e-16,
        "eraPmsafe",
        "pmd2",
        status,
    );
    vvd(
        px2,
        0.9999997295356830666e-2,
        1e-12,
        "eraPmsafe",
        "px2",
        status,
    );
    vvd(rv2, 10.38468380293920069, 1e-10, "eraPmsafe", "rv2", status);
    viv(j, 0, "eraPmsafe", "j", status);
}

fn t_pn(status: &mut i32) {
    let mut p = [0.3, 1.2, -2.5];
    let mut r = 0.0;
    let mut u = [0.0; 3];

    unsafe {
        H1::eraPn(
            &mut p as *mut [f64; 3] as *mut f64,
            &mut r as *mut f64,
            &mut u as *mut [f64; 3] as *mut f64,
        );
    }

    vvd(r, 2.789265136196270604, 1e-12, "eraPn", "r", status);
    vvd(u[0], 0.1075552109073112058, 1e-12, "eraPn", "u1", status);
    vvd(u[1], 0.4302208436292448232, 1e-12, "eraPn", "u2", status);
    vvd(u[2], -0.8962934242275933816, 1e-12, "eraPn", "u3", status);
}

fn t_pn00(status: &mut i32) {
    let dpsi = -0.9632552291149335877e-5;
    let deps = 0.4063197106621141414e-4;

    let mut epsa = 0.0;
    let mut rb = [[0.0; 3]; 3];
    let mut rp = [[0.0; 3]; 3];
    let mut rbp = [[0.0; 3]; 3];
    let mut rn = [[0.0; 3]; 3];
    let mut rbpn = [[0.0; 3]; 3];

    unsafe {
        H1::eraPn00(
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
        0.4090791789404229916,
        1e-12,
        "eraPn00",
        "epsa",
        status,
    );
    vvd(
        rb[0][0],
        0.9999999999999942498,
        1e-12,
        "eraPn00",
        "rb11",
        status,
    );
    vvd(
        rb[0][1],
        -0.7078279744199196626e-7,
        1e-18,
        "eraPn00",
        "rb12",
        status,
    );
    vvd(
        rb[0][2],
        0.8056217146976134152e-7,
        1e-18,
        "eraPn00",
        "rb13",
        status,
    );
    vvd(
        rb[1][0],
        0.7078279477857337206e-7,
        1e-18,
        "eraPn00",
        "rb21",
        status,
    );
    vvd(
        rb[1][1],
        0.9999999999999969484,
        1e-12,
        "eraPn00",
        "rb22",
        status,
    );
    vvd(
        rb[1][2],
        0.3306041454222136517e-7,
        1e-18,
        "eraPn00",
        "rb23",
        status,
    );
    vvd(
        rb[2][0],
        -0.8056217380986972157e-7,
        1e-18,
        "eraPn00",
        "rb31",
        status,
    );
    vvd(
        rb[2][1],
        -0.3306040883980552500e-7,
        1e-18,
        "eraPn00",
        "rb32",
        status,
    );
    vvd(
        rb[2][2],
        0.9999999999999962084,
        1e-12,
        "eraPn00",
        "rb33",
        status,
    );
    vvd(
        rp[0][0],
        0.9999989300532289018,
        1e-12,
        "eraPn00",
        "rp11",
        status,
    );
    vvd(
        rp[0][1],
        -0.1341647226791824349e-2,
        1e-14,
        "eraPn00",
        "rp12",
        status,
    );
    vvd(
        rp[0][2],
        -0.5829880927190296547e-3,
        1e-14,
        "eraPn00",
        "rp13",
        status,
    );
    vvd(
        rp[1][0],
        0.1341647231069759008e-2,
        1e-14,
        "eraPn00",
        "rp21",
        status,
    );
    vvd(
        rp[1][1],
        0.9999990999908750433,
        1e-12,
        "eraPn00",
        "rp22",
        status,
    );
    vvd(
        rp[1][2],
        -0.3837444441583715468e-6,
        1e-14,
        "eraPn00",
        "rp23",
        status,
    );
    vvd(
        rp[2][0],
        0.5829880828740957684e-3,
        1e-14,
        "eraPn00",
        "rp31",
        status,
    );
    vvd(
        rp[2][1],
        -0.3984203267708834759e-6,
        1e-14,
        "eraPn00",
        "rp32",
        status,
    );
    vvd(
        rp[2][2],
        0.9999998300623538046,
        1e-12,
        "eraPn00",
        "rp33",
        status,
    );
    vvd(
        rbp[0][0],
        0.9999989300052243993,
        1e-12,
        "eraPn00",
        "rbp11",
        status,
    );
    vvd(
        rbp[0][1],
        -0.1341717990239703727e-2,
        1e-14,
        "eraPn00",
        "rbp12",
        status,
    );
    vvd(
        rbp[0][2],
        -0.5829075749891684053e-3,
        1e-14,
        "eraPn00",
        "rbp13",
        status,
    );
    vvd(
        rbp[1][0],
        0.1341718013831739992e-2,
        1e-14,
        "eraPn00",
        "rbp21",
        status,
    );
    vvd(
        rbp[1][1],
        0.9999990998959191343,
        1e-12,
        "eraPn00",
        "rbp22",
        status,
    );
    vvd(
        rbp[1][2],
        -0.3505759733565421170e-6,
        1e-14,
        "eraPn00",
        "rbp23",
        status,
    );
    vvd(
        rbp[2][0],
        0.5829075206857717883e-3,
        1e-14,
        "eraPn00",
        "rbp31",
        status,
    );
    vvd(
        rbp[2][1],
        -0.4315219955198608970e-6,
        1e-14,
        "eraPn00",
        "rbp32",
        status,
    );
    vvd(
        rbp[2][2],
        0.9999998301093036269,
        1e-12,
        "eraPn00",
        "rbp33",
        status,
    );
    vvd(
        rn[0][0],
        0.9999999999536069682,
        1e-12,
        "eraPn00",
        "rn11",
        status,
    );
    vvd(
        rn[0][1],
        0.8837746144872140812e-5,
        1e-16,
        "eraPn00",
        "rn12",
        status,
    );
    vvd(
        rn[0][2],
        0.3831488838252590008e-5,
        1e-16,
        "eraPn00",
        "rn13",
        status,
    );
    vvd(
        rn[1][0],
        -0.8837590456633197506e-5,
        1e-16,
        "eraPn00",
        "rn21",
        status,
    );
    vvd(
        rn[1][1],
        0.9999999991354692733,
        1e-12,
        "eraPn00",
        "rn22",
        status,
    );
    vvd(
        rn[1][2],
        -0.4063198798559573702e-4,
        1e-16,
        "eraPn00",
        "rn23",
        status,
    );
    vvd(
        rn[2][0],
        -0.3831847930135328368e-5,
        1e-16,
        "eraPn00",
        "rn31",
        status,
    );
    vvd(
        rn[2][1],
        0.4063195412258150427e-4,
        1e-16,
        "eraPn00",
        "rn32",
        status,
    );
    vvd(
        rn[2][2],
        0.9999999991671806225,
        1e-12,
        "eraPn00",
        "rn33",
        status,
    );
    vvd(
        rbpn[0][0],
        0.9999989440499982806,
        1e-12,
        "eraPn00",
        "rbpn11",
        status,
    );
    vvd(
        rbpn[0][1],
        -0.1332880253640848301e-2,
        1e-14,
        "eraPn00",
        "rbpn12",
        status,
    );
    vvd(
        rbpn[0][2],
        -0.5790760898731087295e-3,
        1e-14,
        "eraPn00",
        "rbpn13",
        status,
    );
    vvd(
        rbpn[1][0],
        0.1332856746979948745e-2,
        1e-14,
        "eraPn00",
        "rbpn21",
        status,
    );
    vvd(
        rbpn[1][1],
        0.9999991109064768883,
        1e-12,
        "eraPn00",
        "rbpn22",
        status,
    );
    vvd(
        rbpn[1][2],
        -0.4097740555723063806e-4,
        1e-14,
        "eraPn00",
        "rbpn23",
        status,
    );
    vvd(
        rbpn[2][0],
        0.5791301929950205000e-3,
        1e-14,
        "eraPn00",
        "rbpn31",
        status,
    );
    vvd(
        rbpn[2][1],
        0.4020553681373702931e-4,
        1e-14,
        "eraPn00",
        "rbpn32",
        status,
    );
    vvd(
        rbpn[2][2],
        0.9999998314958529887,
        1e-12,
        "eraPn00",
        "rbpn33",
        status,
    );
}

fn t_pn00a(status: &mut i32) {
    let mut dpsi = 0.0;
    let mut deps = 0.0;
    let mut epsa = 0.0;
    let mut rb = [[0.0; 3]; 3];
    let mut rp = [[0.0; 3]; 3];
    let mut rbp = [[0.0; 3]; 3];
    let mut rn = [[0.0; 3]; 3];
    let mut rbpn = [[0.0; 3]; 3];

    unsafe {
        H1::eraPn00a(
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
        -0.9630909107115518431e-5,
        1e-12,
        "eraPn00a",
        "dpsi",
        status,
    );
    vvd(
        deps,
        0.4063239174001678710e-4,
        1e-12,
        "eraPn00a",
        "deps",
        status,
    );
    vvd(
        epsa,
        0.4090791789404229916,
        1e-12,
        "eraPn00a",
        "epsa",
        status,
    );
    vvd(
        rb[0][0],
        0.9999999999999942498,
        1e-12,
        "eraPn00a",
        "rb11",
        status,
    );
    vvd(
        rb[0][1],
        -0.7078279744199196626e-7,
        1e-16,
        "eraPn00a",
        "rb12",
        status,
    );
    vvd(
        rb[0][2],
        0.8056217146976134152e-7,
        1e-16,
        "eraPn00a",
        "rb13",
        status,
    );
    vvd(
        rb[1][0],
        0.7078279477857337206e-7,
        1e-16,
        "eraPn00a",
        "rb21",
        status,
    );
    vvd(
        rb[1][1],
        0.9999999999999969484,
        1e-12,
        "eraPn00a",
        "rb22",
        status,
    );
    vvd(
        rb[1][2],
        0.3306041454222136517e-7,
        1e-16,
        "eraPn00a",
        "rb23",
        status,
    );
    vvd(
        rb[2][0],
        -0.8056217380986972157e-7,
        1e-16,
        "eraPn00a",
        "rb31",
        status,
    );
    vvd(
        rb[2][1],
        -0.3306040883980552500e-7,
        1e-16,
        "eraPn00a",
        "rb32",
        status,
    );
    vvd(
        rb[2][2],
        0.9999999999999962084,
        1e-12,
        "eraPn00a",
        "rb33",
        status,
    );
    vvd(
        rp[0][0],
        0.9999989300532289018,
        1e-12,
        "eraPn00a",
        "rp11",
        status,
    );
    vvd(
        rp[0][1],
        -0.1341647226791824349e-2,
        1e-14,
        "eraPn00a",
        "rp12",
        status,
    );
    vvd(
        rp[0][2],
        -0.5829880927190296547e-3,
        1e-14,
        "eraPn00a",
        "rp13",
        status,
    );
    vvd(
        rp[1][0],
        0.1341647231069759008e-2,
        1e-14,
        "eraPn00a",
        "rp21",
        status,
    );
    vvd(
        rp[1][1],
        0.9999990999908750433,
        1e-12,
        "eraPn00a",
        "rp22",
        status,
    );
    vvd(
        rp[1][2],
        -0.3837444441583715468e-6,
        1e-14,
        "eraPn00a",
        "rp23",
        status,
    );
    vvd(
        rp[2][0],
        0.5829880828740957684e-3,
        1e-14,
        "eraPn00a",
        "rp31",
        status,
    );
    vvd(
        rp[2][1],
        -0.3984203267708834759e-6,
        1e-14,
        "eraPn00a",
        "rp32",
        status,
    );
    vvd(
        rp[2][2],
        0.9999998300623538046,
        1e-12,
        "eraPn00a",
        "rp33",
        status,
    );
    vvd(
        rbp[0][0],
        0.9999989300052243993,
        1e-12,
        "eraPn00a",
        "rbp11",
        status,
    );
    vvd(
        rbp[0][1],
        -0.1341717990239703727e-2,
        1e-14,
        "eraPn00a",
        "rbp12",
        status,
    );
    vvd(
        rbp[0][2],
        -0.5829075749891684053e-3,
        1e-14,
        "eraPn00a",
        "rbp13",
        status,
    );
    vvd(
        rbp[1][0],
        0.1341718013831739992e-2,
        1e-14,
        "eraPn00a",
        "rbp21",
        status,
    );
    vvd(
        rbp[1][1],
        0.9999990998959191343,
        1e-12,
        "eraPn00a",
        "rbp22",
        status,
    );
    vvd(
        rbp[1][2],
        -0.3505759733565421170e-6,
        1e-14,
        "eraPn00a",
        "rbp23",
        status,
    );
    vvd(
        rbp[2][0],
        0.5829075206857717883e-3,
        1e-14,
        "eraPn00a",
        "rbp31",
        status,
    );
    vvd(
        rbp[2][1],
        -0.4315219955198608970e-6,
        1e-14,
        "eraPn00a",
        "rbp32",
        status,
    );
    vvd(
        rbp[2][2],
        0.9999998301093036269,
        1e-12,
        "eraPn00a",
        "rbp33",
        status,
    );
    vvd(
        rn[0][0],
        0.9999999999536227949,
        1e-12,
        "eraPn00a",
        "rn11",
        status,
    );
    vvd(
        rn[0][1],
        0.8836238544090873336e-5,
        1e-14,
        "eraPn00a",
        "rn12",
        status,
    );
    vvd(
        rn[0][2],
        0.3830835237722400669e-5,
        1e-14,
        "eraPn00a",
        "rn13",
        status,
    );
    vvd(
        rn[1][0],
        -0.8836082880798569274e-5,
        1e-14,
        "eraPn00a",
        "rn21",
        status,
    );
    vvd(
        rn[1][1],
        0.9999999991354655028,
        1e-12,
        "eraPn00a",
        "rn22",
        status,
    );
    vvd(
        rn[1][2],
        -0.4063240865362499850e-4,
        1e-14,
        "eraPn00a",
        "rn23",
        status,
    );
    vvd(
        rn[2][0],
        -0.3831194272065995866e-5,
        1e-14,
        "eraPn00a",
        "rn31",
        status,
    );
    vvd(
        rn[2][1],
        0.4063237480216291775e-4,
        1e-14,
        "eraPn00a",
        "rn32",
        status,
    );
    vvd(
        rn[2][2],
        0.9999999991671660338,
        1e-12,
        "eraPn00a",
        "rn33",
        status,
    );
    vvd(
        rbpn[0][0],
        0.9999989440476103435,
        1e-12,
        "eraPn00a",
        "rbpn11",
        status,
    );
    vvd(
        rbpn[0][1],
        -0.1332881761240011763e-2,
        1e-14,
        "eraPn00a",
        "rbpn12",
        status,
    );
    vvd(
        rbpn[0][2],
        -0.5790767434730085751e-3,
        1e-14,
        "eraPn00a",
        "rbpn13",
        status,
    );
    vvd(
        rbpn[1][0],
        0.1332858254308954658e-2,
        1e-14,
        "eraPn00a",
        "rbpn21",
        status,
    );
    vvd(
        rbpn[1][1],
        0.9999991109044505577,
        1e-12,
        "eraPn00a",
        "rbpn22",
        status,
    );
    vvd(
        rbpn[1][2],
        -0.4097782710396580452e-4,
        1e-14,
        "eraPn00a",
        "rbpn23",
        status,
    );
    vvd(
        rbpn[2][0],
        0.5791308472168152904e-3,
        1e-14,
        "eraPn00a",
        "rbpn31",
        status,
    );
    vvd(
        rbpn[2][1],
        0.4020595661591500259e-4,
        1e-14,
        "eraPn00a",
        "rbpn32",
        status,
    );
    vvd(
        rbpn[2][2],
        0.9999998314954572304,
        1e-12,
        "eraPn00a",
        "rbpn33",
        status,
    );
}

fn t_pn00b(status: &mut i32) {
    let mut dpsi = 0.0;
    let mut deps = 0.0;
    let mut epsa = 0.0;
    let mut rb = [[0.0; 3]; 3];
    let mut rp = [[0.0; 3]; 3];
    let mut rbp = [[0.0; 3]; 3];
    let mut rn = [[0.0; 3]; 3];
    let mut rbpn = [[0.0; 3]; 3];

    unsafe {
        H1::eraPn00b(
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
        -0.9632552291148362783e-5,
        1e-12,
        "eraPn00b",
        "dpsi",
        status,
    );
    vvd(
        deps,
        0.4063197106621159367e-4,
        1e-12,
        "eraPn00b",
        "deps",
        status,
    );
    vvd(
        epsa,
        0.4090791789404229916,
        1e-12,
        "eraPn00b",
        "epsa",
        status,
    );
    vvd(
        rb[0][0],
        0.9999999999999942498,
        1e-12,
        "eraPn00b",
        "rb11",
        status,
    );
    vvd(
        rb[0][1],
        -0.7078279744199196626e-7,
        1e-16,
        "eraPn00b",
        "rb12",
        status,
    );
    vvd(
        rb[0][2],
        0.8056217146976134152e-7,
        1e-16,
        "eraPn00b",
        "rb13",
        status,
    );
    vvd(
        rb[1][0],
        0.7078279477857337206e-7,
        1e-16,
        "eraPn00b",
        "rb21",
        status,
    );
    vvd(
        rb[1][1],
        0.9999999999999969484,
        1e-12,
        "eraPn00b",
        "rb22",
        status,
    );
    vvd(
        rb[1][2],
        0.3306041454222136517e-7,
        1e-16,
        "eraPn00b",
        "rb23",
        status,
    );
    vvd(
        rb[2][0],
        -0.8056217380986972157e-7,
        1e-16,
        "eraPn00b",
        "rb31",
        status,
    );
    vvd(
        rb[2][1],
        -0.3306040883980552500e-7,
        1e-16,
        "eraPn00b",
        "rb32",
        status,
    );
    vvd(
        rb[2][2],
        0.9999999999999962084,
        1e-12,
        "eraPn00b",
        "rb33",
        status,
    );
    vvd(
        rp[0][0],
        0.9999989300532289018,
        1e-12,
        "eraPn00b",
        "rp11",
        status,
    );
    vvd(
        rp[0][1],
        -0.1341647226791824349e-2,
        1e-14,
        "eraPn00b",
        "rp12",
        status,
    );
    vvd(
        rp[0][2],
        -0.5829880927190296547e-3,
        1e-14,
        "eraPn00b",
        "rp13",
        status,
    );
    vvd(
        rp[1][0],
        0.1341647231069759008e-2,
        1e-14,
        "eraPn00b",
        "rp21",
        status,
    );
    vvd(
        rp[1][1],
        0.9999990999908750433,
        1e-12,
        "eraPn00b",
        "rp22",
        status,
    );
    vvd(
        rp[1][2],
        -0.3837444441583715468e-6,
        1e-14,
        "eraPn00b",
        "rp23",
        status,
    );
    vvd(
        rp[2][0],
        0.5829880828740957684e-3,
        1e-14,
        "eraPn00b",
        "rp31",
        status,
    );
    vvd(
        rp[2][1],
        -0.3984203267708834759e-6,
        1e-14,
        "eraPn00b",
        "rp32",
        status,
    );
    vvd(
        rp[2][2],
        0.9999998300623538046,
        1e-12,
        "eraPn00b",
        "rp33",
        status,
    );
    vvd(
        rbp[0][0],
        0.9999989300052243993,
        1e-12,
        "eraPn00b",
        "rbp11",
        status,
    );
    vvd(
        rbp[0][1],
        -0.1341717990239703727e-2,
        1e-14,
        "eraPn00b",
        "rbp12",
        status,
    );
    vvd(
        rbp[0][2],
        -0.5829075749891684053e-3,
        1e-14,
        "eraPn00b",
        "rbp13",
        status,
    );
    vvd(
        rbp[1][0],
        0.1341718013831739992e-2,
        1e-14,
        "eraPn00b",
        "rbp21",
        status,
    );
    vvd(
        rbp[1][1],
        0.9999990998959191343,
        1e-12,
        "eraPn00b",
        "rbp22",
        status,
    );
    vvd(
        rbp[1][2],
        -0.3505759733565421170e-6,
        1e-14,
        "eraPn00b",
        "rbp23",
        status,
    );
    vvd(
        rbp[2][0],
        0.5829075206857717883e-3,
        1e-14,
        "eraPn00b",
        "rbp31",
        status,
    );
    vvd(
        rbp[2][1],
        -0.4315219955198608970e-6,
        1e-14,
        "eraPn00b",
        "rbp32",
        status,
    );
    vvd(
        rbp[2][2],
        0.9999998301093036269,
        1e-12,
        "eraPn00b",
        "rbp33",
        status,
    );
    vvd(
        rn[0][0],
        0.9999999999536069682,
        1e-12,
        "eraPn00b",
        "rn11",
        status,
    );
    vvd(
        rn[0][1],
        0.8837746144871248011e-5,
        1e-14,
        "eraPn00b",
        "rn12",
        status,
    );
    vvd(
        rn[0][2],
        0.3831488838252202945e-5,
        1e-14,
        "eraPn00b",
        "rn13",
        status,
    );
    vvd(
        rn[1][0],
        -0.8837590456632304720e-5,
        1e-14,
        "eraPn00b",
        "rn21",
        status,
    );
    vvd(
        rn[1][1],
        0.9999999991354692733,
        1e-12,
        "eraPn00b",
        "rn22",
        status,
    );
    vvd(
        rn[1][2],
        -0.4063198798559591654e-4,
        1e-14,
        "eraPn00b",
        "rn23",
        status,
    );
    vvd(
        rn[2][0],
        -0.3831847930134941271e-5,
        1e-14,
        "eraPn00b",
        "rn31",
        status,
    );
    vvd(
        rn[2][1],
        0.4063195412258168380e-4,
        1e-14,
        "eraPn00b",
        "rn32",
        status,
    );
    vvd(
        rn[2][2],
        0.9999999991671806225,
        1e-12,
        "eraPn00b",
        "rn33",
        status,
    );
    vvd(
        rbpn[0][0],
        0.9999989440499982806,
        1e-12,
        "eraPn00b",
        "rbpn11",
        status,
    );
    vvd(
        rbpn[0][1],
        -0.1332880253640849194e-2,
        1e-14,
        "eraPn00b",
        "rbpn12",
        status,
    );
    vvd(
        rbpn[0][2],
        -0.5790760898731091166e-3,
        1e-14,
        "eraPn00b",
        "rbpn13",
        status,
    );
    vvd(
        rbpn[1][0],
        0.1332856746979949638e-2,
        1e-14,
        "eraPn00b",
        "rbpn21",
        status,
    );
    vvd(
        rbpn[1][1],
        0.9999991109064768883,
        1e-12,
        "eraPn00b",
        "rbpn22",
        status,
    );
    vvd(
        rbpn[1][2],
        -0.4097740555723081811e-4,
        1e-14,
        "eraPn00b",
        "rbpn23",
        status,
    );
    vvd(
        rbpn[2][0],
        0.5791301929950208873e-3,
        1e-14,
        "eraPn00b",
        "rbpn31",
        status,
    );
    vvd(
        rbpn[2][1],
        0.4020553681373720832e-4,
        1e-14,
        "eraPn00b",
        "rbpn32",
        status,
    );
    vvd(
        rbpn[2][2],
        0.9999998314958529887,
        1e-12,
        "eraPn00b",
        "rbpn33",
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

    t_pmat00(&mut status);
    t_pmat06(&mut status);
    t_pmat76(&mut status);
    t_pmp(&mut status);
    t_pmpx(&mut status);
    t_pmsafe(&mut status);
    t_pn(&mut status);
    t_pn00(&mut status);
    t_pn00a(&mut status);
    t_pn00b(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
