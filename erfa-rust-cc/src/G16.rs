// G16
//   fk425.c  → eraFk425
//   fk45z.c  → eraFk45z
//   fk524.c  → eraFk524
//   fk52h.c  → eraFk52h
//   fk54z.c  → eraFk54z
//   fk5hz.c  → eraFk5hz
//   fw2m.c   → eraFw2m
//   fw2xy.c  → eraFw2xy

use crate::H1::*;

// fk425.c  →  eraFk425
pub unsafe fn eraFk425(
    r1950: f64,
    d1950: f64,
    dr1950: f64,
    dd1950: f64,
    p1950: f64,
    v1950: f64,
    r2000: *mut f64,
    d2000: *mut f64,
    dr2000: *mut f64,
    dd2000: *mut f64,
    p2000: *mut f64,
    v2000: *mut f64,
) {
    const PMF: f64 = 100.0 * ERFA_DR2AS;
    const TINY: f64 = 1.0e-30;
    const VF: f64 = 21.095;

    const A: [[f64; 3]; 2] = [
        [-1.62557e-6, -0.31919e-6, -0.13843e-6],
        [1.245e-3, -1.580e-3, -0.659e-3],
    ];

    const EM: [[[[f64; 3]; 2]; 3]; 2] = [
        [
            [
                [0.9999256782, -0.0111820611, -0.0048579477],
                [0.00000242395018, -0.00000002710663, -0.00000001177656],
            ],
            [
                [0.0111820610, 0.9999374784, -0.0000271765],
                [0.00000002710663, 0.00000242397878, -0.00000000006587],
            ],
            [
                [0.0048579479, -0.0000271474, 0.9999881997],
                [0.00000001177656, -0.00000000006582, 0.00000242410173],
            ],
        ],
        [
            [
                [-0.000551, -0.238565, 0.435739],
                [0.99994704, -0.01118251, -0.00485767],
            ],
            [
                [0.238514, -0.002667, -0.008541],
                [0.01118251, 0.99995883, -0.00002718],
            ],
            [
                [-0.435623, 0.012254, 0.002117],
                [0.00485767, -0.00002714, 1.00000956],
            ],
        ],
    ];

    let mut r = r1950;
    let mut d = d1950;
    let mut ur = dr1950 * PMF;
    let mut ud = dd1950 * PMF;
    let mut px = p1950;
    let mut rv = v1950;

    let pxvf = px * VF;
    let w_rv = rv * pxvf;

    let mut r0 = [[0.0_f64; 3]; 2];
    eraS2pv(r, d, 1.0, ur, ud, w_rv, r0.as_mut_ptr() as *mut f64);

    let mut pv1 = [[0.0_f64; 3]; 2];
    eraPvmpv(
        r0.as_mut_ptr() as *mut f64,
        A.as_ptr() as *mut f64,
        pv1.as_mut_ptr() as *mut f64,
    );

    let mut pv2 = [[0.0_f64; 3]; 2];
    eraSxp(
        eraPdp(r0.as_mut_ptr() as *mut f64, A[0].as_ptr() as *mut f64),
        r0[0].as_mut_ptr(),
        pv2[0].as_mut_ptr(),
    );
    eraSxp(
        eraPdp(r0.as_mut_ptr() as *mut f64, A[1].as_ptr() as *mut f64),
        r0[0].as_mut_ptr(),
        pv2[1].as_mut_ptr(),
    );
    eraPvppv(
        pv1.as_mut_ptr() as *mut f64,
        pv2.as_mut_ptr() as *mut f64,
        pv1.as_mut_ptr() as *mut f64,
    );

    for i in 0..2 {
        for j in 0..3 {
            let mut sum = 0.0;
            for k in 0..2 {
                for l in 0..3 {
                    sum += EM[i][j][k][l] * pv1[k][l];
                }
            }
            pv2[i][j] = sum;
        }
    }

    let mut rd = 0.0_f64;
    let mut w_scale = 0.0_f64;
    eraPv2s(
        pv2.as_mut_ptr() as *mut f64,
        &mut r,
        &mut d,
        &mut w_scale,
        &mut ur,
        &mut ud,
        &mut rd,
    );

    if px > TINY {
        rv = rd / pxvf;
        px = px / w_scale;
    }

    *r2000 = eraAnp(r);
    *d2000 = d;
    *dr2000 = ur / PMF;
    *dd2000 = ud / PMF;
    *p2000 = px;
    *v2000 = rv;
}

// fk45z.c  →  eraFk45z
pub unsafe fn eraFk45z(r1950: f64, d1950: f64, bepoch: f64, r2000: *mut f64, d2000: *mut f64) {
    const PMF: f64 = 100.0 * ERFA_DR2AS;

    const A: [f64; 3] = [-1.62557e-6, -0.31919e-6, -0.13843e-6];
    const AD: [f64; 3] = [1.245e-3, -1.580e-3, -0.659e-3];

    const EM: [[[f64; 3]; 3]; 2] = [
        [
            [0.9999256782, -0.0111820611, -0.0048579477],
            [0.0111820610, 0.9999374784, -0.0000271765],
            [0.0048579479, -0.0000271474, 0.9999881997],
        ],
        [
            [-0.000551, -0.238565, 0.435739],
            [0.238514, -0.002667, -0.008541],
            [-0.435623, 0.012254, 0.002117],
        ],
    ];

    let mut r0 = [0.0_f64; 3];
    eraS2c(r1950, d1950, r0.as_mut_ptr());

    let w_epoch = (bepoch - 1950.0) / PMF;
    let mut p = [0.0_f64; 3];
    eraPpsp(
        A.as_ptr() as *mut f64,
        w_epoch,
        AD.as_ptr() as *mut f64,
        p.as_mut_ptr(),
    );

    eraPpsp(
        p.as_mut_ptr(),
        -eraPdp(r0.as_mut_ptr(), p.as_mut_ptr()),
        r0.as_mut_ptr(),
        p.as_mut_ptr(),
    );
    eraPmp(r0.as_mut_ptr(), p.as_mut_ptr(), p.as_mut_ptr());

    let mut pv = [[0.0_f64; 3]; 2];
    for i in 0..2 {
        for j in 0..3 {
            let mut sum = 0.0;
            for k in 0..3 {
                sum += EM[i][j][k] * p[k];
            }
            pv[i][j] = sum;
        }
    }

    let mut djm0 = 0.0;
    let mut djm = 0.0;
    eraEpb2jd(bepoch, &mut djm0, &mut djm);
    let w_years = (eraEpj(djm0, djm) - 2000.0) / PMF;
    eraPvu(
        w_years,
        pv.as_mut_ptr() as *mut f64,
        pv.as_mut_ptr() as *mut f64,
    );

    let mut ra = 0.0_f64;
    eraC2s(pv[0].as_mut_ptr(), &mut ra, d2000);
    *r2000 = eraAnp(ra);
}

// fk524.c  →  eraFk524
pub unsafe fn eraFk524(
    r2000: f64,
    d2000: f64,
    dr2000: f64,
    dd2000: f64,
    p2000: f64,
    v2000: f64,
    r1950: *mut f64,
    d1950: *mut f64,
    dr1950: *mut f64,
    dd1950: *mut f64,
    p1950: *mut f64,
    v1950: *mut f64,
) {
    const PMF: f64 = 100.0 * ERFA_DR2AS;
    const TINY: f64 = 1.0e-30;
    const VF: f64 = 21.095;

    const A: [[f64; 3]; 2] = [
        [-1.62557e-6, -0.31919e-6, -0.13843e-6],
        [1.245e-3, -1.580e-3, -0.659e-3],
    ];

    const EM: [[[[f64; 3]; 2]; 3]; 2] = [
        [
            [
                [0.9999256795, 0.0111814828, 0.0048590039],
                [-0.00000242389840, -0.00000002710544, -0.00000001177742],
            ],
            [
                [-0.0111814828, 0.9999374849, -0.0000271771],
                [0.00000002710544, -0.00000242392702, 0.00000000006585],
            ],
            [
                [-0.0048590040, -0.0000271557, 0.9999881946],
                [0.00000001177742, 0.00000000006585, -0.00000242404995],
            ],
        ],
        [
            [
                [-0.000551, 0.238509, -0.435614],
                [0.99990432, 0.01118145, 0.00485852],
            ],
            [
                [-0.238560, -0.002667, 0.012254],
                [-0.01118145, 0.99991613, -0.00002717],
            ],
            [
                [0.435730, -0.008541, 0.002117],
                [-0.00485852, -0.00002716, 0.99996684],
            ],
        ],
    ];

    let pxvf = p2000 * VF;
    let w_rv = v2000 * pxvf;

    let mut r0 = [[0.0_f64; 3]; 2];
    eraS2pv(
        r2000,
        d2000,
        1.0,
        dr2000 * PMF,
        dd2000 * PMF,
        w_rv,
        r0.as_mut_ptr() as *mut f64,
    );

    let mut r1 = [[0.0_f64; 3]; 2];
    for i in 0..2 {
        for j in 0..3 {
            let mut sum = 0.0;
            for k in 0..2 {
                for l in 0..3 {
                    sum += EM[i][j][k][l] * r0[k][l];
                }
            }
            r1[i][j] = sum;
        }
    }

    let mut p1 = [0.0_f64; 3];
    let mut p2 = [0.0_f64; 3];
    let mut pv = [[0.0_f64; 3]; 2];

    let len_r1 = eraPm(r1[0].as_mut_ptr());
    eraSxp(
        eraPdp(r1[0].as_mut_ptr(), A[0].as_ptr() as *mut f64),
        r1[0].as_mut_ptr(),
        p1.as_mut_ptr(),
    );
    eraSxp(len_r1, A[0].as_ptr() as *mut f64, p2.as_mut_ptr());
    eraPmp(p2.as_mut_ptr(), p1.as_mut_ptr(), p1.as_mut_ptr());
    eraPpp(r1[0].as_mut_ptr(), p1.as_mut_ptr(), p1.as_mut_ptr());

    let len_p1 = eraPm(p1.as_mut_ptr());
    eraSxp(
        eraPdp(r1[0].as_mut_ptr(), A[0].as_ptr() as *mut f64),
        r1[0].as_mut_ptr(),
        p1.as_mut_ptr(),
    );
    eraSxp(len_p1, A[0].as_ptr() as *mut f64, p2.as_mut_ptr());
    eraPmp(p2.as_mut_ptr(), p1.as_mut_ptr(), p1.as_mut_ptr());
    eraPpp(r1[0].as_mut_ptr(), p1.as_mut_ptr(), pv[0].as_mut_ptr());

    eraSxp(
        eraPdp(r1[0].as_mut_ptr(), A[1].as_ptr() as *mut f64),
        pv[0].as_mut_ptr(),
        p1.as_mut_ptr(),
    );
    eraSxp(len_p1, A[1].as_ptr() as *mut f64, p2.as_mut_ptr());
    eraPmp(p2.as_mut_ptr(), p1.as_mut_ptr(), p1.as_mut_ptr());
    eraPpp(r1[1].as_mut_ptr(), p1.as_mut_ptr(), pv[1].as_mut_ptr());

    let mut ra = 0.0_f64;
    let mut dec = 0.0_f64;
    let mut w_scale = 0.0_f64;
    let mut ur = 0.0_f64;
    let mut ud = 0.0_f64;
    let mut rd = 0.0_f64;

    eraPv2s(
        pv.as_mut_ptr() as *mut f64,
        &mut ra,
        &mut dec,
        &mut w_scale,
        &mut ur,
        &mut ud,
        &mut rd,
    );

    let mut px = p2000;
    let mut rv = v2000;
    if px > TINY {
        rv = rd / pxvf;
        px = px / w_scale;
    }

    *r1950 = eraAnp(ra);
    *d1950 = dec;
    *dr1950 = ur / PMF;
    *dd1950 = ud / PMF;
    *p1950 = px;
    *v1950 = rv;
}

// fk52h.c  →  eraFk52h
pub unsafe fn eraFk52h(
    r5: f64,
    d5: f64,
    dr5: f64,
    dd5: f64,
    px5: f64,
    rv5: f64,
    rh: *mut f64,
    dh: *mut f64,
    drh: *mut f64,
    ddh: *mut f64,
    pxh: *mut f64,
    rvh: *mut f64,
) {
    let mut pv5 = [[0.0_f64; 3]; 2];
    eraStarpv(r5, d5, dr5, dd5, px5, rv5, pv5.as_mut_ptr() as *mut f64);

    let mut r5h = [[0.0_f64; 3]; 3];
    let mut s5h = [0.0_f64; 3];
    eraFk5hip(r5h.as_mut_ptr() as *mut f64, s5h.as_mut_ptr());

    for v in &mut s5h {
        *v /= 365.25;
    }

    let mut pvh = [[0.0_f64; 3]; 2];
    eraRxp(
        r5h.as_mut_ptr() as *mut f64,
        pv5[0].as_mut_ptr(),
        pvh[0].as_mut_ptr(),
    );

    let mut wxp = [0.0_f64; 3];
    eraPxp(pv5[0].as_mut_ptr(), s5h.as_mut_ptr(), wxp.as_mut_ptr());

    let mut vv = [0.0_f64; 3];
    eraPpp(wxp.as_mut_ptr(), pv5[1].as_mut_ptr(), vv.as_mut_ptr());

    eraRxp(
        r5h.as_mut_ptr() as *mut f64,
        vv.as_mut_ptr(),
        pvh[1].as_mut_ptr(),
    );

    eraPvstar(pvh.as_mut_ptr() as *mut f64, rh, dh, drh, ddh, pxh, rvh);
}

// fk54z.c  →  eraFk54z
pub unsafe fn eraFk54z(
    r2000: f64,
    d2000: f64,
    bepoch: f64,
    r1950: *mut f64,
    d1950: *mut f64,
    dr1950: *mut f64,
    dd1950: *mut f64,
) {
    let mut r = 0.0_f64;
    let mut d = 0.0_f64;
    let mut pr = 0.0_f64;
    let mut pd = 0.0_f64;
    let mut px = 0.0_f64;
    let mut rv = 0.0_f64;

    eraFk524(
        r2000, d2000, 0.0, 0.0, 0.0, 0.0, &mut r, &mut d, &mut pr, &mut pd, &mut px, &mut rv,
    );

    let mut p = [0.0_f64; 3];
    eraS2c(r, d, p.as_mut_ptr());

    let mut v = [0.0_f64; 3];
    v[0] = -pr * p[1] - pd * r.cos() * d.sin();
    v[1] = pr * p[0] - pd * r.sin() * d.sin();
    v[2] = pd * d.cos();

    let dt = bepoch - 1950.0;
    for i in 0..3 {
        p[i] += dt * v[i];
    }

    let mut ra = 0.0_f64;
    eraC2s(p.as_mut_ptr(), &mut ra, d1950);
    *r1950 = eraAnp(ra);
    *dr1950 = pr;
    *dd1950 = pd;
}

// fk5hz.c  →  eraFk5hz
pub unsafe fn eraFk5hz(r5: f64, d5: f64, date1: f64, date2: f64, rh: *mut f64, dh: *mut f64) {
    let t = -((date1 - ERFA_DJ00) + date2) / ERFA_DJY;

    let mut p5e = [0.0_f64; 3];
    eraS2c(r5, d5, p5e.as_mut_ptr());

    let mut r5h = [[0.0_f64; 3]; 3];
    let mut s5h = [0.0_f64; 3];
    eraFk5hip(r5h.as_mut_ptr() as *mut f64, s5h.as_mut_ptr());

    let mut vst = [0.0_f64; 3];
    eraSxp(t, s5h.as_mut_ptr(), vst.as_mut_ptr());

    let mut rst = [[0.0_f64; 3]; 3];
    eraRv2m(vst.as_mut_ptr(), rst.as_mut_ptr() as *mut f64);

    let mut p5 = [0.0_f64; 3];
    eraTrxp(
        rst.as_mut_ptr() as *mut f64,
        p5e.as_mut_ptr(),
        p5.as_mut_ptr(),
    );

    let mut ph = [0.0_f64; 3];
    eraRxp(
        r5h.as_mut_ptr() as *mut f64,
        p5.as_mut_ptr(),
        ph.as_mut_ptr(),
    );

    let mut ra = 0.0_f64;
    eraC2s(ph.as_mut_ptr(), &mut ra, dh);
    *rh = eraAnp(ra);
}

// fw2m.c  →  eraFw2m
pub unsafe fn eraFw2m(gamb: f64, phib: f64, psi: f64, eps: f64, r: *mut f64) {
    eraIr(r);
    eraRz(gamb, r);
    eraRx(phib, r);
    eraRz(-psi, r);
    eraRx(-eps, r);
}

// fw2xy.c  →  eraFw2xy
pub unsafe fn eraFw2xy(gamb: f64, phib: f64, psi: f64, eps: f64, x: *mut f64, y: *mut f64) {
    let mut rmat = [[0.0_f64; 3]; 3];
    eraFw2m(gamb, phib, psi, eps, rmat.as_mut_ptr() as *mut f64);
    eraBpn2xy(rmat.as_mut_ptr() as *mut f64, x, y);
}
