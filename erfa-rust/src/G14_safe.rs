// G14
//   eqec06.c       → eraEqec06_safe
//   eqeq94.c       → eraEqeq94_safe
//   era00.c        → eraEra00_safe
//   erfadatextra.c → eraGetLeapSeconds_safe, eraSetLeapSeconds_safe, eraDatini_safe
//   erfaversion.c  → eraVersion_safe, eraVersionMajor_safe, eraVersionMinor_safe, eraVersionMicro_safe, eraSofaVersion_safe

use std::sync::RwLock;

use crate::G11_safe::eraEcm06_safe;
use crate::G1_safe::{eraAnp_safe, eraAnpm_safe};
use crate::G23_safe::{eraNut80_safe, eraObl80_safe};
use crate::G28_safe::eraRxp_safe;
use crate::G29_safe::eraS2c_safe;
use crate::G7_safe::eraC2s_safe;
use crate::H1_safe::{eraLEAPSECOND, ERFA_D2PI, ERFA_DAS2R, ERFA_DJ00, ERFA_DJC};

pub type ErfaResult<T> = Result<T, ()>;

// Built-in leap second table for initialization.
const G14_BUILTIN_LEAPSECONDS: &[eraLEAPSECOND] = &[
    eraLEAPSECOND {
        iyear: 1960,
        month: 1,
        delat: 1.4178180,
    },
    eraLEAPSECOND {
        iyear: 1961,
        month: 1,
        delat: 1.4228180,
    },
    eraLEAPSECOND {
        iyear: 1961,
        month: 8,
        delat: 1.3728180,
    },
    eraLEAPSECOND {
        iyear: 1962,
        month: 1,
        delat: 1.8458580,
    },
    eraLEAPSECOND {
        iyear: 1963,
        month: 11,
        delat: 1.9458580,
    },
    eraLEAPSECOND {
        iyear: 1964,
        month: 1,
        delat: 3.2401300,
    },
    eraLEAPSECOND {
        iyear: 1964,
        month: 4,
        delat: 3.3401300,
    },
    eraLEAPSECOND {
        iyear: 1964,
        month: 9,
        delat: 3.4401300,
    },
    eraLEAPSECOND {
        iyear: 1965,
        month: 1,
        delat: 3.5401300,
    },
    eraLEAPSECOND {
        iyear: 1965,
        month: 3,
        delat: 3.6401300,
    },
    eraLEAPSECOND {
        iyear: 1965,
        month: 7,
        delat: 3.7401300,
    },
    eraLEAPSECOND {
        iyear: 1965,
        month: 9,
        delat: 3.8401300,
    },
    eraLEAPSECOND {
        iyear: 1966,
        month: 1,
        delat: 4.3131700,
    },
    eraLEAPSECOND {
        iyear: 1968,
        month: 2,
        delat: 4.2131700,
    },
    eraLEAPSECOND {
        iyear: 1972,
        month: 1,
        delat: 10.0,
    },
    eraLEAPSECOND {
        iyear: 1972,
        month: 7,
        delat: 11.0,
    },
    eraLEAPSECOND {
        iyear: 1973,
        month: 1,
        delat: 12.0,
    },
    eraLEAPSECOND {
        iyear: 1974,
        month: 1,
        delat: 13.0,
    },
    eraLEAPSECOND {
        iyear: 1975,
        month: 1,
        delat: 14.0,
    },
    eraLEAPSECOND {
        iyear: 1976,
        month: 1,
        delat: 15.0,
    },
    eraLEAPSECOND {
        iyear: 1977,
        month: 1,
        delat: 16.0,
    },
    eraLEAPSECOND {
        iyear: 1978,
        month: 1,
        delat: 17.0,
    },
    eraLEAPSECOND {
        iyear: 1979,
        month: 1,
        delat: 18.0,
    },
    eraLEAPSECOND {
        iyear: 1980,
        month: 1,
        delat: 19.0,
    },
    eraLEAPSECOND {
        iyear: 1981,
        month: 7,
        delat: 20.0,
    },
    eraLEAPSECOND {
        iyear: 1982,
        month: 7,
        delat: 21.0,
    },
    eraLEAPSECOND {
        iyear: 1983,
        month: 7,
        delat: 22.0,
    },
    eraLEAPSECOND {
        iyear: 1985,
        month: 7,
        delat: 23.0,
    },
    eraLEAPSECOND {
        iyear: 1988,
        month: 1,
        delat: 24.0,
    },
    eraLEAPSECOND {
        iyear: 1990,
        month: 1,
        delat: 25.0,
    },
    eraLEAPSECOND {
        iyear: 1991,
        month: 1,
        delat: 26.0,
    },
    eraLEAPSECOND {
        iyear: 1992,
        month: 7,
        delat: 27.0,
    },
    eraLEAPSECOND {
        iyear: 1993,
        month: 7,
        delat: 28.0,
    },
    eraLEAPSECOND {
        iyear: 1994,
        month: 7,
        delat: 29.0,
    },
    eraLEAPSECOND {
        iyear: 1996,
        month: 1,
        delat: 30.0,
    },
    eraLEAPSECOND {
        iyear: 1997,
        month: 7,
        delat: 31.0,
    },
    eraLEAPSECOND {
        iyear: 1999,
        month: 1,
        delat: 32.0,
    },
    eraLEAPSECOND {
        iyear: 2006,
        month: 1,
        delat: 33.0,
    },
    eraLEAPSECOND {
        iyear: 2009,
        month: 1,
        delat: 34.0,
    },
    eraLEAPSECOND {
        iyear: 2012,
        month: 7,
        delat: 35.0,
    },
    eraLEAPSECOND {
        iyear: 2015,
        month: 7,
        delat: 36.0,
    },
    eraLEAPSECOND {
        iyear: 2017,
        month: 1,
        delat: 37.0,
    },
];

// Minimal state for leap seconds.
#[derive(Clone, Default)]
struct LeapState {
    ndat: i32,
    table: Vec<eraLEAPSECOND>,
}

static LEAP_STATE: RwLock<LeapState> = RwLock::new(LeapState {
    ndat: -1,
    table: Vec::new(),
});

// G14/eqec06.c → eraEqec06_safe
// ICRS equatorial to ecliptic (mean of date, IAU 2006).
pub fn eraEqec06_safe(date1: f64, date2: f64, dr: f64, dd: f64) -> ErfaResult<(f64, f64)> {
    let v1 = eraS2c_safe(dr, dd)?;
    let mut rm = [[0.0_f64; 3]; 3];
    eraEcm06_safe(date1, date2, &mut rm)?;
    let v2 = eraRxp_safe(&rm, &v1)?;
    let (a, b) = eraC2s_safe(&v2)?;
    let dl = eraAnp_safe(a)?;
    let db = eraAnpm_safe(b)?;
    Ok((dl, db))
}

// G14/eqeq94.c → eraEqeq94_safe
// Equation of the equinoxes, IAU 1994 model.
pub fn eraEqeq94_safe(date1: f64, date2: f64) -> ErfaResult<f64> {
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;
    let a = -5.0 * t;
    let fmod1 = a - a.trunc();
    let om = eraAnpm_safe(
        (450_160.280 + (-482_890.539 + (7.455 + 0.008 * t) * t) * t) * ERFA_DAS2R
            + fmod1 * ERFA_D2PI,
    )?;
    let (dpsi, _deps) = eraNut80_safe(date1, date2)?;
    let eps0 = eraObl80_safe(date1, date2)?;
    let ee = dpsi * eps0.cos() + ERFA_DAS2R * (0.00264 * om.sin() + 0.000_063 * (2.0 * om).sin());
    Ok(ee)
}

// G14/era00.c → eraEra00_safe
// Earth rotation angle (IAU 2000), radians in range 0..2π.
pub fn eraEra00_safe(dj1: f64, dj2: f64) -> ErfaResult<f64> {
    let (d1, d2) = if dj1 < dj2 { (dj1, dj2) } else { (dj2, dj1) };
    let t = d1 + (d2 - ERFA_DJ00);
    let f1 = d1 - d1.trunc();
    let f2 = d2 - d2.trunc();
    let f = f1 + f2;
    let theta = eraAnp_safe(ERFA_D2PI * (f + 0.779_057_273_264_0 + 0.002_737_811_911_354_48 * t))?;
    Ok(theta)
}

// G14/erfadatextra.c → eraGetLeapSeconds_safe
// Return current leap-second table (copy); init to builtin if needed.
pub fn eraGetLeapSeconds_safe() -> ErfaResult<Vec<eraLEAPSECOND>> {
    {
        let guard = LEAP_STATE.read().map_err(|_| ())?;
        if guard.ndat > 0 {
            return Ok(guard.table.clone());
        }
    }
    let _ = eraDatini_safe(G14_BUILTIN_LEAPSECONDS)?;
    let guard = LEAP_STATE.read().map_err(|_| ())?;
    if guard.ndat > 0 {
        Ok(guard.table.clone())
    } else {
        Err(())
    }
}

// G14/erfadatextra.c → eraSetLeapSeconds_safe
// Replace current leap-second table; empty slice marks reset-to-builtin.
pub fn eraSetLeapSeconds_safe(table: &[eraLEAPSECOND]) -> ErfaResult<()> {
    let mut guard = LEAP_STATE.write().map_err(|_| ())?;
    if table.is_empty() {
        guard.table.clear();
        guard.ndat = 0;
    } else {
        guard.table = table.to_vec();
        guard.ndat = guard.table.len() as i32;
    }
    Ok(())
}

// G14/erfadatextra.c → eraDatini_safe
// Ensure table is initialized to builtin if needed; return current copy.
pub fn eraDatini_safe(builtin: &[eraLEAPSECOND]) -> ErfaResult<Vec<eraLEAPSECOND>> {
    let mut guard = LEAP_STATE.write().map_err(|_| ())?;
    if guard.ndat <= 0 {
        guard.table = builtin.to_vec();
        guard.ndat = guard.table.len() as i32;
    }
    Ok(guard.table.clone())
}

// G14/erfaversion.c → version helpers
// Return crate version string.
pub fn eraVersion_safe() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

// Return major version number.
pub fn eraVersionMajor_safe() -> i32 {
    let v = env!("CARGO_PKG_VERSION");
    v.split('.')
        .next()
        .unwrap_or("0")
        .parse::<i32>()
        .unwrap_or(0)
}

// Return minor version number.
pub fn eraVersionMinor_safe() -> i32 {
    let v = env!("CARGO_PKG_VERSION");
    v.split('.')
        .nth(1)
        .unwrap_or("0")
        .parse::<i32>()
        .unwrap_or(0)
}

// Return micro/patch version number.
pub fn eraVersionMicro_safe() -> i32 {
    let v = env!("CARGO_PKG_VERSION");
    v.split('.')
        .nth(2)
        .unwrap_or("0")
        .parse::<i32>()
        .unwrap_or(0)
}

// Return upstream SOFA version string (not tracked in this port).
pub fn eraSofaVersion_safe() -> &'static str {
    "unknown"
}
