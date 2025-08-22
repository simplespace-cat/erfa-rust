// H1  ERFA types, constants and public API
//   erfa.h         → core types & function exports
//   erfadatextra.h → leap second initialization
//   erfaextra.h    → version info & leap second access
//   erfam.h        → mathematical constants & macros

// H1/erfa.h

// star-independent astrometry parameters
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct eraASTROM {
    pub pmt: f64,
    pub eb: [f64; 3],
    pub eh: [f64; 3],
    pub em: f64,
    pub v: [f64; 3],
    pub bm1: f64,
    pub bpn: [[f64; 3]; 3],
    pub along: f64,
    pub phi: f64,
    pub xpl: f64,
    pub ypl: f64,
    pub sphi: f64,
    pub cphi: f64,
    pub diurab: f64,
    pub eral: f64,
    pub refa: f64,
    pub refb: f64,
}

// body parameters for light deflection
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct eraLDBODY {
    pub bm: f64,
    pub dl: f64,
    pub pv: [[f64; 3]; 2],
}

// macro_rules! stub { ($ret:ty) => ( { todo!() } ); }

// Astronomy/Calendars
pub use crate::G12::eraEpb;
pub use crate::G12::eraEpb2jd;
pub use crate::G12::eraEpj;
pub use crate::G12::eraEpj2jd;
pub use crate::G19::eraJd2cal;
pub use crate::G19::eraJdcalf;
pub use crate::G8::eraCal2jd;

// Astronomy/Astrometry
pub use crate::G1::eraAb;
pub use crate::G1::eraApcg;
pub use crate::G1::eraApcg13;
pub use crate::G1::eraApci;
pub use crate::G1::eraApci13;

pub use crate::G2::eraApco;
pub use crate::G2::eraApco13;
pub use crate::G2::eraApcs;
pub use crate::G2::eraApcs13;
pub use crate::G2::eraAper;
pub use crate::G20::eraLd;
pub use crate::G20::eraLdn;
pub use crate::G20::eraLdsun;
pub use crate::G25::eraPmpx;
pub use crate::G25::eraPmsafe;
pub use crate::G27::eraPvtob;
pub use crate::G28::eraRefco;
pub use crate::G3::eraAper13;
pub use crate::G3::eraApio;
pub use crate::G3::eraApio13;
pub use crate::G3::eraAtcc13;
pub use crate::G3::eraAtccq;
pub use crate::G4::eraAtci13;
pub use crate::G4::eraAtciq;
pub use crate::G4::eraAtciqn;
pub use crate::G4::eraAtciqz;
pub use crate::G4::eraAtco13;
pub use crate::G4::eraAtic13;
pub use crate::G4::eraAticq;
pub use crate::G4::eraAticqn;
pub use crate::G4::eraAtio13;
pub use crate::G4::eraAtioq;
pub use crate::G5::eraAtoc13;
pub use crate::G5::eraAtoi13;
pub use crate::G5::eraAtoiq;

// Astronomy/Ephemerides
pub use crate::G13::eraEpv00;
pub use crate::G21::eraMoon98;
pub use crate::G24::eraPlan94;

// Astronomy/FundamentalArgs
pub use crate::G15::eraFad03;
pub use crate::G15::eraFae03;
pub use crate::G15::eraFaf03;
pub use crate::G15::eraFaju03;
pub use crate::G15::eraFal03;
pub use crate::G15::eraFalp03;
pub use crate::G15::eraFama03;
pub use crate::G15::eraFame03;
pub use crate::G15::eraFane03;
pub use crate::G15::eraFaom03;
pub use crate::G15::eraFapa03;
pub use crate::G15::eraFasa03;
pub use crate::G15::eraFaur03;
pub use crate::G15::eraFave03;

// Astronomy/PrecNutPolar
pub use crate::G11::eraEo06a;
pub use crate::G11::eraEors;
pub use crate::G16::eraFw2m;
pub use crate::G16::eraFw2xy;
pub use crate::G20::eraLtp;
pub use crate::G20::eraLtpb;
pub use crate::G20::eraLtpecl;
pub use crate::G20::eraLtpequ;
pub use crate::G21::eraNum00a;
pub use crate::G21::eraNum00b;
pub use crate::G21::eraNum06a;
pub use crate::G21::eraNumat;
pub use crate::G22::eraNut00a;
pub use crate::G23::eraNut00b;
pub use crate::G23::eraNut06a;
pub use crate::G23::eraNut80;
pub use crate::G23::eraNutm80;
pub use crate::G23::eraObl06;
pub use crate::G23::eraObl80;
pub use crate::G24::eraP06e;
pub use crate::G24::eraPb06;
pub use crate::G24::eraPfw06;
pub use crate::G25::eraPmat00;
pub use crate::G25::eraPmat06;
pub use crate::G25::eraPmat76;
pub use crate::G25::eraPn00;
pub use crate::G25::eraPn00a;
pub use crate::G25::eraPn00b;
pub use crate::G26::eraPn06;
pub use crate::G26::eraPn06a;
pub use crate::G26::eraPnm00a;
pub use crate::G26::eraPnm00b;
pub use crate::G26::eraPnm06a;
pub use crate::G26::eraPnm80;
pub use crate::G26::eraPom00;
pub use crate::G26::eraPr00;
pub use crate::G27::eraPrec76;
pub use crate::G29::eraS00;
pub use crate::G29::eraS00a;
pub use crate::G29::eraS00b;
pub use crate::G30::eraS06;
pub use crate::G30::eraS06a;
pub use crate::G30::eraSp00;
pub use crate::G34::eraXy06;
pub use crate::G35::eraXys00a;
pub use crate::G35::eraXys00b;
pub use crate::G35::eraXys06a;
pub use crate::G6::eraBi00;
pub use crate::G6::eraBp00;
pub use crate::G6::eraBp06;
pub use crate::G6::eraBpn2xy;
pub use crate::G7::eraC2i00a;
pub use crate::G7::eraC2i00b;
pub use crate::G7::eraC2i06a;
pub use crate::G7::eraC2ibpn;
pub use crate::G7::eraC2ixy;
pub use crate::G7::eraC2ixys;
pub use crate::G7::eraC2t00a;
pub use crate::G7::eraC2t00b;
pub use crate::G7::eraC2t06a;
pub use crate::G8::eraC2tcio;
pub use crate::G8::eraC2teqx;
pub use crate::G8::eraC2tpe;
pub use crate::G8::eraC2txy;

// Astronomy/RotationAndTime
pub use crate::G11::eraEe00;
pub use crate::G11::eraEe00a;
pub use crate::G11::eraEe00b;
pub use crate::G11::eraEe06a;
pub use crate::G11::eraEect00;
pub use crate::G14::eraEqeq94;
pub use crate::G14::eraEra00;
pub use crate::G17::eraGmst00;
pub use crate::G17::eraGmst06;
pub use crate::G17::eraGmst82;
pub use crate::G17::eraGst00a;
pub use crate::G17::eraGst00b;
pub use crate::G17::eraGst06;
pub use crate::G17::eraGst06a;
pub use crate::G17::eraGst94;

// Astronomy/SpaceMotion
pub use crate::G27::eraPvstar;
pub use crate::G30::eraStarpv;

// Astronomy/StarCatalogs
pub use crate::G15::eraFk5hip;
pub use crate::G16::eraFk425;
pub use crate::G16::eraFk45z;
pub use crate::G16::eraFk524;
pub use crate::G16::eraFk52h;
pub use crate::G16::eraFk54z;
pub use crate::G16::eraFk5hz;
pub use crate::G18::eraH2fk5;
pub use crate::G18::eraHfk5z;
pub use crate::G30::eraStarpm;

// Astronomy/EclipticCoordinates
pub use crate::G11::eraEceq06;
pub use crate::G11::eraEcm06;
pub use crate::G14::eraEqec06;
pub use crate::G20::eraLteceq;
pub use crate::G20::eraLtecm;
pub use crate::G20::eraLteqec;

// Astronomy/GalacticCoordinates
pub use crate::G17::eraG2icrs;
pub use crate::G19::eraIcrs2g;

// Astronomy/GeodeticGeocentric
pub use crate::G11::eraEform;
pub use crate::G17::eraGc2gd;
pub use crate::G17::eraGc2gde;
pub use crate::G17::eraGd2gc;
pub use crate::G17::eraGd2gce;

// Astronomy/Timescales
pub use crate::G10::eraDtdb;
pub use crate::G10::eraDtf2d;
pub use crate::G32::eraTaitt;
pub use crate::G32::eraTaiut1;
pub use crate::G32::eraTaiutc;
pub use crate::G32::eraTcbtdb;
pub use crate::G32::eraTcgtt;
pub use crate::G32::eraTdbtcb;
pub use crate::G32::eraTdbtt;
pub use crate::G33::eraTttai;
pub use crate::G33::eraTttcg;
pub use crate::G33::eraTttdb;
pub use crate::G33::eraTtut1;
pub use crate::G33::eraUt1tai;
pub use crate::G33::eraUt1tt;
pub use crate::G33::eraUt1utc;
pub use crate::G33::eraUtctai;
pub use crate::G33::eraUtcut1;
pub use crate::G9::eraD2dtf;
pub use crate::G9::eraDat;

pub use crate::G1::eraAe2hd;

// Astronomy/HorizonEquatorial
pub use crate::G18::eraHd2ae;
pub use crate::G18::eraHd2pa;

// Astronomy/Gnomonic
pub use crate::G32::eraTpors;
pub use crate::G32::eraTporv;
pub use crate::G32::eraTpsts;
pub use crate::G32::eraTpstv;
pub use crate::G32::eraTpxes;
pub use crate::G32::eraTpxev;

// VectorMatrix/AngleOps
pub use crate::G1::eraA2af;
pub use crate::G1::eraA2tf;
pub use crate::G1::eraAf2a;
pub use crate::G1::eraAnp;
pub use crate::G1::eraAnpm;

pub use crate::G32::eraTf2a;
pub use crate::G32::eraTf2d;
pub use crate::G9::eraD2tf;

// VectorMatrix/BuildRotations
pub use crate::G28::eraRx;
pub use crate::G28::eraRy;
pub use crate::G28::eraRz;

// VectorMatrix/CopyExtendExtract
pub use crate::G24::eraP2pv;
pub use crate::G27::eraPv2p;
pub use crate::G8::eraCp;
pub use crate::G8::eraCpv;
pub use crate::G8::eraCr;

// VectorMatrix/Initialization
pub use crate::G19::eraIr;
pub use crate::G35::eraZp;
pub use crate::G35::eraZpv;
pub use crate::G35::eraZr;

// VectorMatrix/MatrixOps
pub use crate::G28::eraRxr;
pub use crate::G33::eraTr;

// VectorMatrix/MatrixVectorProducts
pub use crate::G28::eraRxp;
pub use crate::G28::eraRxpv;
pub use crate::G33::eraTrxp;
pub use crate::G33::eraTrxpv;

// VectorMatrix/RotationVectors
pub use crate::G28::eraRm2v;
pub use crate::G28::eraRv2m;

// VectorMatrix/SeparationAndAngle
pub use crate::G24::eraPap;
pub use crate::G24::eraPas;
pub use crate::G30::eraSepp;
pub use crate::G30::eraSeps;

// VectorMatrix/SphericalCartesian
pub use crate::G24::eraP2s;
pub use crate::G27::eraPv2s;
pub use crate::G29::eraS2c;
pub use crate::G29::eraS2p;
pub use crate::G29::eraS2pv;
pub use crate::G7::eraC2s;

// VectorMatrix/VectorOps
pub use crate::G24::eraPdp;
pub use crate::G24::eraPm;
pub use crate::G25::eraPmp;
pub use crate::G25::eraPn;
pub use crate::G26::eraPpp;
pub use crate::G26::eraPpsp;
pub use crate::G27::eraPvdpv;
pub use crate::G27::eraPvm;
pub use crate::G27::eraPvmpv;
pub use crate::G27::eraPvppv;
pub use crate::G27::eraPvu;
pub use crate::G27::eraPvup;
pub use crate::G27::eraPvxpv;
pub use crate::G27::eraPxp;
pub use crate::G29::eraS2xpv;
pub use crate::G30::eraSxp;
pub use crate::G30::eraSxpv;

// H1/erfaextra.h

// Leap-second table type
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct eraLEAPSECOND {
    pub iyear: i32,
    pub month: i32,
    pub delat: f64,
}

// Version/query helpers
pub use crate::G14::eraSofaVersion;
pub use crate::G14::eraVersion;
pub use crate::G14::eraVersionMajor;
pub use crate::G14::eraVersionMicro;
pub use crate::G14::eraVersionMinor;

// Experimental leap-second accessors
pub use crate::G14::eraGetLeapSeconds;
pub use crate::G14::eraSetLeapSeconds;

// H1/erfadatextra.h

pub use crate::G14::eraDatini;

// H1/erfam.h   macros → const / inline fn

// Numeric constants
pub const ERFA_DPI: f64 = 3.141592653589793238462643;
pub const ERFA_D2PI: f64 = 6.283185307179586476925287;
pub const ERFA_DR2D: f64 = 57.29577951308232087679815;
pub const ERFA_DD2R: f64 = 1.745329251994329576923691e-2;
pub const ERFA_DR2AS: f64 = 206_264.806_247_096_355_156_4734;
pub const ERFA_DAS2R: f64 = 4.848_136_811_095_359_935_899_141e-6;
pub const ERFA_DS2R: f64 = 7.272_205_216_643_039_903_848_712e-5;
pub const ERFA_TURNAS: f64 = 1_296_000.0;
pub const ERFA_DMAS2R: f64 = ERFA_DAS2R / 1.0e3;
pub const ERFA_DTY: f64 = 365.242_198_781;
pub const ERFA_DAYSEC: f64 = 86_400.0;
pub const ERFA_DJY: f64 = 365.25;
pub const ERFA_DJC: f64 = 36_525.0;
pub const ERFA_DJM: f64 = 365_250.0;
pub const ERFA_DJ00: f64 = 2_451_545.0;
pub const ERFA_DJM0: f64 = 2_400_000.5;
pub const ERFA_DJM00: f64 = 51_544.5;
pub const ERFA_DJM77: f64 = 43_144.0;
pub const ERFA_TTMTAI: f64 = 32.184;
pub const ERFA_DAU: f64 = 149_597_870.7e3;
pub const ERFA_CMPS: f64 = 299_792_458.0;
pub const ERFA_AULT: f64 = ERFA_DAU / ERFA_CMPS;
pub const ERFA_DC: f64 = ERFA_DAYSEC / ERFA_AULT;
pub const ERFA_ELG: f64 = 6.969_290_134e-10;
pub const ERFA_ELB: f64 = 1.550_519_768e-8;
pub const ERFA_TDB0: f64 = -6.55e-5;
pub const ERFA_SRS: f64 = 1.974_125_743_36e-8;

// Reference ellipsoids
pub const ERFA_WGS84: i32 = 1;
pub const ERFA_GRS80: i32 = 2;
pub const ERFA_WGS72: i32 = 3;

#[inline]
pub fn ERFA_DINT(a: f64) -> f64 {
    if a < 0.0 {
        a.ceil()
    } else {
        a.floor()
    }
}
#[inline]
pub fn ERFA_DNINT(a: f64) -> f64 {
    if a.abs() < 0.5 {
        0.0
    } else if a < 0.0 {
        (a - 0.5).ceil()
    } else {
        (a + 0.5).floor()
    }
}
#[inline]
pub fn ERFA_DSIGN(a: f64, b: f64) -> f64 {
    if b < 0.0 {
        -a.abs()
    } else {
        a.abs()
    }
}
#[inline]
pub fn ERFA_GMAX<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}
#[inline]
pub fn ERFA_GMIN<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}
