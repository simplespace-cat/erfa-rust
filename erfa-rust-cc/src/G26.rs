// G26
//   pn06.c   → eraPn06
//   pn06a.c  → eraPn06a
//   pnm00a.c → eraPnm00a
//   pnm00b.c → eraPnm00b
//   pnm06a.c → eraPnm06a
//   pnm80.c  → eraPnm80
//   pom00.c  → eraPom00
//   ppp.c    → eraPpp
//   ppsp.c   → eraPpsp
//   pr00.c   → eraPr00

use crate::H1::*;

// eraPn06 (pn06.c)
pub unsafe fn eraPn06(
    date1: f64,
    date2: f64,
    dpsi: f64,
    deps: f64,
    epsa: *mut f64,
    rb: *mut f64,
    rp: *mut f64,
    rbp: *mut f64,
    rn: *mut f64,
    rbpn: *mut f64,
) {
    let mut gamb = 0.0;
    let mut phib = 0.0;
    let mut psib = 0.0;
    let mut eps = 0.0;

    let mut r1 = [0.0_f64; 9];
    let mut r2 = [0.0_f64; 9];
    let mut rt = [0.0_f64; 9];

    eraPfw06(
        ERFA_DJM0, ERFA_DJM00, &mut gamb, &mut phib, &mut psib, &mut eps,
    );
    eraFw2m(gamb, phib, psib, eps, r1.as_mut_ptr());
    eraCr(r1.as_mut_ptr(), rb);

    eraPfw06(date1, date2, &mut gamb, &mut phib, &mut psib, &mut eps);
    eraFw2m(gamb, phib, psib, eps, r2.as_mut_ptr());
    eraCr(r2.as_mut_ptr(), rbp);

    eraTr(r1.as_mut_ptr(), rt.as_mut_ptr());
    eraRxr(r2.as_mut_ptr(), rt.as_mut_ptr(), rp);

    eraFw2m(gamb, phib, psib + dpsi, eps + deps, r1.as_mut_ptr());
    eraCr(r1.as_mut_ptr(), rbpn);

    eraTr(r2.as_mut_ptr(), rt.as_mut_ptr());
    eraRxr(r1.as_mut_ptr(), rt.as_mut_ptr(), rn);

    *epsa = eps;
}

// eraPn06a (pn06a.c)
pub unsafe fn eraPn06a(
    date1: f64,
    date2: f64,
    dpsi: *mut f64,
    deps: *mut f64,
    epsa: *mut f64,
    rb: *mut f64,
    rp: *mut f64,
    rbp: *mut f64,
    rn: *mut f64,
    rbpn: *mut f64,
) {
    eraNut06a(date1, date2, dpsi, deps);

    eraPn06(date1, date2, *dpsi, *deps, epsa, rb, rp, rbp, rn, rbpn);
}

// eraPnm00a (pnm00a.c)
pub unsafe fn eraPnm00a(date1: f64, date2: f64, rbpn: *mut f64) {
    let mut dpsi = 0.0;
    let mut deps = 0.0;
    let mut epsa = 0.0;
    let mut rb = [0.0_f64; 9];
    let mut rp = [0.0_f64; 9];
    let mut rbp = [0.0_f64; 9];
    let mut rn = [0.0_f64; 9];

    eraPn00a(
        date1,
        date2,
        &mut dpsi,
        &mut deps,
        &mut epsa,
        rb.as_mut_ptr(),
        rp.as_mut_ptr(),
        rbp.as_mut_ptr(),
        rn.as_mut_ptr(),
        rbpn,
    );
}

// eraPnm00b (pnm00b.c)
pub unsafe fn eraPnm00b(date1: f64, date2: f64, rbpn: *mut f64) {
    let mut dpsi = 0.0;
    let mut deps = 0.0;
    let mut epsa = 0.0;
    let mut rb = [0.0_f64; 9];
    let mut rp = [0.0_f64; 9];
    let mut rbp = [0.0_f64; 9];
    let mut rn = [0.0_f64; 9];

    eraPn00b(
        date1,
        date2,
        &mut dpsi,
        &mut deps,
        &mut epsa,
        rb.as_mut_ptr(),
        rp.as_mut_ptr(),
        rbp.as_mut_ptr(),
        rn.as_mut_ptr(),
        rbpn,
    );
}

// eraPnm06a (pnm06a.c)
pub unsafe fn eraPnm06a(date1: f64, date2: f64, rbpn: *mut f64) {
    let mut gamb = 0.0;
    let mut phib = 0.0;
    let mut psib = 0.0;
    let mut epsa = 0.0;
    let mut dp = 0.0;
    let mut de = 0.0;

    eraPfw06(date1, date2, &mut gamb, &mut phib, &mut psib, &mut epsa);
    eraNut06a(date1, date2, &mut dp, &mut de);

    eraFw2m(gamb, phib, psib + dp, epsa + de, rbpn);
}

// eraPnm80 (pnm80.c)
pub unsafe fn eraPnm80(date1: f64, date2: f64, rmatpn: *mut f64) {
    let mut rmatp = [0.0_f64; 9];
    let mut rmatn = [0.0_f64; 9];

    eraPmat76(date1, date2, rmatp.as_mut_ptr());
    eraNutm80(date1, date2, rmatn.as_mut_ptr());
    eraRxr(rmatn.as_mut_ptr(), rmatp.as_mut_ptr(), rmatpn);
}

// eraPom00 (pom00.c)
pub unsafe fn eraPom00(xp: f64, yp: f64, sp: f64, rpom: *mut f64) {
    eraIr(rpom);
    eraRz(sp, rpom);
    eraRy(-xp, rpom);
    eraRx(-yp, rpom);
}

// eraPpp (ppp.c)
pub unsafe fn eraPpp(a: *mut f64, b: *mut f64, apb: *mut f64) {
    for i in 0..3 {
        *apb.add(i) = *a.add(i) + *b.add(i);
    }
}

// eraPpsp (ppsp.c)
pub unsafe fn eraPpsp(a: *mut f64, s: f64, b: *mut f64, apsb: *mut f64) {
    let mut sb = [0.0_f64; 3];
    eraSxp(s, b, sb.as_mut_ptr());
    eraPpp(a, sb.as_mut_ptr(), apsb);
}

// eraPr00 (pr00.c)
pub unsafe fn eraPr00(date1: f64, date2: f64, dpsipr: *mut f64, depspr: *mut f64) {
    const PRECOR: f64 = -0.29965 * ERFA_DAS2R;
    const OBLCOR: f64 = -0.02524 * ERFA_DAS2R;

    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    *dpsipr = PRECOR * t;
    *depspr = OBLCOR * t;
}
