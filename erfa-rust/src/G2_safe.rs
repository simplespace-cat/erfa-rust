// G2
//   apco.c    → eraApco_safe
//   apco13.c  → eraApco13_safe
//   apcs.c    → eraApcs_safe
//   apcs13.c  → eraApcs13_safe
//   aper.c    → eraAper_safe


use crate::G11_safe::eraEors_safe;
use crate::G13_safe::eraEpv00_safe;
use crate::G14_safe::eraEra00_safe;
use crate::G19_safe::eraIr_safe;
use crate::G1_safe::eraAnpm_safe;
use crate::G25_safe::eraPn_safe;
use crate::G26_safe::eraPnm06a_safe;
use crate::G27_safe::eraPvtob_safe;
use crate::G28_safe::{eraRefco_safe, eraRx_safe, eraRy_safe, eraRz_safe};
use crate::G30_safe::{eraS06_safe, eraSp00_safe};
use crate::G32_safe::eraTaitt_safe;
use crate::G33_safe::{eraTrxpv_safe, eraUtctai_safe, eraUtcut1_safe};
use crate::G6_safe::eraBpn2xy_safe;
use crate::G7_safe::eraC2ixys_safe;
use crate::G8_safe::{eraCp_safe, eraCr_safe};
use crate::H1_safe::{eraASTROM, ERFA_AULT, ERFA_DAU, ERFA_DAYSEC, ERFA_DJ00, ERFA_DJY};

pub type ErfaResult<T> = Result<T, ()>;

// G2/apco.c
// Prepare star-independent astrometry parameters for a terrestrial observer.
pub fn eraApco_safe(
    date1: f64,
    date2: f64,
    ebpv: &[[f64; 3]; 2],
    ehp: &[f64; 3],
    x: f64,
    y: f64,
    s: f64,
    theta: f64,
    elong: f64,
    phi: f64,
    hm: f64,
    xp: f64,
    yp: f64,
    sp: f64,
    refa: f64,
    refb: f64,
    astrom: &mut eraASTROM,
) -> ErfaResult<()> {
    // Build rotation matrix for local parameters.
    let mut r = [[0.0_f64; 3]; 3];
    eraIr_safe(&mut r)?;
    eraRz_safe(theta + sp, &mut r)?;
    eraRy_safe(-xp, &mut r)?;
    eraRx_safe(-yp, &mut r)?;
    eraRz_safe(elong, &mut r)?;

    // Local Earth rotation angle.
    let mut a = r[0][0];
    let mut b = r[0][1];
    let eral = if a != 0.0 || b != 0.0 {
        b.atan2(a)
    } else {
        0.0
    };
    astrom.eral = eral;

    // Polar motion with respect to local meridian.
    a = r[0][0];
    let c = r[0][2];
    astrom.xpl = c.atan2((a * a + b * b).sqrt());
    a = r[1][2];
    b = r[2][2];
    astrom.ypl = if a != 0.0 || b != 0.0 {
        -a.atan2(b)
    } else {
        0.0
    };

    // Adjusted longitude.
    astrom.along = eraAnpm_safe(eral - theta)?;

    // Latitude functions.
    astrom.sphi = phi.sin();
    astrom.cphi = phi.cos();

    // Refraction constants.
    astrom.refa = refa;
    astrom.refb = refb;

    // Disable diurnal aberration step.
    astrom.diurab = 0.0;

    // CIO-based C2I matrix (overwrite r with returned matrix).
    let r_c2i = eraC2ixys_safe(x, y, s)?;
    r = r_c2i;

    // Observer geocentric PV in CIRS, then rotate into GCRS.
    let pvc = eraPvtob_safe(elong, phi, hm, xp, yp, sp, theta)?;
    let pv = eraTrxpv_safe(&r, &pvc)?;

    // ICRS <-> GCRS parameters.
    eraApcs_safe(date1, date2, &pv, ebpv, ehp, astrom)?;

    // Store the C2I (BPN) matrix.
    eraCr_safe(&r, &mut astrom.bpn)?;

    Ok(())
}

// G2/apco13.c
// Prepare astrometry parameters from UTC; returns (eo, j) where j=0 or +1.
pub fn eraApco13_safe(
    utc1: f64,
    utc2: f64,
    dut1: f64,
    elong: f64,
    phi: f64,
    hm: f64,
    xp: f64,
    yp: f64,
    phpa: f64,
    tc: f64,
    rh: f64,
    wl: f64,
    astrom: &mut eraASTROM,
) -> ErfaResult<(f64, i32)> {
    // UTC→TAI and TAI→TT; UTC→UT1.
    let ((tai1, tai2), j_utctai) = eraUtctai_safe(utc1, utc2)?;
    if j_utctai < 0 {
        return Err(());
    }
    let ((tt1, tt2), _j_tai_tt) = eraTaitt_safe(tai1, tai2)?;
    let ((ut11, ut12), j_ut1) = eraUtcut1_safe(utc1, utc2, dut1)?;
    if j_ut1 < 0 {
        return Err(());
    }

    // Earth ephemeris, CIP/CIO, refraction.
    let (ehpv, ebpv, _jstat) = eraEpv00_safe(tt1, tt2)?;
    let r = eraPnm06a_safe(tt1, tt2)?;
    let (x, y) = eraBpn2xy_safe(&r)?;
    let s = eraS06_safe(tt1, tt2, x, y)?;
    let theta = eraEra00_safe(ut11, ut12)?;
    let sp = eraSp00_safe(tt1, tt2)?;
    let (refa, refb) = eraRefco_safe(phpa, tc, rh, wl)?;

    // Assemble star-independent astrometry parameters.
    eraApco_safe(
        tt1, tt2, &ebpv, &ehpv[0], x, y, s, theta, elong, phi, hm, xp, yp, sp, refa, refb, astrom,
    )?;

    // Equation of the origins.
    let eo = eraEors_safe(&r, s)?;

    // Return EO and UT1 status (0 or +1).
    Ok((eo, j_ut1))
}

// G2/apcs.c
// Compute ICRS↔GCRS parameters for a space observer.
pub fn eraApcs_safe(
    date1: f64,
    date2: f64,
    pv: &[[f64; 3]; 2],   // (m, m/s)
    ebpv: &[[f64; 3]; 2], // (au, au/d)
    ehp: &[f64; 3],       // (au)
    astrom: &mut eraASTROM,
) -> ErfaResult<()> {
    // Unit conversions.
    const AUDMS: f64 = ERFA_DAU / ERFA_DAYSEC; // au/day to m/s
    const CR: f64 = ERFA_AULT / ERFA_DAYSEC; // speed of light in au/day

    // Time since reference epoch, years (for proper motion).
    astrom.pmt = ((date1 - ERFA_DJ00) + date2) / ERFA_DJY;

    // Adjust Earth ephemeris to observer.
    let mut pb = [0.0_f64; 3];
    let mut vb = [0.0_f64; 3];
    let mut ph = [0.0_f64; 3];
    for i in 0..3 {
        let dp = pv[0][i] / ERFA_DAU;
        let dv = pv[1][i] / AUDMS;
        pb[i] = ebpv[0][i] + dp;
        vb[i] = ebpv[1][i] + dv;
        ph[i] = ehp[i] + dp;
    }

    // Barycentric position of observer (au).
    eraCp_safe(&pb, &mut astrom.eb)?;

    // Heliocentric direction (unit) and distance (au).
    let (em, eh) = eraPn_safe(&ph)?;
    astrom.em = em;
    astrom.eh = eh;

    // Barycentric velocity in units of c and reciprocal Lorentz factor.
    let mut v2 = 0.0;
    for i in 0..3 {
        let w = vb[i] * CR;
        astrom.v[i] = w;
        v2 += w * w;
    }
    astrom.bm1 = (1.0 - v2).sqrt();

    // Reset the NPB matrix.
    eraIr_safe(&mut astrom.bpn)?;

    Ok(())
}

// G2/apcs13.c
// As eraApcs_safe but Earth ephemerides computed internally.
pub fn eraApcs13_safe(
    date1: f64,
    date2: f64,
    pv: &[[f64; 3]; 2], // (m, m/s)
    astrom: &mut eraASTROM,
) -> ErfaResult<()> {
    // Earth barycentric & heliocentric PV (au, au/day).
    let (ehpv, ebpv, _jstat) = eraEpv00_safe(date1, date2)?;

    // Compute the star-independent astrometry parameters.
    eraApcs_safe(date1, date2, pv, &ebpv, &ehpv[0], astrom)
}

// G2/aper.c
// Update local apparent sidereal angle.
pub fn eraAper_safe(theta: f64, astrom: &mut eraASTROM) -> ErfaResult<()> {
    astrom.eral = theta + astrom.along;
    Ok(())
}
