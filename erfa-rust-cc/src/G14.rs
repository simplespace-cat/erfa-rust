// G14
//   eqec06.c       → eraEqec06
//   eqeq94.c       → eraEqeq94
//   era00.c        → eraEra00
//   erfadatextra.c → eraGetLeapSeconds, eraSetLeapSeconds, eraDatini
//   erfaversion.c  → eraVersion, eraVersionMajor, eraVersionMinor, eraVersionMicro, eraSofaVersion

use crate::H1::*;
use core::ffi::c_char;
use core::ptr;

// eqec06.c  →  eraEqec06

pub unsafe fn eraEqec06(date1: f64, date2: f64, dr: f64, dd: f64, dl: *mut f64, db: *mut f64) {
    let mut rm = [[0.0_f64; 3]; 3];
    let mut v1 = [0.0_f64; 3];
    let mut v2 = [0.0_f64; 3];

    eraS2c(dr, dd, v1.as_mut_ptr());

    eraEcm06(date1, date2, rm.as_mut_ptr() as *mut f64);

    eraRxp(
        rm.as_mut_ptr() as *mut f64,
        v1.as_mut_ptr(),
        v2.as_mut_ptr(),
    );

    let mut a = 0.0_f64;
    let mut b = 0.0_f64;
    eraC2s(v2.as_mut_ptr(), &mut a, &mut b);

    *dl = eraAnp(a);
    *db = eraAnpm(b);
}

// eqeq94.c  →  eraEqeq94

pub unsafe fn eraEqeq94(date1: f64, date2: f64) -> f64 {
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    let om = eraAnpm(
        (450_160.280 + (-482_890.539 + (7.455 + 0.008 * t) * t) * t) * ERFA_DAS2R
            + ((-5.0 * t) % 1.0) * ERFA_D2PI,
    );

    let mut dpsi = 0.0_f64;
    let mut deps = 0.0_f64;
    eraNut80(date1, date2, &mut dpsi, &mut deps);
    let eps0 = eraObl80(date1, date2);

    dpsi * eps0.cos() + ERFA_DAS2R * (0.00264 * om.sin() + 0.000063 * (2.0 * om).sin())
}

// era00.c  →  eraEra00

pub unsafe fn eraEra00(dj1: f64, dj2: f64) -> f64 {
    let (d1, d2) = if dj1 < dj2 { (dj1, dj2) } else { (dj2, dj1) };
    let t = d1 + (d2 - ERFA_DJ00);

    let f = (d1 % 1.0) + (d2 % 1.0);

    eraAnp(ERFA_D2PI * (f + 0.779_057_273_264_0 + 0.002_737_811_911_354_48 * t))
}

// erfadatextra.c  →  eraGetLeapSeconds / eraSetLeapSeconds / eraDatini

static mut CHANGES: *mut eraLEAPSECOND = ptr::null_mut();
static mut NDAT: i32 = -1;

pub unsafe fn eraGetLeapSeconds(table: *mut *mut eraLEAPSECOND) -> i32 {
    if NDAT <= 0 {
        let mut delat = 0.0_f64;
        let stat = eraDat(2000, 1, 1, 0.0, &mut delat);
        if stat != 0 || NDAT <= 0 {
            return -1;
        }
    }
    *table = CHANGES;
    NDAT
}

pub unsafe fn eraSetLeapSeconds(table: *mut eraLEAPSECOND, count: i32) {
    CHANGES = table;
    NDAT = count;
}

pub unsafe fn eraDatini(
    builtin: *const eraLEAPSECOND,
    n_builtin: i32,
    leapseconds: *mut *mut eraLEAPSECOND,
) -> i32 {
    if NDAT <= 0 {
        eraSetLeapSeconds(builtin as *mut eraLEAPSECOND, n_builtin);
    }
    *leapseconds = CHANGES;
    NDAT
}

// erfaversion.c  →  eraVersion & friends

const VERSION_STR: &str = concat!(env!("CARGO_PKG_VERSION"), "\0");
const SOFA_VERSION_STR: &str = concat!("unknown\0");

fn to_cstr_ptr(s: &str) -> *const c_char {
    s.as_ptr() as *const c_char
}

pub unsafe fn eraVersion() -> *const c_char {
    to_cstr_ptr(VERSION_STR)
}
pub unsafe fn eraVersionMajor() -> i32 {
    VERSION_STR
        .split('.')
        .next()
        .unwrap_or("0")
        .parse::<i32>()
        .unwrap_or(0)
}
pub unsafe fn eraVersionMinor() -> i32 {
    VERSION_STR
        .split('.')
        .nth(1)
        .unwrap_or("0")
        .parse::<i32>()
        .unwrap_or(0)
}
pub unsafe fn eraVersionMicro() -> i32 {
    VERSION_STR
        .split('.')
        .nth(2)
        .unwrap_or("0")
        .parse::<i32>()
        .unwrap_or(0)
}
pub unsafe fn eraSofaVersion() -> *const c_char {
    to_cstr_ptr(SOFA_VERSION_STR)
}
