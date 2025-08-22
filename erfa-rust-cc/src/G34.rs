// G34
//   xy06.c  → eraXy06

use crate::H1::*;

#[path = "data/G34/MFALS.rs"]
mod mfals_mod;
use mfals_mod::MFALS;
#[path = "data/G34/MFAPL.rs"]
mod mfapl_mod;
use mfapl_mod::MFAPL;
#[path = "data/G34/NC.rs"]
mod nc_mod;
use nc_mod::NC;
#[path = "data/G34/AMPL.rs"]
mod ampl_mod;
use ampl_mod::AMPL as A;

const MAXPT: usize = 5;

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

const NFLS: usize = MFALS.len();

const NFPL: usize = MFAPL.len();

const NA: usize = A.len();

static JAXY: [usize; 20] = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
static JASC: [usize; 20] = [0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0];
static JAPT: [usize; 20] = [0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];

pub unsafe fn eraXy06(date1: f64, date2: f64, x: *mut f64, y: *mut f64) {
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    let mut pt = [0.0_f64; MAXPT + 1];
    pt[0] = 1.0;
    for j in 1..=MAXPT {
        pt[j] = pt[j - 1] * t;
    }

    let mut xypr = [0.0_f64; 2];
    let mut xyls = [0.0_f64; 2];
    let mut xypl = [0.0_f64; 2];

    let mut fa = [0.0_f64; 14];
    fa[0] = eraFal03(t);
    fa[1] = eraFalp03(t);
    fa[2] = eraFaf03(t);
    fa[3] = eraFad03(t);
    fa[4] = eraFaom03(t);
    fa[5] = eraFame03(t);
    fa[6] = eraFave03(t);
    fa[7] = eraFae03(t);
    fa[8] = eraFama03(t);
    fa[9] = eraFaju03(t);
    fa[10] = eraFasa03(t);
    fa[11] = eraFaur03(t);
    fa[12] = eraFane03(t);
    fa[13] = eraFapa03(t);

    for jxy in 0..2 {
        for j in (0..=MAXPT).rev() {
            xypr[jxy] += XYP[jxy][j] * pt[j];
        }
    }

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

    let mut ialast: isize = NA as isize;
    for ifreq in (0..NFPL).rev() {
        let mut arg = 0.0_f64;
        for i in 0..14 {
            let m = MFAPL[ifreq][i];
            if m != 0 {
                arg += (m as f64) * fa[i];
            }
        }
        let sc = [arg.sin(), arg.cos()];

        let ia = NC[ifreq + NFLS] as isize;
        for i in ((ia)..=ialast).rev() {
            let j = (i - ia) as usize;
            if j < 20 {
                accumulate(&mut xypl, JAXY[j], JASC[j], JAPT[j], (i - 1) as usize, sc);
            }
        }
        ialast = ia - 1;
    }

    for ifreq in (0..NFLS).rev() {
        let mut arg = 0.0_f64;
        for i in 0..5 {
            let m = MFALS[ifreq][i];
            if m != 0 {
                arg += (m as f64) * fa[i];
            }
        }
        let sc = [arg.sin(), arg.cos()];

        let ia = NC[ifreq] as isize;
        for i in ((ia)..=ialast).rev() {
            let j = (i - ia) as usize;
            if j < 20 {
                accumulate(&mut xyls, JAXY[j], JASC[j], JAPT[j], (i - 1) as usize, sc);
            }
        }
        ialast = ia - 1;
    }

    let res_x = ERFA_DAS2R * (xypr[0] + (xyls[0] + xypl[0]) / 1.0e6);
    let res_y = ERFA_DAS2R * (xypr[1] + (xyls[1] + xypl[1]) / 1.0e6);

    if !x.is_null() {
        *x = res_x;
    }
    if !y.is_null() {
        *y = res_y;
    }
}
