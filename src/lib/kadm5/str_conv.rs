use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:35"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:35"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:35"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:35"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    /* this strange form is necessary since - is a unary operator, not a sign
   indicator */
    /* this strange form is necessary since - is a unary operator, not a sign
   indicator */
    /*
 * end wordsize.h
 */
    /*
 * begin "base-defs.h"
 */
    /*
 * Basic definitions for Kerberos V5 library
 */
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    /* This may change, later on */
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
    /* *
 * Used to convey an operation status.  The value 0 indicates success; any
 * other values are com_err codes.  Use krb5_get_error_message() to obtain a
 * string describing the error.
 */
    #[c2rust::src_loc = "204:1"]
    pub type krb5_error_code = krb5_int32;
    /* Originally used to recognize AFS and default salts.  No longer used. */
    #[c2rust::src_loc = "225:1"]
    pub type krb5_pointer = *mut libc::c_void;
    use super::stdint_intn_h::int32_t;
    extern "C" {
        /* str_conv.c */
/* *
 * Convert a string to an encryption type.
 *
 * @param [in]  string          String to convert to an encryption type
 * @param [out] enctypep        Encryption type
 *
 * @retval 0  Success; otherwise - EINVAL
 */
        #[no_mangle]
        #[c2rust::src_loc = "6266:1"]
        pub fn krb5_string_to_enctype(string: *mut libc::c_char,
                                      enctypep: *mut krb5_enctype)
         -> krb5_error_code;
        /* *
 * Convert a string to a salt type.
 *
 * @param [in]  string          String to convert to an encryption type
 * @param [out] salttypep       Salt type to be filled in
 *
 * @retval 0  Success; otherwise - EINVAL
 */
        #[no_mangle]
        #[c2rust::src_loc = "6277:1"]
        pub fn krb5_string_to_salttype(string: *mut libc::c_char,
                                       salttypep: *mut krb5_int32)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:36"]
pub mod kdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    use super::krb5_h::{krb5_enctype, krb5_int32};
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src = "/usr/include/ctype.h:39"]
pub mod ctype_h {
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed = 256;
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
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed = 8192;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed = 4096;
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed = 2048;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed = 512;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
        #[no_mangle]
        #[c2rust::src_loc = "122:12"]
        pub fn tolower(_: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:35"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:35"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "180:26"]
        pub fn strtoul(_: *const libc::c_char, _: *mut *mut libc::c_char,
                       _: libc::c_int) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/string.h:35"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "345:1"]
        pub fn strtok_r(__s: *mut libc::c_char, __delim: *const libc::c_char,
                        __save_ptr: *mut *mut libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "302:14"]
        pub fn strpbrk(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
pub use self::types_h::__int32_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::krb5_h::{krb5_int32, krb5_boolean, krb5_enctype, krb5_flags,
                       krb5_error_code, krb5_pointer, krb5_string_to_enctype,
                       krb5_string_to_salttype};
pub use self::kdb_h::{__krb5_key_salt_tuple, krb5_key_salt_tuple};
pub use self::ctype_h::{_ISupper, C2RustUnnamed, _ISalnum, _ISpunct, _IScntrl,
                        _ISblank, _ISgraph, _ISprint, _ISspace, _ISxdigit,
                        _ISdigit, _ISalpha, _ISlower, __ctype_b_loc, tolower};
use self::stdio_h::asprintf;
use self::stdlib_h::{free, realloc, strtoul};
use self::string_h::{strtok_r, strpbrk, strdup, strncmp, strcmp};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "44:8"]
pub struct flag_table_row {
    pub spec: *const libc::c_char,
    pub flag: krb5_flags,
    pub invert: libc::c_int,
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/kadm5/str_conv.c */
/*
 * Copyright (C) 1995-2015 by the Massachusetts Institute of Technology.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * * Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.
 *
 * * Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in
 *   the documentation and/or other materials provided with the
 *   distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/* Convert between strings and Kerberos internal data. */
#[c2rust::src_loc = "41:19"]
static mut default_tupleseps: [libc::c_char; 4] = [44, 32, 9, 0];
#[c2rust::src_loc = "42:19"]
static mut default_ksaltseps: [libc::c_char; 2] = [58, 0];
/* Whether to invert the sense */
#[c2rust::src_loc = "50:36"]
static mut ftbl: [flag_table_row; 43] =
    [{
         let mut init =
             flag_table_row{spec:
                                b"allow_postdated\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x1 as libc::c_int,
                            invert: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"postdateable\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x1 as libc::c_int,
                            invert: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"disallow_postdated\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x1 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"allow_forwardable\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x2 as libc::c_int,
                            invert: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"forwardable\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x2 as libc::c_int,
                            invert: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"disallow_forwardable\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x2 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"allow_tgs_req\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x4 as libc::c_int,
                            invert: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"tgt_based\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x4 as libc::c_int,
                            invert: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"disallow_tgt_based\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x4 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"allow_renewable\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x8 as libc::c_int,
                            invert: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"renewable\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x8 as libc::c_int,
                            invert: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"disallow_renewable\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x8 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"allow_proxiable\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x10 as libc::c_int,
                            invert: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"proxiable\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x10 as libc::c_int,
                            invert: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"disallow_proxiable\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x10 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"allow_dup_skey\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x20 as libc::c_int,
                            invert: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"dup_skey\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x20 as libc::c_int,
                            invert: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"disallow_dup_skey\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x20 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"allow_tickets\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x40 as libc::c_int,
                            invert: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"allow_tix\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x40 as libc::c_int,
                            invert: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"disallow_all_tix\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x40 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"preauth\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x80 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"requires_pre_auth\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x80 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"requires_preauth\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x80 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"hwauth\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x100 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"requires_hw_auth\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x100 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"requires_hwauth\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x100 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"needchange\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x200 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"pwchange\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x200 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"requires_pwchange\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x200 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"allow_svr\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x1000 as libc::c_int,
                            invert: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"service\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x1000 as libc::c_int,
                            invert: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"disallow_svr\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x1000 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"password_changing_service\x00" as *const u8
                                    as *const libc::c_char,
                            flag: 0x2000 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"pwchange_service\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x2000 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"pwservice\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x2000 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"md5\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x4000 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"support_desmd5\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x4000 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"new_princ\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x8000 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"ok_as_delegate\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x100000 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"ok_to_auth_as_delegate\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x200000 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"no_auth_data_required\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x400000 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             flag_table_row{spec:
                                b"lockdown_keys\x00" as *const u8 as
                                    *const libc::c_char,
                            flag: 0x800000 as libc::c_int,
                            invert: 0 as libc::c_int,};
         init
     }];
#[c2rust::src_loc = "97:20"]
static mut outflags: [*const libc::c_char; 24] =
    [b"DISALLOW_POSTDATED\x00" as *const u8 as *const libc::c_char,
     b"DISALLOW_FORWARDABLE\x00" as *const u8 as *const libc::c_char,
     b"DISALLOW_TGT_BASED\x00" as *const u8 as *const libc::c_char,
     b"DISALLOW_RENEWABLE\x00" as *const u8 as *const libc::c_char,
     b"DISALLOW_PROXIABLE\x00" as *const u8 as *const libc::c_char,
     b"DISALLOW_DUP_SKEY\x00" as *const u8 as *const libc::c_char,
     b"DISALLOW_ALL_TIX\x00" as *const u8 as *const libc::c_char,
     b"REQUIRES_PRE_AUTH\x00" as *const u8 as *const libc::c_char,
     b"REQUIRES_HW_AUTH\x00" as *const u8 as *const libc::c_char,
     b"REQUIRES_PWCHANGE\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     b"DISALLOW_SVR\x00" as *const u8 as *const libc::c_char,
     b"PWCHANGE_SERVICE\x00" as *const u8 as *const libc::c_char,
     b"SUPPORT_DESMD5\x00" as *const u8 as *const libc::c_char,
     b"NEW_PRINC\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     b"OK_AS_DELEGATE\x00" as *const u8 as *const libc::c_char,
     b"OK_TO_AUTH_AS_DELEGATE\x00" as *const u8 as *const libc::c_char,
     b"NO_AUTH_DATA_REQUIRED\x00" as *const u8 as *const libc::c_char,
     b"LOCKDOWN_KEYS\x00" as *const u8 as *const libc::c_char];
/*
 * Given s, which is a normalized flagspec with the prefix stripped off, and
 * req_neg indicating whether the flagspec is negated, update the toset and
 * toclear masks.
 */
#[c2rust::src_loc = "130:1"]
unsafe extern "C" fn raw_flagspec_to_mask(mut s: *const libc::c_char,
                                          mut req_neg: libc::c_int,
                                          mut toset: *mut krb5_flags,
                                          mut toclear: *mut krb5_flags)
 -> krb5_error_code {
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut invert: libc::c_int = 0 as libc::c_int;
    let mut i: size_t = 0;
    let mut flag: krb5_flags = 0;
    let mut ul: libc::c_ulong = 0;
    i = 0 as libc::c_int as size_t;
    while found == 0 &&
              i <
                  (::std::mem::size_of::<[flag_table_row; 43]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<flag_table_row>()
                                                       as libc::c_ulong) {
        if !(strcmp(s, ftbl[i as usize].spec) != 0 as libc::c_int) {
            /* Found a match */
            found = 1 as libc::c_int;
            invert = ftbl[i as usize].invert;
            flag = ftbl[i as usize].flag
        }
        i = i.wrapping_add(1)
    }
    /* Accept hexadecimal numbers. */
    if found == 0 &&
           strncmp(s, b"0x\x00" as *const u8 as *const libc::c_char,
                   2 as libc::c_int as libc::c_ulong) == 0 as libc::c_int {
        /* Assume that krb5_flags are 32 bits long. */
        ul =
            strtoul(s, 0 as *mut *mut libc::c_char, 16 as libc::c_int) &
                0xffffffff as libc::c_uint as libc::c_ulong;
        flag = ul as krb5_flags;
        found = 1 as libc::c_int
    }
    if found == 0 { return 22 as libc::c_int }
    if req_neg != 0 { invert = (invert == 0) as libc::c_int }
    if invert != 0 { *toclear &= !flag } else { *toset |= flag }
    return 0 as libc::c_int;
}
/* str_conv.c */
/*
 * Update the toset and toclear flag masks according to flag specifier string
 * spec, which is of the form {+|-}flagname.  toset and toclear can point to
 * the same flag word.
 */
#[no_mangle]
#[c2rust::src_loc = "170:1"]
pub unsafe extern "C" fn krb5_flagspec_to_mask(mut spec: *const libc::c_char,
                                               mut toset: *mut krb5_flags,
                                               mut toclear: *mut krb5_flags)
 -> krb5_error_code {
    let mut req_neg: libc::c_int = 0 as libc::c_int;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retval: krb5_error_code = 0;
    copy = strdup(spec);
    s = copy;
    if s.is_null() { return 12 as libc::c_int }
    if *s as libc::c_int == '-' as i32 {
        req_neg = 1 as libc::c_int;
        s = s.offset(1)
    } else if *s as libc::c_int == '+' as i32 { s = s.offset(1) }
    cp = s;
    while *cp as libc::c_int != '\u{0}' as i32 {
        /* Transform hyphens to underscores.*/
        if *cp as libc::c_int == '-' as i32 {
            *cp = '_' as i32 as libc::c_char
        }
        /* Downcase. */
        if *(*__ctype_b_loc()).offset(*cp as libc::c_uchar as libc::c_int as
                                          isize) as libc::c_int &
               _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0 {
            *cp = tolower(*cp as libc::c_uchar as libc::c_int) as libc::c_char
        }
        cp = cp.offset(1)
    }
    retval = raw_flagspec_to_mask(s, req_neg, toset, toclear);
    free(copy as *mut libc::c_void);
    return retval;
}
/*
 * Copy the flag name of flagnum to outstr.  On error, outstr points to a null
 * pointer.
 */
#[no_mangle]
#[c2rust::src_loc = "204:1"]
pub unsafe extern "C" fn krb5_flagnum_to_string(mut flagnum: libc::c_int,
                                                mut outstr:
                                                    *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    *outstr = 0 as *mut libc::c_char;
    if (flagnum as libc::c_uint as libc::c_ulong) <
           (::std::mem::size_of::<[*const libc::c_char; 24]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                as libc::c_ulong) {
        s = outflags[flagnum as usize]
    }
    if s.is_null() {
        /* Assume that krb5_flags are 32 bits long. */
        if asprintf(outstr,
                    b"0x%08lx\x00" as *const u8 as *const libc::c_char,
                    (1 as libc::c_ulong) << flagnum) == -(1 as libc::c_int) {
            *outstr = 0 as *mut libc::c_char
        }
    } else { *outstr = strdup(s) }
    if (*outstr).is_null() { return 12 as libc::c_int }
    return 0 as libc::c_int;
}
/*
 * Create a null-terminated array of string representations of flags.  Store a
 * null pointer into outarray if there would be no strings.
 */
#[no_mangle]
#[c2rust::src_loc = "228:1"]
pub unsafe extern "C" fn krb5_flags_to_strings(mut flags: krb5_int32,
                                               mut outarray:
                                                   *mut *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut a: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut a_new: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ap: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut amax: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    let mut retval: krb5_error_code = 0;
    *outarray = 0 as *mut *mut libc::c_char;
    /* Assume that krb5_flags are 32 bits long. */
    i = 0 as libc::c_int as size_t;
    loop  {
        if !(i < 32 as libc::c_int as libc::c_ulong) {
            current_block = 12800627514080957624;
            break ;
        }
        if !(flags as libc::c_ulong & (1 as libc::c_ulong) << i == 0) {
            a_new =
                realloc(a as *mut libc::c_void,
                        amax.wrapping_add(2 as libc::c_int as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                                              as
                                                                              libc::c_ulong))
                    as *mut *mut libc::c_char;
            if a_new.is_null() {
                retval = 12 as libc::c_int;
                current_block = 3431373866633067287;
                break ;
            } else {
                a = a_new;
                let fresh0 = amax;
                amax = amax.wrapping_add(1);
                retval =
                    krb5_flagnum_to_string(i as libc::c_int,
                                           &mut *a.offset(fresh0 as isize));
                let ref mut fresh1 = *a.offset(amax as isize);
                *fresh1 = 0 as *mut libc::c_char;
                if retval != 0 {
                    current_block = 3431373866633067287;
                    break ;
                }
            }
        }
        i = i.wrapping_add(1)
    }
    match current_block {
        3431373866633067287 => {
            ap = a;
            while !ap.is_null() && !(*ap).is_null() {
                free(*ap as *mut libc::c_void);
                ap = ap.offset(1)
            }
            free(a as *mut libc::c_void);
            return retval
        }
        _ => { *outarray = a; return 0 as libc::c_int }
    };
}
/* keysalt.c */
/*
 * krb5_keysalt_is_present()    - Determine if a key/salt pair is present
 *                                in a list of key/salt tuples.
 *
 *      Salttype may be negative to indicate a search for only a enctype.
 */
#[no_mangle]
#[c2rust::src_loc = "269:1"]
pub unsafe extern "C" fn krb5_keysalt_is_present(mut ksaltlist:
                                                     *mut krb5_key_salt_tuple,
                                                 mut nksalts: krb5_int32,
                                                 mut enctype: krb5_enctype,
                                                 mut salttype: krb5_int32)
 -> krb5_boolean {
    let mut foundit: krb5_boolean = 0;
    let mut i: libc::c_int = 0;
    foundit = 0 as libc::c_int as krb5_boolean;
    if !ksaltlist.is_null() {
        i = 0 as libc::c_int;
        while i < nksalts {
            if (*ksaltlist.offset(i as isize)).ks_enctype == enctype &&
                   ((*ksaltlist.offset(i as isize)).ks_salttype == salttype ||
                        salttype < 0 as libc::c_int) {
                foundit = 1 as libc::c_int as krb5_boolean;
                break ;
            } else { i += 1 }
        }
    }
    return foundit;
}
/* NOTE: This is a destructive parser (writes NULs). */
#[c2rust::src_loc = "294:1"]
unsafe extern "C" fn string_to_keysalt(mut s: *mut libc::c_char,
                                       mut ksaltseps: *const libc::c_char,
                                       mut etype: *mut krb5_enctype,
                                       mut stype: *mut krb5_int32)
 -> krb5_error_code {
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ksseps: *const libc::c_char =
        if !ksaltseps.is_null() {
            ksaltseps
        } else { default_ksaltseps.as_ptr() };
    let mut ret: krb5_error_code = 0 as libc::c_int;
    sp = strpbrk(s, ksseps);
    if !sp.is_null() {
        let fresh2 = sp;
        sp = sp.offset(1);
        *fresh2 = '\u{0}' as i32 as libc::c_char
    }
    ret = krb5_string_to_enctype(s, etype);
    if ret != 0 { return ret }
    /* Default to normal salt if omitted. */
    *stype = 0 as libc::c_int;
    if sp.is_null() { return 0 as libc::c_int }
    return krb5_string_to_salttype(sp, stype);
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved
 *
 */
/*
 * _KADM5_CHECK_HANDLE calls the function _kadm5_check_handle and
 * returns any non-zero error code that function returns.
 * _kadm5_check_handle, in client_handle.c and server_handle.c, exists
 * in both the server- and client- side libraries.  In each library,
 * it calls CHECK_HANDLE, which is defined by the appropriate
 * _internal.h header file to call GENERIC_CHECK_HANDLE as well as
 * CLIENT_CHECK_HANDLE and SERVER_CHECK_HANDLE.
 *
 * _KADM5_CHECK_HANDLE should be used by a function that needs to
 * check the handle but wants to be the same code in both the client
 * and server library; it makes a function call to the right handle
 * checker.  Code that only exists in one library can call the
 * CHECK_HANDLE macro, which inlines the test instead of making
 * another function call.
 *
 * Got that?
 */
/* this is needed by the alt_prof code I stole.  The functions
   maybe shouldn't be named krb5_*, but they are. */
/*
 * krb5_string_to_keysalts()    - Convert a string representation to a list
 *                                of key/salt tuples.
 */
#[no_mangle]
#[c2rust::src_loc = "321:1"]
pub unsafe extern "C" fn krb5_string_to_keysalts(mut string:
                                                     *const libc::c_char,
                                                 mut tupleseps:
                                                     *const libc::c_char,
                                                 mut ksaltseps:
                                                     *const libc::c_char,
                                                 mut dups: krb5_boolean,
                                                 mut ksaltp:
                                                     *mut *mut krb5_key_salt_tuple,
                                                 mut nksaltp: *mut krb5_int32)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ksp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tlasts: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tseps: *const libc::c_char =
        if !tupleseps.is_null() {
            tupleseps
        } else { default_tupleseps.as_ptr() };
    let mut nksalts: krb5_int32 = 0 as libc::c_int;
    let mut stype: krb5_int32 = 0;
    let mut etype: krb5_enctype = 0;
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut ksalts: *mut krb5_key_salt_tuple = 0 as *mut krb5_key_salt_tuple;
    let mut ksalts_new: *mut krb5_key_salt_tuple =
        0 as *mut krb5_key_salt_tuple;
    *ksaltp = 0 as *mut krb5_key_salt_tuple;
    *nksaltp = 0 as libc::c_int;
    copy = strdup(string);
    p = copy;
    if p.is_null() { return 12 as libc::c_int }
    loop  {
        ksp = strtok_r(p, tseps, &mut tlasts);
        if ksp.is_null() { current_block = 224731115979188411; break ; }
        /* Pass a null pointer to subsequent calls to strtok_r(). */
        p = 0 as *mut libc::c_char;
        ret = string_to_keysalt(ksp, ksaltseps, &mut etype, &mut stype);
        if ret != 0 { current_block = 13603524405384418843; break ; }
        /* Ignore duplicate keysalts if caller asks. */
        if dups == 0 &&
               krb5_keysalt_is_present(ksalts, nksalts, etype, stype) != 0 {
            continue ;
        }
        ksalts_new =
            realloc(ksalts as *mut libc::c_void,
                    ((nksalts + 1 as libc::c_int) as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_key_salt_tuple>()
                                                         as libc::c_ulong)) as
                *mut krb5_key_salt_tuple;
        if ksalts_new.is_null() {
            ret = 12 as libc::c_int;
            current_block = 13603524405384418843;
            break ;
        } else {
            ksalts = ksalts_new;
            (*ksalts.offset(nksalts as isize)).ks_enctype = etype;
            (*ksalts.offset(nksalts as isize)).ks_salttype = stype;
            nksalts += 1
        }
    }
    match current_block {
        224731115979188411 => { *ksaltp = ksalts; *nksaltp = nksalts }
        _ => { }
    }
    if ret != 0 { free(ksalts as *mut libc::c_void); }
    free(copy as *mut libc::c_void);
    return ret;
}
/*
 * krb5_keysalt_iterate()       - Do something for each unique key/salt
 *                                combination.
 *
 * If ignoresalt set, then salttype is ignored.
 */
#[no_mangle]
#[c2rust::src_loc = "376:1"]
pub unsafe extern "C" fn krb5_keysalt_iterate(mut ksaltlist:
                                                  *mut krb5_key_salt_tuple,
                                              mut nksalt: krb5_int32,
                                              mut ignoresalt: krb5_boolean,
                                              mut iterator:
                                                  Option<unsafe extern "C" fn(_:
                                                                                  *mut krb5_key_salt_tuple,
                                                                              _:
                                                                                  krb5_pointer)
                                                             ->
                                                                 krb5_error_code>,
                                              mut arg: krb5_pointer)
 -> krb5_error_code {
    let mut i: libc::c_int = 0;
    let mut kret: krb5_error_code = 0;
    let mut scratch: krb5_key_salt_tuple =
        krb5_key_salt_tuple{ks_enctype: 0, ks_salttype: 0,};
    kret = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nksalt {
        scratch.ks_enctype = (*ksaltlist.offset(i as isize)).ks_enctype;
        scratch.ks_salttype =
            if ignoresalt != 0 {
                -(1 as libc::c_int)
            } else { (*ksaltlist.offset(i as isize)).ks_salttype };
        if krb5_keysalt_is_present(ksaltlist, i, scratch.ks_enctype,
                                   scratch.ks_salttype) == 0 {
            kret =
                Some(iterator.expect("non-null function pointer")).expect("non-null function pointer")(&mut scratch,
                                                                                                       arg);
            if kret != 0 { break ; }
        }
        i += 1
    }
    return kret;
}
