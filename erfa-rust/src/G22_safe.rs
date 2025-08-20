// G22
//   nut00a.c → eraNut00a_safe

use crate::G15_safe::{
    eraFae03_safe, eraFaf03_safe, eraFaju03_safe, eraFal03_safe, eraFama03_safe, eraFame03_safe,
    eraFaom03_safe, eraFapa03_safe, eraFasa03_safe, eraFaur03_safe, eraFave03_safe,
};
use crate::H1_safe::{ERFA_D2PI, ERFA_DAS2R, ERFA_DJ00, ERFA_DJC, ERFA_TURNAS};

#[path = "data/G22_safe/XLS.rs"]
mod xls_mod;
use xls_mod::XLS;

#[path = "data/G22_safe/XPL.rs"]
mod xpl_mod;
use xpl_mod::XPL;

pub type ErfaResult<T> = Result<T, ()>;

// Small helper: positive modulo for floating angles.
#[inline]
fn fmod_pos(a: f64, p: f64) -> f64 {
    (a % p + p) % p
}

// coefficient tables
// Types must match the data tables defined in data/G22/*.rs (verbatim).
#[derive(Clone, Copy)]
struct LuniSolar {
    nl: i32,
    nlp: i32,
    nf: i32,
    nd: i32,
    nom: i32,
    sp: f64,
    spt: f64,
    cp: f64,
    ce: f64,
    cet: f64,
    se: f64,
}

#[derive(Clone, Copy)]
struct Planetary {
    nl: i32,
    nf: i32,
    nd: i32,
    nom: i32,
    nme: i32,
    nve: i32,
    nea: i32,
    nma: i32,
    nju: i32,
    nsa: i32,
    nur: i32,
    nne: i32,
    npa: i32,
    sp: i32,
    cp: i32,
    se: i32,
    ce: i32,
}

// main routine
// Nutation, IAU-2000A model (no free-core nutation)
pub fn eraNut00a_safe(date1: f64, date2: f64) -> ErfaResult<(f64, f64)> {
    // 0.1 µas to radians.
    const U2R: f64 = ERFA_DAS2R / 1e7;

    // Time in Julian centuries from J2000.0.
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    // Luni-solar fundamental arguments.
    let el = eraFal03_safe(t)?; // Moon mean anomaly
    // Sun mean anomaly (MHB2000 explicit series in arcsec, then radians).
    let elp = fmod_pos(
        1_287_104.793_05
            + t * (129_596_581.0481 + t * (-0.5532 + t * (0.000_136 + t * (-0.000_011_49)))),
        ERFA_TURNAS,
    ) * ERFA_DAS2R;
    let f = eraFaf03_safe(t)?; // Moon argument of latitude
    let d = fmod_pos(
        1_072_260.703_69
            + t * (1_602_961_601.2090 + t * (-6.3706 + t * (0.006_593 + t * (-0.000_031_69)))),
        ERFA_TURNAS,
    ) * ERFA_DAS2R;
    let om = eraFaom03_safe(t)?; // Moon ascending node longitude

    // Accumulators for luni-solar series.
    let mut dp = 0.0f64;
    let mut de = 0.0f64;

    // Sum luni-solar terms (reverse order to match C loop order).
    for term in XLS.iter().rev() {
        let arg = fmod_pos(
            term.nl as f64 * el
                + term.nlp as f64 * elp
                + term.nf as f64 * f
                + term.nd as f64 * d
                + term.nom as f64 * om,
            ERFA_D2PI,
        );
        let s = arg.sin();
        let c = arg.cos();

        dp += (term.sp + term.spt * t) * s + term.cp * c;
        de += (term.ce + term.cet * t) * c + term.se * s;
    }
    let dpsils = dp * U2R;
    let depsls = de * U2R;

    // Planetary contributions (MHB2000).
    let al = fmod_pos(2.355_555_98 + 8_328.691_426_9554 * t, ERFA_D2PI);
    let af = fmod_pos(1.627_905_234 + 8_433.466_158_1310 * t, ERFA_D2PI);
    let ad = fmod_pos(5.198_466_741 + 7_771.377_146_8121 * t, ERFA_D2PI);
    let aom = fmod_pos(2.182_439_20 - 33.757_045 * t, ERFA_D2PI);
    let apa = eraFapa03_safe(t)?; // General precession in longitude

    // Planetary longitudes.
    let alme = eraFame03_safe(t)?;
    let alve = eraFave03_safe(t)?;
    let alea = eraFae03_safe(t)?;
    let alma = eraFama03_safe(t)?;
    let alju = eraFaju03_safe(t)?;
    let alsa = eraFasa03_safe(t)?;
    let alur = eraFaur03_safe(t)?;
    let alne = fmod_pos(5.321_159_000 + 3.812_777_4000 * t, ERFA_D2PI);

    // Reset accumulators for planetary series.
    dp = 0.0;
    de = 0.0;

    for term in XPL.iter().rev() {
        let arg = fmod_pos(
            term.nl as f64 * al
                + term.nf as f64 * af
                + term.nd as f64 * ad
                + term.nom as f64 * aom
                + term.nme as f64 * alme
                + term.nve as f64 * alve
                + term.nea as f64 * alea
                + term.nma as f64 * alma
                + term.nju as f64 * alju
                + term.nsa as f64 * alsa
                + term.nur as f64 * alur
                + term.nne as f64 * alne
                + term.npa as f64 * apa,
            ERFA_D2PI,
        );
        let s = arg.sin();
        let c = arg.cos();

        dp += term.sp as f64 * s + term.cp as f64 * c;
        de += term.se as f64 * s + term.ce as f64 * c;
    }
    let dpsipl = dp * U2R;
    let depspl = de * U2R;

    // Return total nutation in longitude and obliquity.
    Ok((dpsils + dpsipl, depsls + depspl))
}
