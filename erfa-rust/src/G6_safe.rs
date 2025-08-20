// G6
//   bi00.c   → eraBi00_safe
//   bp00.c   → eraBp00_safe
//   bp06.c   → eraBp06_safe
//   bpn2xy.c → eraBpn2xy_safe

use crate::H1_safe::{ERFA_DAS2R, ERFA_DJ00, ERFA_DJC, ERFA_DJM0, ERFA_DJM00};

use crate::G16_safe::eraFw2m_safe;
use crate::G19_safe::eraIr_safe;
use crate::G24_safe::eraPfw06_safe;
use crate::G25_safe::eraPmat06_safe;
use crate::G26_safe::eraPr00_safe;
use crate::G28_safe::{eraRx_safe, eraRxr_safe, eraRy_safe, eraRz_safe};
use crate::G33_safe::eraTr_safe;
use crate::G8_safe::eraCr_safe;

pub type ErfaResult<T> = Result<T, ()>;

// eraBi00_safe → bi00.c
// Frame-bias corrections (ICRS → J2000.0), radians; returns (dpsibi, depsbi, dra).
pub fn eraBi00_safe() -> ErfaResult<(f64, f64, f64)> {
    const DPBIAS: f64 = -0.041_775 * ERFA_DAS2R;
    const DEBIAS: f64 = -0.006_819_2 * ERFA_DAS2R;
    const DRA0: f64 = -0.0146 * ERFA_DAS2R;
    Ok((DPBIAS, DEBIAS, DRA0))
}

// eraBp00_safe → bp00.c
// Frame bias (B), precession (P) and product (BP), IAU 2000; returns (rb, rp, rbp).
pub fn eraBp00_safe(
    date1: f64,
    date2: f64,
) -> ErfaResult<([[f64; 3]; 3], [[f64; 3]; 3], [[f64; 3]; 3])> {
    const EPS0: f64 = 84_381.448 * ERFA_DAS2R;
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    let (dpsibi, depsbi, dra0) = eraBi00_safe()?;

    let psia77 = (5038.7784 + (-1.07259 + -0.001_147 * t) * t) * t * ERFA_DAS2R;
    let oma77 = EPS0 + (0.05127 + -0.007_726 * t) * t * t * ERFA_DAS2R;
    let chia = (10.5526 + (-2.38064 + -0.001_125 * t) * t) * t * ERFA_DAS2R;

    let (dpsipr, depspr) = eraPr00_safe(date1, date2)?;
    let psia = psia77 + dpsipr;
    let oma = oma77 + depspr;

    let mut rbw = [[0.0_f64; 3]; 3];
    let mut rb = [[0.0_f64; 3]; 3];
    let mut rp = [[0.0_f64; 3]; 3];

    // Bias: GCRS → J2000.0
    eraIr_safe(&mut rbw)?;
    eraRz_safe(dra0, &mut rbw)?;
    eraRy_safe(dpsibi * EPS0.sin(), &mut rbw)?;
    eraRx_safe(-depsbi, &mut rbw)?;
    eraCr_safe(&rbw, &mut rb)?;

    // Precession: J2000.0 → mean of date
    eraIr_safe(&mut rp)?;
    eraRx_safe(EPS0, &mut rp)?;
    eraRz_safe(-psia, &mut rp)?;
    eraRx_safe(-oma, &mut rp)?;
    eraRz_safe(chia, &mut rp)?;

    let rbp = eraRxr_safe(&rp, &rbw)?;
    Ok((rb, rp, rbp))
}

// eraBp06_safe → bp06.c
// Frame bias (B), precession (P) and product (BP), IAU 2006; returns (rb, rp, rbp).
pub fn eraBp06_safe(
    date1: f64,
    date2: f64,
) -> ErfaResult<([[f64; 3]; 3], [[f64; 3]; 3], [[f64; 3]; 3])> {
    // Bias matrix from FukushimaWilliams angles at J2000.0
    let (gamb, phib, psib, epsa) = eraPfw06_safe(ERFA_DJM0, ERFA_DJM00)?;
    let rb = eraFw2m_safe(gamb, phib, psib, epsa)?;

    // Precession matrix for given date (IAU 2006)
    let rbpw = eraPmat06_safe(date1, date2)?;

    // rp = P × Tr(B)
    let rbt = eraTr_safe(&rb)?;
    let rp = eraRxr_safe(&rbpw, &rbt)?;

    // rbp = P
    let rbp = rbpw;

    Ok((rb, rp, rbp))
}

// eraBpn2xy_safe → bpn2xy.c
// Extract CIP X,Y from rbpn matrix; returns (x, y) from row 2.
pub fn eraBpn2xy_safe(rbpn: &[[f64; 3]; 3]) -> ErfaResult<(f64, f64)> {
    let x = rbpn[2][0];
    let y = rbpn[2][1];
    Ok((x, y))
}
