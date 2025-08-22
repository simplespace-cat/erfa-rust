// G33
//   tr.c      → eraTr
//   trxp.c    → eraTrxp
//   trxpv.c   → eraTrxpv
//   tttai.c   → eraTttai
//   tttcg.c   → eraTttcg
//   tttdb.c   → eraTttdb
//   ttut1.c   → eraTtut1
//   ut1tai.c  → eraUt1tai
//   ut1tt.c   → eraUt1tt
//   ut1utc.c  → eraUt1utc
//   utctai.c  → eraUtctai
//   utcut1.c  → eraUtcut1

use crate::H1::*;

// eraTr   transpose 3×3 matrix
pub unsafe fn eraTr(r: *mut f64, rt: *mut f64) {
    let mut wm = [0.0_f64; 9];
    for i in 0..3 {
        for j in 0..3 {
            wm[i * 3 + j] = *r.add(j * 3 + i);
        }
    }

    eraCr(wm.as_mut_ptr(), rt);
}

// eraTrxp   transpose(r) * p-vector
pub unsafe fn eraTrxp(r: *mut f64, p: *mut f64, trp: *mut f64) {
    let mut tr = [0.0_f64; 9];
    eraTr(r, tr.as_mut_ptr());
    eraRxp(tr.as_mut_ptr(), p, trp);
}

// eraTrxpv  transpose(r) * pv-vector
pub unsafe fn eraTrxpv(r: *mut f64, pv: *mut f64, trpv: *mut f64) {
    let mut tr = [0.0_f64; 9];
    eraTr(r, tr.as_mut_ptr());
    eraRxpv(tr.as_mut_ptr(), pv, trpv);
}

// eraTttai   TT → TAI
pub unsafe fn eraTttai(tt1: f64, tt2: f64, tai1: *mut f64, tai2: *mut f64) -> i32 {
    let dtat = ERFA_TTMTAI / ERFA_DAYSEC;
    if tt1.abs() > tt2.abs() {
        *tai1 = tt1;
        *tai2 = tt2 - dtat;
    } else {
        *tai1 = tt1 - dtat;
        *tai2 = tt2;
    }
    0
}

// eraTttcg   TT → TCG
pub unsafe fn eraTttcg(tt1: f64, tt2: f64, tcg1: *mut f64, tcg2: *mut f64) -> i32 {
    const T77T: f64 = ERFA_DJM77 + ERFA_TTMTAI / ERFA_DAYSEC;
    const ELGG: f64 = ERFA_ELG / (1.0 - ERFA_ELG);

    if tt1.abs() > tt2.abs() {
        *tcg1 = tt1;
        *tcg2 = tt2 + ((tt1 - ERFA_DJM0) + (tt2 - T77T)) * ELGG;
    } else {
        *tcg1 = tt1 + ((tt2 - ERFA_DJM0) + (tt1 - T77T)) * ELGG;
        *tcg2 = tt2;
    }
    0
}

// eraTttdb   TT → TDB  (using caller-supplied dtr = TDB−TT seconds)
pub unsafe fn eraTttdb(tt1: f64, tt2: f64, dtr: f64, tdb1: *mut f64, tdb2: *mut f64) -> i32 {
    let dtrd = dtr / ERFA_DAYSEC;
    if tt1.abs() > tt2.abs() {
        *tdb1 = tt1;
        *tdb2 = tt2 + dtrd;
    } else {
        *tdb1 = tt1 + dtrd;
        *tdb2 = tt2;
    }
    0
}

// eraTtut1   TT → UT1   (dt = TT−UT1 seconds)
pub unsafe fn eraTtut1(tt1: f64, tt2: f64, dt: f64, ut11: *mut f64, ut12: *mut f64) -> i32 {
    let dtd = dt / ERFA_DAYSEC;
    if tt1.abs() > tt2.abs() {
        *ut11 = tt1;
        *ut12 = tt2 - dtd;
    } else {
        *ut11 = tt1 - dtd;
        *ut12 = tt2;
    }
    0
}

// eraUt1tai   UT1 → TAI  (dta = UT1−TAI seconds)
pub unsafe fn eraUt1tai(ut11: f64, ut12: f64, dta: f64, tai1: *mut f64, tai2: *mut f64) -> i32 {
    let dtad = dta / ERFA_DAYSEC;
    if ut11.abs() > ut12.abs() {
        *tai1 = ut11;
        *tai2 = ut12 - dtad;
    } else {
        *tai1 = ut11 - dtad;
        *tai2 = ut12;
    }
    0
}

// eraUt1tt   UT1 → TT   (dt = TT−UT1 seconds)
pub unsafe fn eraUt1tt(ut11: f64, ut12: f64, dt: f64, tt1: *mut f64, tt2: *mut f64) -> i32 {
    let dtd = dt / ERFA_DAYSEC;
    if ut11.abs() > ut12.abs() {
        *tt1 = ut11;
        *tt2 = ut12 + dtd;
    } else {
        *tt1 = ut11 + dtd;
        *tt2 = ut12;
    }
    0
}

// eraUt1utc  UT1 → UTC   (handles leap-second wrinkles)

pub unsafe fn eraUt1utc(ut11: f64, ut12: f64, dut1: f64, utc1: *mut f64, utc2: *mut f64) -> i32 {
    let u1: f64;
    let mut u2: f64;
    let big1 = ut11.abs() >= ut12.abs();
    if big1 {
        u1 = ut11;
        u2 = ut12;
    } else {
        u1 = ut12;
        u2 = ut11;
    }

    let mut duts = dut1;

    let mut js: i32 = 0;
    let d1 = u1;
    let mut dats1: f64 = 0.0;
    let mut iy = 0;
    let mut im = 0;
    let mut id = 0;
    let mut fd = 0.0;

    for i in -1..=3 {
        let d2 = u2 + i as f64;

        if eraJd2cal(d1, d2, &mut iy, &mut im, &mut id, &mut fd) != 0 {
            return -1;
        }

        let mut dats2: f64 = 0.0;
        js = eraDat(iy, im, id, 0.0, &mut dats2);
        if js < 0 {
            return -1;
        }

        if i == -1 {
            dats1 = dats2;
        }

        let ddats = dats2 - dats1;

        if ddats.abs() >= 0.5 {
            if ddats * duts >= 0.0 {
                duts -= ddats;
            }

            let mut us1 = 0.0_f64;
            let mut us2 = 0.0_f64;
            if eraCal2jd(iy, im, id, &mut us1, &mut us2) != 0 {
                return -1;
            }
            us2 = us2 - 1.0 + duts / ERFA_DAYSEC;

            let mut du = u1 - us1;
            du += u2 - us2;

            if du > 0.0 {
                fd = du * ERFA_DAYSEC / (ERFA_DAYSEC + ddats);

                duts += ddats * if fd <= 1.0 { fd } else { 1.0 };
            }

            break;
        }

        dats1 = dats2;
    }

    u2 -= duts / ERFA_DAYSEC;

    if big1 {
        *utc1 = u1;
        *utc2 = u2;
    } else {
        *utc1 = u2;
        *utc2 = u1;
    }

    js
}

// eraUtctai  UTC → TAI   (with leap-second handling)
pub unsafe fn eraUtctai(utc1: f64, utc2: f64, tai1: *mut f64, tai2: *mut f64) -> i32 {
    let big1 = utc1.abs() >= utc2.abs();
    let (u1, u2) = if big1 { (utc1, utc2) } else { (utc2, utc1) };

    let mut iy = 0;
    let mut im = 0;
    let mut id = 0;
    let mut fd = 0.0;
    let mut j = eraJd2cal(u1, u2, &mut iy, &mut im, &mut id, &mut fd);
    if j != 0 {
        return j;
    }
    let mut dat0 = 0.0;
    j = eraDat(iy, im, id, 0.0, &mut dat0);
    if j < 0 {
        return j;
    }

    let mut dat12 = 0.0;
    j = eraDat(iy, im, id, 0.5, &mut dat12);
    if j < 0 {
        return j;
    }

    let mut iyt = 0;
    let mut imt = 0;
    let mut idt = 0;
    let mut w = 0.0;
    j = eraJd2cal(u1 + 1.5, u2 - fd, &mut iyt, &mut imt, &mut idt, &mut w);
    if j != 0 {
        return j;
    }
    let mut dat24 = 0.0;
    j = eraDat(iyt, imt, idt, 0.0, &mut dat24);
    if j < 0 {
        return j;
    }

    let dlod = 2.0 * (dat12 - dat0);
    let dleap = dat24 - (dat0 + dlod);

    fd *= (ERFA_DAYSEC + dleap) / ERFA_DAYSEC;
    fd *= (ERFA_DAYSEC + dlod) / ERFA_DAYSEC;

    let mut z1 = 0.0;
    let mut z2 = 0.0;
    if eraCal2jd(iy, im, id, &mut z1, &mut z2) != 0 {
        return -1;
    }
    let a2 = z1 - u1 + z2 + fd + dat0 / ERFA_DAYSEC;

    if big1 {
        *tai1 = u1;
        *tai2 = a2;
    } else {
        *tai1 = a2;
        *tai2 = u1;
    }
    j
}

// eraUtcut1  UTC → UT1
pub unsafe fn eraUtcut1(utc1: f64, utc2: f64, dut1: f64, ut11: *mut f64, ut12: *mut f64) -> i32 {
    let mut iy = 0;
    let mut im = 0;
    let mut id = 0;
    let mut w = 0.0;
    if eraJd2cal(utc1, utc2, &mut iy, &mut im, &mut id, &mut w) != 0 {
        return -1;
    }
    let mut dat = 0.0;
    let mut js = eraDat(iy, im, id, 0.0, &mut dat);
    if js < 0 {
        return -1;
    }

    let dta = dut1 - dat;

    let mut tai1 = 0.0;
    let mut tai2 = 0.0;
    let jw = eraUtctai(utc1, utc2, &mut tai1, &mut tai2);
    if jw < 0 {
        return -1;
    } else if jw > 0 {
        js = jw;
    }

    if eraTaiut1(tai1, tai2, dta, ut11, ut12) != 0 {
        return -1;
    }

    js
}
