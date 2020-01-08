/* _GSSAPIP_GENERIC_H_ */
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__ssize_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::ssize_t;
pub use crate::stdlib::uint32_t;

pub use crate::gssapi_ext_h::gss_any;
pub use crate::gssapi_ext_h::gss_any_t;
pub use crate::gssapi_ext_h::gss_buffer_set_desc_struct;
pub use crate::gssapi_ext_h::gss_buffer_set_t;
pub use crate::gssapi_ext_h::gss_const_key_value_set_t;
pub use crate::gssapi_ext_h::gss_iov_buffer_desc;
pub use crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
pub use crate::gssapi_ext_h::gss_key_value_element_desc;
pub use crate::gssapi_ext_h::gss_key_value_element_struct;
pub use crate::gssapi_ext_h::gss_key_value_set_desc;
pub use crate::gssapi_ext_h::gss_key_value_set_struct;
pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_OID_set;
pub use crate::gssapi_h::gss_OID_set_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_channel_bindings_struct;
pub use crate::gssapi_h::gss_channel_bindings_t;
pub use crate::gssapi_h::gss_const_OID;
pub use crate::gssapi_h::gss_const_buffer_t;
pub use crate::gssapi_h::gss_cred_id_t;
pub use crate::gssapi_h::gss_cred_usage_t;
pub use crate::gssapi_h::gss_ctx_id_struct;
pub use crate::gssapi_h::gss_ctx_id_t;
pub use crate::gssapi_h::gss_name_t;
pub use crate::gssapi_h::gss_qop_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::mglueP_h::gss_config;
pub use crate::mglueP_h::gss_cred_id_struct;
pub use crate::mglueP_h::gss_mechanism;
pub use crate::mglueP_h::gss_name_struct;
pub use crate::mglueP_h::gss_union_cred_t;

pub use crate::src::mechglue::g_glue::gssint_get_mechanism_cred;
pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;
pub use crate::src::mechglue::g_initialize::gssint_get_public_oid;
pub use crate::src::mechglue::g_initialize::gssint_select_mech_type;
pub use crate::src::mechglue::g_oid_ops::gss_add_oid_set_member;
pub use crate::src::mechglue::g_oid_ops::gss_create_empty_oid_set;
/*
 * Copyright 2004 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
/* #pragma ident	"@(#)g_store_cred.c	1.2	04/04/05 SMI" */
/*
 *  glue routine for gss_store_cred
 */

unsafe extern "C" fn store_cred_fallback(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut mech: crate::mglueP_h::gss_mechanism,
    mut mech_cred: crate::gssapi_h::gss_cred_id_t,
    mut cred_usage: crate::gssapi_h::gss_cred_usage_t,
    mut desired_mech: crate::gssapi_h::gss_OID,
    mut overwrite_cred: crate::gssapi_h::OM_uint32,
    mut default_cred: crate::gssapi_h::OM_uint32,
    mut cred_store: crate::gssapi_ext_h::gss_const_key_value_set_t,
    mut elements_stored: *mut crate::gssapi_h::gss_OID_set,
    mut cred_usage_stored: *mut crate::gssapi_h::gss_cred_usage_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut public_mech = crate::src::mechglue::g_initialize::gssint_get_public_oid(
        desired_mech as crate::gssapi_h::gss_const_OID,
    );
    if (*mech).gss_store_cred_into.is_some() {
        return (*mech)
            .gss_store_cred_into
            .expect("non-null function pointer")(
            minor_status,
            mech_cred,
            cred_usage,
            public_mech,
            overwrite_cred,
            default_cred,
            cred_store,
            elements_stored,
            cred_usage_stored,
        );
    } else if cred_store.is_null() {
        return (*mech).gss_store_cred.expect("non-null function pointer")(
            minor_status,
            mech_cred,
            cred_usage,
            public_mech,
            overwrite_cred,
            default_cred,
            elements_stored,
            cred_usage_stored,
        );
    } else {
        return (16u32) << 16i32;
    };
}

unsafe extern "C" fn val_store_cred_args(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    input_cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut cred_usage: crate::gssapi_h::gss_cred_usage_t,
    _desired_mech: crate::gssapi_h::gss_OID,
    mut _overwrite_cred: crate::gssapi_h::OM_uint32,
    mut _default_cred: crate::gssapi_h::OM_uint32,
    mut cred_store: crate::gssapi_ext_h::gss_const_key_value_set_t,
    mut elements_stored: *mut crate::gssapi_h::gss_OID_set,
    mut _cred_usage_stored: *mut crate::gssapi_h::gss_cred_usage_t,
) -> crate::gssapi_h::OM_uint32 {
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !elements_stored.is_null() {
        *elements_stored = 0 as crate::gssapi_h::gss_OID_set
    }
    /* Validate arguments. */
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    if input_cred_handle.is_null() {
        return (1u32) << 24i32 | (7u32) << 16i32;
    }
    if cred_usage != 2i32 && cred_usage != 1i32 && cred_usage != 0i32 {
        if !minor_status.is_null() {
            *minor_status = 22u32;
            *minor_status =
                crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status)
        }
        return (13u32) << 16i32;
    }
    if !cred_store.is_null() && (*cred_store).count == 0u32 {
        *minor_status = 22u32;
        *minor_status =
            crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status);
        return (13u32) << 16i32;
    }
    return 0u32;
}
/* prf_out */
#[no_mangle]

pub unsafe extern "C" fn gss_store_cred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut input_cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut cred_usage: crate::gssapi_h::gss_cred_usage_t,
    desired_mech: crate::gssapi_h::gss_OID,
    mut overwrite_cred: crate::gssapi_h::OM_uint32,
    mut default_cred: crate::gssapi_h::OM_uint32,
    mut elements_stored: *mut crate::gssapi_h::gss_OID_set,
    mut cred_usage_stored: *mut crate::gssapi_h::gss_cred_usage_t,
) -> crate::gssapi_h::OM_uint32 {
    return gss_store_cred_into(
        minor_status,
        input_cred_handle,
        cred_usage,
        desired_mech,
        overwrite_cred,
        default_cred,
        0 as crate::gssapi_ext_h::gss_const_key_value_set_t,
        elements_stored,
        cred_usage_stored,
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
/* first_oid */
/* second_oid */
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
/* minor_status */
/* input_cred_handle */
/* desired_name */
/* desired_mech */
/* cred_usage */
/* initiator_time_req */
/* acceptor_time_req */
/* cred_store */
/* output_cred_handle */
/* actual_mechs */
/* initiator_time_rec */
/* acceptor_time_rec */
#[no_mangle]

pub unsafe extern "C" fn gss_store_cred_into(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut input_cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut cred_usage: crate::gssapi_h::gss_cred_usage_t,
    mut desired_mech: crate::gssapi_h::gss_OID,
    mut overwrite_cred: crate::gssapi_h::OM_uint32,
    mut default_cred: crate::gssapi_h::OM_uint32,
    mut cred_store: crate::gssapi_ext_h::gss_const_key_value_set_t,
    mut elements_stored: *mut crate::gssapi_h::gss_OID_set,
    mut cred_usage_stored: *mut crate::gssapi_h::gss_cred_usage_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut major_status = (13u32) << 16i32;
    let mut union_cred = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    let mut mech_cred = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    let mut dmech = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut selected_mech = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut i: i32 = 0;
    major_status = val_store_cred_args(
        minor_status,
        input_cred_handle,
        cred_usage,
        desired_mech,
        overwrite_cred,
        default_cred,
        cred_store,
        elements_stored,
        cred_usage_stored,
    );
    if major_status != 0u32 {
        return major_status;
    }
    /* Initial value needed below. */
    major_status = (13u32) << 16i32; /* there's no GSS_C_NEITHER */
    if !cred_usage_stored.is_null() {
        *cred_usage_stored = 0i32
    }
    union_cred = input_cred_handle;
    /* desired_mech != GSS_C_NULL_OID -> store one element */
    if !desired_mech.is_null() {
        major_status = crate::src::mechglue::g_initialize::gssint_select_mech_type(
            minor_status,
            desired_mech as crate::gssapi_h::gss_const_OID,
            &mut selected_mech,
        );
        if major_status != 0u32 {
            return major_status;
        }
        mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
            selected_mech as crate::gssapi_h::gss_const_OID,
        );
        if mech.is_null() {
            return (1u32) << 16i32;
        }
        if (*mech).gss_store_cred_into.is_none() && !cred_store.is_null() {
            return major_status;
        }
        if (*mech).gss_store_cred.is_none() && (*mech).gss_store_cred_into.is_none() {
            return major_status;
        }
        mech_cred =
            crate::src::mechglue::g_glue::gssint_get_mechanism_cred(union_cred, selected_mech);
        if mech_cred.is_null() {
            return (7u32) << 16i32;
        }
        major_status = store_cred_fallback(
            minor_status,
            mech,
            mech_cred,
            cred_usage,
            selected_mech,
            overwrite_cred,
            default_cred,
            cred_store,
            elements_stored,
            cred_usage_stored,
        );
        if major_status != 0u32 {
            *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                *minor_status,
                &mut (*mech).mech_type,
            )
        }
        return major_status;
    }
    /* desired_mech == GSS_C_NULL_OID -> store all elements */
    *minor_status = 0u32;
    i = 0i32;
    while i < (*union_cred).count {
        /* Get mech and cred element */
        dmech = &mut *(*union_cred).mechs_array.offset(i as isize)
            as *mut crate::gssapi_h::gss_OID_desc_struct; /* can't happen, but safe to ignore */
        mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
            dmech as crate::gssapi_h::gss_const_OID,
        );
        if !mech.is_null() {
            if !((*mech).gss_store_cred_into.is_none() && !cred_store.is_null()) {
                if !((*mech).gss_store_cred.is_none() && (*mech).gss_store_cred_into.is_none()) {
                    mech_cred =
                        crate::src::mechglue::g_glue::gssint_get_mechanism_cred(union_cred, dmech);
                    if !mech_cred.is_null() {
                        major_status = store_cred_fallback(
                            minor_status,
                            mech,
                            mech_cred,
                            cred_usage,
                            dmech,
                            overwrite_cred,
                            default_cred,
                            cred_store,
                            0 as *mut crate::gssapi_h::gss_OID_set,
                            cred_usage_stored,
                        );
                        if major_status != 0u32 {
                            *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                                *minor_status,
                                &mut (*mech).mech_type,
                            )
                        } else if !elements_stored.is_null() {
                            if (*elements_stored).is_null() {
                                major_status =
                                    crate::src::mechglue::g_oid_ops::gss_create_empty_oid_set(
                                        minor_status,
                                        elements_stored,
                                    );
                                if major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
                                    return major_status;
                                }
                            }
                            major_status = crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(
                                minor_status,
                                dmech,
                                elements_stored,
                            );
                            /* Succeeded for at least one mech */
                            /* The caller should clean up elements_stored */
                            if major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
                                return major_status;
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    /*
     * Success with some mechs may mask failure with others, but
     * that's what elements_stored is for.
     */
    return major_status;
}
