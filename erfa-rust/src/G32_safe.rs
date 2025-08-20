// G32
//   taitt.c   → eraTaitt_safe
//   taiut1.c  → eraTaiut1_safe
//   taiutc.c  → eraTaiutc_safe
//   tcbtdb.c  → eraTcbtdb_safe
//   tcgtt.c   → eraTcgtt_safe
//   tdbtcb.c  → eraTdbtcb_safe
//   tdbtt.c   → eraTdbtt_safe
//   tf2a.c    → eraTf2a_safe
//   tf2d.c    → eraTf2d_safe
//   tpors.c   → eraTpors_safe
//   tporv.c   → eraTporv_safe
//   tpsts.c   → eraTpsts_safe
//   tpstv.c   → eraTpstv_safe
//   tpxes.c   → eraTpxes_safe
//   tpxev.c   → eraTpxev_safe

use crate::G1_safe::eraAnp_safe;
use crate::G33_safe::eraUtctai_safe;
use crate::H1_safe::{
    ERFA_DAYSEC, ERFA_DJM0, ERFA_DJM77, ERFA_DS2R, ERFA_ELB, ERFA_ELG, ERFA_TDB0, ERFA_TTMTAI,
};

pub type ErfaResult<T> = Result<T, ()>;

// TAI → TT (two-part JD)
pub fn eraTaitt_safe(tai1: f64, tai2: f64) -> ErfaResult<((f64, f64), i32)> {
    const DTAT: f64 = ERFA_TTMTAI / ERFA_DAYSEC;
    let (tt1, tt2) = if tai1.abs() > tai2.abs() {
        (tai1, tai2 + DTAT)
    } else {
        (tai1 + DTAT, tai2)
    };
    Ok(((tt1, tt2), 0))
}

// TAI → UT1 given dTA = UT1−TAI (s)
pub fn eraTaiut1_safe(tai1: f64, tai2: f64, dta: f64) -> ErfaResult<((f64, f64), i32)> {
    let dtad = dta / ERFA_DAYSEC;
    let (ut11, ut12) = if tai1.abs() > tai2.abs() {
        (tai1, tai2 + dtad)
    } else {
        (tai1 + dtad, tai2)
    };
    Ok(((ut11, ut12), 0))
}

// TAI → UTC using iteration with UTC→TAI
pub fn eraTaiutc_safe(tai1: f64, tai2: f64) -> ErfaResult<((f64, f64), i32)> {
    let big1 = tai1.abs() >= tai2.abs();
    let (a1, a2) = if big1 { (tai1, tai2) } else { (tai2, tai1) };

    let (u1, mut u2) = (a1, a2);
    let mut j: i32 = 0;

    for _ in 0..3 {
        let ((g1, g2), jj) = eraUtctai_safe(u1, u2)?;
        j = jj;
        if j < 0 {
            let (utc1, utc2) = if big1 { (u1, u2) } else { (u2, u1) };
            return Ok(((utc1, utc2), j));
        }
        u2 += a1 - g1;
        u2 += a2 - g2;
    }

    let (utc1, utc2) = if big1 { (u1, u2) } else { (u2, u1) };
    Ok(((utc1, utc2), j))
}

// TCB → TDB (two-part JD)
pub fn eraTcbtdb_safe(tcb1: f64, tcb2: f64) -> ErfaResult<((f64, f64), i32)> {
    const T77TD: f64 = ERFA_DJM0 + ERFA_DJM77;
    const T77TF: f64 = ERFA_TTMTAI / ERFA_DAYSEC;
    const TDB0: f64 = ERFA_TDB0 / ERFA_DAYSEC;

    let (tdb1, tdb2) = if tcb1.abs() > tcb2.abs() {
        let d = tcb1 - T77TD;
        (tcb1, tcb2 + TDB0 - (d + (tcb2 - T77TF)) * ERFA_ELB)
    } else {
        let d = tcb2 - T77TD;
        (tcb1 + TDB0 - (d + (tcb1 - T77TF)) * ERFA_ELB, tcb2)
    };
    Ok(((tdb1, tdb2), 0))
}

// TCG → TT (two-part JD)
pub fn eraTcgtt_safe(tcg1: f64, tcg2: f64) -> ErfaResult<((f64, f64), i32)> {
    const T77T: f64 = ERFA_DJM77 + ERFA_TTMTAI / ERFA_DAYSEC;
    let (tt1, tt2) = if tcg1.abs() > tcg2.abs() {
        (tcg1, tcg2 - ((tcg1 - ERFA_DJM0) + (tcg2 - T77T)) * ERFA_ELG)
    } else {
        (tcg1 - ((tcg2 - ERFA_DJM0) + (tcg1 - T77T)) * ERFA_ELG, tcg2)
    };
    Ok(((tt1, tt2), 0))
}

// TDB → TCB (two-part JD)
pub fn eraTdbtcb_safe(tdb1: f64, tdb2: f64) -> ErfaResult<((f64, f64), i32)> {
    const T77TD: f64 = ERFA_DJM0 + ERFA_DJM77;
    const T77TF: f64 = ERFA_TTMTAI / ERFA_DAYSEC;
    const TDB0: f64 = ERFA_TDB0 / ERFA_DAYSEC;
    const ELBB: f64 = ERFA_ELB / (1.0 - ERFA_ELB);

    let (tcb1, tcb2) = if tdb1.abs() > tdb2.abs() {
        let d = T77TD - tdb1;
        let f = tdb2 - TDB0;
        (tdb1, f - (d - (f - T77TF)) * ELBB)
    } else {
        let d = T77TD - tdb2;
        let f = tdb1 - TDB0;
        (f - (d - (f - T77TF)) * ELBB, tdb2)
    };
    Ok(((tcb1, tcb2), 0))
}

// TDB → TT using supplied ΔT_R = TDB−TT (s)
pub fn eraTdbtt_safe(tdb1: f64, tdb2: f64, dtr: f64) -> ErfaResult<((f64, f64), i32)> {
    let dtrd = dtr / ERFA_DAYSEC;
    let (tt1, tt2) = if tdb1.abs() > tdb2.abs() {
        (tdb1, tdb2 - dtrd)
    } else {
        (tdb1 - dtrd, tdb2)
    };
    Ok(((tt1, tt2), 0))
}

// HMS → radians; j=0 OK, 1 bad hour, 2 bad minute, 3 bad second
pub fn eraTf2a_safe(s: char, ihour: i32, imin: i32, sec: f64) -> ErfaResult<(f64, i32)> {
    let sign = if s == '-' { -1.0 } else { 1.0 };
    let rad =
        sign * (60.0 * (60.0 * (ihour.abs() as f64) + (imin.abs() as f64)) + sec.abs()) * ERFA_DS2R;
    if ihour < 0 || ihour > 23 {
        return Ok((rad, 1));
    }
    if imin < 0 || imin > 59 {
        return Ok((rad, 2));
    }
    if sec < 0.0 || sec >= 60.0 {
        return Ok((rad, 3));
    }
    Ok((rad, 0))
}

// HMS → days; j=0 OK, 1 bad hour, 2 bad minute, 3 bad second
pub fn eraTf2d_safe(s: char, ihour: i32, imin: i32, sec: f64) -> ErfaResult<(f64, i32)> {
    let sign = if s == '-' { -1.0 } else { 1.0 };
    let days = sign * (60.0 * (60.0 * (ihour.abs() as f64) + (imin.abs() as f64)) + sec.abs())
        / ERFA_DAYSEC;
    if ihour < 0 || ihour > 23 {
        return Ok((days, 1));
    }
    if imin < 0 || imin > 59 {
        return Ok((days, 2));
    }
    if sec < 0.0 || sec >= 60.0 {
        return Ok((days, 3));
    }
    Ok((days, 0))
}

// Tangent-point from star (spherical); returns two solutions and status
pub fn eraTpors_safe(
    xi: f64,
    eta: f64,
    a: f64,
    b: f64,
) -> ErfaResult<((f64, f64), (f64, f64), i32)> {
    let xi2 = xi * xi;
    let r = (1.0 + xi2 + eta * eta).sqrt();
    let sb = b.sin();
    let cb = b.cos();
    let rsb = r * sb;
    let rcb = r * cb;
    let w2 = rcb * rcb - xi2;

    if w2 >= 0.0 {
        let mut w = w2.sqrt();
        let mut s = rsb - eta * w;
        let mut c = rsb * eta + w;
        if xi == 0.0 && w == 0.0 {
            w = 1.0;
        }
        let a01 = eraAnp_safe(a - xi.atan2(w))?;
        let b01 = s.atan2(c);

        let w2n = -w;
        s = rsb - eta * w2n;
        c = rsb * eta + w2n;
        let a02 = eraAnp_safe(a - xi.atan2(w2n))?;
        let b02 = s.atan2(c);

        let j = if rsb.abs() < 1.0 { 1 } else { 2 };
        Ok(((a01, b01), (a02, b02), j))
    } else {
        Ok(((0.0, 0.0), (0.0, 0.0), 0))
    }
}

// Tangent-point from star (vector); returns two unit vectors and status
pub fn eraTporv_safe(xi: f64, eta: f64, v: &[f64; 3]) -> ErfaResult<([f64; 3], [f64; 3], i32)> {
    let x = v[0];
    let y = v[1];
    let z = v[2];

    let rxy2 = x * x + y * y;
    let xi2 = xi * xi;
    let eta2p1 = eta * eta + 1.0;
    let r = (xi2 + eta2p1).sqrt();
    let rsb = r * z;
    let rcb = r * rxy2.sqrt();
    let w2 = rcb * rcb - xi2;

    if w2 > 0.0 {
        let mut w = w2.sqrt();
        let mut c = (rsb * eta + w) / (eta2p1 * ((rxy2 * (w2 + xi2)).sqrt()));
        let v01 = [
            c * (x * w + y * xi),
            c * (y * w - x * xi),
            (rsb - eta * w) / eta2p1,
        ];

        w = -w;
        c = (rsb * eta + w) / (eta2p1 * ((rxy2 * (w2 + xi2)).sqrt()));
        let v02 = [
            c * (x * w + y * xi),
            c * (y * w - x * xi),
            (rsb - eta * w) / eta2p1,
        ];

        let j = if rsb.abs() < 1.0 { 1 } else { 2 };
        Ok((v01, v02, j))
    } else {
        Ok(([0.0; 3], [0.0; 3], 0))
    }
}

// Star coords from tangent-plane coords and tangent point (spherical)
pub fn eraTpsts_safe(xi: f64, eta: f64, a0: f64, b0: f64) -> ErfaResult<(f64, f64)> {
    let sb0 = b0.sin();
    let cb0 = b0.cos();
    let d = cb0 - eta * sb0;
    let a = eraAnp_safe(xi.atan2(d) + a0)?;
    let b = (sb0 + eta * cb0).atan2((xi * xi + d * d).sqrt());
    Ok((a, b))
}

// Star vector from tangent-plane coords and tangent-point vector
pub fn eraTpstv_safe(xi: f64, eta: f64, v0: &[f64; 3]) -> ErfaResult<[f64; 3]> {
    let mut x = v0[0];
    let y = v0[1];
    let z = v0[2];

    let mut r = (x * x + y * y).sqrt();
    if r == 0.0 {
        r = 1e-20;
        x = r;
    }
    let f = (1.0 + xi * xi + eta * eta).sqrt();

    let vx = (x - (xi * y + eta * x * z) / r) / f;
    let vy = (y + (xi * x - eta * y * z) / r) / f;
    let vz = (z + eta * r) / f;

    Ok([vx, vy, vz])
}

// Solve for (xi,eta) given two sets of spherical coordinates
pub fn eraTpxes_safe(a: f64, b: f64, a0: f64, b0: f64) -> ErfaResult<((f64, f64), i32)> {
    const TINY: f64 = 1e-6;

    let sb0 = b0.sin();
    let sb = b.sin();
    let cb0 = b0.cos();
    let cb = b.cos();
    let da = a - a0;
    let sda = da.sin();
    let cda = da.cos();

    let mut d = sb * sb0 + cb * cb0 * cda;

    let j = if d > TINY {
        0
    } else if d >= 0.0 {
        d = TINY;
        1
    } else if d > -TINY {
        d = -TINY;
        2
    } else {
        3
    };

    let xi = cb * sda / d;
    let eta = (sb * cb0 - cb * sb0 * cda) / d;
    Ok(((xi, eta), j))
}

// Solve for (xi,eta) given two direction-cosine vectors
pub fn eraTpxev_safe(v: &[f64; 3], v0: &[f64; 3]) -> ErfaResult<((f64, f64), i32)> {
    const TINY: f64 = 1e-6;

    let x = v[0];
    let y = v[1];
    let z = v[2];
    let mut x0 = v0[0];
    let y0 = v0[1];
    let z0 = v0[2];

    let r2 = x0 * x0 + y0 * y0;
    let mut r = r2.sqrt();
    if r == 0.0 {
        r = 1e-20;
        x0 = r;
    }

    let w = x * x0 + y * y0;
    let mut d = w + z * z0;

    let j = if d > TINY {
        0
    } else if d >= 0.0 {
        d = TINY;
        1
    } else if d > -TINY {
        d = -TINY;
        2
    } else {
        3
    };

    d *= r;
    let xi = (y * x0 - x * y0) / d;
    let eta = (z * r2 - z0 * w) / d;
    Ok(((xi, eta), j))
}
