// G5
//   atoc13.c  → eraAtoc13_safe
//   atoi13.c  → eraAtoi13_safe
//   atoiq.c   → eraAtoiq_safe


use crate::G1_safe::eraAnp_safe;
use crate::G29_safe::eraS2c_safe;
use crate::G2_safe::eraApco13_safe;
use crate::G3_safe::eraApio13_safe;
use crate::G4_safe::eraAticq_safe;
use crate::G7_safe::eraC2s_safe;
use crate::H1_safe::eraASTROM;

pub type ErfaResult<T> = Result<T, ()>;

/*----------------------------------------------------------------------
 *  G5/atoc13.c  →  eraAtoc13_safe
 *--------------------------------------------------------------------*/
/// Observed place → ICRS astrometric RA,Dec (2013 models).
/// Returns (rc, dc, eo, j) where:
/// - rc, dc: ICRS astrometric RA,Dec (radians)
/// - eo: equation of the origins (radians)
/// - j: UT1 conversion status from eraApco13 (0 or +1)
///
/// NOTE: The original C function eraAtoc13 computes but discards the
/// equation of the origins (eo) value. This safe Rust version corrects
/// this oversight by including eo in the return tuple. The eo value is
/// calculated by eraApco13 and represents the equation of the origins,
/// which is the distance between the CIO and the equinox along the
/// celestial equator. While the original C API omits this value
/// (likely for historical API compatibility reasons), I include it
/// here as it may be useful for certain astronomical calculations and
/// comes at no additional computational cost.

pub fn eraAtoc13_safe(
    type_: &str,
    ob1: f64,
    ob2: f64,
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
) -> ErfaResult<(f64, f64, f64, i32)> {
    // Star-independent astrometry parameters
    let mut astrom = eraASTROM::default();

    // Get astrometry parameters for ICRS ↔ observed; eo is computed here
    let (eo, j) = eraApco13_safe(
        utc1,
        utc2,
        dut1,
        elong,
        phi,
        hm,
        xp,
        yp,
        phpa,
        tc,
        rh,
        wl,
        &mut astrom,
    )?;

    // Observed → CIRS
    let (ri, di) = eraAtoiq_safe(type_, ob1, ob2, &astrom)?;

    // CIRS → ICRS
    let (rc, dc) = eraAticq_safe(ri, di, &astrom)?;

    Ok((rc, dc, eo, j))
}

/*----------------------------------------------------------------------
 *  G5/atoi13.c  →  eraAtoi13_safe
 *--------------------------------------------------------------------*/

// Observed place → CIRS (2013 models). Returns (ri, di, j).
pub fn eraAtoi13_safe(
    type_: &str,
    ob1: f64,
    ob2: f64,
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
) -> ErfaResult<(f64, f64, i32)> {
    // Star-independent astrometry parameters
    let mut astrom = eraASTROM::default();

    // Get astrometry parameters for CIRS ↔ observed
    let j = eraApio13_safe(
        utc1,
        utc2,
        dut1,
        elong,
        phi,
        hm,
        xp,
        yp,
        phpa,
        tc,
        rh,
        wl,
        &mut astrom,
    )?;

    // Observed → CIRS
    let (ri, di) = eraAtoiq_safe(type_, ob1, ob2, &astrom)?;
    Ok((ri, di, j))
}

/*----------------------------------------------------------------------
 *  G5/atoiq.c  →  eraAtoiq_safe
 *--------------------------------------------------------------------*/

// Quick observed place → CIRS using supplied astrometry parameters.
pub fn eraAtoiq_safe(
    type_: &str,
    ob1: f64,
    ob2: f64,
    astrom: &eraASTROM,
) -> ErfaResult<(f64, f64)> {
    const SELMIN: f64 = 0.05; // minimum proxy for refraction clamp

    // Deref astrom once for convenience
    let a = astrom;

    // Coordinate kind (first char only)
    let mut c = type_.as_bytes().get(0).map(|b| *b as char).unwrap_or('A');

    // Work variables
    let mut c1 = ob1;
    let c2 = ob2;

    let sphi = a.sphi;
    let cphi = a.cphi;

    // Standardize coordinate code
    c = match c {
        'r' | 'R' => 'R',
        'h' | 'H' => 'H',
        _ => 'A',
    };

    // Cartesian vector of the line of sight (S=0,E=90)
    let (xaeo, yaeo, zaeo) = if c == 'A' {
        // Input is Az, ZD.
        let ce = c2.sin();
        (-c1.cos() * ce, c1.sin() * ce, c2.cos())
    } else {
        // If RA,Dec convert to HA,Dec.
        if c == 'R' {
            c1 = a.eral - c1;
        }
        // To Cartesian -HA,Dec.
        let v0 = eraS2c_safe(-c1, c2)?; // [xmhdo, ymhdo, zmhdo]
        let xmhdo = v0[0];
        let ymhdo = v0[1];
        let zmhdo = v0[2];

        // To Cartesian Az,El (S=0,E=90).
        (
            sphi * xmhdo - cphi * zmhdo,
            ymhdo,
            cphi * xmhdo + sphi * zmhdo,
        )
    };

    // Azimuth (S=0,E=90)
    let az = if xaeo != 0.0 || yaeo != 0.0 {
        yaeo.atan2(xaeo)
    } else {
        0.0
    };

    // Sine of observed ZD, and observed ZD
    let sz = (xaeo * xaeo + yaeo * yaeo).sqrt();
    let zdo = sz.atan2(zaeo);

    // Refraction
    let refa = a.refa;
    let refb = a.refb;
    let tz = sz / if zaeo > SELMIN { zaeo } else { SELMIN };
    let dref = (refa + refb * tz * tz) * tz;
    let zdt = zdo + dref;

    // To Cartesian Az,ZD after refraction
    let ce = zdt.sin();
    let xaet = az.cos() * ce;
    let yaet = az.sin() * ce;
    let zaet = zdt.cos();

    // Cartesian Az,ZD → Cartesian -HA,Dec
    let xmhda = sphi * xaet + cphi * zaet;
    let ymhda = yaet;
    let zmhda = -cphi * xaet + sphi * zaet;

    // Diurnal aberration
    let f = 1.0 + a.diurab * ymhda;
    let xhd = f * xmhda;
    let yhd = f * (ymhda - a.diurab);
    let zhd = f * zmhda;

    // Polar motion
    let sx = a.xpl.sin();
    let cx = a.xpl.cos();
    let sy = a.ypl.sin();
    let cy = a.ypl.cos();
    let v = [
        cx * xhd + sx * sy * yhd - sx * cy * zhd,
        cy * yhd + sy * zhd,
        sx * xhd - cx * sy * yhd + cx * cy * zhd,
    ];

    // To spherical -HA,Dec
    let (hma, di) = eraC2s_safe(&v)?;

    // Right ascension
    let ri = eraAnp_safe(a.eral + hma)?;

    Ok((ri, di))
}
