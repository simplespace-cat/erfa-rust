// G8
//   c2tcio.c  → eraC2tcio_safe
//   c2teqx.c  → eraC2teqx_safe
//   c2tpe.c   → eraC2tpe_safe
//   c2txy.c   → eraC2txy_safe
//   cal2jd.c  → eraCal2jd_safe
//   cp.c      → eraCp_safe
//   cpv.c     → eraCpv_safe
//   cr.c      → eraCr_safe

use crate::H1_safe::ERFA_DJM0;

use crate::G11_safe::eraEe00_safe;
use crate::G14_safe::eraEra00_safe;
use crate::G17_safe::eraGmst00_safe;
use crate::G25_safe::eraPn00_safe;
use crate::G26_safe::eraPom00_safe;
use crate::G28_safe::{eraRxr_safe, eraRz_safe};
use crate::G30_safe::eraSp00_safe;
use crate::G7_safe::eraC2ixy_safe;

pub type ErfaResult<T> = Result<T, ()>;

//----------------------------------------------------------------------
// G8/c2tcio.c → eraC2tcio_safe
//----------------------------------------------------------------------

// Form the celestial-to-terrestrial matrix given rc2i, ERA and polar motion.
pub fn eraC2tcio_safe(
    rc2i: &[[f64; 3]; 3],
    era: f64,
    rpom: &[[f64; 3]; 3],
    rc2t: &mut [[f64; 3]; 3],
) -> ErfaResult<()> {
    // rc2t ← rc2i
    eraCr_safe(rc2i, rc2t)?;

    // rc2t ← Rz(era) * rc2t
    eraRz_safe(era, rc2t)?;

    // rc2t ← rpom * rc2t
    let m = eraRxr_safe(rpom, rc2t)?;
    eraCr_safe(&m, rc2t)
}

//----------------------------------------------------------------------
// G8/c2teqx.c → eraC2teqx_safe
//----------------------------------------------------------------------

// Form celestial-to-terrestrial using equinox method: rbpn, GMST+EE, rpom.
pub fn eraC2teqx_safe(
    rbpn: &[[f64; 3]; 3],
    gst: f64,
    rpom: &[[f64; 3]; 3],
    rc2t: &mut [[f64; 3]; 3],
) -> ErfaResult<()> {
    // rc2t ← rbpn
    eraCr_safe(rbpn, rc2t)?;

    // rc2t ← Rz(gst) * rc2t
    eraRz_safe(gst, rc2t)?;

    // rc2t ← rpom * rc2t
    let m = eraRxr_safe(rpom, rc2t)?;
    eraCr_safe(&m, rc2t)
}

//----------------------------------------------------------------------
// G8/c2tpe.c → eraC2tpe_safe
//----------------------------------------------------------------------

// Full celestial-to-terrestrial, IAU 2000 equinox-based (dψ, dε given).
pub fn eraC2tpe_safe(
    tta: f64,
    ttb: f64,
    uta: f64,
    utb: f64,
    dpsi: f64,
    deps: f64,
    xp: f64,
    yp: f64,
    rc2t: &mut [[f64; 3]; 3],
) -> ErfaResult<()> {
    // Celestial-to-true matrix components for this TT.
    let (epsa, _rb, _rp, _rbp, _rn, rbpn) = eraPn00_safe(tta, ttb, dpsi, deps)?;

    // Greenwich Mean Sidereal Time.
    let gmst = eraGmst00_safe(uta, utb, tta, ttb)?;

    // Equation of the equinoxes.
    let ee = eraEe00_safe(tta, ttb, epsa, dpsi)?;

    // TIO locator s'.
    let sp = eraSp00_safe(tta, ttb)?;

    // Polar-motion matrix.
    let rpom = eraPom00_safe(xp, yp, sp)?;

    // Combine to form celestial-to-terrestrial.
    eraC2teqx_safe(&rbpn, gmst + ee, &rpom, rc2t)
}

//----------------------------------------------------------------------
// G8/c2txy.c → eraC2txy_safe
//----------------------------------------------------------------------

// Celestial-to-terrestrial using X,Y (CIP) method.
pub fn eraC2txy_safe(
    tta: f64,
    ttb: f64,
    uta: f64,
    utb: f64,
    x: f64,
    y: f64,
    xp: f64,
    yp: f64,
    rc2t: &mut [[f64; 3]; 3],
) -> ErfaResult<()> {
    // Celestial-to-intermediate matrix from X,Y.
    let rc2i = eraC2ixy_safe(tta, ttb, x, y)?;

    // Earth rotation angle and TIO locator.
    let era = eraEra00_safe(uta, utb)?;
    let sp = eraSp00_safe(tta, ttb)?;

    // Polar-motion matrix.
    let rpom = eraPom00_safe(xp, yp, sp)?;

    // Combine to form celestial-to-terrestrial.
    eraC2tcio_safe(&rc2i, era, &rpom, rc2t)
}

//----------------------------------------------------------------------
// G8/cal2jd.c → eraCal2jd_safe
//----------------------------------------------------------------------

// Gregorian calendar to JD, split into (djm0, djm); j=0 OK, -1/-2/-3 for errors.
pub fn eraCal2jd_safe(iy: i32, im: i32, id: i32) -> ErfaResult<((f64, f64), i32)> {
    const IYMIN: i32 = -4799;
    const MTAB: [i32; 12] = [31, 28, 31, 30, 31, 31, 30, 31, 30, 31, 30, 31];

    // Validate year & month.
    let mut j = 0;
    if iy < IYMIN {
        j = -1;
    } else if im < 1 || im > 12 {
        j = -2;
    }

    // Leap year test for February.
    let ly = if im == 2 && (iy % 4 == 0) && (iy % 100 != 0 || iy % 400 == 0) {
        1
    } else {
        0
    };

    // Validate day; for -3 we still compute JD per C behavior.
    if j == 0 && (id < 1 || id > MTAB[(im - 1) as usize] + ly) {
        j = -3;
    }

    // Compute JD parts (djm0, djm).
    let (djm0, djm) = {
        let my = (im - 14) / 12;
        let iyp = (iy + my) as i64;

        let djm0 = ERFA_DJM0;
        let djm = (1461 * (iyp + 4800) / 4 + 367 * (im as i64 - 2 - 12 * my as i64) / 12
            - 3 * ((iyp + 4900) / 100) / 4
            + id as i64
            - 2_432_076) as f64;
        (djm0, djm)
    };

    Ok(((djm0, djm), j))
}

//----------------------------------------------------------------------
// G8/cp.c → eraCp_safe
//----------------------------------------------------------------------

// Copy 3-vector.
pub fn eraCp_safe(p: &[f64; 3], c: &mut [f64; 3]) -> ErfaResult<()> {
    c[0] = p[0];
    c[1] = p[1];
    c[2] = p[2];
    Ok(())
}

//----------------------------------------------------------------------
// G8/cpv.c → eraCpv_safe
//----------------------------------------------------------------------

// Copy position/velocity 3+3-vector.
pub fn eraCpv_safe(pv: &[[f64; 3]; 2], c: &mut [[f64; 3]; 2]) -> ErfaResult<()> {
    c[0][0] = pv[0][0];
    c[0][1] = pv[0][1];
    c[0][2] = pv[0][2];
    c[1][0] = pv[1][0];
    c[1][1] = pv[1][1];
    c[1][2] = pv[1][2];
    Ok(())
}

//----------------------------------------------------------------------
// G8/cr.c → eraCr_safe
//----------------------------------------------------------------------

// Copy 3×3 matrix.
pub fn eraCr_safe(r: &[[f64; 3]; 3], c: &mut [[f64; 3]; 3]) -> ErfaResult<()> {
    for i in 0..3 {
        for j in 0..3 {
            c[i][j] = r[i][j];
        }
    }
    Ok(())
}
