#![allow(dead_code)]

use erfa_rust::G14_safe;
use erfa_rust::H1_safe;
use erfa_rust::H1_safe::eraLEAPSECOND;

fn t_versions(status: &mut i32, verbose: bool) {
    let version_str = G14_safe::eraVersion_safe();
    let major = G14_safe::eraVersionMajor_safe();
    let minor = G14_safe::eraVersionMinor_safe();
    let micro = G14_safe::eraVersionMicro_safe();

    let mut ok = true;

    if !version_str.contains(&major.to_string()) {
        *status = 1;
        ok = false;
        println!(
            "t_versions failed - major not found in version string: {} (major={})",
            version_str, major
        );
    }
    if !version_str.contains(&minor.to_string()) {
        *status = 1;
        ok = false;
        println!(
            "t_versions failed - minor not found in version string: {} (minor={})",
            version_str, minor
        );
    }
    if !version_str.contains(&micro.to_string()) {
        *status = 1;
        ok = false;
        println!(
            "t_versions failed - micro not found in version string: {} (micro={})",
            version_str, micro
        );
    }

    if ok {
        if verbose {
            println!(
                "t_versions passed: {} ({}.{}.{})",
                version_str, major, minor, micro
            );
        } else {
            println!("t_versions passed");
        }
    }
}

fn t_leap_seconds(status: &mut i32, verbose: bool) {
    let init_table = match H1_safe::eraGetLeapSeconds_safe() {
        Ok(v) => v,
        Err(_) => {
            *status = 1;
            println!("t_leap_seconds failed - eraGetLeapSeconds_safe() returned Err");
            return;
        }
    };
    let count_init = init_table.len();
    if verbose {
        println!("initial leap-second entries: {}", count_init);
    }

    let fake = vec![eraLEAPSECOND {
        iyear: 2050,
        month: 5,
        delat: 55.0,
    }];

    if let Err(_) = H1_safe::eraSetLeapSeconds_safe(&fake[..]) {
        *status = 1;
        println!("t_leap_seconds set failed - eraSetLeapSeconds_safe() returned Err");
        return;
    }

    let postset_table = match H1_safe::eraGetLeapSeconds_safe() {
        Ok(v) => v,
        Err(_) => {
            *status = 1;
            println!("t_leap_seconds set failed - eraGetLeapSeconds_safe() Err after set");
            return;
        }
    };
    if postset_table.len() == 1 {
        println!("t_leap_seconds set passed");
    } else {
        *status = 1;
        println!(
            "t_leap_seconds set failed - entries {}, expected 1",
            postset_table.len()
        );
    }

    if let Err(_) = H1_safe::eraSetLeapSeconds_safe(&[]) {
        *status = 1;
        println!("t_leap_seconds reset failed - eraSetLeapSeconds_safe(&[]) Err");
        return;
    }

    let postreset_table = match H1_safe::eraGetLeapSeconds_safe() {
        Ok(v) => v,
        Err(_) => {
            *status = 1;
            println!("t_leap_seconds reset failed - eraGetLeapSeconds_safe() Err after reset");
            return;
        }
    };

    if postreset_table.len() == count_init {
        println!("t_leap_seconds reset passed");
    } else {
        *status = 1;
        println!(
            "t_leap_seconds reset failed - entries {}, expected {}",
            postreset_table.len(),
            count_init
        );
    }
}

fn main() {
    let verbose = std::env::args().len() > 1;

    let mut status = 0;
    t_versions(&mut status, verbose);
    t_leap_seconds(&mut status, verbose);

    if status != 0 {
        println!("t_erfa_safe_extra validation failed!");
    } else {
        println!("t_erfa_safe_extra validation successful");
    }
    std::process::exit(status);
}
