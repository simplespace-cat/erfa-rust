#![allow(dead_code)]

use erfa_rust_cc::H1;
use std::ffi::CStr;
use std::ptr;

static mut VERBOSE: bool = false;

fn t_versions(status: &mut i32) {
    let version_str = unsafe {
        let c_str_ptr = H1::eraVersion();
        if c_str_ptr.is_null() {
            *status = 1;
            println!("t_versions failed - eraVersion() returned a null pointer");
            return;
        }
        CStr::from_ptr(c_str_ptr).to_string_lossy()
    };

    let major = unsafe { H1::eraVersionMajor() };
    let minor = unsafe { H1::eraVersionMinor() };
    let micro = unsafe { H1::eraVersionMicro() };

    if !version_str.contains(&major.to_string()) {
        *status = 1;
        println!(
            "t_versions failed - major version not in version string {}",
            version_str
        );
    }

    if !version_str.contains(&minor.to_string()) {
        *status = 1;
        println!(
            "t_versions failed - minor version not in version string {}",
            version_str
        );
    }

    if !version_str.contains(&micro.to_string()) {
        *status = 1;
        println!(
            "t_versions failed - micro version not in version string {}",
            version_str
        );
    }

    if *status == 0 {
        println!("t_versions passed");
    }
}

fn t_leap_seconds(status: &mut i32) {
    let mut fake_leapsecond = [H1::eraLEAPSECOND {
        iyear: 2050,
        month: 5,
        delat: 55.0,
    }];

    let mut leapseconds_init_ptr: *mut H1::eraLEAPSECOND = ptr::null_mut();
    let count_init = unsafe { H1::eraGetLeapSeconds(&mut leapseconds_init_ptr) };

    unsafe { H1::eraSetLeapSeconds(fake_leapsecond.as_mut_ptr(), 1) };

    let mut leapseconds_postset_ptr: *mut H1::eraLEAPSECOND = ptr::null_mut();
    let count_postset = unsafe { H1::eraGetLeapSeconds(&mut leapseconds_postset_ptr) };

    if count_postset == 1 {
        println!("t_leap_seconds set passed");
    } else {
        *status = 1;
        println!(
            "t_leap_seconds set failed - leap second table has {} entries instead of 1",
            count_postset
        );
    }

    unsafe { H1::eraSetLeapSeconds(fake_leapsecond.as_mut_ptr(), -1) };

    let mut leapseconds_postreset_ptr: *mut H1::eraLEAPSECOND = ptr::null_mut();
    let count_postreset = unsafe { H1::eraGetLeapSeconds(&mut leapseconds_postreset_ptr) };

    if count_postreset == count_init {
        println!("t_leap_seconds reset passed");
    } else {
        *status = 1;
        println!(
            "t_leap_seconds reset failed - leap second table has {} entries instead of {}",
            count_postreset, count_init
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        unsafe {
            VERBOSE = true;
        }
    }

    let mut status = 0;

    t_versions(&mut status);
    t_leap_seconds(&mut status);

    if status != 0 {
        println!("t_erfa_c_extra validation failed!");
    } else {
        println!("t_erfa_c_extra validation successful");
    }
    std::process::exit(status);
}
