pub use crate::stddef_h::size_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::uint32_t;

pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_OID_set;
pub use crate::gssapi_h::gss_OID_set_desc;
pub use crate::gssapi_h::gss_OID_set_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_const_OID;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;

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

pub unsafe extern "C" fn gss_create_empty_oid_set(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut oid_set: *mut crate::gssapi_h::gss_OID_set,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    status = crate::src::generic::oid_ops::generic_gss_create_empty_oid_set(minor_status, oid_set);
    if status != 0u32 {
        *minor_status =
            crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status)
    }
    return status;
}
#[no_mangle]

pub unsafe extern "C" fn gss_add_oid_set_member(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut member_oid: crate::gssapi_h::gss_OID,
    mut oid_set: *mut crate::gssapi_h::gss_OID_set,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    status = crate::src::generic::oid_ops::generic_gss_add_oid_set_member(
        minor_status,
        member_oid as *const crate::gssapi_h::gss_OID_desc,
        oid_set,
    );
    if status != 0u32 {
        *minor_status =
            crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status)
    }
    return status;
}
#[no_mangle]

pub unsafe extern "C" fn gss_test_oid_set_member(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut member: crate::gssapi_h::gss_OID,
    mut set: crate::gssapi_h::gss_OID_set,
    mut present: *mut i32,
) -> crate::gssapi_h::OM_uint32 {
    return crate::src::generic::oid_ops::generic_gss_test_oid_set_member(
        minor_status,
        member as *const crate::gssapi_h::gss_OID_desc,
        set,
        present,
    );
}
/* oid */
/* New for V2 */
#[no_mangle]

pub unsafe extern "C" fn gss_oid_to_str(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut oid: crate::gssapi_h::gss_OID,
    mut oid_str: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status = crate::src::generic::oid_ops::generic_gss_oid_to_str(
        minor_status,
        oid as *const crate::gssapi_h::gss_OID_desc,
        oid_str,
    );
    if status != 0u32 {
        *minor_status =
            crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status)
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

pub unsafe extern "C" fn gss_str_to_oid(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut oid_str: crate::gssapi_h::gss_buffer_t,
    mut oid: *mut crate::gssapi_h::gss_OID,
) -> crate::gssapi_h::OM_uint32 {
    let mut status =
        crate::src::generic::oid_ops::generic_gss_str_to_oid(minor_status, oid_str, oid);
    if status != 0u32 {
        *minor_status =
            crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status)
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

pub unsafe extern "C" fn gssint_copy_oid_set(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    oidset: *const crate::gssapi_h::gss_OID_set_desc,
    mut new_oidset: *mut crate::gssapi_h::gss_OID_set,
) -> crate::gssapi_h::OM_uint32 {
    return crate::src::generic::oid_ops::generic_gss_copy_oid_set(
        minor_status,
        oidset,
        new_oidset,
    );
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

pub unsafe extern "C" fn gss_oid_equal(
    mut first_oid: crate::gssapi_h::gss_const_OID,
    mut second_oid: crate::gssapi_h::gss_const_OID,
) -> i32 {
    /* GSS_C_NO_OID doesn't match itself, per draft-josefsson-gss-capsulate. */
    if first_oid.is_null() || second_oid.is_null() {
        return 0i32;
    }
    return ((*first_oid).length == (*second_oid).length
        && crate::stdlib::memcmp(
            (*first_oid).elements,
            (*second_oid).elements,
            (*first_oid).length as usize,
        ) == 0i32) as i32;
}
