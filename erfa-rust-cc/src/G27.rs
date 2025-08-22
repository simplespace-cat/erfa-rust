// G27
//   prec76.c  → eraPrec76
//   pv2p.c    → eraPv2p
//   pv2s.c    → eraPv2s
//   pvdpv.c   → eraPvdpv
//   pvm.c     → eraPvm
//   pvmpv.c   → eraPvmpv
//   pvppv.c   → eraPvppv
//   pvstar.c  → eraPvstar
//   pvtob.c   → eraPvtob
//   pvu.c     → eraPvu
//   pvup.c    → eraPvup
//   pvxpv.c   → eraPvxpv
//   pxp.c     → eraPxp

use crate::H1::*;

pub unsafe fn eraPrec76(
    date01: f64,
    date02: f64,
    date11: f64,
    date12: f64,
    zeta: *mut f64,
    z: *mut f64,
    theta: *mut f64,
) {
    let t0 = ((date01 - ERFA_DJ00) + date02) / ERFA_DJC;

    let t = ((date11 - date01) + (date12 - date02)) / ERFA_DJC;

    let tas2r = t * ERFA_DAS2R;

    let w = 2306.2181 + (1.39656 - 0.000139 * t0) * t0;

    *zeta = (w + ((0.30188 - 0.000344 * t0) + 0.017998 * t) * t) * tas2r;
    *z = (w + ((1.09468 + 0.000066 * t0) + 0.018203 * t) * t) * tas2r;
    *theta = ((2004.3109 + (-0.85330 - 0.000217 * t0) * t0)
        + ((-0.42665 - 0.000217 * t0) - 0.041833 * t) * t)
        * tas2r;
}

pub unsafe fn eraPv2p(pv: *mut f64, p: *mut f64) {
    for i in 0..3 {
        *p.add(i) = *pv.add(i);
    }
}

pub unsafe fn eraPv2s(
    pv: *mut f64,
    theta: *mut f64,
    phi: *mut f64,
    r: *mut f64,
    td: *mut f64,
    pd: *mut f64,
    rd: *mut f64,
) {
    let mut x = *pv.add(0);
    let mut y = *pv.add(1);
    let mut z = *pv.add(2);
    let xd = *pv.add(3);
    let yd = *pv.add(4);
    let zd = *pv.add(5);

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
    if rxy2 != 0.0 {
        *theta = y.atan2(x);
        *phi = z.atan2(rxy);
        *td = (x * yd - y * xd) / rxy2;
        *pd = (zd * rxy2 - z * xyp) / (r2 * rxy);
    } else {
        *theta = 0.0;
        *phi = if z != 0.0 { z.atan2(rxy) } else { 0.0 };
        *td = 0.0;
        *pd = 0.0;
    }
    *r = rtrue;
    *rd = if rw != 0.0 { (xyp + z * zd) / rw } else { 0.0 };
}

pub unsafe fn eraPvdpv(a: *mut f64, b: *mut f64, adb: *mut f64) {
    *adb.add(0) = eraPdp(a, b);
    let adbd = eraPdp(a, b.add(3));
    let addb = eraPdp(a.add(3), b);
    *adb.add(1) = adbd + addb;
}

pub unsafe fn eraPvm(pv: *mut f64, r: *mut f64, s: *mut f64) {
    *r = eraPm(pv);
    *s = eraPm(pv.add(3));
}

pub unsafe fn eraPvmpv(a: *mut f64, b: *mut f64, amb: *mut f64) {
    eraPmp(a, b, amb);
    eraPmp(a.add(3), b.add(3), amb.add(3));
}

pub unsafe fn eraPvppv(a: *mut f64, b: *mut f64, apb: *mut f64) {
    eraPpp(a, b, apb);
    eraPpp(a.add(3), b.add(3), apb.add(3));
}

pub unsafe fn eraPvstar(
    pv: *mut f64,
    ra: *mut f64,
    dec: *mut f64,
    pmr: *mut f64,
    pmd: *mut f64,
    px: *mut f64,
    rv: *mut f64,
) -> i32 {
    let mut r = 0.0;
    let mut pu = [0.0f64; 3];
    eraPn(pv, &mut r, pu.as_mut_ptr());

    let vr = eraPdp(pu.as_mut_ptr(), pv.add(3));
    let mut ur = [0.0f64; 3];
    eraSxp(vr, pu.as_mut_ptr(), ur.as_mut_ptr());

    let mut ut = [0.0f64; 3];
    eraPmp(pv.add(3), ur.as_mut_ptr(), ut.as_mut_ptr());
    let vt = eraPm(ut.as_mut_ptr());

    let bett = vt / ERFA_DC;
    let betr = vr / ERFA_DC;
    let d = 1.0 + betr;
    let w = betr * betr + bett * bett;
    if d == 0.0 || w > 1.0 {
        return -1;
    }
    let del = -w / ((1.0 - w).sqrt() + 1.0);

    let mut ust = [0.0f64; 3];
    eraSxp(1.0 / d, ut.as_mut_ptr(), ust.as_mut_ptr());

    let mut usr = [0.0f64; 3];
    eraSxp(
        ERFA_DC * (betr - del) / d,
        pu.as_mut_ptr(),
        usr.as_mut_ptr(),
    );

    eraPpp(usr.as_mut_ptr(), ust.as_mut_ptr(), pv.add(3));

    let mut a = 0.0;
    let mut rad = 0.0;
    let mut decd = 0.0;
    let mut rd = 0.0;
    eraPv2s(pv, &mut a, dec, &mut r, &mut rad, &mut decd, &mut rd);
    if r == 0.0 {
        return -2;
    }

    *ra = eraAnp(a);
    *pmr = rad * ERFA_DJY;
    *pmd = decd * ERFA_DJY;
    *px = ERFA_DR2AS / r;
    *rv = 1e-3 * rd * ERFA_DAU / ERFA_DAYSEC;
    0
}

pub unsafe fn eraPvtob(
    elong: f64,
    phi: f64,
    hm: f64,
    xp: f64,
    yp: f64,
    sp: f64,
    theta: f64,
    pv: *mut f64,
) {
    const OM: f64 = 1.002_737_811_911_354_48 * ERFA_D2PI / ERFA_DAYSEC;

    let mut xyzm = [0.0f64; 3];
    let mut rpm = [0.0f64; 9];
    let mut xyz = [0.0f64; 3];

    eraGd2gc(1, elong, phi, hm, xyzm.as_mut_ptr());

    eraPom00(xp, yp, sp, rpm.as_mut_ptr());
    eraTrxp(rpm.as_mut_ptr(), xyzm.as_mut_ptr(), xyz.as_mut_ptr());

    let x = xyz[0];
    let y = xyz[1];
    let z = xyz[2];
    let (s, c) = theta.sin_cos();

    *pv.add(0) = c * x - s * y;
    *pv.add(1) = s * x + c * y;
    *pv.add(2) = z;

    *pv.add(3) = OM * (-s * x - c * y);
    *pv.add(4) = OM * (c * x - s * y);
    *pv.add(5) = 0.0;
}

pub unsafe fn eraPvu(dt: f64, pv: *mut f64, upv: *mut f64) {
    eraPpsp(pv, dt, pv.add(3), upv);
    eraCp(pv.add(3), upv.add(3));
}

pub unsafe fn eraPvup(dt: f64, pv: *mut f64, p: *mut f64) {
    for i in 0..3 {
        *p.add(i) = *pv.add(i) + dt * *pv.add(i + 3);
    }
}

pub unsafe fn eraPvxpv(a: *mut f64, b: *mut f64, axb: *mut f64) {
    let mut wa = [0.0f64; 6];
    let mut wb = [0.0f64; 6];
    eraCpv(a, wa.as_mut_ptr());
    eraCpv(b, wb.as_mut_ptr());

    eraPxp(wa.as_mut_ptr(), wb.as_mut_ptr(), axb);
    let mut axbd = [0.0f64; 3];
    let mut adxb = [0.0f64; 3];
    eraPxp(wa.as_mut_ptr(), wb.as_mut_ptr().add(3), axbd.as_mut_ptr());
    eraPxp(wa.as_mut_ptr().add(3), wb.as_mut_ptr(), adxb.as_mut_ptr());
    eraPpp(axbd.as_mut_ptr(), adxb.as_mut_ptr(), axb.add(3));
}

pub unsafe fn eraPxp(a: *mut f64, b: *mut f64, axb: *mut f64) {
    let xa = *a.add(0);
    let ya = *a.add(1);
    let za = *a.add(2);
    let xb = *b.add(0);
    let yb = *b.add(1);
    let zb = *b.add(2);

    *axb.add(0) = ya * zb - za * yb;
    *axb.add(1) = za * xb - xa * zb;
    *axb.add(2) = xa * yb - ya * xb;
}
