// G26
//   pn06.c   → eraPn06_safe
//   pn06a.c  → eraPn06a_safe
//   pnm00a.c → eraPnm00a_safe
//   pnm00b.c → eraPnm00b_safe
//   pnm06a.c → eraPnm06a_safe
//   pnm80.c  → eraPnm80_safe
//   pom00.c  → eraPom00_safe
//   ppp.c    → eraPpp_safe
//   ppsp.c   → eraPpsp_safe
//   pr00.c   → eraPr00_safe

use crate::H1_safe::{ERFA_DAS2R, ERFA_DJ00, ERFA_DJC, ERFA_DJM0, ERFA_DJM00};

use crate::G16_safe::eraFw2m_safe;
use crate::G19_safe::eraIr_safe;
use crate::G23_safe::{eraNut06a_safe, eraNutm80_safe};
use crate::G24_safe::eraPfw06_safe;
use crate::G25_safe::{eraPmat76_safe, eraPn00a_safe, eraPn00b_safe};
use crate::G28_safe::{eraRx_safe, eraRxr_safe, eraRy_safe, eraRz_safe};
use crate::G30_safe::eraSxp_safe;
use crate::G33_safe::eraTr_safe;
use crate::G8_safe::eraCr_safe;

pub type ErfaResult<T> = Result<T, ()>;

// G26/pn06.c
// Precession-nutation products, IAU 2006 bias-precession with given dpsi,deps.
pub fn eraPn06_safe(
    date1: f64,
    date2: f64,
    dpsi: f64,
    deps: f64,
) -> ErfaResult<(
    f64,           // epsa
    [[f64; 3]; 3], // rb
    [[f64; 3]; 3], // rp
    [[f64; 3]; 3], // rbp
    [[f64; 3]; 3], // rn
    [[f64; 3]; 3], // rbpn
)> {
    // Frame-bias FukushimaWilliams angles of J2000.0
    let (mut gamb, mut phib, mut psib, mut eps) = eraPfw06_safe(ERFA_DJM0, ERFA_DJM00)?;
    let r1 = eraFw2m_safe(gamb, phib, psib, eps)?;
    let mut rb = [[0.0_f64; 3]; 3];
    eraCr_safe(&r1, &mut rb)?;

    // Bias-precession angles of date
    let (gd, pd, sd, ed) = eraPfw06_safe(date1, date2)?;
    gamb = gd;
    phib = pd;
    psib = sd;
    eps = ed;
    let r2 = eraFw2m_safe(gamb, phib, psib, eps)?;
    let mut rbp = [[0.0_f64; 3]; 3];
    eraCr_safe(&r2, &mut rbp)?;

    // Precession matrix:  rp = r2 × r1ᵀ
    let rt = eraTr_safe(&r1)?;
    let rp = eraRxr_safe(&r2, &rt)?;

    // Full bias-precession-nutation
    let rnp = eraFw2m_safe(gamb, phib, psib + dpsi, eps + deps)?;
    let mut rbpn = [[0.0_f64; 3]; 3];
    eraCr_safe(&rnp, &mut rbpn)?;

    // Nutation matrix: rn = rnp × r2ᵀ
    let rt2 = eraTr_safe(&r2)?;
    let rn = eraRxr_safe(&rnp, &rt2)?;

    // Mean obliquity of date
    let epsa = eps;

    Ok((epsa, rb, rp, rbp, rn, rbpn))
}

// G26/pn06a.c
// Precession-nutation products using IAU 2006/2000A nutation.
pub fn eraPn06a_safe(
    date1: f64,
    date2: f64,
) -> ErfaResult<(
    f64,           // dpsi
    f64,           // deps
    f64,           // epsa
    [[f64; 3]; 3], // rb
    [[f64; 3]; 3], // rp
    [[f64; 3]; 3], // rbp
    [[f64; 3]; 3], // rn
    [[f64; 3]; 3], // rbpn
)> {
    let (dpsi, deps) = eraNut06a_safe(date1, date2)?;
    let (epsa, rb, rp, rbp, rn, rbpn) = eraPn06_safe(date1, date2, dpsi, deps)?;
    Ok((dpsi, deps, epsa, rb, rp, rbp, rn, rbpn))
}

// G26/pnm00a.c
// Precession-nutation matrix, IAU 2000A.
pub fn eraPnm00a_safe(date1: f64, date2: f64) -> ErfaResult<[[f64; 3]; 3]> {
    let (_dpsi, _deps, _epsa, _rb, _rp, _rbp, _rn, rbpn) = eraPn00a_safe(date1, date2)?;
    Ok(rbpn)
}

// G26/pnm00b.c
// Precession-nutation matrix, IAU 2000B.
pub fn eraPnm00b_safe(date1: f64, date2: f64) -> ErfaResult<[[f64; 3]; 3]> {
    let (_dpsi, _deps, _epsa, _rb, _rp, _rbp, _rn, rbpn) = eraPn00b_safe(date1, date2)?;
    Ok(rbpn)
}

// G26/pnm06a.c
// Precession-nutation matrix, IAU 2006/2000A.
pub fn eraPnm06a_safe(date1: f64, date2: f64) -> ErfaResult<[[f64; 3]; 3]> {
    let (gamb, phib, psib, epsa) = eraPfw06_safe(date1, date2)?;
    let (dp, de) = eraNut06a_safe(date1, date2)?;
    let rbpn = eraFw2m_safe(gamb, phib, psib + dp, epsa + de)?;
    Ok(rbpn)
}

// G26/pnm80.c
// Precession-nutation matrix, IAU 1976/1980.
pub fn eraPnm80_safe(date1: f64, date2: f64) -> ErfaResult<[[f64; 3]; 3]> {
    let rmatp = eraPmat76_safe(date1, date2)?;
    let rmatn = eraNutm80_safe(date1, date2)?;
    let rmatpn = eraRxr_safe(&rmatn, &rmatp)?;
    Ok(rmatpn)
}

// G26/pom00.c
// Polar motion matrix (IAU 2000).
pub fn eraPom00_safe(xp: f64, yp: f64, sp: f64) -> ErfaResult<[[f64; 3]; 3]> {
    let mut rpom = [[0.0_f64; 3]; 3];
    eraIr_safe(&mut rpom)?;
    eraRz_safe(sp, &mut rpom)?;
    eraRy_safe(-xp, &mut rpom)?;
    eraRx_safe(-yp, &mut rpom)?;
    Ok(rpom)
}

// G26/ppp.c
// P-vector addition.
pub fn eraPpp_safe(a: &[f64; 3], b: &[f64; 3]) -> ErfaResult<[f64; 3]> {
    Ok([a[0] + b[0], a[1] + b[1], a[2] + b[2]])
}

// G26/ppsp.c
// Compute a + s*b for p-vectors.
pub fn eraPpsp_safe(a: &[f64; 3], s: f64, b: &[f64; 3]) -> ErfaResult<[f64; 3]> {
    let sb = eraSxp_safe(s, b)?;
    let apb = eraPpp_safe(a, &sb)?;
    Ok(apb)
}

// G26/pr00.c
// IAU 2000 precession-rate corrections (dpsipr, depspr) per century.
pub fn eraPr00_safe(date1: f64, date2: f64) -> ErfaResult<(f64, f64)> {
    const PRECOR: f64 = -0.29965 * ERFA_DAS2R;
    const OBLCOR: f64 = -0.02524 * ERFA_DAS2R;
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;
    let dpsipr = PRECOR * t;
    let depspr = OBLCOR * t;
    Ok((dpsipr, depspr))
}
