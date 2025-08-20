// G24
//   p06e.c   → eraP06e_safe
//   p2pv.c   → eraP2pv_safe
//   p2s.c    → eraP2s_safe
//   pap.c    → eraPap_safe
//   pas.c    → eraPas_safe
//   pb06.c   → eraPb06_safe
//   pdp.c    → eraPdp_safe
//   pfw06.c  → eraPfw06_safe
//   plan94.c → eraPlan94_safe
//   pm.c     → eraPm_safe

use crate::G1_safe::eraAnpm_safe;
use crate::G23_safe::eraObl06_safe;
use crate::G25_safe::{eraPmat06_safe, eraPmp_safe, eraPn_safe};
use crate::G27_safe::eraPxp_safe;
use crate::G28_safe::eraRz_safe;
use crate::G7_safe::eraC2s_safe;
use crate::H1_safe::{ERFA_D2PI, ERFA_DAS2R, ERFA_DJ00, ERFA_DJC, ERFA_DJM};

#[path = "data/G24_safe/plan94_tables.rs"]
mod plan94_tables;
use plan94_tables::{
    ASCENDING_NODE as OMEGA, ECCENTRICITY as E, INCLINATION as DINC, MEAN_LONGITUDE as DLM,
    PERIHELION as PI_, PLANET_MASSES as AMAS, SEMI_MAJOR_AXIS as A, TRIG_ARG_LONGITUDE as KQ,
    TRIG_ARG_PERIHELION as KP, TRIG_COEFF_A_COS as CA, TRIG_COEFF_A_SIN as SA,
    TRIG_COEFF_L_COS as CL, TRIG_COEFF_L_SIN as SL,
};

pub type ErfaResult<T> = Result<T, ()>;

// eraP06e_safe: IAU 2006 equinox-based precession parameter set.
pub fn eraP06e_safe(
    date1: f64,
    date2: f64,
) -> ErfaResult<(
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
)> {
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    let eps0 = 84381.406 * ERFA_DAS2R;

    let psia = (5038.481_507
        + (-1.079_006_9 + (-0.001_140_45 + (0.000_132_851 + (-0.000_000_0951) * t) * t) * t) * t)
        * t
        * ERFA_DAS2R;

    let oma = eps0
        + ((-0.025_754
            + (0.051_262_3 + (-0.007_725_03 + (-0.000_000_467 + 0.000_000_3337 * t) * t) * t) * t)
            * t)
            * ERFA_DAS2R;

    let bpa = (4.199_094
        + (0.193_987_3 + (-0.000_224_66 + (-0.000_000_912 + 0.000_000_0120 * t) * t) * t) * t)
        * t
        * ERFA_DAS2R;

    let bqa = (-46.811_015
        + (0.051_028_3 + (0.000_524_13 + (-0.000_000_646 + (-0.000_000_0172) * t) * t) * t) * t)
        * t
        * ERFA_DAS2R;

    let pia = (46.998_973
        + (-0.033_492_6 + (-0.000_125_59 + (0.000_000_113 + (-0.000_000_0022) * t) * t) * t) * t)
        * t
        * ERFA_DAS2R;

    let bpia = (629_546.7936
        + (-867.95758
            + (0.157_992 + (-0.000_5371 + (-0.000_047_97 + 0.000_000_072 * t) * t) * t) * t)
            * t)
        * ERFA_DAS2R;

    let epsa = eraObl06_safe(date1, date2)?;

    let chia = (10.556_403
        + (-2.381_429_2 + (-0.001_211_97 + (0.000_170_663 + (-0.000_000_0560) * t) * t) * t) * t)
        * t
        * ERFA_DAS2R;

    let za = (-2.650_545
        + (2306.077_181
            + (1.092_734_8 + (0.018_268_37 + (-0.000_028_596 + (-0.000_000_2904) * t) * t) * t)
                * t)
            * t)
        * ERFA_DAS2R;

    let zetaa = (2.650_545
        + (2306.083_227
            + (0.298_849_9 + (0.018_018_28 + (-0.000_005_971 + (-0.000_000_3173) * t) * t) * t)
                * t)
            * t)
        * ERFA_DAS2R;

    let thetaa = (2004.191_903
        + (-0.429_493_4 + (-0.041_822_64 + (-0.000_007_089 + (-0.000_000_1274) * t) * t) * t) * t)
        * t
        * ERFA_DAS2R;

    let pa = (5028.796_195
        + (1.105_434_8 + (0.000_079_64 + (-0.000_023_857 + (-0.000_000_0383) * t) * t) * t) * t)
        * t
        * ERFA_DAS2R;

    let gam = (10.556_403
        + (0.493_204_4 + (-0.000_312_38 + (-0.000_002_788 + 0.000_000_0260 * t) * t) * t) * t)
        * t
        * ERFA_DAS2R;

    let phi = eps0
        + ((-46.811_015
            + (0.051_126_9 + (0.000_532_89 + (-0.000_000_440 + (-0.000_000_0176) * t) * t) * t)
                * t)
            * t)
            * ERFA_DAS2R;

    let psi = (5038.481_507
        + (1.558_417_6 + (-0.000_185_22 + (-0.000_026_452 + (-0.000_000_0148) * t) * t) * t) * t)
        * t
        * ERFA_DAS2R;

    Ok((
        eps0, psia, oma, bpa, bqa, pia, bpia, epsa, chia, za, zetaa, thetaa, pa, gam, phi, psi,
    ))
}

// eraP2pv_safe: Extend p-vector to pv-vector with zero velocity.
pub fn eraP2pv_safe(p: &[f64; 3]) -> ErfaResult<[[f64; 3]; 2]> {
    let mut pv = [[0.0_f64; 3]; 2];
    pv[0] = *p;
    Ok(pv)
}

// eraP2s_safe: Convert p-vector to spherical coordinates (theta, phi, r).
pub fn eraP2s_safe(p: &[f64; 3]) -> ErfaResult<(f64, f64, f64)> {
    let (theta, phi) = eraC2s_safe(p)?;
    let r = eraPm_safe(p)?;
    Ok((theta, phi, r))
}

// eraPap_safe: Position angle from two p-vectors (radians).
pub fn eraPap_safe(a: &[f64; 3], b: &[f64; 3]) -> ErfaResult<f64> {
    let (am, au) = eraPn_safe(a)?;
    let bm = eraPm_safe(b)?;
    let (st, ct) = if am == 0.0 || bm == 0.0 {
        (0.0, 1.0)
    } else {
        let xa = a[0];
        let ya = a[1];
        let za = a[2];
        let eta = [-xa * za, -ya * za, xa * xa + ya * ya];
        let xi = eraPxp_safe(&eta, &au)?;
        let a2b = eraPmp_safe(b, a)?;
        let st_val = eraPdp_safe(&a2b, &xi)?;
        let mut ct_val = eraPdp_safe(&a2b, &eta)?;
        if st_val == 0.0 && ct_val == 0.0 {
            ct_val = 1.0;
        }
        (st_val, ct_val)
    };
    Ok(st.atan2(ct))
}

// eraPas_safe: Position angle from two spherical positions.
pub fn eraPas_safe(al: f64, ap: f64, bl: f64, bp: f64) -> ErfaResult<f64> {
    let dl = bl - al;
    let y = dl.sin() * bp.cos();
    let x = bp.sin() * ap.cos() - bp.cos() * ap.sin() * dl.cos();
    Ok(if x != 0.0 || y != 0.0 {
        y.atan2(x)
    } else {
        0.0
    })
}

// eraPb06_safe: Precession-bias Euler angles (bzeta, bz, btheta), IAU 2006.
pub fn eraPb06_safe(date1: f64, date2: f64) -> ErfaResult<(f64, f64, f64)> {
    let r = eraPmat06_safe(date1, date2)?;

    let mut y = r[1][2];
    let mut x = -r[0][2];
    let bz = if x < 0.0 {
        y = -y;
        x = -x;
        if x != 0.0 || y != 0.0 {
            -y.atan2(x)
        } else {
            0.0
        }
    } else if x != 0.0 || y != 0.0 {
        -y.atan2(x)
    } else {
        0.0
    };

    let mut r2 = r;
    eraRz_safe(bz, &mut r2)?;

    y = r2[0][2];
    x = r2[2][2];
    let btheta = if x != 0.0 || y != 0.0 {
        -y.atan2(x)
    } else {
        0.0
    };

    y = -r2[1][0];
    x = r2[1][1];
    let bzeta = if x != 0.0 || y != 0.0 {
        -y.atan2(x)
    } else {
        0.0
    };

    Ok((bzeta, bz, btheta))
}

// eraPdp_safe: Dot product of two p-vectors.
pub fn eraPdp_safe(a: &[f64; 3], b: &[f64; 3]) -> ErfaResult<f64> {
    Ok(a[0] * b[0] + a[1] * b[1] + a[2] * b[2])
}

// eraPfw06_safe: FukushimaWilliams angles (gamb, phib, psib, epsa), IAU 2006.
pub fn eraPfw06_safe(date1: f64, date2: f64) -> ErfaResult<(f64, f64, f64, f64)> {
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    let gamb = (-0.052_928
        + (10.556_378
            + (0.493_204_4 + (-0.000_312_38 + (-0.000_002_788 + 0.000_000_0260 * t) * t) * t) * t)
            * t)
        * ERFA_DAS2R;

    let phib = (84_381.412_819
        + (-46.811_016
            + (0.051_126_8 + (0.000_532_89 + (-0.000_000_440 + (-0.000_000_0176) * t) * t) * t)
                * t)
            * t)
        * ERFA_DAS2R;

    let psib = (-0.041_775
        + (5038.481_484
            + (1.558_417_5 + (-0.000_185_22 + (-0.000_026_452 + (-0.000_000_0148) * t) * t) * t)
                * t)
            * t)
        * ERFA_DAS2R;

    let epsa = eraObl06_safe(date1, date2)?;

    Ok((gamb, phib, psib, epsa))
}

// eraPm_safe: Modulus of a 3-vector.
pub fn eraPm_safe(p: &[f64; 3]) -> ErfaResult<f64> {
    Ok((p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt())
}

// eraPlan94_safe: Approximate heliocentric position/velocity for planet np (1..8).
pub fn eraPlan94_safe(date1: f64, date2: f64, np_in: i32) -> ErfaResult<([[f64; 3]; 2], i32)> {
    const GK: f64 = 0.017_202_098_950; // Gaussian gravitational constant (au^3/d^2).
    const SINEPS: f64 = 0.397_777_155_931_913_7; // J2000 mean obliquity (IAU 1976).
    const COSEPS: f64 = 0.917_482_062_069_181_8;
    const KMAX: i32 = 10;

    if np_in < 1 || np_in > 8 {
        return Ok(([[0.0; 3]; 2], -1));
    }
    let np = (np_in - 1) as usize;

    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJM;
    let mut jstat = if t.abs() <= 1.0 { 0 } else { 1 };

    let mut da = A[np][0] + (A[np][1] + A[np][2] * t) * t;
    let mut dl = (3600.0 * DLM[np][0] + (DLM[np][1] + DLM[np][2] * t) * t) * ERFA_DAS2R;
    let de = E[np][0] + (E[np][1] + E[np][2] * t) * t;
    let dp = eraAnpm_safe((3600.0 * PI_[np][0] + (PI_[np][1] + PI_[np][2] * t) * t) * ERFA_DAS2R)?;
    let di = (3600.0 * DINC[np][0] + (DINC[np][1] + DINC[np][2] * t) * t) * ERFA_DAS2R;
    let dom =
        eraAnpm_safe((3600.0 * OMEGA[np][0] + (OMEGA[np][1] + OMEGA[np][2] * t) * t) * ERFA_DAS2R)?;

    let dmu = 0.359_536_20 * t;
    for k in 0..8 {
        let arga = KP[np][k] as f64 * dmu;
        let argl = KQ[np][k] as f64 * dmu;
        da += (CA[np][k] as f64 * arga.cos() + SA[np][k] as f64 * arga.sin()) * 1e-7;
        dl += (CL[np][k] as f64 * argl.cos() + SL[np][k] as f64 * argl.sin()) * 1e-7;
    }
    let arga = KP[np][8] as f64 * dmu;
    da += t * (CA[np][8] as f64 * arga.cos() + SA[np][8] as f64 * arga.sin()) * 1e-7;
    for k in 8..10 {
        let argl = KQ[np][k] as f64 * dmu;
        dl += t * (CL[np][k] as f64 * argl.cos() + SL[np][k] as f64 * argl.sin()) * 1e-7;
    }
    dl = dl.rem_euclid(ERFA_D2PI);

    let am = dl - dp;
    let mut ae = am + de * am.sin();
    let mut k_iter = 0;
    loop {
        let dae = (am - ae + de * ae.sin()) / (1.0 - de * ae.cos());
        ae += dae;
        k_iter += 1;
        if k_iter >= KMAX || dae.abs() <= 1e-12 {
            break;
        }
    }
    if k_iter >= KMAX {
        jstat = 2;
    }

    let ae2 = 0.5 * ae;
    let at = 2.0 * ((((1.0 + de) / (1.0 - de)).sqrt()) * ae2.sin()).atan2(ae2.cos());

    let r = da * (1.0 - de * ae.cos());
    let v = GK * ((1.0 + 1.0 / AMAS[np]) / (da * da * da)).sqrt();

    let si2 = (0.5 * di).sin();
    let xq = si2 * dom.cos();
    let xp = si2 * dom.sin();
    let tl = at + dp;
    let (xsw, xcw) = tl.sin_cos();
    let xm2 = 2.0 * (xp * xcw - xq * xsw);
    let xf = da / (1.0 - de * de).sqrt();
    let ci2 = (0.5 * di).cos();
    let xms = (de * dp.sin() + xsw) * xf;
    let xmc = (de * dp.cos() + xcw) * xf;
    let xpxq2 = 2.0 * xp * xq;

    let x = r * (xcw - xm2 * xp);
    let y = r * (xsw + xm2 * xq);
    let z = r * (-xm2 * ci2);

    let mut pv = [[0.0_f64; 3]; 2];
    pv[0][0] = x;
    pv[0][1] = y * COSEPS - z * SINEPS;
    pv[0][2] = y * SINEPS + z * COSEPS;

    let xdot = v * ((-1.0 + 2.0 * xp * xp) * xms + xpxq2 * xmc);
    let ydot = v * ((1.0 - 2.0 * xq * xq) * xmc - xpxq2 * xms);
    let zdot = v * (2.0 * ci2 * (xp * xms + xq * xmc));

    pv[1][0] = xdot;
    pv[1][1] = ydot * COSEPS - zdot * SINEPS;
    pv[1][2] = ydot * SINEPS + zdot * COSEPS;

    Ok((pv, jstat))
}
