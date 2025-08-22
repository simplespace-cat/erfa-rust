// G20
//   ld.c      → eraLd
//   ldn.c     → eraLdn
//   ldsun.c   → eraLdsun
//   lteceq.c  → eraLteceq
//   ltecm.c   → eraLtecm
//   lteqec.c  → eraLteqec
//   ltp.c     → eraLtp
//   ltpb.c    → eraLtpb
//   ltpecl.c  → eraLtpecl
//   ltpequ.c  → eraLtpequ

use crate::H1::*;

use core::slice;

// eraLd   light deflection by one body
pub unsafe fn eraLd(
    bm: f64,
    p: *mut f64,
    q: *mut f64,
    e: *mut f64,
    em: f64,
    dlim: f64,
    p1: *mut f64,
) {
    let qv = slice::from_raw_parts(q, 3);
    let ev = slice::from_raw_parts(e, 3);
    let mut qpe = [0.0_f64; 3];
    for i in 0..3 {
        qpe[i] = qv[i] + ev[i];
    }
    let qdqpe = eraPdp(q as *mut f64, qpe.as_mut_ptr());

    let w = bm * ERFA_SRS / em / ERFA_GMAX(qdqpe, dlim);

    let mut eq = [0.0_f64; 3];
    eraPxp(e, q, eq.as_mut_ptr());
    let mut peq = [0.0_f64; 3];
    eraPxp(p, eq.as_mut_ptr(), peq.as_mut_ptr());

    let pv = slice::from_raw_parts(p, 3);
    let p1v = slice::from_raw_parts_mut(p1, 3);
    for i in 0..3 {
        p1v[i] = pv[i] + w * peq[i];
    }
}

// eraLdn  light deflection fromN bodies
pub unsafe fn eraLdn(n: i32, b: *mut eraLDBODY, ob: *mut f64, sc: *mut f64, sn: *mut f64) {
    const CR: f64 = ERFA_AULT / ERFA_DAYSEC;

    eraCp(sc, sn);

    let bodies = slice::from_raw_parts(b, n as usize);
    for body in bodies {
        let mut v = [0.0_f64; 3];
        eraPmp(ob, body.pv[0].as_ptr() as *mut f64, v.as_mut_ptr());

        let dt = ERFA_GMIN(eraPdp(sn, v.as_mut_ptr()) * CR, 0.0);

        let mut ev = [0.0_f64; 3];
        eraPpsp(
            v.as_mut_ptr(),
            -dt,
            body.pv[1].as_ptr() as *mut f64,
            ev.as_mut_ptr(),
        );

        let mut em = 0.0_f64;
        let mut e = [0.0_f64; 3];
        eraPn(ev.as_mut_ptr(), &mut em, e.as_mut_ptr());

        eraLd(body.bm, sn, sn, e.as_mut_ptr(), em, body.dl, sn);
    }
}

// eraLdsun  solar light deflection
pub unsafe fn eraLdsun(p: *mut f64, e: *mut f64, em: f64, p1: *mut f64) {
    let em2 = if em * em < 1.0 { 1.0 } else { em * em };
    let dlim = 1.0e-6 / em2;
    eraLd(1.0, p, p, e, em, dlim, p1);
}

// eraLteceq  ecliptic → ICRS (long-term model)
pub unsafe fn eraLteceq(epj: f64, dl: f64, db: f64, dr: *mut f64, dd: *mut f64) {
    let mut rm = [0.0_f64; 9];
    let mut v1 = [0.0_f64; 3];
    let mut v2 = [0.0_f64; 3];

    eraS2c(dl, db, v1.as_mut_ptr());

    eraLtecm(epj, rm.as_mut_ptr());

    eraTrxp(rm.as_mut_ptr(), v1.as_mut_ptr(), v2.as_mut_ptr());

    let mut a = 0.0_f64;
    let mut b = 0.0_f64;
    eraC2s(v2.as_mut_ptr(), &mut a, &mut b);

    *dr = eraAnp(a);
    *dd = eraAnpm(b);
}

// eraLtecm  ICRS → ecliptic rotation matrix (long-term)
pub unsafe fn eraLtecm(epj: f64, rm: *mut f64) {
    const DX: f64 = -0.016_617 * ERFA_DAS2R;
    const DE: f64 = -0.006_819_2 * ERFA_DAS2R;
    const DR: f64 = -0.014_6 * ERFA_DAS2R;

    let mut p = [0.0_f64; 3];
    let mut z = [0.0_f64; 3];
    let mut w = [0.0_f64; 3];
    let mut x = [0.0_f64; 3];
    let mut y = [0.0_f64; 3];
    let mut s = 0.0_f64;

    eraLtpequ(epj, p.as_mut_ptr());
    eraLtpecl(epj, z.as_mut_ptr());

    eraPxp(p.as_mut_ptr(), z.as_mut_ptr(), w.as_mut_ptr());
    eraPn(w.as_mut_ptr(), &mut s, x.as_mut_ptr());

    eraPxp(z.as_mut_ptr(), x.as_mut_ptr(), y.as_mut_ptr());

    let m = slice::from_raw_parts_mut(rm, 9);
    m[0] = x[0] - x[1] * DR + x[2] * DX;
    m[1] = x[0] * DR + x[1] + x[2] * DE;
    m[2] = -x[0] * DX - x[1] * DE + x[2];

    m[3] = y[0] - y[1] * DR + y[2] * DX;
    m[4] = y[0] * DR + y[1] + y[2] * DE;
    m[5] = -y[0] * DX - y[1] * DE + y[2];

    m[6] = z[0] - z[1] * DR + z[2] * DX;
    m[7] = z[0] * DR + z[1] + z[2] * DE;
    m[8] = -z[0] * DX - z[1] * DE + z[2];
}

// eraLteqec  ICRS → ecliptic (long-term model)
pub unsafe fn eraLteqec(epj: f64, dr: f64, dd: f64, dl: *mut f64, db: *mut f64) {
    let mut rm = [0.0_f64; 9];
    let mut v1 = [0.0_f64; 3];
    let mut v2 = [0.0_f64; 3];

    eraS2c(dr, dd, v1.as_mut_ptr());

    eraLtecm(epj, rm.as_mut_ptr());

    eraRxp(rm.as_mut_ptr(), v1.as_mut_ptr(), v2.as_mut_ptr());

    let mut a = 0.0_f64;
    let mut b = 0.0_f64;
    eraC2s(v2.as_mut_ptr(), &mut a, &mut b);

    *dl = eraAnp(a);
    *db = eraAnpm(b);
}

// eraLtp  long-term precession matrix
pub unsafe fn eraLtp(epj: f64, rp: *mut f64) {
    let mut peqr = [0.0_f64; 3];
    let mut pecl = [0.0_f64; 3];
    let mut v = [0.0_f64; 3];
    let mut w = 0.0_f64;
    let mut eqx = [0.0_f64; 3];

    eraLtpequ(epj, peqr.as_mut_ptr());
    eraLtpecl(epj, pecl.as_mut_ptr());

    eraPxp(peqr.as_mut_ptr(), pecl.as_mut_ptr(), v.as_mut_ptr());
    eraPn(v.as_mut_ptr(), &mut w, eqx.as_mut_ptr());
    eraPxp(peqr.as_mut_ptr(), eqx.as_mut_ptr(), v.as_mut_ptr());

    let m = slice::from_raw_parts_mut(rp, 9);
    for i in 0..3 {
        m[i] = eqx[i];
        m[3 + i] = v[i];
        m[6 + i] = peqr[i];
    }
}

// eraLtpb  long-term precession + bias
pub unsafe fn eraLtpb(epj: f64, rpb: *mut f64) {
    const DX: f64 = -0.016_617 * ERFA_DAS2R;
    const DE: f64 = -0.006_819_2 * ERFA_DAS2R;
    const DR: f64 = -0.014_6 * ERFA_DAS2R;

    let mut rp = [0.0_f64; 9];
    eraLtp(epj, rp.as_mut_ptr());

    let dst = slice::from_raw_parts_mut(rpb, 9);
    for i in 0..3 {
        dst[3 * i] = rp[3 * i] - rp[3 * i + 1] * DR + rp[3 * i + 2] * DX;
        dst[3 * i + 1] = rp[3 * i] * DR + rp[3 * i + 1] + rp[3 * i + 2] * DE;
        dst[3 * i + 2] = -rp[3 * i] * DX - rp[3 * i + 1] * DE + rp[3 * i + 2];
    }
}

// eraLtpecl  ecliptic pole unit vector (long-term)
pub unsafe fn eraLtpecl(epj: f64, vec: *mut f64) {
    const EPS0: f64 = 84381.406 * ERFA_DAS2R;

    const PQPOL: [[f64; 4]; 2] = [
        [5851.607687, -0.1189000, -0.00028913, 0.000000101],
        [-1600.886300, 1.1689818, -0.00000020, -0.000000437],
    ];

    const PQPER: [[f64; 5]; 8] = [
        [708.15, -5486.751211, -684.661560, 667.666730, -5523.863691],
        [2309.00, -17.127623, 2446.283880, -2354.886252, -549.747450],
        [1620.00, -617.517403, 399.671049, -428.152441, -310.998056],
        [492.20, 413.442940, -356.652376, 376.202861, 421.535876],
        [1183.00, 78.614193, -186.387003, 184.778874, -36.776172],
        [622.00, -180.732815, -316.800070, 335.321713, -145.278396],
        [882.00, -87.676083, 198.296701, -185.138669, -34.744450],
        [547.00, 46.140315, 101.135679, -120.972830, 22.885731],
    ];

    let t = (epj - 2000.0) / 100.0;

    let mut p = 0.0_f64;
    let mut q = 0.0_f64;
    let w = ERFA_D2PI * t;
    for coeff in PQPER {
        let a = w / coeff[0];
        let (s, c) = a.sin_cos();
        p += c * coeff[1] + s * coeff[3];
        q += c * coeff[2] + s * coeff[4];
    }

    let mut tt = 1.0_f64;
    for i in 0..4 {
        p += PQPOL[0][i] * tt;
        q += PQPOL[1][i] * tt;
        tt *= t;
    }

    p *= ERFA_DAS2R;
    q *= ERFA_DAS2R;

    let mut wv = 1.0 - p * p - q * q;
    wv = if wv < 0.0 { 0.0 } else { wv.sqrt() };
    let (s, c) = EPS0.sin_cos();
    let v = slice::from_raw_parts_mut(vec, 3);
    v[0] = p;
    v[1] = -q * c - wv * s;
    v[2] = -q * s + wv * c;
}

// eraLtpequ  equator pole unit vector (long-term)
pub unsafe fn eraLtpequ(epj: f64, veq: *mut f64) {
    const XYPOL: [[f64; 4]; 2] = [
        [5453.282155, 0.4252841, -0.00037173, -0.000000152],
        [-73750.930350, -0.7675452, -0.00018725, 0.000000231],
    ];

    const XYPER: [[f64; 5]; 14] = [
        [256.75, -819.940624, 75004.344875, 81491.287984, 1558.515853],
        [708.15, -8444.676815, 624.033993, 787.163481, 7774.939698],
        [274.20, 2600.009459, 1251.136893, 1251.296102, -2219.534038],
        [
            241.45,
            2755.175630,
            -1102.212834,
            -1257.950837,
            -2523.969396,
        ],
        [2309.00, -167.659835, -2660.664980, -2966.799730, 247.850422],
        [492.20, 871.855056, 699.291817, 639.744522, -846.485643],
        [396.10, 44.769698, 153.167220, 131.600209, -1393.124055],
        [288.90, -512.313065, -950.865637, -445.040117, 368.526116],
        [231.10, -819.415595, 499.754645, 584.522874, 749.045012],
        [1610.00, -538.071099, -145.188210, -89.756563, 444.704518],
        [620.00, -189.793622, 558.116553, 524.429630, 235.934465],
        [157.87, -402.922932, -23.923029, -13.549067, 374.049623],
        [220.30, 179.516345, -165.405086, -210.157124, -171.330180],
        [1200.00, -9.814756, 9.344131, -44.919798, -22.899655],
    ];

    let t = (epj - 2000.0) / 100.0;

    let mut x = 0.0_f64;
    let mut y = 0.0_f64;
    let w = ERFA_D2PI * t;
    for coeff in XYPER {
        let a = w / coeff[0];
        let (s, c) = a.sin_cos();
        x += c * coeff[1] + s * coeff[3];
        y += c * coeff[2] + s * coeff[4];
    }

    let mut tt = 1.0_f64;
    for i in 0..4 {
        x += XYPOL[0][i] * tt;
        y += XYPOL[1][i] * tt;
        tt *= t;
    }

    x *= ERFA_DAS2R;
    y *= ERFA_DAS2R;

    let v = slice::from_raw_parts_mut(veq, 3);
    v[0] = x;
    v[1] = y;
    let wv = 1.0 - x * x - y * y;
    v[2] = if wv < 0.0 { 0.0 } else { wv.sqrt() };
}
