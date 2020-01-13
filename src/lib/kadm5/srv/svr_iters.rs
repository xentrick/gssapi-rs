use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/include/bits/types.h:19"]
pub mod types_h {
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
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:19"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:19"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:21"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:21"]
pub mod krb5_h {
    /* typedef struct _profile_t *profile_t; */
    /*
 * begin wordsize.h
 */
    /*
 * Word-size related definition.
 */
    #[c2rust::src_loc = "136:1"]
    pub type krb5_octet = uint8_t;
    #[c2rust::src_loc = "137:1"]
    pub type krb5_int16 = int16_t;
    #[c2rust::src_loc = "138:1"]
    pub type krb5_ui_2 = uint16_t;
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
    #[c2rust::src_loc = "176:1"]
    pub type krb5_kvno = libc::c_uint;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    /* This may change, later on */
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "228:16"]
    pub struct krb5_principal_data {
        pub magic: krb5_magic,
        pub realm: krb5_data,
        pub data: *mut krb5_data,
        pub length: krb5_int32,
        pub type_0: krb5_int32,
    }
    #[c2rust::src_loc = "236:1"]
    pub type krb5_principal = *mut krb5_principal_data;
    /*
 * Per V5 spec on definition of principal types
 */
    /* *<  Name type not known */
    /* *< Just the name of the principal
                                      as in DCE, or for users */
    /* *< Service and other unique instance (krbtgt) */
    /* *< Service with host name as instance
                                      (telnet, rcommands) */
    /* *< Service with host as remaining components */
    /* *< Unique ID */
    /* *< PKINIT */
    /* *< Name in form of SMTP email name */
    /* *< Windows 2000 UPN */
    /* *< Well-known (special) principal */
    /* *< First component of
                                                NT_WELLKNOWN principals */
    /* *< Windows 2000 UPN and SID */
    /* *< NT 4 style name */
    /* *< NT 4 style name and SID */
    /* * Constant version of krb5_principal_data */
    #[c2rust::src_loc = "261:1"]
    pub type krb5_const_principal = *const krb5_principal_data;
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    extern "C" {
        /* per Kerberos v5 protocol spec */
        /* not yet in the spec... */
        /* macros to determine if a type is a local type */
        /*
 * end "hostaddr.h"
 */
        #[c2rust::src_loc = "350:8"]
        pub type _krb5_context;
        /* *
 * Convert a krb5_principal structure to a string representation.
 *
 * @param [in]  context         Library context
 * @param [in]  principal       Principal
 * @param [out] name            String representation of principal name
 *
 * The resulting string representation uses the format and quoting conventions
 * described for krb5_parse_name().
 *
 * Use krb5_free_unparsed_name() to free @a name when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3489:1"]
        pub fn krb5_unparse_name(context: krb5_context,
                                 principal: krb5_const_principal,
                                 name: *mut *mut libc::c_char)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:21"]
pub mod kdb_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1990, 1991, 2016 by the Massachusetts Institute of Technology.
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
 * Copyright (C) 1998 by the FundsXpress, INC.
 *
 * All rights reserved.
 *
 * Export of this software from the United States of America may require
 * a specific license from the United States Government.  It is the
 * responsibility of any person or organization contemplating export to
 * obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of FundsXpress. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  FundsXpress makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 *
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND WITHOUT ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED
 * WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
 */
/*
 * Copyright 2009 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
    /* KDC Database interface definitions */
    /* This API is not considered as stable as the main krb5 API.
 *
 * - We may make arbitrary incompatible changes between feature
 *   releases (e.g. from 1.7 to 1.8).
 * - We will make some effort to avoid making incompatible changes for
 *   bugfix releases, but will make them if necessary.
 */
    /* This version will be incremented when incompatible changes are made to the
 * KDB API, and will be kept in sync with the libkdb major version. */
    /* Salt types */
    /* #define KRB5_KDB_SALTTYPE_V4            1 */
    /* #define KRB5_KDB_SALTTYPE_AFS3          5 */
    /* Attributes */
    /* S4U2Self OK */
    /* Creation flags */
    /* Entry get flags */
/* Name canonicalization requested */
    /* Include authorization data generated by backend */
    /* Is AS-REQ (client referrals only) */
    /* Map cross-realm principals */
    /* Protocol transition */
    /* Constrained delegation */
    /* User-to-user */
    /* Cross-realm */
    /* Issuing referral */
    /* KDB iteration flags */
    /* String attribute names recognized by krb5 */
    /*
 * Note --- these structures cannot be modified without changing the
 * database version number in libkdb.a, but should be expandable by
 * adding new tl_data types.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "147:16"]
    pub struct _krb5_tl_data {
        pub tl_data_next: *mut _krb5_tl_data,
        pub tl_data_type: krb5_int16,
        pub tl_data_length: krb5_ui_2,
        pub tl_data_contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "147:1"]
    pub type krb5_tl_data = _krb5_tl_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "215:16"]
    pub struct _osa_policy_ent_t {
        pub version: libc::c_int,
        pub name: *mut libc::c_char,
        pub pw_min_life: krb5_ui_4,
        pub pw_max_life: krb5_ui_4,
        pub pw_min_length: krb5_ui_4,
        pub pw_min_classes: krb5_ui_4,
        pub pw_history_num: krb5_ui_4,
        pub policy_refcnt: krb5_ui_4,
        pub pw_max_fail: krb5_ui_4,
        pub pw_failcnt_interval: krb5_ui_4,
        pub pw_lockout_duration: krb5_ui_4,
        pub attributes: krb5_ui_4,
        pub max_life: krb5_ui_4,
        pub max_renewable_life: krb5_ui_4,
        pub allowed_keysalts: *mut libc::c_char,
        pub n_tl_data: krb5_int16,
        pub tl_data: *mut krb5_tl_data,
    }
    #[c2rust::src_loc = "215:1"]
    pub type osa_policy_ent_t = *mut _osa_policy_ent_t;
    #[c2rust::src_loc = "237:1"]
    pub type osa_adb_iter_policy_func
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: osa_policy_ent_t)
                   -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    use super::krb5_h::{krb5_int16, krb5_ui_2, krb5_octet, krb5_ui_4,
                        krb5_enctype, krb5_int32, _krb5_context, krb5_context,
                        krb5_error_code};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "827:1"]
        pub fn krb5_db_iter_policy(kcontext: krb5_context,
                                   match_entry: *mut libc::c_char,
                                   func: osa_adb_iter_policy_func,
                                   data: *mut libc::c_void)
         -> krb5_error_code;
    }
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:21"]
pub mod admin_h {
    #[c2rust::src_loc = "71:1"]
    pub type kadm5_ret_t = libc::c_long;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "241:16"]
    pub struct _kadm5_config_params {
        pub mask: libc::c_long,
        pub realm: *mut libc::c_char,
        pub kadmind_port: libc::c_int,
        pub kpasswd_port: libc::c_int,
        pub admin_server: *mut libc::c_char,
        pub dbname: *mut libc::c_char,
        pub acl_file: *mut libc::c_char,
        pub dict_file: *mut libc::c_char,
        pub mkey_from_kbd: libc::c_int,
        pub stash_file: *mut libc::c_char,
        pub mkey_name: *mut libc::c_char,
        pub enctype: krb5_enctype,
        pub max_life: krb5_deltat,
        pub max_rlife: krb5_deltat,
        pub expiration: krb5_timestamp,
        pub flags: krb5_flags,
        pub keysalts: *mut krb5_key_salt_tuple,
        pub num_keysalts: krb5_int32,
        pub kvno: krb5_kvno,
        pub iprop_enabled: libc::c_int,
        pub iprop_ulogsize: uint32_t,
        pub iprop_poll_time: krb5_deltat,
        pub iprop_logfile: *mut libc::c_char,
        pub iprop_port: libc::c_int,
        pub iprop_resync_timeout: libc::c_int,
        pub kadmind_listen: *mut libc::c_char,
        pub kpasswd_listen: *mut libc::c_char,
        pub iprop_listen: *mut libc::c_char,
    }
    #[c2rust::src_loc = "241:1"]
    pub type kadm5_config_params = _kadm5_config_params;
    use super::krb5_h::{krb5_enctype, krb5_deltat, krb5_timestamp, krb5_flags,
                        krb5_int32, krb5_kvno};
    use super::kdb_h::krb5_key_salt_tuple;
    use super::stdint_uintn_h::uint32_t;
    /* __KADM5_ADMIN_H__ */
}
#[c2rust::header_src = "/usr/include/regex.h:26"]
pub mod regex_h {
    #[c2rust::src_loc = "478:1"]
    pub type regex_t = re_pattern_buffer;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "413:8"]
    pub struct re_pattern_buffer {
        pub buffer: *mut re_dfa_t,
        pub allocated: __re_long_size_t,
        pub used: __re_long_size_t,
        pub syntax: reg_syntax_t,
        pub fastmap: *mut libc::c_char,
        pub translate: *mut libc::c_uchar,
        pub re_nsub: size_t,
        #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
        #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits =
                   "1..=2")]
        #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits =
                   "3..=3")]
        #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
        #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
        #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
        #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits =
                   "7..=7")]
        pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
    }
    #[c2rust::src_loc = "72:1"]
    pub type reg_syntax_t = libc::c_ulong;
    #[c2rust::src_loc = "56:1"]
    pub type __re_long_size_t = libc::c_ulong;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "517:9"]
    pub struct regmatch_t {
        pub rm_so: regoff_t,
        pub rm_eo: regoff_t,
    }
    #[c2rust::src_loc = "490:1"]
    pub type regoff_t = libc::c_int;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "417:10"]
        pub type re_dfa_t;
        #[no_mangle]
        #[c2rust::src_loc = "639:1"]
        pub fn regcomp(__preg: *mut regex_t, __pattern: *const libc::c_char,
                       __cflags: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "643:1"]
        pub fn regexec(__preg: *const regex_t, __String: *const libc::c_char,
                       __nmatch: size_t, __pmatch: *mut regmatch_t,
                       __eflags: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "651:1"]
        pub fn regfree(__preg: *mut regex_t);
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/server_internal.h:30"]
pub mod server_internal_h {
    #[c2rust::src_loc = "42:1"]
    pub type kadm5_server_handle_t = *mut _kadm5_server_handle_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "42:16"]
    pub struct _kadm5_server_handle_t {
        pub magic_number: krb5_ui_4,
        pub struct_version: krb5_ui_4,
        pub api_version: krb5_ui_4,
        pub context: krb5_context,
        pub current_caller: krb5_principal,
        pub params: kadm5_config_params,
        pub lhandle: *mut _kadm5_server_handle_t,
        pub db_args: *mut *mut libc::c_char,
        pub qual_handles: *mut pwqual_handle,
        pub hook_handles: *mut kadm5_hook_handle,
    }
    #[c2rust::src_loc = "40:1"]
    pub type kadm5_hook_handle = *mut kadm5_hook_handle_st;
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved
 *
 * $Header$
 */
    /*
 * This header file is used internally by the Admin API server
 * libraries and Admin server.  IF YOU THINK YOU NEED TO USE THIS FILE
 * FOR ANYTHING, YOU'RE ALMOST CERTAINLY WRONG.
 */
    /*
 * This is the history key version for a newly created DB.  We use this value
 * for principals which have no password history yet to avoid having to look up
 * the history key.  Values other than 2 will cause compatibility issues with
 * pre-1.8 libkadm5 code; the older code will reject key changes when it sees
 * an unexpected value of admin_history_kvno.
 */
    /* A pwqual_handle represents a password quality plugin module. */
    #[c2rust::src_loc = "38:1"]
    pub type pwqual_handle = *mut pwqual_handle_st;
    use super::krb5_h::{krb5_ui_4, krb5_context, krb5_principal,
                        krb5_error_code};
    use super::admin_h::kadm5_config_params;
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type kadm5_hook_handle_st;
        #[c2rust::src_loc = "38:16"]
        pub type pwqual_handle_st;
        #[no_mangle]
        #[c2rust::src_loc = "98:1"]
        pub fn kdb_iter_entry(handle: kadm5_server_handle_t,
                              match_entry: *mut libc::c_char,
                              iter_fct:
                                  Option<unsafe extern "C" fn(_:
                                                                  *mut libc::c_void,
                                                              _:
                                                                  krb5_principal)
                                             -> ()>, data: *mut libc::c_void)
         -> krb5_error_code;
    }
    /* __KADM5_SERVER_INTERNAL_H__ */
    /* * @}*/
}
#[c2rust::header_src = "/usr/include/string.h:20"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "225:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:21"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_kvno, krb5_enctype, krb5_flags,
                       krb5_timestamp, krb5_deltat, krb5_error_code,
                       krb5_magic, _krb5_data, krb5_data, krb5_principal_data,
                       krb5_principal, krb5_const_principal, krb5_context,
                       _krb5_context, krb5_unparse_name};
pub use self::kdb_h::{_krb5_tl_data, krb5_tl_data, _osa_policy_ent_t,
                      osa_policy_ent_t, osa_adb_iter_policy_func,
                      __krb5_key_salt_tuple, krb5_key_salt_tuple,
                      krb5_db_iter_policy};
pub use self::admin_h::{kadm5_ret_t, _kadm5_config_params,
                        kadm5_config_params};
pub use self::regex_h::{regex_t, re_pattern_buffer, reg_syntax_t,
                        __re_long_size_t, regmatch_t, regoff_t, re_dfa_t,
                        regcomp, regexec, regfree};
pub use self::server_internal_h::{kadm5_server_handle_t,
                                  _kadm5_server_handle_t, kadm5_hook_handle,
                                  pwqual_handle, kadm5_hook_handle_st,
                                  pwqual_handle_st, kdb_iter_entry};
use self::string_h::{strdup, strchr, strlen};
use self::stdlib_h::{malloc, realloc, free};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved
 *
 * $Header$
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "32:8"]
pub struct iter_data {
    pub context: krb5_context,
    pub names: *mut *mut libc::c_char,
    pub n_names: libc::c_int,
    pub sz_names: libc::c_int,
    pub malloc_failed: libc::c_uint,
    pub exp: *mut libc::c_char,
    pub preg: regex_t,
}
/* XXX Duplicated in kdb5_util!  */
/*
 * Function: glob_to_regexp
 *
 * Arguments:
 *
 *      glob    (r) the shell-style glob (?*[]) to convert
 *      realm   (r) the default realm to append, or NULL
 *      regexp  (w) the ed-style regexp created from glob
 *
 * Effects:
 *
 * regexp is filled in with allocated memory contained a regular
 * expression to be used with re_comp/compile that matches what the
 * shell-style glob would match.  If glob does not contain an "@"
 * character and realm is not NULL, "@*" is appended to the regexp.
 *
 * Conversion algorithm:
 *
 *      quoted characters are copied quoted
 *      ? is converted to .
 *      * is converted to .*
 *      active characters are quoted: ^, $, .
 *      [ and ] are active but supported and have the same meaning, so
 *              they are copied
 *      other characters are copied
 *      regexp is anchored with ^ and $
 */
#[c2rust::src_loc = "74:1"]
unsafe extern "C" fn glob_to_regexp(mut glob: *mut libc::c_char,
                                    mut realm: *mut libc::c_char,
                                    mut regexp: *mut *mut libc::c_char)
 -> kadm5_ret_t {
    let mut append_realm: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    /* validate the glob */
    if *glob.offset(strlen(glob).wrapping_sub(1 as libc::c_int as
                                                  libc::c_ulong) as isize) as
           libc::c_int == '\\' as i32 {
        return 22 as libc::c_int as kadm5_ret_t
    }
    /* A character of glob can turn into two in regexp, plus ^ and $ */
    /* and trailing null.  If glob has no @, also allocate space for */
    /* the realm. */
    append_realm =
        (!realm.is_null() && strchr(glob, '@' as i32).is_null()) as
            libc::c_int;
    p =
        malloc(strlen(glob).wrapping_mul(2 as libc::c_int as
                                             libc::c_ulong).wrapping_add(3 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_ulong).wrapping_add((if append_realm
                                                                                                                 !=
                                                                                                                 0
                                                                                                             {
                                                                                                              3
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                          } else {
                                                                                                              0
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                          })
                                                                                                             as
                                                                                                             libc::c_ulong))
            as *mut libc::c_char;
    if p.is_null() { return 12 as libc::c_int as kadm5_ret_t }
    *regexp = p;
    let fresh0 = p;
    p = p.offset(1);
    *fresh0 = '^' as i32 as libc::c_char;
    while *glob != 0 {
        match *glob as libc::c_int {
            63 => {
                let fresh1 = p;
                p = p.offset(1);
                *fresh1 = '.' as i32 as libc::c_char
            }
            42 => {
                let fresh2 = p;
                p = p.offset(1);
                *fresh2 = '.' as i32 as libc::c_char;
                let fresh3 = p;
                p = p.offset(1);
                *fresh3 = '*' as i32 as libc::c_char
            }
            46 | 94 | 36 => {
                let fresh4 = p;
                p = p.offset(1);
                *fresh4 = '\\' as i32 as libc::c_char;
                let fresh5 = p;
                p = p.offset(1);
                *fresh5 = *glob
            }
            92 => {
                let fresh6 = p;
                p = p.offset(1);
                *fresh6 = '\\' as i32 as libc::c_char;
                glob = glob.offset(1);
                let fresh7 = p;
                p = p.offset(1);
                *fresh7 = *glob
            }
            _ => { let fresh8 = p; p = p.offset(1); *fresh8 = *glob }
        }
        glob = glob.offset(1)
    }
    if append_realm != 0 {
        let fresh9 = p;
        p = p.offset(1);
        *fresh9 = '@' as i32 as libc::c_char;
        let fresh10 = p;
        p = p.offset(1);
        *fresh10 = '.' as i32 as libc::c_char;
        let fresh11 = p;
        p = p.offset(1);
        *fresh11 = '*' as i32 as libc::c_char
    }
    let fresh12 = p;
    p = p.offset(1);
    *fresh12 = '$' as i32 as libc::c_char;
    let fresh13 = p;
    p = p.offset(1);
    *fresh13 = '\u{0}' as i32 as libc::c_char;
    return 0 as libc::c_int as kadm5_ret_t;
}
#[c2rust::src_loc = "130:1"]
unsafe extern "C" fn get_either_iter(mut data: *mut iter_data,
                                     mut name: *mut libc::c_char) {
    let mut match_0: libc::c_int = 0;
    match_0 =
        (regexec(&mut (*data).preg, name, 0 as libc::c_int as size_t,
                 0 as *mut regmatch_t, 0 as libc::c_int) == 0 as libc::c_int)
            as libc::c_int;
    if match_0 != 0 {
        if (*data).n_names == (*data).sz_names {
            let mut new_sz: libc::c_int = (*data).sz_names * 2 as libc::c_int;
            let mut new_names: *mut *mut libc::c_char =
                realloc((*data).names as *mut libc::c_void,
                        (new_sz as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                             as
                                                             libc::c_ulong))
                    as *mut *mut libc::c_char;
            if !new_names.is_null() {
                (*data).names = new_names;
                (*data).sz_names = new_sz
            } else {
                (*data).malloc_failed = 1 as libc::c_int as libc::c_uint;
                free(name as *mut libc::c_void);
                return
            }
        }
        let fresh14 = (*data).n_names;
        (*data).n_names = (*data).n_names + 1;
        let ref mut fresh15 = *(*data).names.offset(fresh14 as isize);
        *fresh15 = name
    } else { free(name as *mut libc::c_void); };
}
#[c2rust::src_loc = "161:1"]
unsafe extern "C" fn get_pols_iter(mut data: *mut libc::c_void,
                                   mut entry: osa_policy_ent_t) {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    name = strdup((*entry).name);
    if name.is_null() { return }
    get_either_iter(data as *mut iter_data, name);
}
#[c2rust::src_loc = "170:1"]
unsafe extern "C" fn get_princs_iter(mut data: *mut libc::c_void,
                                     mut princ: krb5_principal) {
    let mut id: *mut iter_data = data as *mut iter_data;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    if krb5_unparse_name((*id).context, princ as krb5_const_principal,
                         &mut name) != 0 as libc::c_int {
        return
    }
    get_either_iter(data as *mut iter_data, name);
}
#[c2rust::src_loc = "180:1"]
unsafe extern "C" fn kadm5_get_either(mut princ: libc::c_int,
                                      mut server_handle: *mut libc::c_void,
                                      mut exp: *mut libc::c_char,
                                      mut princs: *mut *mut *mut libc::c_char,
                                      mut count: *mut libc::c_int)
 -> kadm5_ret_t {
    let mut data: iter_data =
        iter_data{context: 0 as *mut _krb5_context,
                  names: 0 as *mut *mut libc::c_char,
                  n_names: 0,
                  sz_names: 0,
                  malloc_failed: 0,
                  exp: 0 as *mut libc::c_char,
                  preg:
                      regex_t{buffer: 0 as *mut re_dfa_t,
                              allocated: 0,
                              used: 0,
                              syntax: 0,
                              fastmap: 0 as *mut libc::c_char,
                              translate: 0 as *mut libc::c_uchar,
                              re_nsub: 0,
                              can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor:
                                  [0; 1],
                              c2rust_padding: [0; 7],},};
    let mut regexp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    *princs = 0 as *mut *mut libc::c_char;
    *count = 0 as libc::c_int;
    if exp.is_null() {
        exp =
            b"*\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    let mut srvr: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787557 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787559 as libc::c_long
    }
    let mut srvr_0: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if (*srvr_0).current_caller.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).lhandle.is_null() { return 43787551 as libc::c_long }
    ret =
        glob_to_regexp(exp,
                       (if princ != 0 {
                            (*handle).params.realm
                        } else { 0 as *mut libc::c_char }), &mut regexp) as
            libc::c_int;
    if ret != 0 as libc::c_int { return ret as kadm5_ret_t }
    if regcomp(&mut data.preg, regexp, (1 as libc::c_int) << 3 as libc::c_int)
           != 0 as libc::c_int {
        /* XXX syslog msg or regerr(regerrno) */
        free(regexp as *mut libc::c_void);
        return 22 as libc::c_int as kadm5_ret_t
    }
    data.n_names = 0 as libc::c_int;
    data.sz_names = 10 as libc::c_int;
    data.malloc_failed = 0 as libc::c_int as libc::c_uint;
    data.names =
        malloc((::std::mem::size_of::<*mut libc::c_char>() as
                    libc::c_ulong).wrapping_mul(data.sz_names as
                                                    libc::c_ulong)) as
            *mut *mut libc::c_char;
    if data.names.is_null() {
        free(regexp as *mut libc::c_void);
        return 12 as libc::c_int as kadm5_ret_t
    }
    if princ != 0 {
        data.context = (*handle).context;
        ret =
            kdb_iter_entry(handle, exp,
                           Some(get_princs_iter as
                                    unsafe extern "C" fn(_: *mut libc::c_void,
                                                         _: krb5_principal)
                                        -> ()),
                           &mut data as *mut iter_data as *mut libc::c_void)
    } else {
        ret =
            krb5_db_iter_policy((*handle).context, exp,
                                Some(get_pols_iter as
                                         unsafe extern "C" fn(_:
                                                                  *mut libc::c_void,
                                                              _:
                                                                  osa_policy_ent_t)
                                             -> ()),
                                &mut data as *mut iter_data as
                                    *mut libc::c_void)
    }
    free(regexp as *mut libc::c_void);
    regfree(&mut data.preg);
    if ret == 0 && data.malloc_failed != 0 { ret = 12 as libc::c_int }
    if ret != 0 {
        i = 0 as libc::c_int;
        while i < data.n_names {
            free(*data.names.offset(i as isize) as *mut libc::c_void);
            i += 1
        }
        free(data.names as *mut libc::c_void);
        return ret as kadm5_ret_t
    }
    *princs = data.names;
    *count = data.n_names;
    return 0 as libc::c_int as kadm5_ret_t;
}
#[no_mangle]
#[c2rust::src_loc = "256:1"]
pub unsafe extern "C" fn kadm5_get_principals(mut server_handle:
                                                  *mut libc::c_void,
                                              mut exp: *mut libc::c_char,
                                              mut princs:
                                                  *mut *mut *mut libc::c_char,
                                              mut count: *mut libc::c_int)
 -> kadm5_ret_t {
    return kadm5_get_either(1 as libc::c_int, server_handle, exp, princs,
                            count);
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/kadm5/admin.h */
/*
 * Copyright 2001, 2008 by the Massachusetts Institute of Technology.
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
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved
 *
 * $Header$
 */
/*
 * This API is not considered as stable as the main krb5 API.
 *
 * - We may make arbitrary incompatible changes between feature
 *   releases (e.g. from 1.7 to 1.8).
 * - We will make some effort to avoid making incompatible changes for
 *   bugfix releases, but will make them if necessary.
 */
/*
 * Successful return code
 */
/*
 * Field masks
 */
/* kadm5_principal_ent_t */
/* version 2 masks */
/* Novell */
/* all but KEY_DATA, TL_DATA, LOAD */
/* kadm5_policy_ent_t */
/* kadm5_config_params */
/*#define KADM5_CONFIG_ADMIN_KEYTAB       0x00000080*/
/*
 * permission bits
 */
/*
 * API versioning constants
 */
/* version 2 fields */
/* no longer used */
/* version 3 fields */
/* version 4 fields */
/*
 * Data structure returned by kadm5_get_config_params()
 */
/* Novell */ /* ABI change? */
/* Deprecated except for db2 backwards compatibility.  Don't add
       new uses except as fallbacks for parameters that should be
       specified in the database module section of the config
       file.  */
/*    char *            iprop_server;*/
/*
 * functions
 */
/*
 * For all initialization functions, the caller must first initialize
 * a context with kadm5_init_krb5_context which will survive as long
 * as the resulting handle.  The caller should free the context with
 * krb5_free_context.
 */
#[no_mangle]
#[c2rust::src_loc = "264:1"]
pub unsafe extern "C" fn kadm5_get_policies(mut server_handle:
                                                *mut libc::c_void,
                                            mut exp: *mut libc::c_char,
                                            mut pols:
                                                *mut *mut *mut libc::c_char,
                                            mut count: *mut libc::c_int)
 -> kadm5_ret_t {
    return kadm5_get_either(0 as libc::c_int, server_handle, exp, pols,
                            count);
}
