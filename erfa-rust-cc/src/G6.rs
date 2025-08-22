// G6
//   bi00.c   → eraBi00
//   bp00.c   → eraBp00
//   bp06.c   → eraBp06
//   bpn2xy.c → eraBpn2xy

use crate::H1::*;

// eraBi00  (bi00.c)
pub unsafe fn eraBi00(dpsibi: *mut f64, depsbi: *mut f64, dra: *mut f64) {
    const DPBIAS: f64 = -0.041_775 * ERFA_DAS2R;
    const DEBIAS: f64 = -0.006_819_2 * ERFA_DAS2R;

    const DRA0: f64 = -0.0146 * ERFA_DAS2R;

    *dpsibi = DPBIAS;
    *depsbi = DEBIAS;
    *dra = DRA0;
}

// eraBp00  (bp00.c)
pub unsafe fn eraBp00(date1: f64, date2: f64, rb: *mut f64, rp: *mut f64, rbp: *mut f64) {
    const EPS0: f64 = 84_381.448 * ERFA_DAS2R;

    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    let (mut dpsibi, mut depsbi, mut dra0) = (0.0, 0.0, 0.0);
    eraBi00(&mut dpsibi, &mut depsbi, &mut dra0);

    let psia77 = (5038.7784 + (-1.07259 + -0.001_147 * t) * t) * t * ERFA_DAS2R;
    let oma77 = EPS0 + (0.05127 + -0.007_726 * t) * t * t * ERFA_DAS2R;
    let chia = (10.5526 + (-2.38064 + -0.001_125 * t) * t) * t * ERFA_DAS2R;

    let (mut dpsipr, mut depspr) = (0.0, 0.0);
    eraPr00(date1, date2, &mut dpsipr, &mut depspr);
    let psia = psia77 + dpsipr;
    let oma = oma77 + depspr;

    let mut rbw = [[0.0_f64; 3]; 3];

    eraIr(rbw.as_mut_ptr() as *mut f64);
    eraRz(dra0, rbw.as_mut_ptr() as *mut f64);
    eraRy(dpsibi * EPS0.sin(), rbw.as_mut_ptr() as *mut f64);
    eraRx(-depsbi, rbw.as_mut_ptr() as *mut f64);
    eraCr(rbw.as_mut_ptr() as *mut f64, rb);

    eraIr(rp);
    eraRx(EPS0, rp);
    eraRz(-psia, rp);
    eraRx(-oma, rp);
    eraRz(chia, rp);

    eraRxr(rp, rbw.as_mut_ptr() as *mut f64, rbp);
}

// eraBp06  (bp06.c)
pub unsafe fn eraBp06(date1: f64, date2: f64, rb: *mut f64, rp: *mut f64, rbp: *mut f64) {
    let (mut gamb, mut phib, mut psib, mut epsa) = (0.0, 0.0, 0.0, 0.0);
    let mut rbpw = [[0.0_f64; 3]; 3];
    let mut rbt = [[0.0_f64; 3]; 3];

    eraPfw06(
        ERFA_DJM0, ERFA_DJM00, &mut gamb, &mut phib, &mut psib, &mut epsa,
    );
    eraFw2m(gamb, phib, psib, epsa, rb);

    eraPmat06(date1, date2, rbpw.as_mut_ptr() as *mut f64);

    eraTr(rb, rbt.as_mut_ptr() as *mut f64);
    eraRxr(
        rbpw.as_mut_ptr() as *mut f64,
        rbt.as_mut_ptr() as *mut f64,
        rp,
    );

    eraCr(rbpw.as_mut_ptr() as *mut f64, rbp);
}

// eraBpn2xy  (bpn2xy.c)
pub unsafe fn eraBpn2xy(rbpn: *mut f64, x: *mut f64, y: *mut f64) {
    let m = core::slice::from_raw_parts(rbpn, 9);
    *x = m[6];
    *y = m[7];
}

// end of file G6.rs
