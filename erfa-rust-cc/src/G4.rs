// G4
//   atci13.c   → eraAtci13
//   atciq.c    → eraAtciq
//   atciqn.c   → eraAtciqn
//   atciqz.c   → eraAtciqz
//   atco13.c   → eraAtco13
//   atic13.c   → eraAtic13
//   aticq.c    → eraAticq
//   aticqn.c   → eraAticqn
//   atio13.c   → eraAtio13
//   atioq.c    → eraAtioq

use crate::H1::*;

// atci13.c
pub unsafe fn eraAtci13(
    rc: f64,
    dc: f64,
    pr: f64,
    pd: f64,
    px: f64,
    rv: f64,
    date1: f64,
    date2: f64,
    ri: *mut f64,
    di: *mut f64,
    eo: *mut f64,
) {
    let mut astrom: eraASTROM = core::mem::zeroed();

    eraApci13(date1, date2, &mut astrom, eo);
    eraAtciq(rc, dc, pr, pd, px, rv, &mut astrom, ri, di);
}

// atciq.c
pub unsafe fn eraAtciq(
    rc: f64,
    dc: f64,
    pr: f64,
    pd: f64,
    px: f64,
    rv: f64,
    astrom: *mut eraASTROM,
    ri: *mut f64,
    di: *mut f64,
) {
    let a = &*astrom;
    let mut pco = [0f64; 3];
    let mut pnat = [0f64; 3];
    let mut ppr = [0f64; 3];
    let mut pi = [0f64; 3];
    let mut w: f64 = 0.0;

    eraPmpx(
        rc,
        dc,
        pr,
        pd,
        px,
        rv,
        a.pmt,
        a.eb.as_ptr() as *mut f64,
        pco.as_mut_ptr(),
    );
    eraLdsun(
        pco.as_mut_ptr(),
        a.eh.as_ptr() as *mut f64,
        a.em,
        pnat.as_mut_ptr(),
    );
    eraAb(
        pnat.as_mut_ptr(),
        a.v.as_ptr() as *mut f64,
        a.em,
        a.bm1,
        ppr.as_mut_ptr(),
    );
    eraRxp(
        a.bpn.as_ptr() as *mut f64,
        ppr.as_mut_ptr(),
        pi.as_mut_ptr(),
    );
    eraC2s(pi.as_mut_ptr(), &mut w, di);
    if !ri.is_null() {
        *ri = eraAnp(w);
    }
}

// atciqn.c
pub unsafe fn eraAtciqn(
    rc: f64,
    dc: f64,
    pr: f64,
    pd: f64,
    px: f64,
    rv: f64,
    astrom: *mut eraASTROM,
    n: i32,
    b: *mut eraLDBODY,
    ri: *mut f64,
    di: *mut f64,
) {
    let a = &*astrom;
    let mut pco = [0f64; 3];
    let mut pnat = [0f64; 3];
    let mut ppr = [0f64; 3];
    let mut pi = [0f64; 3];
    let mut w: f64 = 0.0;

    eraPmpx(
        rc,
        dc,
        pr,
        pd,
        px,
        rv,
        a.pmt,
        a.eb.as_ptr() as *mut f64,
        pco.as_mut_ptr(),
    );
    eraLdn(
        n,
        b,
        a.eb.as_ptr() as *mut f64,
        pco.as_mut_ptr(),
        pnat.as_mut_ptr(),
    );
    eraAb(
        pnat.as_mut_ptr(),
        a.v.as_ptr() as *mut f64,
        a.em,
        a.bm1,
        ppr.as_mut_ptr(),
    );
    eraRxp(
        a.bpn.as_ptr() as *mut f64,
        ppr.as_mut_ptr(),
        pi.as_mut_ptr(),
    );
    eraC2s(pi.as_mut_ptr(), &mut w, di);
    if !ri.is_null() {
        *ri = eraAnp(w);
    }
}

// atciqz.c
pub unsafe fn eraAtciqz(rc: f64, dc: f64, astrom: *mut eraASTROM, ri: *mut f64, di: *mut f64) {
    let a = &*astrom;
    let mut pco = [0f64; 3];
    let mut pnat = [0f64; 3];
    let mut ppr = [0f64; 3];
    let mut pi = [0f64; 3];
    let mut w: f64 = 0.0;

    eraS2c(rc, dc, pco.as_mut_ptr());
    eraLdsun(
        pco.as_mut_ptr(),
        a.eh.as_ptr() as *mut f64,
        a.em,
        pnat.as_mut_ptr(),
    );
    eraAb(
        pnat.as_mut_ptr(),
        a.v.as_ptr() as *mut f64,
        a.em,
        a.bm1,
        ppr.as_mut_ptr(),
    );
    eraRxp(
        a.bpn.as_ptr() as *mut f64,
        ppr.as_mut_ptr(),
        pi.as_mut_ptr(),
    );
    eraC2s(pi.as_mut_ptr(), &mut w, di);
    if !ri.is_null() {
        *ri = eraAnp(w);
    }
}

// atco13.c
pub unsafe fn eraAtco13(
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
    aob: *mut f64,
    zob: *mut f64,
    hob: *mut f64,
    dob: *mut f64,
    rob: *mut f64,
    eo: *mut f64,
) -> i32 {
    let mut astrom: eraASTROM = core::mem::zeroed();
    let mut ri: f64 = 0.0;
    let mut di: f64 = 0.0;

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
        eo,
    );
    if j < 0 {
        return j;
    }

    eraAtciq(rc, dc, pr, pd, px, rv, &mut astrom, &mut ri, &mut di);
    eraAtioq(ri, di, &mut astrom, aob, zob, hob, dob, rob);

    j
}

// atic13.c
pub unsafe fn eraAtic13(
    ri: f64,
    di: f64,
    date1: f64,
    date2: f64,
    rc: *mut f64,
    dc: *mut f64,
    eo: *mut f64,
) {
    let mut astrom: eraASTROM = core::mem::zeroed();

    eraApci13(date1, date2, &mut astrom, eo);
    eraAticq(ri, di, &mut astrom, rc, dc);
}

// aticq.c
pub unsafe fn eraAticq(ri: f64, di: f64, astrom: *mut eraASTROM, rc: *mut f64, dc: *mut f64) {
    let a = &*astrom;
    let mut pi = [0f64; 3];
    let mut ppr = [0f64; 3];
    let mut pnat = [0f64; 3];
    let mut pco = [0f64; 3];
    let mut d = [0f64; 3];
    let mut before = [0f64; 3];
    let mut after = [0f64; 3];
    let mut w: f64 = 0.0;
    let mut r2: f64;
    let mut r: f64;

    eraS2c(ri, di, pi.as_mut_ptr());
    eraTrxp(
        a.bpn.as_ptr() as *mut f64,
        pi.as_mut_ptr(),
        ppr.as_mut_ptr(),
    );

    eraZp(d.as_mut_ptr());
    for _ in 0..2 {
        r2 = 0.0;
        for i in 0..3 {
            w = ppr[i] - d[i];
            before[i] = w;
            r2 += w * w;
        }
        r = r2.sqrt();
        for v in before.iter_mut() {
            *v /= r;
        }
        eraAb(
            before.as_mut_ptr(),
            a.v.as_ptr() as *mut f64,
            a.em,
            a.bm1,
            after.as_mut_ptr(),
        );
        r2 = 0.0;
        for i in 0..3 {
            d[i] = after[i] - before[i];
            w = ppr[i] - d[i];
            pnat[i] = w;
            r2 += w * w;
        }
        r = r2.sqrt();
        for v in pnat.iter_mut() {
            *v /= r;
        }
    }

    eraZp(d.as_mut_ptr());
    for _ in 0..5 {
        r2 = 0.0;
        for i in 0..3 {
            w = pnat[i] - d[i];
            before[i] = w;
            r2 += w * w;
        }
        r = r2.sqrt();
        for v in before.iter_mut() {
            *v /= r;
        }
        eraLdsun(
            before.as_mut_ptr(),
            a.eh.as_ptr() as *mut f64,
            a.em,
            after.as_mut_ptr(),
        );
        r2 = 0.0;
        for i in 0..3 {
            d[i] = after[i] - before[i];
            w = pnat[i] - d[i];
            pco[i] = w;
            r2 += w * w;
        }
        r = r2.sqrt();
        for v in pco.iter_mut() {
            *v /= r;
        }
    }

    eraC2s(pco.as_mut_ptr(), &mut w, dc);
    if !rc.is_null() {
        *rc = eraAnp(w);
    }
}

// aticqn.c
pub unsafe fn eraAticqn(
    ri: f64,
    di: f64,
    astrom: *mut eraASTROM,
    n: i32,
    b: *mut eraLDBODY,
    rc: *mut f64,
    dc: *mut f64,
) {
    let a = &*astrom;
    let mut pi = [0f64; 3];
    let mut ppr = [0f64; 3];
    let mut pnat = [0f64; 3];
    let mut pco = [0f64; 3];
    let mut d = [0f64; 3];
    let mut before = [0f64; 3];
    let mut after = [0f64; 3];
    let mut w: f64 = 0.0;
    let mut r2: f64;
    let mut r: f64;

    eraS2c(ri, di, pi.as_mut_ptr());
    eraTrxp(
        a.bpn.as_ptr() as *mut f64,
        pi.as_mut_ptr(),
        ppr.as_mut_ptr(),
    );

    eraZp(d.as_mut_ptr());
    for _ in 0..2 {
        r2 = 0.0;
        for i in 0..3 {
            w = ppr[i] - d[i];
            before[i] = w;
            r2 += w * w;
        }
        r = r2.sqrt();
        for v in before.iter_mut() {
            *v /= r;
        }
        eraAb(
            before.as_mut_ptr(),
            a.v.as_ptr() as *mut f64,
            a.em,
            a.bm1,
            after.as_mut_ptr(),
        );
        r2 = 0.0;
        for i in 0..3 {
            d[i] = after[i] - before[i];
            w = ppr[i] - d[i];
            pnat[i] = w;
            r2 += w * w;
        }
        r = r2.sqrt();
        for v in pnat.iter_mut() {
            *v /= r;
        }
    }

    eraZp(d.as_mut_ptr());
    for _ in 0..5 {
        r2 = 0.0;
        for i in 0..3 {
            w = pnat[i] - d[i];
            before[i] = w;
            r2 += w * w;
        }
        r = r2.sqrt();
        for v in before.iter_mut() {
            *v /= r;
        }
        eraLdn(
            n,
            b,
            a.eb.as_ptr() as *mut f64,
            before.as_mut_ptr(),
            after.as_mut_ptr(),
        );
        r2 = 0.0;
        for i in 0..3 {
            d[i] = after[i] - before[i];
            w = pnat[i] - d[i];
            pco[i] = w;
            r2 += w * w;
        }
        r = r2.sqrt();
        for v in pco.iter_mut() {
            *v /= r;
        }
    }

    eraC2s(pco.as_mut_ptr(), &mut w, dc);
    if !rc.is_null() {
        *rc = eraAnp(w);
    }
}

// atio13.c
pub unsafe fn eraAtio13(
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
    aob: *mut f64,
    zob: *mut f64,
    hob: *mut f64,
    dob: *mut f64,
    rob: *mut f64,
) -> i32 {
    let mut astrom: eraASTROM = core::mem::zeroed();

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

    eraAtioq(ri, di, &mut astrom, aob, zob, hob, dob, rob);
    j
}

// atioq.c
pub unsafe fn eraAtioq(
    ri: f64,
    di: f64,
    astrom: *mut eraASTROM,
    aob: *mut f64,
    zob: *mut f64,
    hob: *mut f64,
    dob: *mut f64,
    rob: *mut f64,
) {
    const CELMIN: f64 = 1e-6;
    const SELMIN: f64 = 0.05;

    let a = &*astrom;
    let mut v = [0f64; 3];
    let x: f64;
    let y: f64;
    let mut z: f64;
    let (sx, cx) = a.xpl.sin_cos();
    let (sy, cy) = a.ypl.sin_cos();
    let xhd: f64;
    let yhd: f64;
    let zhd: f64;
    let mut f: f64;
    let xhdt: f64;
    let yhdt: f64;
    let zhdt: f64;
    let xaet: f64;
    let yaet: f64;
    let zaet: f64;
    let azobs: f64;
    let mut r: f64;
    let tz: f64;
    let w: f64;
    let del: f64;
    let cosdel: f64;
    let xaeo: f64;
    let yaeo: f64;
    let zaeo: f64;
    let zdobs: f64;
    let mut hmobs: f64 = 0.0;
    let mut dcobs: f64 = 0.0;
    let raobs: f64;

    eraS2c(ri - a.eral, di, v.as_mut_ptr());
    x = v[0];
    y = v[1];
    z = v[2];

    xhd = cx * x + sx * z;
    yhd = sx * sy * x + cy * y - cx * sy * z;
    zhd = -sx * cy * x + sy * y + cx * cy * z;

    f = 1.0 - a.diurab * yhd;
    xhdt = f * xhd;
    yhdt = f * (yhd + a.diurab);
    zhdt = f * zhd;

    xaet = a.sphi * xhdt - a.cphi * zhdt;
    yaet = yhdt;
    zaet = a.cphi * xhdt + a.sphi * zhdt;

    azobs = if xaet != 0.0 || yaet != 0.0 {
        yaet.atan2(-xaet)
    } else {
        0.0
    };

    r = (xaet * xaet + yaet * yaet).sqrt();
    r = if r > CELMIN { r } else { CELMIN };
    z = if zaet > SELMIN { zaet } else { SELMIN };

    tz = r / z;
    w = a.refb * tz * tz;
    del = (a.refa + w) * tz / (1.0 + (a.refa + 3.0 * w) / (z * z));

    cosdel = 1.0 - del * del / 2.0;
    f = cosdel - del * z / r;
    xaeo = xaet * f;
    yaeo = yaet * f;
    zaeo = cosdel * zaet + del * r;

    zdobs = ((xaeo * xaeo + yaeo * yaeo).sqrt()).atan2(zaeo);

    v[0] = a.sphi * xaeo + a.cphi * zaeo;
    v[1] = yaeo;
    v[2] = -a.cphi * xaeo + a.sphi * zaeo;

    eraC2s(v.as_mut_ptr(), &mut hmobs, &mut dcobs);
    raobs = a.eral + hmobs;

    if !aob.is_null() {
        *aob = eraAnp(azobs);
    }
    if !zob.is_null() {
        *zob = zdobs;
    }
    if !hob.is_null() {
        *hob = -hmobs;
    }
    if !dob.is_null() {
        *dob = dcobs;
    }
    if !rob.is_null() {
        *rob = eraAnp(raobs);
    }
}
