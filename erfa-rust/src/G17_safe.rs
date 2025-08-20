// G17
//   g2icrs.c  → eraG2icrs_safe
//   gc2gd.c   → eraGc2gd_safe
//   gc2gde.c  → eraGc2gde_safe
//   gd2gc.c   → eraGd2gc_safe
//   gd2gce.c  → eraGd2gce_safe
//   gmst00.c  → eraGmst00_safe
//   gmst06.c  → eraGmst06_safe
//   gmst82.c  → eraGmst82_safe
//   gst00a.c  → eraGst00a_safe
//   gst00b.c  → eraGst00b_safe
//   gst06.c   → eraGst06_safe
//   gst06a.c  → eraGst06a_safe
//   gst94.c   → eraGst94_safe

use crate::G11_safe::{eraEe00a_safe, eraEe00b_safe, eraEform_safe, eraEors_safe};
use crate::G14_safe::{eraEqeq94_safe, eraEra00_safe};
use crate::G1_safe::{eraAnp_safe, eraAnpm_safe};
use crate::G26_safe::eraPnm06a_safe;
use crate::G29_safe::eraS2c_safe;
use crate::G30_safe::eraS06_safe;
use crate::G33_safe::eraTrxp_safe;
use crate::G6_safe::eraBpn2xy_safe;
use crate::G7_safe::eraC2s_safe;
use crate::H1_safe::{ERFA_DAS2R, ERFA_DAYSEC, ERFA_DJ00, ERFA_DJC, ERFA_DPI, ERFA_DS2R};

pub type ErfaResult<T> = Result<T, ()>;

//----------------------------------------------------------------------
// G17/g2icrs.c → eraG2icrs_safe
//----------------------------------------------------------------------
// Convert Galactic (dl, db) to ICRS (dr, dd), radians.
pub fn eraG2icrs_safe(dl: f64, db: f64) -> ErfaResult<(f64, f64)> {
    // ICRS←Galactic rotation matrix (row-major)
    const R: [[f64; 3]; 3] = [
        [
            -0.054_875_560_416_215_37,
            -0.873_437_090_234_885_0,
            -0.483_835_015_548_713_2,
        ],
        [
            0.494_109_427_875_583_66,
            -0.444_829_629_960_011_16,
            0.746_982_244_497_218_9,
        ],
        [
            -0.867_666_149_019_004_7,
            -0.198_076_373_431_201_53,
            0.455_983_776_175_066_9,
        ],
    ];

    let v1 = eraS2c_safe(dl, db)?;
    let v2 = eraTrxp_safe(&R, &v1)?;
    let (mut dr, mut dd) = eraC2s_safe(&v2)?;
    dr = eraAnp_safe(dr)?;
    dd = eraAnpm_safe(dd)?;
    Ok((dr, dd))
}

//----------------------------------------------------------------------
// G17/gc2gd.c → eraGc2gd_safe
//----------------------------------------------------------------------
// Geocentric XYZ (m) to geodetic using ellipsoid selector n.
pub fn eraGc2gd_safe(n: i32, xyz: &[f64; 3]) -> ErfaResult<(f64, f64, f64, i32)> {
    let ((a, f), j_eform) = eraEform_safe(n)?;
    let mut elong = 0.0_f64;
    let mut phi = 0.0_f64;
    let mut height = 0.0_f64;

    let mut j = j_eform;
    if j == 0 {
        let (el, ph, h, j2) = eraGc2gde_safe(a, f, xyz)?;
        elong = el;
        phi = ph;
        height = h;
        if j2 < 0 {
            j = -2;
        }
    }

    if j < 0 {
        elong = -1e9;
        phi = -1e9;
        height = -1e9;
    }

    Ok((elong, phi, height, j))
}

//----------------------------------------------------------------------
// G17/gc2gde.c → eraGc2gde_safe
//----------------------------------------------------------------------
// Geocentric XYZ (m) to geodetic given ellipsoid (a,f).
pub fn eraGc2gde_safe(a: f64, f: f64, xyz: &[f64; 3]) -> ErfaResult<(f64, f64, f64, i32)> {
    if f < 0.0 || f >= 1.0 {
        return Ok((0.0, 0.0, 0.0, -1));
    }
    if a <= 0.0 {
        return Ok((0.0, 0.0, 0.0, -2));
    }

    let aeps2 = a * a * 1e-32;
    let e2 = (2.0 - f) * f;
    let e4t = e2 * e2 * 1.5;
    let ec2 = 1.0 - e2;
    if ec2 <= 0.0 {
        return Ok((0.0, 0.0, 0.0, -1));
    }
    let ec = ec2.sqrt();
    let b = a * ec;

    let x = xyz[0];
    let y = xyz[1];
    let z = xyz[2];

    let p2 = x * x + y * y;

    let elong = if p2 > 0.0 { y.atan2(x) } else { 0.0 };

    let absz = z.abs();

    let (phi, height) = if p2 > aeps2 {
        let p = p2.sqrt();
        let s0 = absz / a;
        let pn = p / a;
        let zc = ec * s0;

        let c0 = ec * pn;
        let c02 = c0 * c0;
        let c03 = c02 * c0;
        let s02 = s0 * s0;
        let s03 = s02 * s0;
        let a02 = c02 + s02;
        let a0 = a02.sqrt();
        let a03 = a02 * a0;
        let d0 = zc * a03 + e2 * s03;
        let f0 = pn * a03 - e2 * c03;

        let b0 = e4t * s02 * c02 * pn * (a0 - ec);
        let s1 = d0 * f0 - b0 * s0;
        let cc = ec * (f0 * f0 - b0 * c0);

        let mut phi = (s1 / cc).atan();

        let s12 = s1 * s1;
        let cc2 = cc * cc;
        let height = (p * cc + absz * s1 - a * (ec2 * s12 + cc2).sqrt()) / (s12 + cc2).sqrt();

        if z < 0.0 {
            phi = -phi;
        }
        (phi, height)
    } else {
        (ERFA_DPI / 2.0 * if z < 0.0 { -1.0 } else { 1.0 }, absz - b)
    };

    Ok((elong, phi, height, 0))
}

//----------------------------------------------------------------------
// G17/gd2gc.c → eraGd2gc_safe
//----------------------------------------------------------------------
// Geodetic (elong, phi, height) to geocentric XYZ (m) using selector n.
pub fn eraGd2gc_safe(n: i32, elong: f64, phi: f64, height: f64) -> ErfaResult<([f64; 3], i32)> {
    let ((a, f), j_eform) = eraEform_safe(n)?;
    let mut j = j_eform;
    let mut xyz = [0.0_f64; 3];

    if j == 0 {
        let (out, j2) = eraGd2gce_safe(a, f, elong, phi, height)?;
        xyz = out;
        if j2 != 0 {
            j = -2;
        }
    }

    if j != 0 {
        xyz = [0.0, 0.0, 0.0];
    }
    Ok((xyz, j))
}

//----------------------------------------------------------------------
// G17/gd2gce.c → eraGd2gce_safe
//----------------------------------------------------------------------
// Geodetic (elong, phi, height) to geocentric XYZ (m) given (a,f).
pub fn eraGd2gce_safe(
    a: f64,
    f: f64,
    elong: f64,
    phi: f64,
    height: f64,
) -> ErfaResult<([f64; 3], i32)> {
    let sp = phi.sin();
    let cp = phi.cos();
    let mut w = 1.0 - f;
    w *= w;
    let d = cp * cp + w * sp * sp;
    if d <= 0.0 {
        return Ok(([0.0, 0.0, 0.0], -1));
    }

    let ac = a / d.sqrt();
    let as_ = w * ac;

    let r = (ac + height) * cp;
    let x = r * elong.cos();
    let y = r * elong.sin();
    let z = (as_ + height) * sp;

    Ok(([x, y, z], 0))
}

//----------------------------------------------------------------------
// G17/gmst00.c → eraGmst00_safe
//----------------------------------------------------------------------
// GMST (IAU 2000), radians.
pub fn eraGmst00_safe(uta: f64, utb: f64, tta: f64, ttb: f64) -> ErfaResult<f64> {
    let t = ((tta - ERFA_DJ00) + ttb) / ERFA_DJC;
    let gmst = eraAnp_safe(
        eraEra00_safe(uta, utb)?
            + (0.014_506
                + (4612.157_399_66 + (1.396_677_21 + (-0.000_093_44 + 0.000_018_82 * t) * t) * t)
                    * t)
                * ERFA_DAS2R,
    )?;
    Ok(gmst)
}

//----------------------------------------------------------------------
// G17/gmst06.c → eraGmst06_safe
//----------------------------------------------------------------------
// GMST (IAU 2006), radians.
pub fn eraGmst06_safe(uta: f64, utb: f64, tta: f64, ttb: f64) -> ErfaResult<f64> {
    let t = ((tta - ERFA_DJ00) + ttb) / ERFA_DJC;
    let gmst = eraAnp_safe(
        eraEra00_safe(uta, utb)?
            + (0.014_506
                + (4612.156_534
                    + (1.391_581_7
                        + (-0.000_000_44 + (-0.000_029_956 + (-0.000_000_036_8) * t) * t) * t)
                        * t)
                    * t)
                * ERFA_DAS2R,
    )?;
    Ok(gmst)
}

//----------------------------------------------------------------------
// G17/gmst82.c → eraGmst82_safe
//----------------------------------------------------------------------
// GMST (IAU 1982), radians.
pub fn eraGmst82_safe(dj1: f64, dj2: f64) -> ErfaResult<f64> {
    const A: f64 = 24_110.548_41 - ERFA_DAYSEC / 2.0;
    const B: f64 = 8_640_184.812_866;
    const C: f64 = 0.093_104;
    const D: f64 = -6.2e-6;

    let (d1, d2) = if dj1 < dj2 { (dj1, dj2) } else { (dj2, dj1) };
    let t = (d1 + (d2 - ERFA_DJ00)) / ERFA_DJC;

    let f = ERFA_DAYSEC * (d1.fract() + d2.fract());

    let gmst = eraAnp_safe(ERFA_DS2R * (A + (B + (C + D * t) * t) * t + f))?;
    Ok(gmst)
}

//----------------------------------------------------------------------
// G17/gst00a.c → eraGst00a_safe
//----------------------------------------------------------------------
// GAST (IAU 2000A), radians.
pub fn eraGst00a_safe(uta: f64, utb: f64, tta: f64, ttb: f64) -> ErfaResult<f64> {
    let gmst00 = eraGmst00_safe(uta, utb, tta, ttb)?;
    let ee00a = eraEe00a_safe(tta, ttb)?;
    eraAnp_safe(gmst00 + ee00a)
}

//----------------------------------------------------------------------
// G17/gst00b.c → eraGst00b_safe
//----------------------------------------------------------------------
// GAST (IAU 2000B), radians.
pub fn eraGst00b_safe(uta: f64, utb: f64) -> ErfaResult<f64> {
    let gmst00 = eraGmst00_safe(uta, utb, uta, utb)?; // TT from UT
    let ee00b = eraEe00b_safe(uta, utb)?;
    eraAnp_safe(gmst00 + ee00b)
}

//----------------------------------------------------------------------
// G17/gst06.c → eraGst06_safe
//----------------------------------------------------------------------
// GAST (IAU 2006), radians, given NPB matrix.
pub fn eraGst06_safe(
    uta: f64,
    utb: f64,
    tta: f64,
    ttb: f64,
    rnpb: &[[f64; 3]; 3],
) -> ErfaResult<f64> {
    let (x, y) = eraBpn2xy_safe(rnpb)?;
    let s = eraS06_safe(tta, ttb, x, y)?;
    let era = eraEra00_safe(uta, utb)?;
    let eors = eraEors_safe(rnpb, s)?;
    eraAnp_safe(era - eors)
}

//----------------------------------------------------------------------
// G17/gst06a.c → eraGst06a_safe
//----------------------------------------------------------------------
// GAST (IAU 2006/2000A), radians.
pub fn eraGst06a_safe(uta: f64, utb: f64, tta: f64, ttb: f64) -> ErfaResult<f64> {
    let rnpb = eraPnm06a_safe(tta, ttb)?;
    eraGst06_safe(uta, utb, tta, ttb, &rnpb)
}

//----------------------------------------------------------------------
// G17/gst94.c → eraGst94_safe
//----------------------------------------------------------------------
// GAST (IAU 1982/94), radians.
pub fn eraGst94_safe(uta: f64, utb: f64) -> ErfaResult<f64> {
    let gmst82 = eraGmst82_safe(uta, utb)?;
    let eqeq94 = eraEqeq94_safe(uta, utb)?;
    eraAnp_safe(gmst82 + eqeq94)
}
