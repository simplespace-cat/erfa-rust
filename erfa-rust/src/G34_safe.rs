// G34
//   xy06.c  → eraXy06_safe

use crate::G15_safe::{
    eraFad03_safe, eraFae03_safe, eraFaf03_safe, eraFaju03_safe, eraFal03_safe, eraFalp03_safe,
    eraFama03_safe, eraFame03_safe, eraFane03_safe, eraFaom03_safe, eraFapa03_safe, eraFasa03_safe,
    eraFaur03_safe, eraFave03_safe,
};
use crate::H1_safe::{ERFA_DAS2R, ERFA_DJ00, ERFA_DJC};

pub type ErfaResult<T> = Result<T, ()>;

#[path = "data/G34_safe/MFALS.rs"]
mod mfals_mod;
use mfals_mod::MFALS;
#[path = "data/G34_safe/MFAPL.rs"]
mod mfapl_mod;
use mfapl_mod::MFAPL;
#[path = "data/G34_safe/NC.rs"]
mod nc_mod;
use nc_mod::NC;
#[path = "data/G34_safe/AMPL.rs"]
mod ampl_mod;
use ampl_mod::AMPL as A;

// Maximum power of T in the X,Y precession-nutation polynomials
const MAXPT: usize = 5;

// Polynomial coefficients (arcsec).  First row = X, second = Y
static XYP: [[f64; MAXPT + 1]; 2] = [
    [
        -0.016_617,
        2004.191_898,
        -0.429_782_9,
        -0.198_618_34,
        0.000_007_578,
        0.000_005_928_5,
    ],
    [
        -0.006_951,
        -0.025_896,
        -22.407_274_7,
        0.001_900_59,
        0.001_112_526,
        0.000_000_135_8,
    ],
];

/* Amplitude usage: X/Y, sin/cos, power-of-T -- length=20 each */
static JAXY: [usize; 20] = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
static JASC: [usize; 20] = [0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0];
static JAPT: [usize; 20] = [0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];

const NFLS: usize = MFALS.len();
const NFPL: usize = MFAPL.len();
const NA: usize = A.len();

// CIP X,Y using IAU-2006/2000A series (truncated data).
pub fn eraXy06_safe(date1: f64, date2: f64) -> ErfaResult<(f64, f64)> {
    // Time interval in Julian-centuries since J2000.0
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    // Powers of T
    let mut pt = [0.0_f64; MAXPT + 1];
    pt[0] = 1.0;
    for j in 1..=MAXPT {
        pt[j] = pt[j - 1] * t;
    }

    // Accumulators: polynomial (pr), luni-solar (ls), planetary (pl)
    let mut xypr = [0.0_f64; 2];
    let mut xyls = [0.0_f64; 2];
    let mut xypl = [0.0_f64; 2];

    // Fundamental arguments (IERS 2003)
    let mut fa = [0.0_f64; 14];
    fa[0] = eraFal03_safe(t)?; // l
    fa[1] = eraFalp03_safe(t)?; // l'
    fa[2] = eraFaf03_safe(t)?; // F
    fa[3] = eraFad03_safe(t)?; // D
    fa[4] = eraFaom03_safe(t)?; // Om
    fa[5] = eraFame03_safe(t)?; // Mercury
    fa[6] = eraFave03_safe(t)?; // Venus
    fa[7] = eraFae03_safe(t)?; // Earth
    fa[8] = eraFama03_safe(t)?; // Mars
    fa[9] = eraFaju03_safe(t)?; // Jupiter
    fa[10] = eraFasa03_safe(t)?; // Saturn
    fa[11] = eraFaur03_safe(t)?; // Uranus
    fa[12] = eraFane03_safe(t)?; // Neptune
    fa[13] = eraFapa03_safe(t)?; // precession

    // Polynomial part (reverse iteration)
    for jxy in 0..2 {
        for j in (0..=MAXPT).rev() {
            xypr[jxy] += XYP[jxy][j] * pt[j];
        }
    }

    // Helper: accumulate one term into provided target array
    let accumulate = |target: &mut [f64; 2],
                      jxy: usize,
                      jsc: usize,
                      jpt: usize,
                      amp_index: usize,
                      sc: [f64; 2]| {
        if amp_index < NA {
            target[jxy] += A[amp_index] * sc[jsc] * pt[jpt];
        }
    };

    // Planetary nutation terms
    let mut ialast: isize = NA as isize; // 1-based like C code
    for ifreq in (0..NFPL).rev() {
        // Argument = Σ m_i * fa_i
        let mut arg = 0.0_f64;
        for i in 0..14 {
            let m = MFAPL[ifreq][i];
            if m != 0 {
                arg += (m as f64) * fa[i];
            }
        }
        let sc = [arg.sin(), arg.cos()];

        // Iterate over amplitudes for this frequency
        let ia = NC[ifreq + NFLS] as isize;
        for i in (ia..=ialast).rev() {
            let j = (i - ia) as usize;
            if j < 20 {
                accumulate(&mut xypl, JAXY[j], JASC[j], JAPT[j], (i - 1) as usize, sc);
            }
        }
        ialast = ia - 1;
    }

    // Luni-solar nutation terms
    for ifreq in (0..NFLS).rev() {
        // Argument = Σ m_i * fa_i
        let mut arg = 0.0_f64;
        for i in 0..5 {
            let m = MFALS[ifreq][i];
            if m != 0 {
                arg += (m as f64) * fa[i];
            }
        }
        let sc = [arg.sin(), arg.cos()];

        // Iterate over amplitudes for this frequency
        let ia = NC[ifreq] as isize;
        for i in (ia..=ialast).rev() {
            let j = (i - ia) as usize;
            if j < 20 {
                accumulate(&mut xyls, JAXY[j], JASC[j], JAPT[j], (i - 1) as usize, sc);
            }
        }
        ialast = ia - 1;
    }

    // Result: CIP unit-vector components (radians)
    let x = ERFA_DAS2R * (xypr[0] + (xyls[0] + xypl[0]) / 1.0e6);
    let y = ERFA_DAS2R * (xypr[1] + (xyls[1] + xypl[1]) / 1.0e6);

    Ok((x, y))
}
