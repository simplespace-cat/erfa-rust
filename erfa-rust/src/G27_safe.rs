// G27
//   prec76.c  → eraPrec76_safe
//   pv2p.c    → eraPv2p_safe
//   pv2s.c    → eraPv2s_safe
//   pvdpv.c   → eraPvdpv_safe
//   pvm.c     → eraPvm_safe
//   pvmpv.c   → eraPvmpv_safe
//   pvppv.c   → eraPvppv_safe
//   pvstar.c  → eraPvstar_safe
//   pvtob.c   → eraPvtob_safe
//   pvu.c     → eraPvu_safe
//   pvup.c    → eraPvup_safe
//   pvxpv.c   → eraPvxpv_safe
//   pxp.c     → eraPxp_safe
use crate::H1_safe::{
    ERFA_D2PI, ERFA_DAS2R, ERFA_DAU, ERFA_DAYSEC, ERFA_DC, ERFA_DJ00, ERFA_DJC, ERFA_DJY,
    ERFA_DR2AS,
};

use crate::G17_safe::eraGd2gc_safe;
use crate::G1_safe::eraAnp_safe;
use crate::G24_safe::{eraPdp_safe, eraPm_safe};
use crate::G25_safe::{eraPmp_safe, eraPn_safe};
use crate::G26_safe::{eraPom00_safe, eraPpp_safe, eraPpsp_safe};
use crate::G30_safe::eraSxp_safe;
use crate::G33_safe::eraTrxp_safe;
use crate::G8_safe::{eraCp_safe, eraCpv_safe};

pub type ErfaResult<T> = Result<T, ()>;

// Compute IAU 1976 precession angles (zeta, z, theta) between two epochs.
pub fn eraPrec76_safe(
    date01: f64,
    date02: f64,
    date11: f64,
    date12: f64,
) -> ErfaResult<(f64, f64, f64)> {
    let t0 = ((date01 - ERFA_DJ00) + date02) / ERFA_DJC;
    let t = ((date11 - date01) + (date12 - date02)) / ERFA_DJC;
    let tas2r = t * ERFA_DAS2R;
    let w = 2306.2181 + (1.39656 - 0.000139 * t0) * t0;
    let zeta = (w + ((0.30188 - 0.000344 * t0) + 0.017998 * t) * t) * tas2r;
    let z = (w + ((1.09468 + 0.000066 * t0) + 0.018203 * t) * t) * tas2r;
    let theta = ((2004.3109 + (-0.85330 - 0.000217 * t0) * t0)
        + ((-0.42665 - 0.000217 * t0) - 0.041833 * t) * t)
        * tas2r;
    Ok((zeta, z, theta))
}

// Extract position component from pv-vector.
pub fn eraPv2p_safe(pv: &[[f64; 3]; 2]) -> ErfaResult<[f64; 3]> {
    Ok(pv[0])
}

// Convert pv-vector to spherical angles and their rates.
pub fn eraPv2s_safe(pv: &[[f64; 3]; 2]) -> ErfaResult<(f64, f64, f64, f64, f64, f64)> {
    let mut x = pv[0][0];
    let mut y = pv[0][1];
    let mut z = pv[0][2];
    let xd = pv[1][0];
    let yd = pv[1][1];
    let zd = pv[1][2];

    let mut rxy2 = x * x + y * y;
    let mut r2 = rxy2 + z * z;
    let rtrue = r2.sqrt();

    let rw = if rtrue == 0.0 {
        x = xd;
        y = yd;
        z = zd;
        rxy2 = x * x + y * y;
        r2 = rxy2 + z * z;
        r2.sqrt()
    } else {
        rtrue
    };

    let rxy = rxy2.sqrt();
    let xyp = x * xd + y * yd;
    let (theta, phi, td, pd) = if rxy2 != 0.0 {
        let theta = y.atan2(x);
        let phi = z.atan2(rxy);
        let td = (x * yd - y * xd) / rxy2;
        let pd = (zd * rxy2 - z * xyp) / (r2 * rxy);
        (theta, phi, td, pd)
    } else {
        let theta = 0.0;
        let phi = if z != 0.0 { z.atan2(rxy) } else { 0.0 };
        let td = 0.0;
        let pd = 0.0;
        (theta, phi, td, pd)
    };
    let r = rtrue;
    let rd = if rw != 0.0 { (xyp + z * zd) / rw } else { 0.0 };

    Ok((theta, phi, r, td, pd, rd))
}

// Dot product of two pv-vectors and its time derivative.
pub fn eraPvdpv_safe(a: &[[f64; 3]; 2], b: &[[f64; 3]; 2]) -> ErfaResult<(f64, f64)> {
    let p_dot = eraPdp_safe(&a[0], &b[0])?;
    let adbd = eraPdp_safe(&a[0], &b[1])?;
    let addb = eraPdp_safe(&a[1], &b[0])?;
    let d_dot = adbd + addb;
    Ok((p_dot, d_dot))
}

// Moduli of position and velocity from a pv-vector.
pub fn eraPvm_safe(pv: &[[f64; 3]; 2]) -> ErfaResult<(f64, f64)> {
    let r = eraPm_safe(&pv[0])?;
    let s = eraPm_safe(&pv[1])?;
    Ok((r, s))
}

// Subtract two pv-vectors component-wise.
pub fn eraPvmpv_safe(a: &[[f64; 3]; 2], b: &[[f64; 3]; 2]) -> ErfaResult<[[f64; 3]; 2]> {
    let p = eraPmp_safe(&a[0], &b[0])?;
    let v = eraPmp_safe(&a[1], &b[1])?;
    Ok([p, v])
}

// Add two pv-vectors component-wise.
pub fn eraPvppv_safe(a: &[[f64; 3]; 2], b: &[[f64; 3]; 2]) -> ErfaResult<[[f64; 3]; 2]> {
    let p = eraPpp_safe(&a[0], &b[0])?;
    let v = eraPpp_safe(&a[1], &b[1])?;
    Ok([p, v])
}

// Convert pv-vector to catalog parameters (ra,dec,pmr,pmd,px,rv); j=-1 superluminal, -2 null pos.
pub fn eraPvstar_safe(pv: &[[f64; 3]; 2]) -> ErfaResult<((f64, f64, f64, f64, f64, f64), i32)> {
    let (_r, pu) = eraPn_safe(&pv[0])?;
    let vr = eraPdp_safe(&pu, &pv[1])?;
    let ur = eraSxp_safe(vr, &pu)?;
    let ut = eraPmp_safe(&pv[1], &ur)?;
    let vt = eraPm_safe(&ut)?;

    let bett = vt / ERFA_DC;
    let betr = vr / ERFA_DC;
    let d = 1.0 + betr;
    let w = betr * betr + bett * bett;
    if d == 0.0 || w > 1.0 {
        return Ok(((0.0, 0.0, 0.0, 0.0, 0.0, 0.0), -1));
    }
    let del = -w / ((1.0 - w).sqrt() + 1.0);

    let ust = eraSxp_safe(1.0 / d, &ut)?;
    let usr = eraSxp_safe(ERFA_DC * (betr - del) / d, &pu)?;
    let vnew = eraPpp_safe(&usr, &ust)?;
    let pv_mod = [pv[0], vnew];

    let (a, dec, r_out, rad, decd, rd) = eraPv2s_safe(&pv_mod)?;
    if r_out == 0.0 {
        return Ok(((0.0, 0.0, 0.0, 0.0, 0.0, 0.0), -2));
    }

    let ra = eraAnp_safe(a)?;
    let pmr = rad * ERFA_DJY;
    let pmd = decd * ERFA_DJY;
    let px = ERFA_DR2AS / r_out;
    let rv = 1e-3 * rd * ERFA_DAU / ERFA_DAYSEC;

    Ok(((ra, dec, pmr, pmd, px, rv), 0))
}

// Observer geocentric position/velocity from site geodetic coordinates.
pub fn eraPvtob_safe(
    elong: f64,
    phi: f64,
    hm: f64,
    xp: f64,
    yp: f64,
    sp: f64,
    theta: f64,
) -> ErfaResult<[[f64; 3]; 2]> {
    const OM: f64 = 1.002_737_811_911_354_48 * ERFA_D2PI / ERFA_DAYSEC;

    let (xyzm, _j) = eraGd2gc_safe(1, elong, phi, hm)?;
    let rpm = eraPom00_safe(xp, yp, sp)?;
    let xyz = eraTrxp_safe(&rpm, &xyzm)?;

    let x = xyz[0];
    let y = xyz[1];
    let z = xyz[2];
    let (s, c) = theta.sin_cos();

    let px = c * x - s * y;
    let py = s * x + c * y;
    let pz = z;

    let vx = OM * (-s * x - c * y);
    let vy = OM * (c * x - s * y);
    let vz = 0.0;

    Ok([[px, py, pz], [vx, vy, vz]])
}

// Advance a pv-vector by time dt (same units as velocity denominator).
pub fn eraPvu_safe(dt: f64, pv: &[[f64; 3]; 2]) -> ErfaResult<[[f64; 3]; 2]> {
    let p = eraPpsp_safe(&pv[0], dt, &pv[1])?;
    let mut v = [0.0_f64; 3];
    eraCp_safe(&pv[1], &mut v)?;
    Ok([p, v])
}

// Advance position only by time dt using a pv-vector.
pub fn eraPvup_safe(dt: f64, pv: &[[f64; 3]; 2]) -> ErfaResult<[f64; 3]> {
    Ok([
        pv[0][0] + dt * pv[1][0],
        pv[0][1] + dt * pv[1][1],
        pv[0][2] + dt * pv[1][2],
    ])
}

// Cross product of two pv-vectors, returning position and its derivative.
pub fn eraPvxpv_safe(a: &[[f64; 3]; 2], b: &[[f64; 3]; 2]) -> ErfaResult<[[f64; 3]; 2]> {
    let mut wa = [[0.0_f64; 3]; 2];
    let mut wb = [[0.0_f64; 3]; 2];
    eraCpv_safe(a, &mut wa)?;
    eraCpv_safe(b, &mut wb)?;

    let p = eraPxp_safe(&wa[0], &wb[0])?;
    let axbd = eraPxp_safe(&wa[0], &wb[1])?;
    let adxb = eraPxp_safe(&wa[1], &wb[0])?;
    let v = eraPpp_safe(&axbd, &adxb)?;
    Ok([p, v])
}

// 3D vector cross product.
pub fn eraPxp_safe(a: &[f64; 3], b: &[f64; 3]) -> ErfaResult<[f64; 3]> {
    let xa = a[0];
    let ya = a[1];
    let za = a[2];
    let xb = b[0];
    let yb = b[1];
    let zb = b[2];

    Ok([ya * zb - za * yb, za * xb - xa * zb, xa * yb - ya * xb])
}
