// G2
//   apco.c    → eraApco
//   apco13.c  → eraApco13
//   apcs.c    → eraApcs
//   apcs13.c  → eraApcs13
//   aper.c    → eraAper

use crate::H1::*;
use core::f64;

// G2/apco.c

pub unsafe fn eraApco(
    date1: f64,
    date2: f64,
    ebpv: *mut f64,
    ehp: *mut f64,
    x: f64,
    y: f64,
    s: f64,
    theta: f64,
    elong: f64,
    phi: f64,
    hm: f64,
    xp: f64,
    yp: f64,
    sp: f64,
    refa: f64,
    refb: f64,
    astrom: *mut eraASTROM,
) {
    let mut r = [[0.0_f64; 3]; 3];
    let mut a;
    let mut b;
    let eral;
    let c;
    let mut pvc = [[0.0_f64; 3]; 2];
    let mut pv = [[0.0_f64; 3]; 2];

    eraIr(r.as_mut_ptr() as *mut f64);
    eraRz(theta + sp, r.as_mut_ptr() as *mut f64);
    eraRy(-xp, r.as_mut_ptr() as *mut f64);
    eraRx(-yp, r.as_mut_ptr() as *mut f64);
    eraRz(elong, r.as_mut_ptr() as *mut f64);

    a = r[0][0];
    b = r[0][1];
    eral = if a != 0.0 || b != 0.0 {
        b.atan2(a)
    } else {
        0.0
    };
    (*astrom).eral = eral;

    a = r[0][0];
    c = r[0][2];
    (*astrom).xpl = (c).atan2((a * a + b * b).sqrt());
    a = r[1][2];
    b = r[2][2];
    (*astrom).ypl = if a != 0.0 || b != 0.0 {
        -a.atan2(b)
    } else {
        0.0
    };

    (*astrom).along = eraAnpm(eral - theta);

    (*astrom).sphi = phi.sin();
    (*astrom).cphi = phi.cos();

    (*astrom).refa = refa;
    (*astrom).refb = refb;

    (*astrom).diurab = 0.0;

    eraC2ixys(x, y, s, r.as_mut_ptr() as *mut f64);

    eraPvtob(
        elong,
        phi,
        hm,
        xp,
        yp,
        sp,
        theta,
        pvc.as_mut_ptr() as *mut f64,
    );

    eraTrxpv(
        r.as_mut_ptr() as *mut f64,
        pvc.as_mut_ptr() as *mut f64,
        pv.as_mut_ptr() as *mut f64,
    );

    eraApcs(date1, date2, pv.as_mut_ptr() as *mut f64, ebpv, ehp, astrom);

    eraCr(
        r.as_mut_ptr() as *mut f64,
        (*astrom).bpn.as_mut_ptr() as *mut f64,
    );
}

// G2/apco13.c

#[allow(non_snake_case)]
pub unsafe fn eraApco13(
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
    astrom: *mut eraASTROM,
    eo: *mut f64,
) -> i32 {
    use core::mem::MaybeUninit;

    let mut tai1 = 0.0;
    let mut tai2 = 0.0;
    if eraUtctai(utc1, utc2, &mut tai1, &mut tai2) < 0 {
        return -1;
    }

    let mut tt1 = 0.0;
    let mut tt2 = 0.0;
    eraTaitt(tai1, tai2, &mut tt1, &mut tt2);

    let mut ut11 = 0.0;
    let mut ut12 = 0.0;
    let j: i32 = eraUtcut1(utc1, utc2, dut1, &mut ut11, &mut ut12);
    if j < 0 {
        return -1;
    }

    let mut ehpv = [[0.0_f64; 3]; 2];
    let mut ebpv = [[0.0_f64; 3]; 2];
    let mut r = [[0.0_f64; 3]; 3];

    eraEpv00(
        tt1,
        tt2,
        ehpv.as_mut_ptr() as *mut f64,
        ebpv.as_mut_ptr() as *mut f64,
    );
    eraPnm06a(tt1, tt2, r.as_mut_ptr() as *mut f64);

    let (x, y) = {
        let mut x_u = MaybeUninit::<f64>::uninit();
        let mut y_u = MaybeUninit::<f64>::uninit();
        eraBpn2xy(
            r.as_mut_ptr() as *mut f64,
            x_u.as_mut_ptr(),
            y_u.as_mut_ptr(),
        );
        (x_u.assume_init(), y_u.assume_init())
    };

    let s = eraS06(tt1, tt2, x, y);
    let theta = eraEra00(ut11, ut12);
    let sp = eraSp00(tt1, tt2);

    let mut refa = 0.0;
    let mut refb = 0.0;
    eraRefco(phpa, tc, rh, wl, &mut refa, &mut refb);

    eraApco(
        tt1,
        tt2,
        ebpv.as_mut_ptr() as *mut f64,
        ehpv[0].as_mut_ptr(),
        x,
        y,
        s,
        theta,
        elong,
        phi,
        hm,
        xp,
        yp,
        sp,
        refa,
        refb,
        astrom,
    );

    *eo = eraEors(r.as_mut_ptr() as *mut f64, s);

    j
}

// G2/apcs.c

pub unsafe fn eraApcs(
    date1: f64,
    date2: f64,
    pv: *mut f64,
    ebpv: *mut f64,
    ehp: *mut f64,
    astrom: *mut eraASTROM,
) {
    const AUDMS: f64 = ERFA_DAU / ERFA_DAYSEC;

    const CR: f64 = ERFA_AULT / ERFA_DAYSEC;

    let mut pb = [0.0_f64; 3];
    let mut vb = [0.0_f64; 3];
    let mut ph = [0.0_f64; 3];

    (*astrom).pmt = ((date1 - ERFA_DJ00) + date2) / ERFA_DJY;

    let pv_pos = core::slice::from_raw_parts(pv, 6);
    let ebpv_slice = core::slice::from_raw_parts(ebpv, 6);
    let ehp_slice = core::slice::from_raw_parts(ehp, 3);

    for i in 0..3 {
        let dp = pv_pos[i] / ERFA_DAU;
        let dv = pv_pos[i + 3] / AUDMS;
        pb[i] = ebpv_slice[i] + dp;
        vb[i] = ebpv_slice[i + 3] + dv;
        ph[i] = ehp_slice[i] + dp;
    }

    eraCp(pb.as_ptr() as *mut f64, (*astrom).eb.as_mut_ptr());

    eraPn(
        ph.as_mut_ptr(),
        &mut (*astrom).em,
        (*astrom).eh.as_mut_ptr(),
    );

    let mut v2 = 0.0_f64;
    for i in 0..3 {
        let w = vb[i] * CR;
        (*astrom).v[i] = w;
        v2 += w * w;
    }
    (*astrom).bm1 = (1.0 - v2).sqrt();

    eraIr((*astrom).bpn.as_mut_ptr() as *mut f64);
}

// G2/apcs13.c

pub unsafe fn eraApcs13(date1: f64, date2: f64, pv: *mut f64, astrom: *mut eraASTROM) {
    let mut ehpv = [[0.0_f64; 3]; 2];
    let mut ebpv = [[0.0_f64; 3]; 2];

    eraEpv00(
        date1,
        date2,
        ehpv.as_mut_ptr() as *mut f64,
        ebpv.as_mut_ptr() as *mut f64,
    );

    eraApcs(
        date1,
        date2,
        pv,
        ebpv.as_mut_ptr() as *mut f64,
        ehpv[0].as_mut_ptr(),
        astrom,
    );
}

// G2/aper.c

pub unsafe fn eraAper(theta: f64, astrom: *mut eraASTROM) {
    (*astrom).eral = theta + (*astrom).along;
}
