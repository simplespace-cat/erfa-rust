// G10
//   dtdb.c  → eraDtdb_safe
//   dtf2d.c → eraDtf2d_safe

use crate::G19_safe::eraJd2cal_safe;
use crate::G8_safe::eraCal2jd_safe;
use crate::G9_safe::eraDat_safe;
use crate::H1_safe::{ERFA_D2PI, ERFA_DAYSEC, ERFA_DD2R, ERFA_DJ00, ERFA_DJM};

#[path = "data/G10_safe/FAIRHD.rs"]
mod fairhd_mod;
use fairhd_mod::FAIRHD;

pub type ErfaResult<T> = Result<T, ()>;

#[inline]
fn fmod(a: f64, b: f64) -> f64 {
    // Emulate C fmod with correct sign for negatives
    a - (a / b).trunc() * b
}

// Geocentric model for TDB − TT (seconds).
pub fn eraDtdb_safe(
    date1: f64,
    date2: f64,
    ut: f64,
    elong: f64,
    u: f64,
    v: f64,
) -> ErfaResult<f64> {
    // Time since J2000.0 in Julian millennia
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJM;

    // Topocentric part (Moyer/Murray)
    let tsol = fmod(ut, 1.0) * ERFA_D2PI + elong;

    // Fundamental arguments  Simon et al. 1994
    let w = t / 3600.0;

    // Mean longitudes and anomalies (radians)
    let elsun = fmod(280.466_456_83 + 1_296_027_711.034_29 * w, 360.0) * ERFA_DD2R;
    let emsun = fmod(357.529_109_18 + 1_295_965_810.481_00 * w, 360.0) * ERFA_DD2R;
    let d = fmod(297.850_195_47 + 16_029_616_012.090_00 * w, 360.0) * ERFA_DD2R;
    let elj = fmod(34.351_518_74 + 109_306_899.894_53 * w, 360.0) * ERFA_DD2R;
    let els = fmod(50.077_444_30 + 44_046_398.470_38 * w, 360.0) * ERFA_DD2R;

    // Moyer + Murray topocentric terms
    let wt = 0.000_29e-10 * u * (tsol + elsun - els).sin()
        + 0.001_00e-10 * u * (tsol - 2.0 * emsun).sin()
        + 0.001_33e-10 * u * (tsol - d).sin()
        + 0.001_33e-10 * u * (tsol + elsun - elj).sin()
        - 0.002_29e-10 * u * (tsol + 2.0 * elsun + emsun).sin()
        - 0.022_00e-10 * v * (elsun + emsun).cos()
        + 0.053_12e-10 * u * (tsol - emsun).sin()
        - 0.136_77e-10 * u * (tsol + 2.0 * elsun).sin()
        - 1.318_40e-10 * v * elsun.cos()
        + 3.176_79e-10 * u * tsol.sin();

    // Fairhead & Bretagnon (geocentric): T^0 .. T^4 harmonic sums
    let mut w0 = 0.0;
    for j in (0..=473).rev() {
        let row = FAIRHD[j];
        w0 += row[0] * (row[1] * t + row[2]).sin();
    }
    let mut w1 = 0.0;
    for j in (474..=678).rev() {
        let row = FAIRHD[j];
        w1 += row[0] * (row[1] * t + row[2]).sin();
    }
    let mut w2 = 0.0;
    for j in (679..=763).rev() {
        let row = FAIRHD[j];
        w2 += row[0] * (row[1] * t + row[2]).sin();
    }
    let mut w3 = 0.0;
    for j in (764..=783).rev() {
        let row = FAIRHD[j];
        w3 += row[0] * (row[1] * t + row[2]).sin();
    }
    let mut w4 = 0.0;
    for j in (784..=786).rev() {
        let row = FAIRHD[j];
        w4 += row[0] * (row[1] * t + row[2]).sin();
    }

    // Combine powers of T
    let wf = t * (t * (t * (t * w4 + w3) + w2) + w1) + w0;

    // JPL-mass adjustments
    let wj = 0.000_65e-6 * (6_069.776_754 * t + 4.021_194).sin()
        + 0.000_33e-6 * (213.299_095 * t + 5.543_132).sin()
        - 0.001_96e-6 * (6_208.294_251 * t + 5.696_701).sin()
        - 0.001_73e-6 * (74.781_599 * t + 2.435_900).sin()
        + 0.036_38e-6 * t * t;

    // Final result: TDB − TT (seconds)
    Ok(wt + wf + wj)
}

// Calendar + clock to two-part Julian Date/quasi-JD.
pub fn eraDtf2d_safe(
    scale: &str,
    iy: i32,
    im: i32,
    id: i32,
    ihr: i32,
    imn: i32,
    sec: f64,
) -> Result<((f64, f64), i32), i32> {
    // Convert calendar date to JD (0h today)
    let ((djm0, djm), j_cal) = eraCal2jd_safe(iy, im, id).map_err(|_| -9999)?;
    if j_cal != 0 {
        return Err(j_cal);
    }
    let dj = djm0 + djm;

    // Default day length and final-minute seconds
    let mut day = ERFA_DAYSEC;
    let mut seclim = 60.0_f64;

    // Leap-second handling for UTC only (exact match)
    if scale == "UTC" {
        // TAI−UTC at 0h today
        let (dat0, j0) = eraDat_safe(iy, im, id, 0.0).map_err(|_| -9999)?;
        if j0 < 0 {
            return Err(j0);
        }

        // TAI−UTC at 12h today
        let (dat12, j12) = eraDat_safe(iy, im, id, 0.5).map_err(|_| -9999)?;
        if j12 < 0 {
            return Err(j12);
        }

        // TAI−UTC at 0h tomorrow
        let ((iy2, im2, id2), _w, j_next) = eraJd2cal_safe(dj + 1.5, 0.0).map_err(|_| -9999)?;
        if j_next != 0 {
            return Err(j_next);
        }
        let (dat24, j24) = eraDat_safe(iy2, im2, id2, 0.0).map_err(|_| -9999)?;
        if j24 < 0 {
            return Err(j24);
        }

        // Leap-second increment for today
        let dleap = dat24 - (2.0 * dat12 - dat0);

        // Adjust day and final-minute seconds on leap-second days
        day += dleap;
        if ihr == 23 && imn == 59 {
            seclim += dleap;
        }
    }

    // Validate clock fields and produce warning if time after end-of-day
    let mut js_warn = 0;
    if (0..=23).contains(&ihr) {
        if (0..=59).contains(&imn) {
            if sec >= 0.0 {
                if sec >= seclim {
                    js_warn += 2;
                }
            } else {
                return Err(-6);
            }
        } else {
            return Err(-5);
        }
    } else {
        return Err(-4);
    }

    // Fraction of the day
    let time = (60.0 * (60.0 * ihr as f64 + imn as f64) + sec) / day;

    // Return ((d1, d2), j) with j = 0 or +2
    Ok(((dj, time), js_warn))
}
