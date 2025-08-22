// G7
//   c2i00a.c  → eraC2i00a
//   c2i00b.c  → eraC2i00b
//   c2i06a.c  → eraC2i06a
//   c2ibpn.c  → eraC2ibpn
//   c2ixy.c   → eraC2ixy
//   c2ixys.c  → eraC2ixys
//   c2s.c     → eraC2s
//   c2t00a.c  → eraC2t00a
//   c2t00b.c  → eraC2t00b
//   c2t06a.c  → eraC2t06a

use crate::H1::*;

// G7/c2i00a.c  -->  eraC2i00a
pub unsafe fn eraC2i00a(date1: f64, date2: f64, rc2i: *mut f64) {
    let mut rbpn = [[0.0_f64; 3]; 3];

    eraPnm00a(date1, date2, rbpn.as_mut_ptr() as *mut f64);
    eraC2ibpn(date1, date2, rbpn.as_mut_ptr() as *mut f64, rc2i);
}

// G7/c2i00b.c  -->  eraC2i00b
pub unsafe fn eraC2i00b(date1: f64, date2: f64, rc2i: *mut f64) {
    let mut rbpn = [[0.0_f64; 3]; 3];

    eraPnm00b(date1, date2, rbpn.as_mut_ptr() as *mut f64);
    eraC2ibpn(date1, date2, rbpn.as_mut_ptr() as *mut f64, rc2i);
}

// G7/c2i06a.c  -->  eraC2i06a
pub unsafe fn eraC2i06a(date1: f64, date2: f64, rc2i: *mut f64) {
    let mut rbpn = [[0.0_f64; 3]; 3];
    let mut x = 0.0_f64;
    let mut y = 0.0_f64;

    eraPnm06a(date1, date2, rbpn.as_mut_ptr() as *mut f64);
    eraBpn2xy(rbpn.as_mut_ptr() as *mut f64, &mut x, &mut y);
    let s = eraS06(date1, date2, x, y);
    eraC2ixys(x, y, s, rc2i);
}

// G7/c2ibpn.c  -->  eraC2ibpn
pub unsafe fn eraC2ibpn(date1: f64, date2: f64, rbpn: *mut f64, rc2i: *mut f64) {
    let mut x = 0.0_f64;
    let mut y = 0.0_f64;

    eraBpn2xy(rbpn, &mut x, &mut y);
    eraC2ixy(date1, date2, x, y, rc2i);
}

// G7/c2ixy.c  -->  eraC2ixy
pub unsafe fn eraC2ixy(date1: f64, date2: f64, x: f64, y: f64, rc2i: *mut f64) {
    let s = eraS00(date1, date2, x, y);
    eraC2ixys(x, y, s, rc2i);
}

// G7/c2ixys.c  -->  eraC2ixys
pub unsafe fn eraC2ixys(x: f64, y: f64, s: f64, rc2i: *mut f64) {
    let r2 = x * x + y * y;
    let e = if r2 > 0.0 { y.atan2(x) } else { 0.0 };
    let d = (r2 / (1.0 - r2)).sqrt().atan();

    eraIr(rc2i);
    eraRz(e, rc2i);
    eraRy(d, rc2i);
    eraRz(-(e + s), rc2i);
}

// G7/c2s.c  -->  eraC2s
pub unsafe fn eraC2s(p: *mut f64, theta: *mut f64, phi: *mut f64) {
    let v = core::slice::from_raw_parts(p, 3);
    let (x, y, z) = (v[0], v[1], v[2]);
    let d2 = x * x + y * y;

    *theta = if d2 == 0.0 { 0.0 } else { y.atan2(x) };
    *phi = if z == 0.0 { 0.0 } else { z.atan2(d2.sqrt()) };
}

// G7/c2t00a.c  -->  eraC2t00a
pub unsafe fn eraC2t00a(tta: f64, ttb: f64, uta: f64, utb: f64, xp: f64, yp: f64, rc2t: *mut f64) {
    let mut rc2i = [[0.0_f64; 3]; 3];
    let mut rpom = [[0.0_f64; 3]; 3];

    eraC2i00a(tta, ttb, rc2i.as_mut_ptr() as *mut f64);
    let era = eraEra00(uta, utb);
    let sp = eraSp00(tta, ttb);
    eraPom00(xp, yp, sp, rpom.as_mut_ptr() as *mut f64);
    eraC2tcio(
        rc2i.as_mut_ptr() as *mut f64,
        era,
        rpom.as_mut_ptr() as *mut f64,
        rc2t,
    );
}

// G7/c2t00b.c  -->  eraC2t00b
pub unsafe fn eraC2t00b(tta: f64, ttb: f64, uta: f64, utb: f64, xp: f64, yp: f64, rc2t: *mut f64) {
    let mut rc2i = [[0.0_f64; 3]; 3];
    let mut rpom = [[0.0_f64; 3]; 3];

    eraC2i00b(tta, ttb, rc2i.as_mut_ptr() as *mut f64);
    let era = eraEra00(uta, utb);
    eraPom00(xp, yp, 0.0, rpom.as_mut_ptr() as *mut f64);
    eraC2tcio(
        rc2i.as_mut_ptr() as *mut f64,
        era,
        rpom.as_mut_ptr() as *mut f64,
        rc2t,
    );
}

// G7/c2t06a.c  -->  eraC2t06a
pub unsafe fn eraC2t06a(tta: f64, ttb: f64, uta: f64, utb: f64, xp: f64, yp: f64, rc2t: *mut f64) {
    let mut rc2i = [[0.0_f64; 3]; 3];
    let mut rpom = [[0.0_f64; 3]; 3];

    eraC2i06a(tta, ttb, rc2i.as_mut_ptr() as *mut f64);
    let era = eraEra00(uta, utb);
    let sp = eraSp00(tta, ttb);
    eraPom00(xp, yp, sp, rpom.as_mut_ptr() as *mut f64);
    eraC2tcio(
        rc2i.as_mut_ptr() as *mut f64,
        era,
        rpom.as_mut_ptr() as *mut f64,
        rc2t,
    );
}
