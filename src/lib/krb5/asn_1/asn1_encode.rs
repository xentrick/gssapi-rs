use ::libc;
use c2rust_bitfields::*;
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = libc::c_schar;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = libc::c_long;
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = libc::c_ulong;
    #[c2rust::src_loc = "72:1"]
    pub type __intmax_t = libc::c_long;
    #[c2rust::src_loc = "73:1"]
    pub type __uintmax_t = libc::c_ulong;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:27"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:27"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:27"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int8_t, __int16_t, __int32_t, __int64_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:27"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t, __uint64_t};
}
#[c2rust::header_src = "/usr/include/stdint.h:27"]
pub mod stdint_h {
    #[c2rust::src_loc = "101:1"]
    pub type intmax_t = __intmax_t;
    #[c2rust::src_loc = "102:1"]
    pub type uintmax_t = __uintmax_t;
    use super::types_h::{__intmax_t, __uintmax_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_tm.h:27"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:27"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/asn.1/krbasn1.h:27"]
pub mod krbasn1_h {
    #[c2rust::src_loc = "32:9"]
    pub type asn1_construction = libc::c_uint;
    #[c2rust::src_loc = "32:34"]
    pub const CONSTRUCTED: asn1_construction = 32;
    #[c2rust::src_loc = "32:16"]
    pub const PRIMITIVE: asn1_construction = 0;
    #[c2rust::src_loc = "34:9"]
    pub type asn1_class = libc::c_uint;
    #[c2rust::src_loc = "35:41"]
    pub const PRIVATE: asn1_class = 192;
    #[c2rust::src_loc = "35:16"]
    pub const CONTEXT_SPECIFIC: asn1_class = 128;
    #[c2rust::src_loc = "34:34"]
    pub const APPLICATION: asn1_class = 64;
    #[c2rust::src_loc = "34:16"]
    pub const UNIVERSAL: asn1_class = 0;
    #[c2rust::src_loc = "37:1"]
    pub type asn1_tagnum = libc::c_int;
    /* Kerberos Message Types */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/asn.1/asn1_encode.h:27"]
pub mod asn1_encode_h {
    #[c2rust::src_loc = "34:1"]
    pub type asn1buf = asn1buf_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:9"]
    pub struct taginfo {
        pub asn1class: asn1_class,
        pub construction: asn1_construction,
        pub tagnum: asn1_tagnum,
        pub tag_len: size_t,
        pub tag_end_len: size_t,
    }
    #[c2rust::src_loc = "87:1"]
    pub type atype_type = libc::c_uint;
    #[c2rust::src_loc = "138:5"]
    pub const atype_max: atype_type = 15;
    #[c2rust::src_loc = "136:5"]
    pub const atype_int_immediate: atype_type = 14;
    #[c2rust::src_loc = "129:5"]
    pub const atype_uint: atype_type = 13;
    #[c2rust::src_loc = "128:5"]
    pub const atype_int: atype_type = 12;
    #[c2rust::src_loc = "126:5"]
    pub const atype_bool: atype_type = 11;
    #[c2rust::src_loc = "124:5"]
    pub const atype_tagged_thing: atype_type = 10;
    #[c2rust::src_loc = "122:5"]
    pub const atype_nonempty_nullterm_sequence_of: atype_type = 9;
    #[c2rust::src_loc = "121:5"]
    pub const atype_nullterm_sequence_of: atype_type = 8;
    #[c2rust::src_loc = "114:5"]
    pub const atype_sequence: atype_type = 7;
    #[c2rust::src_loc = "112:5"]
    pub const atype_counted: atype_type = 6;
    #[c2rust::src_loc = "106:5"]
    pub const atype_optional: atype_type = 5;
    #[c2rust::src_loc = "99:5"]
    pub const atype_offset: atype_type = 4;
    #[c2rust::src_loc = "96:5"]
    pub const atype_ptr: atype_type = 3;
    #[c2rust::src_loc = "93:5"]
    pub const atype_fn: atype_type = 2;
    #[c2rust::src_loc = "90:5"]
    pub const atype_min: atype_type = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "141:8"]
    pub struct atype_info {
        pub type_0: atype_type,
        pub size: size_t,
        pub tinfo: *const libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "147:8"]
    pub struct fn_info {
        pub enc: Option<unsafe extern "C" fn(_: *mut asn1buf,
                                             _: *const libc::c_void,
                                             _: *mut taginfo)
                            -> krb5_error_code>,
        pub dec: Option<unsafe extern "C" fn(_: *const taginfo,
                                             _: *const uint8_t, _: size_t,
                                             _: *mut libc::c_void)
                            -> krb5_error_code>,
        pub check_tag: Option<unsafe extern "C" fn(_: *const taginfo)
                                  -> libc::c_int>,
        pub free_func: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                  -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "154:8"]
    pub struct ptr_info {
        pub loadptr: Option<unsafe extern "C" fn(_: *const libc::c_void)
                                -> *mut libc::c_void>,
        pub storeptr: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                  _: *mut libc::c_void)
                                 -> ()>,
        pub basetype: *const atype_info,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "160:8"]
    pub struct offset_info {
        #[bitfield(name = "dataoff", ty = "libc::c_uint", bits = "0..=8")]
        pub dataoff: [u8; 2],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 6],
        pub basetype: *const atype_info,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "165:8"]
    pub struct optional_info {
        pub is_present: Option<unsafe extern "C" fn(_: *const libc::c_void)
                                   -> libc::c_int>,
        pub init: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        pub basetype: *const atype_info,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "171:8"]
    pub struct counted_info {
        #[bitfield(name = "dataoff", ty = "libc::c_uint", bits = "0..=8")]
        #[bitfield(name = "lenoff", ty = "libc::c_uint", bits = "9..=17")]
        #[bitfield(name = "lensigned", ty = "libc::c_uint", bits = "18..=18")]
        #[bitfield(name = "lensize", ty = "libc::c_uint", bits = "19..=23")]
        pub dataoff_lenoff_lensigned_lensize: [u8; 3],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 5],
        pub basetype: *const cntype_info,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "220:8"]
    pub struct cntype_info {
        pub type_0: cntype_type,
        pub tinfo: *const libc::c_void,
    }
    #[c2rust::src_loc = "192:1"]
    pub type cntype_type = libc::c_uint;
    #[c2rust::src_loc = "217:5"]
    pub const cntype_max: cntype_type = 6;
    #[c2rust::src_loc = "215:5"]
    pub const cntype_choice: cntype_type = 5;
    #[c2rust::src_loc = "211:5"]
    pub const cntype_seqof: cntype_type = 4;
    #[c2rust::src_loc = "207:5"]
    pub const cntype_der: cntype_type = 3;
    #[c2rust::src_loc = "200:5"]
    pub const cntype_string: cntype_type = 2;
    #[c2rust::src_loc = "193:5"]
    pub const cntype_min: cntype_type = 1;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "179:8"]
    pub struct tagged_info {
        #[bitfield(name = "tagval", ty = "libc::c_uint", bits = "0..=15")]
        #[bitfield(name = "tagtype", ty = "libc::c_uint", bits = "16..=23")]
        #[bitfield(name = "construction", ty = "libc::c_uint", bits =
                   "24..=29")]
        #[bitfield(name = "implicit", ty = "libc::c_uint", bits = "30..=30")]
        pub tagval_tagtype_construction_implicit: [u8; 4],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 4],
        pub basetype: *const atype_info,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "184:8"]
    pub struct immediate_info {
        pub val: intmax_t,
        pub err: krb5_error_code,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "225:8"]
    pub struct string_info {
        pub enc: Option<unsafe extern "C" fn(_: *mut asn1buf,
                                             _: *const *mut uint8_t,
                                             _: size_t) -> krb5_error_code>,
        pub dec: Option<unsafe extern "C" fn(_: *const uint8_t, _: size_t,
                                             _: *mut *mut uint8_t,
                                             _: *mut size_t)
                            -> krb5_error_code>,
        #[bitfield(name = "tagval", ty = "libc::c_uint", bits = "0..=4")]
        pub tagval: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "231:8"]
    pub struct choice_info {
        pub options: *mut *const atype_info,
        pub n_options: size_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "236:8"]
    pub struct seq_info {
        pub fields: *mut *const atype_info,
        pub n_fields: size_t,
    }
    use super::asn1buf_st;
    use super::krbasn1_h::{asn1_class, asn1_construction, asn1_tagnum};
    use super::stddef_h::size_t;
    use super::krb5_h::krb5_error_code;
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_h::intmax_t;
    /*
 * Ugly hack!
 * Like "offsetof", but with type checking.
 */
    /* gobble semicolon */
    /* gobble semicolon */
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:27"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/time.h:27"]
pub mod time_h {
    use super::time_t_h::time_t;
    use super::struct_tm_h::tm;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "128:1"]
        pub fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    }
}
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:27"]
pub mod assert_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn __assert_fail(__assertion: *const libc::c_char,
                             __file: *const libc::c_char,
                             __line: libc::c_uint,
                             __function: *const libc::c_char) -> !;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-gmt_mktime.h:27"]
pub mod k5_gmt_mktime_h {
    use super::struct_tm_h::tm;
    use super::time_t_h::time_t;
    extern "C" {
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
        #[no_mangle]
        #[c2rust::src_loc = "51:1"]
        pub fn krb5int_gmt_mktime(_: *mut tm) -> time_t;
    }
    /* K5_GMT_MKTIME_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:27"]
pub mod k5_int_h {
    #[inline]
    #[c2rust::src_loc = "2251:1"]
    pub unsafe extern "C" fn make_data(mut data: *mut libc::c_void,
                                       mut len: libc::c_uint) -> krb5_data {
        let mut d: krb5_data =
            krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
        d.magic = -(1760647422 as libc::c_long) as krb5_magic;
        d.data = data as *mut libc::c_char;
        d.length = len;
        return d;
    }
    use super::krb5_h::{krb5_data, krb5_magic};
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
pub use self::types_h::{__int8_t, __uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t, __int64_t, __uint64_t, __intmax_t,
                        __uintmax_t, __time_t};
pub use self::time_t_h::time_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int8_t, int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t, uint64_t};
pub use self::stdint_h::{intmax_t, uintmax_t};
pub use self::struct_tm_h::tm;
pub use self::krb5_h::{krb5_int32, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data};
pub use self::krbasn1_h::{asn1_construction, CONSTRUCTED, PRIMITIVE,
                          asn1_class, PRIVATE, CONTEXT_SPECIFIC, APPLICATION,
                          UNIVERSAL, asn1_tagnum};
pub use self::asn1_encode_h::{asn1buf, taginfo, atype_type, atype_max,
                              atype_int_immediate, atype_uint, atype_int,
                              atype_bool, atype_tagged_thing,
                              atype_nonempty_nullterm_sequence_of,
                              atype_nullterm_sequence_of, atype_sequence,
                              atype_counted, atype_optional, atype_offset,
                              atype_ptr, atype_fn, atype_min, atype_info,
                              fn_info, ptr_info, offset_info, optional_info,
                              counted_info, cntype_info, cntype_type,
                              cntype_max, cntype_choice, cntype_seqof,
                              cntype_der, cntype_string, cntype_min,
                              tagged_info, immediate_info, string_info,
                              choice_info, seq_info};
use self::stdlib_h::{malloc, calloc, realloc, free, abort};
use self::stdio_h::snprintf;
use self::time_h::gmtime_r;
use self::string_h::{memcmp, memset, memcpy};
use self::assert_h::__assert_fail;
use self::k5_gmt_mktime_h::krb5int_gmt_mktime;
pub use self::k5_int_h::make_data;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/asn.1/asn1_encode.c */
/*
 * Copyright 1994, 2008 by the Massachusetts Institute of Technology.
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
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "29:8"]
pub struct asn1buf_st {
    pub ptr: *mut uint8_t,
    pub count: size_t,
}
/* Count of bytes written so far */
/* *** Functions for encoding primitive types ****/
/* Insert one byte into buf going backwards. */
#[inline]
#[c2rust::src_loc = "37:1"]
unsafe extern "C" fn insert_byte(mut buf: *mut asn1buf, mut o: uint8_t) {
    if !(*buf).ptr.is_null() {
        (*buf).ptr = (*buf).ptr.offset(-1);
        *(*buf).ptr = o
    }
    (*buf).count = (*buf).count.wrapping_add(1);
}
/* Insert a block of bytes into buf going backwards (but without reversing
 * bytes). */
#[inline]
#[c2rust::src_loc = "49:1"]
unsafe extern "C" fn insert_bytes(mut buf: *mut asn1buf,
                                  mut bytes: *const libc::c_void,
                                  mut len: size_t) {
    if !(*buf).ptr.is_null() {
        memcpy((*buf).ptr.offset(-(len as isize)) as *mut libc::c_void, bytes,
               len);
        (*buf).ptr = (*buf).ptr.offset(-(len as isize))
    }
    (*buf).count =
        ((*buf).count as libc::c_ulong).wrapping_add(len) as size_t as size_t;
}
#[no_mangle]
#[c2rust::src_loc = "59:1"]
pub unsafe extern "C" fn k5_asn1_encode_bool(mut buf: *mut asn1buf,
                                             mut val: intmax_t) {
    insert_byte(buf,
                if val != 0 { 0xff as libc::c_int } else { 0 as libc::c_int }
                    as uint8_t);
}
#[no_mangle]
#[c2rust::src_loc = "65:1"]
pub unsafe extern "C" fn k5_asn1_encode_int(mut buf: *mut asn1buf,
                                            mut val: intmax_t) {
    let mut valcopy: libc::c_long = 0;
    let mut digit: libc::c_int = 0;
    valcopy = val;
    loop  {
        digit =
            (valcopy & 0xff as libc::c_int as libc::c_long) as libc::c_int;
        insert_byte(buf, digit as uint8_t);
        valcopy = valcopy >> 8 as libc::c_int;
        if !(valcopy != 0 as libc::c_int as libc::c_long &&
                 valcopy != !(0 as libc::c_int) as libc::c_long) {
            break ;
        }
    }
    /* Make sure the high bit is of the proper signed-ness. */
    if val > 0 as libc::c_int as libc::c_long &&
           digit & 0x80 as libc::c_int == 0x80 as libc::c_int {
        insert_byte(buf, 0 as libc::c_int as uint8_t);
    } else if val < 0 as libc::c_int as libc::c_long &&
                  digit & 0x80 as libc::c_int != 0x80 as libc::c_int {
        insert_byte(buf, 0xff as libc::c_int as uint8_t);
    };
}
#[no_mangle]
#[c2rust::src_loc = "85:1"]
pub unsafe extern "C" fn k5_asn1_encode_uint(mut buf: *mut asn1buf,
                                             mut val: uintmax_t) {
    let mut valcopy: uintmax_t = 0;
    let mut digit: libc::c_int = 0;
    valcopy = val;
    loop  {
        digit =
            (valcopy & 0xff as libc::c_int as libc::c_ulong) as libc::c_int;
        insert_byte(buf, digit as uint8_t);
        valcopy = valcopy >> 8 as libc::c_int;
        if !(valcopy != 0 as libc::c_int as libc::c_ulong) { break ; }
    }
    /* Make sure the high bit is of the proper signed-ness. */
    if digit & 0x80 as libc::c_int != 0 {
        insert_byte(buf, 0 as libc::c_int as uint8_t);
    };
}
#[no_mangle]
#[c2rust::src_loc = "103:1"]
pub unsafe extern "C" fn k5_asn1_encode_bytestring(mut buf: *mut asn1buf,
                                                   mut val:
                                                       *const *mut uint8_t,
                                                   mut len: size_t)
 -> krb5_error_code {
    if len > 0 as libc::c_int as libc::c_ulong && val.is_null() {
        return 1859794433 as libc::c_long as krb5_error_code
    }
    insert_bytes(buf, *val as *const libc::c_void, len);
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "112:1"]
pub unsafe extern "C" fn k5_asn1_encode_generaltime(mut buf: *mut asn1buf,
                                                    mut val: time_t)
 -> krb5_error_code {
    let mut gtime: *mut tm = 0 as *mut tm;
    let mut gtimebuf: tm =
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
    let mut s: [libc::c_char; 16] = [0; 16];
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gmt_time: time_t = val;
    let mut len: libc::c_int = 0;
    /*
     * Time encoding: YYYYMMDDhhmmssZ
     */
    if gmt_time == 0 as libc::c_int as libc::c_long {
        sp =
            b"19700101000000Z\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else {
        /*
         * Sanity check this just to be paranoid, as gmtime can return NULL,
         * and some bogus implementations might overrun on the sprintf.
         */
        if gmtime_r(&mut gmt_time, &mut gtimebuf).is_null() {
            return 1859794442 as libc::c_long as krb5_error_code
        }
        /* HAVE_GMTIME_R */
        /* HAVE_GMTIME_R */
        gtime = &mut gtimebuf;
        if (*gtime).tm_year > 8099 as libc::c_int ||
               (*gtime).tm_mon > 11 as libc::c_int ||
               (*gtime).tm_mday > 31 as libc::c_int ||
               (*gtime).tm_hour > 23 as libc::c_int ||
               (*gtime).tm_min > 59 as libc::c_int ||
               (*gtime).tm_sec > 59 as libc::c_int {
            return 1859794442 as libc::c_long as krb5_error_code
        }
        len =
            snprintf(s.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 16]>() as
                         libc::c_ulong,
                     b"%04d%02d%02d%02d%02d%02dZ\x00" as *const u8 as
                         *const libc::c_char,
                     1900 as libc::c_int + (*gtime).tm_year,
                     (*gtime).tm_mon + 1 as libc::c_int, (*gtime).tm_mday,
                     (*gtime).tm_hour, (*gtime).tm_min, (*gtime).tm_sec);
        if len as libc::c_uint as libc::c_ulong >=
               ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong {
            /* Shouldn't be possible given above tests.  */
            return 1859794442 as libc::c_long as krb5_error_code
        }
        sp = s.as_mut_ptr()
    }
    insert_bytes(buf, sp as *const libc::c_void, 15 as libc::c_int as size_t);
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "164:1"]
pub unsafe extern "C" fn k5_asn1_encode_bitstring(mut buf: *mut asn1buf,
                                                  mut val:
                                                      *const *mut uint8_t,
                                                  mut len: size_t)
 -> krb5_error_code {
    insert_bytes(buf, *val as *const libc::c_void, len);
    insert_byte(buf, 0 as libc::c_int as uint8_t);
    return 0 as libc::c_int;
}
/* *** Functions for decoding primitive types ****/
#[no_mangle]
#[c2rust::src_loc = "174:1"]
pub unsafe extern "C" fn k5_asn1_decode_bool(mut asn1: *const uint8_t,
                                             mut len: size_t,
                                             mut val: *mut intmax_t)
 -> krb5_error_code {
    if len != 1 as libc::c_int as libc::c_ulong {
        return 1859794439 as libc::c_long as krb5_error_code
    }
    *val =
        (*asn1 as libc::c_int != 0 as libc::c_int) as libc::c_int as intmax_t;
    return 0 as libc::c_int;
}
/* Decode asn1/len as the contents of a DER integer, placing the signed result
 * in val. */
#[no_mangle]
#[c2rust::src_loc = "185:1"]
pub unsafe extern "C" fn k5_asn1_decode_int(mut asn1: *const uint8_t,
                                            mut len: size_t,
                                            mut val: *mut intmax_t)
 -> krb5_error_code {
    let mut n: intmax_t = 0;
    let mut i: size_t = 0;
    if len == 0 as libc::c_int as libc::c_ulong {
        return 1859794439 as libc::c_long as krb5_error_code
    }
    n =
        if *asn1.offset(0 as libc::c_int as isize) as libc::c_int &
               0x80 as libc::c_int != 0 {
            -(1 as libc::c_int)
        } else { 0 as libc::c_int } as intmax_t;
    /* Check length; allow extra octet if first octet is 0. */
    if len >
           (::std::mem::size_of::<intmax_t>() as
                libc::c_ulong).wrapping_add((*asn1.offset(0 as libc::c_int as
                                                              isize) as
                                                 libc::c_int ==
                                                 0 as libc::c_int) as
                                                libc::c_int as libc::c_ulong)
       {
        return 1859794436 as libc::c_long as krb5_error_code
    }
    i = 0 as libc::c_int as size_t;
    while i < len {
        n = n << 8 as libc::c_int | *asn1.offset(i as isize) as libc::c_long;
        i = i.wrapping_add(1)
    }
    *val = n;
    return 0 as libc::c_int;
}
/* Decode asn1/len as the contents of a DER integer, placing the unsigned
 * result in val. */
#[no_mangle]
#[c2rust::src_loc = "205:1"]
pub unsafe extern "C" fn k5_asn1_decode_uint(mut asn1: *const uint8_t,
                                             mut len: size_t,
                                             mut val: *mut uintmax_t)
 -> krb5_error_code {
    let mut n: uintmax_t = 0;
    let mut i: size_t = 0;
    if len == 0 as libc::c_int as libc::c_ulong {
        return 1859794439 as libc::c_long as krb5_error_code
    }
    /* Check for negative values and check length. */
    if *asn1.offset(0 as libc::c_int as isize) as libc::c_int &
           0x80 as libc::c_int != 0 ||
           len >
               (::std::mem::size_of::<uintmax_t>() as
                    libc::c_ulong).wrapping_add((*asn1.offset(0 as libc::c_int
                                                                  as isize) as
                                                     libc::c_int ==
                                                     0 as libc::c_int) as
                                                    libc::c_int as
                                                    libc::c_ulong) {
        return 1859794436 as libc::c_long as krb5_error_code
    }
    i = 0 as libc::c_int as size_t;
    n = 0 as libc::c_int as uintmax_t;
    while i < len {
        n = n << 8 as libc::c_int | *asn1.offset(i as isize) as libc::c_ulong;
        i = i.wrapping_add(1)
    }
    *val = n;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "222:1"]
pub unsafe extern "C" fn k5_asn1_decode_bytestring(mut asn1: *const uint8_t,
                                                   mut len: size_t,
                                                   mut str_out:
                                                       *mut *mut uint8_t,
                                                   mut len_out: *mut size_t)
 -> krb5_error_code {
    let mut str: *mut uint8_t = 0 as *mut uint8_t;
    *str_out = 0 as *mut uint8_t;
    *len_out = 0 as libc::c_int as size_t;
    if len == 0 as libc::c_int as libc::c_ulong { return 0 as libc::c_int }
    str = malloc(len) as *mut uint8_t;
    if str.is_null() { return 12 as libc::c_int }
    memcpy(str as *mut libc::c_void, asn1 as *const libc::c_void, len);
    *str_out = str;
    *len_out = len;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "241:1"]
pub unsafe extern "C" fn k5_asn1_decode_generaltime(mut asn1: *const uint8_t,
                                                    mut len: size_t,
                                                    mut time_out: *mut time_t)
 -> krb5_error_code {
    let mut s: *const libc::c_char = asn1 as *mut libc::c_char;
    let mut ts: tm =
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
    let mut t: time_t = 0;
    *time_out = 0 as libc::c_int as time_t;
    if len != 15 as libc::c_int as libc::c_ulong {
        return 1859794439 as libc::c_long as krb5_error_code
    }
    /* Time encoding: YYYYMMDDhhmmssZ */
    if *s.offset(14 as libc::c_int as isize) as libc::c_int != 'Z' as i32 {
        return 1859794440 as libc::c_long as krb5_error_code
    }
    if memcmp(s as *const libc::c_void,
              b"19700101000000Z\x00" as *const u8 as *const libc::c_char as
                  *const libc::c_void, 15 as libc::c_int as libc::c_ulong) ==
           0 as libc::c_int {
        *time_out = 0 as libc::c_int as time_t;
        return 0 as libc::c_int
    }
    ts.tm_year =
        1000 as libc::c_int *
            (*s.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32)
            +
            100 as libc::c_int *
                (*s.offset(1 as libc::c_int as isize) as libc::c_int -
                     '0' as i32) +
            10 as libc::c_int *
                (*s.offset(2 as libc::c_int as isize) as libc::c_int -
                     '0' as i32) +
            (*s.offset(3 as libc::c_int as isize) as libc::c_int - '0' as i32)
            - 1900 as libc::c_int;
    ts.tm_mon =
        10 as libc::c_int *
            (*s.offset(4 as libc::c_int as isize) as libc::c_int - '0' as i32)
            +
            (*s.offset(5 as libc::c_int as isize) as libc::c_int - '0' as i32)
            - 1 as libc::c_int;
    ts.tm_mday =
        10 as libc::c_int *
            (*s.offset(6 as libc::c_int as isize) as libc::c_int - '0' as i32)
            +
            (*s.offset(7 as libc::c_int as isize) as libc::c_int -
                 '0' as i32);
    ts.tm_hour =
        10 as libc::c_int *
            (*s.offset(8 as libc::c_int as isize) as libc::c_int - '0' as i32)
            +
            (*s.offset(9 as libc::c_int as isize) as libc::c_int -
                 '0' as i32);
    ts.tm_min =
        10 as libc::c_int *
            (*s.offset(10 as libc::c_int as isize) as libc::c_int -
                 '0' as i32) +
            (*s.offset(11 as libc::c_int as isize) as libc::c_int -
                 '0' as i32);
    ts.tm_sec =
        10 as libc::c_int *
            (*s.offset(12 as libc::c_int as isize) as libc::c_int -
                 '0' as i32) +
            (*s.offset(13 as libc::c_int as isize) as libc::c_int -
                 '0' as i32);
    ts.tm_isdst = -(1 as libc::c_int);
    t = krb5int_gmt_mktime(&mut ts);
    if t == -(1 as libc::c_int) as libc::c_long {
        return 1859794432 as libc::c_long as krb5_error_code
    }
    *time_out = t;
    return 0 as libc::c_int;
}
/*
 * Note: we return the number of bytes, not bits, in the bit string.  If the
 * number of bits is not a multiple of 8 we effectively round up to the next
 * multiple of 8.
 */
#[no_mangle]
#[c2rust::src_loc = "279:1"]
pub unsafe extern "C" fn k5_asn1_decode_bitstring(mut asn1: *const uint8_t,
                                                  mut len: size_t,
                                                  mut bits_out:
                                                      *mut *mut uint8_t,
                                                  mut len_out: *mut size_t)
 -> krb5_error_code {
    let mut unused: uint8_t = 0;
    let mut bits: *mut uint8_t = 0 as *mut uint8_t;
    *bits_out = 0 as *mut uint8_t;
    *len_out = 0 as libc::c_int as size_t;
    if len == 0 as libc::c_int as libc::c_ulong {
        return 1859794439 as libc::c_long as krb5_error_code
    }
    let fresh0 = asn1;
    asn1 = asn1.offset(1);
    unused = *fresh0;
    len = len.wrapping_sub(1);
    if unused as libc::c_int > 7 as libc::c_int {
        return 1859794440 as libc::c_long as krb5_error_code
    }
    bits = malloc(len) as *mut uint8_t;
    if bits.is_null() { return 12 as libc::c_int }
    memcpy(bits as *mut libc::c_void, asn1 as *const libc::c_void, len);
    if len > 1 as libc::c_int as libc::c_ulong {
        let ref mut fresh1 =
            *bits.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                             as isize);
        *fresh1 =
            (*fresh1 as libc::c_int &
                 (0xff as libc::c_int) << unused as libc::c_int) as uint8_t
    }
    *bits_out = bits;
    *len_out = len;
    return 0 as libc::c_int;
}
/* *** Functions for encoding and decoding tags ****/
/* Encode a DER tag into buf with the tag parameters in t and the content
 * length len.  Place the length of the encoded tag in *retlen. */
#[c2rust::src_loc = "310:1"]
unsafe extern "C" fn make_tag(mut buf: *mut asn1buf, mut t: *const taginfo,
                              mut len: size_t) -> krb5_error_code {
    let mut tag_copy: asn1_tagnum = 0;
    let mut len_copy: size_t = 0;
    let mut oldcount: size_t = 0;
    if (*t).tagnum > 2147483647 as libc::c_int - 1 as libc::c_int {
        return 1859794436 as libc::c_long as krb5_error_code
    }
    /* Encode the length of the content within the tag. */
    if len < 128 as libc::c_int as libc::c_ulong {
        insert_byte(buf,
                    (len & 0x7f as libc::c_int as libc::c_ulong) as uint8_t);
    } else {
        oldcount = (*buf).count;
        len_copy = len;
        while len_copy != 0 as libc::c_int as libc::c_ulong {
            insert_byte(buf,
                        (len_copy & 0xff as libc::c_int as libc::c_ulong) as
                            uint8_t);
            len_copy >>= 8 as libc::c_int
        }
        insert_byte(buf,
                    (0x80 as libc::c_int as libc::c_ulong |
                         (*buf).count.wrapping_sub(oldcount) &
                             0x7f as libc::c_int as libc::c_ulong) as
                        uint8_t);
    }
    /* Encode the tag and construction bit. */
    if (*t).tagnum < 31 as libc::c_int {
        insert_byte(buf,
                    ((*t).asn1class as libc::c_uint |
                         (*t).construction as libc::c_uint |
                         (*t).tagnum as libc::c_uint) as uint8_t);
    } else {
        tag_copy = (*t).tagnum;
        insert_byte(buf, (tag_copy & 0x7f as libc::c_int) as uint8_t);
        tag_copy >>= 7 as libc::c_int;
        while tag_copy != 0 as libc::c_int {
            insert_byte(buf,
                        (0x80 as libc::c_int | tag_copy & 0x7f as libc::c_int)
                            as uint8_t);
            tag_copy >>= 7 as libc::c_int
        }
        insert_byte(buf,
                    ((*t).asn1class as libc::c_uint |
                         (*t).construction as libc::c_uint |
                         0x1f as libc::c_int as libc::c_uint) as uint8_t);
    }
    return 0 as libc::c_int;
}
/*
 * Read a BER tag and length from asn1/len.  Place the tag parameters in
 * tag_out.  Set contents_out/clen_out to the octet range of the tag's
 * contents, and remainder_out/rlen_out to the octet range after the end of the
 * BER encoding.
 *
 * (krb5 ASN.1 encodings should be in DER, but for compatibility with some
 * really ancient implementations we handle the indefinite length form in tags.
 * However, we still insist on the primitive form of string types.)
 */
#[c2rust::src_loc = "356:1"]
unsafe extern "C" fn get_tag(mut asn1: *const uint8_t, mut len: size_t,
                             mut tag_out: *mut taginfo,
                             mut contents_out: *mut *const uint8_t,
                             mut clen_out: *mut size_t,
                             mut remainder_out: *mut *const uint8_t,
                             mut rlen_out: *mut size_t) -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut o: uint8_t = 0;
    let mut c: *const uint8_t = 0 as *const uint8_t;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut tag_start: *const uint8_t = asn1;
    let mut clen: size_t = 0;
    let mut llen: size_t = 0;
    let mut i: size_t = 0;
    let mut t: taginfo =
        taginfo{asn1class: UNIVERSAL,
                construction: PRIMITIVE,
                tagnum: 0,
                tag_len: 0,
                tag_end_len: 0,};
    *remainder_out = 0 as *const uint8_t;
    *contents_out = *remainder_out;
    *rlen_out = 0 as libc::c_int as size_t;
    *clen_out = *rlen_out;
    if len == 0 as libc::c_int as libc::c_ulong {
        return 1859794437 as libc::c_long as krb5_error_code
    }
    let fresh2 = asn1;
    asn1 = asn1.offset(1);
    o = *fresh2;
    len = len.wrapping_sub(1);
    (*tag_out).asn1class =
        (o as libc::c_int & 0xc0 as libc::c_int) as asn1_class;
    (*tag_out).construction =
        (o as libc::c_int & 0x20 as libc::c_int) as asn1_construction;
    if o as libc::c_int & 0x1f as libc::c_int != 0x1f as libc::c_int {
        (*tag_out).tagnum = o as libc::c_int & 0x1f as libc::c_int
    } else {
        (*tag_out).tagnum = 0 as libc::c_int;
        loop  {
            if len == 0 as libc::c_int as libc::c_ulong {
                return 1859794437 as libc::c_long as krb5_error_code
            }
            let fresh3 = asn1;
            asn1 = asn1.offset(1);
            o = *fresh3;
            len = len.wrapping_sub(1);
            (*tag_out).tagnum =
                (*tag_out).tagnum << 7 as libc::c_int |
                    o as libc::c_int & 0x7f as libc::c_int;
            if !(o as libc::c_int & 0x80 as libc::c_int != 0) { break ; }
        }
    }
    if len == 0 as libc::c_int as libc::c_ulong {
        return 1859794437 as libc::c_long as krb5_error_code
    }
    let fresh4 = asn1;
    asn1 = asn1.offset(1);
    o = *fresh4;
    len = len.wrapping_sub(1);
    if o as libc::c_int == 0x80 as libc::c_int {
        /* Indefinite form (should not be present in DER, but we accept it). */
        if (*tag_out).construction as libc::c_uint !=
               CONSTRUCTED as libc::c_int as libc::c_uint {
            return 1859794443 as libc::c_long as krb5_error_code
        }
        p = asn1;
        while !(len >= 2 as libc::c_int as libc::c_ulong &&
                    *p.offset(0 as libc::c_int as isize) as libc::c_int ==
                        0 as libc::c_int &&
                    *p.offset(1 as libc::c_int as isize) as libc::c_int ==
                        0 as libc::c_int) {
            ret =
                get_tag(p, len, &mut t, &mut c, &mut clen, &mut p, &mut len);
            if ret != 0 { return ret }
        }
        (*tag_out).tag_end_len = 2 as libc::c_int as size_t;
        *contents_out = asn1;
        *clen_out = p.wrapping_offset_from(asn1) as libc::c_long as size_t;
        *remainder_out = p.offset(2 as libc::c_int as isize);
        *rlen_out = len.wrapping_sub(2 as libc::c_int as libc::c_ulong)
    } else if o as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
        /* Short form (first octet gives content length). */
        if o as libc::c_ulong > len {
            return 1859794437 as libc::c_long as krb5_error_code
        }
        (*tag_out).tag_end_len = 0 as libc::c_int as size_t;
        *contents_out = asn1;
        *clen_out = o as size_t;
        *remainder_out = asn1.offset(*clen_out as isize);
        *rlen_out =
            len.wrapping_sub((*remainder_out).wrapping_offset_from(asn1) as
                                 libc::c_long as libc::c_ulong)
    } else {
        /* Long form (first octet gives number of base-256 length octets). */
        llen = (o as libc::c_int & 0x7f as libc::c_int) as size_t;
        if llen > len { return 1859794437 as libc::c_long as krb5_error_code }
        if llen > ::std::mem::size_of::<size_t>() as libc::c_ulong {
            return 1859794436 as libc::c_long as krb5_error_code
        }
        i = 0 as libc::c_int as size_t;
        clen = 0 as libc::c_int as size_t;
        while i < llen {
            clen =
                clen << 8 as libc::c_int |
                    *asn1.offset(i as isize) as libc::c_ulong;
            i = i.wrapping_add(1)
        }
        if clen > len.wrapping_sub(llen) {
            return 1859794437 as libc::c_long as krb5_error_code
        }
        (*tag_out).tag_end_len = 0 as libc::c_int as size_t;
        *contents_out = asn1.offset(llen as isize);
        *clen_out = clen;
        *remainder_out = (*contents_out).offset(clen as isize);
        *rlen_out =
            len.wrapping_sub((*remainder_out).wrapping_offset_from(asn1) as
                                 libc::c_long as libc::c_ulong)
    }
    (*tag_out).tag_len =
        (*contents_out).wrapping_offset_from(tag_start) as libc::c_long as
            size_t;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "448:1"]
unsafe extern "C" fn get_nullterm_sequence_len(mut valp: *const libc::c_void,
                                               mut seq: *const atype_info)
 -> size_t {
    let mut i: size_t = 0;
    let mut a: *const atype_info = 0 as *const atype_info;
    let mut ptr: *const ptr_info = 0 as *const ptr_info;
    let mut elt: *const libc::c_void = 0 as *const libc::c_void;
    let mut eltptr: *const libc::c_void = 0 as *const libc::c_void;
    a = seq;
    i = 0 as libc::c_int as size_t;
    if (*a).type_0 as libc::c_uint == atype_ptr as libc::c_int as libc::c_uint
       {
    } else {
        __assert_fail(b"a->type == atype_ptr\x00" as *const u8 as
                          *const libc::c_char,
                      b"asn1_encode.c\x00" as *const u8 as
                          *const libc::c_char,
                      458 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 74],
                                                &[libc::c_char; 74]>(b"size_t get_nullterm_sequence_len(const void *, const struct atype_info *)\x00")).as_ptr());
    }
    if (*seq).size != 0 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"seq->size != 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"asn1_encode.c\x00" as *const u8 as
                          *const libc::c_char,
                      459 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 74],
                                                &[libc::c_char; 74]>(b"size_t get_nullterm_sequence_len(const void *, const struct atype_info *)\x00")).as_ptr());
    }
    ptr = (*a).tinfo as *const ptr_info;
    loop  {
        eltptr =
            (valp as
                 *const libc::c_char).offset(i.wrapping_mul((*seq).size) as
                                                 isize) as
                *const libc::c_void;
        if (*ptr).loadptr.is_some() {
        } else {
            __assert_fail(b"(ptr)->loadptr != NULL\x00" as *const u8 as
                              *const libc::c_char,
                          b"asn1_encode.c\x00" as *const u8 as
                              *const libc::c_char,
                          464 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 74],
                                                    &[libc::c_char; 74]>(b"size_t get_nullterm_sequence_len(const void *, const struct atype_info *)\x00")).as_ptr());
        }
        elt = (*ptr).loadptr.expect("non-null function pointer")(eltptr);
        if elt.is_null() { break ; }
        i = i.wrapping_add(1)
    }
    return i;
}
#[c2rust::src_loc = "475:1"]
unsafe extern "C" fn encode_nullterm_sequence_of(mut buf: *mut asn1buf,
                                                 mut val: *const libc::c_void,
                                                 mut type_0:
                                                     *const atype_info,
                                                 mut can_be_empty:
                                                     libc::c_int)
 -> krb5_error_code {
    let mut len: size_t = get_nullterm_sequence_len(val, type_0);
    if can_be_empty == 0 && len == 0 as libc::c_int as libc::c_ulong {
        return 1859794433 as libc::c_long as krb5_error_code
    }
    return encode_sequence_of(buf, len, val, type_0);
}
#[c2rust::src_loc = "486:1"]
unsafe extern "C" fn load_int(mut val: *const libc::c_void, mut size: size_t)
 -> intmax_t {
    match size {
        1 => { return *(val as *mut int8_t) as intmax_t }
        2 => { return *(val as *mut int16_t) as intmax_t }
        4 => { return *(val as *mut int32_t) as intmax_t }
        8 => { return *(val as *mut int64_t) }
        _ => { abort(); }
    };
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn load_uint(mut val: *const libc::c_void, mut size: size_t)
 -> uintmax_t {
    match size {
        1 => { return *(val as *mut uint8_t) as uintmax_t }
        2 => { return *(val as *mut uint16_t) as uintmax_t }
        4 => { return *(val as *mut uint32_t) as uintmax_t }
        8 => { return *(val as *mut uint64_t) }
        _ => { abort(); }
    };
}
#[c2rust::src_loc = "510:1"]
unsafe extern "C" fn load_count(mut val: *const libc::c_void,
                                mut counted: *const counted_info,
                                mut count_out: *mut size_t)
 -> krb5_error_code {
    let mut countptr: *const libc::c_void =
        (val as
             *const libc::c_char).offset((*counted).lenoff() as libc::c_int as
                                             isize) as *const libc::c_void;
    if ::std::mem::size_of::<size_t>() as libc::c_ulong <=
           ::std::mem::size_of::<uintmax_t>() as libc::c_ulong {
    } else {
        __assert_fail(b"sizeof(size_t) <= sizeof(uintmax_t)\x00" as *const u8
                          as *const libc::c_char,
                      b"asn1_encode.c\x00" as *const u8 as
                          *const libc::c_char,
                      516 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 80],
                                                &[libc::c_char; 80]>(b"krb5_error_code load_count(const void *, const struct counted_info *, size_t *)\x00")).as_ptr());
    }
    if (*counted).lensigned() != 0 {
        let mut xlen: intmax_t =
            load_int(countptr, (*counted).lensize() as size_t);
        if xlen < 0 as libc::c_int as libc::c_long ||
               xlen as uintmax_t > 18446744073709551615 as libc::c_ulong {
            return 22 as libc::c_int
        }
        *count_out = xlen as size_t
    } else {
        let mut xlen_0: uintmax_t =
            load_uint(countptr, (*counted).lensize() as size_t);
        if xlen_0 != xlen_0 || xlen_0 > 18446744073709551615 as libc::c_ulong
           {
            return 22 as libc::c_int
        }
        *count_out = xlen_0
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "531:1"]
unsafe extern "C" fn store_int(mut intval: intmax_t, mut size: size_t,
                               mut val: *mut libc::c_void)
 -> krb5_error_code {
    match size {
        1 => {
            if intval as int8_t as libc::c_long != intval {
                return 1859794436 as libc::c_long as krb5_error_code
            }
            *(val as *mut int8_t) = intval as int8_t;
            return 0 as libc::c_int
        }
        2 => {
            if intval as int16_t as libc::c_long != intval {
                return 1859794436 as libc::c_long as krb5_error_code
            }
            *(val as *mut int16_t) = intval as int16_t;
            return 0 as libc::c_int
        }
        4 => {
            if intval as int32_t as libc::c_long != intval {
                return 1859794436 as libc::c_long as krb5_error_code
            }
            *(val as *mut int32_t) = intval as int32_t;
            return 0 as libc::c_int
        }
        8 => {
            if intval != intval {
                return 1859794436 as libc::c_long as krb5_error_code
            }
            *(val as *mut int64_t) = intval;
            return 0 as libc::c_int
        }
        _ => { abort(); }
    };
}
#[c2rust::src_loc = "560:1"]
unsafe extern "C" fn store_uint(mut intval: uintmax_t, mut size: size_t,
                                mut val: *mut libc::c_void)
 -> krb5_error_code {
    match size {
        1 => {
            if intval as uint8_t as libc::c_ulong != intval {
                return 1859794436 as libc::c_long as krb5_error_code
            }
            *(val as *mut uint8_t) = intval as uint8_t;
            return 0 as libc::c_int
        }
        2 => {
            if intval as uint16_t as libc::c_ulong != intval {
                return 1859794436 as libc::c_long as krb5_error_code
            }
            *(val as *mut uint16_t) = intval as uint16_t;
            return 0 as libc::c_int
        }
        4 => {
            if intval as uint32_t as libc::c_ulong != intval {
                return 1859794436 as libc::c_long as krb5_error_code
            }
            *(val as *mut uint32_t) = intval as uint32_t;
            return 0 as libc::c_int
        }
        8 => {
            if intval != intval {
                return 1859794436 as libc::c_long as krb5_error_code
            }
            *(val as *mut uint64_t) = intval;
            return 0 as libc::c_int
        }
        _ => { abort(); }
    };
}
/* Store a count value in an integer field of a structure.  If count is
 * SIZE_MAX and the target is a signed field, store -1. */
#[c2rust::src_loc = "591:1"]
unsafe extern "C" fn store_count(mut count: size_t,
                                 mut counted: *const counted_info,
                                 mut val: *mut libc::c_void)
 -> krb5_error_code {
    let mut countptr: *mut libc::c_void =
        (val as
             *mut libc::c_char).offset((*counted).lenoff() as libc::c_int as
                                           isize) as *mut libc::c_void;
    if (*counted).lensigned() != 0 {
        if count == 18446744073709551615 as libc::c_ulong {
            return store_int(-(1 as libc::c_int) as intmax_t,
                             (*counted).lensize() as size_t, countptr)
        } else if (count as intmax_t) < 0 as libc::c_int as libc::c_long {
            return 1859794436 as libc::c_long as krb5_error_code
        } else {
            return store_int(count as intmax_t,
                             (*counted).lensize() as size_t, countptr)
        }
    } else {
        return store_uint(count, (*counted).lensize() as size_t, countptr)
    };
}
/* Split a DER encoding into tag and contents.  Insert the contents into buf,
 * then return the length of the contents and the tag. */
#[c2rust::src_loc = "609:1"]
unsafe extern "C" fn split_der(mut buf: *mut asn1buf,
                               mut der: *const *mut uint8_t, mut len: size_t,
                               mut tag_out: *mut taginfo) -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut contents: *const uint8_t = 0 as *const uint8_t;
    let mut remainder: *const uint8_t = 0 as *const uint8_t;
    let mut clen: size_t = 0;
    let mut rlen: size_t = 0;
    ret =
        get_tag(*der, len, tag_out, &mut contents, &mut clen, &mut remainder,
                &mut rlen);
    if ret != 0 { return ret }
    if rlen != 0 as libc::c_int as libc::c_ulong {
        return 1859794439 as libc::c_long as krb5_error_code
    }
    insert_bytes(buf, contents as *const libc::c_void, clen);
    return 0 as libc::c_int;
}
/*
 * Store the DER encoding given by t and asn1/len into the char * or
 * uint8_t * pointed to by val.  Set *count_out to the length of the
 * DER encoding.
 */
#[c2rust::src_loc = "630:1"]
unsafe extern "C" fn store_der(mut t: *const taginfo,
                               mut asn1: *const uint8_t, mut len: size_t,
                               mut val: *mut libc::c_void,
                               mut count_out: *mut size_t)
 -> krb5_error_code {
    let mut der: *mut uint8_t = 0 as *mut uint8_t;
    let mut der_len: size_t = 0;
    *count_out = 0 as libc::c_int as size_t;
    der_len = (*t).tag_len.wrapping_add(len).wrapping_add((*t).tag_end_len);
    der = malloc(der_len) as *mut uint8_t;
    if der.is_null() { return 12 as libc::c_int }
    memcpy(der as *mut libc::c_void,
           asn1.offset(-((*t).tag_len as isize)) as *const libc::c_void,
           der_len);
    let ref mut fresh5 = *(val as *mut *mut uint8_t);
    *fresh5 = der;
    *count_out = der_len;
    return 0 as libc::c_int;
}
/* Encode a value (contents only, no outer tag) according to a type, and return
 * its encoded tag information. */
#[c2rust::src_loc = "656:1"]
unsafe extern "C" fn encode_atype(mut buf: *mut asn1buf,
                                  mut val: *const libc::c_void,
                                  mut a: *const atype_info,
                                  mut tag_out: *mut taginfo)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    if val.is_null() { return 1859794433 as libc::c_long as krb5_error_code }
    match (*a).type_0 as libc::c_uint {
        2 => {
            let mut fn_0: *const fn_info = (*a).tinfo as *const fn_info;
            if (*fn_0).enc.is_some() {
            } else {
                __assert_fail(b"fn->enc != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              668 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 92],
                                                        &[libc::c_char; 92]>(b"krb5_error_code encode_atype(asn1buf *, const void *, const struct atype_info *, taginfo *)\x00")).as_ptr());
            }
            return (*fn_0).enc.expect("non-null function pointer")(buf, val,
                                                                   tag_out)
        }
        7 => {
            if !(*a).tinfo.is_null() {
            } else {
                __assert_fail(b"a->tinfo != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              672 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 92],
                                                        &[libc::c_char; 92]>(b"krb5_error_code encode_atype(asn1buf *, const void *, const struct atype_info *, taginfo *)\x00")).as_ptr());
            }
            ret = encode_sequence(buf, val, (*a).tinfo as *const seq_info);
            if ret != 0 { return ret }
            (*tag_out).asn1class = UNIVERSAL;
            (*tag_out).construction = CONSTRUCTED;
            (*tag_out).tagnum = 16 as libc::c_int
        }
        3 => {
            let mut ptr: *const ptr_info = (*a).tinfo as *const ptr_info;
            if !(*ptr).basetype.is_null() {
            } else {
                __assert_fail(b"ptr->basetype != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              682 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 92],
                                                        &[libc::c_char; 92]>(b"krb5_error_code encode_atype(asn1buf *, const void *, const struct atype_info *, taginfo *)\x00")).as_ptr());
            }
            if (*ptr).loadptr.is_some() {
            } else {
                __assert_fail(b"(ptr)->loadptr != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              683 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 92],
                                                        &[libc::c_char; 92]>(b"krb5_error_code encode_atype(asn1buf *, const void *, const struct atype_info *, taginfo *)\x00")).as_ptr());
            }
            return encode_atype(buf,
                                (*ptr).loadptr.expect("non-null function pointer")(val),
                                (*ptr).basetype, tag_out)
        }
        4 => {
            let mut off: *const offset_info =
                (*a).tinfo as *const offset_info;
            if !(*off).basetype.is_null() {
            } else {
                __assert_fail(b"off->basetype != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              687 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 92],
                                                        &[libc::c_char; 92]>(b"krb5_error_code encode_atype(asn1buf *, const void *, const struct atype_info *, taginfo *)\x00")).as_ptr());
            }
            return encode_atype(buf,
                                (val as
                                     *const libc::c_char).offset((*off).dataoff()
                                                                     as
                                                                     libc::c_int
                                                                     as isize)
                                    as *const libc::c_void, (*off).basetype,
                                tag_out)
        }
        5 => {
            let mut opt: *const optional_info =
                (*a).tinfo as *const optional_info;
            if (*opt).is_present.is_some() {
            } else {
                __assert_fail(b"opt->is_present != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              693 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 92],
                                                        &[libc::c_char; 92]>(b"krb5_error_code encode_atype(asn1buf *, const void *, const struct atype_info *, taginfo *)\x00")).as_ptr());
            }
            if (*opt).is_present.expect("non-null function pointer")(val) != 0
               {
                return encode_atype(buf, val, (*opt).basetype, tag_out)
            } else { return 1859794445 as libc::c_long as krb5_error_code }
        }
        6 => {
            let mut counted: *const counted_info =
                (*a).tinfo as *const counted_info;
            let mut dataptr: *const libc::c_void =
                (val as
                     *const libc::c_char).offset((*counted).dataoff() as
                                                     libc::c_int as isize) as
                    *const libc::c_void;
            let mut count: size_t = 0;
            if !(*counted).basetype.is_null() {
            } else {
                __assert_fail(b"counted->basetype != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              703 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 92],
                                                        &[libc::c_char; 92]>(b"krb5_error_code encode_atype(asn1buf *, const void *, const struct atype_info *, taginfo *)\x00")).as_ptr());
            }
            ret = load_count(val, counted, &mut count);
            if ret != 0 { return ret }
            return encode_cntype(buf, dataptr, count, (*counted).basetype,
                                 tag_out)
        }
        8 | 9 => {
            if !(*a).tinfo.is_null() {
            } else {
                __assert_fail(b"a->tinfo != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              711 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 92],
                                                        &[libc::c_char; 92]>(b"krb5_error_code encode_atype(asn1buf *, const void *, const struct atype_info *, taginfo *)\x00")).as_ptr());
            }
            ret =
                encode_nullterm_sequence_of(buf, val,
                                            (*a).tinfo as *const atype_info,
                                            ((*a).type_0 as libc::c_uint ==
                                                 atype_nullterm_sequence_of as
                                                     libc::c_int as
                                                     libc::c_uint) as
                                                libc::c_int);
            if ret != 0 { return ret }
            (*tag_out).asn1class = UNIVERSAL;
            (*tag_out).construction = CONSTRUCTED;
            (*tag_out).tagnum = 16 as libc::c_int
        }
        10 => {
            let mut tag: *const tagged_info =
                (*a).tinfo as *const tagged_info;
            let mut oldcount: size_t = (*buf).count;
            ret = encode_atype(buf, val, (*tag).basetype, tag_out);
            if ret != 0 { return ret }
            if (*tag).implicit() == 0 {
                ret =
                    make_tag(buf, tag_out,
                             (*buf).count.wrapping_sub(oldcount));
                if ret != 0 { return ret }
                (*tag_out).construction =
                    (*tag).construction() as asn1_construction
            }
            (*tag_out).asn1class = (*tag).tagtype() as asn1_class;
            (*tag_out).tagnum = (*tag).tagval() as asn1_tagnum
        }
        11 => {
            k5_asn1_encode_bool(buf, load_int(val, (*a).size));
            (*tag_out).asn1class = UNIVERSAL;
            (*tag_out).construction = PRIMITIVE;
            (*tag_out).tagnum = 1 as libc::c_int
        }
        12 => {
            k5_asn1_encode_int(buf, load_int(val, (*a).size));
            (*tag_out).asn1class = UNIVERSAL;
            (*tag_out).construction = PRIMITIVE;
            (*tag_out).tagnum = 2 as libc::c_int
        }
        13 => {
            k5_asn1_encode_uint(buf, load_uint(val, (*a).size));
            (*tag_out).asn1class = UNIVERSAL;
            (*tag_out).construction = PRIMITIVE;
            (*tag_out).tagnum = 2 as libc::c_int
        }
        14 => {
            let mut imm: *const immediate_info =
                (*a).tinfo as *const immediate_info;
            k5_asn1_encode_int(buf, (*imm).val);
            (*tag_out).asn1class = UNIVERSAL;
            (*tag_out).construction = PRIMITIVE;
            (*tag_out).tagnum = 2 as libc::c_int
        }
        _ => {
            if (*a).type_0 as libc::c_uint >
                   atype_min as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(b"a->type > atype_min\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              764 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 92],
                                                        &[libc::c_char; 92]>(b"krb5_error_code encode_atype(asn1buf *, const void *, const struct atype_info *, taginfo *)\x00")).as_ptr());
            }
            if ((*a).type_0 as libc::c_uint) <
                   atype_max as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(b"a->type < atype_max\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              765 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 92],
                                                        &[libc::c_char; 92]>(b"krb5_error_code encode_atype(asn1buf *, const void *, const struct atype_info *, taginfo *)\x00")).as_ptr());
            }
            abort();
        }
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "772:1"]
unsafe extern "C" fn encode_atype_and_tag(mut buf: *mut asn1buf,
                                          mut val: *const libc::c_void,
                                          mut a: *const atype_info)
 -> krb5_error_code {
    let mut t: taginfo =
        taginfo{asn1class: UNIVERSAL,
                construction: PRIMITIVE,
                tagnum: 0,
                tag_len: 0,
                tag_end_len: 0,};
    let mut ret: krb5_error_code = 0;
    let mut oldcount: size_t = (*buf).count;
    ret = encode_atype(buf, val, a, &mut t);
    if ret != 0 { return ret }
    ret = make_tag(buf, &mut t, (*buf).count.wrapping_sub(oldcount));
    if ret != 0 { return ret }
    return 0 as libc::c_int;
}
/*
 * Encode an object and count according to a cntype_info structure.  val is a
 * pointer to the object being encoded, which in most cases is itself a
 * pointer (but is a union in the cntype_choice case).
 */
#[c2rust::src_loc = "793:1"]
unsafe extern "C" fn encode_cntype(mut buf: *mut asn1buf,
                                   mut val: *const libc::c_void,
                                   mut count: size_t,
                                   mut c: *const cntype_info,
                                   mut tag_out: *mut taginfo)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    match (*c).type_0 as libc::c_uint {
        2 => {
            let mut string: *const string_info =
                (*c).tinfo as *const string_info;
            if (*string).enc.is_some() {
            } else {
                __assert_fail(b"string->enc != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              802 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 102],
                                                        &[libc::c_char; 102]>(b"krb5_error_code encode_cntype(asn1buf *, const void *, size_t, const struct cntype_info *, taginfo *)\x00")).as_ptr());
            }
            ret =
                (*string).enc.expect("non-null function pointer")(buf,
                                                                  val as
                                                                      *const *mut uint8_t,
                                                                  count);
            if ret != 0 { return ret }
            (*tag_out).asn1class = UNIVERSAL;
            (*tag_out).construction = PRIMITIVE;
            (*tag_out).tagnum = (*string).tagval() as asn1_tagnum
        }
        3 => {
            return split_der(buf, val as *const *mut uint8_t, count, tag_out)
        }
        4 => {
            let mut a: *const atype_info = (*c).tinfo as *const atype_info;
            let mut ptr: *const ptr_info = (*a).tinfo as *const ptr_info;
            if (*a).type_0 as libc::c_uint ==
                   atype_ptr as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(b"a->type == atype_ptr\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              816 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 102],
                                                        &[libc::c_char; 102]>(b"krb5_error_code encode_cntype(asn1buf *, const void *, size_t, const struct cntype_info *, taginfo *)\x00")).as_ptr());
            }
            if (*ptr).loadptr.is_some() {
            } else {
                __assert_fail(b"(ptr)->loadptr != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              817 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 102],
                                                        &[libc::c_char; 102]>(b"krb5_error_code encode_cntype(asn1buf *, const void *, size_t, const struct cntype_info *, taginfo *)\x00")).as_ptr());
            }
            val = (*ptr).loadptr.expect("non-null function pointer")(val);
            ret = encode_sequence_of(buf, count, val, (*ptr).basetype);
            if ret != 0 { return ret }
            (*tag_out).asn1class = UNIVERSAL;
            (*tag_out).construction = CONSTRUCTED;
            (*tag_out).tagnum = 16 as libc::c_int
        }
        5 => {
            let mut choice: *const choice_info =
                (*c).tinfo as *const choice_info;
            if count >= (*choice).n_options {
                return 1859794433 as libc::c_long as krb5_error_code
            }
            return encode_atype(buf, val,
                                *(*choice).options.offset(count as isize),
                                tag_out)
        }
        _ => {
            if (*c).type_0 as libc::c_uint >
                   cntype_min as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(b"c->type > cntype_min\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              834 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 102],
                                                        &[libc::c_char; 102]>(b"krb5_error_code encode_cntype(asn1buf *, const void *, size_t, const struct cntype_info *, taginfo *)\x00")).as_ptr());
            }
            if ((*c).type_0 as libc::c_uint) <
                   cntype_max as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(b"c->type < cntype_max\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              835 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 102],
                                                        &[libc::c_char; 102]>(b"krb5_error_code encode_cntype(asn1buf *, const void *, size_t, const struct cntype_info *, taginfo *)\x00")).as_ptr());
            }
            abort();
        }
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "842:1"]
unsafe extern "C" fn encode_sequence(mut buf: *mut asn1buf,
                                     mut val: *const libc::c_void,
                                     mut seq: *const seq_info)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut i: size_t = 0;
    i = (*seq).n_fields;
    while i > 0 as libc::c_int as libc::c_ulong {
        ret =
            encode_atype_and_tag(buf, val,
                                 *(*seq).fields.offset(i.wrapping_sub(1 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_ulong)
                                                           as isize));
        if !(ret as libc::c_long == 1859794445 as libc::c_long) {
            if ret != 0 as libc::c_int { return ret }
        }
        i = i.wrapping_sub(1)
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "858:1"]
unsafe extern "C" fn encode_sequence_of(mut buf: *mut asn1buf,
                                        mut seqlen: size_t,
                                        mut val: *const libc::c_void,
                                        mut eltinfo: *const atype_info)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut i: size_t = 0;
    let mut eltptr: *const libc::c_void = 0 as *const libc::c_void;
    if (*eltinfo).size != 0 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"eltinfo->size != 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"asn1_encode.c\x00" as *const u8 as
                          *const libc::c_char,
                      866 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 95],
                                                &[libc::c_char; 95]>(b"krb5_error_code encode_sequence_of(asn1buf *, size_t, const void *, const struct atype_info *)\x00")).as_ptr());
    }
    i = seqlen;
    while i > 0 as libc::c_int as libc::c_ulong {
        eltptr =
            (val as
                 *const libc::c_char).offset(i.wrapping_sub(1 as libc::c_int
                                                                as
                                                                libc::c_ulong).wrapping_mul((*eltinfo).size)
                                                 as isize) as
                *const libc::c_void;
        ret = encode_atype_and_tag(buf, eltptr, eltinfo);
        if ret != 0 { return ret }
        i = i.wrapping_sub(1)
    }
    return 0 as libc::c_int;
}
/*
 * Free a C object according to a type description.  Do not free pointers at
 * the first level; they may be referenced by other fields of a sequence, and
 * will be freed by free_atype_ptr in a second pass.
 */
#[c2rust::src_loc = "889:1"]
unsafe extern "C" fn free_atype(mut a: *const atype_info,
                                mut val: *mut libc::c_void) {
    match (*a).type_0 as libc::c_uint {
        2 => {
            let mut fn_0: *const fn_info = (*a).tinfo as *const fn_info;
            if (*fn_0).free_func.is_some() {
                (*fn_0).free_func.expect("non-null function pointer")(val);
            }
        }
        7 => { free_sequence((*a).tinfo as *const seq_info, val); }
        3 => {
            let mut ptrinfo: *const ptr_info = (*a).tinfo as *const ptr_info;
            if (*ptrinfo).loadptr.is_some() {
            } else {
                __assert_fail(b"(ptrinfo)->loadptr != NULL\x00" as *const u8
                                  as *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              904 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 51],
                                                        &[libc::c_char; 51]>(b"void free_atype(const struct atype_info *, void *)\x00")).as_ptr());
            }
            let mut ptr: *mut libc::c_void =
                (*ptrinfo).loadptr.expect("non-null function pointer")(val);
            if !ptr.is_null() {
                free_atype((*ptrinfo).basetype, ptr);
                free_atype_ptr((*ptrinfo).basetype, ptr);
            }
        }
        4 => {
            let mut off: *const offset_info =
                (*a).tinfo as *const offset_info;
            if !(*off).basetype.is_null() {
            } else {
                __assert_fail(b"off->basetype != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              913 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 51],
                                                        &[libc::c_char; 51]>(b"void free_atype(const struct atype_info *, void *)\x00")).as_ptr());
            }
            free_atype((*off).basetype,
                       (val as
                            *mut libc::c_char).offset((*off).dataoff() as
                                                          libc::c_int as
                                                          isize) as
                           *mut libc::c_void);
        }
        5 => {
            let mut opt: *const optional_info =
                (*a).tinfo as *const optional_info;
            free_atype((*opt).basetype, val);
        }
        6 => {
            let mut counted: *const counted_info =
                (*a).tinfo as *const counted_info;
            let mut dataptr: *mut libc::c_void =
                (val as
                     *mut libc::c_char).offset((*counted).dataoff() as
                                                   libc::c_int as isize) as
                    *mut libc::c_void;
            let mut count: size_t = 0;
            if load_count(val, counted, &mut count) == 0 as libc::c_int {
                free_cntype((*counted).basetype, dataptr, count);
            }
        }
        8 | 9 => {
            let mut count_0: size_t =
                get_nullterm_sequence_len(val,
                                          (*a).tinfo as *const atype_info);
            free_sequence_of((*a).tinfo as *const atype_info, val, count_0);
        }
        10 => {
            let mut tag: *const tagged_info =
                (*a).tinfo as *const tagged_info;
            free_atype((*tag).basetype, val);
        }
        11 | 12 | 13 | 14 => { }
        _ => { abort(); }
    };
}
/* *** Functions for freeing C objects based on type info ****/
#[c2rust::src_loc = "951:1"]
unsafe extern "C" fn free_atype_ptr(mut a: *const atype_info,
                                    mut val: *mut libc::c_void) {
    match (*a).type_0 as libc::c_uint {
        2 | 7 | 6 | 8 | 9 | 11 | 12 | 13 | 14 => { }
        3 => {
            let mut ptrinfo: *const ptr_info = (*a).tinfo as *const ptr_info;
            if (*ptrinfo).loadptr.is_some() {
            } else {
                __assert_fail(b"(ptrinfo)->loadptr != NULL\x00" as *const u8
                                  as *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              967 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 55],
                                                        &[libc::c_char; 55]>(b"void free_atype_ptr(const struct atype_info *, void *)\x00")).as_ptr());
            }
            let mut ptr: *mut libc::c_void =
                (*ptrinfo).loadptr.expect("non-null function pointer")(val);
            free(ptr);
            if (*ptrinfo).storeptr.is_some() {
            } else {
                __assert_fail(b"(ptrinfo)->storeptr != NULL\x00" as *const u8
                                  as *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              969 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 55],
                                                        &[libc::c_char; 55]>(b"void free_atype_ptr(const struct atype_info *, void *)\x00")).as_ptr());
            }
            (*ptrinfo).storeptr.expect("non-null function pointer")(0 as
                                                                        *mut libc::c_void,
                                                                    val);
        }
        4 => {
            let mut off: *const offset_info =
                (*a).tinfo as *const offset_info;
            if !(*off).basetype.is_null() {
            } else {
                __assert_fail(b"off->basetype != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              974 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 55],
                                                        &[libc::c_char; 55]>(b"void free_atype_ptr(const struct atype_info *, void *)\x00")).as_ptr());
            }
            free_atype_ptr((*off).basetype,
                           (val as
                                *mut libc::c_char).offset((*off).dataoff() as
                                                              libc::c_int as
                                                              isize) as
                               *mut libc::c_void);
        }
        5 => {
            let mut opt: *const optional_info =
                (*a).tinfo as *const optional_info;
            free_atype_ptr((*opt).basetype, val);
        }
        10 => {
            let mut tag: *const tagged_info =
                (*a).tinfo as *const tagged_info;
            free_atype_ptr((*tag).basetype, val);
        }
        _ => { abort(); }
    };
}
#[c2rust::src_loc = "993:1"]
unsafe extern "C" fn free_cntype(mut c: *const cntype_info,
                                 mut val: *mut libc::c_void,
                                 mut count: size_t) {
    match (*c).type_0 as libc::c_uint {
        2 | 3 => {
            free(*(val as *mut *mut libc::c_char) as *mut libc::c_void);
            let ref mut fresh6 = *(val as *mut *mut libc::c_char);
            *fresh6 = 0 as *mut libc::c_char
        }
        4 => {
            let mut a: *const atype_info = (*c).tinfo as *const atype_info;
            let mut ptrinfo: *const ptr_info = (*a).tinfo as *const ptr_info;
            if (*ptrinfo).loadptr.is_some() {
            } else {
                __assert_fail(b"(ptrinfo)->loadptr != NULL\x00" as *const u8
                                  as *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1005 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 61],
                                                        &[libc::c_char; 61]>(b"void free_cntype(const struct cntype_info *, void *, size_t)\x00")).as_ptr());
            }
            let mut seqptr: *mut libc::c_void =
                (*ptrinfo).loadptr.expect("non-null function pointer")(val);
            free_sequence_of((*ptrinfo).basetype, seqptr, count);
            free(seqptr);
            if (*ptrinfo).storeptr.is_some() {
            } else {
                __assert_fail(b"(ptrinfo)->storeptr != NULL\x00" as *const u8
                                  as *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1008 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 61],
                                                        &[libc::c_char; 61]>(b"void free_cntype(const struct cntype_info *, void *, size_t)\x00")).as_ptr());
            }
            (*ptrinfo).storeptr.expect("non-null function pointer")(0 as
                                                                        *mut libc::c_void,
                                                                    val);
        }
        5 => {
            let mut choice: *const choice_info =
                (*c).tinfo as *const choice_info;
            if count < (*choice).n_options {
                free_atype(*(*choice).options.offset(count as isize), val);
                free_atype_ptr(*(*choice).options.offset(count as isize),
                               val);
            }
        }
        _ => { abort(); }
    };
}
#[c2rust::src_loc = "1024:1"]
unsafe extern "C" fn free_sequence(mut seq: *const seq_info,
                                   mut val: *mut libc::c_void) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*seq).n_fields {
        free_atype(*(*seq).fields.offset(i as isize), val);
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as size_t;
    while i < (*seq).n_fields {
        free_atype_ptr(*(*seq).fields.offset(i as isize), val);
        i = i.wrapping_add(1)
    };
}
#[c2rust::src_loc = "1035:1"]
unsafe extern "C" fn free_sequence_of(mut eltinfo: *const atype_info,
                                      mut val: *mut libc::c_void,
                                      mut count: size_t) {
    let mut eltptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*eltinfo).size != 0 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"eltinfo->size != 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"asn1_encode.c\x00" as *const u8 as
                          *const libc::c_char,
                      1040 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 65],
                                                &[libc::c_char; 65]>(b"void free_sequence_of(const struct atype_info *, void *, size_t)\x00")).as_ptr());
    }
    loop  {
        let fresh7 = count;
        count = count.wrapping_sub(1);
        if !(fresh7 > 0 as libc::c_int as libc::c_ulong) { break ; }
        eltptr =
            (val as
                 *mut libc::c_char).offset(count.wrapping_mul((*eltinfo).size)
                                               as isize) as *mut libc::c_void;
        free_atype(eltinfo, eltptr);
        free_atype_ptr(eltinfo, eltptr);
    };
}
/* *** Functions for decoding objects based on type info ****/
/* Return nonzero if t is an expected tag for an ASN.1 object of type a. */
#[c2rust::src_loc = "1051:1"]
unsafe extern "C" fn check_atype_tag(mut a: *const atype_info,
                                     mut t: *const taginfo) -> libc::c_int {
    match (*a).type_0 as libc::c_uint {
        2 => {
            let mut fn_0: *const fn_info = (*a).tinfo as *const fn_info;
            if (*fn_0).check_tag.is_some() {
            } else {
                __assert_fail(b"fn->check_tag != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1057 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 64],
                                                        &[libc::c_char; 64]>(b"int check_atype_tag(const struct atype_info *, const taginfo *)\x00")).as_ptr());
            }
            return (*fn_0).check_tag.expect("non-null function pointer")(t)
        }
        7 | 8 | 9 => {
            return ((*t).asn1class as libc::c_uint ==
                        UNIVERSAL as libc::c_int as libc::c_uint &&
                        (*t).construction as libc::c_uint ==
                            CONSTRUCTED as libc::c_int as libc::c_uint &&
                        (*t).tagnum == 16 as libc::c_int) as libc::c_int
        }
        3 => {
            let mut ptrinfo: *const ptr_info = (*a).tinfo as *const ptr_info;
            return check_atype_tag((*ptrinfo).basetype, t)
        }
        4 => {
            let mut off: *const offset_info =
                (*a).tinfo as *const offset_info;
            return check_atype_tag((*off).basetype, t)
        }
        5 => {
            let mut opt: *const optional_info =
                (*a).tinfo as *const optional_info;
            return check_atype_tag((*opt).basetype, t)
        }
        6 => {
            let mut counted: *const counted_info =
                (*a).tinfo as *const counted_info;
            match (*(*counted).basetype).type_0 as libc::c_uint {
                2 => {
                    let mut string: *const string_info =
                        (*(*counted).basetype).tinfo as *const string_info;
                    return ((*t).asn1class as libc::c_uint ==
                                UNIVERSAL as libc::c_int as libc::c_uint &&
                                (*t).construction as libc::c_uint ==
                                    PRIMITIVE as libc::c_int as libc::c_uint
                                &&
                                (*t).tagnum ==
                                    (*string).tagval() as libc::c_int) as
                               libc::c_int
                }
                4 => {
                    return ((*t).asn1class as libc::c_uint ==
                                UNIVERSAL as libc::c_int as libc::c_uint &&
                                (*t).construction as libc::c_uint ==
                                    CONSTRUCTED as libc::c_int as libc::c_uint
                                && (*t).tagnum == 16 as libc::c_int) as
                               libc::c_int
                }
                3 => {
                    /*
             * We treat any tag as matching a stored DER encoding.  In some
             * cases we know what the tag should be; in others, we truly want
             * to accept any tag.  If it ever becomes an issue, we could add
             * optional tag info to the type and check it here.
             */
                    return 1 as libc::c_int
                }
                5 => {
                    /*
             * ASN.1 choices may or may not be extensible.  For now, we treat
             * all choices as extensible and match any tag.  We should consider
             * modeling whether choices are extensible before making the
             * encoder visible to plugins.
             */
                    return 1 as libc::c_int
                }
                _ => { abort(); }
            }
        }
        10 => {
            let mut tag: *const tagged_info =
                (*a).tinfo as *const tagged_info;
            /* NOTE: Doesn't check construction bit for implicit tags. */
            if (*tag).implicit() == 0 &&
                   (*t).construction as libc::c_uint != (*tag).construction()
               {
                return 0 as libc::c_int
            }
            return ((*t).asn1class as libc::c_uint == (*tag).tagtype() &&
                        (*t).tagnum == (*tag).tagval() as libc::c_int) as
                       libc::c_int
        }
        11 => {
            return ((*t).asn1class as libc::c_uint ==
                        UNIVERSAL as libc::c_int as libc::c_uint &&
                        (*t).construction as libc::c_uint ==
                            PRIMITIVE as libc::c_int as libc::c_uint &&
                        (*t).tagnum == 1 as libc::c_int) as libc::c_int
        }
        12 | 13 | 14 => {
            return ((*t).asn1class as libc::c_uint ==
                        UNIVERSAL as libc::c_int as libc::c_uint &&
                        (*t).construction as libc::c_uint ==
                            PRIMITIVE as libc::c_int as libc::c_uint &&
                        (*t).tagnum == 2 as libc::c_int) as libc::c_int
        }
        _ => { abort(); }
    };
}
/* Given the enclosing tag t, decode from asn1/len the contents of the ASN.1
 * type specified by a, placing the result into val (caller-allocated). */
#[c2rust::src_loc = "1146:1"]
unsafe extern "C" fn decode_atype(mut t: *const taginfo,
                                  mut asn1: *const uint8_t, mut len: size_t,
                                  mut a: *const atype_info,
                                  mut val: *mut libc::c_void)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    match (*a).type_0 as libc::c_uint {
        2 => {
            let mut fn_0: *const fn_info = (*a).tinfo as *const fn_info;
            if (*fn_0).dec.is_some() {
            } else {
                __assert_fail(b"fn->dec != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1155 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 106],
                                                        &[libc::c_char; 106]>(b"krb5_error_code decode_atype(const taginfo *, const uint8_t *, size_t, const struct atype_info *, void *)\x00")).as_ptr());
            }
            return (*fn_0).dec.expect("non-null function pointer")(t, asn1,
                                                                   len, val)
        }
        7 => {
            return decode_sequence(asn1, len, (*a).tinfo as *const seq_info,
                                   val)
        }
        3 => {
            let mut ptrinfo: *const ptr_info = (*a).tinfo as *const ptr_info;
            if (*ptrinfo).loadptr.is_some() {
            } else {
                __assert_fail(b"(ptrinfo)->loadptr != NULL\x00" as *const u8
                                  as *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1162 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 106],
                                                        &[libc::c_char; 106]>(b"krb5_error_code decode_atype(const taginfo *, const uint8_t *, size_t, const struct atype_info *, void *)\x00")).as_ptr());
            }
            let mut ptr: *mut libc::c_void =
                (*ptrinfo).loadptr.expect("non-null function pointer")(val);
            if !(*ptrinfo).basetype.is_null() {
            } else {
                __assert_fail(b"ptrinfo->basetype != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1163 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 106],
                                                        &[libc::c_char; 106]>(b"krb5_error_code decode_atype(const taginfo *, const uint8_t *, size_t, const struct atype_info *, void *)\x00")).as_ptr());
            }
            if !ptr.is_null() {
                /* Container was already allocated by a previous sequence field. */
                return decode_atype(t, asn1, len, (*ptrinfo).basetype, ptr)
            } else {
                ret =
                    decode_atype_to_ptr(t, asn1, len, (*ptrinfo).basetype,
                                        &mut ptr);
                if ret != 0 { return ret }
                if (*ptrinfo).storeptr.is_some() {
                } else {
                    __assert_fail(b"(ptrinfo)->storeptr != NULL\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"asn1_encode.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  1171 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 106],
                                                            &[libc::c_char; 106]>(b"krb5_error_code decode_atype(const taginfo *, const uint8_t *, size_t, const struct atype_info *, void *)\x00")).as_ptr());
                }
                (*ptrinfo).storeptr.expect("non-null function pointer")(ptr,
                                                                        val);
            }
        }
        4 => {
            let mut off: *const offset_info =
                (*a).tinfo as *const offset_info;
            if !(*off).basetype.is_null() {
            } else {
                __assert_fail(b"off->basetype != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1177 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 106],
                                                        &[libc::c_char; 106]>(b"krb5_error_code decode_atype(const taginfo *, const uint8_t *, size_t, const struct atype_info *, void *)\x00")).as_ptr());
            }
            return decode_atype(t, asn1, len, (*off).basetype,
                                (val as
                                     *mut libc::c_char).offset((*off).dataoff()
                                                                   as
                                                                   libc::c_int
                                                                   as isize)
                                    as *mut libc::c_void)
        }
        5 => {
            let mut opt: *const optional_info =
                (*a).tinfo as *const optional_info;
            return decode_atype(t, asn1, len, (*opt).basetype, val)
        }
        6 => {
            let mut counted: *const counted_info =
                (*a).tinfo as *const counted_info;
            let mut dataptr: *mut libc::c_void =
                (val as
                     *mut libc::c_char).offset((*counted).dataoff() as
                                                   libc::c_int as isize) as
                    *mut libc::c_void;
            let mut count: size_t = 0;
            if !(*counted).basetype.is_null() {
            } else {
                __assert_fail(b"counted->basetype != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1189 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 106],
                                                        &[libc::c_char; 106]>(b"krb5_error_code decode_atype(const taginfo *, const uint8_t *, size_t, const struct atype_info *, void *)\x00")).as_ptr());
            }
            ret =
                decode_cntype(t, asn1, len, (*counted).basetype, dataptr,
                              &mut count);
            if ret != 0 { return ret }
            return store_count(count, counted, val)
        }
        10 => {
            let mut tag: *const tagged_info =
                (*a).tinfo as *const tagged_info;
            let mut inner_tag: taginfo =
                taginfo{asn1class: UNIVERSAL,
                        construction: PRIMITIVE,
                        tagnum: 0,
                        tag_len: 0,
                        tag_end_len: 0,};
            let mut tp: *const taginfo = t;
            let mut rem: *const uint8_t = 0 as *const uint8_t;
            let mut rlen: size_t = 0;
            if (*tag).implicit() == 0 {
                ret =
                    get_tag(asn1, len, &mut inner_tag, &mut asn1, &mut len,
                            &mut rem, &mut rlen);
                if ret != 0 { return ret }
                /* Note: we don't check rlen (it should be 0). */
                tp = &mut inner_tag;
                if check_atype_tag((*tag).basetype, tp) == 0 {
                    return 1859794438 as libc::c_long as krb5_error_code
                }
            }
            return decode_atype(tp, asn1, len, (*tag).basetype, val)
        }
        11 => {
            let mut intval: intmax_t = 0;
            ret = k5_asn1_decode_bool(asn1, len, &mut intval);
            if ret != 0 { return ret }
            return store_int(intval, (*a).size, val)
        }
        12 => {
            let mut intval_0: intmax_t = 0;
            ret = k5_asn1_decode_int(asn1, len, &mut intval_0);
            if ret != 0 { return ret }
            return store_int(intval_0, (*a).size, val)
        }
        13 => {
            let mut intval_1: uintmax_t = 0;
            ret = k5_asn1_decode_uint(asn1, len, &mut intval_1);
            if ret != 0 { return ret }
            return store_uint(intval_1, (*a).size, val)
        }
        14 => {
            let mut imm: *const immediate_info =
                (*a).tinfo as *const immediate_info;
            let mut intval_2: intmax_t = 0;
            ret = k5_asn1_decode_int(asn1, len, &mut intval_2);
            if ret != 0 { return ret }
            if intval_2 != (*imm).val && (*imm).err != 0 as libc::c_int {
                return (*imm).err
            }
        }
        _ => {
            /* Null-terminated sequence types are handled in decode_atype_to_ptr,
         * since they create variable-sized objects. */
            if (*a).type_0 as libc::c_uint !=
                   atype_nullterm_sequence_of as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(b"a->type != atype_nullterm_sequence_of\x00" as
                                  *const u8 as *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1246 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 106],
                                                        &[libc::c_char; 106]>(b"krb5_error_code decode_atype(const taginfo *, const uint8_t *, size_t, const struct atype_info *, void *)\x00")).as_ptr());
            }
            if (*a).type_0 as libc::c_uint !=
                   atype_nonempty_nullterm_sequence_of as libc::c_int as
                       libc::c_uint {
            } else {
                __assert_fail(b"a->type != atype_nonempty_nullterm_sequence_of\x00"
                                  as *const u8 as *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1247 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 106],
                                                        &[libc::c_char; 106]>(b"krb5_error_code decode_atype(const taginfo *, const uint8_t *, size_t, const struct atype_info *, void *)\x00")).as_ptr());
            }
            if (*a).type_0 as libc::c_uint >
                   atype_min as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(b"a->type > atype_min\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1248 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 106],
                                                        &[libc::c_char; 106]>(b"krb5_error_code decode_atype(const taginfo *, const uint8_t *, size_t, const struct atype_info *, void *)\x00")).as_ptr());
            }
            if ((*a).type_0 as libc::c_uint) <
                   atype_max as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(b"a->type < atype_max\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1249 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 106],
                                                        &[libc::c_char; 106]>(b"krb5_error_code decode_atype(const taginfo *, const uint8_t *, size_t, const struct atype_info *, void *)\x00")).as_ptr());
            }
            abort();
        }
    }
    return 0 as libc::c_int;
}
/*
 * Given the enclosing tag t, decode from asn1/len the contents of the
 * ASN.1 type described by c, placing the counted result into val/count_out.
 * If the resulting count should be -1 (for an unknown union distinguisher),
 * set *count_out to SIZE_MAX.
 */
#[c2rust::src_loc = "1261:1"]
unsafe extern "C" fn decode_cntype(mut t: *const taginfo,
                                   mut asn1: *const uint8_t, mut len: size_t,
                                   mut c: *const cntype_info,
                                   mut val: *mut libc::c_void,
                                   mut count_out: *mut size_t)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    match (*c).type_0 as libc::c_uint {
        2 => {
            let mut string: *const string_info =
                (*c).tinfo as *const string_info;
            if (*string).dec.is_some() {
            } else {
                __assert_fail(b"string->dec != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1270 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 118],
                                                        &[libc::c_char; 118]>(b"krb5_error_code decode_cntype(const taginfo *, const uint8_t *, size_t, const struct cntype_info *, void *, size_t *)\x00")).as_ptr());
            }
            return (*string).dec.expect("non-null function pointer")(asn1,
                                                                     len,
                                                                     val as
                                                                         *mut *mut uint8_t,
                                                                     count_out)
        }
        3 => { return store_der(t, asn1, len, val, count_out) }
        4 => {
            let mut a: *const atype_info = (*c).tinfo as *const atype_info;
            let mut ptrinfo: *const ptr_info = (*a).tinfo as *const ptr_info;
            let mut seq: *mut libc::c_void = 0 as *mut libc::c_void;
            if (*a).type_0 as libc::c_uint ==
                   atype_ptr as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(b"a->type == atype_ptr\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1279 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 118],
                                                        &[libc::c_char; 118]>(b"krb5_error_code decode_cntype(const taginfo *, const uint8_t *, size_t, const struct cntype_info *, void *, size_t *)\x00")).as_ptr());
            }
            ret =
                decode_sequence_of(asn1, len, (*ptrinfo).basetype, &mut seq,
                                   count_out);
            if ret != 0 { return ret }
            if (*ptrinfo).storeptr.is_some() {
            } else {
                __assert_fail(b"(ptrinfo)->storeptr != NULL\x00" as *const u8
                                  as *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1284 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 118],
                                                        &[libc::c_char; 118]>(b"krb5_error_code decode_cntype(const taginfo *, const uint8_t *, size_t, const struct cntype_info *, void *, size_t *)\x00")).as_ptr());
            }
            (*ptrinfo).storeptr.expect("non-null function pointer")(seq, val);
        }
        5 => {
            let mut choice: *const choice_info =
                (*c).tinfo as *const choice_info;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*choice).n_options {
                if check_atype_tag(*(*choice).options.offset(i as isize), t)
                       != 0 {
                    ret =
                        decode_atype(t, asn1, len,
                                     *(*choice).options.offset(i as isize),
                                     val);
                    if ret != 0 { return ret }
                    *count_out = i;
                    return 0 as libc::c_int
                }
                i = i.wrapping_add(1)
            }
            /* SIZE_MAX will be stored as -1 in the distinguisher.  If we start
         * modeling non-extensible choices we should check that here. */
            *count_out = 18446744073709551615 as libc::c_ulong
        }
        _ => {
            if (*c).type_0 as libc::c_uint >
                   cntype_min as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(b"c->type > cntype_min\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1305 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 118],
                                                        &[libc::c_char; 118]>(b"krb5_error_code decode_cntype(const taginfo *, const uint8_t *, size_t, const struct cntype_info *, void *, size_t *)\x00")).as_ptr());
            }
            if ((*c).type_0 as libc::c_uint) <
                   cntype_max as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(b"c->type < cntype_max\x00" as *const u8 as
                                  *const libc::c_char,
                              b"asn1_encode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1306 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 118],
                                                        &[libc::c_char; 118]>(b"krb5_error_code decode_cntype(const taginfo *, const uint8_t *, size_t, const struct cntype_info *, void *, size_t *)\x00")).as_ptr());
            }
            abort();
        }
    }
    return 0 as libc::c_int;
}
/* Add a null pointer to the end of a sequence.  ptr is consumed on success
 * (to be replaced by *ptr_out), left alone on failure. */
#[c2rust::src_loc = "1314:1"]
unsafe extern "C" fn null_terminate(mut eltinfo: *const atype_info,
                                    mut ptr: *mut libc::c_void,
                                    mut count: size_t,
                                    mut ptr_out: *mut *mut libc::c_void)
 -> krb5_error_code {
    let mut ptrinfo: *const ptr_info = (*eltinfo).tinfo as *const ptr_info;
    let mut endptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*eltinfo).type_0 as libc::c_uint ==
           atype_ptr as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"eltinfo->type == atype_ptr\x00" as *const u8 as
                          *const libc::c_char,
                      b"asn1_encode.c\x00" as *const u8 as
                          *const libc::c_char,
                      1321 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 83],
                                                &[libc::c_char; 83]>(b"krb5_error_code null_terminate(const struct atype_info *, void *, size_t, void **)\x00")).as_ptr());
    }
    ptr =
        realloc(ptr,
                count.wrapping_add(1 as libc::c_int as
                                       libc::c_ulong).wrapping_mul((*eltinfo).size));
    if ptr.is_null() { return 12 as libc::c_int }
    endptr =
        (ptr as
             *mut libc::c_char).offset(count.wrapping_mul((*eltinfo).size) as
                                           isize) as *mut libc::c_void;
    if (*ptrinfo).storeptr.is_some() {
    } else {
        __assert_fail(b"(ptrinfo)->storeptr != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"asn1_encode.c\x00" as *const u8 as
                          *const libc::c_char,
                      1326 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 83],
                                                &[libc::c_char; 83]>(b"krb5_error_code null_terminate(const struct atype_info *, void *, size_t, void **)\x00")).as_ptr());
    }
    (*ptrinfo).storeptr.expect("non-null function pointer")(0 as
                                                                *mut libc::c_void,
                                                            endptr);
    *ptr_out = ptr;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1331:1"]
unsafe extern "C" fn decode_atype_to_ptr(mut t: *const taginfo,
                                         mut asn1: *const uint8_t,
                                         mut len: size_t,
                                         mut a: *const atype_info,
                                         mut ptr_out: *mut *mut libc::c_void)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut count: size_t = 0;
    *ptr_out = 0 as *mut libc::c_void;
    match (*a).type_0 as libc::c_uint {
        8 | 9 => {
            ret =
                decode_sequence_of(asn1, len, (*a).tinfo as *const atype_info,
                                   &mut ptr, &mut count);
            if ret != 0 { return ret }
            ret =
                null_terminate((*a).tinfo as *const atype_info, ptr, count,
                               &mut ptr);
            if ret != 0 {
                free_sequence_of((*a).tinfo as *const atype_info, ptr, count);
                return ret
            }
        }
        _ => {
            ptr = calloc((*a).size, 1 as libc::c_int as libc::c_ulong);
            if ptr.is_null() { return 12 as libc::c_int }
            ret = decode_atype(t, asn1, len, a, ptr);
            if ret != 0 { free(ptr); return ret }
        }
    }
    *ptr_out = ptr;
    return 0 as libc::c_int;
}
/* Initialize a C object when the corresponding ASN.1 type was omitted within a
 * sequence.  If the ASN.1 type is not optional, return ASN1_MISSING_FIELD. */
#[c2rust::src_loc = "1371:1"]
unsafe extern "C" fn omit_atype(mut a: *const atype_info,
                                mut val: *mut libc::c_void)
 -> krb5_error_code {
    match (*a).type_0 as libc::c_uint {
        2 | 7 | 8 | 9 | 6 | 11 | 12 | 13 | 14 => {
            return 1859794433 as libc::c_long as krb5_error_code
        }
        3 => {
            let mut ptrinfo: *const ptr_info = (*a).tinfo as *const ptr_info;
            return omit_atype((*ptrinfo).basetype, val)
        }
        4 => {
            let mut off: *const offset_info =
                (*a).tinfo as *const offset_info;
            return omit_atype((*off).basetype,
                              (val as
                                   *mut libc::c_char).offset((*off).dataoff()
                                                                 as
                                                                 libc::c_int
                                                                 as isize) as
                                  *mut libc::c_void)
        }
        10 => {
            let mut tag: *const tagged_info =
                (*a).tinfo as *const tagged_info;
            return omit_atype((*tag).basetype, val)
        }
        5 => {
            let mut opt: *const optional_info =
                (*a).tinfo as *const optional_info;
            if (*opt).init.is_some() {
                (*opt).init.expect("non-null function pointer")(val);
            }
            return 0 as libc::c_int
        }
        _ => { abort(); }
    };
}
/* Decode an ASN.1 sequence into a C object. */
#[c2rust::src_loc = "1410:1"]
unsafe extern "C" fn decode_sequence(mut asn1: *const uint8_t,
                                     mut len: size_t,
                                     mut seq: *const seq_info,
                                     mut val: *mut libc::c_void)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut contents: *const uint8_t = 0 as *const uint8_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut clen: size_t = 0;
    let mut t: taginfo =
        taginfo{asn1class: UNIVERSAL,
                construction: PRIMITIVE,
                tagnum: 0,
                tag_len: 0,
                tag_end_len: 0,};
    if (*seq).n_fields > 0 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"seq->n_fields > 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"asn1_encode.c\x00" as *const u8 as
                          *const libc::c_char,
                      1419 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 90],
                                                &[libc::c_char; 90]>(b"krb5_error_code decode_sequence(const uint8_t *, size_t, const struct seq_info *, void *)\x00")).as_ptr());
    }
    i = 0 as libc::c_int as size_t;
    's_19:
        loop  {
            if !(i < (*seq).n_fields) {
                current_block = 5689001924483802034;
                break ;
            }
            if len == 0 as libc::c_int as libc::c_ulong {
                current_block = 5689001924483802034;
                break ;
            }
            ret =
                get_tag(asn1, len, &mut t, &mut contents, &mut clen,
                        &mut asn1, &mut len);
            if ret != 0 { current_block = 17310599924967574749; break ; }
            /*
         * Find the applicable sequence field.  This logic is a little
         * oversimplified; we could match an element to an optional extensible
         * choice or optional stored-DER type when we ought to match a
         * subsequent non-optional field.  But it's unwise and (hopefully) very
         * rare for ASN.1 modules to require such precision.
         */
            while i < (*seq).n_fields {
                if check_atype_tag(*(*seq).fields.offset(i as isize), &mut t)
                       != 0 {
                    break ;
                }
                ret = omit_atype(*(*seq).fields.offset(i as isize), val);
                if ret != 0 {
                    current_block = 17310599924967574749;
                    break 's_19 ;
                }
                i = i.wrapping_add(1)
            }
            /* We currently model all sequences as extensible.  We should consider
         * changing this before making the encoder visible to plugins. */
            if i == (*seq).n_fields {
                current_block = 5689001924483802034;
                break ;
            }
            ret =
                decode_atype(&mut t, contents, clen,
                             *(*seq).fields.offset(i as isize), val);
            if ret != 0 { current_block = 17310599924967574749; break ; }
            i = i.wrapping_add(1)
        }
    loop  {
        match current_block {
            5689001924483802034 =>
            /* Initialize any fields in the C object which were not accounted for in
     * the sequence.  Error out if any of them aren't optional. */
            {
                if i < (*seq).n_fields {
                    ret = omit_atype(*(*seq).fields.offset(i as isize), val);
                    if ret != 0 {
                        current_block = 17310599924967574749;
                        continue ;
                    }
                    i = i.wrapping_add(1);
                    current_block = 5689001924483802034;
                } else { return 0 as libc::c_int }
            }
            _ => {
                /* Free what we've decoded so far.  Free pointers in a second pass in
     * case multiple fields refer to the same pointer. */
                j = 0 as libc::c_int as size_t;
                while j < i {
                    free_atype(*(*seq).fields.offset(j as isize), val);
                    j = j.wrapping_add(1)
                }
                j = 0 as libc::c_int as size_t;
                while j < i {
                    free_atype_ptr(*(*seq).fields.offset(j as isize), val);
                    j = j.wrapping_add(1)
                }
                return ret
            }
        }
    };
}
#[c2rust::src_loc = "1467:1"]
unsafe extern "C" fn decode_sequence_of(mut asn1: *const uint8_t,
                                        mut len: size_t,
                                        mut elemtype: *const atype_info,
                                        mut seq_out: *mut *mut libc::c_void,
                                        mut count_out: *mut size_t)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut seq: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut elem: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut newseq: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut contents: *const uint8_t = 0 as *const uint8_t;
    let mut clen: size_t = 0;
    let mut count: size_t = 0 as libc::c_int as size_t;
    let mut t: taginfo =
        taginfo{asn1class: UNIVERSAL,
                construction: PRIMITIVE,
                tagnum: 0,
                tag_len: 0,
                tag_end_len: 0,};
    *seq_out = 0 as *mut libc::c_void;
    *count_out = 0 as libc::c_int as size_t;
    loop  {
        if !(len > 0 as libc::c_int as libc::c_ulong) {
            current_block = 7175849428784450219;
            break ;
        }
        ret =
            get_tag(asn1, len, &mut t, &mut contents, &mut clen, &mut asn1,
                    &mut len);
        if ret != 0 { current_block = 6489683750670701722; break ; }
        if check_atype_tag(elemtype, &mut t) == 0 {
            ret = 1859794438 as libc::c_long as krb5_error_code;
            current_block = 6489683750670701722;
            break ;
        } else {
            newseq =
                realloc(seq,
                        count.wrapping_add(1 as libc::c_int as
                                               libc::c_ulong).wrapping_mul((*elemtype).size));
            if newseq.is_null() {
                ret = 12 as libc::c_int;
                current_block = 6489683750670701722;
                break ;
            } else {
                seq = newseq;
                elem =
                    (seq as
                         *mut libc::c_char).offset(count.wrapping_mul((*elemtype).size)
                                                       as isize) as
                        *mut libc::c_void;
                memset(elem, 0 as libc::c_int, (*elemtype).size);
                ret = decode_atype(&mut t, contents, clen, elemtype, elem);
                if ret != 0 { current_block = 6489683750670701722; break ; }
                count = count.wrapping_add(1)
            }
        }
    }
    match current_block {
        6489683750670701722 => {
            free_sequence_of(elemtype, seq, count);
            free(seq);
            return ret
        }
        _ => { *seq_out = seq; *count_out = count; return 0 as libc::c_int }
    };
}
/* These three entry points are only needed for the kdc_req_body hack and may
 * go away at some point.  Define them here so we can use short names above. */
#[no_mangle]
#[c2rust::src_loc = "1514:1"]
pub unsafe extern "C" fn k5_asn1_encode_atype(mut buf: *mut asn1buf,
                                              mut val: *const libc::c_void,
                                              mut a: *const atype_info,
                                              mut tag_out: *mut taginfo)
 -> krb5_error_code {
    return encode_atype(buf, val, a, tag_out);
}
#[no_mangle]
#[c2rust::src_loc = "1521:1"]
pub unsafe extern "C" fn k5_asn1_decode_atype(mut t: *const taginfo,
                                              mut asn1: *const uint8_t,
                                              mut len: size_t,
                                              mut a: *const atype_info,
                                              mut val: *mut libc::c_void)
 -> krb5_error_code {
    return decode_atype(t, asn1, len, a, val);
}
#[no_mangle]
#[c2rust::src_loc = "1528:1"]
pub unsafe extern "C" fn k5_asn1_full_encode(mut rep: *const libc::c_void,
                                             mut a: *const atype_info,
                                             mut code_out:
                                                 *mut *mut krb5_data)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut buf: asn1buf = asn1buf{ptr: 0 as *mut uint8_t, count: 0,};
    let mut d: *mut krb5_data = 0 as *mut krb5_data;
    let mut bytes: *mut uint8_t = 0 as *mut uint8_t;
    *code_out = 0 as *mut krb5_data;
    if rep.is_null() { return 1859794433 as libc::c_long as krb5_error_code }
    /* Make a first pass over rep to count the encoding size. */
    buf.ptr = 0 as *mut uint8_t;
    buf.count = 0 as libc::c_int as size_t;
    ret = encode_atype_and_tag(&mut buf, rep, a);
    if ret != 0 { return ret }
    /* Allocate space for the encoding. */
    bytes =
        malloc(buf.count.wrapping_add(1 as libc::c_int as libc::c_ulong)) as
            *mut uint8_t;
    if bytes.is_null() { return 12 as libc::c_int }
    *bytes.offset(buf.count as isize) = 0 as libc::c_int as uint8_t;
    /* Make a second pass over rep to encode it.  buf.ptr moves backwards as we
     * encode, and will always exactly return to the base. */
    buf.ptr = bytes.offset(buf.count as isize);
    buf.count = 0 as libc::c_int as size_t;
    ret = encode_atype_and_tag(&mut buf, rep, a);
    if ret != 0 { free(bytes as *mut libc::c_void); return ret }
    if buf.ptr == bytes {
    } else {
        __assert_fail(b"buf.ptr == bytes\x00" as *const u8 as
                          *const libc::c_char,
                      b"asn1_encode.c\x00" as *const u8 as
                          *const libc::c_char,
                      1564 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 91],
                                                &[libc::c_char; 91]>(b"krb5_error_code k5_asn1_full_encode(const void *, const struct atype_info *, krb5_data **)\x00")).as_ptr());
    }
    /* Create the output data object. */
    *code_out =
        malloc(::std::mem::size_of::<krb5_data>() as libc::c_ulong) as
            *mut krb5_data;
    if (*code_out).is_null() {
        free(bytes as *mut libc::c_void);
        return 12 as libc::c_int
    }
    **code_out =
        make_data(bytes as *mut libc::c_void, buf.count as libc::c_uint);
    return 0 as libc::c_int;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/asn.1/asn1_encode.h */
/*
 * Copyright 1994, 2008 by the Massachusetts Institute of Technology.
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
/* When decoding, stores the leading and trailing lengths of a tag.  Used
     * by store_der(). */
/* These functions are referenced by encoder structures.  They handle the
 * encoding of primitive ASN.1 types. */
/* These functions are referenced by encoder structures.  They handle the
 * decoding of primitive ASN.1 types. */
/*
 * An atype_info structure specifies how to map a C object to an ASN.1 value.
 *
 * We wind up with a lot of load-time relocations being done, which is
 * a bit annoying.  Be careful about "fixing" that at the cost of too
 * much run-time performance.  It might work to have a master "module"
 * descriptor with pointers to various arrays (type descriptors,
 * strings, field descriptors, functions) most of which don't need
 * relocation themselves, and replace most of the pointers with table
 * indices.
 *
 * It's a work in progress.
 */
/* For bounds checking only.  By starting with 2, we guarantee that
     * zero-initialized storage will be recognized as invalid. */
/* Use a function table to handle encoding or decoding.  tinfo is a struct
     * fn_info *. */
/* C object is a pointer to the object to be encoded or decoded.  tinfo is
     * a struct ptr_info *. */
/* C object to be encoded or decoded is at an offset from the original
     * pointer.  tinfo is a struct offset_info *. */
/*
     * Indicates a sequence field which may or may not be present in the C
     * object or ASN.1 sequence.  tinfo is a struct optional_info *.  Must be
     * used within a sequence, although the optional type may be nested within
     * offset, ptr, and/or tag types.
     */
/*
     * C object contains an integer and another C object at specified offsets,
     * to be combined and encoded or decoded as specified by a cntype_info
     * structure.  tinfo is a struct counted_info *.
     */
/* Sequence.  tinfo is a struct seq_info *. */
/*
     * Sequence-of, with pointer to base type descriptor, represented as a
     * null-terminated array of pointers (and thus the "base" type descriptor
     * is actually an atype_ptr node).  tinfo is a struct atype_info * giving
     * the base type.
     */
/* Tagged version of another type.  tinfo is a struct tagged_info *. */
/* Boolean value.  tinfo is NULL (size field determines C type width). */
/* Signed or unsigned integer.  tinfo is NULL. */
/*
     * Integer value taken from the type info, not from the object being
     * encoded.  tinfo is a struct immediate_info * giving the integer value
     * and error code to return if a decoded object doesn't match it (or 0 if
     * the value shouldn't be checked on decode).
     */
/* Unused except for bounds checking.  */
/* Used for sequence-of processing */
/* Points to type-specific structure */
/* A cntype_info structure specifies how to map a C object and count (length or
 * union distinguisher) to an ASN.1 value. */
/*
     * Apply an encoder function (contents only) and wrap it in a universal
     * primitive tag.  The C object must be a char * or uint8_t *.  tinfo
     * is a struct string_info *.
     */
/*
     * The C object is a DER encoding (with tag), to be simply inserted on
     * encode or stored on decode.  The C object must be a char * or unsigned
     * char *.  tinfo is NULL.
     */
/* An ASN.1 sequence-of value, represtened in C as a counted array.  struct
     * atype_info * giving the base type, which must be of type atype_ptr. */
/* An ASN.1 choice, represented in C as a distinguisher and union.  tinfo
     * is a struct choice_info *. */
/* Currently all sequences are assumed to be extensible. */
/*
 * The various DEF*TYPE macros must:
 *
 * + Define a type named aux_type_##DESCNAME, for use in any types derived from
 *   the type being defined.
 *
 * + Define an atype_info struct named k5_atype_##DESCNAME
 *
 * + Define a type-specific structure, referenced by the tinfo field
 *   of the atype_info structure.
 *
 * + Define any extra stuff needed in the type descriptor, like
 *   pointer-load functions.
 *
 * + Accept a following semicolon syntactically, to keep Emacs parsing
 *   (and indentation calculating) code happy.
 *
 * Nothing else should directly define the atype_info structures.
 */
/* Define a type using a function table. */
/* A sequence, defined by the indicated series of types, and an optional
 * function indicating which fields are not present. */
/* A boolean type. */
/* Integer types.  */
/* Pointers to other types, to be encoded as those other types.  */
/* Optional sequence fields.  The basic form allows arbitrary test and
 * initializer functions to be used.  INIT may be null. */
/* This form defines an is_present function for a zero-valued integer or null
 * pointer of the base type's C type. */
/* This form defines an is_present function for a null or empty null-terminated
 * array of the base type's C type. */
/*
 * This encodes a pointer-to-pointer-to-thing where the passed-in
 * value points to a null-terminated list of pointers to objects to be
 * encoded, and encodes a (possibly empty) SEQUENCE OF these objects.
 *
 * BASEDESCNAME is a descriptor name for the pointer-to-thing
 * type.
 *
 * When dealing with a structure containing a
 * pointer-to-pointer-to-thing field, make a DEFPTRTYPE of this type,
 * and use that type for the structure field.
 */
/* Objects with an explicit or implicit tag.  (Implicit tags will ignore the
 * construction field.) */
/* Objects with an explicit APPLICATION tag added.  */
/* Object with a context-specific tag added */
/* Define an offset type with an explicit context tag wrapper (the usual case
 * for an RFC 4120 sequence field). */
/* Define a counted type with an explicit context tag wrapper. */
/* Like DEFFIELD but with an implicit context tag. */
/*
 * DEFCOUNTED*TYPE macros must:
 *
 * + Define types named aux_ptrtype_##DESCNAME and aux_counttype_##DESCNAME, to
 *   allow type checking when the counted type is referenced with structure
 *   field offsets in DEFCOUNTEDTYPE.
 *
 * + Define a cntype_info struct named k5_cntype_##DESCNAME
 *
 * + Define a type-specific structure, referenced by the tinfo field of the
 *   cntype_info structure.
 *
 * + Accept a following semicolon syntactically.
 */
/*
 * Declare an externally-defined type.  This is a hack we should do
 * away with once we move to generating code from a script.  For now,
 * this macro is unfortunately not compatible with the defining macros
 * above, since you can't do the typedefs twice and we need the
 * declarations to produce typedefs.  (We could eliminate the typedefs
 * from the DEF* macros, but then every DEF* macro use, even the ones
 * for internal type nodes we only use to build other types, would
 * need an accompanying declaration which explicitly lists the
 * type.)
 */
/* Partially encode the contents of a type and return its tag information.
 * Used only by kdc_req_body. */
/* Decode the tag and contents of a type, storing the result in the
 * caller-allocated C object val.  Used only by kdc_req_body. */
/* Returns a completed encoding, with tag and in the correct byte order, in an
 * allocated krb5_data. */
#[no_mangle]
#[c2rust::src_loc = "1576:1"]
pub unsafe extern "C" fn k5_asn1_full_decode(mut code: *const krb5_data,
                                             mut a: *const atype_info,
                                             mut retrep:
                                                 *mut *mut libc::c_void)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut contents: *const uint8_t = 0 as *const uint8_t;
    let mut remainder: *const uint8_t = 0 as *const uint8_t;
    let mut clen: size_t = 0;
    let mut rlen: size_t = 0;
    let mut t: taginfo =
        taginfo{asn1class: UNIVERSAL,
                construction: PRIMITIVE,
                tagnum: 0,
                tag_len: 0,
                tag_end_len: 0,};
    *retrep = 0 as *mut libc::c_void;
    ret =
        get_tag((*code).data as *mut uint8_t, (*code).length as size_t,
                &mut t, &mut contents, &mut clen, &mut remainder, &mut rlen);
    if ret != 0 { return ret }
    /* rlen should be 0, but we don't check it (and due to padding in
     * non-length-preserving enctypes, it will sometimes be nonzero). */
    if check_atype_tag(a, &mut t) == 0 {
        return 1859794438 as libc::c_long as krb5_error_code
    }
    return decode_atype_to_ptr(&mut t, contents, clen, a, retrep);
}
