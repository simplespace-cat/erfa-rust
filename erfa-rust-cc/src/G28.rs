// G28  Vector/Matrix & Refraction helpers
//   refco.c → eraRefco
//   rm2v.c  → eraRm2v
//   rv2m.c  → eraRv2m
//   rx.c    → eraRx
//   rxp.c   → eraRxp
//   rxpv.c  → eraRxpv
//   rxr.c   → eraRxr
//   ry.c    → eraRy
//   rz.c    → eraRz

use crate::H1::*;

pub unsafe fn eraRefco(phpa: f64, tc: f64, rh: f64, wl: f64, refa: *mut f64, refb: *mut f64) {
    let optic = wl <= 100.0;

    let mut t = ERFA_GMAX(tc, -150.0);
    t = ERFA_GMIN(t, 200.0);
    let mut p = ERFA_GMAX(phpa, 0.0);
    p = ERFA_GMIN(p, 10_000.0);
    let mut r = ERFA_GMAX(rh, 0.0);
    r = ERFA_GMIN(r, 1.0);
    let mut w = ERFA_GMAX(wl, 0.1);
    w = ERFA_GMIN(w, 1.0e6);

    let pw = if p > 0.0 {
        let ps = 10f64.powf((0.7859 + 0.03477 * t) / (1.0 + 0.00412 * t))
            * (1.0 + p * (4.5e-6 + 6e-10 * t * t));
        r * ps / (1.0 - (1.0 - r) * ps / p)
    } else {
        0.0
    };

    let tk = t + 273.15;
    let gamma = if optic {
        let wlsq = w * w;
        (((77.534_84e-6) + (4.391_08e-7 + 3.666e-9 / wlsq) / wlsq) * p - 11.2684e-6 * pw) / tk
    } else {
        (77.6890e-6 * p - (6.3938e-6 - 0.375_463 / tk) * pw) / tk
    };

    let mut beta = 4.4474e-6 * tk;
    if !optic {
        beta -= 0.0074 * pw * beta;
    }

    *refa = gamma * (1.0 - beta);
    *refb = -gamma * (beta - gamma / 2.0);
}

#[inline(always)]
unsafe fn m3(r: *mut f64, row: usize, col: usize) -> *mut f64 {
    r.add(row * 3 + col)
}
#[inline(always)]
unsafe fn v3(v: *mut f64, idx: usize) -> *mut f64 {
    v.add(idx)
}

pub unsafe fn eraRm2v(r: *mut f64, w: *mut f64) {
    let x = *m3(r, 1, 2) - *m3(r, 2, 1);
    let y = *m3(r, 2, 0) - *m3(r, 0, 2);
    let z = *m3(r, 0, 1) - *m3(r, 1, 0);
    let s2 = (x * x + y * y + z * z).sqrt();

    if s2 > 0.0 {
        let c2 = *m3(r, 0, 0) + *m3(r, 1, 1) + *m3(r, 2, 2) - 1.0;
        let phi = s2.atan2(c2);
        let f = phi / s2;
        *v3(w, 0) = x * f;
        *v3(w, 1) = y * f;
        *v3(w, 2) = z * f;
    } else {
        *v3(w, 0) = 0.0;
        *v3(w, 1) = 0.0;
        *v3(w, 2) = 0.0;
    }
}

pub unsafe fn eraRv2m(w: *mut f64, r: *mut f64) {
    let x0 = *v3(w, 0);
    let y0 = *v3(w, 1);
    let z0 = *v3(w, 2);
    let phi = (x0 * x0 + y0 * y0 + z0 * z0).sqrt();
    let s = phi.sin();
    let c = phi.cos();
    let f = 1.0 - c;

    let (mut x, mut y, mut z) = (x0, y0, z0);
    if phi > 0.0 {
        x /= phi;
        y /= phi;
        z /= phi;
    }

    *m3(r, 0, 0) = x * x * f + c;
    *m3(r, 0, 1) = x * y * f + z * s;
    *m3(r, 0, 2) = x * z * f - y * s;
    *m3(r, 1, 0) = y * x * f - z * s;
    *m3(r, 1, 1) = y * y * f + c;
    *m3(r, 1, 2) = y * z * f + x * s;
    *m3(r, 2, 0) = z * x * f + y * s;
    *m3(r, 2, 1) = z * y * f - x * s;
    *m3(r, 2, 2) = z * z * f + c;
}

pub unsafe fn eraRx(phi: f64, r: *mut f64) {
    let s = phi.sin();
    let c = phi.cos();

    let a10 = c * *m3(r, 1, 0) + s * *m3(r, 2, 0);
    let a11 = c * *m3(r, 1, 1) + s * *m3(r, 2, 1);
    let a12 = c * *m3(r, 1, 2) + s * *m3(r, 2, 2);
    let a20 = -s * *m3(r, 1, 0) + c * *m3(r, 2, 0);
    let a21 = -s * *m3(r, 1, 1) + c * *m3(r, 2, 1);
    let a22 = -s * *m3(r, 1, 2) + c * *m3(r, 2, 2);

    *m3(r, 1, 0) = a10;
    *m3(r, 1, 1) = a11;
    *m3(r, 1, 2) = a12;
    *m3(r, 2, 0) = a20;
    *m3(r, 2, 1) = a21;
    *m3(r, 2, 2) = a22;
}

pub unsafe fn eraRy(theta: f64, r: *mut f64) {
    let s = theta.sin();
    let c = theta.cos();

    let a00 = c * *m3(r, 0, 0) - s * *m3(r, 2, 0);
    let a01 = c * *m3(r, 0, 1) - s * *m3(r, 2, 1);
    let a02 = c * *m3(r, 0, 2) - s * *m3(r, 2, 2);
    let a20 = s * *m3(r, 0, 0) + c * *m3(r, 2, 0);
    let a21 = s * *m3(r, 0, 1) + c * *m3(r, 2, 1);
    let a22 = s * *m3(r, 0, 2) + c * *m3(r, 2, 2);

    *m3(r, 0, 0) = a00;
    *m3(r, 0, 1) = a01;
    *m3(r, 0, 2) = a02;
    *m3(r, 2, 0) = a20;
    *m3(r, 2, 1) = a21;
    *m3(r, 2, 2) = a22;
}

pub unsafe fn eraRz(psi: f64, r: *mut f64) {
    let s = psi.sin();
    let c = psi.cos();

    let a00 = c * *m3(r, 0, 0) + s * *m3(r, 1, 0);
    let a01 = c * *m3(r, 0, 1) + s * *m3(r, 1, 1);
    let a02 = c * *m3(r, 0, 2) + s * *m3(r, 1, 2);
    let a10 = -s * *m3(r, 0, 0) + c * *m3(r, 1, 0);
    let a11 = -s * *m3(r, 0, 1) + c * *m3(r, 1, 1);
    let a12 = -s * *m3(r, 0, 2) + c * *m3(r, 1, 2);

    *m3(r, 0, 0) = a00;
    *m3(r, 0, 1) = a01;
    *m3(r, 0, 2) = a02;
    *m3(r, 1, 0) = a10;
    *m3(r, 1, 1) = a11;
    *m3(r, 1, 2) = a12;
}

pub unsafe fn eraRxp(r: *mut f64, p: *mut f64, rp: *mut f64) {
    let mut wrp = [0.0_f64; 3];

    for j in 0..3 {
        let mut w = 0.0;
        for i in 0..3 {
            w += *m3(r, j, i) * *v3(p, i);
        }
        wrp[j] = w;
    }

    for i in 0..3 {
        *v3(rp, i) = wrp[i];
    }
}

pub unsafe fn eraRxpv(r: *mut f64, pv: *mut f64, rpv: *mut f64) {
    eraRxp(r, pv, rpv);
    eraRxp(r, pv.add(3), rpv.add(3));
}

pub unsafe fn eraRxr(a: *mut f64, b: *mut f64, atb: *mut f64) {
    let mut wm = [0.0_f64; 9];
    for i in 0..3 {
        for j in 0..3 {
            let mut w = 0.0;
            for k in 0..3 {
                w += *m3(a, i, k) * *m3(b, k, j);
            }
            wm[i * 3 + j] = w;
        }
    }
    for idx in 0..9 {
        *atb.add(idx) = wm[idx];
    }
}
