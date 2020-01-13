use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:34"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:35"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/regex.h:37"]
pub mod regex_h {
    #[c2rust::src_loc = "56:1"]
    pub type __re_long_size_t = libc::c_ulong;
    #[c2rust::src_loc = "72:1"]
    pub type reg_syntax_t = libc::c_ulong;
    #[c2rust::src_loc = "346:9"]
    pub type C2RustUnnamed = libc::c_int;
    #[c2rust::src_loc = "370:3"]
    pub const _REG_ERPAREN: C2RustUnnamed = 16;
    #[c2rust::src_loc = "369:3"]
    pub const _REG_ESIZE: C2RustUnnamed = 15;
    #[c2rust::src_loc = "368:3"]
    pub const _REG_EEND: C2RustUnnamed = 14;
    #[c2rust::src_loc = "365:3"]
    pub const _REG_BADRPT: C2RustUnnamed = 13;
    #[c2rust::src_loc = "364:3"]
    pub const _REG_ESPACE: C2RustUnnamed = 12;
    #[c2rust::src_loc = "363:3"]
    pub const _REG_ERANGE: C2RustUnnamed = 11;
    #[c2rust::src_loc = "362:3"]
    pub const _REG_BADBR: C2RustUnnamed = 10;
    #[c2rust::src_loc = "361:3"]
    pub const _REG_EBRACE: C2RustUnnamed = 9;
    #[c2rust::src_loc = "360:3"]
    pub const _REG_EPAREN: C2RustUnnamed = 8;
    #[c2rust::src_loc = "359:3"]
    pub const _REG_EBRACK: C2RustUnnamed = 7;
    #[c2rust::src_loc = "358:3"]
    pub const _REG_ESUBREG: C2RustUnnamed = 6;
    #[c2rust::src_loc = "357:3"]
    pub const _REG_EESCAPE: C2RustUnnamed = 5;
    #[c2rust::src_loc = "356:3"]
    pub const _REG_ECTYPE: C2RustUnnamed = 4;
    #[c2rust::src_loc = "355:3"]
    pub const _REG_ECOLLATE: C2RustUnnamed = 3;
    #[c2rust::src_loc = "354:3"]
    pub const _REG_BADPAT: C2RustUnnamed = 2;
    #[c2rust::src_loc = "350:3"]
    pub const _REG_NOMATCH: C2RustUnnamed = 1;
    #[c2rust::src_loc = "349:3"]
    pub const _REG_NOERROR: C2RustUnnamed = 0;
    #[c2rust::src_loc = "348:3"]
    pub const _REG_ENOSYS: C2RustUnnamed = -1;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "413:8"]
    pub struct re_pattern_buffer {
        pub __buffer: *mut re_dfa_t,
        pub __allocated: __re_long_size_t,
        pub __used: __re_long_size_t,
        pub __syntax: reg_syntax_t,
        pub __fastmap: *mut libc::c_char,
        pub __translate: *mut libc::c_uchar,
        pub re_nsub: size_t,
        #[bitfield(name = "__can_be_null", ty = "libc::c_uint", bits =
                   "0..=0")]
        #[bitfield(name = "__regs_allocated", ty = "libc::c_uint", bits =
                   "1..=2")]
        #[bitfield(name = "__fastmap_accurate", ty = "libc::c_uint", bits =
                   "3..=3")]
        #[bitfield(name = "__no_sub", ty = "libc::c_uint", bits = "4..=4")]
        #[bitfield(name = "__not_bol", ty = "libc::c_uint", bits = "5..=5")]
        #[bitfield(name = "__not_eol", ty = "libc::c_uint", bits = "6..=6")]
        #[bitfield(name = "__newline_anchor", ty = "libc::c_uint", bits =
                   "7..=7")]
        pub __can_be_null___regs_allocated___fastmap_accurate___no_sub___not_bol___not_eol___newline_anchor: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
    }
    #[c2rust::src_loc = "478:1"]
    pub type regex_t = re_pattern_buffer;
    #[c2rust::src_loc = "490:1"]
    pub type regoff_t = libc::c_int;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "517:9"]
    pub struct regmatch_t {
        pub rm_so: regoff_t,
        pub rm_eo: regoff_t,
    }
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
        #[c2rust::src_loc = "648:1"]
        pub fn regerror(__errcode: libc::c_int, __preg: *const regex_t,
                        __errbuf: *mut libc::c_char, __errbuf_size: size_t)
         -> size_t;
        #[no_mangle]
        #[c2rust::src_loc = "651:1"]
        pub fn regfree(__preg: *mut regex_t);
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:38"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
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
    use super::stdint_intn_h::int32_t;
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
        /* *
 * Free a string representation of a principal.
 *
 * @param [in] context          Library context
 * @param [in] val              Name string to be freed
 */
        #[no_mangle]
        #[c2rust::src_loc = "4767:1"]
        pub fn krb5_free_unparsed_name(context: krb5_context,
                                       val: *mut libc::c_char);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:38"]
pub mod com_err_h {
    /*
 * Copyright 1988, Student Information Processing Board of the
 * Massachusetts Institute of Technology.
 *
 * Copyright 1995 by Cygnus Support.
 *
 * For copyright and distribution info, see the documentation supplied
 * with this package.
 */
    /* Header file for common error description library. */
    #[c2rust::src_loc = "26:1"]
    pub type errcode_t = libc::c_long;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "54:1"]
        pub fn error_message(_: errcode_t) -> *const libc::c_char;
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/preauth/pkinit/pkinit.h:38"]
pub mod pkinit_h {
    #[c2rust::src_loc = "128:1"]
    pub type pkinit_plg_crypto_context = *mut _pkinit_plg_crypto_context;
    #[c2rust::src_loc = "134:1"]
    pub type pkinit_req_crypto_context = *mut _pkinit_req_crypto_context;
    #[c2rust::src_loc = "141:1"]
    pub type pkinit_identity_crypto_context
        =
        *mut _pkinit_identity_crypto_context;
    #[inline]
    #[c2rust::src_loc = "88:1"]
    pub unsafe extern "C" fn pkiDebug(mut fmt: *const libc::c_char,
                                      mut args: ...) {
    }
    use super::krb5_h::{_krb5_context, krb5_context, krb5_data,
                        krb5_error_code};
    extern "C" {
        #[c2rust::src_loc = "128:16"]
        pub type _pkinit_plg_crypto_context;
        #[c2rust::src_loc = "134:16"]
        pub type _pkinit_req_crypto_context;
        #[c2rust::src_loc = "141:16"]
        pub type _pkinit_identity_crypto_context;
        /*
 * Client's list of identities for which it needs PINs or passwords
 */
        /*
 * initialization and free functions
 */
        /*
 * Functions in pkinit_profile.c
 */
        #[no_mangle]
        #[c2rust::src_loc = "364:1"]
        pub fn pkinit_libdefault_strings(context: krb5_context,
                                         realm: *const krb5_data,
                                         option: *const libc::c_char,
                                         ret_value:
                                             *mut *mut *mut libc::c_char)
         -> krb5_error_code;
    }
    /* _PKINIT_H */
    /*
 * Now get crypto function declarations
 */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/preauth/pkinit/pkinit_crypto.h:38"]
pub mod pkinit_crypto_h {
    #[c2rust::src_loc = "94:1"]
    pub type pkinit_cert_matching_data = _pkinit_cert_matching_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "94:16"]
    pub struct _pkinit_cert_matching_data {
        pub subject_dn: *mut libc::c_char,
        pub issuer_dn: *mut libc::c_char,
        pub ku_bits: libc::c_uint,
        pub eku_bits: libc::c_uint,
        pub sans: *mut krb5_principal,
        pub upns: *mut *mut libc::c_char,
    }
    use super::krb5_h::{krb5_principal, _krb5_context, krb5_context,
                        krb5_error_code};
    use super::pkinit_h::{_pkinit_identity_crypto_context,
                          pkinit_identity_crypto_context,
                          _pkinit_plg_crypto_context,
                          pkinit_plg_crypto_context,
                          _pkinit_req_crypto_context,
                          pkinit_req_crypto_context};
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "476:1"]
        pub fn crypto_cert_free_matching_data_list(context: krb5_context,
                                                   matchdata:
                                                       *mut *mut pkinit_cert_matching_data);
        #[no_mangle]
        #[c2rust::src_loc = "485:1"]
        pub fn crypto_cert_select(context: krb5_context,
                                  idctx: pkinit_identity_crypto_context,
                                  cred_index: size_t) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "459:1"]
        pub fn crypto_cert_get_matching_data(context: krb5_context,
                                             plg_cryptoctx:
                                                 pkinit_plg_crypto_context,
                                             req_cryptoctx:
                                                 pkinit_req_crypto_context,
                                             id_cryptoctx:
                                                 pkinit_identity_crypto_context,
                                             md_out:
                                                 *mut *mut *mut pkinit_cert_matching_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "492:1"]
        pub fn crypto_cert_select_default(context: krb5_context,
                                          plg_cryptoctx:
                                              pkinit_plg_crypto_context,
                                          req_cryptoctx:
                                              pkinit_req_crypto_context,
                                          id_cryptoctx:
                                              pkinit_identity_crypto_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "469:1"]
        pub fn crypto_cert_free_matching_data(context: krb5_context,
                                              md:
                                                  *mut pkinit_cert_matching_data);
        #[no_mangle]
        #[c2rust::src_loc = "636:1"]
        pub fn crypto_req_cert_matching_data(context: krb5_context,
                                             plgctx:
                                                 pkinit_plg_crypto_context,
                                             reqctx:
                                                 pkinit_req_crypto_context,
                                             md_out:
                                                 *mut *mut pkinit_cert_matching_data)
         -> krb5_error_code;
    }
    /* _PKINIT_CRYPTO_H */
}
#[c2rust::header_src = "/usr/include/string.h:33"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
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
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/strings.h:33"]
pub mod strings_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "120:12"]
        pub fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char,
                           _: libc::c_ulong) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:35"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:38"]
pub mod profile_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn profile_free_list(list: *mut *mut libc::c_char);
    }
    /*@modifies internalState@*/
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-trace.h:38"]
pub mod k5_trace_h {
    use super::krb5_h::{_krb5_context, krb5_context};
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-trace.h */
/*
 * Copyright (C) 2010 by the Massachusetts Institute of Technology.
 * All rights reserved.
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
 * This header contains trace macro definitions, which map trace points within
 * the code to krb5int_trace() calls with descriptive text strings.
 *
 * A new trace macro must be defined in this file for each new location to
 * be traced; the TRACE() macro should never be used directly.  This keeps
 * the tracing logic centralized in one place, to facilitate integration with
 * alternate tracing backends such as DTrace.
 *
 * Trace logging is intended to aid power users in diagnosing configuration
 * problems by showing what's going on behind the scenes of complex operations.
 * Although trace logging is sometimes useful to developers, it is not intended
 * as a replacement for a debugger, and it is not desirable to drown the user
 * in output.  Observe the following guidelines when adding trace points:
 *
 *   - Avoid mentioning function or variable names in messages.
 *
 *   - Try to convey what decisions are being made and what external inputs
 *     they are based on, not the process of making decisions.
 *
 *   - It is generally not necessary to trace before returning an unrecoverable
 *     error.  If an error code is unclear by itself, make it clearer with
 *     krb5_set_error_message().
 *
 *   - Keep macros simple.  Add format specifiers to krb5int_trace's formatter
 *     as necessary (and document them here) instead of transforming macro
 *     arguments.
 *
 *   - Like printf, the trace formatter interface is not type-safe.  Check your
 *     formats carefully.  Cast integral arguments to the appropriate type if
 *     they do not already patch.
 *
 * The following specifiers are supported by the formatter (see the
 * implementation in lib/krb5/os/trace.c for details):
 *
 *   {int}         int, in decimal
 *   {long}        long, in decimal
 *   {str}         const char *, display as C string
 *   {lenstr}      size_t and const char *, as a counted string
 *   {hexlenstr}   size_t and const char *, as hex bytes
 *   {hashlenstr}  size_t and const char *, as four-character hex hash
 *   {raddr}       struct remote_address *, show socket type, address, port
 *   {data}        krb5_data *, display as counted string
 *   {hexdata}     krb5_data *, display as hex bytes
 *   {errno}       int, display as number/errorstring
 *   {kerr}        krb5_error_code, display as number/errorstring
 *   {keyblock}    const krb5_keyblock *, display enctype and hash of key
 *   {key}         krb5_key, display enctype and hash of key
 *   {cksum}       const krb5_checksum *, display cksumtype and hex checksum
 *   {princ}       krb5_principal, unparse and display
 *   {ptype}       krb5_int32, krb5_principal type, display name
 *   {patype}      krb5_preauthtype, a single padata type number
 *   {patypes}     krb5_pa_data **, display list of padata type numbers
 *   {etype}       krb5_enctype, display shortest name of enctype
 *   {etypes}      krb5_enctype *, display list of enctypes
 *   {ccache}      krb5_ccache, display type:name
 *   {keytab}      krb5_keytab, display name
 *   {creds}       krb5_creds *, display clientprinc -> serverprinc
 */
        #[no_mangle]
        #[c2rust::src_loc = "94:1"]
        pub fn krb5int_trace(context: krb5_context, fmt: *const libc::c_char,
                             _: ...);
    }
    /* K5_TRACE_H */
    /* DISABLE_TRACING */
    /* Try to optimize away argument evaluation and function call when we're not
 * tracing, if this source file knows the internals of the context. */
}
pub use self::stddef_h::size_t;
pub use self::types_h::__int32_t;
pub use self::stdint_intn_h::int32_t;
pub use self::regex_h::{__re_long_size_t, reg_syntax_t, C2RustUnnamed,
                        _REG_ERPAREN, _REG_ESIZE, _REG_EEND, _REG_BADRPT,
                        _REG_ESPACE, _REG_ERANGE, _REG_BADBR, _REG_EBRACE,
                        _REG_EPAREN, _REG_EBRACK, _REG_ESUBREG, _REG_EESCAPE,
                        _REG_ECTYPE, _REG_ECOLLATE, _REG_BADPAT, _REG_NOMATCH,
                        _REG_NOERROR, _REG_ENOSYS, re_pattern_buffer, regex_t,
                        regoff_t, regmatch_t, re_dfa_t, regcomp, regexec,
                        regerror, regfree};
pub use self::krb5_h::{krb5_int32, krb5_boolean, krb5_error_code, krb5_magic,
                       _krb5_data, krb5_data, krb5_principal_data,
                       krb5_principal, krb5_const_principal, krb5_context,
                       _krb5_context, krb5_unparse_name,
                       krb5_free_unparsed_name};
pub use self::com_err_h::{errcode_t, error_message};
pub use self::pkinit_h::{pkinit_plg_crypto_context, pkinit_req_crypto_context,
                         pkinit_identity_crypto_context, pkiDebug,
                         _pkinit_plg_crypto_context,
                         _pkinit_req_crypto_context,
                         _pkinit_identity_crypto_context,
                         pkinit_libdefault_strings};
pub use self::pkinit_crypto_h::{pkinit_cert_matching_data,
                                _pkinit_cert_matching_data,
                                crypto_cert_free_matching_data_list,
                                crypto_cert_select,
                                crypto_cert_get_matching_data,
                                crypto_cert_select_default,
                                crypto_cert_free_matching_data,
                                crypto_req_cert_matching_data};
use self::string_h::{strncmp, strdup, strchr, strlen, memcpy};
use self::strings_h::strncasecmp;
use self::stdlib_h::{calloc, free};
use self::profile_h::profile_free_list;
use self::k5_trace_h::krb5int_trace;
/* Rule component */
#[c2rust::src_loc = "133:1"]
pub type rule_component = _rule_component;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "133:16"]
pub struct _rule_component {
    pub next: *mut _rule_component,
    pub kw_type: keyword_type,
    pub kwval_type: kw_value_type,
    pub regexp: regex_t,
    pub regsrc: *mut libc::c_char,
    pub ku_bits: libc::c_uint,
    pub eku_bits: libc::c_uint,
}
#[c2rust::src_loc = "81:9"]
pub type kw_value_type = libc::c_uint;
#[c2rust::src_loc = "84:5"]
pub const kwvaltype_list: kw_value_type = 2;
#[c2rust::src_loc = "83:5"]
pub const kwvaltype_regexp: kw_value_type = 1;
#[c2rust::src_loc = "82:5"]
pub const kwvaltype_undefined: kw_value_type = 0;
#[c2rust::src_loc = "42:9"]
pub type keyword_type = libc::c_uint;
#[c2rust::src_loc = "48:5"]
pub const kw_ku: keyword_type = 5;
#[c2rust::src_loc = "47:5"]
pub const kw_eku: keyword_type = 4;
#[c2rust::src_loc = "46:5"]
pub const kw_san: keyword_type = 3;
#[c2rust::src_loc = "45:5"]
pub const kw_issuer: keyword_type = 2;
#[c2rust::src_loc = "44:5"]
pub const kw_subject: keyword_type = 1;
#[c2rust::src_loc = "43:5"]
pub const kw_undefined: keyword_type = 0;
/* Compiled regular expression */
/* The regular expression source (for debugging) */
/* Set rule components */
#[c2rust::src_loc = "144:1"]
pub type rule_set = _rule_set;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "144:16"]
pub struct _rule_set {
    pub relation: relation_type,
    pub num_crs: libc::c_int,
    pub crs: *mut rule_component,
}
#[c2rust::src_loc = "64:9"]
pub type relation_type = libc::c_uint;
#[c2rust::src_loc = "67:5"]
pub const relation_or: relation_type = 2;
#[c2rust::src_loc = "66:5"]
pub const relation_and: relation_type = 1;
#[c2rust::src_loc = "65:5"]
pub const relation_none: relation_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "112:8"]
pub struct ku_desc {
    pub value: *const libc::c_char,
    pub length: size_t,
    pub bitval: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "98:8"]
pub struct keyword_desc {
    pub value: *const libc::c_char,
    pub length: size_t,
    pub kwtype: keyword_type,
    pub kwvaltype: kw_value_type,
}
#[c2rust::src_loc = "51:1"]
unsafe extern "C" fn keyword2string(mut kw: libc::c_uint)
 -> *mut libc::c_char {
    match kw {
        0 => {
            return b"NONE\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        1 => {
            return b"SUBJECT\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        2 => {
            return b"ISSUER\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        3 => {
            return b"SAN\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        4 => {
            return b"EKU\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        5 => {
            return b"KU\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        _ => {
            return b"INVALID\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
    };
}
#[c2rust::src_loc = "70:1"]
unsafe extern "C" fn relation2string(mut rel: libc::c_uint)
 -> *mut libc::c_char {
    match rel {
        0 => {
            return b"NONE\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        1 => {
            return b"AND\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        2 => {
            return b"OR\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        _ => {
            return b"INVALID\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
    };
}
#[c2rust::src_loc = "87:1"]
unsafe extern "C" fn kwval2string(mut kwval: libc::c_uint)
 -> *mut libc::c_char {
    match kwval {
        0 => {
            return b"NONE\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        1 => {
            return b"REGEXP\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        2 => {
            return b"LIST\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        _ => {
            return b"INVALID\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "103:3"]
pub static mut matching_keywords: [keyword_desc; 6] =
    [{
         let mut init =
             keyword_desc{value:
                              b"<KU>\x00" as *const u8 as *const libc::c_char,
                          length: 4 as libc::c_int as size_t,
                          kwtype: kw_ku,
                          kwvaltype: kwvaltype_list,};
         init
     },
     {
         let mut init =
             keyword_desc{value:
                              b"<EKU>\x00" as *const u8 as
                                  *const libc::c_char,
                          length: 5 as libc::c_int as size_t,
                          kwtype: kw_eku,
                          kwvaltype: kwvaltype_list,};
         init
     },
     {
         let mut init =
             keyword_desc{value:
                              b"<SAN>\x00" as *const u8 as
                                  *const libc::c_char,
                          length: 5 as libc::c_int as size_t,
                          kwtype: kw_san,
                          kwvaltype: kwvaltype_regexp,};
         init
     },
     {
         let mut init =
             keyword_desc{value:
                              b"<ISSUER>\x00" as *const u8 as
                                  *const libc::c_char,
                          length: 8 as libc::c_int as size_t,
                          kwtype: kw_issuer,
                          kwvaltype: kwvaltype_regexp,};
         init
     },
     {
         let mut init =
             keyword_desc{value:
                              b"<SUBJECT>\x00" as *const u8 as
                                  *const libc::c_char,
                          length: 9 as libc::c_int as size_t,
                          kwtype: kw_subject,
                          kwvaltype: kwvaltype_regexp,};
         init
     },
     {
         let mut init =
             keyword_desc{value: 0 as *const libc::c_char,
                          length: 0 as libc::c_int as size_t,
                          kwtype: kw_undefined,
                          kwvaltype: kwvaltype_undefined,};
         init
     }];
#[no_mangle]
#[c2rust::src_loc = "118:16"]
pub static mut ku_keywords: [ku_desc; 3] =
    [{
         let mut init =
             ku_desc{value:
                         b"digitalSignature\x00" as *const u8 as
                             *const libc::c_char,
                     length: 16 as libc::c_int as size_t,
                     bitval: 0x80000000 as libc::c_uint,};
         init
     },
     {
         let mut init =
             ku_desc{value:
                         b"keyEncipherment\x00" as *const u8 as
                             *const libc::c_char,
                     length: 15 as libc::c_int as size_t,
                     bitval: 0x40000000 as libc::c_int as libc::c_uint,};
         init
     },
     {
         let mut init =
             ku_desc{value: 0 as *const libc::c_char,
                     length: 0 as libc::c_int as size_t,
                     bitval: 0 as libc::c_int as libc::c_uint,};
         init
     }];
#[no_mangle]
#[c2rust::src_loc = "124:17"]
pub static mut eku_keywords: [ku_desc; 5] =
    [{
         let mut init =
             ku_desc{value: b"pkinit\x00" as *const u8 as *const libc::c_char,
                     length: 6 as libc::c_int as size_t,
                     bitval: 0x80000000 as libc::c_uint,};
         init
     },
     {
         let mut init =
             ku_desc{value:
                         b"msScLogin\x00" as *const u8 as *const libc::c_char,
                     length: 9 as libc::c_int as size_t,
                     bitval: 0x40000000 as libc::c_int as libc::c_uint,};
         init
     },
     {
         let mut init =
             ku_desc{value:
                         b"clientAuth\x00" as *const u8 as
                             *const libc::c_char,
                     length: 10 as libc::c_int as size_t,
                     bitval: 0x20000000 as libc::c_int as libc::c_uint,};
         init
     },
     {
         let mut init =
             ku_desc{value:
                         b"emailProtection\x00" as *const u8 as
                             *const libc::c_char,
                     length: 15 as libc::c_int as size_t,
                     bitval: 0x10000000 as libc::c_int as libc::c_uint,};
         init
     },
     {
         let mut init =
             ku_desc{value: 0 as *const libc::c_char,
                     length: 0 as libc::c_int as size_t,
                     bitval: 0 as libc::c_int as libc::c_uint,};
         init
     }];
#[c2rust::src_loc = "150:1"]
unsafe extern "C" fn free_rule_component(mut context: krb5_context,
                                         mut rc: *mut rule_component)
 -> krb5_error_code {
    if rc.is_null() { return 0 as libc::c_int }
    if (*rc).kwval_type as libc::c_uint ==
           kwvaltype_regexp as libc::c_int as libc::c_uint {
        free((*rc).regsrc as *mut libc::c_void);
        regfree(&mut (*rc).regexp);
    }
    free(rc as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "165:1"]
unsafe extern "C" fn free_rule_set(mut context: krb5_context,
                                   mut rs: *mut rule_set) -> krb5_error_code {
    let mut rc: *mut rule_component = 0 as *mut rule_component;
    let mut trc: *mut rule_component = 0 as *mut rule_component;
    if rs.is_null() { return 0 as libc::c_int }
    rc = (*rs).crs;
    while !rc.is_null() {
        trc = (*rc).next;
        free_rule_component(context, rc);
        rc = trc
    }
    free(rs as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "182:1"]
unsafe extern "C" fn parse_list_value(mut context: krb5_context,
                                      mut type_0: keyword_type,
                                      mut value: *mut libc::c_char,
                                      mut rc: *mut rule_component)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut comma: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ku: *mut ku_desc = 0 as *mut ku_desc;
    let mut found: libc::c_int = 0;
    let mut len: size_t = 0;
    let mut bitptr: *mut libc::c_uint = 0 as *mut libc::c_uint;
    if value.is_null() ||
           *value.offset(0 as libc::c_int as isize) as libc::c_int ==
               '\u{0}' as i32 {
        pkiDebug(b"%s: Missing or empty value for list keyword type %d\n\x00"
                     as *const u8 as *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 17],
                                           &[libc::c_char; 17]>(b"parse_list_value\x00")).as_ptr(),
                 type_0 as libc::c_uint);
        retval = 22 as libc::c_int
    } else {
        if type_0 as libc::c_uint == kw_eku as libc::c_int as libc::c_uint {
            bitptr = &mut (*rc).eku_bits;
            current_block = 17407779659766490442;
        } else if type_0 as libc::c_uint ==
                      kw_ku as libc::c_int as libc::c_uint {
            bitptr = &mut (*rc).ku_bits;
            current_block = 17407779659766490442;
        } else {
            pkiDebug(b"%s: Unknown list keyword type %d\n\x00" as *const u8 as
                         *const libc::c_char,
                     (*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"parse_list_value\x00")).as_ptr(),
                     type_0 as libc::c_uint);
            retval = 22 as libc::c_int;
            current_block = 6618313762102813457;
        }
        match current_block {
            6618313762102813457 => { }
            _ => {
                loop  {
                    found = 0 as libc::c_int;
                    comma = strchr(value, ',' as i32);
                    if !comma.is_null() {
                        len =
                            comma.wrapping_offset_from(value) as libc::c_long
                                as size_t
                    } else { len = strlen(value) }
                    if type_0 as libc::c_uint ==
                           kw_eku as libc::c_int as libc::c_uint {
                        ku = eku_keywords.as_mut_ptr()
                    } else if type_0 as libc::c_uint ==
                                  kw_ku as libc::c_int as libc::c_uint {
                        ku = ku_keywords.as_mut_ptr()
                    }
                    while !(*ku).value.is_null() {
                        if strncasecmp(value, (*ku).value, len) ==
                               0 as libc::c_int {
                            *bitptr |= (*ku).bitval;
                            found = 1 as libc::c_int;
                            pkiDebug(b"%s: Found value \'%s\', bitfield is now 0x%x\n\x00"
                                         as *const u8 as *const libc::c_char,
                                     (*::std::mem::transmute::<&[u8; 17],
                                                               &[libc::c_char; 17]>(b"parse_list_value\x00")).as_ptr(),
                                     (*ku).value, *bitptr);
                            break ;
                        } else { ku = ku.offset(1) }
                    }
                    if found != 0 {
                        value = value.offset((*ku).length as isize);
                        if *value as libc::c_int == ',' as i32 {
                            value = value.offset(1 as libc::c_int as isize)
                        }
                        if !(found != 0 &&
                                 *value as libc::c_int != '\u{0}' as i32) {
                            current_block = 11743904203796629665;
                            break ;
                        }
                    } else {
                        pkiDebug(b"%s: Urecognized value \'%s\'\n\x00" as
                                     *const u8 as *const libc::c_char,
                                 (*::std::mem::transmute::<&[u8; 17],
                                                           &[libc::c_char; 17]>(b"parse_list_value\x00")).as_ptr(),
                                 value);
                        retval = 22 as libc::c_int;
                        current_block = 6618313762102813457;
                        break ;
                    }
                }
                match current_block {
                    6618313762102813457 => { }
                    _ => { retval = 0 as libc::c_int }
                }
            }
        }
    }
    pkiDebug(b"%s: returning %d\n\x00" as *const u8 as *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 17],
                                       &[libc::c_char; 17]>(b"parse_list_value\x00")).as_ptr(),
             retval);
    return retval;
}
#[c2rust::src_loc = "253:1"]
unsafe extern "C" fn parse_rule_component(mut context: krb5_context,
                                          mut rule: *mut *const libc::c_char,
                                          mut remaining: *mut libc::c_int,
                                          mut ret_rule:
                                              *mut *mut rule_component)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut rc: *mut rule_component = 0 as *mut rule_component;
    let mut kw_type: keyword_type = kw_undefined;
    let mut kwval_type: kw_value_type = kwvaltype_undefined;
    let mut err_buf: [libc::c_char; 128] = [0; 128];
    let mut ret: libc::c_int = 0;
    let mut kw: *mut keyword_desc = 0 as *mut keyword_desc;
    let mut nextkw: *mut keyword_desc = 0 as *mut keyword_desc;
    let mut nk: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut found_next_kw: libc::c_int = 0 as libc::c_int;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    kw = matching_keywords.as_mut_ptr();
    while !(*kw).value.is_null() {
        if strncmp(*rule, (*kw).value, (*kw).length) == 0 as libc::c_int {
            kw_type = (*kw).kwtype;
            kwval_type = (*kw).kwvaltype;
            *rule = (*rule).offset((*kw).length as isize);
            *remaining =
                (*remaining as libc::c_ulong).wrapping_sub((*kw).length) as
                    libc::c_int as libc::c_int;
            break ;
        } else { kw = kw.offset(1) }
    }
    if (*kw).value.is_null() {
        pkiDebug(b"%s: Missing or invalid keyword in rule \'%s\'\n\x00" as
                     *const u8 as *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 21],
                                           &[libc::c_char; 21]>(b"parse_rule_component\x00")).as_ptr(),
                 *rule);
        retval = 2 as libc::c_int
    } else {
        pkiDebug(b"%s: found keyword \'%s\'\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 21],
                                           &[libc::c_char; 21]>(b"parse_rule_component\x00")).as_ptr(),
                 (*kw).value);
        rc =
            calloc(1 as libc::c_int as libc::c_ulong,
                   ::std::mem::size_of::<rule_component>() as libc::c_ulong)
                as *mut rule_component;
        if rc.is_null() {
            retval = 12 as libc::c_int
        } else {
            (*rc).next = 0 as *mut _rule_component;
            (*rc).kw_type = kw_type;
            (*rc).kwval_type = kwval_type;
            /*
     * Before procesing the value for this keyword,
     * (compiling the regular expression or processing the list)
     * we need to find the end of it.  That means parsing for the
     * beginning of the next keyword (or the end of the rule).
     */
            nk = strchr(*rule, '<' as i32);
            while !nk.is_null() {
                /* Possibly another keyword, check it out */
                nextkw = matching_keywords.as_mut_ptr();
                while !(*nextkw).value.is_null() {
                    if strncmp(nk, (*nextkw).value, (*nextkw).length) ==
                           0 as libc::c_int {
                        /* Found a keyword, nk points to the beginning */
                        found_next_kw = 1 as libc::c_int;
                        break ;
                        /* Need to break out of the while! */
                    } else { nextkw = nextkw.offset(1) }
                } /* keep looking */
                if !(found_next_kw == 0) { break ; }
                nk = strchr(nk.offset(1 as libc::c_int as isize), '<' as i32)
            }
            if !nk.is_null() && found_next_kw != 0 {
                len = nk.wrapping_offset_from(*rule) as libc::c_long as size_t
            } else { len = *remaining as size_t }
            if len == 0 as libc::c_int as libc::c_ulong {
                pkiDebug(b"%s: Missing value for keyword \'%s\'\n\x00" as
                             *const u8 as *const libc::c_char,
                         (*::std::mem::transmute::<&[u8; 21],
                                                   &[libc::c_char; 21]>(b"parse_rule_component\x00")).as_ptr(),
                         (*kw).value);
                retval = 22 as libc::c_int
            } else {
                value =
                    calloc(1 as libc::c_int as libc::c_ulong,
                           len.wrapping_add(1 as libc::c_int as
                                                libc::c_ulong)) as
                        *mut libc::c_char;
                if value.is_null() {
                    retval = 12 as libc::c_int
                } else {
                    memcpy(value as *mut libc::c_void,
                           *rule as *const libc::c_void, len);
                    *remaining =
                        (*remaining as libc::c_ulong).wrapping_sub(len) as
                            libc::c_int as libc::c_int;
                    *rule = (*rule).offset(len as isize);
                    pkiDebug(b"%s: found value \'%s\'\n\x00" as *const u8 as
                                 *const libc::c_char,
                             (*::std::mem::transmute::<&[u8; 21],
                                                       &[libc::c_char; 21]>(b"parse_rule_component\x00")).as_ptr(),
                             value);
                    if (*kw).kwvaltype as libc::c_uint ==
                           kwvaltype_regexp as libc::c_int as libc::c_uint {
                        ret =
                            regcomp(&mut (*rc).regexp, value,
                                    1 as libc::c_int);
                        if ret != 0 {
                            regerror(ret, &mut (*rc).regexp,
                                     err_buf.as_mut_ptr(),
                                     ::std::mem::size_of::<[libc::c_char; 128]>()
                                         as libc::c_ulong);
                            pkiDebug(b"%s: Error compiling reg-exp \'%s\': %s\n\x00"
                                         as *const u8 as *const libc::c_char,
                                     (*::std::mem::transmute::<&[u8; 21],
                                                               &[libc::c_char; 21]>(b"parse_rule_component\x00")).as_ptr(),
                                     value, err_buf.as_mut_ptr());
                            retval = ret;
                            current_block = 17658392462878113341;
                        } else {
                            (*rc).regsrc = strdup(value);
                            if (*rc).regsrc.is_null() {
                                retval = 12 as libc::c_int;
                                current_block = 17658392462878113341;
                            } else { current_block = 17075014677070940716; }
                        }
                    } else if (*kw).kwvaltype as libc::c_uint ==
                                  kwvaltype_list as libc::c_int as
                                      libc::c_uint {
                        retval =
                            parse_list_value(context, (*rc).kw_type, value,
                                             rc);
                        if retval != 0 {
                            pkiDebug(b"%s: Error %d, parsing list values for keyword %s\n\x00"
                                         as *const u8 as *const libc::c_char,
                                     (*::std::mem::transmute::<&[u8; 21],
                                                               &[libc::c_char; 21]>(b"parse_rule_component\x00")).as_ptr(),
                                     retval, (*kw).value);
                            current_block = 17658392462878113341;
                        } else { current_block = 17075014677070940716; }
                    } else { current_block = 17075014677070940716; }
                    match current_block {
                        17658392462878113341 => { }
                        _ => { *ret_rule = rc; retval = 0 as libc::c_int }
                    }
                }
            }
        }
    }
    free(value as *mut libc::c_void);
    if retval != 0 && !rc.is_null() { free_rule_component(context, rc); }
    pkiDebug(b"%s: returning %d\n\x00" as *const u8 as *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 21],
                                       &[libc::c_char; 21]>(b"parse_rule_component\x00")).as_ptr(),
             retval);
    return retval;
}
#[c2rust::src_loc = "375:1"]
unsafe extern "C" fn parse_rule_set(mut context: krb5_context,
                                    mut rule_in: *const libc::c_char,
                                    mut out_rs: *mut *mut rule_set)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut rule: *const libc::c_char = 0 as *const libc::c_char;
    let mut remaining: libc::c_int = 0;
    let mut ret: krb5_error_code = 0;
    let mut retval: krb5_error_code = 0;
    let mut rc: *mut rule_component = 0 as *mut rule_component;
    let mut trc: *mut rule_component = 0 as *mut rule_component;
    let mut rs: *mut rule_set = 0 as *mut rule_set;
    if rule_in.is_null() { return 22 as libc::c_int }
    rule = rule_in;
    remaining = strlen(rule) as libc::c_int;
    rs =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<rule_set>() as libc::c_ulong) as
            *mut rule_set;
    if rs.is_null() {
        retval = 12 as libc::c_int
    } else {
        (*rs).relation = relation_none;
        if remaining > 1 as libc::c_int {
            if *rule.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '&' as i32 &&
                   *rule.offset(1 as libc::c_int as isize) as libc::c_int ==
                       '&' as i32 {
                (*rs).relation = relation_and;
                rule = rule.offset(2 as libc::c_int as isize);
                remaining -= 2 as libc::c_int
            } else if *rule_in.offset(0 as libc::c_int as isize) as
                          libc::c_int == '|' as i32 &&
                          *rule_in.offset(1 as libc::c_int as isize) as
                              libc::c_int == '|' as i32 {
                (*rs).relation = relation_or;
                rule = rule.offset(2 as libc::c_int as isize);
                remaining -= 2 as libc::c_int
            }
        }
        (*rs).num_crs = 0 as libc::c_int;
        loop  {
            if !(remaining > 0 as libc::c_int) {
                current_block = 7746103178988627676;
                break ;
            }
            if (*rs).relation as libc::c_uint ==
                   relation_none as libc::c_int as libc::c_uint &&
                   (*rs).num_crs > 0 as libc::c_int {
                pkiDebug(b"%s: Assuming AND relation for multiple components in rule \'%s\'\n\x00"
                             as *const u8 as *const libc::c_char,
                         (*::std::mem::transmute::<&[u8; 15],
                                                   &[libc::c_char; 15]>(b"parse_rule_set\x00")).as_ptr(),
                         rule_in);
                (*rs).relation = relation_and
            }
            ret =
                parse_rule_component(context, &mut rule, &mut remaining,
                                     &mut rc);
            if ret != 0 {
                retval = ret;
                current_block = 380029518465981964;
                break ;
            } else {
                pkiDebug(b"%s: After parse_rule_component, remaining %d, rule \'%s\'\n\x00"
                             as *const u8 as *const libc::c_char,
                         (*::std::mem::transmute::<&[u8; 15],
                                                   &[libc::c_char; 15]>(b"parse_rule_set\x00")).as_ptr(),
                         remaining, rule);
                (*rs).num_crs += 1;
                /*
         * Chain the new component on the end (order matters since
         * we can short-circuit an OR or an AND relation if an
         * earlier check passes
         */
                trc = (*rs).crs;
                while !trc.is_null() && !(*trc).next.is_null() {
                    trc = (*trc).next
                }
                if trc.is_null() { (*rs).crs = rc } else { (*trc).next = rc }
            }
        }
        match current_block {
            380029518465981964 => { }
            _ => { *out_rs = rs; retval = 0 as libc::c_int }
        }
    }
    if retval != 0 && !rs.is_null() { free_rule_set(context, rs); }
    pkiDebug(b"%s: returning %d\n\x00" as *const u8 as *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 15],
                                       &[libc::c_char; 15]>(b"parse_rule_set\x00")).as_ptr(),
             retval);
    return retval;
}
#[c2rust::src_loc = "450:1"]
unsafe extern "C" fn regexp_match(mut context: krb5_context,
                                  mut rc: *mut rule_component,
                                  mut value: *mut libc::c_char)
 -> libc::c_int {
    let mut code: libc::c_int = 0;
    pkiDebug(b"%s: checking %s rule \'%s\' with value \'%s\'\n\x00" as
                 *const u8 as *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 13],
                                       &[libc::c_char; 13]>(b"regexp_match\x00")).as_ptr(),
             keyword2string((*rc).kw_type as libc::c_uint), (*rc).regsrc,
             value);
    code =
        regexec(&mut (*rc).regexp, value, 0 as libc::c_int as size_t,
                0 as *mut regmatch_t, 0 as libc::c_int);
    pkiDebug(b"%s: the result is%s a match\n\x00" as *const u8 as
                 *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 13],
                                       &[libc::c_char; 13]>(b"regexp_match\x00")).as_ptr(),
             if code == _REG_NOMATCH as libc::c_int {
                 b" NOT\x00" as *const u8 as *const libc::c_char
             } else { b"\x00" as *const u8 as *const libc::c_char });
    return if code == 0 as libc::c_int {
               1 as libc::c_int
           } else { 0 as libc::c_int };
}
#[c2rust::src_loc = "466:1"]
unsafe extern "C" fn component_match(mut context: krb5_context,
                                     mut rc: *mut rule_component,
                                     mut md: *mut pkinit_cert_matching_data)
 -> libc::c_int {
    let mut match_0: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut princ_string: *mut libc::c_char = 0 as *mut libc::c_char;
    match (*rc).kwval_type as libc::c_uint {
        1 => {
            match (*rc).kw_type as libc::c_uint {
                1 => { match_0 = regexp_match(context, rc, (*md).subject_dn) }
                2 => { match_0 = regexp_match(context, rc, (*md).issuer_dn) }
                3 => {
                    i = 0 as libc::c_int;
                    while !(*md).sans.is_null() &&
                              !(*(*md).sans.offset(i as isize)).is_null() {
                        krb5_unparse_name(context,
                                          *(*md).sans.offset(i as isize) as
                                              krb5_const_principal,
                                          &mut princ_string);
                        match_0 = regexp_match(context, rc, princ_string);
                        krb5_free_unparsed_name(context, princ_string);
                        if match_0 != 0 { break ; }
                        i += 1
                    }
                    i = 0 as libc::c_int;
                    while !(*md).upns.is_null() &&
                              !(*(*md).upns.offset(i as isize)).is_null() {
                        match_0 =
                            regexp_match(context, rc,
                                         *(*md).upns.offset(i as isize));
                        if match_0 != 0 { break ; }
                        i += 1
                    }
                }
                _ => {
                    pkiDebug(b"%s: keyword %s, keyword value %s mismatch\n\x00"
                                 as *const u8 as *const libc::c_char,
                             (*::std::mem::transmute::<&[u8; 16],
                                                       &[libc::c_char; 16]>(b"component_match\x00")).as_ptr(),
                             keyword2string((*rc).kw_type as libc::c_uint),
                             kwval2string(kwvaltype_regexp as libc::c_int as
                                              libc::c_uint));
                }
            }
        }
        2 => {
            match (*rc).kw_type as libc::c_uint {
                4 => {
                    pkiDebug(b"%s: checking %s: rule 0x%08x, cert 0x%08x\n\x00"
                                 as *const u8 as *const libc::c_char,
                             (*::std::mem::transmute::<&[u8; 16],
                                                       &[libc::c_char; 16]>(b"component_match\x00")).as_ptr(),
                             keyword2string((*rc).kw_type as libc::c_uint),
                             (*rc).eku_bits, (*md).eku_bits);
                    if (*rc).eku_bits & (*md).eku_bits == (*rc).eku_bits {
                        match_0 = 1 as libc::c_int
                    }
                }
                5 => {
                    pkiDebug(b"%s: checking %s: rule 0x%08x, cert 0x%08x\n\x00"
                                 as *const u8 as *const libc::c_char,
                             (*::std::mem::transmute::<&[u8; 16],
                                                       &[libc::c_char; 16]>(b"component_match\x00")).as_ptr(),
                             keyword2string((*rc).kw_type as libc::c_uint),
                             (*rc).ku_bits, (*md).ku_bits);
                    if (*rc).ku_bits & (*md).ku_bits == (*rc).ku_bits {
                        match_0 = 1 as libc::c_int
                    }
                }
                _ => {
                    pkiDebug(b"%s: keyword %s, keyword value %s mismatch\n\x00"
                                 as *const u8 as *const libc::c_char,
                             (*::std::mem::transmute::<&[u8; 16],
                                                       &[libc::c_char; 16]>(b"component_match\x00")).as_ptr(),
                             keyword2string((*rc).kw_type as libc::c_uint),
                             kwval2string(kwvaltype_regexp as libc::c_int as
                                              libc::c_uint));
                }
            }
        }
        _ => {
            pkiDebug(b"%s: unknown keyword value type %d\n\x00" as *const u8
                         as *const libc::c_char,
                     (*::std::mem::transmute::<&[u8; 16],
                                               &[libc::c_char; 16]>(b"component_match\x00")).as_ptr(),
                     (*rc).kwval_type as libc::c_uint);
        }
    }
    pkiDebug(b"%s: returning match = %d\n\x00" as *const u8 as
                 *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 16],
                                       &[libc::c_char; 16]>(b"component_match\x00")).as_ptr(),
             match_0);
    return match_0;
}
/*
 * Returns match_found == 1 only if exactly one certificate matches
 * the given rule
 */
#[c2rust::src_loc = "540:1"]
unsafe extern "C" fn check_all_certs(mut context: krb5_context,
                                     mut plg_cryptoctx:
                                         pkinit_plg_crypto_context,
                                     mut req_cryptoctx:
                                         pkinit_req_crypto_context,
                                     mut id_cryptoctx:
                                         pkinit_identity_crypto_context,
                                     mut princ: krb5_principal,
                                     mut rs: *mut rule_set,
                                     mut matchdata:
                                         *mut *mut pkinit_cert_matching_data,
                                     mut match_found: *mut libc::c_int,
                                     mut match_index: *mut size_t)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut md: *mut pkinit_cert_matching_data =
        0 as *mut pkinit_cert_matching_data;
    let mut i: libc::c_int = 0;
    let mut comp_match: libc::c_int = 0 as libc::c_int;
    let mut total_cert_matches: libc::c_int = 0 as libc::c_int;
    let mut rc: *mut rule_component = 0 as *mut rule_component;
    let mut certs_checked: libc::c_int = 0 as libc::c_int;
    let mut save_index: size_t = 0 as libc::c_int as size_t;
    if match_found.is_null() || match_index.is_null() {
        return 22 as libc::c_int
    }
    *match_index = 0 as libc::c_int as size_t;
    *match_found = 0 as libc::c_int;
    pkiDebug(b"%s: matching rule relation is %s with %d components\n\x00" as
                 *const u8 as *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 16],
                                       &[libc::c_char; 16]>(b"check_all_certs\x00")).as_ptr(),
             relation2string((*rs).relation as libc::c_uint), (*rs).num_crs);
    /*
     * Loop through all the certs available and count
     * how many match the rule
     */
    i = 0 as libc::c_int;
    md = *matchdata.offset(i as isize);
    while !md.is_null() {
        pkiDebug(b"%s: subject: \'%s\'\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 16],
                                           &[libc::c_char; 16]>(b"check_all_certs\x00")).as_ptr(),
                 (*md).subject_dn);
        certs_checked += 1;
        rc = (*rs).crs;
        loop  {
            if rc.is_null() { current_block = 16203760046146113240; break ; }
            comp_match = component_match(context, rc, md);
            if comp_match != 0 {
                pkiDebug(b"%s: match for keyword type %s\n\x00" as *const u8
                             as *const libc::c_char,
                         (*::std::mem::transmute::<&[u8; 16],
                                                   &[libc::c_char; 16]>(b"check_all_certs\x00")).as_ptr(),
                         keyword2string((*rc).kw_type as libc::c_uint));
            }
            if comp_match != 0 &&
                   (*rs).relation as libc::c_uint ==
                       relation_or as libc::c_int as libc::c_uint {
                pkiDebug(b"%s: cert matches rule (OR relation)\n\x00" as
                             *const u8 as *const libc::c_char,
                         (*::std::mem::transmute::<&[u8; 16],
                                                   &[libc::c_char; 16]>(b"check_all_certs\x00")).as_ptr());
                total_cert_matches += 1;
                save_index = i as size_t;
                current_block = 1394248824506584008;
                break ;
            } else if comp_match == 0 &&
                          (*rs).relation as libc::c_uint ==
                              relation_and as libc::c_int as libc::c_uint {
                pkiDebug(b"%s: cert does not match rule (AND relation)\n\x00"
                             as *const u8 as *const libc::c_char,
                         (*::std::mem::transmute::<&[u8; 16],
                                                   &[libc::c_char; 16]>(b"check_all_certs\x00")).as_ptr());
                current_block = 1394248824506584008;
                break ;
            } else { rc = (*rc).next }
        }
        match current_block {
            16203760046146113240 => {
                if rc.is_null() && comp_match != 0 {
                    pkiDebug(b"%s: cert matches rule (AND relation)\n\x00" as
                                 *const u8 as *const libc::c_char,
                             (*::std::mem::transmute::<&[u8; 16],
                                                       &[libc::c_char; 16]>(b"check_all_certs\x00")).as_ptr());
                    total_cert_matches += 1;
                    save_index = i as size_t
                }
            }
            _ => { }
        }
        i += 1;
        md = *matchdata.offset(i as isize)
    }
    pkiDebug(b"%s: After checking %d certs, we found %d matches\n\x00" as
                 *const u8 as *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 16],
                                       &[libc::c_char; 16]>(b"check_all_certs\x00")).as_ptr(),
             certs_checked, total_cert_matches);
    if total_cert_matches == 1 as libc::c_int {
        *match_found = 1 as libc::c_int;
        *match_index = save_index
    }
    retval = 0 as libc::c_int;
    pkiDebug(b"%s: returning %d, match_found %d\n\x00" as *const u8 as
                 *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 16],
                                       &[libc::c_char; 16]>(b"check_all_certs\x00")).as_ptr(),
             retval, *match_found);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "617:1"]
pub unsafe extern "C" fn pkinit_cert_matching(mut context: krb5_context,
                                              mut plg_cryptoctx:
                                                  pkinit_plg_crypto_context,
                                              mut req_cryptoctx:
                                                  pkinit_req_crypto_context,
                                              mut id_cryptoctx:
                                                  pkinit_identity_crypto_context,
                                              mut princ: krb5_principal)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code =
        -(1765328360 as libc::c_long) as krb5_error_code;
    let mut x: libc::c_int = 0;
    let mut rules: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut rs: *mut rule_set = 0 as *mut rule_set;
    let mut match_found: libc::c_int = 0 as libc::c_int;
    let mut matchdata: *mut *mut pkinit_cert_matching_data =
        0 as *mut *mut pkinit_cert_matching_data;
    let mut match_index: size_t = 0 as libc::c_int as size_t;
    /* If no matching rules, select the default cert and we're done */
    pkinit_libdefault_strings(context, &mut (*princ).realm,
                              b"pkinit_cert_match\x00" as *const u8 as
                                  *const libc::c_char, &mut rules);
    if rules.is_null() {
        pkiDebug(b"%s: no matching rules found in config file\n\x00" as
                     *const u8 as *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 21],
                                           &[libc::c_char; 21]>(b"pkinit_cert_matching\x00")).as_ptr());
        retval =
            crypto_cert_select_default(context, plg_cryptoctx, req_cryptoctx,
                                       id_cryptoctx)
    } else {
        /* parse each rule line one at a time and check all the certs against it */
        x = 0 as libc::c_int;
        loop  {
            if (*rules.offset(x as isize)).is_null() {
                current_block = 3275366147856559585;
                break ;
            }
            pkiDebug(b"%s: Processing rule \'%s\'\n\x00" as *const u8 as
                         *const libc::c_char,
                     (*::std::mem::transmute::<&[u8; 21],
                                               &[libc::c_char; 21]>(b"pkinit_cert_matching\x00")).as_ptr(),
                     *rules.offset(x as isize));
            /* Free rules from previous time through... */
            if !rs.is_null() {
                free_rule_set(context, rs);
                rs = 0 as *mut rule_set
            }
            retval =
                parse_rule_set(context, *rules.offset(x as isize), &mut rs);
            if retval != 0 {
                if !(retval == 22 as libc::c_int) {
                    current_block = 14889457129568826429;
                    break ;
                }
                pkiDebug(b"%s: Ignoring invalid rule pkinit_cert_match = \'%s\'\n\x00"
                             as *const u8 as *const libc::c_char,
                         (*::std::mem::transmute::<&[u8; 21],
                                                   &[libc::c_char; 21]>(b"pkinit_cert_matching\x00")).as_ptr(),
                         *rules.offset(x as isize));
            } else {
                /*
         * Optimize so that we do not get cert info unless we have
         * valid rules to check.  Once obtained, keep it around
         * until we are done.
         */
                if matchdata.is_null() {
                    retval =
                        crypto_cert_get_matching_data(context, plg_cryptoctx,
                                                      req_cryptoctx,
                                                      id_cryptoctx,
                                                      &mut matchdata); /* XXX */
                    if retval != 0 || matchdata.is_null() {
                        pkiDebug(b"%s: Error %d obtaining certificate information\n\x00"
                                     as *const u8 as *const libc::c_char,
                                 (*::std::mem::transmute::<&[u8; 21],
                                                           &[libc::c_char; 21]>(b"pkinit_cert_matching\x00")).as_ptr(),
                                 retval);
                        retval = 2 as libc::c_int;
                        current_block = 14889457129568826429;
                        break ;
                    }
                }
                retval =
                    check_all_certs(context, plg_cryptoctx, req_cryptoctx,
                                    id_cryptoctx, princ, rs, matchdata,
                                    &mut match_found, &mut match_index);
                if retval != 0 {
                    pkiDebug(b"%s: Error %d, checking certs against rule \'%s\'\n\x00"
                                 as *const u8 as *const libc::c_char,
                             (*::std::mem::transmute::<&[u8; 21],
                                                       &[libc::c_char; 21]>(b"pkinit_cert_matching\x00")).as_ptr(),
                             retval, *rules.offset(x as isize));
                    current_block = 14889457129568826429;
                    break ;
                } else if match_found != 0 {
                    pkiDebug(b"%s: We have an exact match with rule \'%s\'\n\x00"
                                 as *const u8 as *const libc::c_char,
                             (*::std::mem::transmute::<&[u8; 21],
                                                       &[libc::c_char; 21]>(b"pkinit_cert_matching\x00")).as_ptr(),
                             *rules.offset(x as isize));
                    current_block = 3275366147856559585;
                    break ;
                }
            }
            x += 1
        }
        match current_block {
            14889457129568826429 => { }
            _ => {
                if match_found != 0 {
                    pkiDebug(b"%s: Selecting the matching cert!\n\x00" as
                                 *const u8 as *const libc::c_char,
                             (*::std::mem::transmute::<&[u8; 21],
                                                       &[libc::c_char; 21]>(b"pkinit_cert_matching\x00")).as_ptr());
                    retval =
                        crypto_cert_select(context, id_cryptoctx,
                                           match_index);
                    if retval != 0 {
                        pkiDebug(b"%s: crypto_cert_select error %d, %s\n\x00"
                                     as *const u8 as *const libc::c_char,
                                 (*::std::mem::transmute::<&[u8; 21],
                                                           &[libc::c_char; 21]>(b"pkinit_cert_matching\x00")).as_ptr(),
                                 retval, error_message(retval as errcode_t));
                    } else { retval = 0 as libc::c_int }
                } else {
                    krb5int_trace(context,
                                  b"PKINIT no matching certificate found\x00"
                                      as *const u8 as *const libc::c_char);
                    retval = 2 as libc::c_int
                }
            }
        }
    }
    profile_free_list(rules);
    free_rule_set(context, rs);
    crypto_cert_free_matching_data_list(context, matchdata);
    return retval;
}
/*
 * COPYRIGHT (C) 2006,2007
 * THE REGENTS OF THE UNIVERSITY OF MICHIGAN
 * ALL RIGHTS RESERVED
 *
 * Permission is granted to use, copy, create derivative works
 * and redistribute this software and such derivative works
 * for any purpose, so long as the name of The University of
 * Michigan is not used in any advertising or publicity
 * pertaining to the use of distribution of this software
 * without specific, written prior authorization.  If the
 * above copyright notice or any other identification of the
 * University of Michigan is included in any copy of any
 * portion of this software, then the disclaimer below must
 * also be included.
 *
 * THIS SOFTWARE IS PROVIDED AS IS, WITHOUT REPRESENTATION
 * FROM THE UNIVERSITY OF MICHIGAN AS TO ITS FITNESS FOR ANY
 * PURPOSE, AND WITHOUT WARRANTY BY THE UNIVERSITY OF
 * MICHIGAN OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING
 * WITHOUT LIMITATION THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE
 * REGENTS OF THE UNIVERSITY OF MICHIGAN SHALL NOT BE LIABLE
 * FOR ANY DAMAGES, INCLUDING SPECIAL, INDIRECT, INCIDENTAL, OR
 * CONSEQUENTIAL DAMAGES, WITH RESPECT TO ANY CLAIM ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OF THE SOFTWARE, EVEN
 * IF IT HAS BEEN OR IS HEREAFTER ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGES.
 */
/* pkinit_kdc_ocsp has been removed */
/* Make pkiDebug(fmt,...) print, or not.  */
/* Still evaluates for side effects.  */
/* This is better if the compiler doesn't inline variadic functions
   well, but gcc will warn about "left-hand operand of comma
   expression has no effect".  Still evaluates for side effects.  */
/* #define pkiDebug	(void) */
/* Solaris compiler doesn't grok __FUNCTION__
 * hack for now.  Fix all the uses eventually. */
/* Macros to deal with converting between various data types... */
/*
 * notes about crypto contexts:
 *
 * the basic idea is that there are crypto contexts that live at
 * both the plugin level and request level. the identity context (that
 * keeps info about your own certs and such) is separate because
 * it is needed at different levels for the kdc and and the client.
 * (the kdc's identity is at the plugin level, the client's identity
 * information could change per-request.)
 * the identity context is meant to have the entity's cert,
 * a list of trusted and intermediate cas, a list of crls, and any
 * pkcs11 information.  the req context is meant to have the
 * received certificate and the DH related information. the plugin
 * context is meant to have global crypto information, i.e., OIDs
 * and constant DH parameter information.
 */
/*
 * plugin crypto context should keep plugin common information,
 * eg., OIDs, known DHparams
 */
/*
 * request crypto context should keep reqyest common information,
 * eg., received credentials, DH parameters of this request
 */
/*
 * identity context should keep information about credentials
 * for the request, eg., my credentials, trusted ca certs,
 * intermediate ca certs, crls, pkcs11 info
 */
/*
 * this structure keeps information about the config options
 */
/* require EKU checking (default is true) */
/* accept secondary EKU (default is false) */
/* allow UPN-SAN instead of pkinit-SAN */
/* selects DH or RSA based pkinit */
/* require CRL for a CA (default is false) */
/* require freshness token (default is false) */
/* disable freshness token on client for testing */
/* minimum DH modulus size allowed */
/*
 * this structure keeps options used for a given request
 */
/* initial request DH modulus size (default=1024) */
/*
 * information about identity from config file or command line
 */
/*
 * Client's plugin context
 */
/*
 * Client's per-request context
 */
/*
 * KDC's (per-realm) plugin context
 */
/*
 * KDC's per-request context
 */
/*
 * Functions in pkinit_lib.c
 */
/*
 * Functions in pkinit_identity.c
 */
/* IN */
/* IN */
/* IN */
/* IN */
/* IN/OUT */
/* IN/OUT */
/* IN/OUT */
/* IN (optional) */
/* IN */
/* IN */
/* IN */
/* IN */
/* IN/OUT */
/* IN/OUT */
/* IN/OUT */
/* IN */
/* IN (optional) */
#[no_mangle]
#[c2rust::src_loc = "717:1"]
pub unsafe extern "C" fn pkinit_client_cert_match(mut context: krb5_context,
                                                  mut plgctx:
                                                      pkinit_plg_crypto_context,
                                                  mut reqctx:
                                                      pkinit_req_crypto_context,
                                                  mut match_rule:
                                                      *const libc::c_char,
                                                  mut matched:
                                                      *mut krb5_boolean)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut md: *mut pkinit_cert_matching_data =
        0 as *mut pkinit_cert_matching_data;
    let mut rc: *mut rule_component = 0 as *mut rule_component;
    let mut comp_match: libc::c_int = 0 as libc::c_int;
    let mut rs: *mut rule_set = 0 as *mut rule_set;
    *matched = 0 as libc::c_int as krb5_boolean;
    ret = parse_rule_set(context, match_rule, &mut rs);
    if !(ret != 0) {
        ret = crypto_req_cert_matching_data(context, plgctx, reqctx, &mut md);
        if !(ret != 0) {
            rc = (*rs).crs;
            while !rc.is_null() {
                comp_match = component_match(context, rc, md);
                if comp_match != 0 &&
                       (*rs).relation as libc::c_uint ==
                           relation_or as libc::c_int as libc::c_uint ||
                       comp_match == 0 &&
                           (*rs).relation as libc::c_uint ==
                               relation_and as libc::c_int as libc::c_uint {
                    break ;
                }
                rc = (*rc).next
            }
            *matched = comp_match as krb5_boolean
        }
    }
    free_rule_set(context, rs);
    crypto_cert_free_matching_data(context, md);
    return ret;
}
