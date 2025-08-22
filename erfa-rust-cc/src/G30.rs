// G30
//   s06.c    → eraS06
//   s06a.c   → eraS06a
//   sepp.c   → eraSepp
//   seps.c   → eraSeps
//   sp00.c   → eraSp00
//   starpm.c → eraStarpm
//   starpv.c → eraStarpv
//   sxp.c    → eraSxp
//   sxpv.c   → eraSxpv

use crate::H1::*;

// G30/s06.c  →  eraS06
#[derive(Clone, Copy)]
struct TERM {
    nfa: [i32; 8],
    s: f64,
    c: f64,
}

const SP: [f64; 6] = [
    94.00e-6,
    3808.65e-6,
    -122.68e-6,
    -72574.11e-6,
    27.98e-6,
    15.62e-6,
];

// series terms (truncated)
const S0: [TERM; 33] = [
    TERM {
        nfa: [0, 0, 0, 0, 1, 0, 0, 0],
        s: -2640.73e-6,
        c: 0.39e-6,
    },
    TERM {
        nfa: [0, 0, 0, 0, 2, 0, 0, 0],
        s: -63.53e-6,
        c: 0.02e-6,
    },
    TERM {
        nfa: [0, 0, 2, -2, 3, 0, 0, 0],
        s: -11.75e-6,
        c: -0.01e-6,
    },
    TERM {
        nfa: [0, 0, 2, -2, 1, 0, 0, 0],
        s: -11.21e-6,
        c: -0.01e-6,
    },
    TERM {
        nfa: [0, 0, 2, -2, 2, 0, 0, 0],
        s: 4.57e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [0, 0, 2, 0, 3, 0, 0, 0],
        s: -2.02e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [0, 0, 2, 0, 1, 0, 0, 0],
        s: -1.98e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [0, 0, 0, 0, 3, 0, 0, 0],
        s: 1.72e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [0, 1, 0, 0, 1, 0, 0, 0],
        s: 1.41e-6,
        c: 0.01e-6,
    },
    TERM {
        nfa: [0, 1, 0, 0, -1, 0, 0, 0],
        s: 1.26e-6,
        c: 0.01e-6,
    },
    TERM {
        nfa: [1, 0, 0, 0, -1, 0, 0, 0],
        s: 0.63e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [1, 0, 0, 0, 1, 0, 0, 0],
        s: 0.63e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [0, 1, 2, -2, 3, 0, 0, 0],
        s: -0.46e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [0, 1, 2, -2, 1, 0, 0, 0],
        s: -0.45e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [0, 0, 4, -4, 4, 0, 0, 0],
        s: -0.36e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [0, 0, 1, -1, 1, -8, 12, 0],
        s: 0.24e-6,
        c: 0.12e-6,
    },
    TERM {
        nfa: [0, 0, 2, 0, 0, 0, 0, 0],
        s: -0.32e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [0, 0, 2, 0, 2, 0, 0, 0],
        s: -0.28e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [1, 0, 2, 0, 3, 0, 0, 0],
        s: -0.27e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [1, 0, 2, 0, 1, 0, 0, 0],
        s: -0.26e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [0, 0, 2, -2, 0, 0, 0, 0],
        s: 0.21e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [0, 1, -2, 2, -3, 0, 0, 0],
        s: -0.19e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [0, 1, -2, 2, -1, 0, 0, 0],
        s: -0.18e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [0, 0, 0, 0, 0, 8, -13, -1],
        s: 0.10e-6,
        c: -0.05e-6,
    },
    TERM {
        nfa: [0, 0, 0, 2, 0, 0, 0, 0],
        s: -0.15e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [2, 0, -2, 0, -1, 0, 0, 0],
        s: 0.14e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [0, 1, 2, -2, 2, 0, 0, 0],
        s: 0.14e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [1, 0, 0, -2, 1, 0, 0, 0],
        s: -0.14e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [1, 0, 0, -2, -1, 0, 0, 0],
        s: -0.14e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [0, 0, 4, -2, 4, 0, 0, 0],
        s: -0.13e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [0, 0, 2, -2, 4, 0, 0, 0],
        s: 0.11e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [1, 0, -2, 0, -3, 0, 0, 0],
        s: -0.11e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [1, 0, -2, 0, -1, 0, 0, 0],
        s: -0.11e-6,
        c: 0.00e-6,
    },
];

const S1: [TERM; 3] = [
    TERM {
        nfa: [0, 0, 0, 0, 2, 0, 0, 0],
        s: -0.07e-6,
        c: 3.57e-6,
    },
    TERM {
        nfa: [0, 0, 0, 0, 1, 0, 0, 0],
        s: 1.73e-6,
        c: -0.03e-6,
    },
    TERM {
        nfa: [0, 0, 2, -2, 3, 0, 0, 0],
        s: 0.00e-6,
        c: 0.48e-6,
    },
];

const S2: [TERM; 25] = [
    TERM {
        nfa: [0, 0, 0, 0, 1, 0, 0, 0],
        s: 743.52e-6,
        c: -0.17e-6,
    },
    TERM {
        nfa: [0, 0, 2, -2, 2, 0, 0, 0],
        s: 56.91e-6,
        c: 0.06e-6,
    },
    TERM {
        nfa: [0, 0, 2, 0, 2, 0, 0, 0],
        s: 9.84e-6,
        c: -0.01e-6,
    },
    TERM {
        nfa: [0, 0, 0, 0, 2, 0, 0, 0],
        s: -8.85e-6,
        c: 0.01e-6,
    },
    TERM {
        nfa: [0, 1, 0, 0, 0, 0, 0, 0],
        s: -6.38e-6,
        c: -0.05e-6,
    },
    TERM {
        nfa: [1, 0, 0, 0, 0, 0, 0, 0],
        s: -3.07e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [0, 1, 2, -2, 2, 0, 0, 0],
        s: 2.23e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [0, 0, 2, 0, 1, 0, 0, 0],
        s: 1.67e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [1, 0, 2, 0, 2, 0, 0, 0],
        s: 1.30e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [0, 1, -2, 2, -2, 0, 0, 0],
        s: 0.93e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [1, 0, 0, -2, 0, 0, 0, 0],
        s: 0.68e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [0, 0, 2, -2, 1, 0, 0, 0],
        s: -0.55e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [1, 0, -2, 0, -2, 0, 0, 0],
        s: 0.53e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [0, 0, 0, 2, 0, 0, 0, 0],
        s: -0.27e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [1, 0, 0, 0, 1, 0, 0, 0],
        s: -0.27e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [1, 0, -2, -2, -2, 0, 0, 0],
        s: -0.26e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [1, 0, 0, 0, -1, 0, 0, 0],
        s: -0.25e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [1, 0, 2, 0, 1, 0, 0, 0],
        s: 0.22e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [2, 0, 0, -2, 0, 0, 0, 0],
        s: -0.21e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [2, 0, -2, 0, -1, 0, 0, 0],
        s: 0.20e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [0, 0, 2, 2, 2, 0, 0, 0],
        s: 0.17e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [2, 0, 2, 0, 2, 0, 0, 0],
        s: 0.13e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [2, 0, 0, 0, 0, 0, 0, 0],
        s: -0.13e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [1, 0, 2, -2, 2, 0, 0, 0],
        s: -0.12e-6,
        c: 0.00e-6,
    },
    TERM {
        nfa: [0, 0, 2, 0, 0, 0, 0, 0],
        s: -0.11e-6,
        c: 0.00e-6,
    },
];

const S3: [TERM; 4] = [
    TERM {
        nfa: [0, 0, 0, 0, 1, 0, 0, 0],
        s: 0.30e-6,
        c: -23.42e-6,
    },
    TERM {
        nfa: [0, 0, 2, -2, 2, 0, 0, 0],
        s: -0.03e-6,
        c: -1.46e-6,
    },
    TERM {
        nfa: [0, 0, 2, 0, 2, 0, 0, 0],
        s: -0.01e-6,
        c: -0.25e-6,
    },
    TERM {
        nfa: [0, 0, 0, 0, 2, 0, 0, 0],
        s: 0.00e-6,
        c: 0.23e-6,
    },
];

const S4: [TERM; 1] = [TERM {
    nfa: [0, 0, 0, 0, 1, 0, 0, 0],
    s: -0.26e-6,
    c: -0.01e-6,
}];

// eraS06    CIO locator s (IAU 2006/2000A), given CIP X,Y
pub unsafe fn eraS06(date1: f64, date2: f64, x: f64, y: f64) -> f64 {
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    let mut fa = [0.0_f64; 8];
    fa[0] = eraFal03(t);
    fa[1] = eraFalp03(t);
    fa[2] = eraFaf03(t);
    fa[3] = eraFad03(t);
    fa[4] = eraFaom03(t);
    fa[5] = eraFave03(t);
    fa[6] = eraFae03(t);
    fa[7] = eraFapa03(t);

    let mut w0 = SP[0];
    let mut w1 = SP[1];
    let mut w2 = SP[2];
    let mut w3 = SP[3];
    let mut w4 = SP[4];
    let w5 = SP[5];

    for term in S0.iter().rev() {
        let mut a = 0.0_f64;
        for j in 0..8 {
            a += term.nfa[j] as f64 * fa[j];
        }
        w0 += term.s * a.sin() + term.c * a.cos();
    }

    for term in S1.iter().rev() {
        let mut a = 0.0_f64;
        for j in 0..8 {
            a += term.nfa[j] as f64 * fa[j];
        }
        w1 += term.s * a.sin() + term.c * a.cos();
    }

    for term in S2.iter().rev() {
        let mut a = 0.0_f64;
        for j in 0..8 {
            a += term.nfa[j] as f64 * fa[j];
        }
        w2 += term.s * a.sin() + term.c * a.cos();
    }

    for term in S3.iter().rev() {
        let mut a = 0.0_f64;
        for j in 0..8 {
            a += term.nfa[j] as f64 * fa[j];
        }
        w3 += term.s * a.sin() + term.c * a.cos();
    }

    for term in S4.iter().rev() {
        let mut a = 0.0_f64;
        for j in 0..8 {
            a += term.nfa[j] as f64 * fa[j];
        }
        w4 += term.s * a.sin() + term.c * a.cos();
    }

    let s = (w0 + (w1 + (w2 + (w3 + (w4 + w5 * t) * t) * t) * t) * t) * ERFA_DAS2R - x * y / 2.0;

    s
}
// end of eraS06 (G30/s06.rs)

// eraS06a    CIO locator s, IAU 2006/2000A
pub unsafe fn eraS06a(date1: f64, date2: f64) -> f64 {
    let mut rnpb: [f64; 9] = [0.0; 9];

    eraPnm06a(date1, date2, rnpb.as_mut_ptr());

    let mut x = 0.0_f64;
    let mut y = 0.0_f64;
    eraBpn2xy(rnpb.as_mut_ptr(), &mut x, &mut y);

    eraS06(date1, date2, x, y)
}

// eraSepp    angular separation between two p-vectors
pub unsafe fn eraSepp(a: *mut f64, b: *mut f64) -> f64 {
    let mut axb: [f64; 3] = [0.0; 3];

    eraPxp(a, b, axb.as_mut_ptr());
    let ss = eraPm(axb.as_mut_ptr());

    let cs = eraPdp(a, b);

    if ss != 0.0 || cs != 0.0 {
        ss.atan2(cs)
    } else {
        0.0
    }
}

// eraSeps    angular separation between two spherical positions
pub unsafe fn eraSeps(al: f64, ap: f64, bl: f64, bp: f64) -> f64 {
    let mut ac: [f64; 3] = [0.0; 3];
    let mut bc: [f64; 3] = [0.0; 3];

    eraS2c(al, ap, ac.as_mut_ptr());
    eraS2c(bl, bp, bc.as_mut_ptr());

    eraSepp(ac.as_mut_ptr(), bc.as_mut_ptr())
}

// eraSp00    TIO locator s′
pub unsafe fn eraSp00(date1: f64, date2: f64) -> f64 {
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    -47e-6 * t * ERFA_DAS2R
}

// eraStarpm    propagate star catalog data for space motion
pub unsafe fn eraStarpm(
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
    let mut pv1: [f64; 6] = [0.0; 6];
    let mut pv: [f64; 6] = [0.0; 6];
    let mut pv2: [f64; 6] = [0.0; 6];

    let j1 = eraStarpv(
        ra1,
        dec1,
        pmr1,
        pmd1,
        px1,
        rv1,
        pv1.as_mut_ptr() as *mut f64,
    );

    let tl1 = eraPm(pv1.as_mut_ptr()) / ERFA_DC;

    let dt = (ep2a - ep1a) + (ep2b - ep1b);

    eraPvu(dt + tl1, pv1.as_mut_ptr(), pv.as_mut_ptr());

    let r2 = eraPdp(pv.as_mut_ptr(), pv.as_mut_ptr());
    let rdv = eraPdp(pv.as_mut_ptr(), pv[3..].as_mut_ptr());
    let v2 = eraPdp(pv[3..].as_mut_ptr(), pv[3..].as_mut_ptr());
    let c2mv2 = ERFA_DC * ERFA_DC - v2;
    if c2mv2 <= 0.0 {
        return -1;
    }
    let tl2 = (-rdv + (rdv * rdv + c2mv2 * r2).sqrt()) / c2mv2;

    eraPvu(dt + (tl1 - tl2), pv1.as_mut_ptr(), pv2.as_mut_ptr());

    let j2 = eraPvstar(pv2.as_mut_ptr(), ra2, dec2, pmr2, pmd2, px2, rv2);

    if j2 == 0 {
        j1
    } else {
        -1
    }
}

// eraStarpv    catalog data → position/velocity vector
// eraStarpv    catalog data → position/velocity vector
pub unsafe fn eraStarpv(
    ra: f64,
    dec: f64,
    pmr: f64,
    pmd: f64,
    px: f64,
    rv: f64,
    pv: *mut f64,
) -> i32 {
    const PXMIN: f64 = 1e-7;
    const VMAX: f64 = 0.5;
    const IMAX: i32 = 100;

    let mut warn = 0;

    let w = if px >= PXMIN {
        px
    } else {
        warn += 1;
        PXMIN
    };
    let r = ERFA_DR2AS / w;

    let rd = ERFA_DAYSEC * rv * 1e3 / ERFA_DAU;

    let rad = pmr / ERFA_DJY;
    let decd = pmd / ERFA_DJY;

    eraS2pv(ra, dec, r, rad, decd, rd, pv);

    let v = eraPm(pv.add(3));
    if v / ERFA_DC > VMAX {
        eraZp(pv.add(3));
        warn += 2;
    }

    let mut pu = [0.0_f64; 3];
    let mut usr = [0.0_f64; 3];
    let mut w_temp = w;
    eraPn(pv, &mut w_temp, pu.as_mut_ptr());
    let vsr = eraPdp(pu.as_mut_ptr(), pv.add(3));
    eraSxp(vsr, pu.as_mut_ptr(), usr.as_mut_ptr());

    let mut ust = [0.0_f64; 3];
    eraPmp(pv.add(3), usr.as_mut_ptr(), ust.as_mut_ptr());
    let vst = eraPm(ust.as_mut_ptr());

    let betsr = vsr / ERFA_DC;
    let betst = vst / ERFA_DC;

    let mut betr = betsr;
    let mut bett = betst;
    let mut d = 1.0_f64;
    let mut del = 0.0_f64;
    let mut od = 0.0_f64;
    let mut odel = 0.0_f64;
    let mut odd = 0.0_f64;
    let mut oddel = 0.0_f64;

    let mut i = 0;
    while i < IMAX {
        d = 1.0 + betr;
        let w2 = betr * betr + bett * bett;
        del = -w2 / ((1.0 - w2).sqrt() + 1.0);
        betr = d * betsr + del;
        bett = d * betst;

        if i > 0 {
            let dd = (d - od).abs();
            let ddel = (del - odel).abs();
            if i > 1 && dd >= odd && ddel >= oddel {
                break;
            }
            odd = dd;
            oddel = ddel;
        }
        od = d;
        odel = del;
        i += 1;
    }

    if i >= IMAX {
        warn += 4;
    }

    let mut ut = [0.0_f64; 3];
    eraSxp(d, ust.as_mut_ptr(), ut.as_mut_ptr());

    let mut ur = [0.0_f64; 3];
    eraSxp(
        ERFA_DC * (d * betsr + del),
        pu.as_mut_ptr(),
        ur.as_mut_ptr(),
    );

    eraPpp(ur.as_mut_ptr(), ut.as_mut_ptr(), pv.add(3));

    warn
}

// eraSxp    scalar × p-vector
pub unsafe fn eraSxp(s: f64, p: *mut f64, sp: *mut f64) {
    *sp.add(0) = s * *p.add(0);
    *sp.add(1) = s * *p.add(1);
    *sp.add(2) = s * *p.add(2);
}

// eraSxpv    scalar × pv-vector
pub unsafe fn eraSxpv(s: f64, pv: *mut f64, spv: *mut f64) {
    eraS2xpv(s, s, pv, spv);
}
