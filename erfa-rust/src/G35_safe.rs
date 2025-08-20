// G35
//   xys00a.c → eraXys00a_safe
//   xys00b.c → eraXys00b_safe
//   xys06a.c → eraXys06a_safe
//   zp.c     → eraZp_safe
//   zpv.c    → eraZpv_safe
//   zr.c     → eraZr_safe

use crate::G26_safe::{eraPnm00a_safe, eraPnm00b_safe, eraPnm06a_safe};
use crate::G29_safe::eraS00_safe;
use crate::G30_safe::eraS06_safe;
use crate::G6_safe::eraBpn2xy_safe;

pub type ErfaResult<T> = Result<T, ()>;


//  eraXys00a_safe    CIP X,Y and CIO locator s using IAU 2000A model
//  Returns: (x, y, s)
pub fn eraXys00a_safe(date1: f64, date2: f64) -> ErfaResult<(f64, f64, f64)> {
    // Form NPB matrix, IAU 2000A
    let rbpn = eraPnm00a_safe(date1, date2)?;
    // Extract X, Y
    let (x, y) = eraBpn2xy_safe(&rbpn)?;
    // CIO locator s
    let s = eraS00_safe(date1, date2, x, y)?;
    Ok((x, y, s))
}

// CIP X,Y and CIO locator s using IAU 2000B model.
pub fn eraXys00b_safe(date1: f64, date2: f64) -> ErfaResult<(f64, f64, f64)> {
    let rbpn = eraPnm00b_safe(date1, date2)?;
    let (x, y) = eraBpn2xy_safe(&rbpn)?;
    let s = eraS00_safe(date1, date2, x, y)?;
    Ok((x, y, s))
}

// CIP X,Y and CIO locator s using IAU 2006/2000A model.
pub fn eraXys06a_safe(date1: f64, date2: f64) -> ErfaResult<(f64, f64, f64)> {
    // NPB matrix, IAU 2006 precession + 2000A nutation
    let rbpn = eraPnm06a_safe(date1, date2)?;
    let (x, y) = eraBpn2xy_safe(&rbpn)?;
    let s = eraS06_safe(date1, date2, x, y)?;
    Ok((x, y, s))
}

// Zero a 3-vector.
pub fn eraZp_safe() -> [f64; 3] {
    [0.0, 0.0, 0.0]
}

// Zero a 2×3 pv-vector.
pub fn eraZpv_safe() -> [[f64; 3]; 2] {
    [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]
}

// Initialize a 3×3 matrix to zero.
pub fn eraZr_safe() -> [[f64; 3]; 3] {
    [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]
}
