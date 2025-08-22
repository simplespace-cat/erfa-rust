// G12
//   epb.c    → eraEpb
//   epb2jd.c → eraEpb2jd
//   epj.c    → eraEpj
//   epj2jd.c → eraEpj2jd

use crate::H1::*;

// eraEpb    Julian Date → Besselian Epoch
pub unsafe fn eraEpb(dj1: f64, dj2: f64) -> f64 {
    const D1900: f64 = 36524.68648;

    1900.0 + ((dj1 - ERFA_DJ00) + (dj2 + D1900)) / ERFA_DTY
}

// eraEpb2jd    Besselian Epoch → Julian Date (MJD parts)
pub unsafe fn eraEpb2jd(epb: f64, djm0: *mut f64, djm: *mut f64) {
    *djm0 = ERFA_DJM0;
    *djm = 15_019.81352 + (epb - 1900.0) * ERFA_DTY;
}

// eraEpj    Julian Date → Julian Epoch
pub unsafe fn eraEpj(dj1: f64, dj2: f64) -> f64 {
    2000.0 + ((dj1 - ERFA_DJ00) + dj2) / ERFA_DJY
}

// eraEpj2jd    Julian Epoch → Julian Date (MJD parts)
pub unsafe fn eraEpj2jd(epj: f64, djm0: *mut f64, djm: *mut f64) {
    *djm0 = ERFA_DJM0;
    *djm = ERFA_DJM00 + (epj - 2000.0) * ERFA_DJY;
}
