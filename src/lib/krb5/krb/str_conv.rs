use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:44"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:44"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:44"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:44"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:44"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_tm.h:44"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:44"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    /* *
 * Represents a timestamp in seconds since the POSIX epoch.  This legacy type
 * is used frequently in the ABI, but cannot represent timestamps after 2038 as
 * a positive number.  Code which uses this type should cast values of it to
 * uint32_t so that negative values are treated as timestamps between 2038 and
 * 2106 on platforms with 64-bit time_t.
 */
    #[c2rust::src_loc = "195:1"]
    pub type krb5_timestamp = krb5_int32;
    #[c2rust::src_loc = "197:1"]
    pub type krb5_deltat = krb5_int32;
    /* *
 * Used to convey an operation status.  The value 0 indicates success; any
 * other values are com_err codes.  Use krb5_get_error_message() to obtain a
 * string describing the error.
 */
    #[c2rust::src_loc = "204:1"]
    pub type krb5_error_code = krb5_int32;
    #[c2rust::src_loc = "206:1"]
    pub type krb5_magic = krb5_error_code;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "208:16"]
    pub struct _krb5_data {
        pub magic: krb5_magic,
        pub length: libc::c_uint,
        pub data: *mut libc::c_char,
    }
    #[c2rust::src_loc = "208:1"]
    pub type krb5_data = _krb5_data;
    use super::stdint_intn_h::int32_t;
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/ctype.h:45"]
pub mod ctype_h {
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed = 8192;
    #[c2rust::src_loc = "46:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed = 8;
    #[c2rust::src_loc = "58:3"]
    pub const _ISpunct: C2RustUnnamed = 4;
    #[c2rust::src_loc = "57:3"]
    pub const _IScntrl: C2RustUnnamed = 2;
    #[c2rust::src_loc = "56:3"]
    pub const _ISblank: C2RustUnnamed = 1;
    #[c2rust::src_loc = "55:3"]
    pub const _ISgraph: C2RustUnnamed = 32768;
    #[c2rust::src_loc = "54:3"]
    pub const _ISprint: C2RustUnnamed = 16384;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed = 4096;
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed = 2048;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed = 256;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:44"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:44"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/time.h:44"]
pub mod time_h {
    use super::time_t_h::time_t;
    use super::struct_tm_h::tm;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "75:1"]
        pub fn time(__timer: *mut time_t) -> time_t;
        #[no_mangle]
        #[c2rust::src_loc = "82:1"]
        pub fn mktime(__tp: *mut tm) -> time_t;
        #[no_mangle]
        #[c2rust::src_loc = "88:1"]
        pub fn strftime(__s: *mut libc::c_char, __maxsize: size_t,
                        __format: *const libc::c_char, __tp: *const tm)
         -> size_t;
        #[no_mangle]
        #[c2rust::src_loc = "95:1"]
        pub fn strptime(__s: *const libc::c_char, __fmt: *const libc::c_char,
                        __tp: *mut tm) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "133:1"]
        pub fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:44"]
pub mod k5_platform_h {
    use super::stddef_h::size_t;
    extern "C" {
        /* Make the interfaces to getpwnam and getpwuid consistent.
   Model the wrappers on the POSIX thread-safe versions, but
   use the unsafe system versions if the safe ones don't exist
   or we can't figure out their interfaces.  */
        /* int k5_getpwnam_r(const char *, blah blah) */
        /* POSIX */
        /* no getpwnam_r, or can't figure out #args or return type */
        /* int k5_getpwuid_r(uid_t, blah blah) */
        /* POSIX */
        /* no getpwuid_r, or can't figure out #args or return type */
        /* Ensure, if possible, that the indicated file descriptor won't be
   kept open if we exec another process (e.g., launching a ccapi
   server).  If we don't know how to do it... well, just go about our
   business.  Probably most callers won't check the return status
   anyways.  */
        /* Macros make the Sun compiler happier, and all variants of this do a
   single evaluation of the argument, and fcntl and fileno should
   produce reasonable error messages on type mismatches, on any system
   with F_SETFD.  */
        /* Since the original ANSI C spec left it undefined whether or
   how you could copy around a va_list, C 99 added va_copy.
   For old implementations, let's do our best to fake it.

   XXX Doesn't yet handle implementations with __va_copy (early draft)
   or GCC's __builtin_va_copy.  */
        /* Do nothing.  */
        /* Provide strlcpy/strlcat interfaces. */
        #[no_mangle]
        #[c2rust::src_loc = "887:1"]
        pub fn krb5int_strlcpy(dst: *mut libc::c_char,
                               src: *const libc::c_char, siz: size_t)
         -> size_t;
    }
    /* K5_PLATFORM_H */
}
#[c2rust::header_src = "/usr/include/strings.h:44"]
pub mod strings_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "116:12"]
        pub fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:44"]
pub mod k5_int_h {
    #[inline]
    #[c2rust::src_loc = "2338:1"]
    pub unsafe extern "C" fn ts2tt(mut timestamp: krb5_timestamp) -> time_t {
        return timestamp as uint32_t as time_t;
    }
    use super::krb5_h::krb5_timestamp;
    use super::time_t_h::time_t;
    use super::stdint_uintn_h::uint32_t;
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
pub use self::types_h::{__int32_t, __uint32_t, __time_t};
pub use self::time_t_h::time_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::struct_tm_h::tm;
pub use self::krb5_h::{krb5_int32, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data};
pub use self::ctype_h::{_ISspace, C2RustUnnamed, _ISalnum, _ISpunct, _IScntrl,
                        _ISblank, _ISgraph, _ISprint, _ISxdigit, _ISdigit,
                        _ISalpha, _ISlower, _ISupper, __ctype_b_loc};
use self::stdio_h::snprintf;
use self::errno_h::__errno_location;
use self::time_h::{time, mktime, strftime, strptime, localtime_r};
use self::k5_platform_h::krb5int_strlcpy;
use self::strings_h::strcasecmp;
pub use self::k5_int_h::ts2tt;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/krb/str_conv.c - Convert between strings and krb5 data types */
/*
 * Copyright 1995, 1999, 2007 by the Massachusetts Institute of Technology.
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
 * Table of contents:
 *
 * String decoding:
 * ----------------
 * krb5_string_to_salttype()    - Convert string to salttype (krb5_int32)
 * krb5_string_to_timestamp()   - Convert string to krb5_timestamp.
 * krb5_string_to_deltat()      - Convert string to krb5_deltat.
 *
 * String encoding:
 * ----------------
 * krb5_salttype_to_string()    - Convert salttype (krb5_int32) to string.
 * krb5_timestamp_to_string()   - Convert krb5_timestamp to string.
 * krb5_timestamp_to_sfstring() - Convert krb5_timestamp to short filled string
 * krb5_deltat_to_string()      - Convert krb5_deltat to string.
 */
/* Salt type conversions */
/*
 * Local data structures.
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "52:8"]
pub struct salttype_lookup_entry {
    pub stt_enctype: krb5_int32,
    pub stt_name: *const libc::c_char,
}
/* Salt type name */
/*
 * Lookup tables.
 */
#[c2rust::src_loc = "62:43"]
static mut salttype_table: [salttype_lookup_entry; 4] =
    [{
         let mut init =
             salttype_lookup_entry{stt_enctype: 0 as libc::c_int,
                                   stt_name:
                                       b"normal\x00" as *const u8 as
                                           *const libc::c_char,};
         init
     },
     {
         let mut init =
             salttype_lookup_entry{stt_enctype: 2 as libc::c_int,
                                   stt_name:
                                       b"norealm\x00" as *const u8 as
                                           *const libc::c_char,};
         init
     },
     {
         let mut init =
             salttype_lookup_entry{stt_enctype: 3 as libc::c_int,
                                   stt_name:
                                       b"onlyrealm\x00" as *const u8 as
                                           *const libc::c_char,};
         init
     },
     {
         let mut init =
             salttype_lookup_entry{stt_enctype: 4 as libc::c_int,
                                   stt_name:
                                       b"special\x00" as *const u8 as
                                           *const libc::c_char,};
         init
     }];
// Initialized in run_static_initializers
#[c2rust::src_loc = "68:18"]
static mut salttype_table_nents: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "71:1"]
pub unsafe extern "C" fn krb5_string_to_salttype(mut string:
                                                     *mut libc::c_char,
                                                 mut salttypep:
                                                     *mut krb5_int32)
 -> krb5_error_code {
    let mut i: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    found = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < salttype_table_nents {
        if strcasecmp(string, salttype_table[i as usize].stt_name) == 0 {
            found = 1 as libc::c_int;
            *salttypep = salttype_table[i as usize].stt_enctype;
            break ;
        } else { i += 1 }
    }
    return if found != 0 { 0 as libc::c_int } else { 22 as libc::c_int };
}
/*
 * Internal datatype to string routines.
 *
 * These routines return 0 for success, EINVAL for invalid parameter, ENOMEM
 * if the supplied buffer/length will not contain the output.
 */
#[no_mangle]
#[c2rust::src_loc = "94:1"]
pub unsafe extern "C" fn krb5_salttype_to_string(mut salttype: krb5_int32,
                                                 mut buffer:
                                                     *mut libc::c_char,
                                                 mut buflen: size_t)
 -> krb5_error_code {
    let mut i: libc::c_int = 0;
    let mut out: *const libc::c_char = 0 as *const libc::c_char;
    out = 0 as *mut libc::c_void as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < salttype_table_nents {
        if salttype == salttype_table[i as usize].stt_enctype {
            out = salttype_table[i as usize].stt_name;
            break ;
        } else { i += 1 }
    }
    if !out.is_null() {
        if krb5int_strlcpy(buffer, out, buflen) >= buflen {
            return 12 as libc::c_int
        }
        return 0 as libc::c_int
    } else { return 22 as libc::c_int };
}
// Initialized in run_static_initializers
static mut atime_format_table_nents: libc::c_int = 0;
/* (absolute) time conversions */
/* HAVE_STRPTIME */
#[no_mangle]
#[c2rust::src_loc = "145:1"]
pub unsafe extern "C" fn krb5_string_to_timestamp(mut string:
                                                      *mut libc::c_char,
                                                  mut timestampp:
                                                      *mut krb5_timestamp)
 -> krb5_error_code {
    let mut i: libc::c_int = 0;
    let mut timebuf: tm =
        tm{tm_sec: 0,
           tm_min: 0,
           tm_hour: 0,
           tm_mday: 0,
           tm_mon: 0,
           tm_year: 0,
           tm_wday: 0,
           tm_yday: 0,
           tm_isdst: 0,
           tm_gmtoff: 0,
           tm_zone: 0 as *const libc::c_char,};
    let mut timebuf2: tm =
        tm{tm_sec: 0,
           tm_min: 0,
           tm_hour: 0,
           tm_mday: 0,
           tm_mon: 0,
           tm_year: 0,
           tm_wday: 0,
           tm_yday: 0,
           tm_isdst: 0,
           tm_gmtoff: 0,
           tm_zone: 0 as *const libc::c_char,};
    let mut now: time_t = 0;
    let mut ret_time: time_t = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut atime_format_table: [*const libc::c_char; 12] =
        [b"%Y%m%d%H%M%S\x00" as *const u8 as *const libc::c_char,
         b"%Y.%m.%d.%H.%M.%S\x00" as *const u8 as *const libc::c_char,
         b"%y%m%d%H%M%S\x00" as *const u8 as *const libc::c_char,
         b"%y.%m.%d.%H.%M.%S\x00" as *const u8 as *const libc::c_char,
         b"%y%m%d%H%M\x00" as *const u8 as *const libc::c_char,
         b"%H%M%S\x00" as *const u8 as *const libc::c_char,
         b"%H%M\x00" as *const u8 as *const libc::c_char,
         b"%T\x00" as *const u8 as *const libc::c_char,
         b"%R\x00" as *const u8 as *const libc::c_char,
         b"%x:%X\x00" as *const u8 as *const libc::c_char,
         b"%d-%b-%Y:%T\x00" as *const u8 as *const libc::c_char,
         b"%d-%b-%Y:%R\x00" as *const u8 as *const libc::c_char];
    now = time(0 as *mut libc::c_void as *mut time_t);
    if localtime_r(&mut now, &mut timebuf2).is_null() {
        return 22 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < atime_format_table_nents {
        /* We reset every time throughout the loop as the manual page
         * indicated that no guarantees are made as to preserving timebuf
         * when parsing fails
         */
        timebuf = timebuf2;
        s = strptime(string, atime_format_table[i as usize], &mut timebuf);
        if !s.is_null() && s != string {
            /* See if at end of buffer - otherwise partial processing */
            while *s as libc::c_int != 0 as libc::c_int &&
                      *(*__ctype_b_loc()).offset(*s as libc::c_int as isize)
                          as libc::c_int &
                          _ISspace as libc::c_int as libc::c_ushort as
                              libc::c_int != 0 {
                s = s.offset(1)
            } /* clearly confused */
            if !(*s as libc::c_int != 0 as libc::c_int) {
                if !(timebuf.tm_year <= 0 as libc::c_int) {
                    ret_time = mktime(&mut timebuf); /* clearly confused */
                    if !(ret_time == -(1 as libc::c_int) as time_t) {
                        *timestampp =
                            ret_time as
                                krb5_timestamp; /* This is to get around gcc -Wall warning that
                               the year returned might be two digits */
                        return 0 as libc::c_int
                    }
                }
            }
        }
        i += 1
    }
    return 22 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "198:1"]
pub unsafe extern "C" fn krb5_timestamp_to_string(mut timestamp:
                                                      krb5_timestamp,
                                                  mut buffer:
                                                      *mut libc::c_char,
                                                  mut buflen: size_t)
 -> krb5_error_code {
    let mut ret: size_t = 0;
    let mut timestamp2: time_t = ts2tt(timestamp);
    let mut tmbuf: tm =
        tm{tm_sec: 0,
           tm_min: 0,
           tm_hour: 0,
           tm_mday: 0,
           tm_mon: 0,
           tm_year: 0,
           tm_wday: 0,
           tm_yday: 0,
           tm_isdst: 0,
           tm_gmtoff: 0,
           tm_zone: 0 as *const libc::c_char,};
    let mut fmt: *const libc::c_char =
        b"%c\x00" as *const u8 as *const libc::c_char;
    if localtime_r(&mut timestamp2, &mut tmbuf).is_null() {
        return 12 as libc::c_int
    }
    ret = strftime(buffer, buflen, fmt, &mut tmbuf);
    if ret == 0 as libc::c_int as libc::c_ulong || ret == buflen {
        return 12 as libc::c_int
    }
    return 0 as libc::c_int;
}
// Initialized in run_static_initializers
static mut sftime_format_table_nents: libc::c_uint = 0;
#[no_mangle]
#[c2rust::src_loc = "215:1"]
pub unsafe extern "C" fn krb5_timestamp_to_sfstring(mut timestamp:
                                                        krb5_timestamp,
                                                    mut buffer:
                                                        *mut libc::c_char,
                                                    mut buflen: size_t,
                                                    mut pad:
                                                        *mut libc::c_char)
 -> krb5_error_code {
    let mut tmp: *mut tm = 0 as *mut tm;
    let mut i: size_t = 0;
    let mut ndone: size_t = 0;
    let mut timestamp2: time_t = ts2tt(timestamp);
    let mut tmbuf: tm =
        tm{tm_sec: 0,
           tm_min: 0,
           tm_hour: 0,
           tm_mday: 0,
           tm_mon: 0,
           tm_year: 0,
           tm_wday: 0,
           tm_yday: 0,
           tm_isdst: 0,
           tm_gmtoff: 0,
           tm_zone: 0 as *const libc::c_char,};
    static mut sftime_format_table: [*const libc::c_char; 9] =
        [b"%c\x00" as *const u8 as *const libc::c_char,
         b"%d %b %Y %T\x00" as *const u8 as *const libc::c_char,
         b"%x %X\x00" as *const u8 as *const libc::c_char,
         b"%x %T\x00" as *const u8 as *const libc::c_char,
         b"%x %R\x00" as *const u8 as *const libc::c_char,
         b"%Y-%m-%dT%H:%M:%S\x00" as *const u8 as *const libc::c_char,
         b"%Y-%m-%dT%H:%M\x00" as *const u8 as *const libc::c_char,
         b"%Y%m%d%H%M%S\x00" as *const u8 as *const libc::c_char,
         b"%Y%m%d%H%M\x00" as *const u8 as *const libc::c_char];
    tmp = localtime_r(&mut timestamp2, &mut tmbuf);
    if tmp.is_null() { return *__errno_location() }
    ndone = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < sftime_format_table_nents as libc::c_ulong {
        ndone =
            strftime(buffer, buflen, sftime_format_table[i as usize], tmp);
        if ndone != 0 { break ; }
        i = i.wrapping_add(1)
    }
    if ndone != 0 && !pad.is_null() {
        i = ndone;
        while i < buflen.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            *buffer.offset(i as isize) = *pad;
            i = i.wrapping_add(1)
        }
        *buffer.offset(buflen.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                           as isize) = '\u{0}' as i32 as libc::c_char
    }
    return if ndone != 0 { 0 as libc::c_int } else { 12 as libc::c_int };
}
/* relative time (delta-t) conversions */
/* string->deltat is in deltat.y */
#[no_mangle]
#[c2rust::src_loc = "258:1"]
pub unsafe extern "C" fn krb5_deltat_to_string(mut deltat: krb5_deltat,
                                               mut buffer: *mut libc::c_char,
                                               mut buflen: size_t)
 -> krb5_error_code {
    let mut days: libc::c_int = 0;
    let mut hours: libc::c_int = 0;
    let mut minutes: libc::c_int = 0;
    let mut seconds: libc::c_int = 0;
    let mut dt: krb5_deltat = 0;
    days =
        (deltat as libc::c_long /
             (24 as libc::c_int as libc::c_long * 3600 as libc::c_long)) as
            libc::c_int;
    dt =
        (deltat as libc::c_long %
             (24 as libc::c_int as libc::c_long * 3600 as libc::c_long)) as
            krb5_deltat;
    hours = dt / 3600 as libc::c_int;
    dt %= 3600 as libc::c_int;
    minutes = dt / 60 as libc::c_int;
    seconds = dt % 60 as libc::c_int;
    if days == 0 as libc::c_int {
        snprintf(buffer, buflen,
                 b"%d:%02d:%02d\x00" as *const u8 as *const libc::c_char,
                 hours, minutes, seconds);
    } else if hours != 0 || minutes != 0 || seconds != 0 {
        snprintf(buffer, buflen,
                 b"%d %s %02d:%02d:%02d\x00" as *const u8 as
                     *const libc::c_char, days,
                 if days > 1 as libc::c_int {
                     b"days\x00" as *const u8 as *const libc::c_char
                 } else { b"day\x00" as *const u8 as *const libc::c_char },
                 hours, minutes, seconds);
    } else {
        snprintf(buffer, buflen,
                 b"%d %s\x00" as *const u8 as *const libc::c_char, days,
                 if days > 1 as libc::c_int {
                     b"days\x00" as *const u8 as *const libc::c_char
                 } else { b"day\x00" as *const u8 as *const libc::c_char });
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn run_static_initializers() {
    salttype_table_nents =
        (::std::mem::size_of::<[salttype_lookup_entry; 4]>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<salttype_lookup_entry>()
                                             as libc::c_ulong) as libc::c_int;
    atime_format_table_nents =
        (::std::mem::size_of::<[*const libc::c_char; 12]>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                             as libc::c_ulong) as libc::c_int;
    sftime_format_table_nents =
        (::std::mem::size_of::<[*const libc::c_char; 9]>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                             as libc::c_ulong) as libc::c_uint
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
