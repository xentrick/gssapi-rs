use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:5"]
pub mod types_h {
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:8"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_tm.h:13"]
pub mod struct_tm_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "7:8"]
    pub struct tm {
        pub tm_sec: libc::c_int,
        pub tm_min: libc::c_int,
        pub tm_hour: libc::c_int,
        pub tm_mday: libc::c_int,
        pub tm_mon: libc::c_int,
        pub tm_year: libc::c_int,
        pub tm_wday: libc::c_int,
        pub tm_yday: libc::c_int,
        pub tm_isdst: libc::c_int,
        pub tm_gmtoff: libc::c_long,
        pub tm_zone: *const libc::c_char,
    }
}
#[c2rust::header_src = "/usr/include/time.h:13"]
pub mod time_h {
    use super::struct_tm_h::tm;
    use super::time_t_h::time_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "196:1"]
        pub fn timegm(__tp: *mut tm) -> time_t;
    }
}
pub use self::types_h::__time_t;
pub use self::time_t_h::time_t;
pub use self::struct_tm_h::tm;
use self::time_h::timegm;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-gmt_mktime.h */
/*
 * Copyright 2008 Massachusetts Institute of Technology.
 * All Rights Reserved.
 *
 * Export of this software from the United States of America may
 *   require a specific license from the United States Government.
 *   It is the responsibility of any person or organization contemplating
 *   export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of M.I.T. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 * M.I.T. makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 */
/*
 *
 * GMT struct tm conversion
 *
 * Because of ordering of things in the UNIX build, we can't just keep
 * the declaration in k5-int.h and include it in
 * util/support/gmt_mktime.c, since k5-int.h includes krb5.h which
 * hasn't been built when gmt_mktime.c gets compiled.  Hence this
 * silly little helper header.
 */
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* This code placed in the public domain by Mark W. Eichin */
/*
 * Use the nonstandard timegm() (if available) to convert broken-down
 * UTC times into time_t values.  Use our custom gmt_mktime() if
 * timegm() is not available.
 *
 * We use gmtime() (or gmtime_r()) when encoding ASN.1 GeneralizedTime
 * values.  On systems where a "right" (leap-second-aware) time zone
 * is configured, gmtime() adjusts for the presence of accumulated
 * leap seconds in the input time_t value.  POSIX requires that time_t
 * values omit leap seconds; systems configured to include leap
 * seconds in their time_t values are non-conforming and will have
 * difficulties exchanging timestamp information with other systems.
 *
 * We use krb5int_gmt_mktime() for decoding ASN.1 GeneralizedTime
 * values.  If timegm() is not available, krb5int_gmt_mktime() won't
 * be the inverse of gmtime() on a system that counts leap seconds.  A
 * system configured with a "right" time zone probably has timegm()
 * available; without it, an application would have no reliable way of
 * converting broken-down UTC times into time_t values.
 */
#[no_mangle]
#[c2rust::src_loc = "45:1"]
pub unsafe extern "C" fn krb5int_gmt_mktime(mut t: *mut tm) -> time_t {
    return timegm(t);
}
/* !HAVE_TIMEGM || TEST_LEAP */
