// G32
//   taitt.c   → eraTaitt
//   taiut1.c  → eraTaiut1
//   taiutc.c  → eraTaiutc
//   tcbtdb.c  → eraTcbtdb
//   tcgtt.c   → eraTcgtt
//   tdbtcb.c  → eraTdbtcb
//   tdbtt.c   → eraTdbtt
//   tf2a.c    → eraTf2a
//   tf2d.c    → eraTf2d
//   tpors.c   → eraTpors
//   tporv.c   → eraTporv
//   tpsts.c   → eraTpsts
//   tpstv.c   → eraTpstv
//   tpxes.c   → eraTpxes
//   tpxev.c   → eraTpxev

use crate::H1::*;

use core::ffi::c_char;

// eraTaitt   TAI → TT
pub unsafe fn eraTaitt(tai1: f64, tai2: f64, tt1: *mut f64, tt2: *mut f64) -> i32 {
    const DTAT: f64 = ERFA_TTMTAI / ERFA_DAYSEC;
    if tai1.abs() > tai2.abs() {
        *tt1 = tai1;
        *tt2 = tai2 + DTAT;
    } else {
        *tt1 = tai1 + DTAT;
        *tt2 = tai2;
    }
    0
}

// eraTaiut1   TAI → UT1
pub unsafe fn eraTaiut1(tai1: f64, tai2: f64, dta: f64, ut11: *mut f64, ut12: *mut f64) -> i32 {
    let dtad = dta / ERFA_DAYSEC;
    if tai1.abs() > tai2.abs() {
        *ut11 = tai1;
        *ut12 = tai2 + dtad;
    } else {
        *ut11 = tai1 + dtad;
        *ut12 = tai2;
    }
    0
}

// eraTaiutc   TAI → UTC  (iterates with eraUtctai)
pub unsafe fn eraTaiutc(tai1: f64, tai2: f64, utc1: *mut f64, utc2: *mut f64) -> i32 {
    let big1 = tai1.abs() >= tai2.abs();
    let (a1, a2) = if big1 { (tai1, tai2) } else { (tai2, tai1) };

    let (u1, mut u2) = (a1, a2);
    let mut j: i32 = 0;

    for _ in 0..3 {
        let mut g1: f64 = 0.0;
        let mut g2: f64 = 0.0;
        j = eraUtctai(u1, u2, &mut g1, &mut g2);
        if j < 0 {
            return j;
        }
        u2 += a1 - g1;
        u2 += a2 - g2;
    }

    if big1 {
        *utc1 = u1;
        *utc2 = u2;
    } else {
        *utc1 = u2;
        *utc2 = u1;
    }
    j
}

// eraTcbtdb   TCB → TDB
pub unsafe fn eraTcbtdb(tcb1: f64, tcb2: f64, tdb1: *mut f64, tdb2: *mut f64) -> i32 {
    const T77TD: f64 = ERFA_DJM0 + ERFA_DJM77;
    const T77TF: f64 = ERFA_TTMTAI / ERFA_DAYSEC;
    const TDB0: f64 = ERFA_TDB0 / ERFA_DAYSEC;

    if tcb1.abs() > tcb2.abs() {
        let d = tcb1 - T77TD;
        *tdb1 = tcb1;
        *tdb2 = tcb2 + TDB0 - (d + (tcb2 - T77TF)) * ERFA_ELB;
    } else {
        let d = tcb2 - T77TD;
        *tdb1 = tcb1 + TDB0 - (d + (tcb1 - T77TF)) * ERFA_ELB;
        *tdb2 = tcb2;
    }
    0
}

// eraTcgtt   TCG → TT
pub unsafe fn eraTcgtt(tcg1: f64, tcg2: f64, tt1: *mut f64, tt2: *mut f64) -> i32 {
    const T77T: f64 = ERFA_DJM77 + ERFA_TTMTAI / ERFA_DAYSEC;

    if tcg1.abs() > tcg2.abs() {
        *tt1 = tcg1;
        *tt2 = tcg2 - ((tcg1 - ERFA_DJM0) + (tcg2 - T77T)) * ERFA_ELG;
    } else {
        *tt1 = tcg1 - ((tcg2 - ERFA_DJM0) + (tcg1 - T77T)) * ERFA_ELG;
        *tt2 = tcg2;
    }
    0
}

// eraTdbtcb   TDB → TCB
pub unsafe fn eraTdbtcb(tdb1: f64, tdb2: f64, tcb1: *mut f64, tcb2: *mut f64) -> i32 {
    const T77TD: f64 = ERFA_DJM0 + ERFA_DJM77;
    const T77TF: f64 = ERFA_TTMTAI / ERFA_DAYSEC;
    const TDB0: f64 = ERFA_TDB0 / ERFA_DAYSEC;
    const ELBB: f64 = ERFA_ELB / (1.0 - ERFA_ELB);

    if tdb1.abs() > tdb2.abs() {
        let d = T77TD - tdb1;
        let f = tdb2 - TDB0;
        *tcb1 = tdb1;
        *tcb2 = f - (d - (f - T77TF)) * ELBB;
    } else {
        let d = T77TD - tdb2;
        let f = tdb1 - TDB0;
        *tcb1 = f - (d - (f - T77TF)) * ELBB;
        *tcb2 = tdb2;
    }
    0
}

// eraTdbtt   TDB → TT (using supplied ΔT_R = TDB-TT)
pub unsafe fn eraTdbtt(tdb1: f64, tdb2: f64, dtr: f64, tt1: *mut f64, tt2: *mut f64) -> i32 {
    let dtrd = dtr / ERFA_DAYSEC;
    if tdb1.abs() > tdb2.abs() {
        *tt1 = tdb1;
        *tt2 = tdb2 - dtrd;
    } else {
        *tt1 = tdb1 - dtrd;
        *tt2 = tdb2;
    }
    0
}

// eraTf2a   HMS → radians
pub unsafe fn eraTf2a(s: c_char, ihour: i32, imin: i32, sec: f64, rad: *mut f64) -> i32 {
    *rad = if s == b'-' as c_char { -1.0 } else { 1.0 }
        * (60.0 * (60.0 * (ihour.abs() as f64) + (imin.abs() as f64)) + sec.abs())
        * ERFA_DS2R;

    if ihour < 0 || ihour > 23 {
        return 1;
    }
    if imin < 0 || imin > 59 {
        return 2;
    }
    if sec < 0.0 || sec >= 60.0 {
        return 3;
    }
    0
}

// eraTf2d   HMS → days
pub unsafe fn eraTf2d(s: c_char, ihour: i32, imin: i32, sec: f64, days: *mut f64) -> i32 {
    *days = if s == b'-' as c_char { -1.0 } else { 1.0 }
        * (60.0 * (60.0 * (ihour.abs() as f64) + (imin.abs() as f64)) + sec.abs())
        / ERFA_DAYSEC;

    if ihour < 0 || ihour > 23 {
        return 1;
    }
    if imin < 0 || imin > 59 {
        return 2;
    }
    if sec < 0.0 || sec >= 60.0 {
        return 3;
    }
    0
}

// eraTpors   solve for tangent-point coordinates (spherical inputs)
pub unsafe fn eraTpors(
    xi: f64,
    eta: f64,
    a: f64,
    b: f64,
    a01: *mut f64,
    b01: *mut f64,
    a02: *mut f64,
    b02: *mut f64,
) -> i32 {
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
        *a01 = eraAnp(a - (xi).atan2(w));
        *b01 = s.atan2(c);

        w = -w;
        s = rsb - eta * w;
        c = rsb * eta + w;
        *a02 = eraAnp(a - (xi).atan2(w));
        *b02 = s.atan2(c);

        if rsb.abs() < 1.0 {
            1
        } else {
            2
        }
    } else {
        0
    }
}

// eraTporv   solve for tangent-point (vector inputs)
pub unsafe fn eraTporv(xi: f64, eta: f64, v: *mut f64, v01: *mut f64, v02: *mut f64) -> i32 {
    let x = *v.add(0);
    let y = *v.add(1);
    let z = *v.add(2);

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
        *v01.add(0) = c * (x * w + y * xi);
        *v01.add(1) = c * (y * w - x * xi);
        *v01.add(2) = (rsb - eta * w) / eta2p1;

        w = -w;
        c = (rsb * eta + w) / (eta2p1 * ((rxy2 * (w2 + xi2)).sqrt()));
        *v02.add(0) = c * (x * w + y * xi);
        *v02.add(1) = c * (y * w - x * xi);
        *v02.add(2) = (rsb - eta * w) / eta2p1;

        if rsb.abs() < 1.0 {
            1
        } else {
            2
        }
    } else {
        0
    }
}

// eraTpsts   star coordinates from tangent-plane coords + tangent point
pub unsafe fn eraTpsts(xi: f64, eta: f64, a0: f64, b0: f64, a: *mut f64, b: *mut f64) {
    let sb0 = b0.sin();
    let cb0 = b0.cos();
    let d = cb0 - eta * sb0;
    *a = eraAnp((xi).atan2(d) + a0);
    *b = (sb0 + eta * cb0).atan2((xi * xi + d * d).sqrt());
}

// eraTpstv   star vector from tangent-plane coords + tangent-point vector
pub unsafe fn eraTpstv(xi: f64, eta: f64, v0: *mut f64, v: *mut f64) {
    let mut x = *v0.add(0);
    let y = *v0.add(1);
    let z = *v0.add(2);

    let mut r = (x * x + y * y).sqrt();
    if r == 0.0 {
        r = 1e-20;
        x = r;
    }
    let f = (1.0 + xi * xi + eta * eta).sqrt();

    *v.add(0) = (x - (xi * y + eta * x * z) / r) / f;
    *v.add(1) = (y + (xi * x - eta * y * z) / r) / f;
    *v.add(2) = (z + eta * r) / f;
}

// eraTpxes   solve for (xi,eta) given two sets of spherical coords
pub unsafe fn eraTpxes(a: f64, b: f64, a0: f64, b0: f64, xi: *mut f64, eta: *mut f64) -> i32 {
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

    *xi = cb * sda / d;
    *eta = (sb * cb0 - cb * sb0 * cda) / d;
    j
}

// eraTpxev   solve for (xi,eta) given two direction-cosine vectors
pub unsafe fn eraTpxev(v: *mut f64, v0: *mut f64, xi: *mut f64, eta: *mut f64) -> i32 {
    const TINY: f64 = 1e-6;

    let x = *v.add(0);
    let y = *v.add(1);
    let z = *v.add(2);
    let mut x0 = *v0.add(0);
    let y0 = *v0.add(1);
    let z0 = *v0.add(2);

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
    *xi = (y * x0 - x * y0) / d;
    *eta = (z * r2 - z0 * w) / d;
    j
}
