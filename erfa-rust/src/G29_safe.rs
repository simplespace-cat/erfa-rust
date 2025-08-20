// G29_safe  CIO locator & spherical/Cartesian
//   s00.c   → eraS00_safe
//   s00a.c  → eraS00a_safe
//   s00b.c  → eraS00b_safe
//   s2c.c   → eraS2c_safe
//   s2p.c   → eraS2p_safe
//   s2pv.c  → eraS2pv_safe
//   s2xpv.c → eraS2xpv_safe

use crate::G15_safe::{
    eraFad03_safe, eraFae03_safe, eraFaf03_safe, eraFal03_safe, eraFalp03_safe, eraFaom03_safe,
    eraFapa03_safe, eraFave03_safe,
};
use crate::G26_safe::{eraPnm00a_safe, eraPnm00b_safe};
use crate::G6_safe::eraBpn2xy_safe;
use crate::H1_safe::{ERFA_DAS2R, ERFA_DJ00, ERFA_DJC};

pub type ErfaResult<T> = Result<T, ()>;

// ===================================
// G29/s00.c
// ===================================
// Compute CIO locator s (IAU 2000), given date and X,Y.
pub fn eraS00_safe(date1: f64, date2: f64, x: f64, y: f64) -> ErfaResult<f64> {
    // Time since J2000.0 (Julian centuries)
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    // Series for s + X*Y/2
    #[derive(Clone, Copy)]
    struct Term {
        nfa: [i32; 8], // coeffs of l,l',F,D,Om,LVe,LE,pA
        s: f64,        // sine coefficient (µas)
        c: f64,        // cosine coefficient (µas)
    }

    // Polynomial coefficients (µas)
    const SP: [f64; 6] = [
        94.00e-6,
        3808.35e-6,
        -119.94e-6,
        -72574.09e-6,
        27.70e-6,
        15.61e-6,
    ];

    // Terms order t^0
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

    // Terms order t^1
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

    // Terms order t^2
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

    // Terms order t^3
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

    // Terms order t^4
    const S4: [Term; 1] = [Term {
        nfa: [0, 0, 0, 0, 1, 0, 0, 0],
        s: -0.26e-6,
        c: -0.01e-6,
    }];

    // Fundamental arguments (radians)
    let mut fa = [0.0_f64; 8];
    fa[0] = eraFal03_safe(t)?;
    fa[1] = eraFalp03_safe(t)?;
    fa[2] = eraFaf03_safe(t)?;
    fa[3] = eraFad03_safe(t)?;
    fa[4] = eraFaom03_safe(t)?;
    fa[5] = eraFave03_safe(t)?;
    fa[6] = eraFae03_safe(t)?;
    fa[7] = eraFapa03_safe(t)?;

    // Evaluate series
    let mut w = [SP[0], SP[1], SP[2], SP[3], SP[4], SP[5]];

    #[inline(always)]
    fn accum(arr: &[Term], idx: usize, fa: &[f64; 8], w: &mut [f64; 6]) {
        for term in arr {
            let mut a = 0.0_f64;
            for j in 0..8 {
                a += (term.nfa[j] as f64) * fa[j];
            }
            w[idx] += term.s * a.sin() + term.c * a.cos();
        }
    }
    accum(&S0, 0, &fa, &mut w);
    accum(&S1, 1, &fa, &mut w);
    accum(&S2, 2, &fa, &mut w);
    accum(&S3, 3, &fa, &mut w);
    accum(&S4, 4, &fa, &mut w);

    // Final value (radians)
    let s = (w[0] + (w[1] + (w[2] + (w[3] + (w[4] + w[5] * t) * t) * t) * t) * t) * ERFA_DAS2R
        - x * y / 2.0;

    Ok(s)
}

// ===================================
// G29/s00a.c
// ===================================
// CIO locator s using full IAU 2000A model.
pub fn eraS00a_safe(date1: f64, date2: f64) -> ErfaResult<f64> {
    let rbpn = eraPnm00a_safe(date1, date2)?;
    let (x, y) = eraBpn2xy_safe(&rbpn)?;
    eraS00_safe(date1, date2, x, y)
}

// ===================================
// G29/s00b.c
// ===================================
// CIO locator s using truncated IAU 2000B model.
pub fn eraS00b_safe(date1: f64, date2: f64) -> ErfaResult<f64> {
    let rbpn = eraPnm00b_safe(date1, date2)?;
    let (x, y) = eraBpn2xy_safe(&rbpn)?;
    eraS00_safe(date1, date2, x, y)
}

// ===================================
// G29/s2c.c
// ===================================
// Convert spherical angles to Cartesian unit vector.
pub fn eraS2c_safe(theta: f64, phi: f64) -> ErfaResult<[f64; 3]> {
    let cp = phi.cos();
    Ok([theta.cos() * cp, theta.sin() * cp, phi.sin()])
}

// ===================================
// G29/s2p.c
// ===================================
// Convert spherical polar to position vector with radius r.
pub fn eraS2p_safe(theta: f64, phi: f64, r: f64) -> ErfaResult<[f64; 3]> {
    let u = eraS2c_safe(theta, phi)?;
    Ok([r * u[0], r * u[1], r * u[2]])
}

// ===================================
// G29/s2pv.c
// ===================================
// Convert spherical angles and rates to pv-vector.
pub fn eraS2pv_safe(
    theta: f64,
    phi: f64,
    r: f64,
    td: f64,
    pd: f64,
    rd: f64,
) -> ErfaResult<[[f64; 3]; 2]> {
    let st = theta.sin();
    let ct = theta.cos();
    let sp = phi.sin();
    let cp = phi.cos();

    let rcp = r * cp;
    let x = rcp * ct;
    let y = rcp * st;
    let rpd = r * pd;
    let w = rpd * sp - cp * rd;

    let p = [x, y, r * sp];
    let v = [-y * td - w * ct, x * td - w * st, rpd * cp + sp * rd];

    Ok([p, v])
}

// ===================================
// G29/s2xpv.c
// ===================================
// Scale pv-vector components by two scalars (position by s1, velocity by s2).
pub fn eraS2xpv_safe(s1: f64, s2: f64, pv: &[[f64; 3]; 2]) -> ErfaResult<[[f64; 3]; 2]> {
    let p = [s1 * pv[0][0], s1 * pv[0][1], s1 * pv[0][2]];
    let v = [s2 * pv[1][0], s2 * pv[1][1], s2 * pv[1][2]];
    Ok([p, v])
}
