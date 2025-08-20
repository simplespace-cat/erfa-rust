// G18
//   h2fk5.c  → eraH2fk5_safe
//   hd2ae.c  → eraHd2ae_safe
//   hd2pa.c  → eraHd2pa_safe
//   hfk5z.c  → eraHfk5z_safe

use crate::G15_safe::eraFk5hip_safe;
use crate::G1_safe::eraAnp_safe;
use crate::G25_safe::eraPmp_safe;
use crate::G27_safe::{eraPv2s_safe, eraPvstar_safe, eraPxp_safe};
use crate::G28_safe::{eraRv2m_safe, eraRxp_safe, eraRxr_safe};
use crate::G29_safe::eraS2c_safe;
use crate::G30_safe::{eraStarpv_safe, eraSxp_safe};
use crate::G33_safe::eraTrxp_safe;
use crate::H1_safe::{ERFA_D2PI, ERFA_DJ00, ERFA_DJY};

pub type ErfaResult<T> = Result<T, ()>;

//----------------------------------------------------------------------
// G18/h2fk5.c → eraH2fk5_safe
//----------------------------------------------------------------------
// Convert Hipparcos catalog parameters to FK5 J2000 parameters.
pub fn eraH2fk5_safe(
    rh: f64,
    dh: f64,
    drh: f64,
    ddh: f64,
    pxh: f64,
    rvh: f64,
) -> ErfaResult<(f64, f64, f64, f64, f64, f64)> {
    // Hipparcos barycentric position/velocity (normalized)
    let (pvh, _warn) = eraStarpv_safe(rh, dh, drh, ddh, pxh, rvh)?;

    // FK5↔Hipparcos rotation and spin (spin in rad/yr)
    let (r5h_m, mut s5h) = eraFk5hip_safe()?;

    // Spin: yr⁻¹ → day⁻¹
    for v in &mut s5h {
        *v /= 365.25;
    }

    // Spin expressed in Hipparcos frame: sh = R * s5h
    let sh = eraRxp_safe(&r5h_m, &s5h)?;

    // De-orient position: pv5.pos = R^T * pvh.pos
    let mut pv5 = [[0.0_f64; 3]; 2];
    pv5[0] = eraTrxp_safe(&r5h_m, &pvh[0])?;

    // Extra motion from spin: wxp = pvh.pos × sh
    let wxp = eraPxp_safe(&pvh[0], &sh)?;

    // Remove spin component from velocity: vv = pvh.vel − wxp
    let vv = eraPmp_safe(&pvh[1], &wxp)?;

    // De-orient velocity: pv5.vel = R^T * vv
    pv5[1] = eraTrxp_safe(&r5h_m, &vv)?;

    // FK5 pv-vector → catalog fields
    let ((r5, d5, dr5, dd5, px5, rv5), _j) = eraPvstar_safe(&pv5)?;
    Ok((r5, d5, dr5, dd5, px5, rv5))
}

//----------------------------------------------------------------------
// G18/hd2ae.c → eraHd2ae_safe
//----------------------------------------------------------------------
// Convert hour angle/declination and latitude to azimuth/elevation.
pub fn eraHd2ae_safe(ha: f64, dec: f64, phi: f64) -> ErfaResult<(f64, f64)> {
    let (sh, ch) = ha.sin_cos();
    let (sd, cd) = dec.sin_cos();
    let (sp, cp) = phi.sin_cos();

    // Horizon-system unit vector
    let x = -ch * cd * sp + sd * cp;
    let y = -sh * cd;
    let z = ch * cd * cp + sd * sp;

    // Spherical coordinates
    let r = (x * x + y * y).sqrt();
    let mut a = if r != 0.0 { y.atan2(x) } else { 0.0 };
    if a < 0.0 {
        a += ERFA_D2PI;
    }

    let az = a;
    let el = z.atan2(r);
    Ok((az, el))
}

//----------------------------------------------------------------------
// G18/hd2pa.c → eraHd2pa_safe
//----------------------------------------------------------------------
// Compute parallactic angle for hour angle/declination and latitude.
pub fn eraHd2pa_safe(ha: f64, dec: f64, phi: f64) -> ErfaResult<f64> {
    let cp = phi.cos();
    let sqsz = cp * ha.sin();
    let cqsz = phi.sin() * dec.cos() - cp * dec.sin() * ha.cos();
    let pa = if sqsz != 0.0 || cqsz != 0.0 {
        sqsz.atan2(cqsz)
    } else {
        0.0
    };
    Ok(pa)
}

//----------------------------------------------------------------------
// G18/hfk5z.c → eraHfk5z_safe
//----------------------------------------------------------------------
// Convert Hipparcos position (zero proper motion) to FK5 at given date.
pub fn eraHfk5z_safe(rh: f64, dh: f64, date1: f64, date2: f64) -> ErfaResult<(f64, f64, f64, f64)> {
    // Interval from J2000.0 in Julian years
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJY;

    // Hipparcos unit vector
    let ph = eraS2c_safe(rh, dh)?;

    // FK5↔Hipparcos rotation and spin (spin in rad/yr)
    let (r5h_m, s5h) = eraFk5hip_safe()?;

    // Spin rotated into Hipparcos: sh = R * s5h
    let sh = eraRxp_safe(&r5h_m, &s5h)?;

    // Accumulated spin over interval (vector): vst = t  s5h
    let vst = eraSxp_safe(t, &s5h)?;

    // Vector → rotation matrix
    let rst = eraRv2m_safe(&vst)?;

    // Total rotation: r5ht = R × rst
    let r5ht = eraRxr_safe(&r5h_m, &rst)?;

    // De-orient + de-spin position: pv5e.pos = r5ht^T * ph
    let mut pv5e = [[0.0_f64; 3]; 2];
    pv5e[0] = eraTrxp_safe(&r5ht, &ph)?;

    // Space motion due to spin: vv = sh × ph
    let vv = eraPxp_safe(&sh, &ph)?;

    // De-orient + de-spin velocity: pv5e.vel = r5ht^T * vv
    pv5e[1] = eraTrxp_safe(&r5ht, &vv)?;

    // PV → spherical (theta, phi, r, td, pd, rd)
    let (w, d5, _r, dr5, dd5, _rd) = eraPv2s_safe(&pv5e)?;
    let r5 = eraAnp_safe(w)?;
    Ok((r5, d5, dr5, dd5))
}
