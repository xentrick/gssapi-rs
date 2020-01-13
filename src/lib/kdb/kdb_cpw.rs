use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:52"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:52"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:52"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:52"]
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
    #[c2rust::src_loc = "8510:1"]
    pub type krb5_post_recv_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: krb5_error_code, _: *const krb5_data,
                                    _: *const krb5_data, _: *const krb5_data,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    /* per Kerberos v5 protocol spec */
    /* not yet in the spec... */
    /* macros to determine if a type is a local type */
    /*
 * end "hostaddr.h"
 */
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
    #[c2rust::src_loc = "8475:1"]
    pub type krb5_pre_send_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: *const krb5_data, _: *const krb5_data,
                                    _: *mut *mut krb5_data,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "8391:1"]
    pub type krb5_trace_callback
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *const krb5_trace_info,
                                    _: *mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "8387:1"]
    pub type krb5_trace_info = _krb5_trace_info;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8387:16"]
    pub struct _krb5_trace_info {
        pub message: *const libc::c_char,
    }
    #[c2rust::src_loc = "7865:1"]
    pub type krb5_prompt_type = krb5_int32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "363:16"]
    pub struct _krb5_keyblock {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /*
 * begin "encryption.h"
 */
    /* * Exposed contents of a key. */
    #[c2rust::src_loc = "363:1"]
    pub type krb5_keyblock = _krb5_keyblock;
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        /* This file is generated, please don't edit it directly.  */
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* General definitions for Kerberos version 5. */
/*
 * Copyright 1989, 1990, 1995, 2001, 2003, 2007, 2011 by the Massachusetts
 * Institute of Technology.  All Rights Reserved.
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
        /* * @defgroup KRB5_H krb5 library API
 * @{
 */
        /* By default, do not expose deprecated interfaces. */
        /* !KRB5_CONFIG__ */
        /* for *_MAX */
        /* from profile.h */
        #[c2rust::src_loc = "125:8"]
        pub type _profile_t;
        /* *
 * Generate an enctype-specific random encryption key.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type of the generated key
 * @param [out] k5_random_key   An allocated and initialized keyblock
 *
 * Use krb5_free_keyblock_contents() to free @a k5_random_key when
 * no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "745:1"]
        pub fn krb5_c_make_random_key(context: krb5_context,
                                      enctype: krb5_enctype,
                                      k5_random_key: *mut krb5_keyblock)
         -> krb5_error_code;
        /* *
 * Generate pseudo-random bytes.
 *
 * @param [in]  context         Library context
 * @param [out] data            Random data
 *
 * Fills in @a data with bytes from the PRNG used by krb5 crypto operations.
 * The caller must preinitialize @a data and allocate the desired amount of
 * space.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "800:1"]
        pub fn krb5_c_random_make_octets(context: krb5_context,
                                         data: *mut krb5_data)
         -> krb5_error_code;
        /* *
 * Convert a string (such as a password) to a key with additional parameters.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [in]  string          String to be converted
 * @param [in]  salt            Salt value
 * @param [in]  params          Parameters
 * @param [out] key             Generated key
 *
 * This function is similar to krb5_c_string_to_key(), but also takes
 * parameters which may affect the algorithm in an enctype-dependent way.  The
 * newly created @a key must be released by calling
 * krb5_free_keyblock_contents() when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "861:1"]
        pub fn krb5_c_string_to_key_with_params(context: krb5_context,
                                                enctype: krb5_enctype,
                                                string: *const krb5_data,
                                                salt: *const krb5_data,
                                                params: *const krb5_data,
                                                key: *mut krb5_keyblock)
         -> krb5_error_code;
        /* *
 * Compare two encryption types.
 *
 * @param [in]  context         Library context
 * @param [in]  e1              First encryption type
 * @param [in]  e2              Second encryption type
 * @param [out] similar         @c TRUE if types are similar, @c FALSE if not
 *
 * This function determines whether two encryption types use the same kind of
 * keys.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "882:1"]
        pub fn krb5_c_enctype_compare(context: krb5_context, e1: krb5_enctype,
                                      e2: krb5_enctype,
                                      similar: *mut krb5_boolean)
         -> krb5_error_code;
        /* *
 * Copy a krb5_data object.
 *
 * @param [in]  context           Library context
 * @param [in]  indata            Data object to be copied
 * @param [out] outdata           Copy of @a indata
 *
 * This function creates a new krb5_data object with the contents of @a indata.
 * Use krb5_free_data() to free @a outdata when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3797:1"]
        pub fn krb5_copy_data(context: krb5_context, indata: *const krb5_data,
                              outdata: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4313:1"]
        pub fn krb5_principal2salt(context: krb5_context,
                                   pr: krb5_const_principal,
                                   ret: *mut krb5_data) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4721:1"]
        pub fn krb5_free_keyblock_contents(context: krb5_context,
                                           key: *mut krb5_keyblock);
        #[no_mangle]
        #[c2rust::src_loc = "4758:1"]
        pub fn krb5_free_data_contents(context: krb5_context,
                                       val: *mut krb5_data);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:52"]
pub mod k5_int_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1208:8"]
    pub struct _krb5_context {
        pub magic: krb5_magic,
        pub in_tkt_etypes: *mut krb5_enctype,
        pub tgs_etypes: *mut krb5_enctype,
        pub os_context: _krb5_os_context,
        pub default_realm: *mut libc::c_char,
        pub profile: profile_t,
        pub dal_handle: *mut kdb5_dal_handle,
        pub clockskew: krb5_deltat,
        pub kdc_default_options: krb5_flags,
        pub library_options: krb5_flags,
        pub profile_secure: krb5_boolean,
        pub fcc_default_format: libc::c_int,
        pub prompt_types: *mut krb5_prompt_type,
        pub udp_pref_limit: libc::c_int,
        pub use_conf_ktypes: krb5_boolean,
        pub libkrb5_plugins: plugin_dir_handle,
        pub preauth_context: krb5_preauth_context,
        pub ccselect_handles: *mut *mut ccselect_module_handle,
        pub localauth_handles: *mut *mut localauth_module_handle,
        pub hostrealm_handles: *mut *mut hostrealm_module_handle,
        pub tls: *mut k5_tls_vtable_st,
        pub err: errinfo,
        pub err_fmt: *mut libc::c_char,
        pub kdblog_context: *mut _kdb_log_context,
        pub allow_weak_crypto: krb5_boolean,
        pub ignore_acceptor_hostname: krb5_boolean,
        pub enforce_ok_as_delegate: krb5_boolean,
        pub dns_canonicalize_hostname: dns_canonhost,
        pub trace_callback: krb5_trace_callback,
        pub trace_callback_data: *mut libc::c_void,
        pub kdc_send_hook: krb5_pre_send_fn,
        pub kdc_send_hook_data: *mut libc::c_void,
        pub kdc_recv_hook: krb5_post_recv_fn,
        pub kdc_recv_hook_data: *mut libc::c_void,
        pub plugins: [plugin_interface; 13],
        pub plugin_base_dir: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1137:8"]
    pub struct plugin_interface {
        pub modules: *mut *mut plugin_mapping,
        pub configured: krb5_boolean,
    }
    #[c2rust::src_loc = "1194:1"]
    pub type dns_canonhost = libc::c_uint;
    #[c2rust::src_loc = "1197:5"]
    pub const CANONHOST_FALLBACK: dns_canonhost = 2;
    #[c2rust::src_loc = "1196:5"]
    pub const CANONHOST_TRUE: dns_canonhost = 1;
    #[c2rust::src_loc = "1195:5"]
    pub const CANONHOST_FALSE: dns_canonhost = 0;
    #[c2rust::src_loc = "1203:1"]
    pub type krb5_preauth_context = *mut krb5_preauth_context_st;
    #[c2rust::src_loc = "1201:1"]
    pub type kdb5_dal_handle = _kdb5_dal_handle;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "702:16"]
    pub struct _krb5_os_context {
        pub magic: krb5_magic,
        pub time_offset: krb5_int32,
        pub usec_offset: krb5_int32,
        pub os_flags: krb5_int32,
        pub default_ccname: *mut libc::c_char,
    }
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
    #[inline]
    #[c2rust::src_loc = "2268:1"]
    pub unsafe extern "C" fn string2data(mut str: *mut libc::c_char)
     -> krb5_data {
        return make_data(str as *mut libc::c_void,
                         strlen(str) as libc::c_uint);
    }
    #[inline]
    #[c2rust::src_loc = "2274:1"]
    pub unsafe extern "C" fn alloc_data(mut data: *mut krb5_data,
                                        mut len: libc::c_uint)
     -> krb5_error_code {
        let mut ptr: *mut libc::c_char =
            calloc(if len > 0 as libc::c_int as libc::c_uint {
                       len
                   } else { 1 as libc::c_int as libc::c_uint } as
                       libc::c_ulong, 1 as libc::c_int as libc::c_ulong) as
                *mut libc::c_char;
        if ptr.is_null() { return 12 as libc::c_int }
        (*data).magic = -(1760647422 as libc::c_long) as krb5_magic;
        (*data).data = ptr;
        (*data).length = len;
        return 0 as libc::c_int;
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_context, krb5_principal_data,
                        krb5_const_principal, krb5_data, krb5_error_code};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::string_h::strlen;
    use super::stdlib_h::calloc;
    extern "C" {
        #[c2rust::src_loc = "1134:8"]
        pub type plugin_mapping;
        #[c2rust::src_loc = "1202:8"]
        pub type _kdb_log_context;
        #[c2rust::src_loc = "1207:8"]
        pub type k5_tls_vtable_st;
        #[c2rust::src_loc = "1206:8"]
        pub type hostrealm_module_handle;
        #[c2rust::src_loc = "1205:8"]
        pub type localauth_module_handle;
        #[c2rust::src_loc = "1204:8"]
        pub type ccselect_module_handle;
        #[c2rust::src_loc = "1203:16"]
        pub type krb5_preauth_context_st;
        #[c2rust::src_loc = "1200:8"]
        pub type _kdb5_dal_handle;
        #[no_mangle]
        #[c2rust::src_loc = "2129:1"]
        pub fn krb5_principal2salt_norealm(_: krb5_context,
                                           _: krb5_const_principal,
                                           _: *mut krb5_data)
         -> krb5_error_code;
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:52"]
pub mod k5_err_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-err.h */
/*
 * Copyright 2006, 2007 Massachusetts Institute of Technology.
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
 * Error-message handling
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:8"]
    pub struct errinfo {
        pub code: libc::c_long,
        pub msg: *mut libc::c_char,
    }
    /* K5_ERR_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:52"]
pub mod k5_plugin_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2006 Massachusetts Institute of Technology.
 * All Rights Reserved.
 *
 * This software is being provided to you, the LICENSEE, by the
 * Massachusetts Institute of Technology (M.I.T.) under the following
 * license.  By obtaining, using and/or copying this software, you agree
 * that you have read, understood, and will comply with these terms and
 * conditions:
 *
 * Export of this software from the United States of America may
 * require a specific license from the United States Government.
 * It is the responsibility of any person or organization contemplating
 * export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify and distribute
 * this software and its documentation for any purpose and without fee or
 * royalty is hereby granted, provided that you agree to comply with the
 * following copyright notice and statements, including the disclaimer, and
 * that the same appear on ALL copies of the software and documentation,
 * including modifications that you make for internal use or for
 * distribution:
 *
 * THIS SOFTWARE IS PROVIDED "AS IS", AND M.I.T. MAKES NO REPRESENTATIONS
 * OR WARRANTIES, EXPRESS OR IMPLIED.  By way of example, but not
 * limitation, M.I.T. MAKES NO REPRESENTATIONS OR WARRANTIES OF
 * MERCHANTABILITY OR FITNESS FOR ANY PARTICULAR PURPOSE OR THAT THE USE OF
 * THE LICENSED SOFTWARE OR DOCUMENTATION WILL NOT INFRINGE ANY THIRD PARTY
 * PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
 *
 * The name of the Massachusetts Institute of Technology or M.I.T. may NOT
 * be used in advertising or publicity pertaining to distribution of the
 * software.  Title to copyright in this software and any associated
 * documentation shall at all times remain with M.I.T., and USER agrees to
 * preserve same.
 *
 * Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 */
    /* Just those definitions which are needed by util/support/plugins.c,
   which gets compiled before util/et is built, which happens before
   we can construct krb5.h, which is included by k5-int.h.

   So, no krb5 types.  */
    /*
 * Plugins normally export fixed symbol names, but when statically
 * linking plugins, we need a different symbol name for each plugin.
 * The first argument to PLUGIN_SYMBOL_NAME acts as the
 * differentiator, and is only used for static plugin linking.
 *
 * Although this macro (and thus this header file) are used in plugins
 * whose code lies inside the krb5 tree, plugins maintained separately
 * from the krb5 tree do not need it; they can just use the fixed
 * symbol name unconditionally.
 */
    /* opaque */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:8"]
    pub struct plugin_dir_handle {
        pub files: *mut *mut plugin_file_handle,
    }
    extern "C" {
        #[c2rust::src_loc = "80:8"]
        pub type plugin_file_handle;
    }
    /* K5_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:52"]
pub mod profile_h {
    /*
 * profile.h
 */
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:53"]
pub mod kdb_h {
    /*
 * If this ever changes up the version number and make the arrays be as
 * big as necessary.
 *
 * Currently the first type is the enctype and the second is the salt type.
 */
    #[c2rust::src_loc = "167:1"]
    pub type krb5_key_data = _krb5_key_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "167:16"]
    pub struct _krb5_key_data {
        pub key_data_ver: krb5_int16,
        pub key_data_kvno: krb5_ui_2,
        pub key_data_type: [krb5_int16; 2],
        pub key_data_length: [krb5_ui_2; 2],
        pub key_data_contents: [*mut krb5_octet; 2],
    }
    /* Version */
    /* Key Version */
    /* Array of types */
    /* Array of lengths */
    /* Array of pointers */
    /* # of array elements */
    #[c2rust::src_loc = "177:1"]
    pub type krb5_keysalt = _krb5_keysalt;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "177:16"]
    pub struct _krb5_keysalt {
        pub type_0: krb5_int16,
        pub data: krb5_data,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "147:16"]
    pub struct _krb5_tl_data {
        pub tl_data_next: *mut _krb5_tl_data,
        pub tl_data_type: krb5_int16,
        pub tl_data_length: krb5_ui_2,
        pub tl_data_contents: *mut krb5_octet,
    }
    /* Length, data */
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
    #[c2rust::src_loc = "147:1"]
    pub type krb5_tl_data = _krb5_tl_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "191:16"]
    pub struct _krb5_db_entry_new {
        pub magic: krb5_magic,
        pub len: krb5_ui_2,
        pub mask: krb5_ui_4,
        pub attributes: krb5_flags,
        pub max_life: krb5_deltat,
        pub max_renewable_life: krb5_deltat,
        pub expiration: krb5_timestamp,
        pub pw_expiration: krb5_timestamp,
        pub last_success: krb5_timestamp,
        pub last_failed: krb5_timestamp,
        pub fail_auth_count: krb5_kvno,
        pub n_tl_data: krb5_int16,
        pub n_key_data: krb5_int16,
        pub e_length: krb5_ui_2,
        pub e_data: *mut krb5_octet,
        pub princ: krb5_principal,
        pub tl_data: *mut krb5_tl_data,
        pub key_data: *mut krb5_key_data,
    }
    /* NOT saved */
    /*
 * A principal database entry.  Extensions to this structure currently use the
 * tl_data list.  The e_data and e_length fields are not used by any calling
 * code except kdb5_util dump and load, which marshal and unmarshal the array
 * in the dump record.  KDB modules may use these fields internally as long as
 * they set e_length appropriately (non-zero if the data should be marshalled
 * across dump and load, zero if not) and handle null e_data values in
 * caller-constructed principal entries.
 */
    #[c2rust::src_loc = "191:1"]
    pub type krb5_db_entry = _krb5_db_entry_new;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    use super::krb5_h::{krb5_int16, krb5_ui_2, krb5_octet, krb5_data,
                        krb5_magic, krb5_ui_4, krb5_flags, krb5_deltat,
                        krb5_timestamp, krb5_kvno, krb5_principal,
                        krb5_enctype, krb5_int32, krb5_context, krb5_keyblock,
                        krb5_error_code};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        /* NOT saved */
        /* members currently changed/set */
        /* When the client expires */
        /* When its passwd expires */
        /* Last successful passwd */
        /* Last failed passwd attempt */
        /* # of failed passwd attempt */
        /* Length of extra data */
        /* Extra data to be saved */
        /* Length, data */
        /* Linked list */
        /* key_data must be sorted by kvno in descending order. */
        /* Array */
        /* *
 * Decrypts the key given in @@a key_data. If @a mkey is specified, that
 * master key is used. If @a mkey is NULL, then all master keys are tried.
 */
        #[no_mangle]
        #[c2rust::src_loc = "446:1"]
        pub fn krb5_dbe_decrypt_key_data(context: krb5_context,
                                         mkey: *const krb5_keyblock,
                                         key_data: *const krb5_key_data,
                                         dbkey: *mut krb5_keyblock,
                                         keysalt: *mut krb5_keysalt)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "453:1"]
        pub fn krb5_dbe_encrypt_key_data(context: krb5_context,
                                         mkey: *const krb5_keyblock,
                                         dbkey: *const krb5_keyblock,
                                         keysalt: *const krb5_keysalt,
                                         keyver: libc::c_int,
                                         key_data: *mut krb5_key_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "537:1"]
        pub fn krb5_dbe_create_key_data(context: krb5_context,
                                        entry: *mut krb5_db_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "848:1"]
        pub fn krb5_dbe_free_key_data_contents(_: krb5_context,
                                               _: *mut krb5_key_data);
    }
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src = "/usr/include/stdlib.h:52"]
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
#[c2rust::header_src = "/usr/include/string.h:52"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_kvno, krb5_enctype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal,
                       krb5_const_principal, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _krb5_keyblock,
                       krb5_keyblock, _profile_t, krb5_c_make_random_key,
                       krb5_c_random_make_octets,
                       krb5_c_string_to_key_with_params,
                       krb5_c_enctype_compare, krb5_copy_data,
                       krb5_principal2salt, krb5_free_keyblock_contents,
                       krb5_free_data_contents};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, make_data, string2data, alloc_data,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5_principal2salt_norealm};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::kdb_h::{krb5_key_data, _krb5_key_data, krb5_keysalt,
                      _krb5_keysalt, _krb5_tl_data, krb5_tl_data,
                      _krb5_db_entry_new, krb5_db_entry,
                      __krb5_key_salt_tuple, krb5_key_salt_tuple,
                      krb5_dbe_decrypt_key_data, krb5_dbe_encrypt_key_data,
                      krb5_dbe_create_key_data,
                      krb5_dbe_free_key_data_contents};
use self::stdlib_h::{calloc, free};
use self::string_h::{strlen, memset};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/kdb/kdb_cpw.c */
/*
 * Copyright 1995, 2009, 2014 by the Massachusetts Institute of Technology.
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
#[c2rust::src_loc = "57:1"]
pub type save = libc::c_uint;
#[c2rust::src_loc = "57:42"]
pub const KEEP_ALL: save = 2;
#[c2rust::src_loc = "57:26"]
pub const KEEP_LAST_KVNO: save = 1;
#[c2rust::src_loc = "57:13"]
pub const DISCARD_ALL: save = 0;
#[no_mangle]
#[c2rust::src_loc = "59:1"]
pub unsafe extern "C" fn krb5_db_get_key_data_kvno(mut context: krb5_context,
                                                   mut count: libc::c_int,
                                                   mut data:
                                                       *mut krb5_key_data)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut kvno: libc::c_int = 0;
    /* Find last key version number */
    i = 0 as libc::c_int;
    kvno = i;
    while i < count {
        if kvno < (*data.offset(i as isize)).key_data_kvno as libc::c_int {
            kvno = (*data.offset(i as isize)).key_data_kvno as libc::c_int
        }
        i += 1
    }
    return kvno;
}
#[c2rust::src_loc = "75:1"]
unsafe extern "C" fn cleanup_key_data(mut context: krb5_context,
                                      mut count: libc::c_int,
                                      mut data: *mut krb5_key_data) {
    let mut i: libc::c_int = 0;
    /* If data is NULL, count is always 0 */
    if data.is_null() { return }
    i = 0 as libc::c_int;
    while i < count {
        krb5_dbe_free_key_data_contents(context,
                                        &mut *data.offset(i as isize));
        i += 1
    }
    free(data as *mut libc::c_void);
}
/* Transfer key data from old_kd to new_kd, making sure that new_kd is
 * encrypted with mkey.  May steal from old_kd and zero it out. */
#[c2rust::src_loc = "93:1"]
unsafe extern "C" fn preserve_one_old_key(mut context: krb5_context,
                                          mut mkey: *mut krb5_keyblock,
                                          mut dbent: *mut krb5_db_entry,
                                          mut old_kd: *mut krb5_key_data,
                                          mut new_kd: *mut krb5_key_data)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut kb: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut salt: krb5_keysalt =
        krb5_keysalt{type_0: 0,
                     data:
                         krb5_data{magic: 0,
                                   length: 0,
                                   data: 0 as *mut libc::c_char,},};
    memset(new_kd as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<krb5_key_data>() as libc::c_ulong);
    ret =
        krb5_dbe_decrypt_key_data(context, mkey, old_kd, &mut kb,
                                  0 as *mut krb5_keysalt);
    if ret == 0 as libc::c_int {
        /* old_kd is already encrypted in mkey, so just move it. */
        *new_kd = *old_kd;
        memset(old_kd as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<krb5_key_data>() as libc::c_ulong);
        krb5_free_keyblock_contents(context, &mut kb);
        return 0 as libc::c_int
    }
    /* Decrypt and re-encrypt old_kd using mkey. */
    ret =
        krb5_dbe_decrypt_key_data(context, 0 as *const krb5_keyblock, old_kd,
                                  &mut kb, &mut salt);
    if ret != 0 { return ret }
    ret =
        krb5_dbe_encrypt_key_data(context, mkey, &mut kb, &mut salt,
                                  (*old_kd).key_data_kvno as libc::c_int,
                                  new_kd);
    krb5_free_keyblock_contents(context, &mut kb);
    krb5_free_data_contents(context, &mut salt.data);
    return ret;
}
/*
 * Add key_data to dbent, making sure that each entry is encrypted in mkey.  If
 * kvno is non-zero, preserve only keys of that kvno.  May steal some elements
 * from key_data and zero them out.
 */
#[c2rust::src_loc = "129:1"]
unsafe extern "C" fn preserve_old_keys(mut context: krb5_context,
                                       mut mkey: *mut krb5_keyblock,
                                       mut dbent: *mut krb5_db_entry,
                                       mut kvno: libc::c_int,
                                       mut n_key_data: libc::c_int,
                                       mut key_data: *mut krb5_key_data)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n_key_data {
        if !(kvno != 0 as libc::c_int &&
                 (*key_data.offset(i as isize)).key_data_kvno as libc::c_int
                     != kvno) {
            ret = krb5_dbe_create_key_data(context, dbent);
            if ret != 0 { return ret }
            ret =
                preserve_one_old_key(context, mkey, dbent,
                                     &mut *key_data.offset(i as isize),
                                     &mut *(*dbent).key_data.offset(((*dbent).n_key_data
                                                                         as
                                                                         libc::c_int
                                                                         -
                                                                         1 as
                                                                             libc::c_int)
                                                                        as
                                                                        isize));
            if ret != 0 { return ret }
        }
        i += 1
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "151:1"]
unsafe extern "C" fn add_key_rnd(mut context: krb5_context,
                                 mut master_key: *mut krb5_keyblock,
                                 mut ks_tuple: *mut krb5_key_salt_tuple,
                                 mut ks_tuple_count: libc::c_int,
                                 mut db_entry: *mut krb5_db_entry,
                                 mut kvno: libc::c_int) -> krb5_error_code {
    let mut key: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut retval: krb5_error_code = 0;
    let mut kd_slot: *mut krb5_key_data = 0 as *mut krb5_key_data;
    i = 0 as libc::c_int;
    while i < ks_tuple_count {
        let mut similar: krb5_boolean = 0;
        similar = 0 as libc::c_int as krb5_boolean;
        /*
         * We could use krb5_keysalt_iterate to replace this loop, or use
         * krb5_keysalt_is_present for the loop below, but we want to avoid
         * circular library dependencies.
         */
        j = 0 as libc::c_int;
        while j < i {
            retval =
                krb5_c_enctype_compare(context,
                                       (*ks_tuple.offset(i as
                                                             isize)).ks_enctype,
                                       (*ks_tuple.offset(j as
                                                             isize)).ks_enctype,
                                       &mut similar);
            if retval != 0 { return retval }
            if similar != 0 { break ; }
            j += 1
        }
        if !(similar != 0) {
            retval = krb5_dbe_create_key_data(context, db_entry);
            if retval != 0 { return retval }
            kd_slot =
                &mut *(*db_entry).key_data.offset(((*db_entry).n_key_data as
                                                       libc::c_int -
                                                       1 as libc::c_int) as
                                                      isize) as
                    *mut krb5_key_data;
            /* there used to be code here to extract the old key, and derive
           a new key from it.  Now that there's a unified prng, that isn't
           necessary. */
            /* make new key */
            retval =
                krb5_c_make_random_key(context,
                                       (*ks_tuple.offset(i as
                                                             isize)).ks_enctype,
                                       &mut key);
            if retval != 0 { return retval }
            retval =
                krb5_dbe_encrypt_key_data(context, master_key, &mut key,
                                          0 as *const krb5_keysalt, kvno,
                                          kd_slot);
            krb5_free_keyblock_contents(context, &mut key);
            if retval != 0 { return retval }
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/* Construct a random explicit salt. */
#[c2rust::src_loc = "214:1"]
unsafe extern "C" fn make_random_salt(mut context: krb5_context,
                                      mut salt_out: *mut krb5_keysalt)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut rndbuf: [libc::c_uchar; 8] = [0; 8];
    let mut salt: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut rnd: krb5_data =
        make_data(rndbuf.as_mut_ptr() as *mut libc::c_void,
                  ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong
                      as libc::c_uint);
    let mut i: libc::c_uint = 0;
    /*
     * Salts are limited by RFC 4120 to 7-bit ASCII.  For ease of examination
     * and to avoid certain folding issues for older enctypes, we use printable
     * characters with four fixed bits and four random bits, encoding 64
     * psuedo-random bits into 16 bytes.
     */
    retval = krb5_c_random_make_octets(context, &mut rnd);
    if retval != 0 { return retval }
    retval =
        alloc_data(&mut salt,
                   (::std::mem::size_of::<[libc::c_uchar; 8]>() as
                        libc::c_ulong).wrapping_mul(2 as libc::c_int as
                                                        libc::c_ulong) as
                       libc::c_uint);
    if retval != 0 { return retval }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) <
              ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong {
        *salt.data.offset(i.wrapping_mul(2 as libc::c_int as libc::c_uint) as
                              isize) =
            (0x40 as libc::c_int |
                 rndbuf[i as usize] as libc::c_int >> 4 as libc::c_int) as
                libc::c_char;
        *salt.data.offset(i.wrapping_mul(2 as libc::c_int as
                                             libc::c_uint).wrapping_add(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
                              as isize) =
            (0x40 as libc::c_int |
                 rndbuf[i as usize] as libc::c_int & 0xf as libc::c_int) as
                libc::c_char;
        i = i.wrapping_add(1)
    }
    (*salt_out).type_0 = 4 as libc::c_int as krb5_int16;
    (*salt_out).data = salt;
    return 0 as libc::c_int;
}
/*
 * Add key_data for a krb5_db_entry
 * If passwd is NULL the assumes that the caller wants a random password.
 */
#[c2rust::src_loc = "248:1"]
unsafe extern "C" fn add_key_pwd(mut context: krb5_context,
                                 mut master_key: *mut krb5_keyblock,
                                 mut ks_tuple: *mut krb5_key_salt_tuple,
                                 mut ks_tuple_count: libc::c_int,
                                 mut passwd: *const libc::c_char,
                                 mut db_entry: *mut krb5_db_entry,
                                 mut kvno: libc::c_int) -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut key_salt: krb5_keysalt =
        krb5_keysalt{type_0: 0,
                     data:
                         krb5_data{magic: 0,
                                   length: 0,
                                   data: 0 as *mut libc::c_char,},};
    let mut key: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut pwd: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut kd_slot: *mut krb5_key_data = 0 as *mut krb5_key_data;
    i = 0 as libc::c_int;
    while i < ks_tuple_count {
        let mut similar: krb5_boolean = 0;
        similar = 0 as libc::c_int as krb5_boolean;
        /*
         * We could use krb5_keysalt_iterate to replace this loop, or use
         * krb5_keysalt_is_present for the loop below, but we want to avoid
         * circular library dependencies.
         */
        j = 0 as libc::c_int;
        while j < i {
            retval =
                krb5_c_enctype_compare(context,
                                       (*ks_tuple.offset(i as
                                                             isize)).ks_enctype,
                                       (*ks_tuple.offset(j as
                                                             isize)).ks_enctype,
                                       &mut similar);
            if retval != 0 { return retval }
            if similar != 0 &&
                   (*ks_tuple.offset(j as isize)).ks_salttype ==
                       (*ks_tuple.offset(i as isize)).ks_salttype {
                break ;
            }
            j += 1
        }
        if !(j < i) {
            retval = krb5_dbe_create_key_data(context, db_entry);
            if retval != 0 { return retval }
            kd_slot =
                &mut *(*db_entry).key_data.offset(((*db_entry).n_key_data as
                                                       libc::c_int -
                                                       1 as libc::c_int) as
                                                      isize) as
                    *mut krb5_key_data;
            /* Convert password string to key using appropriate salt */
            key_salt.type_0 =
                (*ks_tuple.offset(i as isize)).ks_salttype as krb5_int16;
            match key_salt.type_0 as libc::c_int {
                3 => {
                    let mut saltdata: *mut krb5_data = 0 as *mut krb5_data;
                    retval =
                        krb5_copy_data(context,
                                       &mut (*(*db_entry).princ).realm,
                                       &mut saltdata);
                    if retval != 0 { return retval }
                    key_salt.data = *saltdata;
                    free(saltdata as *mut libc::c_void);
                }
                2 => {
                    retval =
                        krb5_principal2salt_norealm(context,
                                                    (*db_entry).princ as
                                                        krb5_const_principal,
                                                    &mut key_salt.data);
                    if retval != 0 { return retval }
                }
                0 => {
                    retval =
                        krb5_principal2salt(context,
                                            (*db_entry).princ as
                                                krb5_const_principal,
                                            &mut key_salt.data);
                    if retval != 0 { return retval }
                }
                4 => {
                    retval = make_random_salt(context, &mut key_salt);
                    if retval != 0 { return retval }
                }
                _ => {
                    return -(1780008421 as libc::c_long) as krb5_error_code
                }
            }
            pwd = string2data(passwd as *mut libc::c_char);
            retval =
                krb5_c_string_to_key_with_params(context,
                                                 (*ks_tuple.offset(i as
                                                                       isize)).ks_enctype,
                                                 &mut pwd, &mut key_salt.data,
                                                 0 as *const krb5_data,
                                                 &mut key);
            if retval != 0 {
                free(key_salt.data.data as *mut libc::c_void);
                return retval
            }
            retval =
                krb5_dbe_encrypt_key_data(context, master_key, &mut key,
                                          &mut key_salt as *mut krb5_keysalt
                                              as *const krb5_keysalt, kvno,
                                          kd_slot);
            if !key_salt.data.data.is_null() {
                free(key_salt.data.data as *mut libc::c_void);
            }
            free(key.contents as *mut libc::c_void);
            if retval != 0 { return retval }
        }
        i += 1
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "351:1"]
unsafe extern "C" fn rekey(mut context: krb5_context,
                           mut mkey: *mut krb5_keyblock,
                           mut ks_tuple: *mut krb5_key_salt_tuple,
                           mut ks_tuple_count: libc::c_int,
                           mut password: *const libc::c_char,
                           mut new_kvno: libc::c_int, mut savekeys: save,
                           mut db_entry: *mut krb5_db_entry)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut key_data: *mut krb5_key_data = 0 as *mut krb5_key_data;
    let mut n_key_data: libc::c_int = 0;
    let mut old_kvno: libc::c_int = 0;
    let mut save_kvno: libc::c_int = 0;
    /* Save aside the old key data. */
    n_key_data = (*db_entry).n_key_data as libc::c_int;
    key_data = (*db_entry).key_data;
    (*db_entry).n_key_data = 0 as libc::c_int as krb5_int16;
    (*db_entry).key_data = 0 as *mut krb5_key_data;
    /* Make sure the new kvno is greater than the old largest kvno. */
    old_kvno = krb5_db_get_key_data_kvno(context, n_key_data, key_data);
    if new_kvno < old_kvno + 1 as libc::c_int {
        new_kvno = old_kvno + 1 as libc::c_int
    }
    /* Wrap from 65535 to 1; we can only store 16-bit kvno values in key_data,
     * and we assign special meaning to kvno 0. */
    if new_kvno == (1 as libc::c_int) << 16 as libc::c_int {
        new_kvno = 1 as libc::c_int
    }
    /* Add new keys to the front of the list. */
    if !password.is_null() {
        ret =
            add_key_pwd(context, mkey, ks_tuple, ks_tuple_count, password,
                        db_entry, new_kvno)
    } else {
        ret =
            add_key_rnd(context, mkey, ks_tuple, ks_tuple_count, db_entry,
                        new_kvno)
    }
    if ret != 0 {
        cleanup_key_data(context, (*db_entry).n_key_data as libc::c_int,
                         (*db_entry).key_data);
        (*db_entry).n_key_data = n_key_data as krb5_int16;
        (*db_entry).key_data = key_data;
        return ret
    }
    /* Possibly add some or all of the old keys to the back of the list.  May
     * steal from and zero out some of the old key data entries. */
    if savekeys as libc::c_uint != DISCARD_ALL as libc::c_int as libc::c_uint
       {
        save_kvno =
            if savekeys as libc::c_uint ==
                   KEEP_LAST_KVNO as libc::c_int as libc::c_uint {
                old_kvno
            } else { 0 as libc::c_int };
        ret =
            preserve_old_keys(context, mkey, db_entry, save_kvno, n_key_data,
                              key_data)
    }
    /* Free any old key data entries not stolen and zeroed out above. */
    cleanup_key_data(context, n_key_data, key_data);
    return ret;
}
/*
 * Change random key for a krb5_db_entry
 * Assumes the max kvno
 *
 * As a side effect all old keys are nuked if keepold is false.
 */
#[no_mangle]
#[c2rust::src_loc = "409:1"]
pub unsafe extern "C" fn krb5_dbe_crk(mut context: krb5_context,
                                      mut mkey: *mut krb5_keyblock,
                                      mut ks_tuple: *mut krb5_key_salt_tuple,
                                      mut ks_tuple_count: libc::c_int,
                                      mut keepold: krb5_boolean,
                                      mut dbent: *mut krb5_db_entry)
 -> krb5_error_code {
    return rekey(context, mkey, ks_tuple, ks_tuple_count,
                 0 as *const libc::c_char, 0 as libc::c_int,
                 if keepold != 0 {
                     KEEP_ALL as libc::c_int
                 } else { DISCARD_ALL as libc::c_int } as save, dbent);
}
/*
 * Add random key for a krb5_db_entry
 * Assumes the max kvno
 *
 * As a side effect all old keys older than the max kvno are nuked.
 */
#[no_mangle]
#[c2rust::src_loc = "424:1"]
pub unsafe extern "C" fn krb5_dbe_ark(mut context: krb5_context,
                                      mut mkey: *mut krb5_keyblock,
                                      mut ks_tuple: *mut krb5_key_salt_tuple,
                                      mut ks_tuple_count: libc::c_int,
                                      mut dbent: *mut krb5_db_entry)
 -> krb5_error_code {
    return rekey(context, mkey, ks_tuple, ks_tuple_count,
                 0 as *const libc::c_char, 0 as libc::c_int, KEEP_LAST_KVNO,
                 dbent);
}
/*
 * Change password for a krb5_db_entry
 * Assumes the max kvno
 *
 * As a side effect all old keys are nuked if keepold is false.
 */
#[no_mangle]
#[c2rust::src_loc = "439:1"]
pub unsafe extern "C" fn krb5_dbe_def_cpw(mut context: krb5_context,
                                          mut mkey: *mut krb5_keyblock,
                                          mut ks_tuple:
                                              *mut krb5_key_salt_tuple,
                                          mut ks_tuple_count: libc::c_int,
                                          mut password: *mut libc::c_char,
                                          mut new_kvno: libc::c_int,
                                          mut keepold: krb5_boolean,
                                          mut dbent: *mut krb5_db_entry)
 -> krb5_error_code {
    return rekey(context, mkey, ks_tuple, ks_tuple_count, password, new_kvno,
                 if keepold != 0 {
                     KEEP_ALL as libc::c_int
                 } else { DISCARD_ALL as libc::c_int } as save, dbent);
}
/*
 * Add password for a krb5_db_entry
 * Assumes the max kvno
 *
 * As a side effect all old keys older than the max kvno are nuked.
 */
#[no_mangle]
#[c2rust::src_loc = "455:1"]
pub unsafe extern "C" fn krb5_dbe_apw(mut context: krb5_context,
                                      mut mkey: *mut krb5_keyblock,
                                      mut ks_tuple: *mut krb5_key_salt_tuple,
                                      mut ks_tuple_count: libc::c_int,
                                      mut password: *mut libc::c_char,
                                      mut dbent: *mut krb5_db_entry)
 -> krb5_error_code {
    return rekey(context, mkey, ks_tuple, ks_tuple_count, password,
                 0 as libc::c_int, KEEP_LAST_KVNO, dbent);
}
