// G7
//   c2i00a.c  → eraC2i00a_safe
//   c2i00b.c  → eraC2i00b_safe
//   c2i06a.c  → eraC2i06a_safe
//   c2ibpn.c  → eraC2ibpn_safe
//   c2ixy.c   → eraC2ixy_safe
//   c2ixys.c  → eraC2ixys_safe
//   c2s.c     → eraC2s_safe
//   c2t00a.c  → eraC2t00a_safe
//   c2t00b.c  → eraC2t00b_safe
//   c2t06a.c  → eraC2t06a_safe

use crate::G14_safe::eraEra00_safe;
use crate::G19_safe::eraIr_safe;
use crate::G26_safe::{eraPnm00a_safe, eraPnm00b_safe, eraPnm06a_safe, eraPom00_safe};
use crate::G28_safe::{eraRy_safe, eraRz_safe};
use crate::G29_safe::eraS00_safe;
use crate::G30_safe::{eraS06_safe, eraSp00_safe};
use crate::G6_safe::eraBpn2xy_safe;
use crate::G8_safe::eraC2tcio_safe;

pub type ErfaResult<T> = Result<T, ()>;

// Celestial-to-intermediate matrix, IAU 2000A; returns rc2i.
pub fn eraC2i00a_safe(date1: f64, date2: f64) -> ErfaResult<[[f64; 3]; 3]> {
    let rbpn = eraPnm00a_safe(date1, date2)?;
    eraC2ibpn_safe(date1, date2, &rbpn)
}

// Celestial-to-intermediate matrix, IAU 2000B; returns rc2i.
pub fn eraC2i00b_safe(date1: f64, date2: f64) -> ErfaResult<[[f64; 3]; 3]> {
    let rbpn = eraPnm00b_safe(date1, date2)?;
    eraC2ibpn_safe(date1, date2, &rbpn)
}

// Celestial-to-intermediate matrix, IAU 2006/2000A; returns rc2i.
pub fn eraC2i06a_safe(date1: f64, date2: f64) -> ErfaResult<[[f64; 3]; 3]> {
    let rbpn = eraPnm06a_safe(date1, date2)?;
    let (x, y) = eraBpn2xy_safe(&rbpn)?;
    let s = eraS06_safe(date1, date2, x, y)?;
    eraC2ixys_safe(x, y, s)
}

// Celestial-to-intermediate matrix from supplied NPB; returns rc2i.
pub fn eraC2ibpn_safe(date1: f64, date2: f64, rbpn: &[[f64; 3]; 3]) -> ErfaResult<[[f64; 3]; 3]> {
    let (x, y) = eraBpn2xy_safe(rbpn)?;
    eraC2ixy_safe(date1, date2, x, y)
}

// Celestial-to-intermediate matrix from X,Y and date (S00); returns rc2i.
pub fn eraC2ixy_safe(date1: f64, date2: f64, x: f64, y: f64) -> ErfaResult<[[f64; 3]; 3]> {
    let s = eraS00_safe(date1, date2, x, y)?;
    eraC2ixys_safe(x, y, s)
}

// Celestial-to-intermediate matrix from X,Y and CIO locator s; returns rc2i.
pub fn eraC2ixys_safe(x: f64, y: f64, s: f64) -> ErfaResult<[[f64; 3]; 3]> {
    // Compute e (longitude of the CIP) and d (pole inclination).
    let r2 = x * x + y * y;
    let e = if r2 > 0.0 { y.atan2(x) } else { 0.0 };
    let d = (r2 / (1.0 - r2)).sqrt().atan();

    // Compose rc2i = Rz(e)  Ry(d)  Rz(-(e+s)).
    let mut rc2i = [[0.0_f64; 3]; 3];
    eraIr_safe(&mut rc2i)?;
    eraRz_safe(e, &mut rc2i)?;
    eraRy_safe(d, &mut rc2i)?;
    eraRz_safe(-(e + s), &mut rc2i)?;
    Ok(rc2i)
}

// P-vector to spherical angles; returns (theta, phi).
pub fn eraC2s_safe(p: &[f64; 3]) -> ErfaResult<(f64, f64)> {
    let x = p[0];
    let y = p[1];
    let z = p[2];
    let d2 = x * x + y * y;
    let theta = if d2 == 0.0 { 0.0 } else { y.atan2(x) };
    let phi = if z == 0.0 { 0.0 } else { z.atan2(d2.sqrt()) };
    Ok((theta, phi))
}

// Celestial-to-terrestrial matrix, IAU 2000A; returns rc2t.
pub fn eraC2t00a_safe(
    tta: f64,
    ttb: f64,
    uta: f64,
    utb: f64,
    xp: f64,
    yp: f64,
) -> ErfaResult<[[f64; 3]; 3]> {
    let rc2i = eraC2i00a_safe(tta, ttb)?;
    let era = eraEra00_safe(uta, utb)?;
    let sp = eraSp00_safe(tta, ttb)?;
    let rpom = eraPom00_safe(xp, yp, sp)?;

    let mut rc2t = [[0.0_f64; 3]; 3];
    eraC2tcio_safe(&rc2i, era, &rpom, &mut rc2t)?;
    Ok(rc2t)
}

// Celestial-to-terrestrial matrix, IAU 2000B; returns rc2t.
pub fn eraC2t00b_safe(
    tta: f64,
    ttb: f64,
    uta: f64,
    utb: f64,
    xp: f64,
    yp: f64,
) -> ErfaResult<[[f64; 3]; 3]> {
    let rc2i = eraC2i00b_safe(tta, ttb)?;
    let era = eraEra00_safe(uta, utb)?;
    let rpom = eraPom00_safe(xp, yp, 0.0)?;

    let mut rc2t = [[0.0_f64; 3]; 3];
    eraC2tcio_safe(&rc2i, era, &rpom, &mut rc2t)?;
    Ok(rc2t)
}

// Celestial-to-terrestrial matrix, IAU 2006/2000A; returns rc2t.
pub fn eraC2t06a_safe(
    tta: f64,
    ttb: f64,
    uta: f64,
    utb: f64,
    xp: f64,
    yp: f64,
) -> ErfaResult<[[f64; 3]; 3]> {
    let rc2i = eraC2i06a_safe(tta, ttb)?;
    let era = eraEra00_safe(uta, utb)?;
    let sp = eraSp00_safe(tta, ttb)?;
    let rpom = eraPom00_safe(xp, yp, sp)?;

    let mut rc2t = [[0.0_f64; 3]; 3];
    eraC2tcio_safe(&rc2i, era, &rpom, &mut rc2t)?;
    Ok(rc2t)
}
