// G9
//   d2tf.c   → eraD2tf
//   d2dtf.c  → eraD2dtf
//   dat.c    → eraDat

use crate::H1::*;
use core::ffi::c_char;
use std::ffi::CStr;

// G9/d2tf.c  -->  eraD2tf
pub unsafe fn eraD2tf(ndp: i32, days: f64, sign: *mut c_char, ihmsf: *mut i32) {
    *sign = if days >= 0.0 {
        b'+' as c_char
    } else {
        b'-' as c_char
    };

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

    let out = core::slice::from_raw_parts_mut(ihmsf, 4);
    out[0] = ah as i32;
    out[1] = am as i32;
    out[2] = as_ as i32;
    out[3] = af as i32;
}

// G9/d2dtf.c  -->  eraD2dtf
pub unsafe fn eraD2dtf(
    scale: *const c_char,
    ndp: i32,
    d1: f64,
    d2: f64,
    iy: *mut i32,
    im: *mut i32,
    id: *mut i32,
    ihmsf: *mut i32,
) -> i32 {
    let (iy_ptr, im_ptr, id_ptr) = (&mut *iy, &mut *im, &mut *id);

    let (mut iy1, mut im1, mut id1) = (0, 0, 0);
    let mut fd = 0.0_f64;
    let mut js: i32;

    let a1 = d1;
    let b1 = d2;

    js = eraJd2cal(a1, b1, &mut iy1, &mut im1, &mut id1, &mut fd);
    if js != 0 {
        return -1;
    }

    let mut leap = false;
    if cstr_eq(scale, b"UTC") {
        let mut dat0 = 0.0_f64;
        js = eraDat(iy1, im1, id1, 0.0, &mut dat0);
        if js < 0 {
            return -1;
        }

        let mut dat12 = 0.0_f64;
        js = eraDat(iy1, im1, id1, 0.5, &mut dat12);
        if js < 0 {
            return -1;
        }

        let (mut iy2, mut im2, mut id2) = (0, 0, 0);
        let mut w = 0.0_f64;
        js = eraJd2cal(a1 + 1.5, b1 - fd, &mut iy2, &mut im2, &mut id2, &mut w);
        if js != 0 {
            return -1;
        }
        let mut dat24 = 0.0_f64;
        js = eraDat(iy2, im2, id2, 0.0, &mut dat24);
        if js < 0 {
            return -1;
        }

        let dleap = dat24 - (2.0 * dat12 - dat0);

        leap = dleap.abs() > 0.5;
        if leap {
            fd += fd * dleap / ERFA_DAYSEC;
        }
    }

    let mut s = 0_i8 as c_char;
    let mut ihmsf1 = [0_i32; 4];
    eraD2tf(ndp, fd, &mut s, ihmsf1.as_mut_ptr());

    if ihmsf1[0] > 23 {
        let (mut iy2, mut im2, mut id2) = (0, 0, 0);
        let mut w = 0.0_f64;
        js = eraJd2cal(a1 + 1.5, b1 - fd, &mut iy2, &mut im2, &mut id2, &mut w);
        if js != 0 {
            return -1;
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

    *iy_ptr = iy1;
    *im_ptr = im1;
    *id_ptr = id1;
    for i in 0..4 {
        *ihmsf.add(i) = ihmsf1[i];
    }

    js
}

// G9/dat.c  -->  eraDat
pub unsafe fn eraDat(iy: i32, im: i32, id: i32, fd: f64, deltat: *mut f64) -> i32 {
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

    const CHANGES: &[eraLEAPSECOND] = &[
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
    const NDAT_EMBED: usize = CHANGES.len();

    let mut changes_ptr: *mut eraLEAPSECOND = core::ptr::null_mut();
    let ndat = eraDatini(CHANGES.as_ptr(), NDAT_EMBED as i32, &mut changes_ptr);
    let changes = core::slice::from_raw_parts(changes_ptr, ndat as usize);

    *deltat = 0.0;

    if fd < 0.0 || fd > 1.0 {
        return -4;
    }

    let mut djm0 = 0.0_f64;
    let mut djm = 0.0_f64;
    let mut j = eraCal2jd(iy, im, id, &mut djm0, &mut djm);
    if j < 0 {
        return j;
    }

    if iy < changes[0].iyear {
        return 1;
    }

    if iy > IYV + 5 {
        j = 1;
    }

    let m = 12 * iy + im;

    let mut idx = None;
    for i in (0..changes.len()).rev() {
        if m >= (12 * changes[i].iyear + changes[i].month) {
            idx = Some(i);
            break;
        }
    }
    let i = match idx {
        Some(i) => i,
        None => return -5,
    };

    let mut da = changes[i].delat;

    if i < NERA1 {
        da += (djm + fd - DRIFT[i][0]) * DRIFT[i][1];
    }

    *deltat = da;
    j
}

// C
fn cstr_eq(ptr: *const c_char, bytes: &[u8]) -> bool {
    if ptr.is_null() {
        return false;
    }
    unsafe {
        let cstr = CStr::from_ptr(ptr);
        cstr.to_bytes() == bytes
    }
}
