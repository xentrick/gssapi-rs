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
pub use crate::mglueP_h::gss_union_ctx_id_struct;
pub use crate::mglueP_h::gss_union_ctx_id_t;

pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;
/* #pragma ident	"@(#)g_verify.c	1.13	98/04/23 SMI" */
/*
 * Copyright 1996 by Sun Microsystems, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of Sun Microsystems not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. Sun Microsystems makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * SUN MICROSYSTEMS DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL SUN MICROSYSTEMS BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
/*
 *  glue routine for gss_verify_mic
 */
#[no_mangle]

pub unsafe extern "C" fn gss_verify_mic(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut message_buffer: crate::gssapi_h::gss_buffer_t,
    mut token_buffer: crate::gssapi_h::gss_buffer_t,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut ctx = 0 as *mut crate::mglueP_h::gss_union_ctx_id_struct;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    *minor_status = 0u32;
    if context_handle.is_null() {
        return (1u32) << 24i32 | (8u32) << 16i32;
    }
    if message_buffer.is_null()
        || (token_buffer.is_null()
            || (*token_buffer).value.is_null()
            || (*token_buffer).length == 0usize)
    {
        return (1u32) << 24i32;
    }
    /*
     * select the approprate underlying mechanism routine and
     * call it.
     */
    ctx = context_handle as crate::mglueP_h::gss_union_ctx_id_t;
    if (*ctx).internal_ctx_id.is_null() {
        return (8u32) << 16i32;
    }
    mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
        (*ctx).mech_type as crate::gssapi_h::gss_const_OID,
    );
    if !mech.is_null() {
        if (*mech).gss_verify_mic.is_some() {
            status = (*mech).gss_verify_mic.expect("non-null function pointer")(
                minor_status,
                (*ctx).internal_ctx_id,
                message_buffer,
                token_buffer,
                qop_state,
            );
            if status != 0u32 {
                *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                    *minor_status,
                    &mut (*mech).mech_type,
                )
            }
        } else {
            status = (16u32) << 16i32
        }
        return status;
    }
    return (1u32) << 16i32;
}
/* message_token */
/* New for V2 */
/* minor_status */
/* context_handle */
/* message_buffer */
/* message_token */
/* qop_state */
/* New for V2 */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* input_message_buffer */
/* conf_state */
/* output_message_buffer */
/* New for V2 */
/* minor_status */
/* context_handle */
/* input_message_buffer */
/* output_message_buffer */
/* conf_state */
/* qop_state */
/* minor_status */
/* status_value */
/* status_type */
/* mech_type (used to be const) */
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
/* input_name_type(used to be const) */
/* output_name */
/* minor_status */
/* input_name */
/* minor_status */
/* buffer */
/* minor_status */
/* set */
/* minor_status */
/* cred_handle */
/* name */
/* lifetime */
/* cred_usage */
/* mechanisms */
/* Last argument new for V2 */
/* minor_status */
/* context_handle */
/* src_name */
/* targ_name */
/* lifetime_rec */
/* mech_type */
/* ctx_flags */
/* locally_initiated */
/* open */
/* New for V2 */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* req_output_size */
/* max_input_size */
/* New for V2 */
/* minor_status */
/* input_name */
/* input_name_type */
/* output_name */
/* New for V2 */
/* minor_status */
/* input_name */
/* desired_name_type */
/* output_name */
/* New for V2 */
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
/* New for V2 */
/* minor_status */
/* cred_handle */
/* mech_type */
/* name */
/* initiator_lifetime */
/* acceptor_lifetime */
/* cred_usage */
/* New for V2 */
/* minor_status */
/* context_handle */
/* interprocess_token */
/* New for V2 */
/* minor_status */
/* interprocess_token */
/* context_handle */
/* New for V2 */
/* minor_status */
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
/* minor_status */
/* oid_str */
/* oid */
/* New for V2 */
/* minor_status */
/* oid */
/* oid_str */
/* New for V2 */
/* minor_status */
/* mechanism */
/* name_types */
/* New for V2 */
/* minor_status */
/* input_name */
/* mech_types */
/*
 * The following routines are obsolete variants of gss_get_mic, gss_wrap,
 * gss_verify_mic and gss_unwrap.  They should be provided by GSSAPI V2
 * implementations for backwards compatibility with V1 applications.  Distinct
 * entrypoints (as opposed to #defines) should be provided, to allow GSSAPI
 * V1 applications to link against GSSAPI V2 implementations.
 */
/* minor_status */
/* context_handle */
/* qop_req */
/* message_buffer */
/* message_token */
#[no_mangle]

pub unsafe extern "C" fn gss_verify(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut message_buffer: crate::gssapi_h::gss_buffer_t,
    mut token_buffer: crate::gssapi_h::gss_buffer_t,
    mut qop_state: *mut i32,
) -> crate::gssapi_h::OM_uint32 {
    return gss_verify_mic(
        minor_status,
        context_handle,
        message_buffer,
        token_buffer,
        qop_state as *mut crate::gssapi_h::gss_qop_t,
    );
}
