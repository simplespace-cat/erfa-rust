// G23
//   nut00b.c → eraNut00b_safe
//   nut06a.c → eraNut06a_safe
//   nut80.c  → eraNut80_safe
//   nutm80.c → eraNutm80_safe
//   obl06.c  → eraObl06_safe
//   obl80.c  → eraObl80_safe

use crate::G1_safe::eraAnpm_safe;
use crate::G21_safe::eraNumat_safe;
use crate::G22_safe::eraNut00a_safe;
use crate::H1_safe::{ERFA_D2PI, ERFA_DAS2R, ERFA_DJ00, ERFA_DJC, ERFA_DMAS2R, ERFA_TURNAS};

#[path = "data/G23_safe/X00B.rs"]
mod x00b_mod;
use x00b_mod::X00B;

#[path = "data/G23_safe/X80.rs"]
mod x80_mod;
use x80_mod::X80;

pub type ErfaResult<T> = Result<T, ()>;

// Helper: positive modulo for angles.
#[inline]
fn fmod_pos(a: f64, b: f64) -> f64 {
    a.rem_euclid(b)
}


 
//  eraNut00b_safe   IAU-2000B nutation

/* 77 luni-solar */
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

const U2R_00B: f64 = ERFA_DAS2R / 1e7; /* 0.1 µas → rad */
const DPPLAN: f64 = -0.135 * ERFA_DMAS2R;
const DEPLAN: f64 = 0.388 * ERFA_DMAS2R;

pub fn eraNut00b_safe(date1: f64, date2: f64) -> ErfaResult<(f64, f64)> {
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    // Fundamental arguments (arcsec -> rad for linear terms).
    let el = fmod_pos(485_868.249_036 + 1_717_915_923.2178 * t, ERFA_TURNAS) * ERFA_DAS2R;
    let elp = fmod_pos(1_287_104.793_05 + 129_596_581.0481 * t, ERFA_TURNAS) * ERFA_DAS2R;
    let f = fmod_pos(335_779.526_232 + 1_739_527_262.8478 * t, ERFA_TURNAS) * ERFA_DAS2R;
    let d = fmod_pos(1_072_260.703_69 + 1_602_961_601.2090 * t, ERFA_TURNAS) * ERFA_DAS2R;
    let om = fmod_pos(450_160.398_036 - 6_962_890.5431 * t, ERFA_TURNAS) * ERFA_DAS2R;

    // Accumulate series.
    let mut dp = 0.0f64;
    let mut de = 0.0f64;
    for term in X00B.iter().rev() {
        let arg = fmod_pos(
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

    let dpsi = dp * U2R_00B + DPPLAN;
    let deps = de * U2R_00B + DEPLAN;
    Ok((dpsi, deps))
}


//  eraNut06a_safe   2000A nutation + 2006
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

pub fn eraNut06a_safe(date1: f64, date2: f64) -> ErfaResult<(f64, f64)> {
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;
    let fj2 = -2.7774e-6 * t;

    let (dp0, de0) = eraNut00a_safe(date1, date2)?;
    let dpsi = dp0 + dp0 * (0.4697e-6 + fj2);
    let deps = de0 + de0 * fj2;
    Ok((dpsi, deps))
}

// IAU 1980 nutation: returns (Δψ, Δε) in radians.
pub fn eraNut80_safe(date1: f64, date2: f64) -> ErfaResult<(f64, f64)> {
    const U2R_80: f64 = ERFA_DAS2R / 1e4; // 0.1 mas → rad
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    // Delaunay arguments, normalized to ±π using eraAnpm_safe.
    let el = eraAnpm_safe(
        (485_866.733 + (715_922.633 + (31.310 + 0.064 * t) * t) * t) * ERFA_DAS2R
            + fmod_pos(1325.0 * t, 1.0) * ERFA_D2PI,
    )?;
    let elp = eraAnpm_safe(
        (1_287_099.804 + (1_292_581.224 + (-0.577 - 0.012 * t) * t) * t) * ERFA_DAS2R
            + fmod_pos(99.0 * t, 1.0) * ERFA_D2PI,
    )?;
    let f = eraAnpm_safe(
        (335_778.877 + (295_263.137 + (-13.257 + 0.011 * t) * t) * t) * ERFA_DAS2R
            + fmod_pos(1342.0 * t, 1.0) * ERFA_D2PI,
    )?;
    let d = eraAnpm_safe(
        (1_072_261.307 + (1_105_601.328 + (-6.891 + 0.019 * t) * t) * t) * ERFA_DAS2R
            + fmod_pos(1236.0 * t, 1.0) * ERFA_D2PI,
    )?;
    let om = eraAnpm_safe(
        (450_160.280 + (-482_890.539 + (7.455 + 0.008 * t) * t) * t) * ERFA_DAS2R
            + fmod_pos(-5.0 * t, 1.0) * ERFA_D2PI,
    )?;

    // Accumulate series.
    let mut dp = 0.0f64;
    let mut de = 0.0f64;
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

    Ok((dp * U2R_80, de * U2R_80))
}

// Nutation matrix (1980 model): builds 3×3 matrix from Δψ, Δε and mean obliquity.
pub fn eraNutm80_safe(date1: f64, date2: f64) -> ErfaResult<[[f64; 3]; 3]> {
    let (dp, de) = eraNut80_safe(date1, date2)?;
    let epsa = eraObl80_safe(date1, date2)?;
    let rmatn = eraNumat_safe(epsa, dp, de)?;
    Ok(rmatn)
}

// Mean obliquity of the ecliptic, IAU 2006, radians.
pub fn eraObl06_safe(date1: f64, date2: f64) -> ErfaResult<f64> {
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;
    let eps = (84_381.406
        + (-46.836_769
            + (-0.000_1831 + (0.002_003_40 + (-0.000_000_576 + (-0.000_000_0434) * t) * t) * t)
                * t)
            * t)
        * ERFA_DAS2R;
    Ok(eps)
}

// Mean obliquity of the ecliptic, IAU 1980, radians.
pub fn eraObl80_safe(date1: f64, date2: f64) -> ErfaResult<f64> {
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;
    let eps = ERFA_DAS2R * (84_381.448 + (-46.8150 + (-0.00059 + 0.001_813 * t) * t) * t);
    Ok(eps)
}
