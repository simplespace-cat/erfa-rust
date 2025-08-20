// G3
//   aper13.c  → eraAper13_safe
//   apio.c    → eraApio_safe
//   apio13.c  → eraApio13_safe
//   atcc13.c  → eraAtcc13_safe
//   atccq.c   → eraAtccq_safe


use crate::G14_safe::eraEra00_safe;
use crate::G19_safe::eraIr_safe;
use crate::G1_safe::{eraAnp_safe, eraAnpm_safe, eraApci13_safe};
use crate::G25_safe::eraPmpx_safe;
use crate::G27_safe::eraPvtob_safe;
use crate::G28_safe::{eraRefco_safe, eraRx_safe, eraRy_safe, eraRz_safe};
use crate::G2_safe::eraAper_safe;
use crate::G30_safe::eraSp00_safe;
use crate::G32_safe::eraTaitt_safe;
use crate::G33_safe::{eraUtctai_safe, eraUtcut1_safe};
use crate::G7_safe::eraC2s_safe;
use crate::H1_safe::{eraASTROM, ERFA_CMPS};

pub type ErfaResult<T> = Result<T, ()>;

// G3/aper13.c
// Update only the ERA element inside an existing eraASTROM.
pub fn eraAper13_safe(ut11: f64, ut12: f64, astrom: &mut eraASTROM) -> ErfaResult<()> {
    let era = eraEra00_safe(ut11, ut12)?;
    eraAper_safe(era, astrom)?;
    Ok(())
}

// G3/apio.c
// Prepare CIRS-observed astrometry parameters using supplied site/refraction.
pub fn eraApio_safe(
    sp: f64,
    theta: f64,
    elong: f64,
    phi: f64,
    hm: f64,
    xp: f64,
    yp: f64,
    refa: f64,
    refb: f64,
    astrom: &mut eraASTROM,
) -> ErfaResult<()> {
    // Build rotation matrix CIRS → apparent [HA,Dec].
    let mut r: [[f64; 3]; 3] = [[0.0; 3]; 3];
    eraIr_safe(&mut r)?;
    eraRz_safe(theta + sp, &mut r)?;
    eraRy_safe(-xp, &mut r)?;
    eraRx_safe(-yp, &mut r)?;
    eraRz_safe(elong, &mut r)?;

    // Local Earth rotation angle.
    let a = r[0][0];
    let b = r[0][1];
    let eral = if a != 0.0 || b != 0.0 {
        b.atan2(a)
    } else {
        0.0
    };
    astrom.eral = eral;

    // Polar motion [X,Y] w.r.t local meridian.
    let a0 = r[0][0];
    let b0 = r[0][1];
    let c0 = r[0][2];
    astrom.xpl = c0.atan2((a0 * a0 + b0 * b0).sqrt());
    let a1 = r[1][2];
    let b1 = r[2][2];
    astrom.ypl = if a1 != 0.0 || b1 != 0.0 {
        -a1.atan2(b1)
    } else {
        0.0
    };

    // Adjusted longitude.
    astrom.along = eraAnpm_safe(eral - theta)?;

    // Functions of latitude.
    astrom.sphi = phi.sin();
    astrom.cphi = phi.cos();

    // Observer geocentric position and velocity (m, m/s, CIRS).
    let pv = eraPvtob_safe(elong, phi, hm, xp, yp, sp, theta)?;

    // Magnitude of diurnal aberration vector.
    astrom.diurab = (pv[1][0] * pv[1][0] + pv[1][1] * pv[1][1]).sqrt() / ERFA_CMPS;

    // Refraction constants.
    astrom.refa = refa;
    astrom.refb = refb;

    Ok(())
}

// G3/apio13.c
// Derive CIRS-observed parameters from UTC/site/weather; returns status 0 or +1.
pub fn eraApio13_safe(
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
) -> ErfaResult<i32> {
    // UTC → TAI.
    let ((tai1, tai2), j_utctai) = eraUtctai_safe(utc1, utc2)?;
    if j_utctai < 0 {
        return Err(());
    }

    // TAI → TT.
    let ((tt1, tt2), _j_taitt) = eraTaitt_safe(tai1, tai2)?;

    // UTC → UT1 (with DUT1).
    let ((ut11, ut12), j_utcut1) = eraUtcut1_safe(utc1, utc2, dut1)?;
    if j_utcut1 < 0 {
        return Err(());
    }

    // TIO locator s'.
    let sp = eraSp00_safe(tt1, tt2)?;

    // Earth rotation angle.
    let theta = eraEra00_safe(ut11, ut12)?;

    // Refraction constants A and B.
    let (refa, refb) = eraRefco_safe(phpa, tc, rh, wl)?;

    // Populate astrometry parameters.
    eraApio_safe(sp, theta, elong, phi, hm, xp, yp, refa, refb, astrom)?;

    // Return the UT1 conversion warning status (0 or +1).
    Ok(j_utcut1)
}

// G3/atcc13.c
// Catalog ICRS (J2000) to ICRS astrometric using 2013 model; returns (ra, da).
pub fn eraAtcc13_safe(
    rc: f64,
    dc: f64,
    pr: f64,
    pd: f64,
    px: f64,
    rv: f64,
    date1: f64,
    date2: f64,
) -> ErfaResult<(f64, f64)> {
    // Star-independent parameters.
    let mut astrom = eraASTROM::default();

    // Transformation parameters (ICRS catalog → astrometric).
    let _eo = eraApci13_safe(date1, date2, &mut astrom)?;

    // Quick transform using precomputed astrom.
    let (ra, da) = eraAtccq_safe(rc, dc, pr, pd, px, rv, &astrom)?;
    Ok((ra, da))
}

// G3/atccq.c
// Quick catalog (ICRS) to astrometric given precomputed astrom; returns (ra, da).
pub fn eraAtccq_safe(
    rc: f64,
    dc: f64,
    pr: f64,
    pd: f64,
    px: f64,
    rv: f64,
    astrom: &eraASTROM,
) -> ErfaResult<(f64, f64)> {
    // Proper motion & parallax to BCRS coordinate direction.
    let p = eraPmpx_safe(rc, dc, pr, pd, px, rv, astrom.pmt, &astrom.eb)?;

    // ICRS astrometric RA,Dec.
    let (w, da) = eraC2s_safe(&p)?;
    let ra = eraAnp_safe(w)?;
    Ok((ra, da))
}
