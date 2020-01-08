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
pub use crate::mglueP_h::gss_union_ctx_id_struct;
pub use crate::mglueP_h::gss_union_ctx_id_t;

pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;
pub use crate::src::mechglue::g_wrap_aead::gssint_wrap_aead;

/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
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
 *  glue routine for gss_wrap
 */

unsafe extern "C" fn val_wrap_args(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut _conf_req_flag: i32,
    mut _qop_req: crate::gssapi_h::gss_qop_t,
    mut input_message_buffer: crate::gssapi_h::gss_buffer_t,
    mut _conf_state: *mut i32,
    mut output_message_buffer: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !output_message_buffer.is_null() {
        (*output_message_buffer).length = 0usize;
        (*output_message_buffer).value = 0 as *mut libc::c_void
    }
    /* Validate arguments. */
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    if context_handle.is_null() {
        return (1u32) << 24i32 | (8u32) << 16i32;
    }
    if input_message_buffer.is_null() {
        return (1u32) << 24i32;
    }
    if output_message_buffer.is_null() {
        return (2u32) << 24i32;
    }
    return 0u32;
}
#[no_mangle]

pub unsafe extern "C" fn gss_wrap(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_req_flag: i32,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut input_message_buffer: crate::gssapi_h::gss_buffer_t,
    mut conf_state: *mut i32,
    mut output_message_buffer: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    /* EXPORT DELETE START */
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut ctx = 0 as *mut crate::mglueP_h::gss_union_ctx_id_struct;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    status = val_wrap_args(
        minor_status,
        context_handle,
        conf_req_flag,
        qop_req,
        input_message_buffer,
        conf_state,
        output_message_buffer,
    );
    if status != 0u32 {
        return status;
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
        if (*mech).gss_wrap.is_some() {
            status = (*mech).gss_wrap.expect("non-null function pointer")(
                minor_status,
                (*ctx).internal_ctx_id,
                conf_req_flag,
                qop_req,
                input_message_buffer,
                conf_state,
                output_message_buffer,
            );
            if status != 0u32 {
                *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                    *minor_status,
                    &mut (*mech).mech_type,
                )
            }
        } else if (*mech).gss_wrap_aead.is_some()
            || (*mech).gss_wrap_iov.is_some() && (*mech).gss_wrap_iov_length.is_some()
        {
            status = crate::src::mechglue::g_wrap_aead::gssint_wrap_aead(
                mech,
                minor_status,
                ctx,
                conf_req_flag,
                qop_req,
                0 as crate::gssapi_h::gss_buffer_t,
                input_message_buffer,
                conf_state,
                output_message_buffer,
            )
        } else {
            status = (16u32) << 16i32
        }
        return status;
    }
    /* EXPORT DELETE END */
    return (1u32) << 16i32;
}
#[no_mangle]

pub unsafe extern "C" fn gss_seal(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_req_flag: i32,
    mut qop_req: i32,
    mut input_message_buffer: crate::gssapi_h::gss_buffer_t,
    mut conf_state: *mut i32,
    mut output_message_buffer: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    return gss_wrap(
        minor_status,
        context_handle,
        conf_req_flag,
        qop_req as crate::gssapi_h::gss_qop_t,
        input_message_buffer,
        conf_state,
        output_message_buffer,
    );
}
/*
 * It is only possible to implement gss_wrap_size_limit() on top
 * of gss_wrap_iov_length() for mechanisms that do not use any
 * padding and have fixed length headers/trailers.
 */

unsafe extern "C" fn gssint_wrap_size_limit_iov_shim(
    mut mech: crate::mglueP_h::gss_mechanism,
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_req_flag: i32,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut req_output_size: crate::gssapi_h::OM_uint32,
    mut max_input_size: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut iov: [crate::gssapi_ext_h::gss_iov_buffer_desc; 4] =
        [crate::gssapi_ext_h::gss_iov_buffer_desc {
            type_0: 0,
            buffer: crate::gssapi_h::gss_buffer_desc {
                length: 0,
                value: 0 as *mut libc::c_void,
            },
        }; 4];
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut ohlen: crate::gssapi_h::OM_uint32 = 0;
    iov[0usize].type_0 = 2u32;
    iov[0usize].buffer.value = 0 as *mut libc::c_void;
    iov[0usize].buffer.length = 0usize;
    iov[1usize].type_0 = 1u32;
    iov[1usize].buffer.length = req_output_size as crate::stddef_h::size_t;
    iov[1usize].buffer.value = 0 as *mut libc::c_void;
    iov[2usize].type_0 = 9u32;
    iov[2usize].buffer.value = 0 as *mut libc::c_void;
    iov[2usize].buffer.length = 0usize;
    iov[3usize].type_0 = 7u32;
    iov[3usize].buffer.value = 0 as *mut libc::c_void;
    iov[3usize].buffer.length = 0usize;
    if (*mech).gss_wrap_iov_length.is_some() {
    } else {
        crate::stdlib::__assert_fail(b"mech->gss_wrap_iov_length\x00" as *const u8 as
                          *const i8,
                      b"g_seal.c\x00" as *const u8 as *const i8,
                      181u32,
                      (*::std::mem::transmute::<&[u8; 124],
                                                &[i8; 124]>(b"OM_uint32 gssint_wrap_size_limit_iov_shim(gss_mechanism, OM_uint32 *, gss_ctx_id_t, int, gss_qop_t, OM_uint32, OM_uint32 *)\x00")).as_ptr());
    }
    status = (*mech)
        .gss_wrap_iov_length
        .expect("non-null function pointer")(
        minor_status,
        context_handle,
        conf_req_flag,
        qop_req,
        0 as *mut i32,
        iov.as_mut_ptr(),
        (::std::mem::size_of::<[crate::gssapi_ext_h::gss_iov_buffer_desc; 4]>()).wrapping_div(
            ::std::mem::size_of::<crate::gssapi_ext_h::gss_iov_buffer_desc>(),
        ) as i32,
    );
    if status != 0u32 {
        *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
            *minor_status,
            &mut (*mech).mech_type,
        );
        return status;
    }
    ohlen = iov[0usize]
        .buffer
        .length
        .wrapping_add(iov[3usize].buffer.length) as crate::gssapi_h::OM_uint32;
    if iov[2usize].buffer.length == 0usize && ohlen < req_output_size {
        *max_input_size = req_output_size.wrapping_sub(ohlen)
    } else {
        *max_input_size = 0u32
    }
    return 0u32;
}
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
/*
 * New for V2
 */
#[no_mangle]

pub unsafe extern "C" fn gss_wrap_size_limit(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_req_flag: i32,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut req_output_size: crate::gssapi_h::OM_uint32,
    mut max_input_size: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = 0 as *mut crate::mglueP_h::gss_union_ctx_id_struct;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    *minor_status = 0u32;
    if context_handle.is_null() {
        return (1u32) << 24i32 | (8u32) << 16i32;
    }
    if max_input_size.is_null() {
        return (2u32) << 24i32;
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
    if mech.is_null() {
        return (1u32) << 16i32;
    }
    if (*mech).gss_wrap_size_limit.is_some() {
        major_status = (*mech)
            .gss_wrap_size_limit
            .expect("non-null function pointer")(
            minor_status,
            (*ctx).internal_ctx_id,
            conf_req_flag,
            qop_req,
            req_output_size,
            max_input_size,
        )
    } else if (*mech).gss_wrap_iov_length.is_some() {
        major_status = gssint_wrap_size_limit_iov_shim(
            mech,
            minor_status,
            (*ctx).internal_ctx_id,
            conf_req_flag,
            qop_req,
            req_output_size,
            max_input_size,
        )
    } else {
        major_status = (16u32) << 16i32
    }
    if major_status != 0u32 {
        *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
            *minor_status,
            &mut (*mech).mech_type,
        )
    }
    return major_status;
}
