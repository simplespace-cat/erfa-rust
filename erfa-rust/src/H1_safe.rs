// H1_safe  ERFA types, constants and public API
//   erfa.h         → core types & function exports
//   erfadatextra.h → leap second initialization
//   erfaextra.h    → version info & leap second access
//   erfam.h        → mathematical constants & macros


// H1/erfa.h

// Star-independent astrometry parameters
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

// Body parameters for light deflection  
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct eraLDBODY {
    pub bm: f64,
    pub dl: f64,
    pub pv: [[f64; 3]; 2],
}

// macro_rules! stub { ($ret:ty) => ( { todo!() } ); }

// Astronomy/Calendars
pub use crate::G12_safe::eraEpb2jd_safe;
pub use crate::G12_safe::eraEpb_safe;
pub use crate::G12_safe::eraEpj2jd_safe;
pub use crate::G12_safe::eraEpj_safe;
pub use crate::G19_safe::eraJd2cal_safe;
pub use crate::G19_safe::eraJdcalf_safe;
pub use crate::G8_safe::eraCal2jd_safe;

// Astronomy/Astrometry
pub use crate::G1_safe::eraAb_safe;
pub use crate::G1_safe::eraApcg13_safe;
pub use crate::G1_safe::eraApcg_safe;
pub use crate::G1_safe::eraApci13_safe;
pub use crate::G1_safe::eraApci_safe;

pub use crate::G2_safe::eraApco13_safe;
pub use crate::G2_safe::eraApco_safe;
pub use crate::G2_safe::eraApcs13_safe;
pub use crate::G2_safe::eraApcs_safe;
pub use crate::G2_safe::eraAper_safe;

pub use crate::G3_safe::eraAper13_safe;
pub use crate::G3_safe::eraApio13_safe;
pub use crate::G3_safe::eraApio_safe;
pub use crate::G3_safe::eraAtcc13_safe;
pub use crate::G3_safe::eraAtccq_safe;

pub use crate::G4_safe::eraAtci13_safe;
pub use crate::G4_safe::eraAtciq_safe;
pub use crate::G4_safe::eraAtciqn_safe;
pub use crate::G4_safe::eraAtciqz_safe;
pub use crate::G4_safe::eraAtco13_safe;
pub use crate::G4_safe::eraAtic13_safe;
pub use crate::G4_safe::eraAticq_safe;
pub use crate::G4_safe::eraAticqn_safe;
pub use crate::G4_safe::eraAtio13_safe;
pub use crate::G4_safe::eraAtioq_safe;

pub use crate::G5_safe::eraAtoc13_safe;
pub use crate::G5_safe::eraAtoi13_safe;
pub use crate::G5_safe::eraAtoiq_safe;

pub use crate::G20_safe::eraLd_safe;
pub use crate::G20_safe::eraLdn_safe;
pub use crate::G20_safe::eraLdsun_safe;
pub use crate::G25_safe::eraPmpx_safe;
pub use crate::G25_safe::eraPmsafe_safe;
pub use crate::G27_safe::eraPvtob_safe;
pub use crate::G28_safe::eraRefco_safe;

// Astronomy/Ephemerides
pub use crate::G13_safe::eraEpv00_safe;
pub use crate::G21_safe::eraMoon98_safe;
pub use crate::G24_safe::eraPlan94_safe;

// Astronomy/FundamentalArgs
pub use crate::G15_safe::eraFad03_safe;
pub use crate::G15_safe::eraFae03_safe;
pub use crate::G15_safe::eraFaf03_safe;
pub use crate::G15_safe::eraFaju03_safe;
pub use crate::G15_safe::eraFal03_safe;
pub use crate::G15_safe::eraFalp03_safe;
pub use crate::G15_safe::eraFama03_safe;
pub use crate::G15_safe::eraFame03_safe;
pub use crate::G15_safe::eraFane03_safe;
pub use crate::G15_safe::eraFaom03_safe;
pub use crate::G15_safe::eraFapa03_safe;
pub use crate::G15_safe::eraFasa03_safe;
pub use crate::G15_safe::eraFaur03_safe;
pub use crate::G15_safe::eraFave03_safe;

// Astronomy/PrecNutPolar
pub use crate::G6_safe::eraBi00_safe;
pub use crate::G6_safe::eraBp00_safe;
pub use crate::G6_safe::eraBp06_safe;
pub use crate::G6_safe::eraBpn2xy_safe;

pub use crate::G7_safe::eraC2i00a_safe;
pub use crate::G7_safe::eraC2i00b_safe;
pub use crate::G7_safe::eraC2i06a_safe;
pub use crate::G7_safe::eraC2ibpn_safe;
pub use crate::G7_safe::eraC2ixy_safe;
pub use crate::G7_safe::eraC2ixys_safe;
pub use crate::G7_safe::eraC2t00a_safe;
pub use crate::G7_safe::eraC2t00b_safe;
pub use crate::G7_safe::eraC2t06a_safe;

pub use crate::G8_safe::eraC2tcio_safe;
pub use crate::G8_safe::eraC2teqx_safe;
pub use crate::G8_safe::eraC2tpe_safe;
pub use crate::G8_safe::eraC2txy_safe;

pub use crate::G11_safe::eraEo06a_safe;
pub use crate::G11_safe::eraEors_safe;

pub use crate::G16_safe::eraFw2m_safe;
pub use crate::G16_safe::eraFw2xy_safe;

pub use crate::G20_safe::eraLtp_safe;
pub use crate::G20_safe::eraLtpb_safe;
pub use crate::G20_safe::eraLtpecl_safe;
pub use crate::G20_safe::eraLtpequ_safe;

pub use crate::G21_safe::eraNum00a_safe;
pub use crate::G21_safe::eraNum00b_safe;
pub use crate::G21_safe::eraNum06a_safe;
pub use crate::G21_safe::eraNumat_safe;

pub use crate::G22_safe::eraNut00a_safe;

pub use crate::G23_safe::eraNut00b_safe;
pub use crate::G23_safe::eraNut06a_safe;
pub use crate::G23_safe::eraNut80_safe;
pub use crate::G23_safe::eraNutm80_safe;
pub use crate::G23_safe::eraObl06_safe;
pub use crate::G23_safe::eraObl80_safe;

pub use crate::G24_safe::eraP06e_safe;
pub use crate::G24_safe::eraPb06_safe;
pub use crate::G24_safe::eraPfw06_safe;

pub use crate::G25_safe::eraPmat00_safe;
pub use crate::G25_safe::eraPmat06_safe;
pub use crate::G25_safe::eraPmat76_safe;
pub use crate::G25_safe::eraPn00_safe;
pub use crate::G25_safe::eraPn00a_safe;
pub use crate::G25_safe::eraPn00b_safe;

pub use crate::G26_safe::eraPn06_safe;
pub use crate::G26_safe::eraPn06a_safe;
pub use crate::G26_safe::eraPnm00a_safe;
pub use crate::G26_safe::eraPnm00b_safe;
pub use crate::G26_safe::eraPnm06a_safe;
pub use crate::G26_safe::eraPnm80_safe;
pub use crate::G26_safe::eraPom00_safe;
pub use crate::G26_safe::eraPr00_safe;

pub use crate::G27_safe::eraPrec76_safe;

pub use crate::G29_safe::eraS00_safe;
pub use crate::G29_safe::eraS00a_safe;
pub use crate::G29_safe::eraS00b_safe;

pub use crate::G30_safe::eraS06_safe;
pub use crate::G30_safe::eraS06a_safe;
pub use crate::G30_safe::eraSp00_safe;

pub use crate::G34_safe::eraXy06_safe;
pub use crate::G35_safe::eraXys00a_safe;
pub use crate::G35_safe::eraXys00b_safe;
pub use crate::G35_safe::eraXys06a_safe;

// Astronomy/RotationAndTime
pub use crate::G11_safe::eraEe00_safe;
pub use crate::G11_safe::eraEe00a_safe;
pub use crate::G11_safe::eraEe00b_safe;
pub use crate::G11_safe::eraEe06a_safe;
pub use crate::G11_safe::eraEect00_safe;

pub use crate::G14_safe::eraEqeq94_safe;
pub use crate::G14_safe::eraEra00_safe;

pub use crate::G17_safe::eraGmst00_safe;
pub use crate::G17_safe::eraGmst06_safe;
pub use crate::G17_safe::eraGmst82_safe;
pub use crate::G17_safe::eraGst00a_safe;
pub use crate::G17_safe::eraGst00b_safe;
pub use crate::G17_safe::eraGst06_safe;
pub use crate::G17_safe::eraGst06a_safe;
pub use crate::G17_safe::eraGst94_safe;

// Astronomy/SpaceMotion
pub use crate::G27_safe::eraPvstar_safe;
pub use crate::G30_safe::eraStarpv_safe;

// Astronomy/StarCatalogs
pub use crate::G15_safe::eraFk5hip_safe;
pub use crate::G16_safe::eraFk425_safe;
pub use crate::G16_safe::eraFk45z_safe;
pub use crate::G16_safe::eraFk524_safe;
pub use crate::G16_safe::eraFk52h_safe;
pub use crate::G16_safe::eraFk54z_safe;
pub use crate::G16_safe::eraFk5hz_safe;
pub use crate::G18_safe::eraH2fk5_safe;
pub use crate::G18_safe::eraHfk5z_safe;
pub use crate::G30_safe::eraStarpm_safe;

// Astronomy/EclipticCoordinates
pub use crate::G11_safe::eraEceq06_safe;
pub use crate::G11_safe::eraEcm06_safe;
pub use crate::G14_safe::eraEqec06_safe;
pub use crate::G20_safe::eraLteceq_safe;
pub use crate::G20_safe::eraLtecm_safe;
pub use crate::G20_safe::eraLteqec_safe;

// Astronomy/GalacticCoordinates
pub use crate::G17_safe::eraG2icrs_safe;
pub use crate::G19_safe::eraIcrs2g_safe;

// Astronomy/GeodeticGeocentric
pub use crate::G11_safe::eraEform_safe;
pub use crate::G17_safe::eraGc2gd_safe;
pub use crate::G17_safe::eraGc2gde_safe;
pub use crate::G17_safe::eraGd2gc_safe;
pub use crate::G17_safe::eraGd2gce_safe;

// Astronomy/Timescales
pub use crate::G10_safe::eraDtdb_safe;
pub use crate::G10_safe::eraDtf2d_safe;
pub use crate::G32_safe::eraTaitt_safe;
pub use crate::G32_safe::eraTaiut1_safe;
pub use crate::G32_safe::eraTaiutc_safe;
pub use crate::G32_safe::eraTcbtdb_safe;
pub use crate::G32_safe::eraTcgtt_safe;
pub use crate::G32_safe::eraTdbtcb_safe;
pub use crate::G32_safe::eraTdbtt_safe;
pub use crate::G33_safe::eraTttai_safe;
pub use crate::G33_safe::eraTttcg_safe;
pub use crate::G33_safe::eraTttdb_safe;
pub use crate::G33_safe::eraTtut1_safe;
pub use crate::G33_safe::eraUt1tai_safe;
pub use crate::G33_safe::eraUt1tt_safe;
pub use crate::G33_safe::eraUt1utc_safe;
pub use crate::G33_safe::eraUtctai_safe;
pub use crate::G33_safe::eraUtcut1_safe;
pub use crate::G9_safe::eraD2dtf_safe;
pub use crate::G9_safe::eraDat_safe;

// Astronomy/HorizonEquatorial
pub use crate::G18_safe::eraHd2ae_safe;
pub use crate::G18_safe::eraHd2pa_safe;
pub use crate::G1_safe::eraAe2hd_safe;

// Astronomy/Gnomonic
pub use crate::G32_safe::eraTpors_safe;
pub use crate::G32_safe::eraTporv_safe;
pub use crate::G32_safe::eraTpsts_safe;
pub use crate::G32_safe::eraTpstv_safe;
pub use crate::G32_safe::eraTpxes_safe;
pub use crate::G32_safe::eraTpxev_safe;

// VectorMatrix/AngleOps
pub use crate::G1_safe::eraA2af_safe;
pub use crate::G1_safe::eraA2tf_safe;
pub use crate::G1_safe::eraAf2a_safe;
pub use crate::G1_safe::eraAnp_safe;
pub use crate::G1_safe::eraAnpm_safe;
pub use crate::G32_safe::eraTf2a_safe;
pub use crate::G32_safe::eraTf2d_safe;
pub use crate::G9_safe::eraD2tf_safe;

// VectorMatrix/BuildRotations
pub use crate::G28_safe::eraRx_safe;
pub use crate::G28_safe::eraRy_safe;
pub use crate::G28_safe::eraRz_safe;

// VectorMatrix/CopyExtendExtract
pub use crate::G24_safe::eraP2pv_safe;
pub use crate::G27_safe::eraPv2p_safe;
pub use crate::G8_safe::eraCp_safe;
pub use crate::G8_safe::eraCpv_safe;
pub use crate::G8_safe::eraCr_safe;

// VectorMatrix/Initialization
pub use crate::G19_safe::eraIr_safe;
pub use crate::G35_safe::eraZp_safe;
pub use crate::G35_safe::eraZpv_safe;
pub use crate::G35_safe::eraZr_safe;

// VectorMatrix/MatrixOps
pub use crate::G28_safe::eraRxr_safe;
pub use crate::G33_safe::eraTr_safe;

// VectorMatrix/MatrixVectorProducts
pub use crate::G28_safe::eraRxp_safe;
pub use crate::G28_safe::eraRxpv_safe;
pub use crate::G33_safe::eraTrxp_safe;
pub use crate::G33_safe::eraTrxpv_safe;

// VectorMatrix/RotationVectors
pub use crate::G28_safe::eraRm2v_safe;
pub use crate::G28_safe::eraRv2m_safe;

// VectorMatrix/SeparationAndAngle
pub use crate::G24_safe::eraPap_safe;
pub use crate::G24_safe::eraPas_safe;
pub use crate::G30_safe::eraSepp_safe;
pub use crate::G30_safe::eraSeps_safe;

// VectorMatrix/SphericalCartesian
pub use crate::G24_safe::eraP2s_safe;
pub use crate::G27_safe::eraPv2s_safe;
pub use crate::G29_safe::eraS2c_safe;
pub use crate::G29_safe::eraS2p_safe;
pub use crate::G29_safe::eraS2pv_safe;
pub use crate::G7_safe::eraC2s_safe;

// VectorMatrix/VectorOps
pub use crate::G24_safe::eraPdp_safe;
pub use crate::G24_safe::eraPm_safe;
pub use crate::G25_safe::eraPmp_safe;
pub use crate::G25_safe::eraPn_safe;
pub use crate::G26_safe::eraPpp_safe;
pub use crate::G26_safe::eraPpsp_safe;
pub use crate::G27_safe::eraPvdpv_safe;
pub use crate::G27_safe::eraPvm_safe;
pub use crate::G27_safe::eraPvmpv_safe;
pub use crate::G27_safe::eraPvppv_safe;
pub use crate::G27_safe::eraPvu_safe;
pub use crate::G27_safe::eraPvup_safe;
pub use crate::G27_safe::eraPvxpv_safe;
pub use crate::G27_safe::eraPxp_safe;
pub use crate::G29_safe::eraS2xpv_safe;
pub use crate::G30_safe::eraSxp_safe;
pub use crate::G30_safe::eraSxpv_safe;


//  H1/erfaextra.h

// Leap-second table type
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct eraLEAPSECOND {
    pub iyear: i32,
    pub month: i32,
    pub delat: f64,
}

// Version/query helpers
pub use crate::G14_safe::eraSofaVersion_safe;
pub use crate::G14_safe::eraVersionMajor_safe;
pub use crate::G14_safe::eraVersionMicro_safe;
pub use crate::G14_safe::eraVersionMinor_safe;
pub use crate::G14_safe::eraVersion_safe;

// Experimental leap-second accessors
pub use crate::G14_safe::eraGetLeapSeconds_safe;
pub use crate::G14_safe::eraSetLeapSeconds_safe;


//  H1/erfadatextra.h

pub use crate::G14_safe::eraDatini_safe;


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

// Macro helpers → inline fn
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
