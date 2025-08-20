// G33
//   tr.c      → eraTr_safe
//   trxp.c    → eraTrxp_safe
//   trxpv.c   → eraTrxpv_safe
//   tttai.c   → eraTttai_safe
//   tttcg.c   → eraTttcg_safe
//   tttdb.c   → eraTttdb_safe
//   ttut1.c   → eraTtut1_safe
//   ut1tai.c  → eraUt1tai_safe
//   ut1tt.c   → eraUt1tt_safe
//   ut1utc.c  → eraUt1utc_safe
//   utctai.c  → eraUtctai_safe
//   utcut1.c  → eraUtcut1_safe

use crate::G19_safe::eraJd2cal_safe;
use crate::G28_safe::{eraRxp_safe, eraRxpv_safe};
use crate::G32_safe::eraTaiut1_safe;
use crate::G8_safe::{eraCal2jd_safe, eraCr_safe};
use crate::G9_safe::eraDat_safe;
use crate::H1_safe::{ERFA_DAYSEC, ERFA_DJM0, ERFA_DJM77, ERFA_ELG, ERFA_TTMTAI};

pub type ErfaResult<T> = Result<T, ()>;

// Transpose 3×3 matrix.
pub fn eraTr_safe(r: &[[f64; 3]; 3]) -> ErfaResult<[[f64; 3]; 3]> {
    let mut wm = [[0.0_f64; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            wm[i][j] = r[j][i];
        }
    }
    // Copy to result (preserve original helper usage semantics)
    let mut rt = [[0.0_f64; 3]; 3];
    eraCr_safe(&wm, &mut rt)?;
    Ok(rt)
}

// Transpose(r) × p-vector.
pub fn eraTrxp_safe(r: &[[f64; 3]; 3], p: &[f64; 3]) -> ErfaResult<[f64; 3]> {
    let tr = eraTr_safe(r)?;
    eraRxp_safe(&tr, p)
}

// Transpose(r) × pv-vector.
pub fn eraTrxpv_safe(r: &[[f64; 3]; 3], pv: &[[f64; 3]; 2]) -> ErfaResult<[[f64; 3]; 2]> {
    let tr = eraTr_safe(r)?;
    eraRxpv_safe(&tr, pv)
}

// TT → TAI.
pub fn eraTttai_safe(tt1: f64, tt2: f64) -> ErfaResult<((f64, f64), i32)> {
    let dtat = ERFA_TTMTAI / ERFA_DAYSEC;
    let (tai1, tai2) = if tt1.abs() > tt2.abs() {
        (tt1, tt2 - dtat)
    } else {
        (tt1 - dtat, tt2)
    };
    Ok(((tai1, tai2), 0))
}

// TT → TCG.
pub fn eraTttcg_safe(tt1: f64, tt2: f64) -> ErfaResult<((f64, f64), i32)> {
    const T77T: f64 = ERFA_DJM77 + ERFA_TTMTAI / ERFA_DAYSEC; // 1977-Jan-1 00:00:32.184 TT
    const ELGG: f64 = ERFA_ELG / (1.0 - ERFA_ELG); // TT→TCG rate

    let (tcg1, tcg2) = if tt1.abs() > tt2.abs() {
        (tt1, tt2 + ((tt1 - ERFA_DJM0) + (tt2 - T77T)) * ELGG)
    } else {
        (tt1 + ((tt2 - ERFA_DJM0) + (tt1 - T77T)) * ELGG, tt2)
    };
    Ok(((tcg1, tcg2), 0))
}

// TT → TDB using caller-supplied dtr = TDB−TT seconds.
pub fn eraTttdb_safe(tt1: f64, tt2: f64, dtr: f64) -> ErfaResult<((f64, f64), i32)> {
    let dtrd = dtr / ERFA_DAYSEC;
    let (tdb1, tdb2) = if tt1.abs() > tt2.abs() {
        (tt1, tt2 + dtrd)
    } else {
        (tt1 + dtrd, tt2)
    };
    Ok(((tdb1, tdb2), 0))
}

// TT → UT1 (dt = TT−UT1 seconds).
pub fn eraTtut1_safe(tt1: f64, tt2: f64, dt: f64) -> ErfaResult<((f64, f64), i32)> {
    let dtd = dt / ERFA_DAYSEC;
    let (ut11, ut12) = if tt1.abs() > tt2.abs() {
        (tt1, tt2 - dtd)
    } else {
        (tt1 - dtd, tt2)
    };
    Ok(((ut11, ut12), 0))
}

// UT1 → TAI (dta = UT1−TAI seconds).
pub fn eraUt1tai_safe(ut11: f64, ut12: f64, dta: f64) -> ErfaResult<((f64, f64), i32)> {
    let dtad = dta / ERFA_DAYSEC;
    let (tai1, tai2) = if ut11.abs() > ut12.abs() {
        (ut11, ut12 - dtad)
    } else {
        (ut11 - dtad, ut12)
    };
    Ok(((tai1, tai2), 0))
}

// UT1 → TT (dt = TT−UT1 seconds).
pub fn eraUt1tt_safe(ut11: f64, ut12: f64, dt: f64) -> ErfaResult<((f64, f64), i32)> {
    let dtd = dt / ERFA_DAYSEC;
    let (tt1, tt2) = if ut11.abs() > ut12.abs() {
        (ut11, ut12 + dtd)
    } else {
        (ut11 + dtd, ut12)
    };
    Ok(((tt1, tt2), 0))
}

// eraUt1utc_safe  UT1 → UTC (handles leap-second wrinkles)
// Returns: ((utc1, utc2), js) where js = 0 OK, +1 dubious year, -1 error
pub fn eraUt1utc_safe(ut11: f64, ut12: f64, dut1: f64) -> ErfaResult<((f64, f64), i32)> {
    // Arrange inputs big-first
    let big1 = ut11.abs() >= ut12.abs();
    let (u1, mut u2) = if big1 { (ut11, ut12) } else { (ut12, ut11) };

    // Start with caller-supplied UT1-UTC.
    let mut duts = dut1;

    // Variables used inside the leap-second detection loop.
    let d1 = u1; // first JD part (big)
    let mut dats1: f64 = 0.0; // TAI-UTC at i = 1
    let mut js: i32 = 0; // status from eraDat_safe

    // Scan for possible leap-second day: from -1 to +3 days of UT1.
    for i in -1..=3 {
        let d2 = u2 + i as f64;

        // Convert candidate day to calendar date.
        // NOTE: fractional day '_fd' is intentionally unused in this path.
        let ((iy, im, id), _fd, jcal) = eraJd2cal_safe(d1, d2)?;
        if jcal != 0 {
            return Ok(((0.0, 0.0), -1));
        }

        // TAI-UTC at 0h of this candidate day.
        let (dats2, jdat) = eraDat_safe(iy, im, id, 0.0)?;
        if jdat < 0 {
            return Ok(((0.0, 0.0), -1));
        }
        js = jdat; // track dubious year flag if any

        // Record first value (i = 1).
        if i == -1 {
            dats1 = dats2;
        }

        // Difference between consecutive days TAI-UTC.
        let ddats = dats2 - dats1;

        // Jump of ≥0.5 s implies leap-second boundary.
        if ddats.abs() >= 0.5 {
            // Ensure duts is the before value.
            if ddats * duts >= 0.0 {
                duts -= ddats;
            }

            // JD(UTC) of 0h UTC that ends with the leap second.
            let ((us1, mut us2), jcz) = eraCal2jd_safe(iy, im, id)?;
            if jcz != 0 {
                return Ok(((0.0, 0.0), -1));
            }
            // Subtract 1 day then add current duts (now before value).
            us2 = us2 - 1.0 + duts / ERFA_DAYSEC;

            // How far into the current UTC day is the given UT1?
            let mut du = u1 - us1;
            du += u2 - us2;

            if du > 0.0 {
                // Fraction of the UTC day that has elapsed.
                let fd = du * ERFA_DAYSEC / (ERFA_DAYSEC + ddats);
                // Ramp UT1-UTC linearly during the leap second.
                duts += ddats * if fd <= 1.0 { fd } else { 1.0 };
            }

            // Leap-second processing finished.
            break;
        }

        // Prepare for next iteration.
        dats1 = dats2;
    }

    // Subtract (possibly adjusted) UT1-UTC to obtain UTC.
    u2 -= duts / ERFA_DAYSEC;

    // Restore original part ordering for result.
    let (utc1, utc2) = if big1 { (u1, u2) } else { (u2, u1) };
    Ok(((utc1, utc2), js))
}

// UTC → TAI (with leap-second handling).
pub fn eraUtctai_safe(utc1: f64, utc2: f64) -> ErfaResult<((f64, f64), i32)> {
    let big1 = utc1.abs() >= utc2.abs();
    let (u1, u2) = if big1 { (utc1, utc2) } else { (utc2, utc1) };

    // Calendar for UTC
    let ((iy, im, id), mut fd, jcal) = eraJd2cal_safe(u1, u2)?;
    if jcal != 0 {
        return Ok(((0.0, 0.0), jcal));
    }

    // TAI-UTC at 0h
    let (dat0, mut j) = eraDat_safe(iy, im, id, 0.0)?;
    if j < 0 {
        return Ok(((0.0, 0.0), j));
    }

    let (dat12, j12) = eraDat_safe(iy, im, id, 0.5)?;
    if j12 < 0 {
        return Ok(((0.0, 0.0), j12));
    }

    // TAI-UTC at 24h (next day)
    let ((iyt, imt, idt), _w, jcal2) = eraJd2cal_safe(u1 + 1.5, u2 - fd)?;
    if jcal2 != 0 {
        return Ok(((0.0, 0.0), jcal2));
    }
    let (dat24, j24) = eraDat_safe(iyt, imt, idt, 0.0)?;
    if j24 < 0 {
        return Ok(((0.0, 0.0), j24));
    }
    j = j24;

    // Interpolate for any day duration changes
    let dlod = 2.0 * (dat12 - dat0);
    let dleap = dat24 - (dat0 + dlod);

    fd *= (ERFA_DAYSEC + dleap) / ERFA_DAYSEC;
    fd *= (ERFA_DAYSEC + dlod) / ERFA_DAYSEC;

    // Build TAI parts
    let ((z1, z2), jcz) = eraCal2jd_safe(iy, im, id)?;
    if jcz != 0 {
        return Ok(((0.0, 0.0), -1));
    }
    let a2 = z1 - u1 + z2 + fd + dat0 / ERFA_DAYSEC;

    let (tai1, tai2) = if big1 { (u1, a2) } else { (a2, u1) };
    Ok(((tai1, tai2), j))
}

// eraUtcut1_safe  UTC → UT1
// Returns: ((ut11, ut12), js) where js=0 OK, +1 dubious year, -1 error
pub fn eraUtcut1_safe(utc1: f64, utc2: f64, dut1: f64) -> ErfaResult<((f64, f64), i32)> {
    // Date
    let ((iy, im, id), _w, jcal) = eraJd2cal_safe(utc1, utc2)?;
    if jcal != 0 {
        return Ok(((0.0, 0.0), -1));
    }

    // TAI-UTC at 0h for the day
    let (dat, mut js) = eraDat_safe(iy, im, id, 0.0)?;
    if js < 0 {
        return Ok(((0.0, 0.0), -1));
    }

    // UT1-TAI = (UT1-UTC) - (TAI-UTC)
    let dta = dut1 - dat;

    // UTC → TAI
    let ((tai1, tai2), jw) = eraUtctai_safe(utc1, utc2)?;
    if jw < 0 {
        return Ok(((0.0, 0.0), -1));
    } else if jw > 0 {
        js = jw;
    }

    // TAI → UT1
    let ((ut11, ut12), jt) = eraTaiut1_safe(tai1, tai2, dta)?;
    if jt != 0 {
        return Ok(((0.0, 0.0), -1));
    }

    Ok(((ut11, ut12), js))
}
