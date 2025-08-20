#![allow(dead_code)]

use erfa_rust::G24_safe;
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
    let result = G24_safe::eraP06e_safe(2400000.5, 52541.0);

    match result {
        Ok((
            eps0,
            psia,
            oma,
            bpa,
            bqa,
            pia,
            bpia,
            epsa,
            chia,
            za,
            zetaa,
            thetaa,
            pa,
            gam,
            phi,
            psi,
        )) => {
            vvd(
                eps0,
                0.4090926006005828715,
                1e-14,
                "eraP06e_safe",
                "eps0",
                status,
            );
            vvd(
                psia,
                0.6664369630191613431e-3,
                1e-14,
                "eraP06e_safe",
                "psia",
                status,
            );
            vvd(
                oma,
                0.4090925973783255982,
                1e-14,
                "eraP06e_safe",
                "oma",
                status,
            );
            vvd(
                bpa,
                0.5561149371265209445e-6,
                1e-14,
                "eraP06e_safe",
                "bpa",
                status,
            );
            vvd(
                bqa,
                -0.6191517193290621270e-5,
                1e-14,
                "eraP06e_safe",
                "bqa",
                status,
            );
            vvd(
                pia,
                0.6216441751884382923e-5,
                1e-14,
                "eraP06e_safe",
                "pia",
                status,
            );
            vvd(
                bpia,
                3.052014180023779882,
                1e-14,
                "eraP06e_safe",
                "bpia",
                status,
            );
            vvd(
                epsa,
                0.4090864054922431688,
                1e-14,
                "eraP06e_safe",
                "epsa",
                status,
            );
            vvd(
                chia,
                0.1387703379530915364e-5,
                1e-14,
                "eraP06e_safe",
                "chia",
                status,
            );
            vvd(
                za,
                0.2921789846651790546e-3,
                1e-14,
                "eraP06e_safe",
                "za",
                status,
            );
            vvd(
                zetaa,
                0.3178773290332009310e-3,
                1e-14,
                "eraP06e_safe",
                "zetaa",
                status,
            );
            vvd(
                thetaa,
                0.2650932701657497181e-3,
                1e-14,
                "eraP06e_safe",
                "thetaa",
                status,
            );
            vvd(
                pa,
                0.6651637681381016288e-3,
                1e-14,
                "eraP06e_safe",
                "pa",
                status,
            );
            vvd(
                gam,
                0.1398077115963754987e-5,
                1e-14,
                "eraP06e_safe",
                "gam",
                status,
            );
            vvd(
                phi,
                0.4090864090837462602,
                1e-14,
                "eraP06e_safe",
                "phi",
                status,
            );
            vvd(
                psi,
                0.6664464807480920325e-3,
                1e-14,
                "eraP06e_safe",
                "psi",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraP06e_safe failed: unexpected error");
        }
    }
}

fn t_p2pv(status: &mut i32) {
    let p = [0.25, 1.2, 3.0];

    let result = G24_safe::eraP2pv_safe(&p);

    match result {
        Ok(pv) => {
            vvd(pv[0][0], 0.25, 0.0, "eraP2pv_safe", "p1", status);
            vvd(pv[0][1], 1.2, 0.0, "eraP2pv_safe", "p2", status);
            vvd(pv[0][2], 3.0, 0.0, "eraP2pv_safe", "p3", status);
            vvd(pv[1][0], 0.0, 0.0, "eraP2pv_safe", "v1", status);
            vvd(pv[1][1], 0.0, 0.0, "eraP2pv_safe", "v2", status);
            vvd(pv[1][2], 0.0, 0.0, "eraP2pv_safe", "v3", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraP2pv_safe failed: unexpected error");
        }
    }
}

fn t_p2s(status: &mut i32) {
    let p = [100.0, -50.0, 25.0];

    let result = G24_safe::eraP2s_safe(&p);

    match result {
        Ok((theta, phi, r)) => {
            vvd(
                theta,
                -0.4636476090008061162,
                1e-12,
                "eraP2s_safe",
                "theta",
                status,
            );
            vvd(
                phi,
                0.2199879773954594463,
                1e-12,
                "eraP2s_safe",
                "phi",
                status,
            );
            vvd(r, 114.5643923738960002, 1e-9, "eraP2s_safe", "r", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraP2s_safe failed: unexpected error");
        }
    }
}

fn t_pap(status: &mut i32) {
    let a = [1.0, 0.1, 0.2];
    let b = [-3.0, 1e-3, 0.2];

    let result = G24_safe::eraPap_safe(&a, &b);

    match result {
        Ok(theta) => {
            vvd(
                theta,
                0.3671514267841113674,
                1e-12,
                "eraPap_safe",
                "",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraPap_safe failed: unexpected error");
        }
    }
}

fn t_pas(status: &mut i32) {
    let result = G24_safe::eraPas_safe(1.0, 0.1, 0.2, -1.0);

    match result {
        Ok(theta) => {
            vvd(
                theta,
                -2.724544922932270424,
                1e-12,
                "eraPas_safe",
                "",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraPas_safe failed: unexpected error");
        }
    }
}

fn t_pb06(status: &mut i32) {
    let result = G24_safe::eraPb06_safe(2400000.5, 50123.9999);

    match result {
        Ok((bzeta, bz, btheta)) => {
            vvd(
                bzeta,
                -0.5092634016326478238e-3,
                1e-12,
                "eraPb06_safe",
                "bzeta",
                status,
            );
            vvd(
                bz,
                -0.3602772060566044413e-3,
                1e-12,
                "eraPb06_safe",
                "bz",
                status,
            );
            vvd(
                btheta,
                -0.3779735537167811177e-3,
                1e-12,
                "eraPb06_safe",
                "btheta",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraPb06_safe failed: unexpected error");
        }
    }
}

fn t_pdp(status: &mut i32) {
    let a = [2.0, 2.0, 3.0];
    let b = [1.0, 3.0, 4.0];

    let result = G24_safe::eraPdp_safe(&a, &b);

    match result {
        Ok(adb) => {
            vvd(adb, 20.0, 1e-12, "eraPdp_safe", "", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraPdp_safe failed: unexpected error");
        }
    }
}

fn t_pfw06(status: &mut i32) {
    let result = G24_safe::eraPfw06_safe(2400000.5, 50123.9999);

    match result {
        Ok((gamb, phib, psib, epsa)) => {
            vvd(
                gamb,
                -0.2243387670997995690e-5,
                1e-16,
                "eraPfw06_safe",
                "gamb",
                status,
            );
            vvd(
                phib,
                0.4091014602391312808,
                1e-12,
                "eraPfw06_safe",
                "phib",
                status,
            );
            vvd(
                psib,
                -0.9501954178013031895e-3,
                1e-14,
                "eraPfw06_safe",
                "psib",
                status,
            );
            vvd(
                epsa,
                0.4091014316587367491,
                1e-12,
                "eraPfw06_safe",
                "epsa",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraPfw06_safe failed: unexpected error");
        }
    }
}

fn t_plan94(status: &mut i32) {
    let result = G24_safe::eraPlan94_safe(2400000.5, 1e6, 0);
    match result {
        Ok((pv, j)) => {
            vvd(pv[0][0], 0.0, 0.0, "eraPlan94_safe", "x 1", status);
            vvd(pv[0][1], 0.0, 0.0, "eraPlan94_safe", "y 1", status);
            vvd(pv[0][2], 0.0, 0.0, "eraPlan94_safe", "z 1", status);
            vvd(pv[1][0], 0.0, 0.0, "eraPlan94_safe", "xd 1", status);
            vvd(pv[1][1], 0.0, 0.0, "eraPlan94_safe", "yd 1", status);
            vvd(pv[1][2], 0.0, 0.0, "eraPlan94_safe", "zd 1", status);
            viv(j, -1, "eraPlan94_safe", "j 1", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraPlan94_safe failed: unexpected error");
        }
    }

    let result = G24_safe::eraPlan94_safe(2400000.5, 1e6, 10);
    match result {
        Ok((_, j)) => {
            viv(j, -1, "eraPlan94_safe", "j 2", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraPlan94_safe failed: unexpected error");
        }
    }

    let result = G24_safe::eraPlan94_safe(2400000.5, -320000.0, 3);
    match result {
        Ok((pv, j)) => {
            vvd(
                pv[0][0],
                0.9308038666832975759,
                1e-11,
                "eraPlan94_safe",
                "x 3",
                status,
            );
            vvd(
                pv[0][1],
                0.3258319040261346000,
                1e-11,
                "eraPlan94_safe",
                "y 3",
                status,
            );
            vvd(
                pv[0][2],
                0.1422794544481140560,
                1e-11,
                "eraPlan94_safe",
                "z 3",
                status,
            );
            vvd(
                pv[1][0],
                -0.6429458958255170006e-2,
                1e-11,
                "eraPlan94_safe",
                "xd 3",
                status,
            );
            vvd(
                pv[1][1],
                0.1468570657704237764e-1,
                1e-11,
                "eraPlan94_safe",
                "yd 3",
                status,
            );
            vvd(
                pv[1][2],
                0.6406996426270981189e-2,
                1e-11,
                "eraPlan94_safe",
                "zd 3",
                status,
            );
            viv(j, 1, "eraPlan94_safe", "j 3", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraPlan94_safe failed: unexpected error");
        }
    }

    let result = G24_safe::eraPlan94_safe(2400000.5, 43999.9, 1);
    match result {
        Ok((pv, j)) => {
            vvd(
                pv[0][0],
                0.2945293959257430832,
                1e-11,
                "eraPlan94_safe",
                "x 4",
                status,
            );
            vvd(
                pv[0][1],
                -0.2452204176601049596,
                1e-11,
                "eraPlan94_safe",
                "y 4",
                status,
            );
            vvd(
                pv[0][2],
                -0.1615427700571978153,
                1e-11,
                "eraPlan94_safe",
                "z 4",
                status,
            );
            vvd(
                pv[1][0],
                0.1413867871404614441e-1,
                1e-11,
                "eraPlan94_safe",
                "xd 4",
                status,
            );
            vvd(
                pv[1][1],
                0.1946548301104706582e-1,
                1e-11,
                "eraPlan94_safe",
                "yd 4",
                status,
            );
            vvd(
                pv[1][2],
                0.8929809783898904786e-2,
                1e-11,
                "eraPlan94_safe",
                "zd 4",
                status,
            );
            viv(j, 0, "eraPlan94_safe", "j 4", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraPlan94_safe failed: unexpected error");
        }
    }
}

fn t_pm(status: &mut i32) {
    let p = [0.3, 1.2, -2.5];

    let result = G24_safe::eraPm_safe(&p);

    match result {
        Ok(r) => {
            vvd(r, 2.789265136196270604, 1e-12, "eraPm_safe", "", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraPm_safe failed: unexpected error");
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
