// G17
//   g2icrs.c  → eraG2icrs
//   gc2gd.c   → eraGc2gd
//   gc2gde.c  → eraGc2gde
//   gd2gc.c   → eraGd2gc
//   gd2gce.c  → eraGd2gce
//   gmst00.c  → eraGmst00
//   gmst06.c  → eraGmst06
//   gmst82.c  → eraGmst82
//   gst00a.c  → eraGst00a
//   gst00b.c  → eraGst00b
//   gst06.c   → eraGst06
//   gst06a.c  → eraGst06a
//   gst94.c   → eraGst94

use crate::H1::*;

// eraG2icrs    Galactic → ICRS
pub unsafe fn eraG2icrs(dl: f64, db: f64, dr: *mut f64, dd: *mut f64) {
    let mut v1 = [0.0_f64; 3];
    let mut v2 = [0.0_f64; 3];

    const R: [f64; 9] = [
        -0.054875560416215368492398900454,
        -0.873437090234885048760383168409,
        -0.483835015548713226831774175116,
        0.494109427875583673525222371358,
        -0.444829629960011178146614061616,
        0.746982244497218890527388004556,
        -0.867666149019004701181616534570,
        -0.198076373431201528180486091412,
        0.455983776175066922272100478348,
    ];

    eraS2c(dl, db, v1.as_mut_ptr());

    eraTrxp(R.as_ptr() as *mut f64, v1.as_mut_ptr(), v2.as_mut_ptr());

    eraC2s(v2.as_mut_ptr(), dr, dd);

    *dr = eraAnp(*dr);
    *dd = eraAnpm(*dd);
}

// eraGc2gd    Geocentric → Geodetic  (reference ellipsoid selector)
pub unsafe fn eraGc2gd(
    n: i32,
    xyz: *mut f64,
    elong: *mut f64,
    phi: *mut f64,
    height: *mut f64,
) -> i32 {
    let mut a = 0.0_f64;
    let mut f = 0.0_f64;

    let mut j = eraEform(n, &mut a, &mut f);

    if j == 0 {
        j = eraGc2gde(a, f, xyz, elong, phi, height);
        if j < 0 {
            j = -2;
        }
    }

    if j < 0 {
        *elong = -1e9;
        *phi = -1e9;
        *height = -1e9;
    }

    j
}

// eraGc2gde    Geocentric → Geodetic (given a,f)
pub unsafe fn eraGc2gde(
    a: f64,
    f: f64,
    xyz: *mut f64,
    elong: *mut f64,
    phi: *mut f64,
    height: *mut f64,
) -> i32 {
    if f < 0.0 || f >= 1.0 {
        return -1;
    }
    if a <= 0.0 {
        return -2;
    }

    let aeps2 = a * a * 1e-32;
    let e2 = (2.0 - f) * f;
    let e4t = e2 * e2 * 1.5;
    let ec2 = 1.0 - e2;
    if ec2 <= 0.0 {
        return -1;
    }
    let ec = ec2.sqrt();
    let b = a * ec;

    let x = *xyz.add(0);
    let y = *xyz.add(1);
    let z = *xyz.add(2);

    let p2 = x * x + y * y;

    *elong = if p2 > 0.0 { y.atan2(x) } else { 0.0 };

    let absz = z.abs();

    if p2 > aeps2 {
        let p = p2.sqrt();
        let s0 = absz / a;
        let pn = p / a;
        let zc = ec * s0;

        let c0 = ec * pn;
        let c02 = c0 * c0;
        let c03 = c02 * c0;
        let s02 = s0 * s0;
        let s03 = s02 * s0;
        let a02 = c02 + s02;
        let a0 = a02.sqrt();
        let a03 = a02 * a0;
        let d0 = zc * a03 + e2 * s03;
        let f0 = pn * a03 - e2 * c03;

        let b0 = e4t * s02 * c02 * pn * (a0 - ec);
        let s1 = d0 * f0 - b0 * s0;
        let cc = ec * (f0 * f0 - b0 * c0);

        *phi = (s1 / cc).atan();

        let s12 = s1 * s1;
        let cc2 = cc * cc;
        *height = (p * cc + absz * s1 - a * (ec2 * s12 + cc2).sqrt()) / (s12 + cc2).sqrt();
    } else {
        *phi = ERFA_DPI / 2.0;
        *height = absz - b;
    }

    if z < 0.0 {
        *phi = -*phi;
    }

    0
}

// eraGd2gc    Geodetic → Geocentric (ellipsoid selector)
pub unsafe fn eraGd2gc(n: i32, elong: f64, phi: f64, height: f64, xyz: *mut f64) -> i32 {
    let mut a = 0.0_f64;
    let mut f = 0.0_f64;
    let mut j = eraEform(n, &mut a, &mut f);

    if j == 0 {
        j = eraGd2gce(a, f, elong, phi, height, xyz);
        if j != 0 {
            j = -2;
        }
    }

    if j != 0 {
        eraZp(xyz);
    }
    j
}

// eraGd2gce    Geodetic → Geocentric (given a,f)
pub unsafe fn eraGd2gce(a: f64, f: f64, elong: f64, phi: f64, height: f64, xyz: *mut f64) -> i32 {
    let sp = phi.sin();
    let cp = phi.cos();
    let mut w = 1.0 - f;
    w *= w;
    let d = cp * cp + w * sp * sp;
    if d <= 0.0 {
        return -1;
    }

    let ac = a / d.sqrt();
    let as_ = w * ac;

    let r = (ac + height) * cp;
    *xyz.add(0) = r * elong.cos();
    *xyz.add(1) = r * elong.sin();
    *xyz.add(2) = (as_ + height) * sp;

    0
}

// eraGmst00    Greenwich Mean Sidereal Time, IAU 2000
pub unsafe fn eraGmst00(uta: f64, utb: f64, tta: f64, ttb: f64) -> f64 {
    let t = ((tta - ERFA_DJ00) + ttb) / ERFA_DJC;

    eraAnp(
        eraEra00(uta, utb)
            + (0.014506
                + (4612.15739966 + (1.39667721 + (-0.00009344 + (0.00001882) * t) * t) * t) * t)
                * ERFA_DAS2R,
    )
}

// eraGmst06    Greenwich Mean Sidereal Time, IAU 2006
pub unsafe fn eraGmst06(uta: f64, utb: f64, tta: f64, ttb: f64) -> f64 {
    let t = ((tta - ERFA_DJ00) + ttb) / ERFA_DJC;

    eraAnp(
        eraEra00(uta, utb)
            + (0.014506
                + (4612.156534
                    + (1.3915817 + (-0.00000044 + (-0.000029956 + (-0.0000000368) * t) * t) * t)
                        * t)
                    * t)
                * ERFA_DAS2R,
    )
}

// eraGmst82    Greenwich Mean Sidereal Time, IAU 1982
pub unsafe fn eraGmst82(dj1: f64, dj2: f64) -> f64 {
    const A: f64 = 24110.54841 - ERFA_DAYSEC / 2.0;
    const B: f64 = 8640184.812866;
    const C: f64 = 0.093104;
    const D: f64 = -6.2e-6;

    let (d1, d2) = if dj1 < dj2 { (dj1, dj2) } else { (dj2, dj1) };

    let t = (d1 + (d2 - ERFA_DJ00)) / ERFA_DJC;

    let f = ERFA_DAYSEC * (d1.fract() + d2.fract());

    eraAnp(ERFA_DS2R * (A + (B + (C + D * t) * t) * t + f))
}

// eraGst00a    Greenwich Apparent Sidereal Time, IAU 2000A
pub unsafe fn eraGst00a(uta: f64, utb: f64, tta: f64, ttb: f64) -> f64 {
    let gmst00 = eraGmst00(uta, utb, tta, ttb);
    let ee00a = eraEe00a(tta, ttb);
    eraAnp(gmst00 + ee00a)
}

// eraGst00b    Greenwich Apparent Sidereal Time, IAU 2000B
pub unsafe fn eraGst00b(uta: f64, utb: f64) -> f64 {
    let gmst00 = eraGmst00(uta, utb, uta, utb);
    let ee00b = eraEe00b(uta, utb);
    eraAnp(gmst00 + ee00b)
}

// eraGst06    Greenwich Apparent Sidereal Time, IAU 2006 (matrix given)
pub unsafe fn eraGst06(uta: f64, utb: f64, tta: f64, ttb: f64, rnpb: *mut f64) -> f64 {
    let mut x = 0.0_f64;
    let mut y = 0.0_f64;
    eraBpn2xy(rnpb, &mut x, &mut y);

    let s = eraS06(tta, ttb, x, y);
    let era = eraEra00(uta, utb);
    let eors = eraEors(rnpb, s);

    eraAnp(era - eors)
}

// eraGst06a    Greenwich Apparent Sidereal Time, IAU 2006/2000A
pub unsafe fn eraGst06a(uta: f64, utb: f64, tta: f64, ttb: f64) -> f64 {
    let mut rnpb = [0.0_f64; 9];
    eraPnm06a(tta, ttb, rnpb.as_mut_ptr());

    eraGst06(uta, utb, tta, ttb, rnpb.as_mut_ptr())
}

// eraGst94    Greenwich Apparent Sidereal Time, IAU 1982/94
pub unsafe fn eraGst94(uta: f64, utb: f64) -> f64 {
    let gmst82 = eraGmst82(uta, utb);
    let eqeq94 = eraEqeq94(uta, utb);
    eraAnp(gmst82 + eqeq94)
}
