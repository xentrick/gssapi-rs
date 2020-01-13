use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:27"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:27"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:27"]
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
    pub type gss_OID_desc = gss_OID_desc_struct;
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    /* OM_STRING */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:16"]
    pub struct gss_OID_set_desc_struct {
        pub count: size_t,
        pub elements: gss_OID,
    }
    #[c2rust::src_loc = "112:1"]
    pub type gss_OID_set = *mut gss_OID_set_desc_struct;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    /* _GSSAPI_H_ */
}
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapi_generic.h:27"]
pub mod gssapi_generic_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID};
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
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
 * $Id$
 */
        /* Deprecated MIT krb5 oid names provided for compatibility.
 * The correct oids (GSS_C_NT_USER_NAME, etc) from rfc 2744
 * are defined in gssapi.h. */
        #[no_mangle]
        #[c2rust::src_loc = "50:27"]
        pub static mut gss_nt_user_name: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "51:27"]
        pub static mut gss_nt_machine_uid_name: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "52:27"]
        pub static mut gss_nt_string_uid_name: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "53:16"]
        pub static mut gss_nt_service_name_v2: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "54:27"]
        pub static mut gss_nt_service_name: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "55:16"]
        pub static mut gss_nt_exported_name: gss_OID;
    }
    /* _GSSAPI_GENERIC_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapi_ext.h:27"]
pub mod gssapi_ext_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "434:27"]
        pub static mut GSS_C_NT_COMPOSITE_EXPORT: gss_OID;
    }
    /* GSSAPI_EXT_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapiP_generic.h:27"]
pub mod gssapiP_generic_h {
    use super::gssapi_h::{OM_uint32, gss_OID_set, gss_OID_desc};
    extern "C" {
        /* buffer */
        #[no_mangle]
        #[c2rust::src_loc = "196:1"]
        pub fn generic_gss_release_oid_set(_: *mut OM_uint32,
                                           _: *mut gss_OID_set) -> OM_uint32;
        /* new_oid */
        #[no_mangle]
        #[c2rust::src_loc = "212:1"]
        pub fn generic_gss_create_empty_oid_set(_: *mut OM_uint32,
                                                _: *mut gss_OID_set)
         -> OM_uint32;
        /* oid_set */
        #[no_mangle]
        #[c2rust::src_loc = "217:1"]
        pub fn generic_gss_add_oid_set_member(_: *mut OM_uint32,
                                              _: *const gss_OID_desc,
                                              _: *mut gss_OID_set)
         -> OM_uint32;
    }
    /* _GSSAPIP_GENERIC_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/krb5/gssapi_krb5.h:27"]
pub mod gssapi_krb5_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID};
    extern "C" {
        /* {iso(1) member-body(2) United States(840) mit(113554) infosys(1) gssapi(2)
 * krb5(2) krb5-enterprise-name(6)}. */
        #[no_mangle]
        #[c2rust::src_loc = "81:33"]
        pub static gss_mech_krb5: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "82:33"]
        pub static gss_mech_krb5_old: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "83:33"]
        pub static gss_mech_krb5_wrong: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "84:33"]
        pub static gss_mech_iakerb: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "89:33"]
        pub static gss_nt_krb5_name: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "90:33"]
        pub static gss_nt_krb5_principal: gss_OID;
    }
    /* _GSSAPI_KRB5_H_ */
    /* __cplusplus */
}
pub use self::types_h::__uint32_t;
pub use self::stddef_h::size_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssapi_h::{OM_uint32, gss_uint32, gss_OID_desc_struct,
                         gss_OID_desc, gss_OID, gss_OID_set_desc_struct,
                         gss_OID_set};
use self::string_h::memcmp;
use self::gssapi_generic_h::{gss_nt_user_name, gss_nt_machine_uid_name,
                             gss_nt_string_uid_name, gss_nt_service_name_v2,
                             gss_nt_service_name, gss_nt_exported_name};
use self::gssapi_ext_h::GSS_C_NT_COMPOSITE_EXPORT;
use self::gssapiP_generic_h::{generic_gss_release_oid_set,
                              generic_gss_create_empty_oid_set,
                              generic_gss_add_oid_set_member};
use self::gssapi_krb5_h::{gss_mech_krb5, gss_mech_krb5_old,
                          gss_mech_krb5_wrong, gss_mech_iakerb,
                          gss_nt_krb5_name, gss_nt_krb5_principal};
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
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/gssapi/krb5/inq_names.c - Return nametypes supported by krb5 mech */
/*
 * Copyright 1995 by the Massachusetts Institute of Technology.
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
#[no_mangle]
#[c2rust::src_loc = "29:1"]
pub unsafe extern "C" fn krb5_gss_inquire_names_for_mech(mut minor_status:
                                                             *mut OM_uint32,
                                                         mut mechanism:
                                                             gss_OID,
                                                         mut name_types:
                                                             *mut gss_OID_set)
 -> OM_uint32 {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    /*
     * We only know how to handle our own mechanism.
     */
    if !mechanism.is_null() &&
           !((*gss_mech_krb5).length == (*mechanism).length &&
                 memcmp((*gss_mech_krb5).elements, (*mechanism).elements,
                        (*gss_mech_krb5).length as libc::c_ulong) ==
                     0 as libc::c_int) &&
           !((*gss_mech_krb5_old).length == (*mechanism).length &&
                 memcmp((*gss_mech_krb5_old).elements, (*mechanism).elements,
                        (*gss_mech_krb5_old).length as libc::c_ulong) ==
                     0 as libc::c_int) &&
           !((*gss_mech_krb5_wrong).length == (*mechanism).length &&
                 memcmp((*gss_mech_krb5_wrong).elements,
                        (*mechanism).elements,
                        (*gss_mech_krb5_wrong).length as libc::c_ulong) ==
                     0 as libc::c_int) &&
           !((*gss_mech_iakerb).length == (*mechanism).length &&
                 memcmp((*gss_mech_iakerb).elements, (*mechanism).elements,
                        (*gss_mech_iakerb).length as libc::c_ulong) ==
                     0 as libc::c_int) {
        *minor_status = 0 as libc::c_int as OM_uint32;
        return (1 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    /* We're okay.  Create an empty OID set */
    major = generic_gss_create_empty_oid_set(minor_status, name_types);
    if major == 0 as libc::c_int as libc::c_uint {
        /* Now add our members. */
        major =
            generic_gss_add_oid_set_member(minor_status,
                                           gss_nt_user_name as
                                               *const gss_OID_desc,
                                           name_types);
        if major == 0 as libc::c_int as libc::c_uint &&
               {
                   major =
                       generic_gss_add_oid_set_member(minor_status,
                                                      gss_nt_machine_uid_name
                                                          as
                                                          *const gss_OID_desc,
                                                      name_types);
                   (major) == 0 as libc::c_int as libc::c_uint
               } &&
               {
                   major =
                       generic_gss_add_oid_set_member(minor_status,
                                                      gss_nt_string_uid_name
                                                          as
                                                          *const gss_OID_desc,
                                                      name_types);
                   (major) == 0 as libc::c_int as libc::c_uint
               } &&
               {
                   major =
                       generic_gss_add_oid_set_member(minor_status,
                                                      gss_nt_service_name as
                                                          *const gss_OID_desc,
                                                      name_types);
                   (major) == 0 as libc::c_int as libc::c_uint
               } &&
               {
                   major =
                       generic_gss_add_oid_set_member(minor_status,
                                                      gss_nt_service_name_v2
                                                          as
                                                          *const gss_OID_desc,
                                                      name_types);
                   (major) == 0 as libc::c_int as libc::c_uint
               } &&
               {
                   major =
                       generic_gss_add_oid_set_member(minor_status,
                                                      gss_nt_exported_name as
                                                          *const gss_OID_desc,
                                                      name_types);
                   (major) == 0 as libc::c_int as libc::c_uint
               } &&
               {
                   major =
                       generic_gss_add_oid_set_member(minor_status,
                                                      gss_nt_krb5_name as
                                                          *const gss_OID_desc,
                                                      name_types);
                   (major) == 0 as libc::c_int as libc::c_uint
               } &&
               {
                   major =
                       generic_gss_add_oid_set_member(minor_status,
                                                      GSS_C_NT_COMPOSITE_EXPORT
                                                          as
                                                          *const gss_OID_desc,
                                                      name_types);
                   (major) == 0 as libc::c_int as libc::c_uint
               } {
            major =
                generic_gss_add_oid_set_member(minor_status,
                                               gss_nt_krb5_principal as
                                                   *const gss_OID_desc,
                                               name_types)
        }
        /*
         * If we choked, then release the set, but don't overwrite the minor
         * status with the release call.
         */
        if major != 0 as libc::c_int as libc::c_uint {
            generic_gss_release_oid_set(&mut minor, name_types);
        }
    }
    return major;
}
