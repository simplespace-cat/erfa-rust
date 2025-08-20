// G20
//   ld.c      → eraLd_safe
//   ldn.c     → eraLdn_safe
//   ldsun.c   → eraLdsun_safe
//   lteceq.c  → eraLteceq_safe
//   ltecm.c   → eraLtecm_safe
//   lteqec.c  → eraLteqec_safe
//   ltp.c     → eraLtp_safe
//   ltpb.c    → eraLtpb_safe
//   ltpecl.c  → eraLtpecl_safe
//   ltpequ.c  → eraLtpequ_safe

use crate::G1_safe::{eraAnp_safe, eraAnpm_safe};
use crate::G24_safe::eraPdp_safe;
use crate::G25_safe::{eraPmp_safe, eraPn_safe};
use crate::G26_safe::eraPpsp_safe;
use crate::G27_safe::eraPxp_safe;
use crate::G28_safe::eraRxp_safe;
use crate::G29_safe::eraS2c_safe;
use crate::G33_safe::eraTrxp_safe;
use crate::G7_safe::eraC2s_safe;
use crate::H1_safe::{
    eraLDBODY, ERFA_AULT, ERFA_D2PI, ERFA_DAS2R, ERFA_DAYSEC, ERFA_GMAX, ERFA_GMIN, ERFA_SRS,
};

pub type ErfaResult<T> = Result<T, ()>;

//----------------------------------------------------------------------
// G20/ld.c → eraLd_safe
//----------------------------------------------------------------------
// Single-body light deflection; returns deflected unit vector.
pub fn eraLd_safe(
    bm: f64,
    p: &[f64; 3],
    q: &[f64; 3],
    e: &[f64; 3],
    em: f64,
    dlim: f64,
) -> ErfaResult<[f64; 3]> {
    let qpe = [q[0] + e[0], q[1] + e[1], q[2] + e[2]];
    let qdqpe = eraPdp_safe(q, &qpe)?;
    let w = bm * ERFA_SRS / em / ERFA_GMAX(qdqpe, dlim);

    let eq = eraPxp_safe(e, q)?;
    let peq = eraPxp_safe(p, &eq)?;

    Ok([p[0] + w * peq[0], p[1] + w * peq[1], p[2] + w * peq[2]])
}

//----------------------------------------------------------------------
// G20/ldn.c → eraLdn_safe
//----------------------------------------------------------------------
// N-body light deflection; returns deflected unit vector.
pub fn eraLdn_safe(bodies: &[eraLDBODY], ob: &[f64; 3], sc: &[f64; 3]) -> ErfaResult<[f64; 3]> {
    // Light time for 1 au (days).
    const CR: f64 = ERFA_AULT / ERFA_DAYSEC;

    let mut sn = *sc;

    for body in bodies {
        // Body→observer at observation epoch (au).
        let v = eraPmp_safe(ob, &body.pv[0])?;

        // Minus light-time since ray passed body (days).
        let dt = ERFA_GMIN(eraPdp_safe(&sn, &v)? * CR, 0.0);

        // Backtrack body to retarded epoch.
        let ev = eraPpsp_safe(&v, -dt, &body.pv[1])?;

        // Unit vector and distance body→observer.
        let (em, e) = eraPn_safe(&ev)?;

        // Apply deflection for this body (use sn as both p and q).
        sn = eraLd_safe(body.bm, &sn, &sn, &e, em, body.dl)?;
    }

    Ok(sn)
}

//----------------------------------------------------------------------
// G20/ldsun.c → eraLdsun_safe
//----------------------------------------------------------------------
// Solar light deflection; convenience wrapper around eraLd_safe.
pub fn eraLdsun_safe(p: &[f64; 3], e: &[f64; 3], em: f64) -> ErfaResult<[f64; 3]> {
    let em2 = em * em;
    let dlim = 1.0e-6 / if em2 < 1.0 { 1.0 } else { em2 };
    eraLd_safe(1.0, p, p, e, em, dlim)
}

//----------------------------------------------------------------------
// G20/lteceq.c → eraLteceq_safe
//----------------------------------------------------------------------
// Long-term model: ecliptic (dl, db) at epoch epj → ICRS (dr, dd).
pub fn eraLteceq_safe(epj: f64, dl: f64, db: f64) -> ErfaResult<(f64, f64)> {
    let v1 = eraS2c_safe(dl, db)?;
    let rm = eraLtecm_safe(epj)?;
    let v2 = eraTrxp_safe(&rm, &v1)?;
    let (a, b) = eraC2s_safe(&v2)?;
    let dr = eraAnp_safe(a)?;
    let dd = eraAnpm_safe(b)?;
    Ok((dr, dd))
}

//----------------------------------------------------------------------
// G20/ltecm.c → eraLtecm_safe
//----------------------------------------------------------------------
// Build rotation matrix ICRS←ecliptic at epoch epj (long-term).
pub fn eraLtecm_safe(epj: f64) -> ErfaResult<[[f64; 3]; 3]> {
    // Frame-bias constants (rad).
    const DX: f64 = -0.016_617 * ERFA_DAS2R;
    const DE: f64 = -0.006_819_2 * ERFA_DAS2R;
    const DR: f64 = -0.014_6 * ERFA_DAS2R;

    let p = eraLtpequ_safe(epj)?;
    let z = eraLtpecl_safe(epj)?;

    // Equinox vector x = unit(p × z).
    let w = eraPxp_safe(&p, &z)?;
    let (_s, x) = eraPn_safe(&w)?;

    // y = z × x.
    let y = eraPxp_safe(&z, &x)?;

    // Matrix rows with small frame-bias terms.
    let mut m = [[0.0_f64; 3]; 3];

    m[0][0] = x[0] - x[1] * DR + x[2] * DX;
    m[0][1] = x[0] * DR + x[1] + x[2] * DE;
    m[0][2] = -x[0] * DX - x[1] * DE + x[2];

    m[1][0] = y[0] - y[1] * DR + y[2] * DX;
    m[1][1] = y[0] * DR + y[1] + y[2] * DE;
    m[1][2] = -y[0] * DX - y[1] * DE + y[2];

    m[2][0] = z[0] - z[1] * DR + z[2] * DX;
    m[2][1] = z[0] * DR + z[1] + z[2] * DE;
    m[2][2] = -z[0] * DX - z[1] * DE + z[2];

    Ok(m)
}

//----------------------------------------------------------------------
// G20/lteqec.c → eraLteqec_safe
//----------------------------------------------------------------------
// Long-term model: ICRS (dr, dd) → ecliptic (dl, db) at epoch epj.
pub fn eraLteqec_safe(epj: f64, dr: f64, dd: f64) -> ErfaResult<(f64, f64)> {
    let v1 = eraS2c_safe(dr, dd)?;
    let rm = eraLtecm_safe(epj)?;
    let v2 = eraRxp_safe(&rm, &v1)?;
    let (a, b) = eraC2s_safe(&v2)?;
    let dl = eraAnp_safe(a)?;
    let db = eraAnpm_safe(b)?;
    Ok((dl, db))
}

//----------------------------------------------------------------------
// G20/ltp.c → eraLtp_safe
//----------------------------------------------------------------------
// Long-term precession matrix at epoch epj.
pub fn eraLtp_safe(epj: f64) -> ErfaResult<[[f64; 3]; 3]> {
    let peqr = eraLtpequ_safe(epj)?;
    let pecl = eraLtpecl_safe(epj)?;

    let v = eraPxp_safe(&peqr, &pecl)?;
    let (_w, eqx) = eraPn_safe(&v)?;

    let v2 = eraPxp_safe(&peqr, &eqx)?;

    let mut m = [[0.0_f64; 3]; 3];
    for i in 0..3 {
        m[0][i] = eqx[i];
        m[1][i] = v2[i];
        m[2][i] = peqr[i];
    }
    Ok(m)
}

//----------------------------------------------------------------------
// G20/ltpb.c → eraLtpb_safe
//----------------------------------------------------------------------
// Long-term precession+bias matrix at epoch epj.
pub fn eraLtpb_safe(epj: f64) -> ErfaResult<[[f64; 3]; 3]> {
    const DX: f64 = -0.016_617 * ERFA_DAS2R;
    const DE: f64 = -0.006_819_2 * ERFA_DAS2R;
    const DR: f64 = -0.014_6 * ERFA_DAS2R;

    let rp = eraLtp_safe(epj)?;

    let mut rpb = [[0.0_f64; 3]; 3];
    for i in 0..3 {
        rpb[i][0] = rp[i][0] - rp[i][1] * DR + rp[i][2] * DX;
        rpb[i][1] = rp[i][0] * DR + rp[i][1] + rp[i][2] * DE;
        rpb[i][2] = -rp[i][0] * DX - rp[i][1] * DE + rp[i][2];
    }
    Ok(rpb)
}

//----------------------------------------------------------------------
// G20/ltpecl.c → eraLtpecl_safe
//----------------------------------------------------------------------
// Long-term ecliptic pole unit vector at epoch epj.
pub fn eraLtpecl_safe(epj: f64) -> ErfaResult<[f64; 3]> {
    // Obliquity at J2000.
    const EPS0: f64 = 84_381.406 * ERFA_DAS2R;

    // Polynomial coefficients.
    const PQPOL: [[f64; 4]; 2] = [
        [5_851.607_687, -0.118_900_0, -0.000_289_13, 0.000_000_101],
        [-1_600.886_300, 1.168_981_8, -0.000_000_20, -0.000_000_437],
    ];

    // Periodic coefficients.
    const PQPER: [[f64; 5]; 8] = [
        [
            708.15,
            -5_486.751_211,
            -684.661_560,
            667.666_730,
            -5_523.863_691,
        ],
        [
            2_309.00,
            -17.127_623,
            2_446.283_880,
            -2_354.886_252,
            -549.747_450,
        ],
        [
            1_620.00,
            -617.517_403,
            399.671_049,
            -428.152_441,
            -310.998_056,
        ],
        [492.20, 413.442_940, -356.652_376, 376.202_861, 421.535_876],
        [1_183.00, 78.614_193, -186.387_003, 184.778_874, -36.776_172],
        [
            622.00,
            -180.732_815,
            -316.800_070,
            335.321_713,
            -145.278_396,
        ],
        [882.00, -87.676_083, 198.296_701, -185.138_669, -34.744_450],
        [547.00, 46.140_315, 101.135_679, -120.972_830, 22.885_731],
    ];

    // Centuries since J2000.
    let t = (epj - 2000.0) / 100.0;

    // Periodic part.
    let mut p = 0.0_f64;
    let mut q = 0.0_f64;
    let w = ERFA_D2PI * t;
    for coeff in PQPER {
        let a = w / coeff[0];
        let (s, c) = a.sin_cos();
        p += c * coeff[1] + s * coeff[3];
        q += c * coeff[2] + s * coeff[4];
    }

    // Polynomial part.
    let mut tt = 1.0_f64;
    for i in 0..4 {
        p += PQPOL[0][i] * tt;
        q += PQPOL[1][i] * tt;
        tt *= t;
    }

    // To radians.
    let p = p * ERFA_DAS2R;
    let q = q * ERFA_DAS2R;

    // Form pole vector.
    let mut wv = 1.0 - p * p - q * q;
    wv = if wv < 0.0 { 0.0 } else { wv.sqrt() };
    let (s0, c0) = EPS0.sin_cos();

    Ok([p, -q * c0 - wv * s0, -q * s0 + wv * c0])
}

//----------------------------------------------------------------------
// G20/ltpequ.c → eraLtpequ_safe
//----------------------------------------------------------------------
// Long-term equator pole unit vector at epoch epj.
pub fn eraLtpequ_safe(epj: f64) -> ErfaResult<[f64; 3]> {
    // Polynomial coefficients.
    const XYPOL: [[f64; 4]; 2] = [
        [5_453.282_155, 0.425_284_1, -0.000_371_73, -0.000_000_152],
        [-73_750.930_350, -0.767_545_2, -0.000_187_25, 0.000_000_231],
    ];

    // Periodic coefficients.
    const XYPER: [[f64; 5]; 14] = [
        [
            256.75,
            -819.940_624,
            75_004.344_875,
            81_491.287_984,
            1_558.515_853,
        ],
        [
            708.15,
            -8_444.676_815,
            624.033_993,
            787.163_481,
            7_774.939_698,
        ],
        [
            274.20,
            2_600.009_459,
            1_251.136_893,
            1_251.296_102,
            -2_219.534_038,
        ],
        [
            241.45,
            2_755.175_630,
            -1_102.212_834,
            -1_257.950_837,
            -2_523.969_396,
        ],
        [
            2_309.00,
            -167.659_835,
            -2_660.664_980,
            -2_966.799_730,
            247.850_422,
        ],
        [492.20, 871.855_056, 699.291_817, 639.744_522, -846.485_643],
        [396.10, 44.769_698, 153.167_220, 131.600_209, -1_393.124_055],
        [
            288.90,
            -512.313_065,
            -950.865_637,
            -445.040_117,
            368.526_116,
        ],
        [231.10, -819.415_595, 499.754_645, 584.522_874, 749.045_012],
        [
            1_610.00,
            -538.071_099,
            -145.188_210,
            -89.756_563,
            444.704_518,
        ],
        [620.00, -189.793_622, 558.116_553, 524.429_630, 235.934_465],
        [157.87, -402.922_932, -23.923_029, -13.549_067, 374.049_623],
        [
            220.30,
            179.516_345,
            -165.405_086,
            -210.157_124,
            -171.330_180,
        ],
        [1_200.00, -9.814_756, 9.344_131, -44.919_798, -22.899_655],
    ];

    let t = (epj - 2000.0) / 100.0;

    // Periodic contribution.
    let mut x = 0.0_f64;
    let mut y = 0.0_f64;
    let w = ERFA_D2PI * t;
    for coeff in XYPER {
        let a = w / coeff[0];
        let (s, c) = a.sin_cos();
        x += c * coeff[1] + s * coeff[3];
        y += c * coeff[2] + s * coeff[4];
    }

    // Polynomial contribution.
    let mut tt = 1.0_f64;
    for i in 0..4 {
        x += XYPOL[0][i] * tt;
        y += XYPOL[1][i] * tt;
        tt *= t;
    }

    // To radians and form vector.
    let x = x * ERFA_DAS2R;
    let y = y * ERFA_DAS2R;
    let wv = 1.0 - x * x - y * y;
    let z = if wv < 0.0 { 0.0 } else { wv.sqrt() };
    Ok([x, y, z])
}
