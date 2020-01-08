use ::libc;

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
pub use crate::mglueP_h::gss_union_ctx_id_struct;
pub use crate::mglueP_h::gss_union_ctx_id_t;
pub use crate::mglueP_h::gss_union_name_t;

pub use crate::src::mechglue::g_glue::gssint_create_union_context;
pub use crate::src::mechglue::g_glue::gssint_delete_internal_sec_context;
pub use crate::src::mechglue::g_glue::gssint_get_mechanism_cred;
pub use crate::src::mechglue::g_glue::gssint_import_internal_name;
pub use crate::src::mechglue::g_glue::gssint_release_internal_name;
pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;
pub use crate::src::mechglue::g_initialize::gssint_get_public_oid;
pub use crate::src::mechglue::g_initialize::gssint_select_mech_type;

/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2011 by the Massachusetts Institute of Technology.
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
/*
 * This file contains dispatch functions for the three GSSAPI extensions
 * described in draft-zhu-negoex-04, renamed to use the gssspi_ prefix.  Since
 * the only caller of these functions is SPNEGO, argument validation is
 * omitted.
 */
#[no_mangle]

pub unsafe extern "C" fn gssspi_query_meta_data(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut mech_oid: crate::gssapi_h::gss_const_OID,
    mut cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    targ_name: crate::gssapi_h::gss_name_t,
    mut req_flags: crate::gssapi_h::OM_uint32,
    mut meta_data: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut minor: crate::gssapi_h::OM_uint32 = 0;
    let mut ctx = *context_handle as crate::mglueP_h::gss_union_ctx_id_t;
    let mut cred = cred_handle;
    let mut union_name = targ_name;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    let mut selected_mech = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut public_mech = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut internal_cred = 0 as crate::gssapi_h::gss_cred_id_t;
    let mut internal_name = 0 as crate::gssapi_h::gss_name_t;
    let mut imported_name = 0 as crate::gssapi_h::gss_name_t;
    let mut new_ctx = 0 as crate::gssapi_h::gss_ctx_id_t;
    let mut internal_ctx = 0 as *mut crate::gssapi_h::gss_ctx_id_t;
    *minor_status = 0u32;
    (*meta_data).length = 0usize;
    (*meta_data).value = 0 as *mut libc::c_void;
    status = crate::src::mechglue::g_initialize::gssint_select_mech_type(
        minor_status,
        mech_oid,
        &mut selected_mech,
    );
    if status != 0u32 {
        return status;
    }
    public_mech = crate::src::mechglue::g_initialize::gssint_get_public_oid(
        selected_mech as crate::gssapi_h::gss_const_OID,
    );
    mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
        selected_mech as crate::gssapi_h::gss_const_OID,
    );
    if mech.is_null() {
        return (1u32) << 16i32;
    }
    if (*mech).gssspi_query_meta_data.is_none() {
        return (16u32) << 16i32;
    }
    if !cred.is_null() {
        internal_cred =
            crate::src::mechglue::g_glue::gssint_get_mechanism_cred(cred, selected_mech);
        if internal_cred.is_null() {
            return (7u32) << 16i32;
        }
    }
    if !union_name.is_null() {
        if !(*union_name).mech_type.is_null()
            && ((*(*union_name).mech_type).length == (*selected_mech).length
                && crate::stdlib::memcmp(
                    (*(*union_name).mech_type).elements,
                    (*selected_mech).elements,
                    (*(*union_name).mech_type).length as usize,
                ) == 0i32)
        {
            internal_name = (*union_name).mech_name;
            current_block = 15125582407903384992;
        } else {
            status = crate::src::mechglue::g_glue::gssint_import_internal_name(
                minor_status,
                selected_mech,
                union_name,
                &mut imported_name,
            );
            if status != 0u32 {
                current_block = 1135352847978496863;
            } else {
                internal_name = imported_name;
                current_block = 15125582407903384992;
            }
        }
    } else {
        current_block = 15125582407903384992;
    }
    match current_block {
        15125582407903384992 => {
            internal_ctx = if !ctx.is_null() {
                &mut (*ctx).internal_ctx_id
            } else {
                &mut new_ctx
            };
            status = (*mech)
                .gssspi_query_meta_data
                .expect("non-null function pointer")(
                minor_status,
                public_mech as crate::gssapi_h::gss_const_OID,
                internal_cred,
                internal_ctx,
                internal_name,
                req_flags,
                meta_data,
            );
            if status != 0u32 {
                *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                    *minor_status,
                    &mut (*mech).mech_type,
                )
            } else if !new_ctx.is_null() {
                if ctx.is_null() {
                } else {
                    crate::stdlib::__assert_fail(b"ctx == NULL\x00" as *const u8 as
                                      *const i8,
                                  b"g_negoex.c\x00" as *const u8 as
                                      *const i8,
                                  102u32,
                                  (*::std::mem::transmute::<&[u8; 135],
                                                            &[i8; 135]>(b"OM_uint32 gssspi_query_meta_data(OM_uint32 *, gss_const_OID, gss_cred_id_t, gss_ctx_id_t *, const gss_name_t, OM_uint32, gss_buffer_t)\x00")).as_ptr());
                }
                status = crate::src::mechglue::g_glue::gssint_create_union_context(
                    minor_status,
                    selected_mech as crate::gssapi_h::gss_const_OID,
                    &mut ctx,
                );
                if !(status != 0u32) {
                    (*ctx).internal_ctx_id = new_ctx;
                    new_ctx = 0 as crate::gssapi_h::gss_ctx_id_t;
                    *context_handle = ctx as crate::gssapi_h::gss_ctx_id_t
                }
            }
        }
        _ => {}
    }
    if !imported_name.is_null() {
        crate::src::mechglue::g_glue::gssint_release_internal_name(
            &mut minor,
            selected_mech,
            &mut imported_name,
        );
    }
    if !new_ctx.is_null() {
        crate::src::mechglue::g_glue::gssint_delete_internal_sec_context(
            &mut minor,
            &mut (*mech).mech_type,
            &mut new_ctx,
            0 as crate::gssapi_h::gss_buffer_t,
        );
    }
    return status;
}
#[no_mangle]

pub unsafe extern "C" fn gssspi_exchange_meta_data(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut mech_oid: crate::gssapi_h::gss_const_OID,
    mut cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    targ_name: crate::gssapi_h::gss_name_t,
    mut req_flags: crate::gssapi_h::OM_uint32,
    mut meta_data: crate::gssapi_h::gss_const_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut minor: crate::gssapi_h::OM_uint32 = 0;
    let mut ctx = *context_handle as crate::mglueP_h::gss_union_ctx_id_t;
    let mut cred = cred_handle;
    let mut union_name = targ_name;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    let mut selected_mech = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut public_mech = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut internal_cred = 0 as crate::gssapi_h::gss_cred_id_t;
    let mut internal_name = 0 as crate::gssapi_h::gss_name_t;
    let mut imported_name = 0 as crate::gssapi_h::gss_name_t;
    let mut new_ctx = 0 as crate::gssapi_h::gss_ctx_id_t;
    let mut internal_ctx = 0 as *mut crate::gssapi_h::gss_ctx_id_t;
    *minor_status = 0u32;
    status = crate::src::mechglue::g_initialize::gssint_select_mech_type(
        minor_status,
        mech_oid,
        &mut selected_mech,
    );
    if status != 0u32 {
        return status;
    }
    public_mech = crate::src::mechglue::g_initialize::gssint_get_public_oid(
        selected_mech as crate::gssapi_h::gss_const_OID,
    );
    mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
        selected_mech as crate::gssapi_h::gss_const_OID,
    );
    if mech.is_null() {
        return (1u32) << 16i32;
    }
    if (*mech).gssspi_exchange_meta_data.is_none() {
        return (16u32) << 16i32;
    }
    if !cred.is_null() {
        internal_cred =
            crate::src::mechglue::g_glue::gssint_get_mechanism_cred(cred, selected_mech);
        if internal_cred.is_null() {
            return (7u32) << 16i32;
        }
    }
    if !union_name.is_null() {
        if !(*union_name).mech_type.is_null()
            && ((*(*union_name).mech_type).length == (*selected_mech).length
                && crate::stdlib::memcmp(
                    (*(*union_name).mech_type).elements,
                    (*selected_mech).elements,
                    (*(*union_name).mech_type).length as usize,
                ) == 0i32)
        {
            internal_name = (*union_name).mech_name
        } else {
            status = crate::src::mechglue::g_glue::gssint_import_internal_name(
                minor_status,
                selected_mech,
                union_name,
                &mut imported_name,
            );
            if status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
                return status;
            }
            internal_name = imported_name
        }
    }
    internal_ctx = if !ctx.is_null() {
        &mut (*ctx).internal_ctx_id
    } else {
        &mut new_ctx
    };
    status = (*mech)
        .gssspi_exchange_meta_data
        .expect("non-null function pointer")(
        minor_status,
        public_mech as crate::gssapi_h::gss_const_OID,
        internal_cred,
        internal_ctx,
        internal_name,
        req_flags,
        meta_data,
    );
    if status != 0u32 {
        *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
            *minor_status,
            &mut (*mech).mech_type,
        )
    } else if !new_ctx.is_null() {
        if ctx.is_null() {
        } else {
            crate::stdlib::__assert_fail(b"ctx == NULL\x00" as *const u8 as
                              *const i8,
                          b"g_negoex.c\x00" as *const u8 as
                              *const i8,
                          186u32,
                          (*::std::mem::transmute::<&[u8; 144],
                                                    &[i8; 144]>(b"OM_uint32 gssspi_exchange_meta_data(OM_uint32 *, gss_const_OID, gss_cred_id_t, gss_ctx_id_t *, const gss_name_t, OM_uint32, gss_const_buffer_t)\x00")).as_ptr());
        }
        status = crate::src::mechglue::g_glue::gssint_create_union_context(
            minor_status,
            selected_mech as crate::gssapi_h::gss_const_OID,
            &mut ctx,
        );
        if !(status != 0u32) {
            (*ctx).internal_ctx_id = new_ctx;
            new_ctx = 0 as crate::gssapi_h::gss_ctx_id_t;
            *context_handle = ctx as crate::gssapi_h::gss_ctx_id_t
        }
    }
    if !imported_name.is_null() {
        crate::src::mechglue::g_glue::gssint_release_internal_name(
            &mut minor,
            selected_mech,
            &mut imported_name,
        );
    }
    if !new_ctx.is_null() {
        crate::src::mechglue::g_glue::gssint_delete_internal_sec_context(
            &mut minor,
            &mut (*mech).mech_type,
            &mut new_ctx,
            0 as crate::gssapi_h::gss_buffer_t,
        );
    }
    return status;
}
/* If the mech created a context, wrap it in a union context. */
/* If the mech created a context, wrap it in a union context. */
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
/* minor_status */
/* input_cred_handle */
/* input_usage */
/* desired_mech */
/* overwrite_cred */
/* default_cred */
/* cred_store */
/* elements_stored */
/* cred_usage_stored */
/*
 * A mech can make itself negotiable via NegoEx (draft-zhu-negoex) by
 * implementing the following three SPIs, and also implementing
 * gss_inquire_sec_context_by_oid() and answering the GSS_C_INQ_NEGOEX_KEY and
 * GSS_C_INQ_NEGOEX_VERIFY_KEY OIDs.  The answer must be in two buffers: the
 * first contains the key contents, and the second contains the key enctype as
 * a four-byte little-endian integer.
 *
 * By default, NegoEx mechanisms will not be directly negotiated via SPNEGO.
 * If direct SPNEGO negotiation is required for interoperability, implement
 * gss_inquire_attrs_for_mech() and assert the GSS_C_MA_NEGOEX_AND_SPNEGO
 * attribute (along with any applicable RFC 5587 attributes).
 */
#[no_mangle]

pub unsafe extern "C" fn gssspi_query_mechanism_info(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut mech_oid: crate::gssapi_h::gss_const_OID,
    mut auth_scheme: *mut u8,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut selected_mech = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut public_mech = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    *minor_status = 0u32;
    crate::stdlib::memset(auth_scheme as *mut libc::c_void, 0i32, 16usize);
    status = crate::src::mechglue::g_initialize::gssint_select_mech_type(
        minor_status,
        mech_oid,
        &mut selected_mech,
    );
    if status != 0u32 {
        return status;
    }
    public_mech = crate::src::mechglue::g_initialize::gssint_get_public_oid(
        selected_mech as crate::gssapi_h::gss_const_OID,
    );
    mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
        selected_mech as crate::gssapi_h::gss_const_OID,
    );
    if mech.is_null() {
        return (1u32) << 16i32;
    }
    if (*mech).gssspi_query_mechanism_info.is_none() {
        return (16u32) << 16i32;
    }
    status = (*mech)
        .gssspi_query_mechanism_info
        .expect("non-null function pointer")(
        minor_status,
        public_mech as crate::gssapi_h::gss_const_OID,
        auth_scheme,
    );
    if status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
        *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
            *minor_status,
            &mut (*mech).mech_type,
        )
    }
    return status;
}
