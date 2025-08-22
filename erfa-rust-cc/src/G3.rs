// G3
//   aper13.c  → eraAper13
//   apio.c    → eraApio
//   apio13.c  → eraApio13
//   atcc13.c  → eraAtcc13
//   atccq.c   → eraAtccq

use crate::H1::*;

// G3/aper13.c

pub unsafe fn eraAper13(ut11: f64, ut12: f64, astrom: *mut eraASTROM) {
    let era = eraEra00(ut11, ut12);
    eraAper(era, astrom);
}

// G3/apio.c

pub unsafe fn eraApio(
    sp: f64,
    theta: f64,
    elong: f64,
    phi: f64,
    hm: f64,
    xp: f64,
    yp: f64,
    refa: f64,
    refb: f64,
    astrom: *mut eraASTROM,
) {
    let mut r: [[f64; 3]; 3] = [[0.0; 3]; 3];
    let mut a: f64;
    let mut b: f64;
    let eral: f64;
    let c: f64;
    let mut pv: [[f64; 3]; 2] = [[0.0; 3]; 2];

    eraIr(r.as_mut_ptr() as *mut f64);
    eraRz(theta + sp, r.as_mut_ptr() as *mut f64);
    eraRy(-xp, r.as_mut_ptr() as *mut f64);
    eraRx(-yp, r.as_mut_ptr() as *mut f64);
    eraRz(elong, r.as_mut_ptr() as *mut f64);

    a = r[0][0];
    b = r[0][1];
    eral = if a != 0.0 || b != 0.0 {
        b.atan2(a)
    } else {
        0.0
    };
    (*astrom).eral = eral;

    a = r[0][0];
    c = r[0][2];
    (*astrom).xpl = c.atan2((a * a + b * b).sqrt());
    a = r[1][2];
    b = r[2][2];
    (*astrom).ypl = if a != 0.0 || b != 0.0 {
        -a.atan2(b)
    } else {
        0.0
    };

    (*astrom).along = eraAnpm(eral - theta);

    (*astrom).sphi = phi.sin();
    (*astrom).cphi = phi.cos();

    eraPvtob(
        elong,
        phi,
        hm,
        xp,
        yp,
        sp,
        theta,
        pv.as_mut_ptr() as *mut f64,
    );

    (*astrom).diurab = (pv[1][0] * pv[1][0] + pv[1][1] * pv[1][1]).sqrt() / ERFA_CMPS;

    (*astrom).refa = refa;
    (*astrom).refb = refb;
}

// G3/apio13.c

pub unsafe fn eraApio13(
    utc1: f64,
    utc2: f64,
    dut1: f64,
    elong: f64,
    phi: f64,
    hm: f64,
    xp: f64,
    yp: f64,
    phpa: f64,
    tc: f64,
    rh: f64,
    wl: f64,
    astrom: *mut eraASTROM,
) -> i32 {
    let mut tai1 = 0.0;
    let mut tai2 = 0.0;
    let mut tt1 = 0.0;
    let mut tt2 = 0.0;
    let mut ut11 = 0.0;
    let mut ut12 = 0.0;

    if eraUtctai(utc1, utc2, &mut tai1, &mut tai2) < 0 {
        return -1;
    }

    eraTaitt(tai1, tai2, &mut tt1, &mut tt2);

    let j = eraUtcut1(utc1, utc2, dut1, &mut ut11, &mut ut12);
    if j < 0 {
        return -1;
    }

    let sp = eraSp00(tt1, tt2);

    let theta = eraEra00(ut11, ut12);

    let mut refa = 0.0;
    let mut refb = 0.0;
    eraRefco(phpa, tc, rh, wl, &mut refa, &mut refb);

    eraApio(sp, theta, elong, phi, hm, xp, yp, refa, refb, astrom);

    j
}

// G3/atcc13.c

pub unsafe fn eraAtcc13(
    rc: f64,
    dc: f64,
    pr: f64,
    pd: f64,
    px: f64,
    rv: f64,
    date1: f64,
    date2: f64,
    ra: *mut f64,
    da: *mut f64,
) {
    let mut astrom = eraASTROM::default();
    let mut w = 0.0;

    eraApci13(date1, date2, &mut astrom as *mut _, &mut w);

    eraAtccq(rc, dc, pr, pd, px, rv, &mut astrom as *mut _, ra, da);
}

// G3/atccq.c

pub unsafe fn eraAtccq(
    rc: f64,
    dc: f64,
    pr: f64,
    pd: f64,
    px: f64,
    rv: f64,
    astrom: *mut eraASTROM,
    ra: *mut f64,
    da: *mut f64,
) {
    let mut p = [0.0_f64; 3];
    let mut w = 0.0_f64;

    eraPmpx(
        rc,
        dc,
        pr,
        pd,
        px,
        rv,
        (*astrom).pmt,
        (*astrom).eb.as_ptr() as *mut f64,
        p.as_mut_ptr(),
    );

    eraC2s(p.as_mut_ptr(), &mut w, da);
    *ra = eraAnp(w);
}
