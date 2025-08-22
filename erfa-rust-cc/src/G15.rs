// G15
//   fad03.c   → eraFad03
//   fae03.c   → eraFae03
//   faf03.c   → eraFaf03
//   faju03.c  → eraFaju03
//   fal03.c   → eraFal03
//   falp03.c  → eraFalp03
//   fama03.c  → eraFama03
//   fame03.c  → eraFame03
//   fane03.c  → eraFane03
//   faom03.c  → eraFaom03
//   fapa03.c  → eraFapa03
//   fasa03.c  → eraFasa03
//   faur03.c  → eraFaur03
//   fave03.c  → eraFave03
//   fk5hip.c  → eraFk5hip

use crate::H1::*;

// internal helper: positive modulus with a f64 divisor
#[inline]
fn fmod_pos(x: f64, y: f64) -> f64 {
    let r = x % y;
    if r < 0.0 {
        r + y
    } else {
        r
    }
}

// eraFad03   mean elongation of the Moon from the Sun

pub unsafe fn eraFad03(t: f64) -> f64 {
    let a = 1_072_260.703_692
        + t * (1_602_961_601.2090 + t * (-6.3706 + t * (0.006_593 + t * (-0.000_031_69))));
    (a % ERFA_TURNAS) * ERFA_DAS2R
}

// eraFae03   mean longitude of Earth
pub unsafe fn eraFae03(t: f64) -> f64 {
    fmod_pos(1.753_470_314 + 628.307_584_9991 * t, ERFA_D2PI)
}

// eraFaf03   mean longitude of Moon minus node
pub unsafe fn eraFaf03(t: f64) -> f64 {
    let a = 335_779.526_232
        + t * (1_739_527_262.8478 + t * (-12.7512 + t * (-0.001_037 + t * (0.000_004_17))));
    fmod_pos(a, ERFA_TURNAS) * ERFA_DAS2R
}

// eraFaju03   mean longitude of Jupiter
pub unsafe fn eraFaju03(t: f64) -> f64 {
    fmod_pos(0.599_546_497 + 52.969_096_2641 * t, ERFA_D2PI)
}

// eraFal03   mean anomaly of the Moon
pub unsafe fn eraFal03(t: f64) -> f64 {
    let a = 485_868.249_036
        + t * (1_717_915_923.2178 + t * (31.8792 + t * (0.051_635 + t * (-0.000_244_70))));
    fmod_pos(a, ERFA_TURNAS) * ERFA_DAS2R
}

// eraFalp03   mean anomaly of the Sun
pub unsafe fn eraFalp03(t: f64) -> f64 {
    let a = 1_287_104.793_048
        + t * (129_596_581.0481 + t * (-0.5532 + t * (0.000_136 + t * (-0.000_011_49))));
    fmod_pos(a, ERFA_TURNAS) * ERFA_DAS2R
}

// eraFama03   mean longitude of Mars
pub unsafe fn eraFama03(t: f64) -> f64 {
    fmod_pos(6.203_480_913 + 334.061_242_6700 * t, ERFA_D2PI)
}

// eraFame03   mean longitude of Mercury
pub unsafe fn eraFame03(t: f64) -> f64 {
    fmod_pos(4.402_608_842 + 2_608.790_314_1574 * t, ERFA_D2PI)
}

// eraFane03   mean longitude of Neptune
pub unsafe fn eraFane03(t: f64) -> f64 {
    fmod_pos(5.311_886_287 + 3.813_303_5638 * t, ERFA_D2PI)
}

// eraFaom03   mean longitude of Moons ascending node
pub fn eraFaom03(t: f64) -> f64 {
    let a = 450_160.398_036
        + t * (-6_962_890.5431 + t * (7.4722 + t * (0.007_702 + t * (-0.000_059_39))));

    (a % ERFA_TURNAS) * ERFA_DAS2R
}

// eraFapa03   general accumulated precession in longitude
pub unsafe fn eraFapa03(t: f64) -> f64 {
    (0.024_381_750 + 0.000_005_386_91 * t) * t
}

// eraFasa03   mean longitude of Saturn
pub unsafe fn eraFasa03(t: f64) -> f64 {
    fmod_pos(0.874_016_757 + 21.329_910_4960 * t, ERFA_D2PI)
}

// eraFaur03   mean longitude of Uranus
pub unsafe fn eraFaur03(t: f64) -> f64 {
    fmod_pos(5.481_293_872 + 7.478_159_8567 * t, ERFA_D2PI)
}

// eraFave03   mean longitude of Venus
pub unsafe fn eraFave03(t: f64) -> f64 {
    fmod_pos(3.176_146_697 + 1_021.328_554_6211 * t, ERFA_D2PI)
}

// eraFk5hip   FK5 → Hipparcos orientation matrix and spin vector
pub unsafe fn eraFk5hip(r5h: *mut f64, s5h: *mut f64) {
    let (epx, epy, epz) = (
        -19.9e-3 * ERFA_DAS2R,
        -9.1e-3 * ERFA_DAS2R,
        22.9e-3 * ERFA_DAS2R,
    );
    let (omx, omy, omz) = (
        -0.30e-3 * ERFA_DAS2R,
        0.60e-3 * ERFA_DAS2R,
        0.70e-3 * ERFA_DAS2R,
    );

    let mut v = [epx, epy, epz];
    eraRv2m(v.as_mut_ptr(), r5h);

    unsafe {
        *s5h.add(0) = omx;
        *s5h.add(1) = omy;
        *s5h.add(2) = omz;
    }
}
