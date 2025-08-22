// G18
//   h2fk5.c  → eraH2fk5
//   hd2ae.c  → eraHd2ae
//   hd2pa.c  → eraHd2pa
//   hfk5z.c  → eraHfk5z

use crate::H1::*;

// eraH2fk5    Hipparcos catalog data → FK5 (J2000)                   *
pub unsafe fn eraH2fk5(
    rh: f64,
    dh: f64,
    drh: f64,
    ddh: f64,
    pxh: f64,
    rvh: f64,
    r5: *mut f64,
    d5: *mut f64,
    dr5: *mut f64,
    dd5: *mut f64,
    px5: *mut f64,
    rv5: *mut f64,
) {
    let mut pvh = [[0.0_f64; 3]; 2];
    let mut r5h = [[0.0_f64; 3]; 3];
    let mut s5h = [0.0_f64; 3];
    let mut sh = [0.0_f64; 3];
    let mut wxp = [0.0_f64; 3];
    let mut vv = [0.0_f64; 3];
    let mut pv5 = [[0.0_f64; 3]; 2];

    eraStarpv(rh, dh, drh, ddh, pxh, rvh, pvh.as_mut_ptr() as *mut f64);

    eraFk5hip(r5h.as_mut_ptr() as *mut f64, s5h.as_mut_ptr());

    for i in 0..3 {
        s5h[i] /= 365.25;
    }

    eraRxp(
        r5h.as_mut_ptr() as *mut f64,
        s5h.as_mut_ptr(),
        sh.as_mut_ptr(),
    );

    eraTrxp(
        r5h.as_mut_ptr() as *mut f64,
        pvh[0].as_mut_ptr(),
        pv5[0].as_mut_ptr(),
    );

    eraPxp(pvh[0].as_mut_ptr(), sh.as_mut_ptr(), wxp.as_mut_ptr());

    eraPmp(pvh[1].as_mut_ptr(), wxp.as_mut_ptr(), vv.as_mut_ptr());

    eraTrxp(
        r5h.as_mut_ptr() as *mut f64,
        vv.as_mut_ptr(),
        pv5[1].as_mut_ptr(),
    );

    eraPvstar(pv5.as_mut_ptr() as *mut f64, r5, d5, dr5, dd5, px5, rv5);
}

// eraHd2ae    HA/Dec → Az/El                                         *
pub unsafe fn eraHd2ae(ha: f64, dec: f64, phi: f64, az: *mut f64, el: *mut f64) {
    let (sh, ch) = ha.sin_cos();
    let (sd, cd) = dec.sin_cos();
    let (sp, cp) = phi.sin_cos();

    let x = -ch * cd * sp + sd * cp;
    let y = -sh * cd;
    let z = ch * cd * cp + sd * sp;

    let r = (x * x + y * y).sqrt();
    let mut a = if r != 0.0 { y.atan2(x) } else { 0.0 };
    if a < 0.0 {
        a += ERFA_D2PI;
    }

    *az = a;
    *el = z.atan2(r);
}

// eraHd2pa    Parallactic angle                                     *
pub unsafe fn eraHd2pa(ha: f64, dec: f64, phi: f64) -> f64 {
    let cp = phi.cos();
    let sqsz = cp * ha.sin();
    let cqsz = phi.sin() * dec.cos() - cp * dec.sin() * ha.cos();
    if sqsz != 0.0 || cqsz != 0.0 {
        sqsz.atan2(cqsz)
    } else {
        0.0
    }
}

// eraHfk5z  Hipparcos pos (zero μ) → FK5 at given date               *
pub unsafe fn eraHfk5z(
    rh: f64,
    dh: f64,
    date1: f64,
    date2: f64,
    r5: *mut f64,
    d5: *mut f64,
    dr5: *mut f64,
    dd5: *mut f64,
) {
    let mut ph = [0.0_f64; 3];
    let mut r5h = [[0.0_f64; 3]; 3];
    let mut s5h = [0.0_f64; 3];
    let mut sh = [0.0_f64; 3];
    let mut vst = [0.0_f64; 3];
    let mut rst = [[0.0_f64; 3]; 3];
    let mut r5ht = [[0.0_f64; 3]; 3];
    let mut pv5e = [[0.0_f64; 3]; 2];
    let mut vv = [0.0_f64; 3];

    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJY;

    eraS2c(rh, dh, ph.as_mut_ptr());

    eraFk5hip(r5h.as_mut_ptr() as *mut f64, s5h.as_mut_ptr());

    eraRxp(
        r5h.as_mut_ptr() as *mut f64,
        s5h.as_mut_ptr(),
        sh.as_mut_ptr(),
    );

    eraSxp(t, s5h.as_mut_ptr(), vst.as_mut_ptr());

    eraRv2m(vst.as_mut_ptr(), rst.as_mut_ptr() as *mut f64);

    eraRxr(
        r5h.as_mut_ptr() as *mut f64,
        rst.as_mut_ptr() as *mut f64,
        r5ht.as_mut_ptr() as *mut f64,
    );

    eraTrxp(
        r5ht.as_mut_ptr() as *mut f64,
        ph.as_mut_ptr(),
        pv5e[0].as_mut_ptr(),
    );

    eraPxp(sh.as_mut_ptr(), ph.as_mut_ptr(), vv.as_mut_ptr());

    eraTrxp(
        r5ht.as_mut_ptr() as *mut f64,
        vv.as_mut_ptr(),
        pv5e[1].as_mut_ptr(),
    );

    let mut w = 0.0_f64;
    let mut r = 0.0_f64;
    let mut v = 0.0_f64;
    eraPv2s(
        pv5e.as_mut_ptr() as *mut f64,
        &mut w,
        d5,
        &mut r,
        dr5,
        dd5,
        &mut v,
    );
    *r5 = eraAnp(w);
}
