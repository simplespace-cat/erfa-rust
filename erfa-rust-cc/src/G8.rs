// G8
//   c2tcio.c  → eraC2tcio
//   c2teqx.c  → eraC2teqx
//   c2tpe.c   → eraC2tpe
//   c2txy.c   → eraC2txy
//   cal2jd.c  → eraCal2jd
//   cp.c      → eraCp
//   cpv.c     → eraCpv
//   cr.c      → eraCr

use crate::H1::*;

// c2tcio.c   →  eraC2tcio
pub unsafe fn eraC2tcio(rc2i: *const f64, era: f64, rpom: *const f64, rc2t: *mut f64) {
    eraCr(rc2i as *mut f64, rc2t);
    eraRz(era, rc2t);
    eraRxr(rpom as *mut f64, rc2t, rc2t);
}

// c2teqx.c   →  eraC2teqx
pub unsafe fn eraC2teqx(rbpn: *const f64, gst: f64, rpom: *const f64, rc2t: *mut f64) {
    eraCr(rbpn as *mut f64, rc2t);
    eraRz(gst, rc2t);
    eraRxr(rpom as *mut f64, rc2t, rc2t);
}

// c2tpe.c   →  eraC2tpe   (IAU 2000)
pub unsafe fn eraC2tpe(
    tta: f64,
    ttb: f64,
    uta: f64,
    utb: f64,
    dpsi: f64,
    deps: f64,
    xp: f64,
    yp: f64,
    rc2t: *mut f64,
) {
    let mut epsa = 0.0;
    let mut rb = [0.0f64; 9];
    let mut rp = [0.0f64; 9];
    let mut rbp = [0.0f64; 9];
    let mut rn = [0.0f64; 9];
    let mut rbpn = [0.0f64; 9];

    eraPn00(
        tta,
        ttb,
        dpsi,
        deps,
        &mut epsa,
        rb.as_mut_ptr(),
        rp.as_mut_ptr(),
        rbp.as_mut_ptr(),
        rn.as_mut_ptr(),
        rbpn.as_mut_ptr(),
    );

    let gmst = eraGmst00(uta, utb, tta, ttb);

    let ee = eraEe00(tta, ttb, epsa, dpsi);

    let sp = eraSp00(tta, ttb);

    let mut rpom = [0.0f64; 9];
    eraPom00(xp, yp, sp, rpom.as_mut_ptr());

    eraC2teqx(rbpn.as_ptr(), gmst + ee, rpom.as_ptr(), rc2t);
}

// c2txy.c   →  eraC2txy   (IAU 2000)
pub unsafe fn eraC2txy(
    tta: f64,
    ttb: f64,
    uta: f64,
    utb: f64,
    x: f64,
    y: f64,
    xp: f64,
    yp: f64,
    rc2t: *mut f64,
) {
    let mut rc2i = [0.0f64; 9];
    eraC2ixy(tta, ttb, x, y, rc2i.as_mut_ptr());

    let era = eraEra00(uta, utb);
    let sp = eraSp00(tta, ttb);

    let mut rpom = [0.0f64; 9];
    eraPom00(xp, yp, sp, rpom.as_mut_ptr());

    eraC2tcio(rc2i.as_ptr(), era, rpom.as_ptr(), rc2t);
}

// cal2jd.c   →  eraCal2jd
pub unsafe fn eraCal2jd(iy: i32, im: i32, id: i32, djm0: *mut f64, djm: *mut f64) -> i32 {
    const IYMIN: i32 = -4799;
    const MTAB: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    if iy < IYMIN {
        return -1;
    }
    if im < 1 || im > 12 {
        return -2;
    }

    let ly = ((im == 2) && (iy % 4 == 0) && (iy % 100 != 0 || iy % 400 == 0)) as i32;

    if id < 1 || id > MTAB[(im - 1) as usize] + ly {
        compute_jd(iy, im, id, djm0, djm);
        return -3;
    }

    compute_jd(iy, im, id, djm0, djm);
    0
}

#[inline]
unsafe fn compute_jd(iy: i32, im: i32, id: i32, djm0: *mut f64, djm: *mut f64) {
    let my = (im - 14) / 12;
    let iyp = (iy + my) as i64;

    *djm0 = ERFA_DJM0;
    *djm = (1461 * (iyp + 4800) / 4 + 367 * (im as i64 - 2 - 12 * my as i64) / 12
        - 3 * ((iyp + 4900) / 100) / 4
        + id as i64
        - 2432076) as f64;
}

// cp.c   →  eraCp
pub unsafe fn eraCp(p: *const f64, c: *mut f64) {
    for i in 0..3 {
        *c.add(i) = *p.add(i);
    }
}

// cpv.c   →  eraCpv

pub unsafe fn eraCpv(pv: *const f64, c: *mut f64) {
    eraCp(pv, c);
    eraCp(pv.add(3), c.add(3));
}

// cr.c   →  eraCr
pub unsafe fn eraCr(r: *const f64, c: *mut f64) {
    eraCp(r, c);
    eraCp(r.add(3), c.add(3));
    eraCp(r.add(6), c.add(6));
}
