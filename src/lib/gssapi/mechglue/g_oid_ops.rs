use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:27"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:27"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:27"]
pub mod gssapi_h {
    /*
 * The following type must be defined as the smallest natural unsigned integer
 * supported by the platform that has at least 32 bits of precision.
 */
    /* OM_STRING */
    /*
 * We can't use X/Open definitions, so roll our own.
 */
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "106:16"]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    /* OM_STRING */
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID_desc = gss_OID_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:16"]
    pub struct gss_OID_set_desc_struct {
        pub count: size_t,
        pub elements: gss_OID,
    }
    #[c2rust::src_loc = "112:1"]
    pub type gss_OID_set_desc = gss_OID_set_desc_struct;
    #[c2rust::src_loc = "112:1"]
    pub type gss_OID_set = *mut gss_OID_set_desc_struct;
    #[c2rust::src_loc = "850:1"]
    pub type gss_const_OID = *const gss_OID_desc;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapiP_generic.h:27"]
pub mod gssapiP_generic_h {
    use super::gssapi_h::{OM_uint32, gss_OID_set, gss_OID_desc,
                          gss_OID_set_desc_struct, gss_buffer_desc_struct,
                          gss_buffer_t, gss_OID, gss_OID_set_desc};
    extern "C" {
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
 * $Id$
 */
        /* * helper macros **/
        /* this code knows that an int on the wire is 32 bits.  The type of
   num should be at least this big, or the extra shifts may do weird
   things */
        /* * malloc wrappers; these may actually do something later */
        /* * helper functions **/
        /* hide names from applications, especially glib applications */
        /* flags for g_verify_token_header() */
        /* * declarations of internal name mechanism functions **/
        /* minor_status */
        /* buffer */
        /* minor_status */
        /* set */
        /* minor_status */
        /* set */
        /* minor_status */
        /* oid */
        /* new_oid */
        /* minor_status */
        /* oid_set */
        /* minor_status */
        /* member_oid */
        /* oid_set */
        /* minor_status */
        /* member */
        /* set */
        /* present */
        /* minor_status */
        /* oid */
        /* oid_str */
        /* minor_status */
        /* oid_str */
        /* oid */
        /* minor_status */
        /* prefix */
        /* prefix_len */
        /* suffix */
        /* oid */
        /* minor_status */
        /*prefix */
        /* prefix_len */
        /* oid */
        /* suffix */
        #[no_mangle]
        #[c2rust::src_loc = "263:1"]
        pub fn gssint_mecherrmap_map_errcode(errcode: OM_uint32) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "212:1"]
        pub fn generic_gss_create_empty_oid_set(_: *mut OM_uint32,
                                                _: *mut gss_OID_set)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "217:1"]
        pub fn generic_gss_add_oid_set_member(_: *mut OM_uint32,
                                              _: *const gss_OID_desc,
                                              _: *mut gss_OID_set)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "223:1"]
        pub fn generic_gss_test_oid_set_member(_: *mut OM_uint32,
                                               _: *const gss_OID_desc,
                                               _: gss_OID_set,
                                               _: *mut libc::c_int)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "236:1"]
        pub fn generic_gss_str_to_oid(_: *mut OM_uint32, _: gss_buffer_t,
                                      _: *mut gss_OID) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "230:1"]
        pub fn generic_gss_oid_to_str(_: *mut OM_uint32,
                                      _: *const gss_OID_desc, _: gss_buffer_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "316:1"]
        pub fn generic_gss_copy_oid_set(_: *mut OM_uint32,
                                        _: *const gss_OID_set_desc,
                                        _: *mut gss_OID_set) -> OM_uint32;
    }
    /* _GSSAPIP_GENERIC_H_ */
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
pub use self::types_h::__uint32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::stddef_h::size_t;
pub use self::gssapi_h::{gss_OID, gss_OID_desc_struct, OM_uint32, gss_uint32,
                         gss_buffer_t, gss_buffer_desc_struct, gss_OID_desc,
                         gss_OID_set_desc_struct, gss_OID_set_desc,
                         gss_OID_set, gss_const_OID};
use self::gssapiP_generic_h::{gssint_mecherrmap_map_errcode,
                              generic_gss_create_empty_oid_set,
                              generic_gss_add_oid_set_member,
                              generic_gss_test_oid_set_member,
                              generic_gss_str_to_oid, generic_gss_oid_to_str,
                              generic_gss_copy_oid_set};
use self::string_h::memcmp;
/* #pragma ident	"@(#)g_oid_ops.c	1.11	98/01/22 SMI" */
/* lib/gssapi/mechglue/g_oid_ops.c - GSSAPI V2 interfaces to manipulate OIDs */
/*
 * Copyright 1995, 2007 by the Massachusetts Institute of Technology.
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
 * gss_release_oid has been moved to g_initialize, becasue it requires access
 * to the mechanism list.  All functions requiring direct access to the
 * mechanism list are now in g_initialize.c
 */
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn gss_create_empty_oid_set(mut minor_status:
                                                      *mut OM_uint32,
                                                  mut oid_set:
                                                      *mut gss_OID_set)
 -> OM_uint32 {
    let mut status: OM_uint32 = 0;
    status = generic_gss_create_empty_oid_set(minor_status, oid_set);
    if status != 0 as libc::c_int as libc::c_uint {
        *minor_status = gssint_mecherrmap_map_errcode(*minor_status)
    }
    return status;
}
#[no_mangle]
#[c2rust::src_loc = "47:1"]
pub unsafe extern "C" fn gss_add_oid_set_member(mut minor_status:
                                                    *mut OM_uint32,
                                                mut member_oid: gss_OID,
                                                mut oid_set: *mut gss_OID_set)
 -> OM_uint32 {
    let mut status: OM_uint32 = 0;
    status =
        generic_gss_add_oid_set_member(minor_status,
                                       member_oid as *const gss_OID_desc,
                                       oid_set);
    if status != 0 as libc::c_int as libc::c_uint {
        *minor_status = gssint_mecherrmap_map_errcode(*minor_status)
    }
    return status;
}
#[no_mangle]
#[c2rust::src_loc = "60:1"]
pub unsafe extern "C" fn gss_test_oid_set_member(mut minor_status:
                                                     *mut OM_uint32,
                                                 mut member: gss_OID,
                                                 mut set: gss_OID_set,
                                                 mut present:
                                                     *mut libc::c_int)
 -> OM_uint32 {
    return generic_gss_test_oid_set_member(minor_status,
                                           member as *const gss_OID_desc, set,
                                           present);
}
/* oid */
/* New for V2 */
#[no_mangle]
#[c2rust::src_loc = "70:1"]
pub unsafe extern "C" fn gss_oid_to_str(mut minor_status: *mut OM_uint32,
                                        mut oid: gss_OID,
                                        mut oid_str: gss_buffer_t)
 -> OM_uint32 {
    let mut status: OM_uint32 =
        generic_gss_oid_to_str(minor_status, oid as *const gss_OID_desc,
                               oid_str);
    if status != 0 as libc::c_int as libc::c_uint {
        *minor_status = gssint_mecherrmap_map_errcode(*minor_status)
    }
    return status;
}
/* oid */
/* New for V2 */
/* minor_status */
/* oid_set */
/* New for V2 */
/* minor_status */
/* member_oid */
/* oid_set */
/* New for V2 */
/* minor_status */
/* member */
/* set */
/* present */
/* New for V2 */
#[no_mangle]
#[c2rust::src_loc = "82:1"]
pub unsafe extern "C" fn gss_str_to_oid(mut minor_status: *mut OM_uint32,
                                        mut oid_str: gss_buffer_t,
                                        mut oid: *mut gss_OID) -> OM_uint32 {
    let mut status: OM_uint32 =
        generic_gss_str_to_oid(minor_status, oid_str, oid);
    if status != 0 as libc::c_int as libc::c_uint {
        *minor_status = gssint_mecherrmap_map_errcode(*minor_status)
    }
    return status;
}
/*
 * Rudimentary pointer validation macro to check whether the
 * "loopback" field of an opaque struct points back to itself.  This
 * field also catches some programming errors where an opaque pointer
 * is passed to a function expecting the address of the opaque
 * pointer.
 */
/* *******************************************************/
/* The Mechanism Dispatch Table -- a mechanism needs to */
/* define one of these and provide a function to return */
/* it to initialize the GSSAPI library		  */
/*
 * This is the definition of the mechs_array struct, which is used to
 * define the mechs array table. This table is used to indirectly
 * access mechanism specific versions of the gssapi routines through
 * the routines in the glue module (gssd_mech_glue.c)
 *
 * This contants all of the functions defined in gssapi.h except for
 * gss_release_buffer() and gss_release_oid_set(), which I am
 * assuming, for now, to be equal across mechanisms.
 */
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
/* token_buffer */
/* minor_status */
/* context_handle */
/* output_token */
/* minor_status */
/* context_handle */
/* time_rec */
/* minor_status */
/* context_handle */
/* qop_req */
/* message_buffer */
/* message_token */
/* minor_status */
/* context_handle */
/* message_buffer */
/* token_buffer */
/* qop_state */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* input_message_buffer */
/* conf_state */
/* output_message_buffer */
/* minor_status */
/* context_handle */
/* input_message_buffer */
/* output_message_buffer */
/* conf_state */
/* qop_state */
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
/* input_cred_handle */
/* desired_name */
/* desired_mech */
/* cred_usage */
/* initiator_time_req */
/* acceptor_time_req */
/* output_cred_handle */
/* actual_mechs */
/* initiator_time_rec */
/* acceptor_time_rec */
/* minor_status */
/* context_handle */
/* interprocess_token */
/* minor_status */
/* interprocess_token */
/* context_handle */
/* minor_status */
/* cred_handle */
/* mech_type */
/* name */
/* initiator_lifetime */
/* acceptor_lifetime */
/* cred_usage */
/* minor_status */
/* mechanism */
/* name_types */
/* minor_status */
/* context_handle */
/* src_name */
/* targ_name */
/* lifetime_rec */
/* mech_type */
/* ctx_flags */
/* locally_initiated */
/* open */
/* minor_status */
/* OID */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* req_output_size */
/* max_input_size */
/* minor */
/* name */
/* mech_type */
/* localname */
/* minor_status */
/* pname */
/* local user */
/* local nametype */
/* */
/* minor_status */
/* input_name */
/* exported_name */
/* */
/* minor_status */
/* input_name */
/* output_name */
/* */
/* minor_status */
/* input_cred */
/* cred_usage */
/* desired_mech */
/* overwrite_cred */
/* default_cred */
/* elements_stored */
/* cred_usage_stored */
/* */
/* GGF extensions */
/* minor_status */
/* context_handle */
/* OID */
/* data_set */
/* minor_status */
/* cred_handle */
/* OID */
/* data_set */
/* minor_status */
/* context_handle */
/* OID */
/* value */
/* minor_status */
/* cred_handle */
/* OID */
/* value */
/* minor_status */
/* mech OID */
/* OID */
/* value */
/* AEAD extensions */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* input_assoc_buffer */
/* input_payload_buffer */
/* conf_state */
/* output_message_buffer */
/* */
/* minor_status */
/* context_handle */
/* input_message_buffer */
/* input_assoc_buffer */
/* output_payload_buffer */
/* conf_state */
/* qop_state */
/* */
/* SSPI extensions */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* conf_state */
/* iov */
/* iov_count */
/* */
/* minor_status */
/* context_handle */
/* conf_state */
/* qop_state */
/* iov */
/* iov_count */
/* */
/* minor_status */
/* context_handle */
/* conf_req_flag*/
/* qop_req */
/* conf_state */
/* iov */
/* iov_count */
/* */
/* minor_status */
/* context_handle */
/* input_message_buffer */
/* New for 1.8 */
/* minor_status */
/* impersonator_cred_handle */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* */
/* minor_status */
/* input_cred_handle */
/* impersonator_cred_handle */
/* desired_name */
/* desired_mech */
/* cred_usage */
/* initiator_time_req */
/* acceptor_time_req */
/* output_cred_handle */
/* actual_mechs */
/* initiator_time_rec */
/* acceptor_time_rec */
/* */
/* minor_status */
/* name */
/* display_as_name_type */
/* display_name */
/* */
/* minor_status */
/* name */
/* name_is_MN */
/* MN_mech */
/* attrs */
/* */
/* minor_status */
/* name */
/* attr */
/* authenticated */
/* complete */
/* value */
/* display_value */
/* more */
/* */
/* minor_status */
/* name */
/* complete */
/* attr */
/* value */
/* */
/* minor_status */
/* name */
/* attr */
/* */
/* minor_status */
/* name */
/* exp_composite_name */
/* */
/* minor_status */
/* name */
/* authenticated */
/* type_id */
/* output */
/* */
/* minor_status */
/* name */
/* type_id */
/* input */
/* */
/* minor_status */
/* context */
/* prf_key */
/* prf_in */
/* desired_output_len */
/* prf_out */
/* */
/* minor_status */
/* cred_handle */
/* mech_set */
/* */
/* minor_status */
/* desired_mech */
/* sasl_mech_name */
/* mech_name */
/* mech_description */
/* */
/* minor_status */
/* sasl_mech_name */
/* mech_type */
/* */
/* minor_status */
/* mech */
/* mech_attrs */
/* known_mech_attrs */
/* */
/* Credential store extensions */
/* minor_status */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* cred_store */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* */
/* minor_status */
/* input_cred_handle */
/* input_usage */
/* desired_mech */
/* overwrite_cred */
/* default_cred */
/* cred_store */
/* elements_stored */
/* cred_usage_stored */
/* */
/* minor_status */
/* desired_name */
/* password */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* */
/* minor_status */
/* cred_handle */
/* token */
/* */
/* minor_status */
/* token */
/* cred_handle */
/* */
/* minor_status */
/* desired_mech */
/* interprocess_token */
/* context_handle */
/* */
/* minor_status */
/* mech_type */
/* input_name_buffer */
/* input_name_type */
/* output_name */
/* */
/* minor_status */
/* mech_type */
/* token */
/* cred_handle */
/* */
/* get_mic_iov extensions, added in 1.12 */
/* minor_status */
/* context_handle */
/* qop_req */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* qop_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* qop_req */
/* iov */
/* iov_count */
/* NegoEx extensions added in 1.18 */
/* minor_status */
/* mech_oid */
/* cred_handle */
/* context_handle */
/* targ_name */
/* req_flags */
/* meta_data */
/* */
/* minor_status */
/* mech_oid */
/* cred_handle */
/* context_handle */
/* targ_name */
/* req_flags */
/* meta_data */
/* */
/* minor_status */
/* mech_oid */
/* auth_scheme */
/* */
/*
 * In the user space we use a wrapper structure to encompass the
 * mechanism entry points.  The wrapper contain the mechanism
 * entry points and other data which is only relevant to the gss-api
 * layer.  In the kernel we use only the gss_config strucutre because
 * the kernal does not cantain any of the extra gss-api specific data.
 */
/* kernel module name */
/* user library name */
/* mechanism string name */
/* optional mech parameters */
/* RTLD object handle for the mech */
/* mechanism oid */
/* mechanism initialization struct */
/* mechanism preference order */
/* free mech table */
/* interposer mechanism flag */
/* points to the interposer OID */
/* points to the interposer mech */
/* next element in the list */
/* *******************************************************/
/* Internal mechglue routines */
/* minor_status */
/* mech */
/* internal_name */
/* external_name */
/* union_cred */
/* mech_type */
/* src buffer */
/* destination buffer */
/* NULL terminate buffer ? */
/* minor_status */
/* mech_oid */
/* ctx_out */
#[no_mangle]
#[c2rust::src_loc = "94:1"]
pub unsafe extern "C" fn gssint_copy_oid_set(mut minor_status: *mut OM_uint32,
                                             oidset: *const gss_OID_set_desc,
                                             mut new_oidset: *mut gss_OID_set)
 -> OM_uint32 {
    return generic_gss_copy_oid_set(minor_status, oidset, new_oidset);
}
/*
 * Copyright 2008 by the Massachusetts Institute of Technology.
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
/* __cplusplus */
/*
 * Solaris extensions
 */
/* *
 * Provides a platform-specific name for a GSSAPI name as interpreted by a
 * given mechanism.
 *
 * @param [out] minor      Minor status code
 * @param [in] name        The gss name resulting from accept_sec_context
 * @param [in] mech_type   The mechanism that will be asked to map @a name to a
 *                         local name
 * @param [out] localname  Caller-allocated buffer to be filled in with the
 *                         local name on success
 */
/* *
 * Determine whether a mechanism name is authorized to act as a username.
 *
 * @param [in] name      Mechanism name
 * @param [in] username  System username
 *
 * This is a simple wrapper around gss_authorize_localname().  It only supports
 * system usernames as local names, and cannot distinguish between lack of
 * authorization and other errors.
 *
 * @retval 1 @a name is authorized to act as @a username
 * @retval 0 @a name is not authorized or an error occurred
 */
/* *
 *  Determine whether a mechanism name is authorized to act as a local name.
 *
 * @param [out] minor  Minor status code
 * @param [in] name    Mechanism name
 * @param [in] user    Local name
 *
 * @a name is a mechanism name, typically the result of a completed
 * gss_accept_sec_context().  @a user is an internal name representing a local
 * name, such as a name imported by gss_import_name() with an @a
 * input_name_type of @c GSS_C_NT_USER_NAME.
 *
 * @return Return GSS_S_COMPLETE if @a name is authorized to act as @a user,
 * GSS_S_UNAUTHORIZED if not, or an appropriate GSS error code if an error
 * occurred.
 *
 * @sa gss_userok
 */
/* minor_status */
/* desired_name */
/* password */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* minor_status */
/* input_cred_handle */
/* desired_name */
/* desired_mech */
/* password */
/* cred_usage */
/* initiator_time_req */
/* acceptor_time_req */
/* output_cred_handle */
/* actual_mechs */
/* initiator_time_rec */
/* acceptor_time_rec */
/*
 * GGF extensions
 */
/*minor_status*/
/*buffer_set*/
/*minor_status*/
/*member_buffer*/
/*buffer_set*/
/*minor_status*/
/*buffer_set*/
/*minor_status*/
/*context_handle*/
/*desired_object*/
/*data_set*/
/*minor_status*/
/*cred_handle*/
/*desired_object*/
/*data_set*/
/*minor_status*/
/*cred_handle*/
/*desired_object*/
/*value*/
/*
 * Export import cred extensions from GGF, but using Heimdal's signatures
 */
/* minor_status */
/* cred_handle */
/* token */
/* minor_status */
/* token */
/* cred_handle */
/*
 * Heimdal extension
 */
/*minor_status*/
/*cred*/
/*desired_object*/
/*value*/
/*
 * Call the given method on the given mechanism
 */
/*minor_status*/
/*desired_mech*/
/*desired_object*/
/*value*/
/*
 * AEAD extensions
 */
/*minor_status*/
/*context_handle*/
/*conf_req_flag*/
/*qop_req*/
/*input_assoc_buffer*/
/*input_payload_buffer*/
/*conf_state*/
/*output_message_buffer*/
/*minor_status*/
/*context_handle*/
/*input_message_buffer*/
/*input_assoc_buffer*/
/*output_payload_buffer*/
/*conf_state*/
/*qop_state*/
/*
 * SSPI extensions
 */
/*
 * Returns a buffer set with the first member containing the
 * session key for SSPI compatibility. The optional second
 * member contains an OID identifying the session key type.
 */
/* Packet data */
/* Mechanism header */
/* Mechanism specific parameters */
/* Mechanism trailer */
/* Padding */
/* Complete wrap token */
/* Sign only packet data */
/* MIC token destination */
/* indicates GSS should allocate */
/* indicates caller should free */
/*
 * Sign and optionally encrypt a sequence of buffers. The buffers
 * shall be ordered HEADER | DATA | PADDING | TRAILER. Suitable
 * space for the header, padding and trailer should be provided
 * by calling gss_wrap_iov_length(), or the ALLOCATE flag should
 * be set on those buffers.
 *
 * Encryption is in-place. SIGN_ONLY buffers are untouched. Only
 * a single PADDING buffer should be provided. The order of the
 * buffers in memory does not matter. Buffers in the IOV should
 * be arranged in the order above, and in the case of multiple
 * DATA buffers the sender and receiver should agree on the
 * order.
 *
 * With GSS_C_DCE_STYLE it is acceptable to not provide PADDING
 * and TRAILER, but the caller must guarantee the plaintext data
 * being encrypted is correctly padded, otherwise an error will
 * be returned.
 *
 * While applications that have knowledge of the underlying
 * cryptosystem may request a specific configuration of data
 * buffers, the only generally supported configurations are:
 *
 *  HEADER | DATA | PADDING | TRAILER
 *
 * which will emit GSS_Wrap() compatible tokens, and:
 *
 *  HEADER | SIGN_ONLY | DATA | PADDING | TRAILER
 *
 * for AEAD.
 *
 * The typical (special cased) usage for DCE is as follows:
 *
 *  SIGN_ONLY_1 | DATA | SIGN_ONLY_2 | HEADER
 */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* conf_state */
/* iov */
/* iov_count */
/*
 * Verify and optionally decrypt a sequence of buffers. To process
 * a GSS-API message without separate buffer, pass STREAM | DATA.
 * Upon return DATA will contain the decrypted or integrity
 * protected message. Only a single DATA buffer may be provided
 * with this usage. DATA by default will point into STREAM, but if
 * the ALLOCATE flag is set a copy will be returned.
 *
 * Otherwise, decryption is in-place. SIGN_ONLY buffers are
 * untouched.
 */
/* minor_status */
/* context_handle */
/* conf_state */
/* qop_state */
/* iov */
/* iov_count */
/*
 * Query HEADER, PADDING and TRAILER buffer lengths. DATA buffers
 * should be provided so the correct padding length can be determined.
 */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* conf_state */
/* iov */
/* iov_count */
/*
 * Produce a GSSAPI MIC token for a sequence of buffers.  All SIGN_ONLY and
 * DATA buffers will be signed, in the order they appear.  One MIC_TOKEN buffer
 * must be included for the result.  Suitable space should be provided for the
 * MIC_TOKEN buffer by calling gss_get_mic_iov_length, or the ALLOCATE flag
 * should be set on that buffer.  If the ALLOCATE flag is used, use
 * gss_release_iov_buffer to free the allocated buffer within the iov list when
 * it is no longer needed.
 */
/* minor_status */
/* context_handle */
/* qop_req */
/* iov */
/* iov_count */
/*
 * Query the MIC_TOKEN buffer length within the iov list.
 */
/* minor_status */
/* context_handle */
/* qop_req */
/* iov */
/* iov_count */
/*
 * Verify the MIC_TOKEN buffer within the iov list against the SIGN_ONLY and
 * DATA buffers in the order they appear.  Return values are the same as for
 * gss_verify_mic.
 */
/* minor_status */
/* context_handle */
/* qop_state */
/* iov */
/* iov_count */
/*
 * Release buffers that have the ALLOCATED flag set.
 */
/* minor_status */
/* iov */
/* iov_count */
/*
 * Protocol transition
 */
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
/* input_cred_handle */
/* impersonator_cred_handle */
/* desired_name */
/* desired_mech */
/* cred_usage */
/* initiator_time_req */
/* acceptor_time_req */
/* output_cred_handle */
/* actual_mechs */
/* initiator_time_rec */
/* acceptor_time_rec */
/*
 * Naming extensions
 */
/* minor_status */
/* name */
/* display_as_name_type */
/* display_name */
/* minor_status */
/* name */
/* name_is_MN */
/* MN_mech */
/* attrs */
/* minor_status */
/* name */
/* attr */
/* authenticated */
/* complete */
/* value */
/* display_value */
/* more */
/* minor_status */
/* name */
/* complete */
/* attr */
/* value */
/* minor_status */
/* name */
/* attr */
/* minor_status */
/* name */
/* exp_composite_name */
/* minor_status */
/* name */
/* authenticated */
/* type_id */
/* output */
/* minor_status */
/* name */
/* type_id */
/* input */
/* draft-josefsson-gss-capsulate */
/* input_token */
/* token_oid */
/* output_token */
/* input_token */
/* token_oid */
/* output_token */
#[no_mangle]
#[c2rust::src_loc = "103:1"]
pub unsafe extern "C" fn gss_oid_equal(mut first_oid: gss_const_OID,
                                       mut second_oid: gss_const_OID)
 -> libc::c_int {
    /* GSS_C_NO_OID doesn't match itself, per draft-josefsson-gss-capsulate. */
    if first_oid.is_null() || second_oid.is_null() { return 0 as libc::c_int }
    return ((*first_oid).length == (*second_oid).length &&
                memcmp((*first_oid).elements, (*second_oid).elements,
                       (*first_oid).length as libc::c_ulong) ==
                    0 as libc::c_int) as libc::c_int;
}
