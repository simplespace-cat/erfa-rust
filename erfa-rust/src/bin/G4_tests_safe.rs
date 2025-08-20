#![allow(dead_code)]

use erfa_rust::G1_safe;
use erfa_rust::G3_safe;
use erfa_rust::G4_safe;
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

fn t_atci13(status: &mut i32) {
    let rc = 2.71;
    let dc = 0.174;
    let pr = 1e-5;
    let pd = 5e-6;
    let px = 0.1;
    let rv = 55.0;
    let date1 = 2456165.5;
    let date2 = 0.401182685;

    let result = G4_safe::eraAtci13_safe(rc, dc, pr, pd, px, rv, date1, date2);

    match result {
        Ok((ri, di, eo)) => {
            vvd(
                ri,
                2.710121572968696744,
                1e-12,
                "eraAtci13_safe",
                "ri",
                status,
            );
            vvd(
                di,
                0.1729371367219539137,
                1e-12,
                "eraAtci13_safe",
                "di",
                status,
            );
            vvd(
                eo,
                -0.002900618712657375647,
                1e-14,
                "eraAtci13_safe",
                "eo",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraAtci13_safe failed: unexpected error");
        }
    }
}

fn t_atciq(status: &mut i32) {
    let date1 = 2456165.5;
    let date2 = 0.401182685;
    let rc = 2.71;
    let dc = 0.174;
    let pr = 1e-5;
    let pd = 5e-6;
    let px = 0.1;
    let rv = 55.0;
    let mut astrom = H1_safe::eraASTROM::default();

    let eo_result = G1_safe::eraApci13_safe(date1, date2, &mut astrom);
    if eo_result.is_err() {
        *status = 1;
        println!("eraApci13_safe failed in t_atciq");
        return;
    }

    let result = G4_safe::eraAtciq_safe(rc, dc, pr, pd, px, rv, &astrom);

    match result {
        Ok((ri, di)) => {
            vvd(
                ri,
                2.710121572968696744,
                1e-12,
                "eraAtciq_safe",
                "ri",
                status,
            );
            vvd(
                di,
                0.1729371367219539137,
                1e-12,
                "eraAtciq_safe",
                "di",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraAtciq_safe failed: unexpected error");
        }
    }
}

fn t_atciqn(status: &mut i32) {
    let mut b: [H1_safe::eraLDBODY; 3] = [H1_safe::eraLDBODY::default(); 3];
    let date1 = 2456165.5;
    let date2 = 0.401182685;
    let rc = 2.71;
    let dc = 0.174;
    let pr = 1e-5;
    let pd = 5e-6;
    let px = 0.1;
    let rv = 55.0;
    let mut astrom = H1_safe::eraASTROM::default();

    let eo_result = G1_safe::eraApci13_safe(date1, date2, &mut astrom);
    if eo_result.is_err() {
        *status = 1;
        println!("eraApci13_safe failed in t_atciqn");
        return;
    }

    b[0].bm = 0.00028574;
    b[0].dl = 3e-10;
    b[0].pv[0][0] = -7.81014427;
    b[0].pv[0][1] = -5.60956681;
    b[0].pv[0][2] = -1.98079819;
    b[0].pv[1][0] = 0.0030723249;
    b[0].pv[1][1] = -0.00406995477;
    b[0].pv[1][2] = -0.00181335842;
    b[1].bm = 0.00095435;
    b[1].dl = 3e-9;
    b[1].pv[0][0] = 0.738098796;
    b[1].pv[0][1] = 4.63658692;
    b[1].pv[0][2] = 1.9693136;
    b[1].pv[1][0] = -0.00755816922;
    b[1].pv[1][1] = 0.00126913722;
    b[1].pv[1][2] = 0.000727999001;
    b[2].bm = 1.0;
    b[2].dl = 6e-6;
    b[2].pv[0][0] = -0.000712174377;
    b[2].pv[0][1] = -0.00230478303;
    b[2].pv[0][2] = -0.00105865966;
    b[2].pv[1][0] = 6.29235213e-6;
    b[2].pv[1][1] = -3.30888387e-7;
    b[2].pv[1][2] = -2.96486623e-7;

    let result = G4_safe::eraAtciqn_safe(rc, dc, pr, pd, px, rv, &astrom, &b);

    match result {
        Ok((ri, di)) => {
            vvd(
                ri,
                2.710122008104983335,
                1e-12,
                "eraAtciqn_safe",
                "ri",
                status,
            );
            vvd(
                di,
                0.1729371916492767821,
                1e-12,
                "eraAtciqn_safe",
                "di",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraAtciqn_safe failed: unexpected error");
        }
    }
}

fn t_atciqz(status: &mut i32) {
    let date1 = 2456165.5;
    let date2 = 0.401182685;
    let rc = 2.71;
    let dc = 0.174;
    let mut astrom = H1_safe::eraASTROM::default();

    let eo_result = G1_safe::eraApci13_safe(date1, date2, &mut astrom);
    if eo_result.is_err() {
        *status = 1;
        println!("eraApci13_safe failed in t_atciqz");
        return;
    }

    let result = G4_safe::eraAtciqz_safe(rc, dc, &astrom);

    match result {
        Ok((ri, di)) => {
            vvd(
                ri,
                2.709994899247256984,
                1e-12,
                "eraAtciqz_safe",
                "ri",
                status,
            );
            vvd(
                di,
                0.1728740720984931891,
                1e-12,
                "eraAtciqz_safe",
                "di",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraAtciqz_safe failed: unexpected error");
        }
    }
}

fn t_atco13(status: &mut i32) {
    let rc = 2.71;
    let dc = 0.174;
    let pr = 1e-5;
    let pd = 5e-6;
    let px = 0.1;
    let rv = 55.0;
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

    let result = G4_safe::eraAtco13_safe(
        rc, dc, pr, pd, px, rv, utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, wl,
    );

    match result {
        Ok((aob, zob, hob, dob, rob, eo, j)) => {
            vvd(
                aob,
                0.9251774485485515207e-1,
                1e-12,
                "eraAtco13_safe",
                "aob",
                status,
            );
            vvd(
                zob,
                1.407661405256499357,
                1e-12,
                "eraAtco13_safe",
                "zob",
                status,
            );
            vvd(
                hob,
                -0.9265154431529724692e-1,
                1e-12,
                "eraAtco13_safe",
                "hob",
                status,
            );
            vvd(
                dob,
                0.1716626560072526200,
                1e-12,
                "eraAtco13_safe",
                "dob",
                status,
            );
            vvd(
                rob,
                2.710260453504961012,
                1e-12,
                "eraAtco13_safe",
                "rob",
                status,
            );
            vvd(
                eo,
                -0.003020548354802412839,
                1e-14,
                "eraAtco13_safe",
                "eo",
                status,
            );
            viv(j, 0, "eraAtco13_safe", "j", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraAtco13_safe failed: unexpected error");
        }
    }
}

fn t_atic13(status: &mut i32) {
    let ri = 2.710121572969038991;
    let di = 0.1729371367218230438;
    let date1 = 2456165.5;
    let date2 = 0.401182685;

    let result = G4_safe::eraAtic13_safe(ri, di, date1, date2);

    match result {
        Ok((rc, dc, eo)) => {
            vvd(
                rc,
                2.710126504531716819,
                1e-12,
                "eraAtic13_safe",
                "rc",
                status,
            );
            vvd(
                dc,
                0.1740632537627034482,
                1e-12,
                "eraAtic13_safe",
                "dc",
                status,
            );
            vvd(
                eo,
                -0.002900618712657375647,
                1e-14,
                "eraAtic13_safe",
                "eo",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraAtic13_safe failed: unexpected error");
        }
    }
}

fn t_aticq(status: &mut i32) {
    let date1 = 2456165.5;
    let date2 = 0.401182685;
    let ri = 2.710121572969038991;
    let di = 0.1729371367218230438;
    let mut astrom = H1_safe::eraASTROM::default();

    let eo_result = G1_safe::eraApci13_safe(date1, date2, &mut astrom);
    if eo_result.is_err() {
        *status = 1;
        println!("eraApci13_safe failed in t_aticq");
        return;
    }

    let result = G4_safe::eraAticq_safe(ri, di, &astrom);

    match result {
        Ok((rc, dc)) => {
            vvd(
                rc,
                2.710126504531716819,
                1e-12,
                "eraAticq_safe",
                "rc",
                status,
            );
            vvd(
                dc,
                0.1740632537627034482,
                1e-12,
                "eraAticq_safe",
                "dc",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraAticq_safe failed: unexpected error");
        }
    }
}

fn t_aticqn(status: &mut i32) {
    let date1 = 2456165.5;
    let date2 = 0.401182685;
    let ri = 2.709994899247599271;
    let di = 0.1728740720983623469;
    let mut b: [H1_safe::eraLDBODY; 3] = [H1_safe::eraLDBODY::default(); 3];
    let mut astrom = H1_safe::eraASTROM::default();

    let eo_result = G1_safe::eraApci13_safe(date1, date2, &mut astrom);
    if eo_result.is_err() {
        *status = 1;
        println!("eraApci13_safe failed in t_aticqn");
        return;
    }

    b[0].bm = 0.00028574;
    b[0].dl = 3e-10;
    b[0].pv[0][0] = -7.81014427;
    b[0].pv[0][1] = -5.60956681;
    b[0].pv[0][2] = -1.98079819;
    b[0].pv[1][0] = 0.0030723249;
    b[0].pv[1][1] = -0.00406995477;
    b[0].pv[1][2] = -0.00181335842;
    b[1].bm = 0.00095435;
    b[1].dl = 3e-9;
    b[1].pv[0][0] = 0.738098796;
    b[1].pv[0][1] = 4.63658692;
    b[1].pv[0][2] = 1.9693136;
    b[1].pv[1][0] = -0.00755816922;
    b[1].pv[1][1] = 0.00126913722;
    b[1].pv[1][2] = 0.000727999001;
    b[2].bm = 1.0;
    b[2].dl = 6e-6;
    b[2].pv[0][0] = -0.000712174377;
    b[2].pv[0][1] = -0.00230478303;
    b[2].pv[0][2] = -0.00105865966;
    b[2].pv[1][0] = 6.29235213e-6;
    b[2].pv[1][1] = -3.30888387e-7;
    b[2].pv[1][2] = -2.96486623e-7;

    let result = G4_safe::eraAticqn_safe(ri, di, &astrom, &b);

    match result {
        Ok((rc, dc)) => {
            vvd(
                rc,
                2.709999575033027333,
                1e-12,
                "eraAticqn_safe",
                "rc",
                status,
            );
            vvd(
                dc,
                0.1739999656316469990,
                1e-12,
                "eraAticqn_safe",
                "dc",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraAticqn_safe failed: unexpected error");
        }
    }
}

fn t_atio13(status: &mut i32) {
    let ri = 2.710121572969038991;
    let di = 0.1729371367218230438;
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

    let result = G4_safe::eraAtio13_safe(
        ri, di, utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, wl,
    );

    match result {
        Ok((aob, zob, hob, dob, rob, j)) => {
            vvd(
                aob,
                0.9233952224895122499e-1,
                1e-12,
                "eraAtio13_safe",
                "aob",
                status,
            );
            vvd(
                zob,
                1.407758704513549991,
                1e-12,
                "eraAtio13_safe",
                "zob",
                status,
            );
            vvd(
                hob,
                -0.9247619879881698140e-1,
                1e-12,
                "eraAtio13_safe",
                "hob",
                status,
            );
            vvd(
                dob,
                0.1717653435756234676,
                1e-12,
                "eraAtio13_safe",
                "dob",
                status,
            );
            vvd(
                rob,
                2.710085107988480746,
                1e-12,
                "eraAtio13_safe",
                "rob",
                status,
            );
            viv(j, 0, "eraAtio13_safe", "j", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraAtio13_safe failed: unexpected error");
        }
    }
}

fn t_atioq(status: &mut i32) {
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
    let ri = 2.710121572969038991;
    let di = 0.1729371367218230438;
    let mut astrom = H1_safe::eraASTROM::default();

    let j_result = G3_safe::eraApio13_safe(
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
    if j_result.is_err() {
        *status = 1;
        println!("eraApio13_safe failed in t_atioq");
        return;
    }

    let result = G4_safe::eraAtioq_safe(ri, di, &astrom);

    match result {
        Ok((aob, zob, hob, dob, rob)) => {
            vvd(
                aob,
                0.9233952224895122499e-1,
                1e-12,
                "eraAtioq_safe",
                "aob",
                status,
            );
            vvd(
                zob,
                1.407758704513549991,
                1e-12,
                "eraAtioq_safe",
                "zob",
                status,
            );
            vvd(
                hob,
                -0.9247619879881698140e-1,
                1e-12,
                "eraAtioq_safe",
                "hob",
                status,
            );
            vvd(
                dob,
                0.1717653435756234676,
                1e-12,
                "eraAtioq_safe",
                "dob",
                status,
            );
            vvd(
                rob,
                2.710085107988480746,
                1e-12,
                "eraAtioq_safe",
                "rob",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraAtioq_safe failed: unexpected error");
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

    t_atci13(&mut status);
    t_atciq(&mut status);
    t_atciqn(&mut status);
    t_atciqz(&mut status);
    t_atco13(&mut status);
    t_atic13(&mut status);
    t_aticq(&mut status);
    t_aticqn(&mut status);
    t_atio13(&mut status);
    t_atioq(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
