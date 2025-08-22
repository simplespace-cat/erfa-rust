// G29  CIO locator & spherical/Cartesian
//   s00.c   → eraS00
//   s00a.c  → eraS00a
//   s00b.c  → eraS00b
//   s2c.c   → eraS2c
//   s2p.c   → eraS2p
//   s2pv.c  → eraS2pv
//   s2xpv.c → eraS2xpv

use crate::H1::*;

// eraS00   CIO locator s, IAU 2000A (needs X,Y)
pub unsafe fn eraS00(date1: f64, date2: f64, x: f64, y: f64) -> f64 {
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    #[derive(Clone, Copy)]
    struct Term {
        nfa: [i32; 8],
        s: f64,
        c: f64,
    }

    const SP: [f64; 6] = [
        94.00e-6,
        3808.35e-6,
        -119.94e-6,
        -72574.09e-6,
        27.70e-6,
        15.61e-6,
    ];

    const S0: [Term; 33] = [
        Term {
            nfa: [0, 0, 0, 0, 1, 0, 0, 0],
            s: -2640.73e-6,
            c: 0.39e-6,
        },
        Term {
            nfa: [0, 0, 0, 0, 2, 0, 0, 0],
            s: -63.53e-6,
            c: 0.02e-6,
        },
        Term {
            nfa: [0, 0, 2, -2, 3, 0, 0, 0],
            s: -11.75e-6,
            c: -0.01e-6,
        },
        Term {
            nfa: [0, 0, 2, -2, 1, 0, 0, 0],
            s: -11.21e-6,
            c: -0.01e-6,
        },
        Term {
            nfa: [0, 0, 2, -2, 2, 0, 0, 0],
            s: 4.57e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [0, 0, 2, 0, 3, 0, 0, 0],
            s: -2.02e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [0, 0, 2, 0, 1, 0, 0, 0],
            s: -1.98e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [0, 0, 0, 0, 3, 0, 0, 0],
            s: 1.72e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [0, 1, 0, 0, 1, 0, 0, 0],
            s: 1.41e-6,
            c: 0.01e-6,
        },
        Term {
            nfa: [0, 1, 0, 0, -1, 0, 0, 0],
            s: 1.26e-6,
            c: 0.01e-6,
        },
        Term {
            nfa: [1, 0, 0, 0, -1, 0, 0, 0],
            s: 0.63e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [1, 0, 0, 0, 1, 0, 0, 0],
            s: 0.63e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [0, 1, 2, -2, 3, 0, 0, 0],
            s: -0.46e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [0, 1, 2, -2, 1, 0, 0, 0],
            s: -0.45e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [0, 0, 4, -4, 4, 0, 0, 0],
            s: -0.36e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [0, 0, 1, -1, 1, -8, 12, 0],
            s: 0.24e-6,
            c: 0.12e-6,
        },
        Term {
            nfa: [0, 0, 2, 0, 0, 0, 0, 0],
            s: -0.32e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [0, 0, 2, 0, 2, 0, 0, 0],
            s: -0.28e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [1, 0, 2, 0, 3, 0, 0, 0],
            s: -0.27e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [1, 0, 2, 0, 1, 0, 0, 0],
            s: -0.26e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [0, 0, 2, -2, 0, 0, 0, 0],
            s: 0.21e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [0, 1, -2, 2, -3, 0, 0, 0],
            s: -0.19e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [0, 1, -2, 2, -1, 0, 0, 0],
            s: -0.18e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [0, 0, 0, 0, 0, 8, -13, -1],
            s: 0.10e-6,
            c: -0.05e-6,
        },
        Term {
            nfa: [0, 0, 0, 2, 0, 0, 0, 0],
            s: -0.15e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [2, 0, -2, 0, -1, 0, 0, 0],
            s: 0.14e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [0, 1, 2, -2, 2, 0, 0, 0],
            s: 0.14e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [1, 0, 0, -2, 1, 0, 0, 0],
            s: -0.14e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [1, 0, 0, -2, -1, 0, 0, 0],
            s: -0.14e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [0, 0, 4, -2, 4, 0, 0, 0],
            s: -0.13e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [0, 0, 2, -2, 4, 0, 0, 0],
            s: 0.11e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [1, 0, -2, 0, -3, 0, 0, 0],
            s: -0.11e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [1, 0, -2, 0, -1, 0, 0, 0],
            s: -0.11e-6,
            c: 0.00e-6,
        },
    ];

    const S1: [Term; 3] = [
        Term {
            nfa: [0, 0, 0, 0, 2, 0, 0, 0],
            s: -0.07e-6,
            c: 3.57e-6,
        },
        Term {
            nfa: [0, 0, 0, 0, 1, 0, 0, 0],
            s: 1.71e-6,
            c: -0.03e-6,
        },
        Term {
            nfa: [0, 0, 2, -2, 3, 0, 0, 0],
            s: 0.00e-6,
            c: 0.48e-6,
        },
    ];

    const S2: [Term; 25] = [
        Term {
            nfa: [0, 0, 0, 0, 1, 0, 0, 0],
            s: 743.53e-6,
            c: -0.17e-6,
        },
        Term {
            nfa: [0, 0, 2, -2, 2, 0, 0, 0],
            s: 56.91e-6,
            c: 0.06e-6,
        },
        Term {
            nfa: [0, 0, 2, 0, 2, 0, 0, 0],
            s: 9.84e-6,
            c: -0.01e-6,
        },
        Term {
            nfa: [0, 0, 0, 0, 2, 0, 0, 0],
            s: -8.85e-6,
            c: 0.01e-6,
        },
        Term {
            nfa: [0, 1, 0, 0, 0, 0, 0, 0],
            s: -6.38e-6,
            c: -0.05e-6,
        },
        Term {
            nfa: [1, 0, 0, 0, 0, 0, 0, 0],
            s: -3.07e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [0, 1, 2, -2, 2, 0, 0, 0],
            s: 2.23e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [0, 0, 2, 0, 1, 0, 0, 0],
            s: 1.67e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [1, 0, 2, 0, 2, 0, 0, 0],
            s: 1.30e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [0, 1, -2, 2, -2, 0, 0, 0],
            s: 0.93e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [1, 0, 0, -2, 0, 0, 0, 0],
            s: 0.68e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [0, 0, 2, -2, 1, 0, 0, 0],
            s: -0.55e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [1, 0, -2, 0, -2, 0, 0, 0],
            s: 0.53e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [0, 0, 0, 2, 0, 0, 0, 0],
            s: -0.27e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [1, 0, 0, 0, 1, 0, 0, 0],
            s: -0.27e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [1, 0, -2, -2, -2, 0, 0, 0],
            s: -0.26e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [1, 0, 0, 0, -1, 0, 0, 0],
            s: -0.25e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [1, 0, 2, 0, 1, 0, 0, 0],
            s: 0.22e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [2, 0, 0, -2, 0, 0, 0, 0],
            s: -0.21e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [2, 0, -2, 0, -1, 0, 0, 0],
            s: 0.20e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [0, 0, 2, 2, 2, 0, 0, 0],
            s: 0.17e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [2, 0, 2, 0, 2, 0, 0, 0],
            s: 0.13e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [2, 0, 0, 0, 0, 0, 0, 0],
            s: -0.13e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [1, 0, 2, -2, 2, 0, 0, 0],
            s: -0.12e-6,
            c: 0.00e-6,
        },
        Term {
            nfa: [0, 0, 2, 0, 0, 0, 0, 0],
            s: -0.11e-6,
            c: 0.00e-6,
        },
    ];

    const S3: [Term; 4] = [
        Term {
            nfa: [0, 0, 0, 0, 1, 0, 0, 0],
            s: 0.30e-6,
            c: -23.51e-6,
        },
        Term {
            nfa: [0, 0, 2, -2, 2, 0, 0, 0],
            s: -0.03e-6,
            c: -1.39e-6,
        },
        Term {
            nfa: [0, 0, 2, 0, 2, 0, 0, 0],
            s: -0.01e-6,
            c: -0.24e-6,
        },
        Term {
            nfa: [0, 0, 0, 0, 2, 0, 0, 0],
            s: 0.00e-6,
            c: 0.22e-6,
        },
    ];

    /* Terms order t^4 */
    const S4: [Term; 1] = [Term {
        nfa: [0, 0, 0, 0, 1, 0, 0, 0],
        s: -0.26e-6,
        c: -0.01e-6,
    }];

    let mut fa = [0.0_f64; 8];
    fa[0] = eraFal03(t);
    fa[1] = eraFalp03(t);
    fa[2] = eraFaf03(t);
    fa[3] = eraFad03(t);
    fa[4] = eraFaom03(t);
    fa[5] = eraFave03(t);
    fa[6] = eraFae03(t);
    fa[7] = eraFapa03(t);

    let mut w = [SP[0], SP[1], SP[2], SP[3], SP[4], SP[5]];

    macro_rules! accumulate {
        ($arr:ident, $idx:expr) => {
            for term in &$arr {
                let mut a = 0.0_f64;
                for j in 0..8 {
                    a += (term.nfa[j] as f64) * fa[j];
                }
                w[$idx] += term.s * a.sin() + term.c * a.cos();
            }
        };
    }
    accumulate!(S0, 0);
    accumulate!(S1, 1);
    accumulate!(S2, 2);
    accumulate!(S3, 3);
    accumulate!(S4, 4);

    let s = (w[0] + (w[1] + (w[2] + (w[3] + (w[4] + w[5] * t) * t) * t) * t) * t) * ERFA_DAS2R
        - x * y / 2.0;

    s
}

// eraS00a  CIO locator using full IAU 2000A model
pub unsafe fn eraS00a(date1: f64, date2: f64) -> f64 {
    let mut rbpn = [0.0_f64; 9];
    let mut x = 0.0_f64;
    let mut y = 0.0_f64;

    eraPnm00a(date1, date2, rbpn.as_mut_ptr());
    eraBpn2xy(rbpn.as_mut_ptr(), &mut x, &mut y);

    eraS00(date1, date2, x, y)
}

// eraS00b  CIO locator using truncated IAU 2000B model
pub unsafe fn eraS00b(date1: f64, date2: f64) -> f64 {
    let mut rbpn = [0.0_f64; 9];
    let mut x = 0.0_f64;
    let mut y = 0.0_f64;

    eraPnm00b(date1, date2, rbpn.as_mut_ptr());
    eraBpn2xy(rbpn.as_mut_ptr(), &mut x, &mut y);

    eraS00(date1, date2, x, y)
}

// eraS2c  spherical to Cartesian (unit vector)
pub unsafe fn eraS2c(theta: f64, phi: f64, c: *mut f64) {
    let cp = phi.cos();
    *c.add(0) = theta.cos() * cp;
    *c.add(1) = theta.sin() * cp;
    *c.add(2) = phi.sin();
}

// eraS2p  spherical polar to position vector
pub unsafe fn eraS2p(theta: f64, phi: f64, r: f64, p: *mut f64) {
    let mut u = [0.0_f64; 3];
    eraS2c(theta, phi, u.as_mut_ptr());
    eraSxp(r, u.as_mut_ptr(), p);
}

// eraS2pv  spherical ⟶ pv-vector
pub unsafe fn eraS2pv(theta: f64, phi: f64, r: f64, td: f64, pd: f64, rd: f64, pv: *mut f64) {
    let st = theta.sin();
    let ct = theta.cos();
    let sp = phi.sin();
    let cp = phi.cos();

    let rcp = r * cp;
    let x = rcp * ct;
    let y = rcp * st;
    let rpd = r * pd;
    let w = rpd * sp - cp * rd;

    *pv.add(0) = x;
    *pv.add(1) = y;
    *pv.add(2) = r * sp;

    *pv.add(3) = -y * td - w * ct;
    *pv.add(4) = x * td - w * st;
    *pv.add(5) = rpd * cp + sp * rd;
}

// eraS2xpv  scale pv-vector components by two scalars
pub unsafe fn eraS2xpv(s1: f64, s2: f64, pv: *mut f64, spv: *mut f64) {
    eraSxp(s1, pv, spv);
    eraSxp(s2, pv.add(3), spv.add(3));
}
