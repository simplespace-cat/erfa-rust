// G11
//   eceq06.c  → eraEceq06_safe
//   ecm06.c   → eraEcm06_safe
//   ee00.c    → eraEe00_safe
//   ee00a.c   → eraEe00a_safe
//   ee00b.c   → eraEe00b_safe
//   ee06a.c   → eraEe06a_safe
//   eect00.c  → eraEect00_safe
//   eform.c   → eraEform_safe
//   eo06a.c   → eraEo06a_safe
//   eors.c    → eraEors_safe

use crate::G15_safe::{
    eraFad03_safe, eraFae03_safe, eraFaf03_safe, eraFal03_safe, eraFalp03_safe, eraFaom03_safe,
    eraFapa03_safe, eraFave03_safe,
};
use crate::G17_safe::{eraGmst06_safe, eraGst06a_safe};
use crate::G19_safe::eraIr_safe;
use crate::G1_safe::{eraAnp_safe, eraAnpm_safe};
use crate::G22_safe::eraNut00a_safe;
use crate::G23_safe::{eraNut00b_safe, eraObl06_safe, eraObl80_safe};
use crate::G25_safe::eraPmat06_safe;
use crate::G26_safe::{eraPnm06a_safe, eraPr00_safe};
use crate::G28_safe::{eraRx_safe, eraRxr_safe};
use crate::G29_safe::eraS2c_safe;
use crate::G30_safe::eraS06_safe;
use crate::G33_safe::eraTrxp_safe;
use crate::G6_safe::eraBpn2xy_safe;
use crate::G7_safe::eraC2s_safe;
use crate::H1_safe::{ERFA_DAS2R, ERFA_DJ00, ERFA_DJC, ERFA_GRS80, ERFA_WGS72, ERFA_WGS84};

pub type ErfaResult<T> = Result<T, ()>;

// Ecliptic to ICRS (equatorial), IAU 2006.
pub fn eraEceq06_safe(date1: f64, date2: f64, dl: f64, db: f64) -> ErfaResult<(f64, f64)> {
    let v1 = eraS2c_safe(dl, db)?;
    let mut rm = [[0.0_f64; 3]; 3];
    eraEcm06_safe(date1, date2, &mut rm)?;
    let v2 = eraTrxp_safe(&rm, &v1)?;
    let (a, b) = eraC2s_safe(&v2)?;
    let dr = eraAnp_safe(a)?;
    let dd = eraAnpm_safe(b)?;
    Ok((dr, dd))
}

// Rotation matrix, ICRS equatorial to ecliptic (IAU 2006).
pub fn eraEcm06_safe(date1: f64, date2: f64, rm: &mut [[f64; 3]; 3]) -> ErfaResult<()> {
    let ob = eraObl06_safe(date1, date2)?;
    let bp = eraPmat06_safe(date1, date2)?;
    let mut e = [[0.0_f64; 3]; 3];
    eraIr_safe(&mut e)?;
    eraRx_safe(ob, &mut e)?;
    let m = eraRxr_safe(&e, &bp)?;
    *rm = m;
    Ok(())
}

// Equation of the equinoxes (IAU 2000), given epsa and dpsi plus complementary terms.
pub fn eraEe00_safe(date1: f64, date2: f64, epsa: f64, dpsi: f64) -> ErfaResult<f64> {
    let ee = dpsi * epsa.cos() + eraEect00_safe(date1, date2)?;
    Ok(ee)
}

// Equation of the equinoxes consistent with IAU 2000A nutation.
pub fn eraEe00a_safe(date1: f64, date2: f64) -> ErfaResult<f64> {
    let (_dpsipr, depspr) = eraPr00_safe(date1, date2)?;
    let epsa = eraObl80_safe(date1, date2)? + depspr;
    let (dpsi, _deps) = eraNut00a_safe(date1, date2)?;
    eraEe00_safe(date1, date2, epsa, dpsi)
}

// Equation of the equinoxes consistent with IAU 2000B nutation.
pub fn eraEe00b_safe(date1: f64, date2: f64) -> ErfaResult<f64> {
    let (_dpsipr, depspr) = eraPr00_safe(date1, date2)?;
    let epsa = eraObl80_safe(date1, date2)? + depspr;
    let (dpsi, _deps) = eraNut00b_safe(date1, date2)?;
    eraEe00_safe(date1, date2, epsa, dpsi)
}

// Equation of the equinoxes, IAU 2006/2000A.
pub fn eraEe06a_safe(date1: f64, date2: f64) -> ErfaResult<f64> {
    let gst06a = eraGst06a_safe(0.0, 0.0, date1, date2)?;
    let gmst06 = eraGmst06_safe(0.0, 0.0, date1, date2)?;
    let ee = eraAnpm_safe(gst06a - gmst06)?;
    Ok(ee)
}

// Complementary terms for the equation of the equinoxes (IAU 2000), radians.
pub fn eraEect00_safe(date1: f64, date2: f64) -> ErfaResult<f64> {
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    #[repr(C)]
    struct TERM {
        nfa: [i32; 8],
        s: f64,
        c: f64,
    }

    // Terms of order t^0
    const E0: &[TERM] = &[
        TERM {
            nfa: [0, 0, 0, 0, 1, 0, 0, 0],
            s: 2640.96e-6,
            c: -0.39e-6,
        },
        TERM {
            nfa: [0, 0, 0, 0, 2, 0, 0, 0],
            s: 63.52e-6,
            c: -0.02e-6,
        },
        TERM {
            nfa: [0, 0, 2, -2, 3, 0, 0, 0],
            s: 11.75e-6,
            c: 0.01e-6,
        },
        TERM {
            nfa: [0, 0, 2, -2, 1, 0, 0, 0],
            s: 11.21e-6,
            c: 0.01e-6,
        },
        TERM {
            nfa: [0, 0, 2, -2, 2, 0, 0, 0],
            s: -4.55e-6,
            c: 0.00e-6,
        },
        TERM {
            nfa: [0, 0, 2, 0, 3, 0, 0, 0],
            s: 2.02e-6,
            c: 0.00e-6,
        },
        TERM {
            nfa: [0, 0, 2, 0, 1, 0, 0, 0],
            s: 1.98e-6,
            c: 0.00e-6,
        },
        TERM {
            nfa: [0, 0, 0, 0, 3, 0, 0, 0],
            s: -1.72e-6,
            c: 0.00e-6,
        },
        TERM {
            nfa: [0, 1, 0, 0, 1, 0, 0, 0],
            s: -1.41e-6,
            c: -0.01e-6,
        },
        TERM {
            nfa: [0, 1, 0, 0, -1, 0, 0, 0],
            s: -1.26e-6,
            c: -0.01e-6,
        },
        TERM {
            nfa: [1, 0, 0, 0, -1, 0, 0, 0],
            s: -0.63e-6,
            c: 0.00e-6,
        },
        TERM {
            nfa: [1, 0, 0, 0, 1, 0, 0, 0],
            s: -0.63e-6,
            c: 0.00e-6,
        },
        TERM {
            nfa: [0, 1, 2, -2, 3, 0, 0, 0],
            s: 0.46e-6,
            c: 0.00e-6,
        },
        TERM {
            nfa: [0, 1, 2, -2, 1, 0, 0, 0],
            s: 0.45e-6,
            c: 0.00e-6,
        },
        TERM {
            nfa: [0, 0, 4, -4, 4, 0, 0, 0],
            s: 0.36e-6,
            c: 0.00e-6,
        },
        TERM {
            nfa: [0, 0, 1, -1, 1, -8, 12, 0],
            s: -0.24e-6,
            c: -0.12e-6,
        },
        TERM {
            nfa: [0, 0, 2, 0, 0, 0, 0, 0],
            s: 0.32e-6,
            c: 0.00e-6,
        },
        TERM {
            nfa: [0, 0, 2, 0, 2, 0, 0, 0],
            s: 0.28e-6,
            c: 0.00e-6,
        },
        TERM {
            nfa: [1, 0, 2, 0, 3, 0, 0, 0],
            s: 0.27e-6,
            c: 0.00e-6,
        },
        TERM {
            nfa: [1, 0, 2, 0, 1, 0, 0, 0],
            s: 0.26e-6,
            c: 0.00e-6,
        },
        TERM {
            nfa: [0, 0, 2, -2, 0, 0, 0, 0],
            s: -0.21e-6,
            c: 0.00e-6,
        },
        TERM {
            nfa: [0, 1, -2, 2, -3, 0, 0, 0],
            s: 0.19e-6,
            c: 0.00e-6,
        },
        TERM {
            nfa: [0, 1, -2, 2, -1, 0, 0, 0],
            s: 0.18e-6,
            c: 0.00e-6,
        },
        TERM {
            nfa: [0, 0, 0, 0, 0, 8, -13, -1],
            s: -0.10e-6,
            c: 0.05e-6,
        },
        TERM {
            nfa: [0, 0, 0, 2, 0, 0, 0, 0],
            s: 0.15e-6,
            c: 0.00e-6,
        },
        TERM {
            nfa: [2, 0, -2, 0, -1, 0, 0, 0],
            s: -0.14e-6,
            c: 0.00e-6,
        },
        TERM {
            nfa: [1, 0, 0, -2, 1, 0, 0, 0],
            s: 0.14e-6,
            c: 0.00e-6,
        },
        TERM {
            nfa: [0, 1, 2, -2, 2, 0, 0, 0],
            s: -0.14e-6,
            c: 0.00e-6,
        },
        TERM {
            nfa: [1, 0, 0, -2, -1, 0, 0, 0],
            s: 0.14e-6,
            c: 0.00e-6,
        },
        TERM {
            nfa: [0, 0, 4, -2, 4, 0, 0, 0],
            s: 0.13e-6,
            c: 0.00e-6,
        },
        TERM {
            nfa: [0, 0, 2, -2, 4, 0, 0, 0],
            s: -0.11e-6,
            c: 0.00e-6,
        },
        TERM {
            nfa: [1, 0, -2, 0, -3, 0, 0, 0],
            s: 0.11e-6,
            c: 0.00e-6,
        },
        TERM {
            nfa: [1, 0, -2, 0, -1, 0, 0, 0],
            s: 0.11e-6,
            c: 0.00e-6,
        },
    ];

    // Terms of order t^1
    const E1: &[TERM] = &[TERM {
        nfa: [0, 0, 0, 0, 1, 0, 0, 0],
        s: -0.87e-6,
        c: 0.00e-6,
    }];

    const NE0: usize = 33;
    const NE1: usize = 1;

    let mut fa = [0.0_f64; 14];
    fa[0] = eraFal03_safe(t)?;
    fa[1] = eraFalp03_safe(t)?;
    fa[2] = eraFaf03_safe(t)?;
    fa[3] = eraFad03_safe(t)?;
    fa[4] = eraFaom03_safe(t)?;
    fa[5] = eraFave03_safe(t)?;
    fa[6] = eraFae03_safe(t)?;
    fa[7] = eraFapa03_safe(t)?;

    let mut s0 = 0.0_f64;
    let mut s1 = 0.0_f64;

    for i in (0..NE0).rev() {
        let mut a = 0.0_f64;
        for j in 0..8 {
            a += (E0[i].nfa[j] as f64) * fa[j];
        }
        s0 += E0[i].s * a.sin() + E0[i].c * a.cos();
    }

    for i in (0..NE1).rev() {
        let mut a = 0.0_f64;
        for j in 0..8 {
            a += (E1[i].nfa[j] as f64) * fa[j];
        }
        s1 += E1[i].s * a.sin() + E1[i].c * a.cos();
    }

    let eect = (s0 + s1 * t) * ERFA_DAS2R;
    Ok(eect)
}

// Reference ellipsoid parameters; returns ((a, f), j).
pub fn eraEform_safe(n: i32) -> ErfaResult<((f64, f64), i32)> {
    let (a, f, j) = match n {
        ERFA_WGS84 => (6_378_137.0, 1.0 / 298.257_223_563, 0),
        ERFA_GRS80 => (6_378_137.0, 1.0 / 298.257_222_101, 0),
        ERFA_WGS72 => (6_378_135.0, 1.0 / 298.26, 0),
        _ => (0.0, 0.0, -1),
    };
    Ok(((a, f), j))
}

// Equation of origins, IAU 2006/2000A.
pub fn eraEo06a_safe(date1: f64, date2: f64) -> ErfaResult<f64> {
    let r = eraPnm06a_safe(date1, date2)?;
    let (x, y) = eraBpn2xy_safe(&r)?;
    let s = eraS06_safe(date1, date2, x, y)?;
    let eo = eraEors_safe(&r, s)?;
    Ok(eo)
}

// Equation of the origins, given NPB matrix and s.
pub fn eraEors_safe(rnpb: &[[f64; 3]; 3], s: f64) -> ErfaResult<f64> {
    let x = rnpb[2][0];
    let ax = x / (1.0 + rnpb[2][2]);
    let xs = 1.0 - ax * x;
    let ys = -ax * rnpb[2][1];
    let zs = -x;
    let p = rnpb[0][0] * xs + rnpb[0][1] * ys + rnpb[0][2] * zs;
    let q = rnpb[1][0] * xs + rnpb[1][1] * ys + rnpb[1][2] * zs;
    let eo = if p != 0.0 || q != 0.0 {
        s - q.atan2(p)
    } else {
        s
    };
    Ok(eo)
}
