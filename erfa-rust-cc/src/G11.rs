// G11
//   eceq06.c  → eraEceq06
//   ecm06.c   → eraEcm06
//   ee00.c    → eraEe00
//   ee00a.c   → eraEe00a
//   ee00b.c   → eraEe00b
//   ee06a.c   → eraEe06a
//   eect00.c  → eraEect00
//   eform.c   → eraEform
//   eo06a.c   → eraEo06a
//   eors.c    → eraEors

use crate::H1::*;

// G11/eceq06.c
pub unsafe fn eraEceq06(date1: f64, date2: f64, dl: f64, db: f64, dr: *mut f64, dd: *mut f64) {
    let mut rm = [[0.0f64; 3]; 3];
    let mut v1 = [0.0f64; 3];
    let mut v2 = [0.0f64; 3];
    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;

    eraS2c(dl, db, v1.as_mut_ptr());

    eraEcm06(date1, date2, rm.as_mut_ptr() as *mut f64);

    eraTrxp(rm.as_ptr() as *mut f64, v1.as_mut_ptr(), v2.as_mut_ptr());

    eraC2s(v2.as_mut_ptr(), &mut a, &mut b);

    *dr = eraAnp(a);
    *dd = eraAnpm(b);
}

// G11/ecm06.c
pub unsafe fn eraEcm06(date1: f64, date2: f64, rm: *mut f64) {
    let ob: f64;
    let mut bp = [[0.0f64; 3]; 3];
    let mut e = [[0.0f64; 3]; 3];

    ob = eraObl06(date1, date2);

    eraPmat06(date1, date2, bp.as_mut_ptr() as *mut f64);

    eraIr(e.as_mut_ptr() as *mut f64);
    eraRx(ob, e.as_mut_ptr() as *mut f64);

    eraRxr(e.as_ptr() as *mut f64, bp.as_ptr() as *mut f64, rm);
}

// G11/ee00.c
pub unsafe fn eraEe00(date1: f64, date2: f64, epsa: f64, dpsi: f64) -> f64 {
    let ee: f64;

    ee = dpsi * epsa.cos() + eraEect00(date1, date2);

    ee
}

// G11/ee00a.c
pub unsafe fn eraEe00a(date1: f64, date2: f64) -> f64 {
    let mut dpsipr: f64 = 0.0;
    let mut depspr: f64 = 0.0;
    let epsa: f64;
    let mut dpsi: f64 = 0.0;
    let mut deps: f64 = 0.0;
    let ee: f64;

    eraPr00(date1, date2, &mut dpsipr, &mut depspr);

    epsa = eraObl80(date1, date2) + depspr;

    eraNut00a(date1, date2, &mut dpsi, &mut deps);

    ee = eraEe00(date1, date2, epsa, dpsi);

    ee
}

// G11/ee00b.c
pub unsafe fn eraEe00b(date1: f64, date2: f64) -> f64 {
    let mut dpsipr: f64 = 0.0;
    let mut depspr: f64 = 0.0;
    let epsa: f64;
    let mut dpsi: f64 = 0.0;
    let mut deps: f64 = 0.0;
    let ee: f64;

    eraPr00(date1, date2, &mut dpsipr, &mut depspr);

    epsa = eraObl80(date1, date2) + depspr;

    eraNut00b(date1, date2, &mut dpsi, &mut deps);

    ee = eraEe00(date1, date2, epsa, dpsi);

    ee
}

// G11/ee06a.c
pub unsafe fn eraEe06a(date1: f64, date2: f64) -> f64 {
    let gst06a: f64;
    let gmst06: f64;
    let ee: f64;

    gst06a = eraGst06a(0.0, 0.0, date1, date2);
    gmst06 = eraGmst06(0.0, 0.0, date1, date2);

    ee = eraAnpm(gst06a - gmst06);

    ee
}

// G11/eect00.c
pub unsafe fn eraEect00(date1: f64, date2: f64) -> f64 {
    let t: f64;

    let mut a: f64;
    let mut s0: f64;
    let mut s1: f64;

    let mut fa = [0.0f64; 14];

    let eect: f64;

    #[repr(C)]
    struct TERM {
        nfa: [i32; 8],
        s: f64,
        c: f64,
    }

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

    const E1: &[TERM] = &[TERM {
        nfa: [0, 0, 0, 0, 1, 0, 0, 0],
        s: -0.87e-6,
        c: 0.00e-6,
    }];

    const NE0: usize = E0.len();
    const NE1: usize = E1.len();

    t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    fa[0] = eraFal03(t);

    fa[1] = eraFalp03(t);

    fa[2] = eraFaf03(t);

    fa[3] = eraFad03(t);

    fa[4] = eraFaom03(t);

    fa[5] = eraFave03(t);

    fa[6] = eraFae03(t);

    fa[7] = eraFapa03(t);

    s0 = 0.0;
    s1 = 0.0;

    for i in (0..NE0).rev() {
        a = 0.0;
        for j in 0..8 {
            a += (E0[i].nfa[j] as f64) * fa[j];
        }
        s0 += E0[i].s * a.sin() + E0[i].c * a.cos();
    }

    for i in (0..NE1).rev() {
        a = 0.0;
        for j in 0..8 {
            a += (E1[i].nfa[j] as f64) * fa[j];
        }
        s1 += E1[i].s * a.sin() + E1[i].c * a.cos();
    }

    eect = (s0 + s1 * t) * ERFA_DAS2R;

    eect
}

// G11/eform.c
pub unsafe fn eraEform(n: i32, a: *mut f64, f: *mut f64) -> i32 {
    match n {
        ERFA_WGS84 => {
            *a = 6378137.0;
            *f = 1.0 / 298.257223563;
        }
        ERFA_GRS80 => {
            *a = 6378137.0;
            *f = 1.0 / 298.257222101;
        }
        ERFA_WGS72 => {
            *a = 6378135.0;
            *f = 1.0 / 298.26;
        }
        _ => {
            *a = 0.0;
            *f = 0.0;
            return -1;
        }
    }

    0
}

// G11/eo06a.c
pub unsafe fn eraEo06a(date1: f64, date2: f64) -> f64 {
    let mut r = [[0.0f64; 3]; 3];
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let s: f64;
    let eo: f64;

    eraPnm06a(date1, date2, r.as_mut_ptr() as *mut f64);

    eraBpn2xy(r.as_ptr() as *mut f64, &mut x, &mut y);

    s = eraS06(date1, date2, x, y);

    eo = eraEors(r.as_ptr() as *mut f64, s);

    eo
}

// G11/eors.c
pub unsafe fn eraEors(rnpb: *mut f64, s: f64) -> f64 {
    let x: f64;
    let ax: f64;
    let xs: f64;
    let ys: f64;
    let zs: f64;
    let p: f64;
    let q: f64;
    let eo: f64;

    let rnpb_arr = rnpb as *mut [[f64; 3]; 3];
    let r = &*rnpb_arr;

    x = r[2][0];
    ax = x / (1.0 + r[2][2]);
    xs = 1.0 - ax * x;
    ys = -ax * r[2][1];
    zs = -x;
    p = r[0][0] * xs + r[0][1] * ys + r[0][2] * zs;
    q = r[1][0] * xs + r[1][1] * ys + r[1][2] * zs;
    eo = if p != 0.0 || q != 0.0 {
        s - q.atan2(p)
    } else {
        s
    };

    eo
}
