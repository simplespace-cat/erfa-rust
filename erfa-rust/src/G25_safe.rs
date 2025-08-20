// G25
//   pmat00.c → eraPmat00_safe
//   pmat06.c → eraPmat06_safe
//   pmat76.c → eraPmat76_safe
//   pmp.c    → eraPmp_safe
//   pmpx.c   → eraPmpx_safe
//   pmsafe.c → eraPmsafe_safe
//   pn.c     → eraPn_safe
//   pn00.c   → eraPn00_safe
//   pn00a.c  → eraPn00a_safe
//   pn00b.c  → eraPn00b_safe

use crate::G16_safe::eraFw2m_safe;
use crate::G19_safe::eraIr_safe;
use crate::G21_safe::eraNumat_safe;
use crate::G22_safe::eraNut00a_safe;
use crate::G23_safe::{eraNut00b_safe, eraObl80_safe};
use crate::G24_safe::{eraPdp_safe, eraPfw06_safe, eraPm_safe};
use crate::G26_safe::eraPr00_safe;
use crate::G27_safe::eraPrec76_safe;
use crate::G28_safe::{eraRxr_safe, eraRy_safe, eraRz_safe};
use crate::G30_safe::{eraSeps_safe, eraStarpm_safe, eraSxp_safe};
use crate::G35_safe::eraZp_safe;
use crate::G6_safe::eraBp00_safe;
use crate::H1_safe::{ERFA_AULT, ERFA_DAS2R, ERFA_DAU, ERFA_DAYSEC, ERFA_DJ00, ERFA_DJM, ERFA_DJY};

pub type ErfaResult<T> = Result<T, ()>;

// Precession matrix, IAU 2000 bias-precession model.
pub fn eraPmat00_safe(date1: f64, date2: f64) -> ErfaResult<[[f64; 3]; 3]> {
    let (_rb, _rp, rbp) = eraBp00_safe(date1, date2)?;
    Ok(rbp)
}

// Precession matrix, IAU 2006 bias-precession model.
pub fn eraPmat06_safe(date1: f64, date2: f64) -> ErfaResult<[[f64; 3]; 3]> {
    let (gamb, phib, psib, epsa) = eraPfw06_safe(date1, date2)?;
    let rbp = eraFw2m_safe(gamb, phib, psib, epsa)?;
    Ok(rbp)
}

// Precession matrix, IAU 1976 model.
pub fn eraPmat76_safe(date1: f64, date2: f64) -> ErfaResult<[[f64; 3]; 3]> {
    let (zeta, z, theta) = eraPrec76_safe(ERFA_DJ00, 0.0, date1, date2)?;
    let mut r = [[0.0_f64; 3]; 3];
    eraIr_safe(&mut r)?;
    eraRz_safe(-zeta, &mut r)?;
    eraRy_safe(theta, &mut r)?;
    eraRz_safe(-z, &mut r)?;
    Ok(r)
}

// P-vector subtraction (a − b).
pub fn eraPmp_safe(a: &[f64; 3], b: &[f64; 3]) -> ErfaResult<[f64; 3]> {
    Ok([a[0] - b[0], a[1] - b[1], a[2] - b[2]])
}

// Proper motion and parallax: catalog position → updated unit vector.
pub fn eraPmpx_safe(
    rc: f64,
    dc: f64,
    pr: f64,
    pd: f64,
    px: f64,
    rv: f64,
    pmt: f64,
    pob: &[f64; 3],
) -> ErfaResult<[f64; 3]> {
    // Unit conversions and constants.
    const VF: f64 = ERFA_DAYSEC * ERFA_DJM / ERFA_DAU; // km/s → au/yr
    const AULTY: f64 = ERFA_AULT / ERFA_DAYSEC / ERFA_DJY; // 1 au light-time (yr)

    // Spherical to Cartesian for catalog direction.
    let (sr, cr) = rc.sin_cos();
    let (sd, cd) = dc.sin_cos();
    let mut p = [cr * cd, sr * cd, sd];

    // Effective interval with Roemer term.
    let dt = pmt + eraPdp_safe(&p, pob)? * AULTY;

    // Space motion (rad/yr) and parallax.
    let pxr = px * ERFA_DAS2R;
    let w = VF * rv * pxr;
    let pdz = pd * p[2];
    let pm = [
        -pr * p[1] - pdz * cr + w * p[0],
        pr * p[0] - pdz * sr + w * p[1],
        pd * cd + w * p[2],
    ];

    // Apply motion and parallax.
    for i in 0..3 {
        p[i] += dt * pm[i] - pxr * pob[i];
    }

    // Normalize to unit vector.
    let (_r, u) = eraPn_safe(&p)?;
    Ok(u)
}

// Proper-motion propagation with safe distance override.
pub fn eraPmsafe_safe(
    ra1: f64,
    dec1: f64,
    pmr1: f64,
    pmd1: f64,
    px1: f64,
    rv1: f64,
    ep1a: f64,
    ep1b: f64,
    ep2a: f64,
    ep2b: f64,
) -> ErfaResult<((f64, f64, f64, f64, f64, f64), i32)> {
    const PXMIN: f64 = 5.0e-7; // arcsec
    const F: f64 = 326.0; // scale giving ~1% c max transverse speed

    // Proper motion magnitude over one year (radians).
    let pm = eraSeps_safe(ra1, dec1, ra1 + pmr1, dec1 + pmd1)?;

    // Override small parallax if implausible given PM.
    let mut jpx = 0;
    let mut px1a = px1;
    let pm_scaled = pm * F;
    if px1a < pm_scaled {
        px1a = pm_scaled;
        jpx = 1;
    }
    if px1a < PXMIN {
        px1a = PXMIN;
        jpx = 1;
    }

    // Propagate star parameters.
    let ((ra2, dec2, pmr2, pmd2, px2, rv2), mut j) =
        eraStarpm_safe(ra1, dec1, pmr1, pmd1, px1a, rv1, ep1a, ep1b, ep2a, ep2b)?;

    // Revise status to reflect parallax override if no error bits set.
    if (j & 1) == 0 {
        j += jpx;
    }

    Ok(((ra2, dec2, pmr2, pmd2, px2, rv2), j))
}

// Decompose p-vector into modulus and unit vector.
pub fn eraPn_safe(p: &[f64; 3]) -> ErfaResult<(f64, [f64; 3])> {
    let w = eraPm_safe(p)?;
    if w == 0.0 {
        let u = eraZp_safe();
        Ok((w, u))
    } else {
        let u = eraSxp_safe(1.0 / w, p)?;
        Ok((w, u))
    }
}

// Precession-nutation, IAU 2000, with supplied dpsi,deps.
pub fn eraPn00_safe(
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
    // IAU 2000 precession-rate corrections.
    let (_dpsipr, depspr) = eraPr00_safe(date1, date2)?;

    // Mean obliquity corrected by depspr only.
    let epsa = eraObl80_safe(date1, date2)? + depspr;

    // Bias and precession matrices.
    let (rb, rp, rbpw) = eraBp00_safe(date1, date2)?;
    let rbp = rbpw;

    // Nutation matrix built from given dpsi,deps (do not add dpsipr to dpsi).
    let rnw = eraNumat_safe(epsa, dpsi, deps)?;
    let rn = rnw;

    // Bias-precession-nutation.
    let rbpn = eraRxr_safe(&rnw, &rbpw)?;

    Ok((epsa, rb, rp, rbp, rn, rbpn))
}

// Precession-nutation, IAU 2000A model.
pub fn eraPn00a_safe(
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
    let (dpsi, deps) = eraNut00a_safe(date1, date2)?;
    let (epsa, rb, rp, rbp, rn, rbpn) = eraPn00_safe(date1, date2, dpsi, deps)?;
    Ok((dpsi, deps, epsa, rb, rp, rbp, rn, rbpn))
}

// Precession-nutation, IAU 2000B model.
pub fn eraPn00b_safe(
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
    let (dpsi, deps) = eraNut00b_safe(date1, date2)?;
    let (epsa, rb, rp, rbp, rn, rbpn) = eraPn00_safe(date1, date2, dpsi, deps)?;
    Ok((dpsi, deps, epsa, rb, rp, rbp, rn, rbpn))
}
