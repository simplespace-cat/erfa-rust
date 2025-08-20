// G16
//   fk425.c  → eraFk425_safe
//   fk45z.c  → eraFk45z_safe
//   fk524.c  → eraFk524_safe
//   fk52h.c  → eraFk52h_safe
//   fk54z.c  → eraFk54z_safe
//   fk5hz.c  → eraFk5hz_safe
//   fw2m.c   → eraFw2m_safe
//   fw2xy.c  → eraFw2xy_safe

use crate::H1_safe::{ERFA_DJ00, ERFA_DJY, ERFA_DR2AS};

use crate::G12_safe::{eraEpb2jd_safe, eraEpj_safe};
use crate::G15_safe::eraFk5hip_safe;
use crate::G1_safe::eraAnp_safe;
use crate::G6_safe::eraBpn2xy_safe;
use crate::G7_safe::eraC2s_safe;

use crate::G19_safe::eraIr_safe;

use crate::G24_safe::{eraPdp_safe, eraPm_safe};
use crate::G25_safe::eraPmp_safe;
use crate::G26_safe::eraPpp_safe;
use crate::G27_safe::{
    eraPv2s_safe, eraPvmpv_safe, eraPvppv_safe, eraPvstar_safe, eraPvu_safe, eraPxp_safe,
};
use crate::G28_safe::{eraRv2m_safe, eraRx_safe, eraRxp_safe, eraRz_safe};
use crate::G29_safe::{eraS2c_safe, eraS2pv_safe};
use crate::G30_safe::eraStarpv_safe;
use crate::G33_safe::eraTrxp_safe;

pub type ErfaResult<T> = Result<T, ()>;

// G16/fk425.c → eraFk425_safe

// FK4 B1950 → FK5 J2000 full conversion.
pub fn eraFk425_safe(
    r1950: f64,
    d1950: f64,
    dr1950: f64,
    dd1950: f64,
    p1950: f64,
    v1950: f64,
) -> ErfaResult<(f64, f64, f64, f64, f64, f64)> {
    const PMF: f64 = 100.0 * ERFA_DR2AS; // rad/yr → "/cy
    const TINY: f64 = 1.0e-30;
    const VF: f64 = 21.095; // km/s → au/trop. century

    // E-terms of aberration: A (position), Adot (velocity)
    const A: [[f64; 3]; 2] = [
        [-1.62557e-6, -0.31919e-6, -0.13843e-6],
        [1.245e-3, -1.580e-3, -0.659e-3],
    ];

    // 3×2 transform EM (Seidelmann 3.591-4)
    const EM: [[[[f64; 3]; 2]; 3]; 2] = [
        [
            [
                [0.999_925_678_2, -0.011_182_061_1, -0.004_857_947_7],
                [
                    0.000_002_423_950_18,
                    -0.000_000_027_106_63,
                    -0.000_000_011_776_56,
                ],
            ],
            [
                [0.011_182_061_0, 0.999_937_478_4, -0.000_027_176_5],
                [
                    0.000_000_027_106_63,
                    0.000_002_423_978_78,
                    -0.000_000_000_065_87,
                ],
            ],
            [
                [0.004_857_947_9, -0.000_027_147_4, 0.999_988_199_7],
                [
                    0.000_000_011_776_56,
                    -0.000_000_000_065_82,
                    0.000_002_424_101_73,
                ],
            ],
        ],
        [
            [
                [-0.000_551, -0.238_565, 0.435_739],
                [0.999_947_04, -0.011_182_51, -0.004_857_67],
            ],
            [
                [0.238_514, -0.002_667, -0.008_541],
                [0.011_182_51, 0.999_958_83, -0.000_027_18],
            ],
            [
                [-0.435_623, 0.012_254, 0.002_117],
                [0.004_857_67, -0.000_027_14, 1.000_009_56],
            ],
        ],
    ];

    // Catalog FK4 → pv
    let ur = dr1950 * PMF;
    let ud = dd1950 * PMF;
    let mut px = p1950;
    let mut rv = v1950;
    let pxvf = px * VF;
    let w_rv = rv * pxvf;

    let r0 = eraS2pv_safe(r1950, d1950, 1.0, ur, ud, w_rv)?;

    // Subtract E-terms
    let mut pv1 = eraPvmpv_safe(&r0, &A)?;

    // Add small corrections proportional to dot(r0.pos, A.pos/Adot)
    let s_pos = eraPdp_safe(&r0[0], &A[0])?;
    let v_pos = eraPdp_safe(&r0[0], &A[1])?;
    let cpos = eraSxp_safe(s_pos, &r0[0])?;
    let cvel = eraSxp_safe(v_pos, &r0[0])?;
    let pv2 = [cpos, cvel];
    pv1 = eraPvppv_safe(&pv1, &pv2)?;

    // FK4 → FK5 frame
    let mut pv2_out = [[0.0_f64; 3]; 2];
    for i in 0..2 {
        for j in 0..3 {
            let mut sum = 0.0_f64;
            for k in 0..2 {
                for l in 0..3 {
                    sum += EM[i][j][k][l] * pv1[k][l];
                }
            }
            pv2_out[i][j] = sum;
        }
    }

    // pv → catalog FK5
    let (r, d, w_scale, ur_out, ud_out, rd) = eraPv2s_safe(&pv2_out)?;
    if px > TINY {
        rv = rd / pxvf;
        px = px / w_scale;
    }

    let r2000 = eraAnp_safe(r)?;
    let d2000 = d;
    let dr2000 = ur_out / PMF;
    let dd2000 = ud_out / PMF;
    let p2000 = px;
    let v2000 = rv;

    Ok((r2000, d2000, dr2000, dd2000, p2000, v2000))
}

// G16/fk45z.c → eraFk45z_safe

// FK4 position at B1950 and epoch → FK5 J2000 position.
pub fn eraFk45z_safe(r1950: f64, d1950: f64, bepoch: f64) -> ErfaResult<(f64, f64)> {
    const PMF: f64 = 100.0 * ERFA_DR2AS;

    const A: [f64; 3] = [-1.62557e-6, -0.31919e-6, -0.13843e-6];
    const AD: [f64; 3] = [1.245e-3, -1.580e-3, -0.659e-3];

    const EM: [[[f64; 3]; 3]; 2] = [
        [
            [0.999_925_678_2, -0.011_182_061_1, -0.004_857_947_7],
            [0.011_182_061_0, 0.999_937_478_4, -0.000_027_176_5],
            [0.004_857_947_9, -0.000_027_147_4, 0.999_988_199_7],
        ],
        [
            [-0.000_551, -0.238_565, 0.435_739],
            [0.238_514, -0.002_667, -0.008_541],
            [-0.435_623, 0.012_254, 0.002_117],
        ],
    ];

    // Unit vector for FK4 coordinates
    let r0 = eraS2c_safe(r1950, d1950)?;

    // p = A + w_epoch * AD
    let w_epoch = (bepoch - 1950.0) / PMF;
    let mut p = eraPpsp_safe(&A, w_epoch, &AD)?;

    // p = p - (r0p) r0
    let dot = eraPdp_safe(&r0, &p)?;
    p = eraPpsp_safe(&p, -dot, &r0)?;

    // p = r0 - p
    p = eraPmp_safe(&r0, &p)?;

    // Transform with EM into position/velocity-like pair
    let mut pv = [[0.0_f64; 3]; 2];
    for i in 0..2 {
        for j in 0..3 {
            let mut sum = 0.0_f64;
            for k in 0..3 {
                sum += EM[i][j][k] * p[k];
            }
            pv[i][j] = sum;
        }
    }

    // Advance from B epoch to J2000 using uniform motion
    let (djm0, djm) = eraEpb2jd_safe(bepoch)?;
    let w_years = (eraEpj_safe(djm0, djm)? - 2000.0) / PMF;
    let pv = eraPvu_safe(w_years, &pv)?;

    // Cartesian → spherical
    let (ra, d2000) = eraC2s_safe(&pv[0])?;
    let r2000 = eraAnp_safe(ra)?;
    Ok((r2000, d2000))
}

// G16/fk524.c → eraFk524_safe

// FK5 J2000 → FK4 B1950 full conversion.
pub fn eraFk524_safe(
    r2000: f64,
    d2000: f64,
    dr2000: f64,
    dd2000: f64,
    p2000: f64,
    v2000: f64,
) -> ErfaResult<(f64, f64, f64, f64, f64, f64)> {
    const PMF: f64 = 100.0 * ERFA_DR2AS;
    const TINY: f64 = 1.0e-30;
    const VF: f64 = 21.095;

    const A: [[f64; 3]; 2] = [
        [-1.62557e-6, -0.31919e-6, -0.13843e-6],
        [1.245e-3, -1.580e-3, -0.659e-3],
    ];

    const EM: [[[[f64; 3]; 2]; 3]; 2] = [
        [
            [
                [0.999_925_679_5, 0.011_181_482_8, 0.004_859_003_9],
                [
                    -0.000_002_423_898_40,
                    -0.000_000_027_105_44,
                    -0.000_000_011_777_42,
                ],
            ],
            [
                [-0.011_181_482_8, 0.999_937_484_9, -0.000_027_177_1],
                [
                    0.000_000_027_105_44,
                    -0.000_002_423_927_02,
                    0.000_000_000_065_85,
                ],
            ],
            [
                [-0.004_859_004_0, -0.000_027_155_7, 0.999_988_194_6],
                [
                    0.000_000_011_777_42,
                    0.000_000_000_065_85,
                    -0.000_002_424_049_95,
                ],
            ],
        ],
        [
            [
                [-0.000_551, 0.238_509, -0.435_614],
                [0.999_904_32, 0.011_181_45, 0.004_858_52],
            ],
            [
                [-0.238_560, -0.002_667, 0.012_254],
                [-0.011_181_45, 0.999_916_13, -0.000_027_17],
            ],
            [
                [0.435_730, -0.008_541, 0.002_117],
                [-0.004_858_52, -0.000_027_16, 0.999_966_84],
            ],
        ],
    ];

    // FK5 catalog → pv
    let pxvf = p2000 * VF;
    let w_rv = v2000 * pxvf;
    let r0 = eraS2pv_safe(r2000, d2000, 1.0, dr2000 * PMF, dd2000 * PMF, w_rv)?;

    // FK5 → FK4 frame
    let mut r1 = [[0.0_f64; 3]; 2];
    for i in 0..2 {
        for j in 0..3 {
            let mut sum = 0.0_f64;
            for k in 0..2 {
                for l in 0..3 {
                    sum += EM[i][j][k][l] * r0[k][l];
                }
            }
            r1[i][j] = sum;
        }
    }

    // Apply one-step E-terms
    // First for position using A[0]
    let len_r1 = eraPm_safe(&r1[0])?;
    let mut p1 = eraSxp_safe(eraPdp_safe(&r1[0], &A[0])?, &r1[0])?;
    let mut p2 = eraSxp_safe(len_r1, &A[0])?;
    p1 = eraPmp_safe(&p2, &p1)?;
    p1 = eraPpp_safe(&r1[0], &p1)?;

    // Recompute length and repeat to refine
    let len_p1 = eraPm_safe(&p1)?;
    p1 = eraSxp_safe(eraPdp_safe(&r1[0], &A[0])?, &r1[0])?;
    p2 = eraSxp_safe(len_p1, &A[0])?;
    p1 = eraPmp_safe(&p2, &p1)?;
    let mut pv = [[0.0_f64; 3]; 2];
    pv[0] = eraPpp_safe(&r1[0], &p1)?;

    // Now velocity using A[1]
    let q1 = eraSxp_safe(eraPdp_safe(&r1[0], &A[1])?, &pv[0])?;
    let q2 = eraSxp_safe(len_p1, &A[1])?;
    let q = eraPmp_safe(&q2, &q1)?;
    pv[1] = eraPpp_safe(&r1[1], &q)?;

    // pv → catalog FK4
    let (ra, dec, w_scale, ur, ud, rd) = eraPv2s_safe(&pv)?;

    let mut px = p2000;
    let mut rv = v2000;
    if px > TINY {
        rv = rd / pxvf;
        px = px / w_scale;
    }

    let r1950 = eraAnp_safe(ra)?;
    let d1950 = dec;
    let dr1950 = ur / PMF;
    let dd1950 = ud / PMF;
    let p1950 = px;
    let v1950 = rv;

    Ok((r1950, d1950, dr1950, dd1950, p1950, v1950))
}

// G16/fk52h.c → eraFk52h_safe

// FK5 J2000 → Hipparcos catalog parameters.
pub fn eraFk52h_safe(
    r5: f64,
    d5: f64,
    dr5: f64,
    dd5: f64,
    px5: f64,
    rv5: f64,
) -> ErfaResult<(f64, f64, f64, f64, f64, f64)> {
    let (pv5, _warn) = eraStarpv_safe(r5, d5, dr5, dd5, px5, rv5)?;

    let (r5h_m, mut s5h) = eraFk5hip_safe()?;

    // Spin in rad/yr → rad/day
    for v in &mut s5h {
        *v /= 365.25;
    }

    // Rotate position to Hipparcos
    let pvh_pos = eraRxp_safe(&r5h_m, &pv5[0])?;

    // Add spin × position to velocity, then rotate
    let wxp = eraPxp_safe(&pv5[0], &s5h)?;
    let vv = eraPpp_safe(&wxp, &pv5[1])?;
    let pvh_vel = eraRxp_safe(&r5h_m, &vv)?;

    let pvh = [pvh_pos, pvh_vel];
    let ((rh, dh, drh, ddh, pxh, rvh), _j) = eraPvstar_safe(&pvh)?;
    Ok((rh, dh, drh, ddh, pxh, rvh))
}

// G16/fk54z.c → eraFk54z_safe

// FK5 J2000 position at epoch → FK4 B1950 position and proper motion.
pub fn eraFk54z_safe(r2000: f64, d2000: f64, bepoch: f64) -> ErfaResult<(f64, f64, f64, f64)> {
    let (r, d, pr, pd, _px, _rv) = eraFk524_safe(r2000, d2000, 0.0, 0.0, 0.0, 0.0)?;

    // Position vector
    let mut p = eraS2c_safe(r, d)?;

    // Proper-motion vector (approximate Cartesian)
    let v = [
        -pr * p[1] - pd * r.cos() * d.sin(),
        pr * p[0] - pd * r.sin() * d.sin(),
        pd * d.cos(),
    ];

    // Advance by epoch offset
    let dt = bepoch - 1950.0;
    for i in 0..3 {
        p[i] += dt * v[i];
    }

    // Back to spherical
    let (ra, d1950) = eraC2s_safe(&p)?;
    let r1950 = eraAnp_safe(ra)?;
    let dr1950 = pr;
    let dd1950 = pd;

    Ok((r1950, d1950, dr1950, dd1950))
}

// G16/fk5hz.c → eraFk5hz_safe

// FK5 J2000 position at date → Hipparcos position.
pub fn eraFk5hz_safe(r5: f64, d5: f64, date1: f64, date2: f64) -> ErfaResult<(f64, f64)> {
    // Time from date to J2000 in Julian centuries (negative for forward date)
    let t = -((date1 - ERFA_DJ00) + date2) / ERFA_DJY;

    let p5e = eraS2c_safe(r5, d5)?;

    let (r5h, s5h) = eraFk5hip_safe()?;

    // Build small rotation from spin×t
    let vst = eraSxp_safe(t, &s5h)?;
    let rst = eraRv2m_safe(&vst)?;

    // Apply spin rotation (transpose product) and then rotate to Hipparcos
    let p5 = eraTrxp_safe(&rst, &p5e)?;
    let ph = eraRxp_safe(&r5h, &p5)?;

    let (ra, dh) = eraC2s_safe(&ph)?;
    let rh = eraAnp_safe(ra)?;
    Ok((rh, dh))
}

// G16/fw2m.c → eraFw2m_safe

// FukushimaWilliams angles → rotation matrix.
pub fn eraFw2m_safe(gamb: f64, phib: f64, psi: f64, eps: f64) -> ErfaResult<[[f64; 3]; 3]> {
    let mut r = [[0.0_f64; 3]; 3];
    eraIr_safe(&mut r)?;
    eraRz_safe(gamb, &mut r)?;
    eraRx_safe(phib, &mut r)?;
    eraRz_safe(-psi, &mut r)?;
    eraRx_safe(-eps, &mut r)?;
    Ok(r)
}

// G16/fw2xy.c → eraFw2xy_safe

// FukushimaWilliams angles → (x, y) CIP coordinates.
pub fn eraFw2xy_safe(gamb: f64, phib: f64, psi: f64, eps: f64) -> ErfaResult<(f64, f64)> {
    let rmat = eraFw2m_safe(gamb, phib, psi, eps)?;
    let (x, y) = eraBpn2xy_safe(&rmat)?;
    Ok((x, y))
}

// Scalar×vector → vector.
#[inline]
fn eraSxp_safe(s: f64, p: &[f64; 3]) -> ErfaResult<[f64; 3]> {
    Ok([s * p[0], s * p[1], s * p[2]])
}

// a + sb → vector.
#[inline]
fn eraPpsp_safe(a: &[f64; 3], s: f64, b: &[f64; 3]) -> ErfaResult<[f64; 3]> {
    Ok([a[0] + s * b[0], a[1] + s * b[1], a[2] + s * b[2]])
}
