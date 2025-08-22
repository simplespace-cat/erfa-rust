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

fn t_p06e(status: &mut i32) {
    let mut eps0: f64 = 0.0;
    let mut psia: f64 = 0.0;
    let mut oma: f64 = 0.0;
    let mut bpa: f64 = 0.0;
    let mut bqa: f64 = 0.0;
    let mut pia: f64 = 0.0;
    let mut bpia: f64 = 0.0;
    let mut epsa: f64 = 0.0;
    let mut chia: f64 = 0.0;
    let mut za: f64 = 0.0;
    let mut zetaa: f64 = 0.0;
    let mut thetaa: f64 = 0.0;
    let mut pa: f64 = 0.0;
    let mut gam: f64 = 0.0;
    let mut phi: f64 = 0.0;
    let mut psi: f64 = 0.0;

    unsafe {
        H1::eraP06e(
            2400000.5,
            52541.0,
            &mut eps0 as *mut f64,
            &mut psia as *mut f64,
            &mut oma as *mut f64,
            &mut bpa as *mut f64,
            &mut bqa as *mut f64,
            &mut pia as *mut f64,
            &mut bpia as *mut f64,
            &mut epsa as *mut f64,
            &mut chia as *mut f64,
            &mut za as *mut f64,
            &mut zetaa as *mut f64,
            &mut thetaa as *mut f64,
            &mut pa as *mut f64,
            &mut gam as *mut f64,
            &mut phi as *mut f64,
            &mut psi as *mut f64,
        );
    }

    vvd(
        eps0,
        0.4090926006005828715,
        1e-14,
        "eraP06e",
        "eps0",
        status,
    );
    vvd(
        psia,
        0.6664369630191613431e-3,
        1e-14,
        "eraP06e",
        "psia",
        status,
    );
    vvd(oma, 0.4090925973783255982, 1e-14, "eraP06e", "oma", status);
    vvd(
        bpa,
        0.5561149371265209445e-6,
        1e-14,
        "eraP06e",
        "bpa",
        status,
    );
    vvd(
        bqa,
        -0.6191517193290621270e-5,
        1e-14,
        "eraP06e",
        "bqa",
        status,
    );
    vvd(
        pia,
        0.6216441751884382923e-5,
        1e-14,
        "eraP06e",
        "pia",
        status,
    );
    vvd(bpia, 3.052014180023779882, 1e-14, "eraP06e", "bpia", status);
    vvd(
        epsa,
        0.4090864054922431688,
        1e-14,
        "eraP06e",
        "epsa",
        status,
    );
    vvd(
        chia,
        0.1387703379530915364e-5,
        1e-14,
        "eraP06e",
        "chia",
        status,
    );
    vvd(za, 0.2921789846651790546e-3, 1e-14, "eraP06e", "za", status);
    vvd(
        zetaa,
        0.3178773290332009310e-3,
        1e-14,
        "eraP06e",
        "zetaa",
        status,
    );
    vvd(
        thetaa,
        0.2650932701657497181e-3,
        1e-14,
        "eraP06e",
        "thetaa",
        status,
    );
    vvd(pa, 0.6651637681381016288e-3, 1e-14, "eraP06e", "pa", status);
    vvd(
        gam,
        0.1398077115963754987e-5,
        1e-14,
        "eraP06e",
        "gam",
        status,
    );
    vvd(phi, 0.4090864090837462602, 1e-14, "eraP06e", "phi", status);
    vvd(
        psi,
        0.6664464807480920325e-3,
        1e-14,
        "eraP06e",
        "psi",
        status,
    );
}

fn t_p2pv(status: &mut i32) {
    let mut p = [0.25, 1.2, 3.0];
    let mut pv = [[0.3, 1.2, -2.5], [-0.5, 3.1, 0.9]];

    unsafe {
        H1::eraP2pv(p.as_mut_ptr(), &mut pv as *mut [[f64; 3]; 2] as *mut f64);
    }

    vvd(pv[0][0], 0.25, 0.0, "eraP2pv", "p1", status);
    vvd(pv[0][1], 1.2, 0.0, "eraP2pv", "p2", status);
    vvd(pv[0][2], 3.0, 0.0, "eraP2pv", "p3", status);
    vvd(pv[1][0], 0.0, 0.0, "eraP2pv", "v1", status);
    vvd(pv[1][1], 0.0, 0.0, "eraP2pv", "v2", status);
    vvd(pv[1][2], 0.0, 0.0, "eraP2pv", "v3", status);
}

fn t_p2s(status: &mut i32) {
    let mut p = [100.0, -50.0, 25.0];
    let mut theta: f64 = 0.0;
    let mut phi: f64 = 0.0;
    let mut r: f64 = 0.0;

    unsafe {
        H1::eraP2s(
            p.as_mut_ptr(),
            &mut theta as *mut f64,
            &mut phi as *mut f64,
            &mut r as *mut f64,
        );
    }

    vvd(
        theta,
        -0.4636476090008061162,
        1e-12,
        "eraP2s",
        "theta",
        status,
    );
    vvd(phi, 0.2199879773954594463, 1e-12, "eraP2s", "phi", status);
    vvd(r, 114.5643923738960002, 1e-9, "eraP2s", "r", status);
}

fn t_pap(status: &mut i32) {
    let mut a = [1.0, 0.1, 0.2];
    let mut b = [-3.0, 1e-3, 0.2];

    let theta = unsafe { H1::eraPap(a.as_mut_ptr(), b.as_mut_ptr()) };

    vvd(theta, 0.3671514267841113674, 1e-12, "eraPap", "", status);
}

fn t_pas(status: &mut i32) {
    let theta = unsafe { H1::eraPas(1.0, 0.1, 0.2, -1.0) };
    vvd(theta, -2.724544922932270424, 1e-12, "eraPas", "", status);
}

fn t_pb06(status: &mut i32) {
    let mut bzeta: f64 = 0.0;
    let mut bz: f64 = 0.0;
    let mut btheta: f64 = 0.0;

    unsafe {
        H1::eraPb06(
            2400000.5,
            50123.9999,
            &mut bzeta as *mut f64,
            &mut bz as *mut f64,
            &mut btheta as *mut f64,
        );
    }

    vvd(
        bzeta,
        -0.5092634016326478238e-3,
        1e-12,
        "eraPb06",
        "bzeta",
        status,
    );
    vvd(
        bz,
        -0.3602772060566044413e-3,
        1e-12,
        "eraPb06",
        "bz",
        status,
    );
    vvd(
        btheta,
        -0.3779735537167811177e-3,
        1e-12,
        "eraPb06",
        "btheta",
        status,
    );
}

fn t_pdp(status: &mut i32) {
    let mut a = [2.0, 2.0, 3.0];
    let mut b = [1.0, 3.0, 4.0];

    let adb = unsafe { H1::eraPdp(a.as_mut_ptr(), b.as_mut_ptr()) };

    vvd(adb, 20.0, 1e-12, "eraPdp", "", status);
}

fn t_pfw06(status: &mut i32) {
    let mut gamb: f64 = 0.0;
    let mut phib: f64 = 0.0;
    let mut psib: f64 = 0.0;
    let mut epsa: f64 = 0.0;

    unsafe {
        H1::eraPfw06(
            2400000.5,
            50123.9999,
            &mut gamb as *mut f64,
            &mut phib as *mut f64,
            &mut psib as *mut f64,
            &mut epsa as *mut f64,
        );
    }

    vvd(
        gamb,
        -0.2243387670997995690e-5,
        1e-16,
        "eraPfw06",
        "gamb",
        status,
    );
    vvd(
        phib,
        0.4091014602391312808,
        1e-12,
        "eraPfw06",
        "phib",
        status,
    );
    vvd(
        psib,
        -0.9501954178013031895e-3,
        1e-14,
        "eraPfw06",
        "psib",
        status,
    );
    vvd(
        epsa,
        0.4091014316587367491,
        1e-12,
        "eraPfw06",
        "epsa",
        status,
    );
}

fn t_plan94(status: &mut i32) {
    let mut pv = [[0.0; 3]; 2];
    let mut j: i32;

    j = unsafe { H1::eraPlan94(2400000.5, 1e6, 0, &mut pv as *mut [[f64; 3]; 2] as *mut f64) };
    vvd(pv[0][0], 0.0, 0.0, "eraPlan94", "x 1", status);
    vvd(pv[0][1], 0.0, 0.0, "eraPlan94", "y 1", status);
    vvd(pv[0][2], 0.0, 0.0, "eraPlan94", "z 1", status);
    vvd(pv[1][0], 0.0, 0.0, "eraPlan94", "xd 1", status);
    vvd(pv[1][1], 0.0, 0.0, "eraPlan94", "yd 1", status);
    vvd(pv[1][2], 0.0, 0.0, "eraPlan94", "zd 1", status);
    viv(j, -1, "eraPlan94", "j 1", status);

    j = unsafe {
        H1::eraPlan94(
            2400000.5,
            1e6,
            10,
            &mut pv as *mut [[f64; 3]; 2] as *mut f64,
        )
    };
    viv(j, -1, "eraPlan94", "j 2", status);

    j = unsafe {
        H1::eraPlan94(
            2400000.5,
            -320000.0,
            3,
            &mut pv as *mut [[f64; 3]; 2] as *mut f64,
        )
    };
    vvd(
        pv[0][0],
        0.9308038666832975759,
        1e-11,
        "eraPlan94",
        "x 3",
        status,
    );
    vvd(
        pv[0][1],
        0.3258319040261346000,
        1e-11,
        "eraPlan94",
        "y 3",
        status,
    );
    vvd(
        pv[0][2],
        0.1422794544481140560,
        1e-11,
        "eraPlan94",
        "z 3",
        status,
    );
    vvd(
        pv[1][0],
        -0.6429458958255170006e-2,
        1e-11,
        "eraPlan94",
        "xd 3",
        status,
    );
    vvd(
        pv[1][1],
        0.1468570657704237764e-1,
        1e-11,
        "eraPlan94",
        "yd 3",
        status,
    );
    vvd(
        pv[1][2],
        0.6406996426270981189e-2,
        1e-11,
        "eraPlan94",
        "zd 3",
        status,
    );
    viv(j, 1, "eraPlan94", "j 3", status);

    j = unsafe {
        H1::eraPlan94(
            2400000.5,
            43999.9,
            1,
            &mut pv as *mut [[f64; 3]; 2] as *mut f64,
        )
    };
    vvd(
        pv[0][0],
        0.2945293959257430832,
        1e-11,
        "eraPlan94",
        "x 4",
        status,
    );
    vvd(
        pv[0][1],
        -0.2452204176601049596,
        1e-11,
        "eraPlan94",
        "y 4",
        status,
    );
    vvd(
        pv[0][2],
        -0.1615427700571978153,
        1e-11,
        "eraPlan94",
        "z 4",
        status,
    );
    vvd(
        pv[1][0],
        0.1413867871404614441e-1,
        1e-11,
        "eraPlan94",
        "xd 4",
        status,
    );
    vvd(
        pv[1][1],
        0.1946548301104706582e-1,
        1e-11,
        "eraPlan94",
        "yd 4",
        status,
    );
    vvd(
        pv[1][2],
        0.8929809783898904786e-2,
        1e-11,
        "eraPlan94",
        "zd 4",
        status,
    );
    viv(j, 0, "eraPlan94", "j 4", status);
}

fn t_pm(status: &mut i32) {
    let mut p = [0.3, 1.2, -2.5];
    let r = unsafe { H1::eraPm(p.as_mut_ptr()) };
    vvd(r, 2.789265136196270604, 1e-12, "eraPm", "", status);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        unsafe {
            VERBOSE = true;
        }
    }

    let mut status = 0;

    t_p06e(&mut status);
    t_p2pv(&mut status);
    t_p2s(&mut status);
    t_pap(&mut status);
    t_pas(&mut status);
    t_pb06(&mut status);
    t_pdp(&mut status);
    t_pfw06(&mut status);
    t_plan94(&mut status);
    t_pm(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
