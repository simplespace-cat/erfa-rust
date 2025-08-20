// G15
//   fad03.c   → eraFad03_safe
//   fae03.c   → eraFae03_safe
//   faf03.c   → eraFaf03_safe
//   faju03.c  → eraFaju03_safe
//   fal03.c   → eraFal03_safe
//   falp03.c  → eraFalp03_safe
//   fama03.c  → eraFama03_safe
//   fame03.c  → eraFame03_safe
//   fane03.c  → eraFane03_safe
//   faom03.c  → eraFaom03_safe
//   fapa03.c  → eraFapa03_safe
//   fasa03.c  → eraFasa03_safe
//   faur03.c  → eraFaur03_safe
//   fave03.c  → eraFave03_safe
//   fk5hip.c  → eraFk5hip_safe

use crate::G28_safe::eraRv2m_safe;
use crate::H1_safe::{ERFA_D2PI, ERFA_DAS2R, ERFA_TURNAS};

pub type ErfaResult<T> = Result<T, ()>;

// Positive modulus for angles; ensures result in [0,y).
#[inline]
fn fmod_pos(x: f64, y: f64) -> f64 {
    let r = x % y;
    if r < 0.0 {
        r + y
    } else {
        r
    }
}

// Mean elongation of the Moon from the Sun (radians).
pub fn eraFad03_safe(t: f64) -> ErfaResult<f64> {
    let a = 1_072_260.703_692
        + t * (1_602_961_601.2090 + t * (-6.3706 + t * (0.006_593 + t * (-0.000_031_69))));
    let r = (a % ERFA_TURNAS) * ERFA_DAS2R;
    Ok(r)
}

// Mean longitude of Earth (radians in [0,2π)).
pub fn eraFae03_safe(t: f64) -> ErfaResult<f64> {
    Ok(fmod_pos(1.753_470_314 + 628.307_584_9991 * t, ERFA_D2PI))
}

// Mean longitude of the Moon minus the ascending node (radians).
pub fn eraFaf03_safe(t: f64) -> ErfaResult<f64> {
    let a = 335_779.526_232
        + t * (1_739_527_262.8478 + t * (-12.7512 + t * (-0.001_037 + t * (0.000_004_17))));
    Ok(fmod_pos(a, ERFA_TURNAS) * ERFA_DAS2R)
}

// Mean longitude of Jupiter (radians in [0,2π)).
pub fn eraFaju03_safe(t: f64) -> ErfaResult<f64> {
    Ok(fmod_pos(0.599_546_497 + 52.969_096_2641 * t, ERFA_D2PI))
}

// Mean anomaly of the Moon (radians).
pub fn eraFal03_safe(t: f64) -> ErfaResult<f64> {
    let a = 485_868.249_036
        + t * (1_717_915_923.2178 + t * (31.8792 + t * (0.051_635 + t * (-0.000_244_70))));
    Ok(fmod_pos(a, ERFA_TURNAS) * ERFA_DAS2R)
}

// Mean anomaly of the Sun (radians).
pub fn eraFalp03_safe(t: f64) -> ErfaResult<f64> {
    let a = 1_287_104.793_048
        + t * (129_596_581.0481 + t * (-0.5532 + t * (0.000_136 + t * (-0.000_011_49))));
    Ok(fmod_pos(a, ERFA_TURNAS) * ERFA_DAS2R)
}

// Mean longitude of Mars (radians in [0,2π)).
pub fn eraFama03_safe(t: f64) -> ErfaResult<f64> {
    Ok(fmod_pos(6.203_480_913 + 334.061_242_6700 * t, ERFA_D2PI))
}

// Mean longitude of Mercury (radians in [0,2π)).
pub fn eraFame03_safe(t: f64) -> ErfaResult<f64> {
    Ok(fmod_pos(4.402_608_842 + 2_608.790_314_1574 * t, ERFA_D2PI))
}

// Mean longitude of Neptune (radians in [0,2π)).
pub fn eraFane03_safe(t: f64) -> ErfaResult<f64> {
    Ok(fmod_pos(5.311_886_287 + 3.813_303_5638 * t, ERFA_D2PI))
}

// Mean longitude of the Moons ascending node (radians).
pub fn eraFaom03_safe(t: f64) -> ErfaResult<f64> {
    let a = 450_160.398_036
        + t * (-6_962_890.5431 + t * (7.4722 + t * (0.007_702 + t * (-0.000_059_39))));
    Ok((a % ERFA_TURNAS) * ERFA_DAS2R)
}

// General accumulated precession in longitude (radians).
pub fn eraFapa03_safe(t: f64) -> ErfaResult<f64> {
    Ok((0.024_381_750 + 0.000_005_386_91 * t) * t)
}

// Mean longitude of Saturn (radians in [0,2π)).
pub fn eraFasa03_safe(t: f64) -> ErfaResult<f64> {
    Ok(fmod_pos(0.874_016_757 + 21.329_910_4960 * t, ERFA_D2PI))
}

// Mean longitude of Uranus (radians in [0,2π)).
pub fn eraFaur03_safe(t: f64) -> ErfaResult<f64> {
    Ok(fmod_pos(5.481_293_872 + 7.478_159_8567 * t, ERFA_D2PI))
}

// Mean longitude of Venus (radians in [0,2π)).
pub fn eraFave03_safe(t: f64) -> ErfaResult<f64> {
    Ok(fmod_pos(3.176_146_697 + 1_021.328_554_6211 * t, ERFA_D2PI))
}

// FK5 → Hipparcos orientation matrix and spin vector.
pub fn eraFk5hip_safe() -> ErfaResult<([[f64; 3]; 3], [f64; 3])> {
    let epx = -19.9e-3 * ERFA_DAS2R;
    let epy = -9.1e-3 * ERFA_DAS2R;
    let epz = 22.9e-3 * ERFA_DAS2R;

    let omx = -0.30e-3 * ERFA_DAS2R;
    let omy = 0.60e-3 * ERFA_DAS2R;
    let omz = 0.70e-3 * ERFA_DAS2R;

    let v = [epx, epy, epz];
    let r5h = eraRv2m_safe(&v)?;
    let s5h = [omx, omy, omz];
    Ok((r5h, s5h))
}
