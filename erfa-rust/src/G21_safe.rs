// G21
//   moon98.c  → eraMoon98_safe
//   num00a.c  → eraNum00a_safe
//   num00b.c  → eraNum00b_safe
//   num06a.c  → eraNum06a_safe
//   numat.c   → eraNumat_safe

use crate::G19_safe::eraIr_safe;
use crate::G23_safe::{eraNut06a_safe, eraObl06_safe};
use crate::G24_safe::eraPfw06_safe;
use crate::G25_safe::{eraPn00a_safe, eraPn00b_safe};
use crate::G28_safe::{eraRx_safe, eraRxpv_safe, eraRz_safe};
use crate::G29_safe::eraS2pv_safe;
use crate::H1_safe::{ERFA_DAU, ERFA_DD2R, ERFA_DJ00, ERFA_DJC};

#[path = "data/G21_safe/TLR.rs"]
mod tlr_mod;
use tlr_mod::TLR;

#[path = "data/G21_safe/TB.rs"]
mod tb_mod;
use tb_mod::TB;

pub type ErfaResult<T> = Result<T, ()>;

// eraMoon98_safe: Approximate geocentric Moon pv (Meeus 1998), GCRS, au and au/day.
pub fn eraMoon98_safe(date1: f64, date2: f64) -> ErfaResult<[[f64; 3]; 2]> {
    // 1) Fundamental-argument coefficients (degrees).
    const EL0: [f64; 5] = [
        218.316_654_36,
        481_267.881_234_21,
        -0.001_578_6,
        1.0 / 538_841.0,
        -1.0 / 65_194_000.0,
    ];
    const D0: [f64; 5] = [
        297.850_192_1,
        445_267.111_403_4,
        -0.001_881_9,
        1.0 / 545_868.0,
        1.0 / 113_065_000.0,
    ];
    const EM0: [f64; 5] = [
        357.529_109_2,
        35_999.050_290_9,
        -0.000_153_6,
        1.0 / 24_490_000.0,
        0.0,
    ];
    const EMP0: [f64; 5] = [
        134.963_396_4,
        477_198.867_505_5,
        0.008_741_4,
        1.0 / 69_699.0,
        -1.0 / 14_712_000.0,
    ];
    const F0: [f64; 5] = [
        93.272_095_0,
        483_202.017_523_3,
        -0.003_653_9,
        1.0 / 3_526_000.0,
        1.0 / 863_310_000.0,
    ];

    // Meeus additional arguments.
    const A1: (f64, f64) = (119.75, 131.849);
    const A2: (f64, f64) = (53.09, 479_264.290);
    const A3: (f64, f64) = (313.45, 481_266.484);

    // Meeus additive-term coefficients.
    const AL: (f64, f64, f64) = (0.003_958, 0.001_962, 0.000_318);
    const AB: (f64, f64, f64, f64, f64, f64) = (
        -0.002_235, 0.000_382, 0.000_175, 0.000_175, 0.000_127, -0.000_115,
    );

    const R0_METERS: f64 = 385_000_560.0;
    const E_COEF: (f64, f64) = (-0.002_516, -0.000_007_4);

    // 2) Time argument and fundamental angles.
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    #[inline]
    fn poly(coeff: &[f64; 5], t: f64) -> (f64, f64) {
        let w = coeff[0] + (coeff[1] + (coeff[2] + (coeff[3] + coeff[4] * t) * t) * t) * t;
        let dw = coeff[1] + (2.0 * coeff[2] + (3.0 * coeff[3] + 4.0 * coeff[4] * t) * t) * t;
        (ERFA_DD2R * (w.rem_euclid(360.0)), ERFA_DD2R * dw)
    }

    let (elp, delp) = poly(&EL0, t);
    let (d, dd) = poly(&D0, t);
    let (em, dem) = poly(&EM0, t);
    let (emp, demp) = poly(&EMP0, t);
    let (f, df) = poly(&F0, t);

    let a1 = ERFA_DD2R * (A1.0 + A1.1 * t);
    let a2 = ERFA_DD2R * (A2.0 + A2.1 * t);
    let a3 = ERFA_DD2R * (A3.0 + A3.1 * t);
    let da1 = ERFA_DD2R * AL.0;
    let da2 = ERFA_DD2R * A2.1;
    let da3 = ERFA_DD2R * A3.1;

    // E-factor.
    let e = 1.0 + (E_COEF.0 + E_COEF.1 * t) * t;
    let de = E_COEF.0 + 2.0 * E_COEF.1 * t;
    let esq = e * e;
    let desq = 2.0 * e * de;

    // 3) Meeus additive terms.
    let elpmf = elp - f;
    let delpmf = delp - df;
    let mut vel = AL.0 * a1.sin() + AL.1 * elpmf.sin() + AL.2 * a2.sin();
    let mut vdel = AL.0 * da1 * a1.cos() + AL.1 * delpmf * elpmf.cos() + AL.2 * da2 * a2.cos();

    let mut vr = 0.0_f64;
    let mut vdr = 0.0_f64;

    let a1mf = a1 - f;
    let da1mf = da1 - df;
    let a1pf = a1 + f;
    let da1pf = da1 + df;
    let dlpmp = elp - emp;
    let slpmp = elp + emp;

    let vb0 = AB.0 * elp.sin()
        + AB.1 * a3.sin()
        + AB.2 * a1mf.sin()
        + AB.3 * a1pf.sin()
        + AB.4 * dlpmp.sin()
        + AB.5 * slpmp.sin();
    let vdb0 = AB.0 * delp * elp.cos()
        + AB.1 * da3 * a3.cos()
        + AB.2 * da1mf * a1mf.cos()
        + AB.3 * da1pf * a1pf.cos()
        + AB.4 * (delp - demp) * dlpmp.cos()
        + AB.5 * (delp + demp) * slpmp.cos();
    let mut vb = vb0;
    let mut vdb = vdb0;

    // 4) Longitude and distance series.
    for (nd, nem, nemp, nf, coefl, coefr) in TLR.iter().rev() {
        let dn = *nd as f64;
        let emn = *nem as f64;
        let empn = *nemp as f64;
        let fn_ = *nf as f64;

        let (en, den) = match nem.abs() {
            1 => (e, de),
            2 => (esq, desq),
            _ => (1.0, 0.0),
        };

        let arg = dn * d + emn * em + empn * emp + fn_ * f;
        let darg = dn * dd + emn * dem + empn * demp + fn_ * df;

        let v = arg.sin() * en;
        let dv = arg.cos() * darg * en + arg.sin() * den;
        vel += coefl * v;
        vdel += coefl * dv;

        let v = arg.cos() * en;
        let dv = -arg.sin() * darg * en + arg.cos() * den;
        vr += coefr * v;
        vdr += coefr * dv;
    }
    let el = elp + ERFA_DD2R * vel;
    let del = (delp + ERFA_DD2R * vdel) / ERFA_DJC;
    let r = (vr + R0_METERS) / ERFA_DAU;
    let dr = vdr / ERFA_DAU / ERFA_DJC;

    // 5) Latitude series.
    for (nd, nem, nemp, nf, coefb) in TB.iter().rev() {
        let dn = *nd as f64;
        let emn = *nem as f64;
        let empn = *nemp as f64;
        let fn_ = *nf as f64;
        let (en, den) = match nem.abs() {
            1 => (e, de),
            2 => (esq, desq),
            _ => (1.0, 0.0),
        };
        let arg = dn * d + emn * em + empn * emp + fn_ * f;
        let darg = dn * dd + emn * dem + empn * demp + fn_ * df;
        let v = arg.sin() * en;
        let dv = arg.cos() * darg * en + arg.sin() * den;
        vb += coefb * v;
        vdb += coefb * dv;
    }
    let b = vb * ERFA_DD2R;
    let db = vdb * ERFA_DD2R / ERFA_DJC;

    // 6) Spherical to pv (ecliptic of date), then rotate to GCRS.
    let pv_local = eraS2pv_safe(el, b, r, del, db, dr)?; // Meeus, ecliptic of date

    // FukushimaWilliams bias-precession, IAU 2006.
    let (gamb, phib, psib, _epsa) = eraPfw06_safe(date1, date2)?;

    // Build rotation matrix mean ecliptic → GCRS: Rz(psib)  Rx(-phib)  Rz(-gamb).
    let mut rm = [[0.0_f64; 3]; 3];
    eraIr_safe(&mut rm)?;
    eraRz_safe(psib, &mut rm)?;
    eraRx_safe(-phib, &mut rm)?;
    eraRz_safe(-gamb, &mut rm)?;

    // Rotate pv into GCRS.
    let pv_out = eraRxpv_safe(&rm, &pv_local)?;
    Ok(pv_out)
}

// eraNum00a_safe: Nutation matrix for IAU 2000A.
pub fn eraNum00a_safe(date1: f64, date2: f64) -> ErfaResult<[[f64; 3]; 3]> {
    let (_dpsi, _deps, epsa, _rb, _rp, _rbp, _rn, _rbpn) = eraPn00a_safe(date1, date2)?;
    let rmatn = eraNumat_safe(epsa, _dpsi, _deps)?;
    Ok(rmatn)
}

// eraNum00b_safe: Nutation matrix for IAU 2000B.
pub fn eraNum00b_safe(date1: f64, date2: f64) -> ErfaResult<[[f64; 3]; 3]> {
    let (_dpsi, _deps, epsa, _rb, _rp, _rbp, _rn, _rbpn) = eraPn00b_safe(date1, date2)?;
    let rmatn = eraNumat_safe(epsa, _dpsi, _deps)?;
    Ok(rmatn)
}

// eraNum06a_safe: Nutation matrix for IAU 2006/2000A.
pub fn eraNum06a_safe(date1: f64, date2: f64) -> ErfaResult<[[f64; 3]; 3]> {
    let eps = eraObl06_safe(date1, date2)?;
    let (dp, de) = eraNut06a_safe(date1, date2)?;
    let rmatn = eraNumat_safe(eps, dp, de)?;
    Ok(rmatn)
}

// eraNumat_safe: Build nutation matrix from mean obliquity and nutation in longitude/obliquity.
pub fn eraNumat_safe(epsa: f64, dpsi: f64, deps: f64) -> ErfaResult<[[f64; 3]; 3]> {
    let mut r = [[0.0_f64; 3]; 3];
    eraIr_safe(&mut r)?;
    eraRx_safe(epsa, &mut r)?;
    eraRz_safe(-dpsi, &mut r)?;
    eraRx_safe(-(epsa + deps), &mut r)?;
    Ok(r)
}
