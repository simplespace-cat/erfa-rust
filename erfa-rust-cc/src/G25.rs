// G25
//   pmat00.c → eraPmat00
//   pmat06.c → eraPmat06
//   pmat76.c → eraPmat76
//   pmp.c    → eraPmp
//   pmpx.c   → eraPmpx
//   pmsafe.c → eraPmsafe
//   pn.c     → eraPn
//   pn00.c   → eraPn00
//   pn00a.c  → eraPn00a
//   pn00b.c  → eraPn00b

use crate::H1::*;

// eraPmat00   Precession matrix, IAU 2000 bias-precession model
pub unsafe fn eraPmat00(date1: f64, date2: f64, rbp: *mut f64) {
    let mut rb = [[0.0_f64; 3]; 3];
    let mut rp = [[0.0_f64; 3]; 3];

    eraBp00(
        date1,
        date2,
        rb.as_mut_ptr().cast::<f64>(),
        rp.as_mut_ptr().cast::<f64>(),
        rbp,
    );
}

// eraPmat06   Precession matrix, IAU 2006 bias-precession model
pub unsafe fn eraPmat06(date1: f64, date2: f64, rbp: *mut f64) {
    let mut gamb = 0.0;
    let mut phib = 0.0;
    let mut psib = 0.0;
    let mut epsa = 0.0;

    eraPfw06(date1, date2, &mut gamb, &mut phib, &mut psib, &mut epsa);

    eraFw2m(gamb, phib, psib, epsa, rbp);
}

// eraPmat76   Precession matrix, IAU 1976 model
pub unsafe fn eraPmat76(date1: f64, date2: f64, rmatp: *mut f64) {
    let mut zeta = 0.0;
    let mut z = 0.0;
    let mut theta = 0.0;
    let mut wmat = [[0.0_f64; 3]; 3];

    eraPrec76(ERFA_DJ00, 0.0, date1, date2, &mut zeta, &mut z, &mut theta);

    eraIr(wmat.as_mut_ptr().cast::<f64>());
    eraRz(-zeta, wmat.as_mut_ptr().cast::<f64>());
    eraRy(theta, wmat.as_mut_ptr().cast::<f64>());
    eraRz(-z, wmat.as_mut_ptr().cast::<f64>());
    eraCr(wmat.as_mut_ptr().cast::<f64>(), rmatp);
}

// eraPmp   P-vector subtraction
pub unsafe fn eraPmp(a: *mut f64, b: *mut f64, amb: *mut f64) {
    for i in 0..3 {
        *amb.add(i) = *a.add(i) - *b.add(i);
    }
}

// eraPmpx   Proper motion & parallax
pub unsafe fn eraPmpx(
    rc: f64,
    dc: f64,
    pr: f64,
    pd: f64,
    px: f64,
    rv: f64,
    pmt: f64,
    pob: *mut f64,
    pco: *mut f64,
) {
    const VF: f64 = ERFA_DAYSEC * ERFA_DJM / ERFA_DAU;
    const AULTY: f64 = ERFA_AULT / ERFA_DAYSEC / ERFA_DJY;

    let (sr, cr) = rc.sin_cos();
    let (sd, cd) = dc.sin_cos();
    let x = cr * cd;
    let y = sr * cd;
    let z = sd;
    let mut p = [x, y, z];

    let dt = pmt + eraPdp(p.as_mut_ptr(), pob) * AULTY;

    let pxr = px * ERFA_DAS2R;
    let w = VF * rv * pxr;
    let pdz = pd * z;
    let pm = [
        -pr * y - pdz * cr + w * x,
        pr * x - pdz * sr + w * y,
        pd * cd + w * z,
    ];

    for i in 0..3 {
        p[i] += dt * pm[i] - pxr * *pob.add(i);
    }
    eraPn(p.as_mut_ptr(), &mut 0.0, pco);
}

// eraPmsafe   Proper-motion update with safe‐distance handling
pub unsafe fn eraPmsafe(
    ra1: f64,
    dec1: f64,
    pmr1: f64,
    pmd1: f64,
    px1: f64,
    rv1: f64,
    ep1a: f64,
    ep1b: f64,
    ep2a: f64,
    ep2b: f64,
    ra2: *mut f64,
    dec2: *mut f64,
    pmr2: *mut f64,
    pmd2: *mut f64,
    px2: *mut f64,
    rv2: *mut f64,
) -> i32 {
    const PXMIN: f64 = 5.0e-7;
    const F: f64 = 326.0;

    let pm = eraSeps(ra1, dec1, ra1 + pmr1, dec1 + pmd1);

    let mut jpx = 0;
    let mut px1a = px1;
    let pm_scaled = pm * F;
    if px1a < pm_scaled {
        px1a = pm_scaled;
        jpx = 1;
    }
    if px1a < PXMIN {
        px1a = PXMIN;
        jpx = 1;
    }

    let mut j = eraStarpm(
        ra1, dec1, pmr1, pmd1, px1a, rv1, ep1a, ep1b, ep2a, ep2b, ra2, dec2, pmr2, pmd2, px2, rv2,
    );

    if j & 1 == 0 {
        j += jpx;
    }
    j
}

// eraPn   Decompose p-vector into modulus and unit vector
pub unsafe fn eraPn(p: *mut f64, r: *mut f64, u: *mut f64) {
    let w = eraPm(p);
    if w == 0.0 {
        eraZp(u);
    } else {
        eraSxp(1.0 / w, p, u);
    }
    *r = w;
}

// eraPn00   Precession-nutation (IAU 2000, caller-supplied dψ,dε)
pub unsafe fn eraPn00(
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
    let mut dpsipr = 0.0;
    let mut depspr = 0.0;
    let mut rbpw = [[0.0_f64; 3]; 3];
    let mut rnw = [[0.0_f64; 3]; 3];

    eraPr00(date1, date2, &mut dpsipr, &mut depspr);

    *epsa = eraObl80(date1, date2) + depspr;

    eraBp00(date1, date2, rb, rp, rbpw.as_mut_ptr().cast::<f64>());
    eraCr(rbpw.as_mut_ptr().cast::<f64>(), rbp);

    eraNumat(*epsa, dpsi, deps, rnw.as_mut_ptr().cast::<f64>());
    eraCr(rnw.as_mut_ptr().cast::<f64>(), rn);

    eraRxr(
        rnw.as_mut_ptr().cast::<f64>(),
        rbpw.as_mut_ptr().cast::<f64>(),
        rbpn,
    );
}

// eraPn00a   Precession-nutation, IAU 2000A model
pub unsafe fn eraPn00a(
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
    eraNut00a(date1, date2, dpsi, deps);

    eraPn00(date1, date2, *dpsi, *deps, epsa, rb, rp, rbp, rn, rbpn);
}

// eraPn00b   Precession-nutation, IAU 2000B model
pub unsafe fn eraPn00b(
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
    eraNut00b(date1, date2, dpsi, deps);

    eraPn00(date1, date2, *dpsi, *deps, epsa, rb, rp, rbp, rn, rbpn);
}
