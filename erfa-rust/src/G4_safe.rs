// G4
//   atci13.c   → eraAtci13_safe
//   atciq.c    → eraAtciq_safe
//   atciqn.c   → eraAtciqn_safe
//   atciqz.c   → eraAtciqz_safe
//   atco13.c   → eraAtco13_safe
//   atic13.c   → eraAtic13_safe
//   aticq.c    → eraAticq_safe
//   aticqn.c   → eraAticqn_safe
//   atio13.c   → eraAtio13_safe
//   atioq.c    → eraAtioq_safe

use crate::G1_safe::{eraAb_safe, eraAnp_safe, eraApci13_safe};
use crate::G20_safe::{eraLdn_safe, eraLdsun_safe};
use crate::G25_safe::eraPmpx_safe;
use crate::G28_safe::eraRxp_safe;
use crate::G29_safe::eraS2c_safe;
use crate::G2_safe::eraApco13_safe;
use crate::G33_safe::eraTrxp_safe;
use crate::G35_safe::eraZp_safe;
use crate::G3_safe::eraApio13_safe;
use crate::G7_safe::eraC2s_safe;
use crate::H1_safe::{eraASTROM, eraLDBODY};

pub type ErfaResult<T> = Result<T, ()>;

/*----------------------------------------------------------------------
 *  atci13.c  → eraAtci13_safe
 *--------------------------------------------------------------------*/

// ICRS catalog (J2000) to CIRS; returns (ri, di, eo).
pub fn eraAtci13_safe(
    rc: f64,
    dc: f64,
    pr: f64,
    pd: f64,
    px: f64,
    rv: f64,
    date1: f64,
    date2: f64,
) -> ErfaResult<(f64, f64, f64)> {
    let mut astrom = eraASTROM::default();
    let eo = eraApci13_safe(date1, date2, &mut astrom)?;
    let (ri, di) = eraAtciq_safe(rc, dc, pr, pd, px, rv, &astrom)?;
    Ok((ri, di, eo))
}

/*----------------------------------------------------------------------
 *  atciq.c  → eraAtciq_safe
 *--------------------------------------------------------------------*/

// ICRS catalog to CIRS given precomputed astrom; returns (ri, di).
pub fn eraAtciq_safe(
    rc: f64,
    dc: f64,
    pr: f64,
    pd: f64,
    px: f64,
    rv: f64,
    astrom: &eraASTROM,
) -> ErfaResult<(f64, f64)> {
    let a = astrom;

    // Proper motion & parallax to BCRS direction.
    let pco = eraPmpx_safe(rc, dc, pr, pd, px, rv, a.pmt, &a.eb)?;

    // Solar light deflection.
    let pnat = eraLdsun_safe(&pco, &a.eh, a.em)?;

    // Aberration.
    let ppr = eraAb_safe(&pnat, &a.v, a.em, a.bm1)?;

    // Rotate by BPN to CIRS.
    let pi = eraRxp_safe(&a.bpn, &ppr)?;

    // To spherical and wrap.
    let (w, di) = eraC2s_safe(&pi)?;
    let ri = eraAnp_safe(w)?;
    Ok((ri, di))
}

/*----------------------------------------------------------------------
 *  atciqn.c  → eraAtciqn_safe
 *--------------------------------------------------------------------*/

// ICRS catalog to CIRS including N-body deflection; returns (ri, di).
pub fn eraAtciqn_safe(
    rc: f64,
    dc: f64,
    pr: f64,
    pd: f64,
    px: f64,
    rv: f64,
    astrom: &eraASTROM,
    bodies: &[eraLDBODY],
) -> ErfaResult<(f64, f64)> {
    let a = astrom;

    // Proper motion & parallax to BCRS direction.
    let pco = eraPmpx_safe(rc, dc, pr, pd, px, rv, a.pmt, &a.eb)?;

    // N-body light deflection.
    let pnat = eraLdn_safe(bodies, &a.eb, &pco)?;

    // Aberration.
    let ppr = eraAb_safe(&pnat, &a.v, a.em, a.bm1)?;

    // Rotate by BPN to CIRS.
    let pi = eraRxp_safe(&a.bpn, &ppr)?;

    // To spherical and wrap.
    let (w, di) = eraC2s_safe(&pi)?;
    let ri = eraAnp_safe(w)?;
    Ok((ri, di))
}

/*----------------------------------------------------------------------
 *  atciqz.c  → eraAtciqz_safe
 *--------------------------------------------------------------------*/

// ICRS to CIRS for zero PM/parallax; returns (ri, di).
pub fn eraAtciqz_safe(rc: f64, dc: f64, astrom: &eraASTROM) -> ErfaResult<(f64, f64)> {
    let a = astrom;

    // Spherical to Cartesian.
    let pco = eraS2c_safe(rc, dc)?;

    // Solar deflection then aberration.
    let pnat = eraLdsun_safe(&pco, &a.eh, a.em)?;
    let ppr = eraAb_safe(&pnat, &a.v, a.em, a.bm1)?;

    // Rotate by BPN to CIRS.
    let pi = eraRxp_safe(&a.bpn, &ppr)?;

    // To spherical and wrap.
    let (w, di) = eraC2s_safe(&pi)?;
    let ri = eraAnp_safe(w)?;
    Ok((ri, di))
}

/*----------------------------------------------------------------------
 *  atco13.c  → eraAtco13_safe
 *--------------------------------------------------------------------*/

// ICRS catalog to observed place from UTC/site/weather; returns (aob, zob, hob, dob, rob, eo, j).
pub fn eraAtco13_safe(
    rc: f64,
    dc: f64,
    pr: f64,
    pd: f64,
    px: f64,
    rv: f64,
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
) -> ErfaResult<(f64, f64, f64, f64, f64, f64, i32)> {
    let mut astrom = eraASTROM::default();

    // Site-dependent astrometry params from UTC.
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

    // ICRS -> CIRS.
    let (ri, di) = eraAtciq_safe(rc, dc, pr, pd, px, rv, &astrom)?;

    // CIRS -> observed.
    let (aob, zob, hob, dob, rob) = eraAtioq_safe(ri, di, &astrom)?;
    Ok((aob, zob, hob, dob, rob, eo, j))
}

/*----------------------------------------------------------------------
 *  atic13.c  → eraAtic13_safe
 *--------------------------------------------------------------------*/

// CIRS astrometric to ICRS catalog using 2013 models; returns (rc, dc, eo).
pub fn eraAtic13_safe(ri: f64, di: f64, date1: f64, date2: f64) -> ErfaResult<(f64, f64, f64)> {
    let mut astrom = eraASTROM::default();
    let eo = eraApci13_safe(date1, date2, &mut astrom)?;
    let (rc, dc) = eraAticq_safe(ri, di, &astrom)?;
    Ok((rc, dc, eo))
}

/*----------------------------------------------------------------------
 *  aticq.c  → eraAticq_safe
 *--------------------------------------------------------------------*/

// CIRS astrometric to ICRS catalog given precomputed astrom; returns (rc, dc).
pub fn eraAticq_safe(ri: f64, di: f64, astrom: &eraASTROM) -> ErfaResult<(f64, f64)> {
    let a = astrom;

    // CIRS to Cartesian.
    let pi = eraS2c_safe(ri, di)?;

    // Bias-precession-nutation transpose to GCRS proper.
    let ppr = eraTrxp_safe(&a.bpn, &pi)?;

    // Aberration iteration (2 passes).
    let mut d = eraZp_safe();
    let mut pnat = [0.0_f64; 3];
    let mut before = [0.0_f64; 3];
    let mut after = [0.0_f64; 3];
    for _ in 0..2 {
        // before = unit(ppr - d)
        let mut r2 = 0.0;
        for i in 0..3 {
            let w = ppr[i] - d[i];
            before[i] = w;
            r2 += w * w;
        }
        let r = r2.sqrt();
        for v in before.iter_mut() {
            *v /= r;
        }

        // after = aberrated(before)
        let tmp = eraAb_safe(&before, &a.v, a.em, a.bm1)?;
        after.copy_from_slice(&tmp);

        // d = after - before; pnat = unit(ppr - d)
        let mut r2b = 0.0;
        for i in 0..3 {
            d[i] = after[i] - before[i];
            let w = ppr[i] - d[i];
            pnat[i] = w;
            r2b += w * w;
        }
        let rb = r2b.sqrt();
        for v in pnat.iter_mut() {
            *v /= rb;
        }
    }

    // Light deflection iteration (5 passes).
    let mut pco = [0.0_f64; 3];
    d = eraZp_safe();
    for _ in 0..5 {
        // before = unit(pnat - d)
        let mut r2 = 0.0;
        for i in 0..3 {
            let w = pnat[i] - d[i];
            before[i] = w;
            r2 += w * w;
        }
        let r = r2.sqrt();
        for v in before.iter_mut() {
            *v /= r;
        }

        // after = ldsun(before)
        let tmp = eraLdsun_safe(&before, &a.eh, a.em)?;
        after.copy_from_slice(&tmp);

        // d = after - before; pco = unit(pnat - d)
        let mut r2b = 0.0;
        for i in 0..3 {
            d[i] = after[i] - before[i];
            let w = pnat[i] - d[i];
            pco[i] = w;
            r2b += w * w;
        }
        let rb = r2b.sqrt();
        for v in pco.iter_mut() {
            *v /= rb;
        }
    }

    // To spherical and wrap.
    let (w, dc) = eraC2s_safe(&pco)?;
    let rc = eraAnp_safe(w)?;
    Ok((rc, dc))
}

/*----------------------------------------------------------------------
 *  aticqn.c  → eraAticqn_safe
 *--------------------------------------------------------------------*/

// CIRS astrometric to ICRS with n-body deflection; returns (rc, dc).
pub fn eraAticqn_safe(
    ri: f64,
    di: f64,
    astrom: &eraASTROM,
    bodies: &[eraLDBODY],
) -> ErfaResult<(f64, f64)> {
    let a = astrom;

    // CIRS to Cartesian and remove BPN.
    let pi = eraS2c_safe(ri, di)?;
    let ppr = eraTrxp_safe(&a.bpn, &pi)?;

    // Aberration iteration (2 passes).
    let mut d = eraZp_safe();
    let mut pnat = [0.0_f64; 3];
    let mut before = [0.0_f64; 3];
    let mut after = [0.0_f64; 3];
    for _ in 0..2 {
        let mut r2 = 0.0;
        for i in 0..3 {
            let w = ppr[i] - d[i];
            before[i] = w;
            r2 += w * w;
        }
        let r = r2.sqrt();
        for v in before.iter_mut() {
            *v /= r;
        }
        let tmp = eraAb_safe(&before, &a.v, a.em, a.bm1)?;
        after.copy_from_slice(&tmp);
        let mut r2b = 0.0;
        for i in 0..3 {
            d[i] = after[i] - before[i];
            let w = ppr[i] - d[i];
            pnat[i] = w;
            r2b += w * w;
        }
        let rb = r2b.sqrt();
        for v in pnat.iter_mut() {
            *v /= rb;
        }
    }

    // N-body deflection iteration (5 passes).
    let mut pco = [0.0_f64; 3];
    d = eraZp_safe();
    for _ in 0..5 {
        let mut r2 = 0.0;
        for i in 0..3 {
            let w = pnat[i] - d[i];
            before[i] = w;
            r2 += w * w;
        }
        let r = r2.sqrt();
        for v in before.iter_mut() {
            *v /= r;
        }
        let tmp = eraLdn_safe(bodies, &a.eb, &before)?;
        after.copy_from_slice(&tmp);
        let mut r2b = 0.0;
        for i in 0..3 {
            d[i] = after[i] - before[i];
            let w = pnat[i] - d[i];
            pco[i] = w;
            r2b += w * w;
        }
        let rb = r2b.sqrt();
        for v in pco.iter_mut() {
            *v /= rb;
        }
    }

    // To spherical and wrap.
    let (w, dc) = eraC2s_safe(&pco)?;
    let rc = eraAnp_safe(w)?;
    Ok((rc, dc))
}

/*----------------------------------------------------------------------
 *  atio13.c  → eraAtio13_safe
 *--------------------------------------------------------------------*/

// CIRS astrometric to observed place from UTC/site/weather; returns (aob, zob, hob, dob, rob, j).
pub fn eraAtio13_safe(
    ri: f64,
    di: f64,
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
) -> ErfaResult<(f64, f64, f64, f64, f64, i32)> {
    let mut astrom = eraASTROM::default();
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
    let (aob, zob, hob, dob, rob) = eraAtioq_safe(ri, di, &astrom)?;
    Ok((aob, zob, hob, dob, rob, j))
}

/*----------------------------------------------------------------------
 *  atioq.c  → eraAtioq_safe
 *--------------------------------------------------------------------*/

// CIRS (ri,di) to observed (az,zd,HA,Dec,RA) given precomputed astrom; returns (aob, zob, hob, dob, rob).
pub fn eraAtioq_safe(
    ri: f64,
    di: f64,
    astrom: &eraASTROM,
) -> ErfaResult<(f64, f64, f64, f64, f64)> {
    const CELMIN: f64 = 1e-6;
    const SELMIN: f64 = 0.05;

    let a = astrom;

    // CIRS -> HA,Dec unit vector in topocentric frame.
    let v = eraS2c_safe(ri - a.eral, di)?;
    let x = v[0];
    let y = v[1];
    let mut z = v[2];

    // Polar motion rotation (X then Y).
    let (sx, cx) = a.xpl.sin_cos();
    let (sy, cy) = a.ypl.sin_cos();
    let xhd = cx * x + sx * z;
    let yhd = sx * sy * x + cy * y - cx * sy * z;
    let zhd = -sx * cy * x + sy * y + cx * cy * z;

    // Diurnal aberration.
    let f = 1.0 - a.diurab * yhd;
    let xhdt = f * xhd;
    let yhdt = f * (yhd + a.diurab);
    let zhdt = f * zhd;

    // Latitude rotation.
    let xaet = a.sphi * xhdt - a.cphi * zhdt;
    let yaet = yhdt;
    let zaet = a.cphi * xhdt + a.sphi * zhdt;

    // Azimuth.
    let azobs = if xaet != 0.0 || yaet != 0.0 {
        yaet.atan2(-xaet)
    } else {
        0.0
    };

    // Elevation with floors to avoid extreme refraction.
    let mut r = (xaet * xaet + yaet * yaet).sqrt();
    r = if r > CELMIN { r } else { CELMIN };
    z = if zaet > SELMIN { zaet } else { SELMIN };

    // Refraction: apply to tangent of zenith distance.
    let tz = r / z;
    let w = a.refb * tz * tz;
    let del = (a.refa + w) * tz / (1.0 + (a.refa + 3.0 * w) / (z * z));

    // Undo to get observed az/el vector.
    let cosdel = 1.0 - del * del / 2.0;
    let f2 = cosdel - del * z / r;
    let xaeo = xaet * f2;
    let yaeo = yaet * f2;
    let zaeo = cosdel * zaet + del * r;

    // Zenith distance.
    let zdobs = ((xaeo * xaeo + yaeo * yaeo).sqrt()).atan2(zaeo);

    // Undo latitude rotation.
    let vx = a.sphi * xaeo + a.cphi * zaeo;
    let vy = yaeo;
    let vz = -a.cphi * xaeo + a.sphi * zaeo;
    let v_obs = [vx, vy, vz];

    // To spherical: hour angle (negative sign) and declination.
    let (hmobs, dcobs) = eraC2s_safe(&v_obs)?;

    // Compute RA only after hmobs is known.
    let raobs = a.eral + hmobs;

    // Wrap azimuth and RA into canonical ranges.
    let aob = eraAnp_safe(azobs)?;
    let zob = zdobs;
    let hob = -hmobs;
    let dob = dcobs;
    let rob = eraAnp_safe(raobs)?;
    Ok((aob, zob, hob, dob, rob))
}
