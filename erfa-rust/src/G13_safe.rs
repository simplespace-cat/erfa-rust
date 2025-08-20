// G13
//   epv00.c → eraEpv00_safe

use crate::H1_safe::{ERFA_DJ00, ERFA_DJY};

#[path = "data/G13_safe/E0Z.rs"]
mod e0z_mod;
use e0z_mod::E0Z;
#[path = "data/G13_safe/E0X.rs"]
mod e0x_mod;
use e0x_mod::E0X;
#[path = "data/G13_safe/E0Y.rs"]
mod e0y_mod;
use e0y_mod::E0Y;
#[path = "data/G13_safe/E1X.rs"]
mod e1x_mod;
use e1x_mod::E1X;
#[path = "data/G13_safe/E1Y.rs"]
mod e1y_mod;
use e1y_mod::E1Y;
#[path = "data/G13_safe/E1Z.rs"]
mod e1z_mod;
use e1z_mod::E1Z;
#[path = "data/G13_safe/S0X.rs"]
mod s0x_mod;
use s0x_mod::S0X;
#[path = "data/G13_safe/S0Y.rs"]
mod s0y_mod;
use s0y_mod::S0Y;
#[path = "data/G13_safe/S0Z.rs"]
mod s0z_mod;
use s0z_mod::S0Z;
#[path = "data/G13_safe/S1X.rs"]
mod s1x_mod;
use s1x_mod::S1X;
#[path = "data/G13_safe/S1Y.rs"]
mod s1y_mod;
use s1y_mod::S1Y;
#[path = "data/G13_safe/S2Y.rs"]
mod s2y_mod;
use s2y_mod::S2Y;
#[path = "data/G13_safe/S2X.rs"]
mod s2x_mod;
use s2x_mod::S2X;
#[path = "data/G13_safe/S1Z.rs"]
mod s1z_mod;
use s1z_mod::S1Z;

pub type ErfaResult<T> = Result<T, ()>;

// Orientation matrix aligning simplified VSOP2000 to DE405.
const AM12: f64 = 0.000000211284;
const AM13: f64 = -0.000000091603;
const AM21: f64 = -0.000000230286;
const AM22: f64 = 0.917482137087;
const AM23: f64 = -0.397776982902;
const AM32: f64 = 0.397776982902;
const AM33: f64 = 0.917482137087;

// Exact constants as in the original source (keep scientific notation).
static E2X: &[f64] = &[
    -0.4143818297913e-10,
    0.0000000000000e+00,
    0.0000000000000e+00,
    0.2171497694435e-10,
    0.4398225628264e+01,
    0.1256615170089e+02,
    0.9845398442516e-11,
    0.2079720838384e+00,
    0.6283075850446e+01,
    0.9256833552682e-12,
    0.4191264694361e+01,
    0.1884922755134e+02,
    0.1022049384115e-12,
    0.5381133195658e+01,
    0.8399684731857e+02,
];
static E2Y: &[f64] = &[
    0.5063375872532e-10,
    0.0000000000000e+00,
    0.0000000000000e+00,
    0.2173815785980e-10,
    0.2827805833053e+01,
    0.1256615170089e+02,
    0.1010231999920e-10,
    0.4634612377133e+01,
    0.6283075850446e+01,
    0.9259745317636e-12,
    0.2620612076189e+01,
    0.1884922755134e+02,
    0.1022202095812e-12,
    0.3809562326066e+01,
    0.8399684731857e+02,
];
static E2Z: &[f64] = &[
    0.9722666114891e-10,
    0.5152219582658e+01,
    0.6283075850446e+01,
    -0.3494819171909e-11,
    0.0000000000000e+00,
    0.0000000000000e+00,
    0.6713034376076e-12,
    0.6440188750495e+00,
    0.1256615170089e+02,
];

static S2Z: &[f64] = &[
    0.3749920358054e-12,
    0.3230285558668e+01,
    0.2132990797783e+00,
    0.2735037220939e-12,
    0.6154322683046e+01,
    0.5296909721118e+00,
];

const CE0: [&[f64]; 3] = [E0X, E0Y, E0Z];
const CE1: [&[f64]; 3] = [E1X, E1Y, E1Z];
const CE2: [&[f64]; 3] = [E2X, E2Y, E2Z];
const CS0: [&[f64]; 3] = [S0X, S0Y, S0Z];
const CS1: [&[f64]; 3] = [S1X, S1Y, S1Z];
const CS2: [&[f64]; 3] = [S2X, S2Y, S2Z];

// Earth heliocentric and barycentric position-velocity (au, au/day).
// Returns (pvh, pvb, jstat) with jstat=0 OK, +1 if |t|>100y (warning).
pub fn eraEpv00_safe(date1: f64, date2: f64) -> ErfaResult<([[f64; 3]; 2], [[f64; 3]; 2], i32)> {
    // Time since J2000.0 in Julian years
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJY;

    // Warn if |t| > 100 years
    let jstat = if t.abs() <= 100.0 { 0 } else { 1 };

    // Work vectors
    let mut ph = [0.0_f64; 3];
    let mut vh = [0.0_f64; 3];
    let mut pb = [0.0_f64; 3];
    let mut vb = [0.0_f64; 3];

    // Loop over X,Y,Z
    for i in 0..3 {
        let (mut xyz, mut xyzd) = (0.0_f64, 0.0_f64);

        // Sun-to-Earth
        accumulate_component(&mut xyz, &mut xyzd, CE0[i], t, 0);
        accumulate_component(&mut xyz, &mut xyzd, CE1[i], t, 1);
        accumulate_component(&mut xyz, &mut xyzd, CE2[i], t, 2);

        ph[i] = xyz;
        vh[i] = xyzd / ERFA_DJY;

        // SSB-to-Sun (accumulate without reset → SSB-to-Earth)
        accumulate_component(&mut xyz, &mut xyzd, CS0[i], t, 0);
        accumulate_component(&mut xyz, &mut xyzd, CS1[i], t, 1);
        accumulate_component(&mut xyz, &mut xyzd, CS2[i], t, 2);

        pb[i] = xyz;
        vb[i] = xyzd / ERFA_DJY;
    }

    // Rotate ecliptic vectors to BCRS
    let mut pvh = [[0.0_f64; 3]; 2];
    let mut pvb = [[0.0_f64; 3]; 2];
    rotate_xyz(&ph, &mut pvh[0]);
    rotate_xyz(&vh, &mut pvh[1]);
    rotate_xyz(&pb, &mut pvb[0]);
    rotate_xyz(&vb, &mut pvb[1]);

    Ok((pvh, pvb, jstat))
}

// Accumulate a single component for given power of t (0,1,2), summing in reverse order.
#[inline]
fn accumulate_component(xyz: &mut f64, xyzd: &mut f64, coeffs: &[f64], t: f64, power: u8) {
    let nterms = coeffs.len() / 3;
    for k in (0..nterms).rev() {
        let a = coeffs[3 * k];
        let b = coeffs[3 * k + 1];
        let c = coeffs[3 * k + 2];
        let ct = c * t;
        let p = b + ct;
        let (sp, cp) = p.sin_cos();
        match power {
            0 => {
                *xyz += a * cp;
                *xyzd -= a * c * sp;
            }
            1 => {
                *xyz += a * t * cp;
                *xyzd += a * (cp - ct * sp);
            }
            2 => {
                *xyz += a * t * t * cp;
                *xyzd += a * t * (2.0 * cp - ct * sp);
            }
            _ => unreachable!(),
        }
    }
}

// Apply the small rotation to align to the BCRS frame.
#[inline]
fn rotate_xyz(src: &[f64; 3], dst: &mut [f64; 3]) {
    let (x, y, z) = (src[0], src[1], src[2]);
    dst[0] = x + AM12 * y + AM13 * z;
    dst[1] = AM21 * x + AM22 * y + AM23 * z;
    dst[2] = AM32 * y + AM33 * z;
}
