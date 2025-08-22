// G19
//   icrs2g.c  → eraIcrs2g
//   ir.c      → eraIr
//   jd2cal.c  → eraJd2cal
//   jdcalf.c  → eraJdcalf

use crate::H1::*;

// eraIcrs2g    ICRS → Galactic coordinates
pub unsafe fn eraIcrs2g(dr: f64, dd: f64, dl: *mut f64, db: *mut f64) {
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

    let mut v1 = [0.0_f64; 3];
    let mut v2 = [0.0_f64; 3];

    eraS2c(dr, dd, v1.as_mut_ptr());

    eraRxp(R.as_ptr() as *mut f64, v1.as_mut_ptr(), v2.as_mut_ptr());

    eraC2s(v2.as_mut_ptr(), dl, db);

    *dl = eraAnp(*dl);
    *db = eraAnpm(*db);
}

// eraIr    Identity 3×3 matrix
pub unsafe fn eraIr(r: *mut f64) {
    for i in 0..9 {
        *r.add(i) = if i % 4 == 0 { 1.0 } else { 0.0 };
    }
}

// eraJd2cal    Julian Date → Gregorian Y/M/D + fraction
pub unsafe fn eraJd2cal(
    dj1: f64,
    dj2: f64,
    iy: *mut i32,
    im: *mut i32,
    id: *mut i32,
    fd: *mut f64,
) -> i32 {
    const DJMIN: f64 = -68_569.5;
    const DJMAX: f64 = 1e9;

    let dj = dj1 + dj2;
    if dj < DJMIN || dj > DJMAX {
        return -1;
    }

    let mut jd = ERFA_DNINT(dj1) as i64;
    let f1 = dj1 - jd as f64;
    let d = ERFA_DNINT(dj2);
    let f2 = dj2 - d;
    jd += d as i64;

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

    if f < 0.0 {
        f += 1.0;
        jd -= 1;
    }

    if (f - 1.0) >= -f64::EPSILON / 4.0 {
        f -= 1.0;
        jd += 1;
        if f < 0.0 {
            f = 0.0;
        }
    }

    let mut l = jd + 68_569;
    let n = (4 * l) / 146_097;
    l -= (146_097 * n + 3) / 4;
    let i = (4_000 * (l + 1)) / 1_461_001;
    l -= (1_461 * i) / 4 - 31;
    let k = (80 * l) / 2_447;

    *id = (l - (2_447 * k) / 80) as i32;
    l = k / 11;
    *im = (k + 2 - 12 * l) as i32;
    *iy = (100 * (n - 49) + i + l) as i32;
    *fd = f;

    0
}

// eraJdcalf    JD → Gregorian calendar (rounded, four-field)
pub unsafe fn eraJdcalf(ndp: i32, dj1: f64, dj2: f64, iymdf: *mut i32) -> i32 {
    let (mut jstat, denom) = if (0..=9).contains(&ndp) {
        (0, 10_f64.powi(ndp))
    } else {
        (1, 1.0)
    };

    let (mut d1, d2) = if dj1.abs() >= dj2.abs() {
        (dj1, dj2)
    } else {
        (dj2, dj1)
    };

    d1 -= 0.5;

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

    let rf = ERFA_DNINT(f * denom) / denom;

    djd += 0.5;

    let mut year = 0;
    let mut month = 0;
    let mut day = 0;
    let mut frac = 0.0;
    let js = eraJd2cal(djd, rf, &mut year, &mut month, &mut day, &mut frac);
    if js == 0 {
        let frac_i = ERFA_DNINT(frac * denom) as i32;
        *iymdf.add(0) = year;
        *iymdf.add(1) = month;
        *iymdf.add(2) = day;
        *iymdf.add(3) = frac_i;
    } else {
        jstat = js;
    }

    jstat
}
