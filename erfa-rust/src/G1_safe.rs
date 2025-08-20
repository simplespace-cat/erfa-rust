// G1
//   a2af.c       → eraA2af_safe
//   a2tf.c       → eraA2tf_safe
//   ab.c         → eraAb_safe
//   ae2hd.c      → eraAe2hd_safe
//   af2a.c       → eraAf2a_safe
//   anp.c        → eraAnp_safe
//   anpm.c       → eraAnpm_safe
//   apcg.c       → eraApcg_safe
//   apcg13.c     → eraApcg13_safe
//   apci.c       → eraApci_safe
//   apci13.c     → eraApci13_safe


use crate::G11_safe::eraEors_safe;
use crate::G13_safe::eraEpv00_safe;
use crate::G24_safe::eraPdp_safe;
use crate::G26_safe::eraPnm06a_safe;
use crate::G2_safe::eraApcs_safe;
use crate::G30_safe::eraS06_safe;
use crate::G6_safe::eraBpn2xy_safe;
use crate::G7_safe::eraC2ixys_safe;
use crate::G9_safe::eraD2tf_safe;
use crate::H1_safe::{eraASTROM, ERFA_D2PI, ERFA_DAS2R, ERFA_DPI, ERFA_SRS};

pub type ErfaResult<T> = Result<T, ()>;

//----------------------------------------------------------------------
//  G1/a2af.c
//----------------------------------------------------------------------

// Convert radians to sign and DMS fields with C-compatible rounding.
pub fn eraA2af_safe(ndp: i32, angle: f64) -> ErfaResult<(char, [i32; 4])> {
    // Hours to degrees × radians to turns factor as in ERFA.
    const F: f64 = 15.0 / ERFA_D2PI;
    let (sign, idmsf) = eraD2tf_safe(ndp, angle * F)?;
    Ok((sign, idmsf))
}

//----------------------------------------------------------------------
//  G1/a2tf.c
//----------------------------------------------------------------------

// Convert radians to sign and HMS fields with C-compatible rounding.
pub fn eraA2tf_safe(ndp: i32, angle: f64) -> ErfaResult<(char, [i32; 4])> {
    let (sign, ihmsf) = eraD2tf_safe(ndp, angle / ERFA_D2PI)?;
    Ok((sign, ihmsf))
}

//----------------------------------------------------------------------
//  G1/ab.c
//----------------------------------------------------------------------

// Stellar aberration: return unit vector after aberration correction.
pub fn eraAb_safe(pnat: &[f64; 3], v: &[f64; 3], s: f64, bm1: f64) -> ErfaResult<[f64; 3]> {
    // Dot product of pnat and v.
    let pdv = eraPdp_safe(pnat, v)?;

    // Terms per ERFA ab.c
    let w1 = 1.0 + pdv / (1.0 + bm1);
    let w2 = ERFA_SRS / s;

    let mut r2 = 0.0;
    let mut p = [0.0_f64; 3];
    for i in 0..3 {
        let pnat_i = pnat[i];
        let v_i = v[i];
        let w = pnat_i * bm1 + w1 * v_i + w2 * (v_i - pdv * pnat_i);
        p[i] = w;
        r2 += w * w;
    }

    let r = r2.sqrt();
    let mut ppr = [0.0_f64; 3];
    for i in 0..3 {
        ppr[i] = p[i] / r;
    }
    Ok(ppr)
}

//----------------------------------------------------------------------
//  G1/ae2hd.c
//----------------------------------------------------------------------

// Convert azimuth/elevation to hour angle/declination (radians).
pub fn eraAe2hd_safe(az: f64, el: f64, phi: f64) -> ErfaResult<(f64, f64)> {
    let (sa, ca) = az.sin_cos();
    let (se, ce) = el.sin_cos();
    let (sp, cp) = phi.sin_cos();

    // HA,Dec unit vector.
    let x = -ca * ce * sp + se * cp;
    let y = -sa * ce;
    let z = ca * ce * cp + se * sp;

    // To spherical.
    let r = (x * x + y * y).sqrt();
    let ha = if r != 0.0 { y.atan2(x) } else { 0.0 };
    let dec = z.atan2(r);

    Ok((ha, dec))
}

//----------------------------------------------------------------------
//  G1/af2a.c
//----------------------------------------------------------------------

// Convert sign and DMS fields to radians; return status code per ERFA.
pub fn eraAf2a_safe(s: char, ideg: i32, iamin: i32, asec: f64) -> ErfaResult<(f64, i32)> {
    // Magnitude.
    let mag =
        (60.0 * (60.0 * (ideg.abs() as f64) + (iamin.abs() as f64)) + asec.abs()) * ERFA_DAS2R;
    let rad = if s == '-' { -mag } else { mag };

    // Range checks.
    let status = if ideg < 0 || ideg > 359 {
        1
    } else if iamin < 0 || iamin > 59 {
        2
    } else if asec < 0.0 || asec >= 60.0 {
        3
    } else {
        0
    };

    Ok((rad, status))
}

//----------------------------------------------------------------------
//  G1/anp.c
//----------------------------------------------------------------------

// Normalize angle into [0, 2π).
pub fn eraAnp_safe(a: f64) -> ErfaResult<f64> {
    let mut w = a % ERFA_D2PI;
    if w < 0.0 {
        w += ERFA_D2PI;
    }
    Ok(w)
}

//----------------------------------------------------------------------
//  G1/anpm.c
//----------------------------------------------------------------------

// Normalize angle into (-π, +π].
pub fn eraAnpm_safe(a: f64) -> ErfaResult<f64> {
    let mut w = a % ERFA_D2PI;
    if w.abs() >= ERFA_DPI {
        // Subtract 2π with the sign of a (DSIGN equivalent).
        w -= ERFA_D2PI.copysign(a);
    }
    Ok(w)
}

//----------------------------------------------------------------------
//  G1/apcg.c
//----------------------------------------------------------------------

// Fill astrom for a geocentric observer (PV at origin).
pub fn eraApcg_safe(
    date1: f64,
    date2: f64,
    ebpv: &[[f64; 3]; 2],
    ehp: &[f64; 3],
    astrom: &mut eraASTROM,
) -> ErfaResult<()> {
    // Geocenter PV = 0.
    let pv = [[0.0_f64; 3]; 2];
    eraApcs_safe(date1, date2, &pv, ebpv, ehp, astrom)?;
    Ok(())
}

//----------------------------------------------------------------------
//  G1/apcg13.c
//----------------------------------------------------------------------

// Fill astrom using IAU 2006/2000A star-independent parameters.
pub fn eraApcg13_safe(date1: f64, date2: f64, astrom: &mut eraASTROM) -> ErfaResult<()> {
    // Earth barycentric/heliocentric PV (au, au/day).
    let (ehpv, ebpv, _jstat) = eraEpv00_safe(date1, date2)?;
    // Use heliocentric position as 'ehp' input.
    eraApcg_safe(date1, date2, &ebpv, &ehpv[0], astrom)
}

//----------------------------------------------------------------------
//  G1/apci.c
//----------------------------------------------------------------------

// CIO-based astrometry parameters with supplied X,Y,s; updates astrom.bpn.
pub fn eraApci_safe(
    date1: f64,
    date2: f64,
    ebpv: &[[f64; 3]; 2],
    ehp: &[f64; 3],
    x: f64,
    y: f64,
    s: f64,
    astrom: &mut eraASTROM,
) -> ErfaResult<()> {
    // Geocenter parameters.
    eraApcg_safe(date1, date2, ebpv, ehp, astrom)?;

    // Build CIO-based BPN matrix and assign.
    let rc2i = eraC2ixys_safe(x, y, s)?;
    astrom.bpn = rc2i;
    Ok(())
}

//----------------------------------------------------------------------
//  G1/apci13.c
//----------------------------------------------------------------------

// Compute astrometry parameters and return equation of origins (eo).
pub fn eraApci13_safe(date1: f64, date2: f64, astrom: &mut eraASTROM) -> ErfaResult<f64> {
    // Earth PV (au, au/day).
    let (ehpv, ebpv, _jstat) = eraEpv00_safe(date1, date2)?;

    // NPB matrix, IAU 2006/2000A.
    let r = eraPnm06a_safe(date1, date2)?;

    // CIP X,Y from NPB.
    let (x, y) = eraBpn2xy_safe(&r)?;

    // CIO locator s.
    let s = eraS06_safe(date1, date2, x, y)?;

    // Astrometry parameters.
    eraApci_safe(date1, date2, &ebpv, &ehpv[0], x, y, s, astrom)?;

    // Equation of origins.
    let eo = eraEors_safe(&r, s)?;
    Ok(eo)
}
