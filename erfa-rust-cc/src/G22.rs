// G22
//   nut00a.c → eraNut00a

use crate::H1::*;
#[path = "data/G22/XLS.rs"]
mod xls_mod;
use xls_mod::XLS;
#[path = "data/G22/XPL.rs"]
mod xpl_mod;
use xpl_mod::XPL;

#[inline]
fn fmod(a: f64, p: f64) -> f64 {
    (a % p + p) % p
}

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

pub unsafe fn eraNut00a(date1: f64, date2: f64, dpsi: *mut f64, deps: *mut f64) {
    const U2R: f64 = ERFA_DAS2R / 1e7;

    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    let el = eraFal03(t);
    let elp = fmod(
        1287104.79305 + t * (129596581.0481 + t * (-0.5532 + t * (0.000136 + t * (-0.00001149)))),
        ERFA_TURNAS,
    ) * ERFA_DAS2R;
    let f = eraFaf03(t);
    let d = fmod(
        1072260.70369 + t * (1602961601.2090 + t * (-6.3706 + t * (0.006593 + t * (-0.00003169)))),
        ERFA_TURNAS,
    ) * ERFA_DAS2R;
    let om = eraFaom03(t);

    let mut dp = 0.0;
    let mut de = 0.0;

    for term in XLS.iter().rev() {
        let arg = fmod(
            term.nl as f64 * el
                + term.nlp as f64 * elp
                + term.nf as f64 * f
                + term.nd as f64 * d
                + term.nom as f64 * om,
            ERFA_D2PI,
        );
        let sarg = arg.sin();
        let carg = arg.cos();

        dp += (term.sp + term.spt * t) * sarg + term.cp * carg;
        de += (term.ce + term.cet * t) * carg + term.se * sarg;
    }
    let dpsils = dp * U2R;
    let depsls = de * U2R;

    let al = fmod(2.35555598 + 8328.6914269554 * t, ERFA_D2PI);
    let af = fmod(1.627905234 + 8433.466158131 * t, ERFA_D2PI);
    let ad = fmod(5.198466741 + 7771.3771468121 * t, ERFA_D2PI);
    let aom = fmod(2.18243920 - 33.757045 * t, ERFA_D2PI);
    let apa = eraFapa03(t);
    let alme = eraFame03(t);
    let alve = eraFave03(t);
    let alea = eraFae03(t);
    let alma = eraFama03(t);
    let alju = eraFaju03(t);
    let alsa = eraFasa03(t);
    let alur = eraFaur03(t);
    let alne = fmod(5.321159000 + 3.8127774000 * t, ERFA_D2PI);

    dp = 0.0;
    de = 0.0;

    for term in XPL.iter().rev() {
        let arg = fmod(
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
        let sarg = arg.sin();
        let carg = arg.cos();

        dp += term.sp as f64 * sarg + term.cp as f64 * carg;
        de += term.se as f64 * sarg + term.ce as f64 * carg;
    }
    let dpsipl = dp * U2R;
    let depspl = de * U2R;

    *dpsi = dpsils + dpsipl;
    *deps = depsls + depspl;
}
