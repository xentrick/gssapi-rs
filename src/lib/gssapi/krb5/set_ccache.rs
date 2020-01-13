use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:32"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:33"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:33"]
pub mod k5_thread_h {
    /* Thread-specific data; implemented in a support file, because we'll
   need to keep track of some global data for cleanup purposes.

   Note that the callback function type is such that the C library
   routine free() is a valid callback.  */
    #[c2rust::src_loc = "401:9"]
    pub type k5_key_t = libc::c_uint;
    #[c2rust::src_loc = "410:5"]
    pub const K5_KEY_MAX: k5_key_t = 5;
    #[c2rust::src_loc = "406:5"]
    pub const K5_KEY_GSS_SPNEGO_STATUS: k5_key_t = 4;
    #[c2rust::src_loc = "405:5"]
    pub const K5_KEY_GSS_KRB5_ERROR_MESSAGE: k5_key_t = 3;
    #[c2rust::src_loc = "404:5"]
    pub const K5_KEY_GSS_KRB5_CCACHE_NAME: k5_key_t = 2;
    #[c2rust::src_loc = "403:5"]
    pub const K5_KEY_GSS_KRB5_SET_CCACHE_OLD_NAME: k5_key_t = 1;
    #[c2rust::src_loc = "402:5"]
    pub const K5_KEY_COM_ERR: k5_key_t = 0;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "418:1"]
        pub fn krb5int_getspecific(_: k5_key_t) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "419:1"]
        pub fn krb5int_setspecific(_: k5_key_t, _: *mut libc::c_void)
         -> libc::c_int;
    }
    /* multiple inclusion? */
    /* In time, many of the definitions above should move into the support
   library, and this file should be greatly simplified.  For type
   definitions, that'll take some work, since other data structures
   incorporate mutexes directly, and our mutex type is dependent on
   configuration options and system attributes.  For most functions,
   though, it should be relatively easy.

   For now, plugins should use the exported functions, and not the
   above macros, and use krb5int_mutex_alloc for allocations.  */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:33"]
pub mod gssapi_h {
    /* This is the gssapi.h prologue. */
/* no xom.h */
/* End of gssapi.h prologue. */
/* -*- mode: c; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 by OpenVision Technologies, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
    /*
 * Determine platform-dependent configuration.
 */
    /* __cplusplus */
    /*
 * First, include stddef.h to get size_t defined.
 */
    /*
 * POSIX says that sys/types.h is where size_t is defined.
 */
    /*
 * $Id$
 */
    /*
 * First, define the three platform-dependent pointer types.
 */
    /*
 * The following type must be defined as the smallest natural unsigned integer
 * supported by the platform that has at least 32 bits of precision.
 */
    /* OM_STRING */
    /*
 * We can't use X/Open definitions, so roll our own.
 */
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "106:16"]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/krb5/gssapiP_krb5.h:33"]
pub mod gssapiP_krb5_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1059:8"]
    pub struct krb5_gss_ccache_name_req {
        pub name: *const libc::c_char,
        pub out_name: *mut *const libc::c_char,
    }
    use super::gssapi_h::OM_uint32;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "381:1"]
        pub fn kg_get_ccache_name(minor_status: *mut OM_uint32,
                                  out_name: *mut *const libc::c_char)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "384:1"]
        pub fn kg_set_ccache_name(minor_status: *mut OM_uint32,
                                  name: *const libc::c_char) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1171:1"]
        pub fn gss_krb5int_initialize_library() -> OM_uint32;
    }
    /* _GSSAPIP_KRB5_H_ */
}
#[c2rust::header_src = "/usr/include/stdlib.h:33"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/assert.h:33"]
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
pub use self::stddef_h::size_t;
pub use self::types_h::__uint32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::k5_thread_h::{k5_key_t, K5_KEY_MAX, K5_KEY_GSS_SPNEGO_STATUS,
                            K5_KEY_GSS_KRB5_ERROR_MESSAGE,
                            K5_KEY_GSS_KRB5_CCACHE_NAME,
                            K5_KEY_GSS_KRB5_SET_CCACHE_OLD_NAME,
                            K5_KEY_COM_ERR, krb5int_getspecific,
                            krb5int_setspecific};
pub use self::gssapi_h::{OM_uint32, gss_uint32, gss_OID_desc_struct, gss_OID,
                         gss_buffer_desc_struct, gss_buffer_t};
pub use self::gssapiP_krb5_h::{krb5_gss_ccache_name_req, kg_get_ccache_name,
                               kg_set_ccache_name,
                               gss_krb5int_initialize_library};
use self::stdlib_h::free;
use self::assert_h::__assert_fail;
/* -*- mode: c; indent-tabs-mode: nil -*- */
/*
 * Copyright 2000, 2008 by the Massachusetts Institute of Technology.
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
 *
 */
/*
 * Copyright 1993 by OpenVision Technologies, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
/* work around sunos braindamage */
/* The include of gssapi_krb5.h will dtrt with the above #defines in
 * effect.
 */
/* for debugging */
/* * constants **/
/* Incorrect krb5 mech OID emitted by MS. */
/* IAKERB variant */
/* * CFX flags **/
/* These are to be stored in little-endian order, i.e., des-mac is
   stored as 02 00.  */
/* SGN_ALG_DES_MAC_MD5           = 0x0000, */
    /* SGN_ALG_MD2_5                 = 0x0001, */
    /* SGN_ALG_DES_MAC               = 0x0002, */
    /* SGN_ALG_3                     = 0x0003, /\* not published *\/ */
/* microsoft w2k;  */
/* SEAL_ALG_DES             = 0x0000, */
    /* SEAL_ALG_1               = 0x0001, /\* not published *\/ */
/* microsoft w2k;  */
/* for 3DES */
/* for draft-ietf-krb-wg-gssapi-cfx-01 */
/* GSS_KRB5_INTEG_C_QOP_MD5       = 0x0001, */
    /* GSS_KRB5_INTEG_C_QOP_DES_MD5   = 0x0002, */
    /* GSS_KRB5_INTEG_C_QOP_DES_MAC   = 0x0003, */
/* GSS_KRB5_CONF_C_QOP_DES        = 0x0100, */
/* * internal types **/
/* immutable */
/* immutable */
/* immutable */
/* protects ad_context only for now */
/* protect against simultaneous accesses */
/* name/type of credential */
/* keytab (accept) data */
/* ccache (init) data */
/* limit negotiated enctypes to this list */
/* nonzero if initiating, zero if accepting */
/* XXX tested but never actually set */
/* One of two potential keys to use with RFC 4121
                      * packets; this key must always be set. */
/* RFC 1964 encryption key; seq xored with a constant
                   * for DES, seq for other RFC 1964 enctypes  */
/* RFC 1964 sequencing key */
/* XXX these used to be signed.  the old spec is inspecific, and
       the new spec specifies unsigned.  I don't believe that the change
       affects the wire encoding. */
/* Protocol spec revision for sending packets
       0 => RFC 1964 with 3DES and RC4 enhancements
       1 => RFC 4121
       No others defined so far.  It is always permitted to receive
       tokens in RFC 4121 format.  If enc is non-null, receiving RFC
       1964 tokens is permitted.*/
/* for "main" subkey */
/* CFX only */
/* did we get rcache from creds? */
/* LEAN_CLIENT */
/* * helper functions **/
/* Encrypt length bytes at ptr in place, with the given key and usage.  If
 * iv is not NULL, use it as the cipher state. */
/* AEAD */
/* for conf len */
/* * declarations of internal name mechanism functions **/
/* minor_status */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* minor_status */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* minor_status */
/* cred_handle */
/* minor_status */
/* claimant_cred_handle */
/* context_handle */
/* target_name */
/* mech_type */
/* req_flags */
/* time_req */
/* input_chan_bindings */
/* input_token */
/* actual_mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* minor_status */
/* claimant_cred_handle */
/* context_handle */
/* target_name */
/* mech_type */
/* req_flags */
/* time_req */
/* input_chan_bindings */
/* input_token */
/* actual_mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* exts */
/* minor_status */
/* context_handle */
/* verifier_cred_handle */
/* input_token_buffer */
/* input_chan_bindings */
/* src_name */
/* mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* delegated_cred_handle */
/* minor_status */
/* context_handle */
/* verifier_cred_handle */
/* input_token_buffer */
/* input_chan_bindings */
/* src_name */
/* mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* delegated_cred_handle */
/*exts */
/* LEAN_CLIENT */
/* minor_status */
/* context_handle */
/* desired_object */
/* data_set */
/* minor_status */
/* context_handle */
/* desired_object */
/* value */
/* minor_status */
/* context_handle */
/* token_buffer */
/* minor_status */
/* context_handle */
/* output_token */
/* minor_status */
/* context_handle */
/* time_rec */
/* minor_status */
/* status_value */
/* status_type */
/* mech_type */
/* message_context */
/* status_string */
/* minor_status */
/* mech_set */
/* minor_status */
/* name1 */
/* name2 */
/* name_equal */
/* minor_status */
/* input_name */
/* output_name_buffer */
/* output_name_type */
/* minor_status */
/* input_name_buffer */
/* input_name_type */
/* output_name */
/* minor_status */
/* input_name */
/* minor_status */
/* cred_handle */
/* name */
/* lifetime */
/* cred_usage */
/* mechanisms */
/* minor_status */
/* context_handle */
/* initiator_name */
/* acceptor_name */
/* lifetime_rec */
/* mech_type */
/* ret_flags */
/* locally_initiated */
/* open */
/* New V2 entry points */
/* minor_status */
/* context_handle */
/* qop_req */
/* message_buffer */
/* message_token */
/* minor_status */
/* context_handle */
/* qop_req */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* qop_req */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* message_buffer */
/* message_token */
/* qop_state */
/* minor_status */
/* context_handle */
/* qop_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* input_message_buffer */
/* conf_state */
/* output_message_buffer */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* conf_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* conf_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* input_message_buffer */
/* output_message_buffer */
/* conf_state */
/* qop_state */
/* minor_status */
/* context_handle */
/* conf_state */
/* qop_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* req_output_size */
/* max_input_size */
/* minor_status */
/* input_name */
/* input_name_type */
/* output_name */
/* minor_status */
/* input_name */
/* desired_name_type */
/* output_name */
/* minor_status */
/* cred_handle */
/* mech_type */
/* name */
/* initiator_lifetime */
/* acceptor_lifetime */
/* cred_usage */
/* minor_status */
/* context_handle */
/* interprocess_token */
/* minor_status */
/* interprocess_token */
/* context_handle */
/* LEAN_CLIENT */
/* minor_status */
/* oid */
/* minor_status */
/* oid */
/* minor_status */
/* mechanism */
/* name_types */
/* minor_status */
/* input_name */
/* mech_type */
/* output_name */
/* minor_status */
/* input_name */
/* exported_name */
/* minor_status */
/* input_name */
/* dest_name */
/* minor_status */
/* cred */
/* minor_status */
/* impersonator_cred_handle */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* minor_status */
/* cred_handle */
/* context */
/* naming_exts.c */
/* s4u_gss_glue.c */
/*
 * These take unglued krb5-mech-specific contexts.
 */
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/gssapi/krb5/set_ccache.c */
/*
 * Copyright 1999, 2003 by the Massachusetts Institute of Technology.
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
 * Set ccache name used by gssapi, and optionally obtain old ccache
 * name.  Caller should not free returned name.
 */
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn gss_krb5int_ccache_name(mut minor_status:
                                                     *mut OM_uint32,
                                                 desired_mech: gss_OID,
                                                 desired_object: gss_OID,
                                                 value: gss_buffer_t)
 -> OM_uint32 {
    let mut old_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut err: OM_uint32 = 0 as libc::c_int as OM_uint32;
    let mut minor: OM_uint32 = 0 as libc::c_int as OM_uint32;
    let mut gss_out_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut req: *mut krb5_gss_ccache_name_req =
        0 as *mut krb5_gss_ccache_name_req;
    err = gss_krb5int_initialize_library();
    if err != 0 {
        *minor_status = err;
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if (*value).length ==
           ::std::mem::size_of::<krb5_gss_ccache_name_req>() as libc::c_ulong
       {
    } else {
        __assert_fail(b"value->length == sizeof(*req)\x00" as *const u8 as
                          *const libc::c_char,
                      b"set_ccache.c\x00" as *const u8 as *const libc::c_char,
                      53 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 97],
                                                &[libc::c_char; 97]>(b"OM_uint32 gss_krb5int_ccache_name(OM_uint32 *, const gss_OID, const gss_OID, const gss_buffer_t)\x00")).as_ptr());
    }
    if (*value).length !=
           ::std::mem::size_of::<krb5_gss_ccache_name_req>() as libc::c_ulong
       {
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    req = (*value).value as *mut krb5_gss_ccache_name_req;
    gss_out_name =
        krb5int_getspecific(K5_KEY_GSS_KRB5_SET_CCACHE_OLD_NAME) as
            *mut libc::c_char;
    if !(*req).out_name.is_null() {
        let mut tmp_name: *const libc::c_char = 0 as *const libc::c_char;
        if err == 0 { kg_get_ccache_name(&mut err, &mut tmp_name); }
        if err == 0 {
            old_name = gss_out_name;
            gss_out_name = tmp_name as *mut libc::c_char
        }
    }
    /* If out_name was NULL, we keep the same gss_out_name value, and
       don't free up any storage (leave old_name NULL).  */
    if err == 0 { kg_set_ccache_name(&mut err, (*req).name); }
    minor =
        krb5int_setspecific(K5_KEY_GSS_KRB5_SET_CCACHE_OLD_NAME,
                            gss_out_name as *mut libc::c_void) as OM_uint32;
    if minor != 0 {
        /* Um.  Now what?  */
        if err == 0 as libc::c_int as libc::c_uint { err = minor }
        free(gss_out_name as *mut libc::c_void);
        gss_out_name = 0 as *mut libc::c_char
    }
    if err == 0 {
        if !(*req).out_name.is_null() { *(*req).out_name = gss_out_name }
    }
    if !old_name.is_null() { free(old_name as *mut libc::c_void); }
    *minor_status = err;
    return if *minor_status == 0 as libc::c_int as libc::c_uint {
               0 as libc::c_int as libc::c_uint
           } else {
               ((13 as libc::c_ulong as OM_uint32)) << 16 as libc::c_int
           };
}
