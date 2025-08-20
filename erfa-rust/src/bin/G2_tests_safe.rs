#![allow(dead_code)]

use erfa_rust::G2_safe;
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

fn t_apco(status: &mut i32) {
    let date1 = 2456384.5;
    let date2 = 0.970031644;
    let ebpv = [
        [-0.974170438, -0.211520082, -0.0917583024],
        [0.00364365824, -0.0154287319, -0.00668922024],
    ];
    let ehp = [-0.973458265, -0.209215307, -0.0906996477];
    let x = 0.0013122272;
    let y = -2.92808623e-5;
    let s = 3.05749468e-8;
    let theta = 3.14540971;
    let elong = -0.527800806;
    let phi = -1.2345856;
    let hm = 2738.0;
    let xp = 2.47230737e-7;
    let yp = 1.82640464e-6;
    let sp = -3.01974337e-11;
    let refa = 0.000201418779;
    let refb = -2.36140831e-7;
    let mut astrom = H1_safe::eraASTROM::default();

    let result = G2_safe::eraApco_safe(
        date1,
        date2,
        &ebpv,
        &ehp,
        x,
        y,
        s,
        theta,
        elong,
        phi,
        hm,
        xp,
        yp,
        sp,
        refa,
        refb,
        &mut astrom,
    );

    match result {
        Ok(()) => {
            vvd(
                astrom.pmt,
                13.25248468622587269,
                1e-11,
                "eraApco_safe",
                "pmt",
                status,
            );
            vvd(
                astrom.eb[0],
                -0.9741827110630322720,
                1e-12,
                "eraApco_safe",
                "eb(1)",
                status,
            );
            vvd(
                astrom.eb[1],
                -0.2115130190135344832,
                1e-12,
                "eraApco_safe",
                "eb(2)",
                status,
            );
            vvd(
                astrom.eb[2],
                -0.09179840186949532298,
                1e-12,
                "eraApco_safe",
                "eb(3)",
                status,
            );
            vvd(
                astrom.eh[0],
                -0.9736425571689739035,
                1e-12,
                "eraApco_safe",
                "eh(1)",
                status,
            );
            vvd(
                astrom.eh[1],
                -0.2092452125849330936,
                1e-12,
                "eraApco_safe",
                "eh(2)",
                status,
            );
            vvd(
                astrom.eh[2],
                -0.09075578152243272599,
                1e-12,
                "eraApco_safe",
                "eh(3)",
                status,
            );
            vvd(
                astrom.em,
                0.9998233241709957653,
                1e-12,
                "eraApco_safe",
                "em",
                status,
            );
            vvd(
                astrom.v[0],
                0.2078704992916728762e-4,
                1e-16,
                "eraApco_safe",
                "v(1)",
                status,
            );
            vvd(
                astrom.v[1],
                -0.8955360107151952319e-4,
                1e-16,
                "eraApco_safe",
                "v(2)",
                status,
            );
            vvd(
                astrom.v[2],
                -0.3863338994288951082e-4,
                1e-16,
                "eraApco_safe",
                "v(3)",
                status,
            );
            vvd(
                astrom.bm1,
                0.9999999950277561236,
                1e-12,
                "eraApco_safe",
                "bm1",
                status,
            );
            vvd(
                astrom.bpn[0][0],
                0.9999991390295159156,
                1e-12,
                "eraApco_safe",
                "bpn(1,1)",
                status,
            );
            vvd(
                astrom.bpn[1][0],
                0.4978650072505016932e-7,
                1e-12,
                "eraApco_safe",
                "bpn(2,1)",
                status,
            );
            vvd(
                astrom.bpn[2][0],
                0.1312227200000000000e-2,
                1e-12,
                "eraApco_safe",
                "bpn(3,1)",
                status,
            );
            vvd(
                astrom.bpn[0][1],
                -0.1136336653771609630e-7,
                1e-12,
                "eraApco_safe",
                "bpn(1,2)",
                status,
            );
            vvd(
                astrom.bpn[1][1],
                0.9999999995713154868,
                1e-12,
                "eraApco_safe",
                "bpn(2,2)",
                status,
            );
            vvd(
                astrom.bpn[2][1],
                -0.2928086230000000000e-4,
                1e-12,
                "eraApco_safe",
                "bpn(3,2)",
                status,
            );
            vvd(
                astrom.bpn[0][2],
                -0.1312227200895260194e-2,
                1e-12,
                "eraApco_safe",
                "bpn(1,3)",
                status,
            );
            vvd(
                astrom.bpn[1][2],
                0.2928082217872315680e-4,
                1e-12,
                "eraApco_safe",
                "bpn(2,3)",
                status,
            );
            vvd(
                astrom.bpn[2][2],
                0.9999991386008323373,
                1e-12,
                "eraApco_safe",
                "bpn(3,3)",
                status,
            );
            vvd(
                astrom.along,
                -0.5278008060295995734,
                1e-12,
                "eraApco_safe",
                "along",
                status,
            );
            vvd(
                astrom.xpl,
                0.1133427418130752958e-5,
                1e-17,
                "eraApco_safe",
                "xpl",
                status,
            );
            vvd(
                astrom.ypl,
                0.1453347595780646207e-5,
                1e-17,
                "eraApco_safe",
                "ypl",
                status,
            );
            vvd(
                astrom.sphi,
                -0.9440115679003211329,
                1e-12,
                "eraApco_safe",
                "sphi",
                status,
            );
            vvd(
                astrom.cphi,
                0.3299123514971474711,
                1e-12,
                "eraApco_safe",
                "cphi",
                status,
            );
            vvd(astrom.diurab, 0.0, 0.0, "eraApco_safe", "diurab", status);
            vvd(
                astrom.eral,
                2.617608903970400427,
                1e-12,
                "eraApco_safe",
                "eral",
                status,
            );
            vvd(
                astrom.refa,
                0.2014187790000000000e-3,
                1e-15,
                "eraApco_safe",
                "refa",
                status,
            );
            vvd(
                astrom.refb,
                -0.2361408310000000000e-6,
                1e-18,
                "eraApco_safe",
                "refb",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraApco_safe failed: unexpected error");
        }
    }
}

fn t_apco13(status: &mut i32) {
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
    let mut astrom = H1_safe::eraASTROM::default();

    let result = G2_safe::eraApco13_safe(
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

    match result {
        Ok((eo, j)) => {
            vvd(
                astrom.pmt,
                13.25248468622475727,
                1e-11,
                "eraApco13_safe",
                "pmt",
                status,
            );
            vvd(
                astrom.eb[0],
                -0.9741827107320875162,
                1e-12,
                "eraApco13_safe",
                "eb(1)",
                status,
            );
            vvd(
                astrom.eb[1],
                -0.2115130190489716682,
                1e-12,
                "eraApco13_safe",
                "eb(2)",
                status,
            );
            vvd(
                astrom.eb[2],
                -0.09179840189496755339,
                1e-12,
                "eraApco13_safe",
                "eb(3)",
                status,
            );
            vvd(
                astrom.eh[0],
                -0.9736425572586935247,
                1e-12,
                "eraApco13_safe",
                "eh(1)",
                status,
            );
            vvd(
                astrom.eh[1],
                -0.2092452121603336166,
                1e-12,
                "eraApco13_safe",
                "eh(2)",
                status,
            );
            vvd(
                astrom.eh[2],
                -0.09075578153885665295,
                1e-12,
                "eraApco13_safe",
                "eh(3)",
                status,
            );
            vvd(
                astrom.em,
                0.9998233240913898141,
                1e-12,
                "eraApco13_safe",
                "em",
                status,
            );
            vvd(
                astrom.v[0],
                0.2078704994520489246e-4,
                1e-16,
                "eraApco13_safe",
                "v(1)",
                status,
            );
            vvd(
                astrom.v[1],
                -0.8955360133238868938e-4,
                1e-16,
                "eraApco13_safe",
                "v(2)",
                status,
            );
            vvd(
                astrom.v[2],
                -0.3863338993055887398e-4,
                1e-16,
                "eraApco13_safe",
                "v(3)",
                status,
            );
            vvd(
                astrom.bm1,
                0.9999999950277561004,
                1e-12,
                "eraApco13_safe",
                "bm1",
                status,
            );
            vvd(
                astrom.bpn[0][0],
                0.9999991390295147999,
                1e-12,
                "eraApco13_safe",
                "bpn(1,1)",
                status,
            );
            vvd(
                astrom.bpn[1][0],
                0.4978650075315529277e-7,
                1e-12,
                "eraApco13_safe",
                "bpn(2,1)",
                status,
            );
            vvd(
                astrom.bpn[2][0],
                0.001312227200850293372,
                1e-12,
                "eraApco13_safe",
                "bpn(3,1)",
                status,
            );
            vvd(
                astrom.bpn[0][1],
                -0.1136336652812486604e-7,
                1e-12,
                "eraApco13_safe",
                "bpn(1,2)",
                status,
            );
            vvd(
                astrom.bpn[1][1],
                0.9999999995713154865,
                1e-12,
                "eraApco13_safe",
                "bpn(2,2)",
                status,
            );
            vvd(
                astrom.bpn[2][1],
                -0.2928086230975367296e-4,
                1e-12,
                "eraApco13_safe",
                "bpn(3,2)",
                status,
            );
            vvd(
                astrom.bpn[0][2],
                -0.001312227201745553566,
                1e-12,
                "eraApco13_safe",
                "bpn(1,3)",
                status,
            );
            vvd(
                astrom.bpn[1][2],
                0.2928082218847679162e-4,
                1e-12,
                "eraApco13_safe",
                "bpn(2,3)",
                status,
            );
            vvd(
                astrom.bpn[2][2],
                0.9999991386008312212,
                1e-12,
                "eraApco13_safe",
                "bpn(3,3)",
                status,
            );
            vvd(
                astrom.along,
                -0.5278008060295995733,
                1e-12,
                "eraApco13_safe",
                "along",
                status,
            );
            vvd(
                astrom.xpl,
                0.1133427418130752958e-5,
                1e-17,
                "eraApco13_safe",
                "xpl",
                status,
            );
            vvd(
                astrom.ypl,
                0.1453347595780646207e-5,
                1e-17,
                "eraApco13_safe",
                "ypl",
                status,
            );
            vvd(
                astrom.sphi,
                -0.9440115679003211329,
                1e-12,
                "eraApco13_safe",
                "sphi",
                status,
            );
            vvd(
                astrom.cphi,
                0.3299123514971474711,
                1e-12,
                "eraApco13_safe",
                "cphi",
                status,
            );
            vvd(astrom.diurab, 0.0, 0.0, "eraApco13_safe", "diurab", status);
            vvd(
                astrom.eral,
                2.617608909189664000,
                1e-12,
                "eraApco13_safe",
                "eral",
                status,
            );
            vvd(
                astrom.refa,
                0.2014187785940396921e-3,
                1e-15,
                "eraApco13_safe",
                "refa",
                status,
            );
            vvd(
                astrom.refb,
                -0.2361408314943696227e-6,
                1e-18,
                "eraApco13_safe",
                "refb",
                status,
            );
            vvd(
                eo,
                -0.003020548354802412839,
                1e-14,
                "eraApco13_safe",
                "eo",
                status,
            );
            viv(j, 0, "eraApco13_safe", "j", status);
        }
        Err(_) => {
            *status = 1;
            println!("eraApco13_safe failed: unexpected error");
        }
    }
}

fn t_apcs(status: &mut i32) {
    let date1 = 2456384.5;
    let date2 = 0.970031644;
    let pv = [
        [-1836024.09, 1056607.72, -5998795.26],
        [-77.0361767, -133.310856, 0.0971855934],
    ];
    let ebpv = [
        [-0.974170438, -0.211520082, -0.0917583024],
        [0.00364365824, -0.0154287319, -0.00668922024],
    ];
    let ehp = [-0.973458265, -0.209215307, -0.0906996477];
    let mut astrom = H1_safe::eraASTROM::default();

    let result = G2_safe::eraApcs_safe(date1, date2, &pv, &ebpv, &ehp, &mut astrom);

    match result {
        Ok(()) => {
            vvd(
                astrom.pmt,
                13.25248468622587269,
                1e-11,
                "eraApcs_safe",
                "pmt",
                status,
            );
            vvd(
                astrom.eb[0],
                -0.9741827110629881886,
                1e-12,
                "eraApcs_safe",
                "eb(1)",
                status,
            );
            vvd(
                astrom.eb[1],
                -0.2115130190136415986,
                1e-12,
                "eraApcs_safe",
                "eb(2)",
                status,
            );
            vvd(
                astrom.eb[2],
                -0.09179840186954412099,
                1e-12,
                "eraApcs_safe",
                "eb(3)",
                status,
            );
            vvd(
                astrom.eh[0],
                -0.9736425571689454706,
                1e-12,
                "eraApcs_safe",
                "eh(1)",
                status,
            );
            vvd(
                astrom.eh[1],
                -0.2092452125850435930,
                1e-12,
                "eraApcs_safe",
                "eh(2)",
                status,
            );
            vvd(
                astrom.eh[2],
                -0.09075578152248299218,
                1e-12,
                "eraApcs_safe",
                "eh(3)",
                status,
            );
            vvd(
                astrom.em,
                0.9998233241709796859,
                1e-12,
                "eraApcs_safe",
                "em",
                status,
            );
            vvd(
                astrom.v[0],
                0.2078704993282685510e-4,
                1e-16,
                "eraApcs_safe",
                "v(1)",
                status,
            );
            vvd(
                astrom.v[1],
                -0.8955360106989405683e-4,
                1e-16,
                "eraApcs_safe",
                "v(2)",
                status,
            );
            vvd(
                astrom.v[2],
                -0.3863338994289409097e-4,
                1e-16,
                "eraApcs_safe",
                "v(3)",
                status,
            );
            vvd(
                astrom.bm1,
                0.9999999950277561237,
                1e-12,
                "eraApcs_safe",
                "bm1",
                status,
            );
            vvd(
                astrom.bpn[0][0],
                1.0,
                0.0,
                "eraApcs_safe",
                "bpn(1,1)",
                status,
            );
            vvd(
                astrom.bpn[1][0],
                0.0,
                0.0,
                "eraApcs_safe",
                "bpn(2,1)",
                status,
            );
            vvd(
                astrom.bpn[2][0],
                0.0,
                0.0,
                "eraApcs_safe",
                "bpn(3,1)",
                status,
            );
            vvd(
                astrom.bpn[0][1],
                0.0,
                0.0,
                "eraApcs_safe",
                "bpn(1,2)",
                status,
            );
            vvd(
                astrom.bpn[1][1],
                1.0,
                0.0,
                "eraApcs_safe",
                "bpn(2,2)",
                status,
            );
            vvd(
                astrom.bpn[2][1],
                0.0,
                0.0,
                "eraApcs_safe",
                "bpn(3,2)",
                status,
            );
            vvd(
                astrom.bpn[0][2],
                0.0,
                0.0,
                "eraApcs_safe",
                "bpn(1,3)",
                status,
            );
            vvd(
                astrom.bpn[1][2],
                0.0,
                0.0,
                "eraApcs_safe",
                "bpn(2,3)",
                status,
            );
            vvd(
                astrom.bpn[2][2],
                1.0,
                0.0,
                "eraApcs_safe",
                "bpn(3,3)",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraApcs_safe failed: unexpected error");
        }
    }
}

fn t_apcs13(status: &mut i32) {
    let date1 = 2456165.5;
    let date2 = 0.401182685;
    let pv = [
        [-6241497.16, 401346.896, -1251136.04],
        [-29.264597, -455.021831, 0.0266151194],
    ];
    let mut astrom = H1_safe::eraASTROM::default();

    let result = G2_safe::eraApcs13_safe(date1, date2, &pv, &mut astrom);

    match result {
        Ok(()) => {
            vvd(
                astrom.pmt,
                12.65133794027378508,
                1e-11,
                "eraApcs13_safe",
                "pmt",
                status,
            );
            vvd(
                astrom.eb[0],
                0.9012691529025250644,
                1e-12,
                "eraApcs13_safe",
                "eb(1)",
                status,
            );
            vvd(
                astrom.eb[1],
                -0.4173999812023194317,
                1e-12,
                "eraApcs13_safe",
                "eb(2)",
                status,
            );
            vvd(
                astrom.eb[2],
                -0.1809906511146429670,
                1e-12,
                "eraApcs13_safe",
                "eb(3)",
                status,
            );
            vvd(
                astrom.eh[0],
                0.8939939101760130792,
                1e-12,
                "eraApcs13_safe",
                "eh(1)",
                status,
            );
            vvd(
                astrom.eh[1],
                -0.4111053891734021478,
                1e-12,
                "eraApcs13_safe",
                "eh(2)",
                status,
            );
            vvd(
                astrom.eh[2],
                -0.1782336880636997374,
                1e-12,
                "eraApcs13_safe",
                "eh(3)",
                status,
            );
            vvd(
                astrom.em,
                1.010428384373491095,
                1e-12,
                "eraApcs13_safe",
                "em",
                status,
            );
            vvd(
                astrom.v[0],
                0.4279877294121697570e-4,
                1e-16,
                "eraApcs13_safe",
                "v(1)",
                status,
            );
            vvd(
                astrom.v[1],
                0.7963255087052120678e-4,
                1e-16,
                "eraApcs13_safe",
                "v(2)",
                status,
            );
            vvd(
                astrom.v[2],
                0.3517564013384691531e-4,
                1e-16,
                "eraApcs13_safe",
                "v(3)",
                status,
            );
            vvd(
                astrom.bm1,
                0.9999999952947980978,
                1e-12,
                "eraApcs13_safe",
                "bm1",
                status,
            );
            vvd(
                astrom.bpn[0][0],
                1.0,
                0.0,
                "eraApcs13_safe",
                "bpn(1,1)",
                status,
            );
            vvd(
                astrom.bpn[1][0],
                0.0,
                0.0,
                "eraApcs13_safe",
                "bpn(2,1)",
                status,
            );
            vvd(
                astrom.bpn[2][0],
                0.0,
                0.0,
                "eraApcs13_safe",
                "bpn(3,1)",
                status,
            );
            vvd(
                astrom.bpn[0][1],
                0.0,
                0.0,
                "eraApcs13_safe",
                "bpn(1,2)",
                status,
            );
            vvd(
                astrom.bpn[1][1],
                1.0,
                0.0,
                "eraApcs13_safe",
                "bpn(2,2)",
                status,
            );
            vvd(
                astrom.bpn[2][1],
                0.0,
                0.0,
                "eraApcs13_safe",
                "bpn(3,2)",
                status,
            );
            vvd(
                astrom.bpn[0][2],
                0.0,
                0.0,
                "eraApcs13_safe",
                "bpn(1,3)",
                status,
            );
            vvd(
                astrom.bpn[1][2],
                0.0,
                0.0,
                "eraApcs13_safe",
                "bpn(2,3)",
                status,
            );
            vvd(
                astrom.bpn[2][2],
                1.0,
                0.0,
                "eraApcs13_safe",
                "bpn(3,3)",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraApcs13_safe failed: unexpected error");
        }
    }
}

fn t_aper(status: &mut i32) {
    let mut astrom = H1_safe::eraASTROM::default();
    astrom.along = 1.234;
    let theta = 5.678;

    let result = G2_safe::eraAper_safe(theta, &mut astrom);

    match result {
        Ok(()) => {
            vvd(
                astrom.eral,
                6.912000000000000000,
                1e-12,
                "eraAper_safe",
                "eral",
                status,
            );
        }
        Err(_) => {
            *status = 1;
            println!("eraAper_safe failed: unexpected error");
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

    t_apco(&mut status);
    t_apco13(&mut status);
    t_apcs(&mut status);
    t_apcs13(&mut status);
    t_aper(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
