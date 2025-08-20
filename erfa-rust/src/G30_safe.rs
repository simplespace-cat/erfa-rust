// G30
//   s06.c    → eraS06_safe
//   s06a.c   → eraS06a_safe
//   sepp.c   → eraSepp_safe
//   seps.c   → eraSeps_safe
//   sp00.c   → eraSp00_safe
//   starpm.c → eraStarpm_safe
//   starpv.c → eraStarpv_safe
//   sxp.c    → eraSxp_safe
//   sxpv.c   → eraSxpv_safe

use crate::G15_safe::{
    eraFad03_safe, eraFae03_safe, eraFaf03_safe, eraFal03_safe, eraFalp03_safe, eraFaom03_safe,
    eraFapa03_safe, eraFave03_safe,
};
use crate::G24_safe::{eraPdp_safe, eraPm_safe};
use crate::G25_safe::{eraPmp_safe, eraPn_safe};
use crate::G26_safe::{eraPnm06a_safe, eraPpp_safe};
use crate::G27_safe::{eraPvstar_safe, eraPvu_safe, eraPxp_safe};
use crate::G29_safe::{eraS2c_safe, eraS2pv_safe, eraS2xpv_safe};
use crate::G35_safe::eraZp_safe;
use crate::G6_safe::eraBpn2xy_safe;
use crate::H1_safe::{
    ERFA_DAS2R, ERFA_DAU, ERFA_DAYSEC, ERFA_DC, ERFA_DJ00, ERFA_DJC, ERFA_DJY, ERFA_DR2AS,
};

pub type ErfaResult<T> = Result<T, ()>;

//----------------------------------------------------------------------
// G30/s06.c
//----------------------------------------------------------------------
// Internal data for s06 series
#[derive(Clone, Copy)]
struct TERM {
    nfa: [i32; 8], // coefficients of l,l',F,D,Om,LVe,LE,pA
    s: f64,        // sine coefficient (µas)
    c: f64,        // cosine coefficient (µas)
}

const SP: [f64; 6] = [
    94.00e-6,
    3808.65e-6,
    -122.68e-6,
    -72574.11e-6,
    27.98e-6,
    15.62e-6,
];

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
    // 11-20
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
    // 21-30
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
    // 31-33
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

// t^1
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

// t^2
const S2: [TERM; 25] = [
    // 1-10
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
    // 11-20
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
    // 21-25
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

// t^3
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

// t^4
const S4: [TERM; 1] = [TERM {
    nfa: [0, 0, 0, 0, 1, 0, 0, 0],
    s: -0.26e-6,
    c: -0.01e-6,
}];

// CIO locator s (IAU 2006/2000A), given CIP X,Y.
pub fn eraS06_safe(date1: f64, date2: f64, x: f64, y: f64) -> ErfaResult<f64> {
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    let mut fa = [0.0_f64; 8];
    fa[0] = eraFal03_safe(t)?; // l
    fa[1] = eraFalp03_safe(t)?; // l'
    fa[2] = eraFaf03_safe(t)?; // F
    fa[3] = eraFad03_safe(t)?; // D
    fa[4] = eraFaom03_safe(t)?; // Om
    fa[5] = eraFave03_safe(t)?; // LVe
    fa[6] = eraFae03_safe(t)?; // LE
    fa[7] = eraFapa03_safe(t)?; // pA

    // evaluate series for s + X*Y/2
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

    Ok(s)
}

//----------------------------------------------------------------------
// G30/s06a.c
//----------------------------------------------------------------------
// CIO locator s, IAU 2006/2000A (auto X,Y).
pub fn eraS06a_safe(date1: f64, date2: f64) -> ErfaResult<f64> {
    let rnpb = eraPnm06a_safe(date1, date2)?;
    let (x, y) = eraBpn2xy_safe(&rnpb)?;
    eraS06_safe(date1, date2, x, y)
}

//----------------------------------------------------------------------
// G30/sepp.c
//----------------------------------------------------------------------
// Angular separation between two p-vectors.
pub fn eraSepp_safe(a: &[f64; 3], b: &[f64; 3]) -> ErfaResult<f64> {
    let axb = eraPxp_safe(a, b)?;
    let ss = eraPm_safe(&axb)?;
    let cs = eraPdp_safe(a, b)?;
    let ang = if ss != 0.0 || cs != 0.0 {
        ss.atan2(cs)
    } else {
        0.0
    };
    Ok(ang)
}

//----------------------------------------------------------------------
// G30/seps.c
//----------------------------------------------------------------------
// Angular separation between two spherical positions.
pub fn eraSeps_safe(al: f64, ap: f64, bl: f64, bp: f64) -> ErfaResult<f64> {
    let ac = eraS2c_safe(al, ap)?;
    let bc = eraS2c_safe(bl, bp)?;
    eraSepp_safe(&ac, &bc)
}

//----------------------------------------------------------------------
// G30/sp00.c
//----------------------------------------------------------------------
// TIO locator s′ (linear model).
pub fn eraSp00_safe(date1: f64, date2: f64) -> ErfaResult<f64> {
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;
    Ok(-47e-6 * t * ERFA_DAS2R)
}

//----------------------------------------------------------------------
// G30/starpm.c
//----------------------------------------------------------------------
// Propagate star catalog data for space motion.
pub fn eraStarpm_safe(
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
) -> ErfaResult<((f64, f64, f64, f64, f64, f64), i32)> {
    let (pv1, j1) = eraStarpv_safe(ra1, dec1, pmr1, pmd1, px1, rv1)?;
    let tl1 = eraPm_safe(&pv1[0])? / ERFA_DC;
    let dt = (ep2a - ep1a) + (ep2b - ep1b);
    let pv = eraPvu_safe(dt + tl1, &pv1)?;

    let r2 = eraPdp_safe(&pv[0], &pv[0])?;
    let rdv = eraPdp_safe(&pv[0], &pv[1])?;
    let v2 = eraPdp_safe(&pv[1], &pv[1])?;
    let c2mv2 = ERFA_DC * ERFA_DC - v2;
    if c2mv2 <= 0.0 {
        return Ok(((0.0, 0.0, 0.0, 0.0, 0.0, 0.0), -1));
    }
    let tl2 = (-rdv + (rdv * rdv + c2mv2 * r2).sqrt()) / c2mv2;

    let pv2 = eraPvu_safe(dt + (tl1 - tl2), &pv1)?;
    let ((ra2, dec2, pmr2, pmd2, px2, rv2), j2) = eraPvstar_safe(&pv2)?;
    let j = if j2 == 0 { j1 } else { -1 };
    Ok(((ra2, dec2, pmr2, pmd2, px2, rv2), j))
}

//----------------------------------------------------------------------
// G30/starpv.c
//----------------------------------------------------------------------
// Catalog data to position/velocity vector.
pub fn eraStarpv_safe(
    ra: f64,
    dec: f64,
    pmr: f64,
    pmd: f64,
    px: f64,
    rv: f64,
) -> ErfaResult<([[f64; 3]; 2], i32)> {
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

    let pv_init = eraS2pv_safe(ra, dec, r, rad, decd, rd)?;

    let mut pv = pv_init;
    let v = eraPm_safe(&pv[1])?;
    if v / ERFA_DC > VMAX {
        pv[1] = eraZp_safe();
        warn += 2;
    }

    let (_r_u, pu) = eraPn_safe(&pv[0])?;
    let vsr = eraPdp_safe(&pu, &pv[1])?;
    let usr = eraSxp_safe(vsr, &pu)?;

    let ust = eraPmp_safe(&pv[1], &usr)?;
    let vst = eraPm_safe(&ust)?;

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

    let ut = eraSxp_safe(d, &ust)?;
    let ur = eraSxp_safe(ERFA_DC * (d * betsr + del), &pu)?;
    let vfin = eraPpp_safe(&ur, &ut)?;
    pv[1] = vfin;

    Ok((pv, warn))
}

//----------------------------------------------------------------------
// G30/sxp.c
//----------------------------------------------------------------------
// Scalar × p-vector.
pub fn eraSxp_safe(s: f64, p: &[f64; 3]) -> ErfaResult<[f64; 3]> {
    Ok([s * p[0], s * p[1], s * p[2]])
}

//----------------------------------------------------------------------
// G30/sxpv.c
//----------------------------------------------------------------------
// Scalar × pv-vector.
pub fn eraSxpv_safe(s: f64, pv: &[[f64; 3]; 2]) -> ErfaResult<[[f64; 3]; 2]> {
    eraS2xpv_safe(s, s, pv)
}
