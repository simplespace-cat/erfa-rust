// G23
//   nut00b.c → eraNut00b
//   nut06a.c → eraNut06a
//   nut80.c  → eraNut80
//   nutm80.c → eraNutm80
//   obl06.c  → eraObl06
//   obl80.c  → eraObl80

use crate::H1::*;

#[path = "data/G23/X00B.rs"]
mod x00b_mod;
use x00b_mod::X00B;
#[path = "data/G23/X80.rs"]
mod x80_mod;
use x80_mod::X80;

#[inline]
fn fmod(a: f64, b: f64) -> f64 {
    a.rem_euclid(b)
}

// eraNut00b   IAU-2000B nutation

#[derive(Clone, Copy)]
struct Term00B {
    nl: i32,
    nlp: i32,
    nf: i32,
    nd: i32,
    nom: i32,
    ps: f64,
    pst: f64,
    pc: f64,
    ec: f64,
    ect: f64,
    es: f64,
}

// eraNut06a   2000A nutation + 2006
#[derive(Clone, Copy)]
struct Term80 {
    nl: i32,
    nlp: i32,
    nf: i32,
    nd: i32,
    nom: i32,
    sp: f64,
    spt: f64,
    ce: f64,
    cet: f64,
}

const U2R_00B: f64 = ERFA_DAS2R / 1e7;
const DPPLAN: f64 = -0.135 * ERFA_DMAS2R;
const DEPLAN: f64 = 0.388 * ERFA_DMAS2R;

pub unsafe fn eraNut00b(date1: f64, date2: f64, dpsi: *mut f64, deps: *mut f64) {
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    let el = fmod(485_868.249036 + 1_717_915_923.2178 * t, ERFA_TURNAS) * ERFA_DAS2R;
    let elp = fmod(1_287_104.79305 + 129_596_581.0481 * t, ERFA_TURNAS) * ERFA_DAS2R;
    let f = fmod(335_779.526232 + 1_739_527_262.8478 * t, ERFA_TURNAS) * ERFA_DAS2R;
    let d = fmod(1_072_260.70369 + 1_602_961_601.2090 * t, ERFA_TURNAS) * ERFA_DAS2R;
    let om = fmod(450_160.398036 + -6_962_890.5431 * t, ERFA_TURNAS) * ERFA_DAS2R;

    let mut dp = 0.0;
    let mut de = 0.0;
    for term in X00B.iter().rev() {
        let arg = fmod(
            (term.nl as f64) * el
                + (term.nlp as f64) * elp
                + (term.nf as f64) * f
                + (term.nd as f64) * d
                + (term.nom as f64) * om,
            ERFA_D2PI,
        );
        let sarg = arg.sin();
        let carg = arg.cos();
        dp += (term.ps + term.pst * t) * sarg + term.pc * carg;
        de += (term.ec + term.ect * t) * carg + term.es * sarg;
    }
    *dpsi = dp * U2R_00B + DPPLAN;
    *deps = de * U2R_00B + DEPLAN;
}

// eraNut06a
pub unsafe fn eraNut06a(date1: f64, date2: f64, dpsi: *mut f64, deps: *mut f64) {
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;
    let fj2 = -2.7774e-6 * t;
    let mut dp = 0.0;
    let mut de = 0.0;
    eraNut00a(date1, date2, &mut dp, &mut de);
    *dpsi = dp + dp * (0.4697e-6 + fj2);
    *deps = de + de * fj2;
}

// eraNut80  IAU 1980 nutation
const U2R_80: f64 = ERFA_DAS2R / 1e4;

pub unsafe fn eraNut80(date1: f64, date2: f64, dpsi: *mut f64, deps: *mut f64) {
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    let el = eraAnpm(
        (485_866.733 + (715_922.633 + (31.310 + 0.064 * t) * t) * t) * ERFA_DAS2R
            + fmod(1325.0 * t, 1.0) * ERFA_D2PI,
    );
    let elp = eraAnpm(
        (1_287_099.804 + (1_292_581.224 + (-0.577 - 0.012 * t) * t) * t) * ERFA_DAS2R
            + fmod(99.0 * t, 1.0) * ERFA_D2PI,
    );
    let f = eraAnpm(
        (335_778.877 + (295_263.137 + (-13.257 + 0.011 * t) * t) * t) * ERFA_DAS2R
            + fmod(1342.0 * t, 1.0) * ERFA_D2PI,
    );
    let d = eraAnpm(
        (1_072_261.307 + (1_105_601.328 + (-6.891 + 0.019 * t) * t) * t) * ERFA_DAS2R
            + fmod(1236.0 * t, 1.0) * ERFA_D2PI,
    );
    let om = eraAnpm(
        (450_160.280 + (-482_890.539 + (7.455 + 0.008 * t) * t) * t) * ERFA_DAS2R
            + fmod(-5.0 * t, 1.0) * ERFA_D2PI,
    );

    let mut dp = 0.0;
    let mut de = 0.0;
    for term in X80.iter().rev() {
        let arg = (term.nl as f64) * el
            + (term.nlp as f64) * elp
            + (term.nf as f64) * f
            + (term.nd as f64) * d
            + (term.nom as f64) * om;
        let s = term.sp + term.spt * t;
        let c = term.ce + term.cet * t;
        if s != 0.0 {
            dp += s * arg.sin();
        }
        if c != 0.0 {
            de += c * arg.cos();
        }
    }
    *dpsi = dp * U2R_80;
    *deps = de * U2R_80;
}

// eraNutm80  Nutation matrix (1980 model)
pub unsafe fn eraNutm80(date1: f64, date2: f64, rmatn: *mut f64) {
    let mut dp = 0.0;
    let mut de = 0.0;
    eraNut80(date1, date2, &mut dp, &mut de);
    let epsa = eraObl80(date1, date2);
    eraNumat(epsa, dp, de, rmatn);
}

// eraObl06 / eraObl80
pub unsafe fn eraObl06(date1: f64, date2: f64) -> f64 {
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;
    (84381.406
        + (-46.836769
            + (-0.0001831 + (0.00200340 + (-0.000000576 + (-0.0000000434) * t) * t) * t) * t)
            * t)
        * ERFA_DAS2R
}

pub unsafe fn eraObl80(date1: f64, date2: f64) -> f64 {
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;
    ERFA_DAS2R * (84381.448 + (-46.8150 + (-0.00059 + 0.001813 * t) * t) * t)
}
