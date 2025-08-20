#![allow(dead_code)]

use erfa_rust::G1_safe;
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

fn t_a2af(status: &mut i32) {
    let result = G1_safe::eraA2af_safe(4, 2.345);
    match result {
        Ok((s, idmsf)) => {
            viv(s as i32, '+' as i32, "eraA2af_safe", "s", status);
            viv(idmsf[0], 134, "eraA2af_safe", "0", status);
            viv(idmsf[1], 21, "eraA2af_safe", "1", status);
            viv(idmsf[2], 30, "eraA2af_safe", "2", status);
            viv(idmsf[3], 9706, "eraA2af_safe", "3", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraA2af_safe failed: unexpected error");
        }
    }
}

fn t_a2tf(status: &mut i32) {
    let result = G1_safe::eraA2tf_safe(4, -3.01234);
    match result {
        Ok((s, ihmsf)) => {
            viv(s as i32, '-' as i32, "eraA2tf_safe", "s", status);
            viv(ihmsf[0], 11, "eraA2tf_safe", "0", status);
            viv(ihmsf[1], 30, "eraA2tf_safe", "1", status);
            viv(ihmsf[2], 22, "eraA2tf_safe", "2", status);
            viv(ihmsf[3], 6484, "eraA2tf_safe", "3", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraA2tf_safe failed: unexpected error");
        }
    }
}

fn t_ab(status: &mut i32) {
    let pnat = [
        -0.76321968546737951,
        -0.60869453983060384,
        -0.21676408580639883,
    ];
    let v = [
        2.1044018893653786e-5,
        -8.9108923304429319e-5,
        -3.8633714797716569e-5,
    ];
    let s = 0.99980921395708788;
    let bm1 = 0.99999999506209258;

    let result = G1_safe::eraAb_safe(&pnat, &v, s, bm1);
    match result {
        Ok(ppr) => {
            vvd(
                ppr[0],
                -0.7631631094219556269,
                1e-12,
                "eraAb_safe",
                "1",
                status,
            );
            vvd(
                ppr[1],
                -0.6087553082505590832,
                1e-12,
                "eraAb_safe",
                "2",
                status,
            );
            vvd(
                ppr[2],
                -0.2167926269368471279,
                1e-12,
                "eraAb_safe",
                "3",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraAb_safe failed: unexpected error");
        }
    }
}

fn t_ae2hd(status: &mut i32) {
    let a = 5.5;
    let e = 1.1;
    let p = 0.7;

    let result = G1_safe::eraAe2hd_safe(a, e, p);
    match result {
        Ok((h, d)) => {
            vvd(
                h,
                0.5933291115507309663,
                1e-14,
                "eraAe2hd_safe",
                "h",
                status,
            );
            vvd(
                d,
                0.9613934761647817620,
                1e-14,
                "eraAe2hd_safe",
                "d",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraAe2hd_safe failed: unexpected error");
        }
    }
}

fn t_af2a(status: &mut i32) {
    let result = G1_safe::eraAf2a_safe('-', 45, 13, 27.2);
    match result {
        Ok((a, j)) => {
            vvd(
                a,
                -0.7893115794313644842,
                1e-12,
                "eraAf2a_safe",
                "a",
                status,
            );
            viv(j, 0, "eraAf2a_safe", "j", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraAf2a_safe failed: unexpected error");
        }
    }
}

fn t_anp(status: &mut i32) {
    let result = G1_safe::eraAnp_safe(-0.1);
    match result {
        Ok(val) => {
            vvd(val, 6.183185307179586477, 1e-12, "eraAnp_safe", "", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraAnp_safe failed: unexpected error");
        }
    }
}

fn t_anpm(status: &mut i32) {
    let result = G1_safe::eraAnpm_safe(-4.0);
    match result {
        Ok(val) => {
            vvd(val, 2.283185307179586477, 1e-12, "eraAnpm_safe", "", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraAnpm_safe failed: unexpected error");
        }
    }
}

fn t_apcg(status: &mut i32) {
    let date1 = 2456165.5;
    let date2 = 0.401182685;
    let ebpv = [
        [0.901310875, -0.417402664, -0.180982288],
        [0.00742727954, 0.0140507459, 0.00609045792],
    ];
    let ehp = [0.903358544, -0.415395237, -0.180084014];
    let mut astrom = H1_safe::eraASTROM::default();

    let result = G1_safe::eraApcg_safe(date1, date2, &ebpv, &ehp, &mut astrom);
    match result {
        Ok(()) => {
            vvd(
                astrom.pmt,
                12.65133794027378508,
                1e-11,
                "eraApcg_safe",
                "pmt",
                status,
            );
            vvd(
                astrom.eb[0],
                0.901310875,
                1e-12,
                "eraApcg_safe",
                "eb(1)",
                status,
            );
            vvd(
                astrom.eb[1],
                -0.417402664,
                1e-12,
                "eraApcg_safe",
                "eb(2)",
                status,
            );
            vvd(
                astrom.eb[2],
                -0.180982288,
                1e-12,
                "eraApcg_safe",
                "eb(3)",
                status,
            );
            vvd(
                astrom.eh[0],
                0.8940025429324143045,
                1e-12,
                "eraApcg_safe",
                "eh(1)",
                status,
            );
            vvd(
                astrom.eh[1],
                -0.4110930268679817955,
                1e-12,
                "eraApcg_safe",
                "eh(2)",
                status,
            );
            vvd(
                astrom.eh[2],
                -0.1782189004872870264,
                1e-12,
                "eraApcg_safe",
                "eh(3)",
                status,
            );
            vvd(
                astrom.em,
                1.010465295811013146,
                1e-12,
                "eraApcg_safe",
                "em",
                status,
            );
            vvd(
                astrom.v[0],
                0.4289638913597693554e-4,
                1e-16,
                "eraApcg_safe",
                "v(1)",
                status,
            );
            vvd(
                astrom.v[1],
                0.8115034051581320575e-4,
                1e-16,
                "eraApcg_safe",
                "v(2)",
                status,
            );
            vvd(
                astrom.v[2],
                0.3517555136380563427e-4,
                1e-16,
                "eraApcg_safe",
                "v(3)",
                status,
            );
            vvd(
                astrom.bm1,
                0.9999999951686012981,
                1e-12,
                "eraApcg_safe",
                "bm1",
                status,
            );
            vvd(
                astrom.bpn[0][0],
                1.0,
                0.0,
                "eraApcg_safe",
                "bpn(1,1)",
                status,
            );
            vvd(
                astrom.bpn[1][0],
                0.0,
                0.0,
                "eraApcg_safe",
                "bpn(2,1)",
                status,
            );
            vvd(
                astrom.bpn[2][0],
                0.0,
                0.0,
                "eraApcg_safe",
                "bpn(3,1)",
                status,
            );
            vvd(
                astrom.bpn[0][1],
                0.0,
                0.0,
                "eraApcg_safe",
                "bpn(1,2)",
                status,
            );
            vvd(
                astrom.bpn[1][1],
                1.0,
                0.0,
                "eraApcg_safe",
                "bpn(2,2)",
                status,
            );
            vvd(
                astrom.bpn[2][1],
                0.0,
                0.0,
                "eraApcg_safe",
                "bpn(3,2)",
                status,
            );
            vvd(
                astrom.bpn[0][2],
                0.0,
                0.0,
                "eraApcg_safe",
                "bpn(1,3)",
                status,
            );
            vvd(
                astrom.bpn[1][2],
                0.0,
                0.0,
                "eraApcg_safe",
                "bpn(2,3)",
                status,
            );
            vvd(
                astrom.bpn[2][2],
                1.0,
                0.0,
                "eraApcg_safe",
                "bpn(3,3)",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraApcg_safe failed: unexpected error");
        }
    }
}

fn t_apcg13(status: &mut i32) {
    let date1 = 2456165.5;
    let date2 = 0.401182685;
    let mut astrom = H1_safe::eraASTROM::default();

    let result = G1_safe::eraApcg13_safe(date1, date2, &mut astrom);
    match result {
        Ok(()) => {
            vvd(
                astrom.pmt,
                12.65133794027378508,
                1e-11,
                "eraApcg13_safe",
                "pmt",
                status,
            );
            vvd(
                astrom.eb[0],
                0.9013108747340644755,
                1e-12,
                "eraApcg13_safe",
                "eb(1)",
                status,
            );
            vvd(
                astrom.eb[1],
                -0.4174026640406119957,
                1e-12,
                "eraApcg13_safe",
                "eb(2)",
                status,
            );
            vvd(
                astrom.eb[2],
                -0.1809822877867817771,
                1e-12,
                "eraApcg13_safe",
                "eb(3)",
                status,
            );
            vvd(
                astrom.eh[0],
                0.8940025429255499549,
                1e-12,
                "eraApcg13_safe",
                "eh(1)",
                status,
            );
            vvd(
                astrom.eh[1],
                -0.4110930268331896318,
                1e-12,
                "eraApcg13_safe",
                "eh(2)",
                status,
            );
            vvd(
                astrom.eh[2],
                -0.1782189006019749850,
                1e-12,
                "eraApcg13_safe",
                "eh(3)",
                status,
            );
            vvd(
                astrom.em,
                1.010465295964664178,
                1e-12,
                "eraApcg13_safe",
                "em",
                status,
            );
            vvd(
                astrom.v[0],
                0.4289638912941341125e-4,
                1e-16,
                "eraApcg13_safe",
                "v(1)",
                status,
            );
            vvd(
                astrom.v[1],
                0.8115034032405042132e-4,
                1e-16,
                "eraApcg13_safe",
                "v(2)",
                status,
            );
            vvd(
                astrom.v[2],
                0.3517555135536470279e-4,
                1e-16,
                "eraApcg13_safe",
                "v(3)",
                status,
            );
            vvd(
                astrom.bm1,
                0.9999999951686013142,
                1e-12,
                "eraApcg13_safe",
                "bm1",
                status,
            );
            vvd(
                astrom.bpn[0][0],
                1.0,
                0.0,
                "eraApcg13_safe",
                "bpn(1,1)",
                status,
            );
            vvd(
                astrom.bpn[1][0],
                0.0,
                0.0,
                "eraApcg13_safe",
                "bpn(2,1)",
                status,
            );
            vvd(
                astrom.bpn[2][0],
                0.0,
                0.0,
                "eraApcg13_safe",
                "bpn(3,1)",
                status,
            );
            vvd(
                astrom.bpn[0][1],
                0.0,
                0.0,
                "eraApcg13_safe",
                "bpn(1,2)",
                status,
            );
            vvd(
                astrom.bpn[1][1],
                1.0,
                0.0,
                "eraApcg13_safe",
                "bpn(2,2)",
                status,
            );
            vvd(
                astrom.bpn[2][1],
                0.0,
                0.0,
                "eraApcg13_safe",
                "bpn(3,2)",
                status,
            );
            vvd(
                astrom.bpn[0][2],
                0.0,
                0.0,
                "eraApcg13_safe",
                "bpn(1,3)",
                status,
            );
            vvd(
                astrom.bpn[1][2],
                0.0,
                0.0,
                "eraApcg13_safe",
                "bpn(2,3)",
                status,
            );
            vvd(
                astrom.bpn[2][2],
                1.0,
                0.0,
                "eraApcg13_safe",
                "bpn(3,3)",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraApcg13_safe failed: unexpected error");
        }
    }
}

fn t_apci(status: &mut i32) {
    let date1 = 2456165.5;
    let date2 = 0.401182685;
    let ebpv = [
        [0.901310875, -0.417402664, -0.180982288],
        [0.00742727954, 0.0140507459, 0.00609045792],
    ];
    let ehp = [0.903358544, -0.415395237, -0.180084014];
    let x = 0.0013122272;
    let y = -2.92808623e-5;
    let s = 3.05749468e-8;
    let mut astrom = H1_safe::eraASTROM::default();

    let result = G1_safe::eraApci_safe(date1, date2, &ebpv, &ehp, x, y, s, &mut astrom);
    match result {
        Ok(()) => {
            vvd(
                astrom.pmt,
                12.65133794027378508,
                1e-11,
                "eraApci_safe",
                "pmt",
                status,
            );
            vvd(
                astrom.eb[0],
                0.901310875,
                1e-12,
                "eraApci_safe",
                "eb(1)",
                status,
            );
            vvd(
                astrom.eb[1],
                -0.417402664,
                1e-12,
                "eraApci_safe",
                "eb(2)",
                status,
            );
            vvd(
                astrom.eb[2],
                -0.180982288,
                1e-12,
                "eraApci_safe",
                "eb(3)",
                status,
            );
            vvd(
                astrom.eh[0],
                0.8940025429324143045,
                1e-12,
                "eraApci_safe",
                "eh(1)",
                status,
            );
            vvd(
                astrom.eh[1],
                -0.4110930268679817955,
                1e-12,
                "eraApci_safe",
                "eh(2)",
                status,
            );
            vvd(
                astrom.eh[2],
                -0.1782189004872870264,
                1e-12,
                "eraApci_safe",
                "eh(3)",
                status,
            );
            vvd(
                astrom.em,
                1.010465295811013146,
                1e-12,
                "eraApci_safe",
                "em",
                status,
            );
            vvd(
                astrom.v[0],
                0.4289638913597693554e-4,
                1e-16,
                "eraApci_safe",
                "v(1)",
                status,
            );
            vvd(
                astrom.v[1],
                0.8115034051581320575e-4,
                1e-16,
                "eraApci_safe",
                "v(2)",
                status,
            );
            vvd(
                astrom.v[2],
                0.3517555136380563427e-4,
                1e-16,
                "eraApci_safe",
                "v(3)",
                status,
            );
            vvd(
                astrom.bm1,
                0.9999999951686012981,
                1e-12,
                "eraApci_safe",
                "bm1",
                status,
            );
            vvd(
                astrom.bpn[0][0],
                0.9999991390295159156,
                1e-12,
                "eraApci_safe",
                "bpn(1,1)",
                status,
            );
            vvd(
                astrom.bpn[1][0],
                0.4978650072505016932e-7,
                1e-12,
                "eraApci_safe",
                "bpn(2,1)",
                status,
            );
            vvd(
                astrom.bpn[2][0],
                0.1312227200000000000e-2,
                1e-12,
                "eraApci_safe",
                "bpn(3,1)",
                status,
            );
            vvd(
                astrom.bpn[0][1],
                -0.1136336653771609630e-7,
                1e-12,
                "eraApci_safe",
                "bpn(1,2)",
                status,
            );
            vvd(
                astrom.bpn[1][1],
                0.9999999995713154868,
                1e-12,
                "eraApci_safe",
                "bpn(2,2)",
                status,
            );
            vvd(
                astrom.bpn[2][1],
                -0.2928086230000000000e-4,
                1e-12,
                "eraApci_safe",
                "bpn(3,2)",
                status,
            );
            vvd(
                astrom.bpn[0][2],
                -0.1312227200895260194e-2,
                1e-12,
                "eraApci_safe",
                "bpn(1,3)",
                status,
            );
            vvd(
                astrom.bpn[1][2],
                0.2928082217872315680e-4,
                1e-12,
                "eraApci_safe",
                "bpn(2,3)",
                status,
            );
            vvd(
                astrom.bpn[2][2],
                0.9999991386008323373,
                1e-12,
                "eraApci_safe",
                "bpn(3,3)",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraApci_safe failed: unexpected error");
        }
    }
}

fn t_apci13(status: &mut i32) {
    let date1 = 2456165.5;
    let date2 = 0.401182685;
    let mut astrom = H1_safe::eraASTROM::default();

    let result = G1_safe::eraApci13_safe(date1, date2, &mut astrom);
    match result {
        Ok(eo) => {
            vvd(
                astrom.pmt,
                12.65133794027378508,
                1e-11,
                "eraApci13_safe",
                "pmt",
                status,
            );
            vvd(
                astrom.eb[0],
                0.9013108747340644755,
                1e-12,
                "eraApci13_safe",
                "eb(1)",
                status,
            );
            vvd(
                astrom.eb[1],
                -0.4174026640406119957,
                1e-12,
                "eraApci13_safe",
                "eb(2)",
                status,
            );
            vvd(
                astrom.eb[2],
                -0.1809822877867817771,
                1e-12,
                "eraApci13_safe",
                "eb(3)",
                status,
            );
            vvd(
                astrom.eh[0],
                0.8940025429255499549,
                1e-12,
                "eraApci13_safe",
                "eh(1)",
                status,
            );
            vvd(
                astrom.eh[1],
                -0.4110930268331896318,
                1e-12,
                "eraApci13_safe",
                "eh(2)",
                status,
            );
            vvd(
                astrom.eh[2],
                -0.1782189006019749850,
                1e-12,
                "eraApci13_safe",
                "eh(3)",
                status,
            );
            vvd(
                astrom.em,
                1.010465295964664178,
                1e-12,
                "eraApci13_safe",
                "em",
                status,
            );
            vvd(
                astrom.v[0],
                0.4289638912941341125e-4,
                1e-16,
                "eraApci13_safe",
                "v(1)",
                status,
            );
            vvd(
                astrom.v[1],
                0.8115034032405042132e-4,
                1e-16,
                "eraApci13_safe",
                "v(2)",
                status,
            );
            vvd(
                astrom.v[2],
                0.3517555135536470279e-4,
                1e-16,
                "eraApci13_safe",
                "v(3)",
                status,
            );
            vvd(
                astrom.bm1,
                0.9999999951686013142,
                1e-12,
                "eraApci13_safe",
                "bm1",
                status,
            );
            vvd(
                astrom.bpn[0][0],
                0.9999992060376761710,
                1e-12,
                "eraApci13_safe",
                "bpn(1,1)",
                status,
            );
            vvd(
                astrom.bpn[1][0],
                0.4124244860106037157e-7,
                1e-12,
                "eraApci13_safe",
                "bpn(2,1)",
                status,
            );
            vvd(
                astrom.bpn[2][0],
                0.1260128571051709670e-2,
                1e-12,
                "eraApci13_safe",
                "bpn(3,1)",
                status,
            );
            vvd(
                astrom.bpn[0][1],
                -0.1282291987222130690e-7,
                1e-12,
                "eraApci13_safe",
                "bpn(1,2)",
                status,
            );
            vvd(
                astrom.bpn[1][1],
                0.9999999997456835325,
                1e-12,
                "eraApci13_safe",
                "bpn(2,2)",
                status,
            );
            vvd(
                astrom.bpn[2][1],
                -0.2255288829420524935e-4,
                1e-12,
                "eraApci13_safe",
                "bpn(3,2)",
                status,
            );
            vvd(
                astrom.bpn[0][2],
                -0.1260128571661374559e-2,
                1e-12,
                "eraApci13_safe",
                "bpn(1,3)",
                status,
            );
            vvd(
                astrom.bpn[1][2],
                0.2255285422953395494e-4,
                1e-12,
                "eraApci13_safe",
                "bpn(2,3)",
                status,
            );
            vvd(
                astrom.bpn[2][2],
                0.9999992057833604343,
                1e-12,
                "eraApci13_safe",
                "bpn(3,3)",
                status,
            );
            vvd(
                eo,
                -0.2900618712657375647e-2,
                1e-12,
                "eraApci13_safe",
                "eo",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraApci13_safe failed: unexpected error");
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

    t_a2af(&mut status);
    t_a2tf(&mut status);
    t_ab(&mut status);
    t_ae2hd(&mut status);
    t_af2a(&mut status);
    t_anp(&mut status);
    t_anpm(&mut status);
    t_apcg(&mut status);
    t_apcg13(&mut status);
    t_apci(&mut status);
    t_apci13(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
