// G28_safe  Vector/Matrix & Refraction helpers
//   refco.c → eraRefco_safe
//   rm2v.c  → eraRm2v_safe
//   rv2m.c  → eraRv2m_safe
//   rx.c    → eraRx_safe
//   rxp.c   → eraRxp_safe
//   rxpv.c  → eraRxpv_safe
//   rxr.c   → eraRxr_safe
//   ry.c    → eraRy_safe
//   rz.c    → eraRz_safe

pub type ErfaResult<T> = Result<T, ()>;

// Compute refraction coefficients A & B.
pub fn eraRefco_safe(phpa: f64, tc: f64, rh: f64, wl: f64) -> ErfaResult<(f64, f64)> {
    // optical/IR if wl ≤ 100 µm
    let optic = wl <= 100.0;

    // guard inputs, using explicit branching to mirror original macros
    let mut t = if tc > -150.0 { tc } else { -150.0 };
    t = if t < 200.0 { t } else { 200.0 };
    let mut p = if phpa > 0.0 { phpa } else { 0.0 };
    p = if p < 10_000.0 { p } else { 10_000.0 };
    let mut r = if rh > 0.0 { rh } else { 0.0 };
    r = if r < 1.0 { r } else { 1.0 };
    let mut w = if wl > 0.1 { wl } else { 0.1 };
    w = if w < 1.0e6 { w } else { 1.0e6 };

    // water-vapour pressure at observer
    let pw = if p > 0.0 {
        let ps = 10f64.powf((0.7859 + 0.03477 * t) / (1.0 + 0.00412 * t))
            * (1.0 + p * (4.5e-6 + 6e-10 * t * t));
        r * ps / (1.0 - (1.0 - r) * ps / p)
    } else {
        0.0
    };

    // refractivity (n  1) at observer
    let tk = t + 273.15;
    let gamma = if optic {
        let wlsq = w * w;
        (((77.534_84e-6) + (4.391_08e-7 + 3.666e-9 / wlsq) / wlsq) * p - 11.2684e-6 * pw) / tk
    } else {
        (77.6890e-6 * p - (6.3938e-6 - 0.375_463 / tk) * pw) / tk
    };

    // Stones beta, tweaked
    let mut beta = 4.4474e-6 * tk;
    if !optic {
        beta -= 0.0074 * pw * beta;
    }

    // Greens refraction constants
    let refa = gamma * (1.0 - beta);
    let refb = -gamma * (beta - gamma / 2.0);

    Ok((refa, refb))
}

// Rotation matrix → rotation vector.
pub fn eraRm2v_safe(r: &[[f64; 3]; 3]) -> ErfaResult<[f64; 3]> {
    let x = r[1][2] - r[2][1];
    let y = r[2][0] - r[0][2];
    let z = r[0][1] - r[1][0];
    let s2 = (x * x + y * y + z * z).sqrt();

    let w = if s2 > 0.0 {
        let c2 = r[0][0] + r[1][1] + r[2][2] - 1.0;
        let phi = s2.atan2(c2);
        let f = phi / s2;
        [x * f, y * f, z * f]
    } else {
        [0.0, 0.0, 0.0]
    };

    Ok(w)
}

// Rotation vector → rotation matrix.
pub fn eraRv2m_safe(w: &[f64; 3]) -> ErfaResult<[[f64; 3]; 3]> {
    let x0 = w[0];
    let y0 = w[1];
    let z0 = w[2];
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

    let r = [
        [x * x * f + c, x * y * f + z * s, x * z * f - y * s],
        [y * x * f - z * s, y * y * f + c, y * z * f + x * s],
        [z * x * f + y * s, z * y * f - x * s, z * z * f + c],
    ];
    Ok(r)
}

// Rotate matrix about X-axis.
pub fn eraRx_safe(phi: f64, r: &mut [[f64; 3]; 3]) -> ErfaResult<()> {
    let s = phi.sin();
    let c = phi.cos();

    let (r10, r11, r12) = (r[1][0], r[1][1], r[1][2]);
    let (r20, r21, r22) = (r[2][0], r[2][1], r[2][2]);

    let a10 = c * r10 + s * r20;
    let a11 = c * r11 + s * r21;
    let a12 = c * r12 + s * r22;
    let a20 = -s * r10 + c * r20;
    let a21 = -s * r11 + c * r21;
    let a22 = -s * r12 + c * r22;

    r[1][0] = a10;
    r[1][1] = a11;
    r[1][2] = a12;
    r[2][0] = a20;
    r[2][1] = a21;
    r[2][2] = a22;
    Ok(())
}

// Rotate matrix about Y-axis.
pub fn eraRy_safe(theta: f64, r: &mut [[f64; 3]; 3]) -> ErfaResult<()> {
    let s = theta.sin();
    let c = theta.cos();

    let (r00, r01, r02) = (r[0][0], r[0][1], r[0][2]);
    let (r20, r21, r22) = (r[2][0], r[2][1], r[2][2]);

    let a00 = c * r00 - s * r20;
    let a01 = c * r01 - s * r21;
    let a02 = c * r02 - s * r22;
    let a20 = s * r00 + c * r20;
    let a21 = s * r01 + c * r21;
    let a22 = s * r02 + c * r22;

    r[0][0] = a00;
    r[0][1] = a01;
    r[0][2] = a02;
    r[2][0] = a20;
    r[2][1] = a21;
    r[2][2] = a22;
    Ok(())
}

// Rotate matrix about Z-axis.
pub fn eraRz_safe(psi: f64, r: &mut [[f64; 3]; 3]) -> ErfaResult<()> {
    let s = psi.sin();
    let c = psi.cos();

    let (r00, r01, r02) = (r[0][0], r[0][1], r[0][2]);
    let (r10, r11, r12) = (r[1][0], r[1][1], r[1][2]);

    let a00 = c * r00 + s * r10;
    let a01 = c * r01 + s * r11;
    let a02 = c * r02 + s * r12;
    let a10 = -s * r00 + c * r10;
    let a11 = -s * r01 + c * r11;
    let a12 = -s * r02 + c * r12;

    r[0][0] = a00;
    r[0][1] = a01;
    r[0][2] = a02;
    r[1][0] = a10;
    r[1][1] = a11;
    r[1][2] = a12;
    Ok(())
}

// r-matrix × p-vector.
pub fn eraRxp_safe(r: &[[f64; 3]; 3], p: &[f64; 3]) -> ErfaResult<[f64; 3]> {
    let mut rp = [0.0_f64; 3];
    for j in 0..3 {
        let mut w = 0.0;
        for i in 0..3 {
            w += r[j][i] * p[i];
        }
        rp[j] = w;
    }
    Ok(rp)
}

// r-matrix × pv-vector.
pub fn eraRxpv_safe(r: &[[f64; 3]; 3], pv: &[[f64; 3]; 2]) -> ErfaResult<[[f64; 3]; 2]> {
    let p = eraRxp_safe(r, &pv[0])?;
    let v = eraRxp_safe(r, &pv[1])?;
    Ok([p, v])
}

// Multiply two r-matrices.
pub fn eraRxr_safe(a: &[[f64; 3]; 3], b: &[[f64; 3]; 3]) -> ErfaResult<[[f64; 3]; 3]> {
    let mut atb = [[0.0_f64; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            let mut w = 0.0;
            for k in 0..3 {
                w += a[i][k] * b[k][j];
            }
            atb[i][j] = w;
        }
    }
    Ok(atb)
}
