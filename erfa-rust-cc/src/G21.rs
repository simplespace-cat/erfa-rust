// G21
//   moon98.c  → eraMoon98
//   num00a.c  → eraNum00a
//   num00b.c  → eraNum00b
//   num06a.c  → eraNum06a
//   numat.c   → eraNumat

use crate::H1::*;

// eraMoon98   Approximate geocentric Moon position & velocity (Meeus 98)
pub unsafe fn eraMoon98(date1: f64, date2: f64, pv: *mut f64) {
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

    const A1: (f64, f64) = (119.75, 131.849);
    const A2: (f64, f64) = (53.09, 479_264.290);
    const A3: (f64, f64) = (313.45, 481_266.484);

    const AL: (f64, f64, f64) = (0.003_958, 0.001_962, 0.000_318);
    const AB: (f64, f64, f64, f64, f64, f64) = (
        -0.002_235, 0.000_382, 0.000_175, 0.000_175, 0.000_127, -0.000_115,
    );

    const R0_METERS: f64 = 385_000_560.0;
    const E_COEF: (f64, f64) = (-0.002_516, -0.000_007_4);

    #[rustfmt::skip]
    const TLR: &[(i8,i8,i8,i8,f64,f64)] = &[
        (0,  0,  1,  0,  6.288774, -20905355.0),
        (2,  0, -1,  0,  1.274027,  -3699111.0),
        (2,  0,  0,  0,  0.658314,  -2955968.0),
        (0,  0,  2,  0,  0.213618,   -569925.0),
        (0,  1,  0,  0, -0.185116,     48888.0),
        (0,  0,  0,  2, -0.114332,     -3149.0),
        (2,  0, -2,  0,  0.058793,    246158.0),
        (2, -1, -1,  0,  0.057066,   -152138.0),
        (2,  0,  1,  0,  0.053322,   -170733.0),
        (2, -1,  0,  0,  0.045758,   -204586.0),
        (0,  1, -1,  0, -0.040923,   -129620.0),
        (1,  0,  0,  0, -0.034720,    108743.0),
        (0,  1,  1,  0, -0.030383,    104755.0),
        (2,  0,  0, -2,  0.015327,     10321.0),
        (0,  0,  1,  2, -0.012528,         0.0),
        (0,  0,  1, -2,  0.010980,     79661.0),
        (4,  0, -1,  0,  0.010675,    -34782.0),
        (0,  0,  3,  0,  0.010034,    -23210.0),
        (4,  0, -2,  0,  0.008548,    -21636.0),
        (2,  1, -1,  0, -0.007888,     24208.0),
        (2,  1,  0,  0, -0.006766,     30824.0),
        (1,  0, -1,  0, -0.005163,     -8379.0),
        (1,  1,  0,  0,  0.004987,    -16675.0),
        (2, -1,  1,  0,  0.004036,    -12831.0),
        (2,  0,  2,  0,  0.003994,    -10445.0),
        (4,  0,  0,  0,  0.003861,    -11650.0),
        (2,  0, -3,  0,  0.003665,     14403.0),
        (0,  1, -2,  0, -0.002689,     -7003.0),
        (2,  0, -1,  2, -0.002602,         0.0),
        (2, -1, -2,  0,  0.002390,     10056.0),
        (1,  0,  1,  0, -0.002348,      6322.0),
        (2, -2,  0,  0,  0.002236,     -9884.0),
        (0,  1,  2,  0, -0.002120,      5751.0),
        (0,  2,  0,  0, -0.002069,         0.0),
        (2, -2, -1,  0,  0.002048,     -4950.0),
        (2,  0,  1, -2, -0.001773,      4130.0),
        (2,  0,  0,  2, -0.001595,         0.0),
        (4, -1, -1,  0,  0.001215,     -3958.0),
        (0,  0,  2,  2, -0.001110,         0.0),
        (3,  0, -1,  0, -0.000892,      3258.0),
        (2,  1,  1,  0, -0.000810,      2616.0),
        (4, -1, -2,  0,  0.000759,     -1897.0),
        (0,  2, -1,  0, -0.000713,     -2117.0),
        (2,  2, -1,  0, -0.000700,      2354.0),
        (2,  1, -2,  0,  0.000691,         0.0),
        (2, -1,  0, -2,  0.000596,         0.0),
        (4,  0,  1,  0,  0.000549,     -1423.0),
        (0,  0,  4,  0,  0.000537,     -1117.0),
        (4, -1,  0,  0,  0.000520,     -1571.0),
        (1,  0, -2,  0, -0.000487,     -1739.0),
        (2,  1,  0, -2, -0.000399,         0.0),
        (0,  0,  2, -2, -0.000381,     -4421.0),
        (1,  1,  1,  0,  0.000351,         0.0),
        (3,  0, -2,  0, -0.000340,         0.0),
        (4,  0, -3,  0,  0.000330,         0.0),
        (2, -1,  2,  0,  0.000327,         0.0),
        (0,  2,  1,  0, -0.000323,      1165.0),
        (1,  1, -1,  0,  0.000299,         0.0),
        (2,  0,  3,  0,  0.000294,         0.0),
        (2,  0, -1, -2,  0.000000,      8752.0),
    ];
    #[rustfmt::skip]
    const TB : &[(i8,i8,i8,i8,f64)] = &[
        (0,  0,  0,  1,  5.128122),
        (0,  0,  1,  1,  0.280602),
        (0,  0,  1, -1,  0.277693),
        (2,  0,  0, -1,  0.173237),
        (2,  0, -1,  1,  0.055413),
        (2,  0, -1, -1,  0.046271),
        (2,  0,  0,  1,  0.032573),
        (0,  0,  2,  1,  0.017198),
        (2,  0,  1, -1,  0.009266),
        (0,  0,  2, -1,  0.008822),
        (2, -1,  0, -1,  0.008216),
        (2,  0, -2, -1,  0.004324),
        (2,  0,  1,  1,  0.004200),
        (2,  1,  0, -1, -0.003359),
        (2, -1, -1,  1,  0.002463),
        (2, -1,  0,  1,  0.002211),
        (2, -1, -1, -1,  0.002065),
        (0,  1, -1, -1, -0.001870),
        (4,  0, -1, -1,  0.001828),
        (0,  1,  0,  1, -0.001794),
        (0,  0,  0,  3, -0.001749),
        (0,  1, -1,  1, -0.001565),
        (1,  0,  0,  1, -0.001491),
        (0,  1,  1,  1, -0.001475),
        (0,  1,  1, -1, -0.001410),
        (0,  1,  0, -1, -0.001344),
        (1,  0,  0, -1, -0.001335),
        (0,  0,  3,  1,  0.001107),
        (4,  0,  0, -1,  0.001021),
        (4,  0, -1,  1,  0.000833),
        (0,  0,  1, -3,  0.000777),
        (4,  0, -2,  1,  0.000671),
        (2,  0,  0, -3,  0.000607),
        (2,  0,  2, -1,  0.000596),
        (2, -1,  1, -1,  0.000491),
        (2,  0, -2,  1, -0.000451),
        (0,  0,  3, -1,  0.000439),
        (2,  0,  2,  1,  0.000422),
        (2,  0, -3, -1,  0.000421),
        (2,  1, -1,  1, -0.000366),
        (2,  1,  0,  1, -0.000351),
        (4,  0,  0,  1,  0.000331),
        (2, -1,  1,  1,  0.000315),
        (2, -2,  0, -1,  0.000302),
        (0,  0,  1,  3, -0.000283),
        (2,  1,  1, -1, -0.000229),
        (1,  1,  0, -1,  0.000223),
        (1,  1,  0,  1,  0.000223),
        (0,  1, -2, -1, -0.000220),
        (2,  1, -1, -1, -0.000220),
        (1,  0,  1,  1, -0.000185),
        (2, -1, -2, -1,  0.000181),
        (0,  1,  2,  1, -0.000177),
        (4,  0, -2, -1,  0.000176),
        (4, -1, -1, -1,  0.000166),
        (1,  0,  1, -1, -0.000164),
        (4,  0,  1, -1,  0.000132),
        (1,  0, -1, -1, -0.000119),
        (4, -1,  0, -1,  0.000115),
        (2, -2,  0,  1,  0.000107),
    ];

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

    let e = 1.0 + (E_COEF.0 + E_COEF.1 * t) * t;
    let de = E_COEF.0 + 2.0 * E_COEF.1 * t;
    let esq = e * e;
    let desq = 2.0 * e * de;

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

    let vb = AB.0 * elp.sin()
        + AB.1 * a3.sin()
        + AB.2 * a1mf.sin()
        + AB.3 * a1pf.sin()
        + AB.4 * dlpmp.sin()
        + AB.5 * slpmp.sin();
    let vdb = AB.0 * delp * elp.cos()
        + AB.1 * da3 * a3.cos()
        + AB.2 * da1mf * a1mf.cos()
        + AB.3 * da1pf * a1pf.cos()
        + AB.4 * (delp - demp) * dlpmp.cos()
        + AB.5 * (delp + demp) * slpmp.cos();
    let mut vb = vb;
    let mut vdb = vdb;

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

    let mut pv_local = [0.0_f64; 6];
    eraS2pv(el, b, r, del, db, dr, pv_local.as_mut_ptr());

    let mut gamb = 0.0;
    let mut phib = 0.0;
    let mut psib = 0.0;
    let mut epsa = 0.0;
    eraPfw06(date1, date2, &mut gamb, &mut phib, &mut psib, &mut epsa);

    let mut rm = [0.0_f64; 9];
    eraIr(rm.as_mut_ptr());
    eraRz(psib, rm.as_mut_ptr());
    eraRx(-phib, rm.as_mut_ptr());
    eraRz(-gamb, rm.as_mut_ptr());

    eraRxpv(rm.as_mut_ptr(), pv_local.as_mut_ptr(), pv);
}

// eraNum00a  Nutation matrix, IAU 2000A
pub unsafe fn eraNum00a(date1: f64, date2: f64, rmatn: *mut f64) {
    let mut dpsi = 0.0;
    let mut deps = 0.0;
    let mut epsa = 0.0;
    let mut rb = [0.0_f64; 9];
    let mut rp = [0.0_f64; 9];
    let mut rbp = [0.0_f64; 9];
    let mut rbpn = [0.0_f64; 9];
    eraPn00a(
        date1,
        date2,
        &mut dpsi,
        &mut deps,
        &mut epsa,
        rb.as_mut_ptr(),
        rp.as_mut_ptr(),
        rbp.as_mut_ptr(),
        rmatn,
        rbpn.as_mut_ptr(),
    );
}

// eraNum00b  Nutation matrix, IAU 2000B
pub unsafe fn eraNum00b(date1: f64, date2: f64, rmatn: *mut f64) {
    let mut dpsi = 0.0;
    let mut deps = 0.0;
    let mut epsa = 0.0;
    let mut rb = [0.0_f64; 9];
    let mut rp = [0.0_f64; 9];
    let mut rbp = [0.0_f64; 9];
    let mut rbpn = [0.0_f64; 9];
    eraPn00b(
        date1,
        date2,
        &mut dpsi,
        &mut deps,
        &mut epsa,
        rb.as_mut_ptr(),
        rp.as_mut_ptr(),
        rbp.as_mut_ptr(),
        rmatn,
        rbpn.as_mut_ptr(),
    );
}

// eraNum06a  Nutation matrix, IAU 2006/2000A
pub unsafe fn eraNum06a(date1: f64, date2: f64, rmatn: *mut f64) {
    let eps = eraObl06(date1, date2);
    let mut dp = 0.0;
    let mut de = 0.0;
    eraNut06a(date1, date2, &mut dp, &mut de);
    eraNumat(eps, dp, de, rmatn);
}

// eraNumat  Build nutation matrix from components
pub unsafe fn eraNumat(epsa: f64, dpsi: f64, deps: f64, rmatn: *mut f64) {
    eraIr(rmatn);
    eraRx(epsa, rmatn);
    eraRz(-dpsi, rmatn);
    eraRx(-(epsa + deps), rmatn);
}
