// G35
//   xys00a.c → eraXys00a
//   xys00b.c → eraXys00b
//   xys06a.c → eraXys06a
//   zp.c     → eraZp
//   zpv.c    → eraZpv
//   zr.c     → eraZr

use crate::H1::*;

// eraXys00a    CIP X,Y and CIO locator s using IAU 2000A model
pub unsafe fn eraXys00a(date1: f64, date2: f64, x: *mut f64, y: *mut f64, s: *mut f64) {
    let mut rbpn = [0.0_f64; 9];

    eraPnm00a(date1, date2, rbpn.as_mut_ptr());

    eraBpn2xy(rbpn.as_mut_ptr(), x, y);

    *s = eraS00(date1, date2, *x, *y);
}

// eraXys00b    CIP X,Y and CIO locator s using IAU 2000B model
pub unsafe fn eraXys00b(date1: f64, date2: f64, x: *mut f64, y: *mut f64, s: *mut f64) {
    let mut rbpn = [0.0_f64; 9];

    eraPnm00b(date1, date2, rbpn.as_mut_ptr());

    eraBpn2xy(rbpn.as_mut_ptr(), x, y);

    *s = eraS00(date1, date2, *x, *y);
}

// eraXys06a    CIP X,Y and CIO locator s using IAU 2006/2000A model
pub unsafe fn eraXys06a(date1: f64, date2: f64, x: *mut f64, y: *mut f64, s: *mut f64) {
    let mut rbpn = [0.0_f64; 9];

    eraPnm06a(date1, date2, rbpn.as_mut_ptr());

    eraBpn2xy(rbpn.as_mut_ptr(), x, y);

    *s = eraS06(date1, date2, *x, *y);
}

// eraZp    Zero a 3-vector
pub unsafe fn eraZp(p: *mut f64) {
    *p.add(0) = 0.0;
    *p.add(1) = 0.0;
    *p.add(2) = 0.0;
}

// eraZpv    Zero a 2×3 pv-vector
pub unsafe fn eraZpv(pv: *mut f64) {
    eraZp(pv.add(0));
    eraZp(pv.add(3));
}

// eraZr    Initialize a 3×3 matrix to zero
pub unsafe fn eraZr(r: *mut f64) {
    for i in 0..9 {
        *r.add(i) = 0.0;
    }
}
