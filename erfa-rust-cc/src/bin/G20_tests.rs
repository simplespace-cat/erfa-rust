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

fn t_ld(status: &mut i32) {
    let bm = 0.00028574;
    let mut p = [-0.763276255, -0.608633767, -0.216735543];
    let mut q = [-0.763276255, -0.608633767, -0.216735543];
    let mut e = [0.76700421, 0.605629598, 0.211937094];
    let em = 8.91276983;
    let dlim = 3e-10;
    let mut p1 = [0.0; 3];

    unsafe {
        H1::eraLd(
            bm,
            p.as_mut_ptr(),
            q.as_mut_ptr(),
            e.as_mut_ptr(),
            em,
            dlim,
            p1.as_mut_ptr(),
        );
    }

    vvd(p1[0], -0.7632762548968159627, 1e-12, "eraLd", "1", status);
    vvd(p1[1], -0.6086337670823762701, 1e-12, "eraLd", "2", status);
    vvd(p1[2], -0.2167355431320546947, 1e-12, "eraLd", "3", status);
}

fn t_ldn(status: &mut i32) {
    let n = 3;
    let mut b = [
        H1::eraLDBODY {
            bm: 0.00028574,
            dl: 3e-10,
            pv: [
                [-7.81014427, -5.60956681, -1.98079819],
                [0.0030723249, -0.00406995477, -0.00181335842],
            ],
        },
        H1::eraLDBODY {
            bm: 0.00095435,
            dl: 3e-9,
            pv: [
                [0.738098796, 4.63658692, 1.9693136],
                [-0.00755816922, 0.00126913722, 0.000727999001],
            ],
        },
        H1::eraLDBODY {
            bm: 1.0,
            dl: 6e-6,
            pv: [
                [-0.000712174377, -0.00230478303, -0.00105865966],
                [6.29235213e-6, -3.30888387e-7, -2.96486623e-7],
            ],
        },
    ];
    let mut ob = [-0.974170437, -0.2115201, -0.0917583114];
    let mut sc = [-0.763276255, -0.608633767, -0.216735543];
    let mut sn = [0.0; 3];

    unsafe {
        H1::eraLdn(
            n,
            b.as_mut_ptr(),
            ob.as_mut_ptr(),
            sc.as_mut_ptr(),
            sn.as_mut_ptr(),
        );
    }

    vvd(sn[0], -0.7632762579693333866, 1e-12, "eraLdn", "1", status);
    vvd(sn[1], -0.6086337636093002660, 1e-12, "eraLdn", "2", status);
    vvd(sn[2], -0.2167355420646328159, 1e-12, "eraLdn", "3", status);
}

fn t_ldsun(status: &mut i32) {
    let mut p = [-0.763276255, -0.608633767, -0.216735543];
    let mut e = [-0.973644023, -0.20925523, -0.0907169552];
    let em = 0.999809214;
    let mut p1 = [0.0; 3];

    unsafe {
        H1::eraLdsun(p.as_mut_ptr(), e.as_mut_ptr(), em, p1.as_mut_ptr());
    }

    vvd(
        p1[0],
        -0.7632762580731413169,
        1e-12,
        "eraLdsun",
        "1",
        status,
    );
    vvd(
        p1[1],
        -0.6086337635262647900,
        1e-12,
        "eraLdsun",
        "2",
        status,
    );
    vvd(
        p1[2],
        -0.2167355419322321302,
        1e-12,
        "eraLdsun",
        "3",
        status,
    );
}

fn t_lteceq(status: &mut i32) {
    let epj = 2500.0;
    let dl = 1.5;
    let db = 0.6;
    let mut dr = 0.0;
    let mut dd = 0.0;

    unsafe {
        H1::eraLteceq(epj, dl, db, &mut dr, &mut dd);
    }

    vvd(dr, 1.275156021861921167, 1e-14, "eraLteceq", "dr", status);
    vvd(dd, 0.9966573543519204791, 1e-14, "eraLteceq", "dd", status);
}

fn t_ltecm(status: &mut i32) {
    let epj = -3000.0;
    let mut rm = [[0.0; 3]; 3];

    unsafe {
        H1::eraLtecm(epj, &mut rm as *mut [[f64; 3]; 3] as *mut f64);
    }

    vvd(
        rm[0][0],
        0.3564105644859788825,
        1e-14,
        "eraLtecm",
        "rm11",
        status,
    );
    vvd(
        rm[0][1],
        0.8530575738617682284,
        1e-14,
        "eraLtecm",
        "rm12",
        status,
    );
    vvd(
        rm[0][2],
        0.3811355207795060435,
        1e-14,
        "eraLtecm",
        "rm13",
        status,
    );
    vvd(
        rm[1][0],
        -0.9343283469640709942,
        1e-14,
        "eraLtecm",
        "rm21",
        status,
    );
    vvd(
        rm[1][1],
        0.3247830597681745976,
        1e-14,
        "eraLtecm",
        "rm22",
        status,
    );
    vvd(
        rm[1][2],
        0.1467872751535940865,
        1e-14,
        "eraLtecm",
        "rm23",
        status,
    );
    vvd(
        rm[2][0],
        0.1431636191201167793e-2,
        1e-14,
        "eraLtecm",
        "rm31",
        status,
    );
    vvd(
        rm[2][1],
        -0.4084222566960599342,
        1e-14,
        "eraLtecm",
        "rm32",
        status,
    );
    vvd(
        rm[2][2],
        0.9127919865189030899,
        1e-14,
        "eraLtecm",
        "rm33",
        status,
    );
}

fn t_lteqec(status: &mut i32) {
    let epj = -1500.0;
    let dr = 1.234;
    let dd = 0.987;
    let mut dl = 0.0;
    let mut db = 0.0;

    unsafe {
        H1::eraLteqec(epj, dr, dd, &mut dl, &mut db);
    }

    vvd(dl, 0.5039483649047114859, 1e-14, "eraLteqec", "dl", status);
    vvd(db, 0.5848534459726224882, 1e-14, "eraLteqec", "db", status);
}

fn t_ltp(status: &mut i32) {
    let epj = 1666.666;
    let mut rp = [[0.0; 3]; 3];

    unsafe {
        H1::eraLtp(epj, &mut rp as *mut [[f64; 3]; 3] as *mut f64);
    }

    vvd(
        rp[0][0],
        0.9967044141159213819,
        1e-14,
        "eraLtp",
        "rp11",
        status,
    );
    vvd(
        rp[0][1],
        0.7437801893193210840e-1,
        1e-14,
        "eraLtp",
        "rp12",
        status,
    );
    vvd(
        rp[0][2],
        0.3237624409345603401e-1,
        1e-14,
        "eraLtp",
        "rp13",
        status,
    );
    vvd(
        rp[1][0],
        -0.7437802731819618167e-1,
        1e-14,
        "eraLtp",
        "rp21",
        status,
    );
    vvd(
        rp[1][1],
        0.9972293894454533070,
        1e-14,
        "eraLtp",
        "rp22",
        status,
    );
    vvd(
        rp[1][2],
        -0.1205768842723593346e-2,
        1e-14,
        "eraLtp",
        "rp23",
        status,
    );
    vvd(
        rp[2][0],
        -0.3237622482766575399e-1,
        1e-14,
        "eraLtp",
        "rp31",
        status,
    );
    vvd(
        rp[2][1],
        -0.1206286039697609008e-2,
        1e-14,
        "eraLtp",
        "rp32",
        status,
    );
    vvd(
        rp[2][2],
        0.9994750246704010914,
        1e-14,
        "eraLtp",
        "rp33",
        status,
    );
}

fn t_ltpb(status: &mut i32) {
    let epj = 1666.666;
    let mut rpb = [[0.0; 3]; 3];

    unsafe {
        H1::eraLtpb(epj, &mut rpb as *mut [[f64; 3]; 3] as *mut f64);
    }

    vvd(
        rpb[0][0],
        0.9967044167723271851,
        1e-14,
        "eraLtpb",
        "rpb11",
        status,
    );
    vvd(
        rpb[0][1],
        0.7437794731203340345e-1,
        1e-14,
        "eraLtpb",
        "rpb12",
        status,
    );
    vvd(
        rpb[0][2],
        0.3237632684841625547e-1,
        1e-14,
        "eraLtpb",
        "rpb13",
        status,
    );
    vvd(
        rpb[1][0],
        -0.7437795663437177152e-1,
        1e-14,
        "eraLtpb",
        "rpb21",
        status,
    );
    vvd(
        rpb[1][1],
        0.9972293947500013666,
        1e-14,
        "eraLtpb",
        "rpb22",
        status,
    );
    vvd(
        rpb[1][2],
        -0.1205741865911243235e-2,
        1e-14,
        "eraLtpb",
        "rpb23",
        status,
    );
    vvd(
        rpb[2][0],
        -0.3237630543224664992e-1,
        1e-14,
        "eraLtpb",
        "rpb31",
        status,
    );
    vvd(
        rpb[2][1],
        -0.1206316791076485295e-2,
        1e-14,
        "eraLtpb",
        "rpb32",
        status,
    );
    vvd(
        rpb[2][2],
        0.9994750220222438819,
        1e-14,
        "eraLtpb",
        "rpb33",
        status,
    );
}

fn t_ltpecl(status: &mut i32) {
    let epj = -1500.0;
    let mut vec = [0.0; 3];

    unsafe {
        H1::eraLtpecl(epj, vec.as_mut_ptr());
    }

    vvd(
        vec[0],
        0.4768625676477096525e-3,
        1e-14,
        "eraLtpecl",
        "vec1",
        status,
    );
    vvd(
        vec[1],
        -0.4052259533091875112,
        1e-14,
        "eraLtpecl",
        "vec2",
        status,
    );
    vvd(
        vec[2],
        0.9142164401096448012,
        1e-14,
        "eraLtpecl",
        "vec3",
        status,
    );
}

fn t_ltpequ(status: &mut i32) {
    let epj = -2500.0;
    let mut veq = [0.0; 3];

    unsafe {
        H1::eraLtpequ(epj, veq.as_mut_ptr());
    }

    vvd(
        veq[0],
        -0.3586652560237326659,
        1e-14,
        "eraLtpequ",
        "veq1",
        status,
    );
    vvd(
        veq[1],
        -0.1996978910771128475,
        1e-14,
        "eraLtpequ",
        "veq2",
        status,
    );
    vvd(
        veq[2],
        0.9118552442250819624,
        1e-14,
        "eraLtpequ",
        "veq3",
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

    t_ld(&mut status);
    t_ldn(&mut status);
    t_ldsun(&mut status);
    t_lteceq(&mut status);
    t_ltecm(&mut status);
    t_lteqec(&mut status);
    t_ltp(&mut status);
    t_ltpb(&mut status);
    t_ltpecl(&mut status);
    t_ltpequ(&mut status);

    if status != 0 {
        println!("t_erfa_c validation failed!");
    } else {
        println!("t_erfa_c validation successful");
    }
    std::process::exit(status);
}
