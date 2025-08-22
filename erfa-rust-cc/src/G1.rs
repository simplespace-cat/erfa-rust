// G1
//   a2af.c       → eraA2af
//   a2tf.c       → eraA2tf
//   ab.c         → eraAb
//   ae2hd.c      → eraAe2hd
//   af2a.c       → eraAf2a
//   anp.c        → eraAnp
//   anpm.c       → eraAnpm
//   apcg.c       → eraApcg
//   apcg13.c     → eraApcg13
//   apci.c       → eraApci
//   apci13.c     → eraApci13

use crate::H1::*;
use core::ffi::c_char;

// G1/a2af.c

pub unsafe fn eraA2af(ndp: i32, angle: f64, sign: *mut c_char, idmsf: *mut i32) {
    const F: f64 = 15.0 / ERFA_D2PI;
    eraD2tf(ndp, angle * F, sign, idmsf);
}

// G1/a2tf.c

pub unsafe fn eraA2tf(ndp: i32, angle: f64, sign: *mut c_char, ihmsf: *mut i32) {
    eraD2tf(ndp, angle / ERFA_D2PI, sign, ihmsf);
}

// G1/ab.c

pub unsafe fn eraAb(pnat: *const f64, v: *const f64, s: f64, bm1: f64, ppr: *mut f64) {
    let pdv = eraPdp(pnat as *mut f64, v as *mut f64);
    let w1 = 1.0 + pdv / (1.0 + bm1);
    let w2 = ERFA_SRS / s;

    let mut r2 = 0.0;
    let mut p = [0.0f64; 3];

    for i in 0..3 {
        let pnat_i = *pnat.add(i);
        let v_i = *v.add(i);
        let w = pnat_i * bm1 + w1 * v_i + w2 * (v_i - pdv * pnat_i);
        p[i] = w;
        r2 += w * w;
    }
    let r = r2.sqrt();
    for i in 0..3 {
        *ppr.add(i) = p[i] / r;
    }
}

// G1/ae2hd.c

pub unsafe fn eraAe2hd(az: f64, el: f64, phi: f64, ha: *mut f64, dec: *mut f64) {
    let (sa, ca) = az.sin_cos();
    let (se, ce) = el.sin_cos();
    let (sp, cp) = phi.sin_cos();

    let x = -ca * ce * sp + se * cp;
    let y = -sa * ce;
    let z = ca * ce * cp + se * sp;

    let r = (x * x + y * y).sqrt();
    *ha = if r != 0.0 { y.atan2(x) } else { 0.0 };
    *dec = z.atan2(r);
}

// G1/af2a.c

pub unsafe fn eraAf2a(s: c_char, ideg: i32, iamin: i32, asec: f64, rad: *mut f64) -> i32 {
    *rad = if s == b'-' as c_char { -1.0 } else { 1.0 }
        * (60.0 * (60.0 * (ideg.abs() as f64) + (iamin.abs() as f64)) + asec.abs())
        * ERFA_DAS2R;

    if ideg < 0 || ideg > 359 {
        return 1;
    }
    if iamin < 0 || iamin > 59 {
        return 2;
    }
    if asec < 0.0 || asec >= 60.0 {
        return 3;
    }
    0
}

// G1/anp.c

pub unsafe fn eraAnp(a: f64) -> f64 {
    let mut w = a % ERFA_D2PI;
    if w < 0.0 {
        w += ERFA_D2PI;
    }
    w
}

// G1/anpm.c

pub unsafe fn eraAnpm(a: f64) -> f64 {
    let mut w = a % ERFA_D2PI;
    if w.abs() >= ERFA_DPI {
        w -= ERFA_DSIGN(ERFA_D2PI, a);
    }
    w
}

// G1/apcg.c

pub unsafe fn eraApcg(
    date1: f64,
    date2: f64,
    ebpv: *mut f64,
    ehp: *mut f64,
    astrom: *mut eraASTROM,
) {
    let mut pv = [[0.0_f64; 3]; 2];
    eraApcs(date1, date2, pv.as_mut_ptr() as *mut f64, ebpv, ehp, astrom);
}

// G1/apcg13.c

pub unsafe fn eraApcg13(date1: f64, date2: f64, astrom: *mut eraASTROM) {
    let mut ehpv = [[0.0_f64; 3]; 2];
    let mut ebpv = [[0.0_f64; 3]; 2];

    eraEpv00(
        date1,
        date2,
        ehpv.as_mut_ptr() as *mut f64,
        ebpv.as_mut_ptr() as *mut f64,
    );

    eraApcg(
        date1,
        date2,
        ebpv.as_mut_ptr() as *mut f64,
        ehpv[0].as_mut_ptr(),
        astrom,
    );
}

// G1/apci.c

pub unsafe fn eraApci(
    date1: f64,
    date2: f64,
    ebpv: *mut f64,
    ehp: *mut f64,
    x: f64,
    y: f64,
    s: f64,
    astrom: *mut eraASTROM,
) {
    eraApcg(date1, date2, ebpv, ehp, astrom);

    eraC2ixys(x, y, s, (*astrom).bpn.as_mut_ptr() as *mut f64);
}

// G1/apci13.c

pub unsafe fn eraApci13(date1: f64, date2: f64, astrom: *mut eraASTROM, eo: *mut f64) {
    let mut ehpv = [[0.0_f64; 3]; 2];
    let mut ebpv = [[0.0_f64; 3]; 2];
    let mut r = [[0.0_f64; 3]; 3];
    let mut x = 0.0f64;
    let mut y = 0.0f64;

    eraEpv00(
        date1,
        date2,
        ehpv.as_mut_ptr() as *mut f64,
        ebpv.as_mut_ptr() as *mut f64,
    );

    eraPnm06a(date1, date2, r.as_mut_ptr() as *mut f64);

    eraBpn2xy(r.as_mut_ptr() as *mut f64, &mut x, &mut y);

    let s = eraS06(date1, date2, x, y);

    eraApci(
        date1,
        date2,
        ebpv.as_mut_ptr() as *mut f64,
        ehpv[0].as_mut_ptr(),
        x,
        y,
        s,
        astrom,
    );

    *eo = eraEors(r.as_mut_ptr() as *mut f64, s);
}
