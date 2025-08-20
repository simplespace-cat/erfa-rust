// G19
//   icrs2g.c  → eraIcrs2g_safe
//   ir.c      → eraIr_safe
//   jd2cal.c  → eraJd2cal_safe
//   jdcalf.c  → eraJdcalf_safe

use crate::G1_safe::{eraAnp_safe, eraAnpm_safe};
use crate::G28_safe::eraRxp_safe;
use crate::G29_safe::eraS2c_safe;
use crate::G7_safe::eraC2s_safe;
use crate::H1_safe::ERFA_DNINT;

pub type ErfaResult<T> = Result<T, ()>;

//----------------------------------------------------------------------
// G19/icrs2g.c → eraIcrs2g_safe
//----------------------------------------------------------------------
// Convert ICRS (dr, dd) to Galactic (dl, db), radians.
pub fn eraIcrs2g_safe(dr: f64, dd: f64) -> ErfaResult<(f64, f64)> {
    // ICRS→Galactic rotation matrix (row-major).
    const R: [[f64; 3]; 3] = [
        [
            -0.054_875_560_416_215_368_492_398_900_454,
            -0.873_437_090_234_885_048_760_383_168_409,
            -0.483_835_015_548_713_226_831_774_175_116,
        ],
        [
            0.494_109_427_875_583_673_525_222_371_358,
            -0.444_829_629_960_011_178_146_614_061_616,
            0.746_982_244_497_218_890_527_388_004_556,
        ],
        [
            -0.867_666_149_019_004_701_181_616_534_570,
            -0.198_076_373_431_201_528_180_486_091_412,
            0.455_983_776_175_066_922_272_100_478_348,
        ],
    ];

    let v1 = eraS2c_safe(dr, dd)?;
    let v2 = eraRxp_safe(&R, &v1)?;
    let (mut dl, mut db) = eraC2s_safe(&v2)?;
    dl = eraAnp_safe(dl)?;
    db = eraAnpm_safe(db)?;
    Ok((dl, db))
}

//----------------------------------------------------------------------
// G19/ir.c → eraIr_safe
//----------------------------------------------------------------------
// Set 3×3 matrix to identity.
pub fn eraIr_safe(r: &mut [[f64; 3]; 3]) -> ErfaResult<()> {
    for i in 0..3 {
        for j in 0..3 {
            r[i][j] = if i == j { 1.0 } else { 0.0 };
        }
    }
    Ok(())
}

//----------------------------------------------------------------------
// G19/jd2cal.c → eraJd2cal_safe
//----------------------------------------------------------------------
// Two-part JD to Gregorian (iy, im, id) and fractional day fd.
// Returns ((iy, im, id), fd, j) with j=0 OK, -1 out of range.
pub fn eraJd2cal_safe(dj1: f64, dj2: f64) -> ErfaResult<((i32, i32, i32), f64, i32)> {
    const DJMIN: f64 = -68_569.5;
    const DJMAX: f64 = 1e9;

    // Range check.
    let dj = dj1 + dj2;
    if dj < DJMIN || dj > DJMAX {
        return Ok(((0, 0, 0), 0.0, -1));
    }

    // Separate integer and fractional parts (compensated summation).
    let mut jd = ERFA_DNINT(dj1) as i64;
    let f1 = dj1 - jd as f64;
    let d = ERFA_DNINT(dj2);
    let f2 = dj2 - d;
    jd += d as i64;

    // Add 0.5 with compensation (KahanNeumaier).
    let mut s = 0.5;
    let mut cs = 0.0;
    for x in [f1, f2] {
        let t = s + x;
        cs += if s.abs() >= x.abs() {
            (s - t) + x
        } else {
            (x - t) + s
        };
        s = t;
        if s >= 1.0 {
            jd += 1;
            s -= 1.0;
        }
    }
    let mut f = s + cs;

    // Handle negative fraction.
    if f < 0.0 {
        f += 1.0;
        jd -= 1;
    }

    // Handle rounding up to 1.0.
    if (f - 1.0) >= -f64::EPSILON / 4.0 {
        f -= 1.0;
        jd += 1;
        if f < 0.0 {
            f = 0.0;
        }
    }

    // Gregorian calendar from integer JD (Fliegel & Van Flandern).
    let mut l = jd + 68_569;
    let n = (4 * l) / 146_097;
    l -= (146_097 * n + 3) / 4;
    let i = (4_000 * (l + 1)) / 1_461_001;
    l -= (1_461 * i) / 4 - 31;
    let k = (80 * l) / 2_447;

    let id = (l - (2_447 * k) / 80) as i32;
    l = k / 11;
    let im = (k + 2 - 12 * l) as i32;
    let iy = (100 * (n - 49) + i + l) as i32;

    Ok(((iy, im, id), f, 0))
}

//----------------------------------------------------------------------
// G19/jdcalf.c → eraJdcalf_safe
//----------------------------------------------------------------------
// JD to calendar with ndp decimals; returns [iy, im, id, frac×10^ndp] and status.
pub fn eraJdcalf_safe(ndp: i32, dj1: f64, dj2: f64) -> ErfaResult<([i32; 4], i32)> {
    let (mut jstat, denom) = if (0..=9).contains(&ndp) {
        (0, 10_f64.powi(ndp))
    } else {
        (1, 1.0)
    };

    // Arrange parts so |d1| ≥ |d2|.
    let (mut d1, d2) = if dj1.abs() >= dj2.abs() {
        (dj1, dj2)
    } else {
        (dj2, dj1)
    };

    // Realign to midnight.
    d1 -= 0.5;

    // Separate day & fraction.
    let mut d = ERFA_DNINT(d1);
    let f1 = d1 - d;
    let mut djd = d;
    d = ERFA_DNINT(d2);
    let f2 = d2 - d;
    djd += d;
    d = ERFA_DNINT(f1 + f2);
    let mut f = (f1 - d) + f2;
    if f < 0.0 {
        f += 1.0;
        d -= 1.0;
    }
    djd += d;

    // Round fraction to requested decimals.
    let rf = ERFA_DNINT(f * denom) / denom;

    // Re-align to noon.
    djd += 0.5;

    // Convert to calendar date.
    let ((year, month, day), frac, js) = eraJd2cal_safe(djd, rf)?;
    let mut out = [0i32; 4];
    if js == 0 {
        let frac_i = ERFA_DNINT(frac * denom) as i32;
        out[0] = year;
        out[1] = month;
        out[2] = day;
        out[3] = frac_i;
    } else {
        jstat = js;
    }

    Ok((out, jstat))
}
