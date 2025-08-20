// G9
//   d2tf.c   → eraD2tf_safe
//   d2dtf.c  → eraD2dtf_safe
//   dat.c    → eraDat_safe

use crate::G14_safe::eraDatini_safe;
use crate::G19_safe::eraJd2cal_safe;
use crate::G8_safe::eraCal2jd_safe;
use crate::H1_safe::{eraLEAPSECOND, ERFA_DAYSEC, ERFA_DINT, ERFA_DNINT};

pub type ErfaResult<T> = Result<T, ()>;

// G9/d2tf.c → eraD2tf_safe
// Convert interval in days to sign and HMS with fractional field.
pub fn eraD2tf_safe(ndp: i32, days: f64) -> ErfaResult<(char, [i32; 4])> {
    let sign = if days >= 0.0 { '+' } else { '-' };

    let mut a = ERFA_DAYSEC * days.abs();

    if ndp < 0 {
        let mut nrs = 1;
        for n in 1..=-ndp {
            nrs *= if n == 2 || n == 4 { 6 } else { 10 };
        }
        let rs = nrs as f64;
        let w = a / rs;
        a = rs * ERFA_DNINT(w);
    }

    let mut nrs = 1;
    for _ in 1..=ndp.max(0) {
        nrs *= 10;
    }
    let rs = nrs as f64;
    let rm = rs * 60.0;
    let rh = rm * 60.0;

    a = ERFA_DNINT(rs * a);

    let ah = ERFA_DINT(a / rh);
    a -= ah * rh;
    let am = ERFA_DINT(a / rm);
    a -= am * rm;
    let as_ = ERFA_DINT(a / rs);
    let af = a - as_ * rs;

    let ihmsf = [ah as i32, am as i32, as_ as i32, af as i32];
    Ok((sign, ihmsf))
}

// G9/d2dtf.c → eraD2dtf_safe
// Format two-part JD into calendar date and time fields for a given scale.
pub fn eraD2dtf_safe(
    scale: &str,
    ndp: i32,
    d1: f64,
    d2: f64,
) -> ErfaResult<((i32, i32, i32), [i32; 4], i32)> {
    let ((iy0, im0, id0), mut fd, j_cal) = eraJd2cal_safe(d1, d2)?;
    if j_cal != 0 {
        return Err(());
    }
    let (mut iy1, mut im1, mut id1) = (iy0, im0, id0);

    let mut js = 0;
    let mut leap = false;

    if scale == "UTC" {
        let (dat0, j0) = eraDat_safe(iy1, im1, id1, 0.0)?;
        if j0 < 0 {
            return Err(());
        }
        if j0 > 0 {
            js = j0;
        }

        let (dat12, j12) = eraDat_safe(iy1, im1, id1, 0.5)?;
        if j12 < 0 {
            return Err(());
        }
        if j12 > 0 && js == 0 {
            js = j12;
        }

        let ((iy2, im2, id2), _w, j_next) = eraJd2cal_safe(d1 + 1.5, d2 - fd)?;
        if j_next != 0 {
            return Err(());
        }
        let (dat24, j24) = eraDat_safe(iy2, im2, id2, 0.0)?;
        if j24 < 0 {
            return Err(());
        }
        if j24 > 0 && js == 0 {
            js = j24;
        }

        let dleap = dat24 - (2.0 * dat12 - dat0);
        leap = dleap.abs() > 0.5;
        if leap {
            fd += fd * dleap / ERFA_DAYSEC;
        }
    }

    let (_s, mut ihmsf1) = eraD2tf_safe(ndp, fd)?;

    if ihmsf1[0] > 23 {
        let ((iy2, im2, id2), _w, j_next) = eraJd2cal_safe(d1 + 1.5, d2 - fd)?;
        if j_next != 0 {
            return Err(());
        }

        if !leap {
            iy1 = iy2;
            im1 = im2;
            id1 = id2;
            ihmsf1 = [0, 0, 0, 0];
        } else {
            if ihmsf1[2] > 0 {
                iy1 = iy2;
                im1 = im2;
                id1 = id2;
                ihmsf1 = [0, 0, 0, 0];
            } else {
                ihmsf1[0] = 23;
                ihmsf1[1] = 59;
                ihmsf1[2] = 60;
            }
            if ndp < 0 && ihmsf1[2] == 60 {
                iy1 = iy2;
                im1 = im2;
                id1 = id2;
                ihmsf1 = [0, 0, 0, 0];
            }
        }
    }

    Ok(((iy1, im1, id1), ihmsf1, js))
}

// G9/dat.c → eraDat_safe
// TAI−UTC = ΔAT for a given date; returns (ΔAT seconds, status).
pub fn eraDat_safe(iy: i32, im: i32, id: i32, fd: f64) -> ErfaResult<(f64, i32)> {
    const IYV: i32 = 2023;

    const DRIFT: &[[f64; 2]] = &[
        [37300.0, 0.0012960],
        [37300.0, 0.0012960],
        [37300.0, 0.0012960],
        [37665.0, 0.0011232],
        [37665.0, 0.0011232],
        [38761.0, 0.0012960],
        [38761.0, 0.0012960],
        [38761.0, 0.0012960],
        [38761.0, 0.0012960],
        [38761.0, 0.0012960],
        [38761.0, 0.0012960],
        [38761.0, 0.0012960],
        [39126.0, 0.0025920],
        [39126.0, 0.0025920],
    ];
    const NERA1: usize = DRIFT.len();

    const CHANGES_BUILTIN: &[eraLEAPSECOND] = &[
        eraLEAPSECOND {
            iyear: 1960,
            month: 1,
            delat: 1.4178180,
        },
        eraLEAPSECOND {
            iyear: 1961,
            month: 1,
            delat: 1.4228180,
        },
        eraLEAPSECOND {
            iyear: 1961,
            month: 8,
            delat: 1.3728180,
        },
        eraLEAPSECOND {
            iyear: 1962,
            month: 1,
            delat: 1.8458580,
        },
        eraLEAPSECOND {
            iyear: 1963,
            month: 11,
            delat: 1.9458580,
        },
        eraLEAPSECOND {
            iyear: 1964,
            month: 1,
            delat: 3.2401300,
        },
        eraLEAPSECOND {
            iyear: 1964,
            month: 4,
            delat: 3.3401300,
        },
        eraLEAPSECOND {
            iyear: 1964,
            month: 9,
            delat: 3.4401300,
        },
        eraLEAPSECOND {
            iyear: 1965,
            month: 1,
            delat: 3.5401300,
        },
        eraLEAPSECOND {
            iyear: 1965,
            month: 3,
            delat: 3.6401300,
        },
        eraLEAPSECOND {
            iyear: 1965,
            month: 7,
            delat: 3.7401300,
        },
        eraLEAPSECOND {
            iyear: 1965,
            month: 9,
            delat: 3.8401300,
        },
        eraLEAPSECOND {
            iyear: 1966,
            month: 1,
            delat: 4.3131700,
        },
        eraLEAPSECOND {
            iyear: 1968,
            month: 2,
            delat: 4.2131700,
        },
        eraLEAPSECOND {
            iyear: 1972,
            month: 1,
            delat: 10.0,
        },
        eraLEAPSECOND {
            iyear: 1972,
            month: 7,
            delat: 11.0,
        },
        eraLEAPSECOND {
            iyear: 1973,
            month: 1,
            delat: 12.0,
        },
        eraLEAPSECOND {
            iyear: 1974,
            month: 1,
            delat: 13.0,
        },
        eraLEAPSECOND {
            iyear: 1975,
            month: 1,
            delat: 14.0,
        },
        eraLEAPSECOND {
            iyear: 1976,
            month: 1,
            delat: 15.0,
        },
        eraLEAPSECOND {
            iyear: 1977,
            month: 1,
            delat: 16.0,
        },
        eraLEAPSECOND {
            iyear: 1978,
            month: 1,
            delat: 17.0,
        },
        eraLEAPSECOND {
            iyear: 1979,
            month: 1,
            delat: 18.0,
        },
        eraLEAPSECOND {
            iyear: 1980,
            month: 1,
            delat: 19.0,
        },
        eraLEAPSECOND {
            iyear: 1981,
            month: 7,
            delat: 20.0,
        },
        eraLEAPSECOND {
            iyear: 1982,
            month: 7,
            delat: 21.0,
        },
        eraLEAPSECOND {
            iyear: 1983,
            month: 7,
            delat: 22.0,
        },
        eraLEAPSECOND {
            iyear: 1985,
            month: 7,
            delat: 23.0,
        },
        eraLEAPSECOND {
            iyear: 1988,
            month: 1,
            delat: 24.0,
        },
        eraLEAPSECOND {
            iyear: 1990,
            month: 1,
            delat: 25.0,
        },
        eraLEAPSECOND {
            iyear: 1991,
            month: 1,
            delat: 26.0,
        },
        eraLEAPSECOND {
            iyear: 1992,
            month: 7,
            delat: 27.0,
        },
        eraLEAPSECOND {
            iyear: 1993,
            month: 7,
            delat: 28.0,
        },
        eraLEAPSECOND {
            iyear: 1994,
            month: 7,
            delat: 29.0,
        },
        eraLEAPSECOND {
            iyear: 1996,
            month: 1,
            delat: 30.0,
        },
        eraLEAPSECOND {
            iyear: 1997,
            month: 7,
            delat: 31.0,
        },
        eraLEAPSECOND {
            iyear: 1999,
            month: 1,
            delat: 32.0,
        },
        eraLEAPSECOND {
            iyear: 2006,
            month: 1,
            delat: 33.0,
        },
        eraLEAPSECOND {
            iyear: 2009,
            month: 1,
            delat: 34.0,
        },
        eraLEAPSECOND {
            iyear: 2012,
            month: 7,
            delat: 35.0,
        },
        eraLEAPSECOND {
            iyear: 2015,
            month: 7,
            delat: 36.0,
        },
        eraLEAPSECOND {
            iyear: 2017,
            month: 1,
            delat: 37.0,
        },
    ];

    if fd < 0.0 || fd > 1.0 {
        return Err(());
    }

    let ((_, djm), jcal) = eraCal2jd_safe(iy, im, id)?;
    if jcal < 0 {
        return Err(());
    }

    let table = eraDatini_safe(CHANGES_BUILTIN)?;

    if iy < table[0].iyear {
        return Ok((0.0, 1));
    }

    let mut j = 0;
    if iy > IYV + 5 {
        j = 1;
    }

    let m = 12 * iy + im;

    let mut idx = None;
    for i in (0..table.len()).rev() {
        if m >= (12 * table[i].iyear + table[i].month) {
            idx = Some(i);
            break;
        }
    }
    let i = match idx {
        Some(i) => i,
        None => return Err(()),
    };

    let mut da = table[i].delat;

    if i < NERA1 {
        da += (djm + fd - DRIFT[i][0]) * DRIFT[i][1];
    }

    Ok((da, j))
}
