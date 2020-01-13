use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:33"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:33"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:33"]
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
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "176:1"]
    pub type krb5_kvno = libc::c_uint;
    #[c2rust::src_loc = "178:1"]
    pub type krb5_addrtype = krb5_int32;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "180:1"]
    pub type krb5_cksumtype = krb5_int32;
    #[c2rust::src_loc = "181:1"]
    pub type krb5_authdatatype = krb5_int32;
    #[c2rust::src_loc = "182:1"]
    pub type krb5_keyusage = krb5_int32;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "323:16"]
    pub struct _krb5_address {
        pub magic: krb5_magic,
        pub addrtype: krb5_addrtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* *< Anonymous realm */
    /* *< Anonymous principal name */
    /*
 * end "base-defs.h"
 */
    /*
 * begin "hostaddr.h"
 */
    /* * Structure for address */
    #[c2rust::src_loc = "323:1"]
    pub type krb5_address = _krb5_address;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "391:16"]
    pub struct _krb5_checksum {
        pub magic: krb5_magic,
        pub checksum_type: krb5_cksumtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "391:1"]
    pub type krb5_checksum = _krb5_checksum;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1936:16"]
    pub struct _krb5_ticket_times {
        pub authtime: krb5_timestamp,
        pub starttime: krb5_timestamp,
        pub endtime: krb5_timestamp,
        pub renew_till: krb5_timestamp,
    }
    /* checksum type */
    /* *< RFC 4537 */
    /* *< formerly 142 in krb5 1.8 */
    /* * @} */
    /* end of KRB5_AUTHDATA group */
    /* password change constants */
    /* *< Success */
    /* *< Malformed request */
    /* *< Server error */
    /* *< Authentication error */
    /* *< Password change rejected */
    /* These are Microsoft's extensions in RFC 3244, and it looks like
   they'll become standardized, possibly with other additions.  */
    /* *< Not authorized */
    /* *< Unknown RPC version */
    /* * The presented credentials were not obtained using a password directly */
    /*
 * end "proto.h"
 */
    /* Time set */
/* * Ticket start time, end time, and renewal duration. */
    #[c2rust::src_loc = "1936:1"]
    pub type krb5_ticket_times = _krb5_ticket_times;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1946:16"]
    pub struct _krb5_authdata {
        pub magic: krb5_magic,
        pub ad_type: krb5_authdatatype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* *< Time at which KDC issued the initial ticket that corresponds to this ticket */
    /* XXX ? should ktime in KDC_REP == authtime
       in ticket? otherwise client can't get this */
    /* *< optional in ticket, if not present, use @a authtime */
    /* *< Ticket expiration time */
    /* *< Latest time at which renewal of ticket can be valid */
    /* * Structure for auth data */
    #[c2rust::src_loc = "1946:1"]
    pub type krb5_authdata = _krb5_authdata;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1954:16"]
    pub struct _krb5_transited {
        pub magic: krb5_magic,
        pub tr_type: krb5_octet,
        pub tr_contents: krb5_data,
    }
    /* *< ADTYPE */
    /* *< Length of data  */
    /* *< Data */
    /* * Structure for transited encoding */
    #[c2rust::src_loc = "1954:1"]
    pub type krb5_transited = _krb5_transited;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1961:16"]
    pub struct _krb5_enc_tkt_part {
        pub magic: krb5_magic,
        pub flags: krb5_flags,
        pub session: *mut krb5_keyblock,
        pub client: krb5_principal,
        pub transited: krb5_transited,
        pub times: krb5_ticket_times,
        pub caddrs: *mut *mut krb5_address,
        pub authorization_data: *mut *mut krb5_authdata,
    }
    /* *< Transited encoding type */
    /* *< Contents */
    /* * Encrypted part of ticket. */
    #[c2rust::src_loc = "1961:1"]
    pub type krb5_enc_tkt_part = _krb5_enc_tkt_part;
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        /* to-be-encrypted portion */
        /* *< flags */
        /* *< session key: includes enctype */
        /* *< client name/realm */
        /* *< list of transited realms */
        /* *< auth, start, end, renew_till */
        /* *< array of ptrs to addresses */
        /* *< auth data */
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
 * Compute a checksum (operates on keyblock).
 *
 * @param [in]  context         Library context
 * @param [in]  cksumtype       Checksum type (0 for mandatory type)
 * @param [in]  key             Encryption key for a keyed checksum
 * @param [in]  usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]  input           Input data
 * @param [out] cksum           Generated checksum
 *
 * This function computes a checksum of type @a cksumtype over @a input, using
 * @a key if the checksum type is a keyed checksum.  If @a cksumtype is 0 and
 * @a key is non-null, the checksum type will be the mandatory-to-implement
 * checksum type for the key's encryption type.  The actual checksum key will
 * be derived from @a key and @a usage if key derivation is specified for the
 * checksum type.  The newly created @a cksum must be released by calling
 * krb5_free_checksum_contents() when it is no longer needed.
 *
 * @note This function is similar to krb5_k_make_checksum(), but operates
 * on keyblock @a key.
 *
 * @sa krb5_c_verify_checksum()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "911:1"]
        pub fn krb5_c_make_checksum(context: krb5_context,
                                    cksumtype: krb5_cksumtype,
                                    key: *const krb5_keyblock,
                                    usage: krb5_keyusage,
                                    input: *const krb5_data,
                                    cksum: *mut krb5_checksum)
         -> krb5_error_code;
        /* *
 * Verify a checksum (operates on keyblock).
 *
 * @param [in]  context         Library context
 * @param [in]  key             Encryption key for a keyed checksum
 * @param [in]  usage           @a key usage
 * @param [in]  data            Data to be used to compute a new checksum
 *                              using @a key to compare @a cksum against
 * @param [in]  cksum           Checksum to be verified
 * @param [out] valid           Non-zero for success, zero for failure
 *
 * This function verifies that @a cksum is a valid checksum for @a data.  If
 * the checksum type of @a cksum is a keyed checksum, @a key is used to verify
 * the checksum.  If the checksum type in @a cksum is 0 and @a key is not NULL,
 * the mandatory checksum type for @a key will be used.  The actual checksum
 * key will be derived from @a key and @a usage if key derivation is specified
 * for the checksum type.
 *
 * @note This function is similar to krb5_k_verify_checksum(), but operates
 * on keyblock @a key.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "939:1"]
        pub fn krb5_c_verify_checksum(context: krb5_context,
                                      key: *const krb5_keyblock,
                                      usage: krb5_keyusage,
                                      data: *const krb5_data,
                                      cksum: *const krb5_checksum,
                                      valid: *mut krb5_boolean)
         -> krb5_error_code;
        /* *
 * Free the contents of a krb5_checksum structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Checksum structure to free contents of
 *
 * This function frees the contents of @a val, but not the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4699:1"]
        pub fn krb5_free_checksum_contents(context: krb5_context,
                                           val: *mut krb5_checksum);
        /* *
 * Free the contents of a krb5_keyblock structure.
 *
 * @param [in] context          Library context
 * @param [in] key              Keyblock to be freed
 *
 * This function frees the contents of @a key, but not the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4721:1"]
        pub fn krb5_free_keyblock_contents(context: krb5_context,
                                           key: *mut krb5_keyblock);
        /* *
 * Free a krb5_data structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Data structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4743:1"]
        pub fn krb5_free_data(context: krb5_context, val: *mut krb5_data);
        #[no_mangle]
        #[c2rust::src_loc = "8077:1"]
        pub fn krb5_encode_authdata_container(context: krb5_context,
                                              type_0: krb5_authdatatype,
                                              authdata:
                                                  *const *mut krb5_authdata,
                                              container:
                                                  *mut *mut *mut krb5_authdata)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:33"]
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
    #[c2rust::src_loc = "843:1"]
    pub type krb5_verifier_mac = _krb5_verifier_mac;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "843:16"]
    pub struct _krb5_verifier_mac {
        pub princ: krb5_principal,
        pub kvno: krb5_kvno,
        pub enctype: krb5_enctype,
        pub checksum: krb5_checksum,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "856:16"]
    pub struct _krb5_cammac {
        pub elements: *mut *mut krb5_authdata,
        pub kdc_verifier: *mut krb5_verifier_mac,
        pub svc_verifier: *mut krb5_verifier_mac,
        pub other_verifiers: *mut *mut krb5_verifier_mac,
    }
    #[c2rust::src_loc = "856:1"]
    pub type krb5_cammac = _krb5_cammac;
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_principal, krb5_kvno, krb5_checksum,
                        krb5_authdata, krb5_enc_tkt_part, krb5_data,
                        krb5_error_code};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
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
        #[c2rust::src_loc = "1381:1"]
        pub fn encode_krb5_enc_tkt_part(rep: *const krb5_enc_tkt_part,
                                        code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1442:1"]
        pub fn encode_krb5_authdata(rep: *const *mut krb5_authdata,
                                    code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1538:1"]
        pub fn encode_krb5_cammac(_: *const krb5_cammac,
                                  _: *mut *mut krb5_data) -> krb5_error_code;
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:33"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:33"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:33"]
pub mod profile_h {
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:34"]
pub mod kdb_h {
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
    #[c2rust::src_loc = "167:16"]
    pub struct _krb5_key_data {
        pub key_data_ver: krb5_int16,
        pub key_data_kvno: krb5_ui_2,
        pub key_data_type: [krb5_int16; 2],
        pub key_data_length: [krb5_ui_2; 2],
        pub key_data_contents: [*mut krb5_octet; 2],
    }
    #[c2rust::src_loc = "167:1"]
    pub type krb5_key_data = _krb5_key_data;
    #[c2rust::src_loc = "191:1"]
    pub type krb5_db_entry = _krb5_db_entry_new;
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
    #[c2rust::src_loc = "147:1"]
    pub type krb5_tl_data = _krb5_tl_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "147:16"]
    pub struct _krb5_tl_data {
        pub tl_data_next: *mut _krb5_tl_data,
        pub tl_data_type: krb5_int16,
        pub tl_data_length: krb5_ui_2,
        pub tl_data_contents: *mut krb5_octet,
    }
    use super::krb5_h::{krb5_int16, krb5_data, krb5_ui_2, krb5_octet,
                        krb5_magic, krb5_ui_4, krb5_flags, krb5_deltat,
                        krb5_timestamp, krb5_kvno, krb5_principal,
                        krb5_context, krb5_int32, krb5_error_code,
                        krb5_keyblock};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "418:1"]
        pub fn krb5_dbe_find_enctype(kcontext: krb5_context,
                                     dbentp: *mut krb5_db_entry,
                                     ktype: krb5_int32, stype: krb5_int32,
                                     kvno: krb5_int32,
                                     kdatap: *mut *mut krb5_key_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "446:1"]
        pub fn krb5_dbe_decrypt_key_data(context: krb5_context,
                                         mkey: *const krb5_keyblock,
                                         key_data: *const krb5_key_data,
                                         dbkey: *mut krb5_keyblock,
                                         keysalt: *mut krb5_keysalt)
         -> krb5_error_code;
    }
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src = "/usr/include/string.h:33"]
pub mod string_h {
    extern "C" {
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
                       krb5_ui_4, krb5_boolean, krb5_kvno, krb5_addrtype,
                       krb5_enctype, krb5_cksumtype, krb5_authdatatype,
                       krb5_keyusage, krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal, _krb5_address,
                       krb5_address, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _krb5_keyblock,
                       krb5_keyblock, _krb5_checksum, krb5_checksum,
                       _krb5_ticket_times, krb5_ticket_times, _krb5_authdata,
                       krb5_authdata, _krb5_transited, krb5_transited,
                       _krb5_enc_tkt_part, krb5_enc_tkt_part, _profile_t,
                       krb5_c_make_checksum, krb5_c_verify_checksum,
                       krb5_free_checksum_contents,
                       krb5_free_keyblock_contents, krb5_free_data,
                       krb5_encode_authdata_container};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, krb5_verifier_mac,
                         _krb5_verifier_mac, _krb5_cammac, krb5_cammac,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, encode_krb5_enc_tkt_part,
                         encode_krb5_authdata, encode_krb5_cammac};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::kdb_h::{krb5_keysalt, _krb5_keysalt, _krb5_key_data,
                      krb5_key_data, krb5_db_entry, _krb5_db_entry_new,
                      krb5_tl_data, _krb5_tl_data, krb5_dbe_find_enctype,
                      krb5_dbe_decrypt_key_data};
use self::string_h::memset;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kdc/cammac.c - Functions for wrapping and unwrapping CAMMACs */
/*
 * Copyright (C) 2015 by the Massachusetts Institute of Technology.
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
/* Encode enc_tkt with contents as the authdata field, for use in KDC
 * verifier checksums. */
#[c2rust::src_loc = "38:1"]
unsafe extern "C" fn encode_kdcver_encpart(mut enc_tkt:
                                               *mut krb5_enc_tkt_part,
                                           mut contents:
                                               *mut *mut krb5_authdata,
                                           mut der_out: *mut *mut krb5_data)
 -> krb5_error_code {
    let mut ck_enctkt: krb5_enc_tkt_part =
        krb5_enc_tkt_part{magic: 0,
                          flags: 0,
                          session: 0 as *mut krb5_keyblock,
                          client: 0 as *mut krb5_principal_data,
                          transited:
                              krb5_transited{magic: 0,
                                             tr_type: 0,
                                             tr_contents:
                                                 krb5_data{magic: 0,
                                                           length: 0,
                                                           data:
                                                               0 as
                                                                   *mut libc::c_char,},},
                          times:
                              krb5_ticket_times{authtime: 0,
                                                starttime: 0,
                                                endtime: 0,
                                                renew_till: 0,},
                          caddrs: 0 as *mut *mut krb5_address,
                          authorization_data: 0 as *mut *mut krb5_authdata,};
    ck_enctkt = *enc_tkt;
    ck_enctkt.authorization_data = contents;
    return encode_krb5_enc_tkt_part(&mut ck_enctkt, der_out);
}
/*
 * Create a CAMMAC for contents, using enc_tkt and tgt_key for the KDC
 * verifier.  tgt_key must be the decrypted first key data entry in tgt.  Set
 * *cammac_out to a single-element authdata list containing the CAMMAC inside
 * an IF-RELEVANT container.
 */
#[no_mangle]
#[c2rust::src_loc = "55:1"]
pub unsafe extern "C" fn cammac_create(mut context: krb5_context,
                                       mut enc_tkt: *mut krb5_enc_tkt_part,
                                       mut server_key: *mut krb5_keyblock,
                                       mut tgt: *mut krb5_db_entry,
                                       mut tgt_key: *mut krb5_keyblock,
                                       mut contents: *mut *mut krb5_authdata,
                                       mut cammac_out:
                                           *mut *mut *mut krb5_authdata)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut der_authdata: *mut krb5_data = 0 as *mut krb5_data;
    let mut der_enctkt: *mut krb5_data = 0 as *mut krb5_data;
    let mut der_cammac: *mut krb5_data = 0 as *mut krb5_data;
    let mut ad: krb5_authdata =
        krb5_authdata{magic: 0,
                      ad_type: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut list: [*mut krb5_authdata; 2] = [0 as *mut krb5_authdata; 2];
    let mut cammac: krb5_cammac =
        krb5_cammac{elements: 0 as *mut *mut krb5_authdata,
                    kdc_verifier: 0 as *mut krb5_verifier_mac,
                    svc_verifier: 0 as *mut krb5_verifier_mac,
                    other_verifiers: 0 as *mut *mut krb5_verifier_mac,};
    let mut kdc_verifier: krb5_verifier_mac =
        krb5_verifier_mac{princ: 0 as *mut krb5_principal_data,
                          kvno: 0,
                          enctype: 0,
                          checksum:
                              krb5_checksum{magic: 0,
                                            checksum_type: 0,
                                            length: 0,
                                            contents:
                                                0 as *mut krb5_octet,},};
    let mut svc_verifier: krb5_verifier_mac =
        krb5_verifier_mac{princ: 0 as *mut krb5_principal_data,
                          kvno: 0,
                          enctype: 0,
                          checksum:
                              krb5_checksum{magic: 0,
                                            checksum_type: 0,
                                            length: 0,
                                            contents:
                                                0 as *mut krb5_octet,},};
    let mut tgtkey: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut kdc_cksum: krb5_checksum =
        krb5_checksum{magic: 0,
                      checksum_type: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut svc_cksum: krb5_checksum =
        krb5_checksum{magic: 0,
                      checksum_type: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    *cammac_out = 0 as *mut *mut krb5_authdata;
    memset(&mut tgtkey as *mut krb5_keyblock as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong);
    memset(&mut kdc_cksum as *mut krb5_checksum as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_checksum>() as libc::c_ulong);
    memset(&mut svc_cksum as *mut krb5_checksum as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_checksum>() as libc::c_ulong);
    /* Checksum the reply with contents as authdata for the KDC verifier. */
    ret = encode_kdcver_encpart(enc_tkt, contents, &mut der_enctkt);
    if !(ret != 0) {
        ret =
            krb5_c_make_checksum(context, 0 as libc::c_int, tgt_key,
                                 64 as libc::c_int, der_enctkt,
                                 &mut kdc_cksum);
        if !(ret != 0) {
            kdc_verifier.princ = 0 as krb5_principal;
            kdc_verifier.kvno =
                (*(*tgt).key_data.offset(0 as libc::c_int as
                                             isize)).key_data_kvno as
                    krb5_kvno;
            kdc_verifier.enctype = 0 as libc::c_int;
            kdc_verifier.checksum = kdc_cksum;
            /* Encode the authdata and checksum it for the service verifier. */
            ret = encode_krb5_authdata(contents, &mut der_authdata);
            if !(ret != 0) {
                ret =
                    krb5_c_make_checksum(context, 0 as libc::c_int,
                                         server_key, 64 as libc::c_int,
                                         der_authdata, &mut svc_cksum);
                if !(ret != 0) {
                    svc_verifier.princ = 0 as krb5_principal;
                    svc_verifier.kvno = 0 as libc::c_int as krb5_kvno;
                    svc_verifier.enctype = 0 as libc::c_int;
                    svc_verifier.checksum = svc_cksum;
                    cammac.elements = contents;
                    cammac.kdc_verifier = &mut kdc_verifier;
                    cammac.svc_verifier = &mut svc_verifier;
                    cammac.other_verifiers = 0 as *mut *mut krb5_verifier_mac;
                    ret = encode_krb5_cammac(&mut cammac, &mut der_cammac);
                    if !(ret != 0) {
                        /* Wrap the encoded CAMMAC in an IF-RELEVANT container and return it as a
     * single-element authdata list. */
                        ad.ad_type = 96 as libc::c_int;
                        ad.length = (*der_cammac).length;
                        ad.contents = (*der_cammac).data as *mut uint8_t;
                        list[0 as libc::c_int as usize] = &mut ad;
                        list[1 as libc::c_int as usize] =
                            0 as *mut krb5_authdata;
                        ret =
                            krb5_encode_authdata_container(context,
                                                           1 as libc::c_int,
                                                           list.as_mut_ptr(),
                                                           cammac_out)
                    }
                }
            }
        }
    }
    krb5_free_data(context, der_enctkt);
    krb5_free_data(context, der_authdata);
    krb5_free_data(context, der_cammac);
    krb5_free_keyblock_contents(context, &mut tgtkey);
    krb5_free_checksum_contents(context, &mut kdc_cksum);
    krb5_free_checksum_contents(context, &mut svc_cksum);
    return ret;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kdc/kdc_util.h */
/*
 * Portions Copyright (C) 2007 Apple Inc.
 * Copyright 1990, 2007, 2014 by the Massachusetts Institute of Technology.
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
 *
 *
 * Declarations for policy.c
 */
/* authind.c */
/* cammac.c */
/*
 * Return true if cammac's KDC verifier is valid for enc_tkt, using tgt to
 * retrieve the TGT key indicated by the verifier.  tgt_key must be the
 * decrypted first key data entry in tgt.
 */
#[no_mangle]
#[c2rust::src_loc = "134:1"]
pub unsafe extern "C" fn cammac_check_kdcver(mut context: krb5_context,
                                             mut cammac: *mut krb5_cammac,
                                             mut enc_tkt:
                                                 *mut krb5_enc_tkt_part,
                                             mut tgt: *mut krb5_db_entry,
                                             mut tgt_key: *mut krb5_keyblock)
 -> krb5_boolean {
    let mut current_block: u64;
    let mut ver: *mut krb5_verifier_mac = (*cammac).kdc_verifier;
    let mut kd: *mut krb5_key_data = 0 as *mut krb5_key_data;
    let mut tgtkey: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut key: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut valid: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut der_enctkt: *mut krb5_data = 0 as *mut krb5_data;
    memset(&mut tgtkey as *mut krb5_keyblock as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong);
    if !ver.is_null() {
        /* Fetch the krbtgt key indicated by the KDC verifier.  Only allow the
     * first krbtgt key of the specified kvno. */
        if (*ver).kvno ==
               (*(*tgt).key_data.offset(0 as libc::c_int as
                                            isize)).key_data_kvno as
                   libc::c_uint {
            key = tgt_key;
            current_block = 7746791466490516765;
        } else if krb5_dbe_find_enctype(context, tgt, -(1 as libc::c_int),
                                        -(1 as libc::c_int),
                                        (*ver).kvno as krb5_int32, &mut kd) !=
                      0 as libc::c_int {
            current_block = 14737279513370991710;
        } else if krb5_dbe_decrypt_key_data(context,
                                            0 as *const krb5_keyblock, kd,
                                            &mut tgtkey,
                                            0 as *mut krb5_keysalt) !=
                      0 as libc::c_int {
            current_block = 14737279513370991710;
        } else { key = &mut tgtkey; current_block = 7746791466490516765; }
        match current_block {
            14737279513370991710 => { }
            _ => {
                if !((*ver).enctype != 0 as libc::c_int &&
                         tgtkey.enctype != (*ver).enctype) {
                    /* Verify the checksum over the DER-encoded enc_tkt with the CAMMAC
     * elements as authdata. */
                    if !(encode_kdcver_encpart(enc_tkt, (*cammac).elements,
                                               &mut der_enctkt) !=
                             0 as libc::c_int) {
                        krb5_c_verify_checksum(context, key,
                                               64 as libc::c_int, der_enctkt,
                                               &mut (*ver).checksum,
                                               &mut valid);
                    }
                }
            }
        }
    }
    krb5_free_keyblock_contents(context, &mut tgtkey);
    krb5_free_data(context, der_enctkt);
    return valid;
}
