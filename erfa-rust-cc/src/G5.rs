// G5
//   atoc13.c  → eraAtoc13
//   atoi13.c  → eraAtoi13
//   atoiq.c   → eraAtoiq

use crate::H1::*;
use core::ffi::c_char;

// eraAtoc13  ― Observed place → ICRS astrometric RA,Dec (2013 models)
pub unsafe fn eraAtoc13(
    type_: *const c_char,
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
    rc: *mut f64,
    dc: *mut f64,
) -> i32 {
    let mut astrom = eraASTROM::default();
    let mut eo: f64 = 0.0;

    let j = eraApco13(
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
        &mut eo,
    );
    if j < 0 {
        return j;
    }

    let mut ri = 0.0_f64;
    let mut di = 0.0_f64;
    eraAtoiq(type_, ob1, ob2, &mut astrom, &mut ri, &mut di);

    eraAticq(ri, di, &mut astrom, rc, dc);

    j
}

// G5/atoi13.c  -->  eraAtoi13

// eraAtoi13  ― Observed place → CIRS (2013 models)
pub unsafe fn eraAtoi13(
    type_: *const c_char,
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
    ri: *mut f64,
    di: *mut f64,
) -> i32 {
    let mut astrom = eraASTROM::default();

    let j = eraApio13(
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
    );
    if j < 0 {
        return j;
    }

    eraAtoiq(type_, ob1, ob2, &mut astrom, ri, di);

    j
}

// G5/atoiq.c  -->  eraAtoiq

// eraAtoiq  ― Quick observed place → CIRS
pub unsafe fn eraAtoiq(
    type_: *const c_char,
    ob1: f64,
    ob2: f64,
    astrom: *mut eraASTROM,
    ri: *mut f64,
    di: *mut f64,
) {
    const SELMIN: f64 = 0.05;

    let a = &*astrom;

    let mut c = *type_ as u8 as char;

    let mut c1 = ob1;
    let c2 = ob2;

    let sphi = a.sphi;
    let cphi = a.cphi;

    c = match c {
        'r' | 'R' => 'R',
        'h' | 'H' => 'H',
        _ => 'A',
    };

    let (xaeo, yaeo, zaeo) = if c == 'A' {
        let ce = c2.sin();
        (-c1.cos() * ce, c1.sin() * ce, c2.cos())
    } else {
        if c == 'R' {
            c1 = a.eral - c1;
        }
        let mut v = [0.0_f64; 3];
        eraS2c(-c1, c2, v.as_mut_ptr());
        let xmhdo = v[0];
        let ymhdo = v[1];
        let zmhdo = v[2];

        (
            sphi * xmhdo - cphi * zmhdo,
            ymhdo,
            cphi * xmhdo + sphi * zmhdo,
        )
    };

    let az = if xaeo != 0.0 || yaeo != 0.0 {
        yaeo.atan2(xaeo)
    } else {
        0.0
    };

    let sz = (xaeo * xaeo + yaeo * yaeo).sqrt();
    let zdo = sz.atan2(zaeo);

    let refa = a.refa;
    let refb = a.refb;
    let tz = sz / if zaeo > SELMIN { zaeo } else { SELMIN };
    let dref = (refa + refb * tz * tz) * tz;
    let zdt = zdo + dref;

    let ce = zdt.sin();
    let xaet = az.cos() * ce;
    let yaet = az.sin() * ce;
    let zaet = zdt.cos();

    let xmhda = sphi * xaet + cphi * zaet;
    let ymhda = yaet;
    let zmhda = -cphi * xaet + sphi * zaet;

    let f = 1.0 + a.diurab * ymhda;
    let xhd = f * xmhda;
    let yhd = f * (ymhda - a.diurab);
    let zhd = f * zmhda;

    let sx = a.xpl.sin();
    let cx = a.xpl.cos();
    let sy = a.ypl.sin();
    let cy = a.ypl.cos();
    let mut v = [
        cx * xhd + sx * sy * yhd - sx * cy * zhd,
        cy * yhd + sy * zhd,
        sx * xhd - cx * sy * yhd + cx * cy * zhd,
    ];

    let mut hma = 0.0_f64;
    eraC2s(v.as_mut_ptr(), &mut hma, di);

    *ri = eraAnp(a.eral + hma);
}
