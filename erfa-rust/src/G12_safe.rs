// G12
//   epb.c    → eraEpb_safe
//   epb2jd.c → eraEpb2jd_safe
//   epj.c    → eraEpj_safe
//   epj2jd.c → eraEpj2jd_safe

use crate::H1_safe::{ERFA_DJ00, ERFA_DJM0, ERFA_DJM00, ERFA_DJY, ERFA_DTY};

pub type ErfaResult<T> = Result<T, ()>;

// Julian Date → Besselian Epoch (B1900-based).
pub fn eraEpb_safe(dj1: f64, dj2: f64) -> ErfaResult<f64> {
    const D1900: f64 = 36524.68648;
    let epb = 1900.0 + ((dj1 - ERFA_DJ00) + (dj2 + D1900)) / ERFA_DTY;
    Ok(epb)
}

// Besselian Epoch → two-part MJD (djm0, djm).
pub fn eraEpb2jd_safe(epb: f64) -> ErfaResult<(f64, f64)> {
    let djm0 = ERFA_DJM0;
    let djm = 15_019.813_52 + (epb - 1900.0) * ERFA_DTY;
    Ok((djm0, djm))
}

// Julian Date → Julian Epoch (J2000-based).
pub fn eraEpj_safe(dj1: f64, dj2: f64) -> ErfaResult<f64> {
    let epj = 2000.0 + ((dj1 - ERFA_DJ00) + dj2) / ERFA_DJY;
    Ok(epj)
}

// Julian Epoch → two-part MJD (djm0, djm).
pub fn eraEpj2jd_safe(epj: f64) -> ErfaResult<(f64, f64)> {
    let djm0 = ERFA_DJM0;
    let djm = ERFA_DJM00 + (epj - 2000.0) * ERFA_DJY;
    Ok((djm0, djm))
}
