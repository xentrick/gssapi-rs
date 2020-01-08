use ::libc;

pub mod gssapi_alloc_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
    /* To the extent possible under law, Painless Security, LLC has waived
     * all copyright and related or neighboring rights to GSS-API Memory
     * Management Header. This work is published from: United States.
     */
    /* not _WIN32 or DEBUG_GSSALLOC */
    /* Normal Unix case, just use free/malloc/calloc/realloc. */
    #[inline]

    pub unsafe extern "C" fn gssalloc_malloc(
        mut size: crate::stddef_h::size_t,
    ) -> *mut libc::c_void {
        return crate::stdlib::malloc(size);
    }
}

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
pub use crate::src::mechglue::g_rel_buffer::gss_release_buffer;
pub use crate::src::mechglue::g_wrap_aead::gssapi_alloc_h::gssalloc_malloc;

/* #pragma ident	"@(#)g_seal.c	1.19	98/04/21 SMI" */
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
 *  glue routine for gss_wrap_aead
 */

unsafe extern "C" fn val_wrap_aead_args(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut _conf_req_flag: i32,
    mut _qop_req: crate::gssapi_h::gss_qop_t,
    mut _input_assoc_buffer: crate::gssapi_h::gss_buffer_t,
    mut input_payload_buffer: crate::gssapi_h::gss_buffer_t,
    mut _conf_state: *mut i32,
    mut output_message_buffer: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    /* Validate arguments. */
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    if context_handle.is_null() {
        return (1u32) << 24i32 | (8u32) << 16i32;
    }
    if input_payload_buffer.is_null() {
        return (1u32) << 24i32;
    }
    if output_message_buffer.is_null() {
        return (2u32) << 24i32;
    }
    return 0u32;
}

unsafe extern "C" fn gssint_wrap_aead_iov_shim(
    mut mech: crate::mglueP_h::gss_mechanism,
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_req_flag: i32,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut input_assoc_buffer: crate::gssapi_h::gss_buffer_t,
    mut input_payload_buffer: crate::gssapi_h::gss_buffer_t,
    mut conf_state: *mut i32,
    mut output_message_buffer: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut iov: [crate::gssapi_ext_h::gss_iov_buffer_desc; 5] =
        [crate::gssapi_ext_h::gss_iov_buffer_desc {
            type_0: 0,
            buffer: crate::gssapi_h::gss_buffer_desc {
                length: 0,
                value: 0 as *mut libc::c_void,
            },
        }; 5];
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut offset: crate::stddef_h::size_t = 0;
    let mut i = 0i32;
    let mut iov_count: i32 = 0;
    /* HEADER | SIGN_ONLY_DATA | DATA | PADDING | TRAILER */
    iov[i as usize].type_0 = 2u32;
    iov[i as usize].buffer.value = 0 as *mut libc::c_void;
    iov[i as usize].buffer.length = 0usize;
    i += 1;
    if !input_assoc_buffer.is_null() {
        iov[i as usize].type_0 = 11u32;
        iov[i as usize].buffer = *input_assoc_buffer;
        i += 1
    }
    iov[i as usize].type_0 = 1u32;
    iov[i as usize].buffer = *input_payload_buffer;
    i += 1;
    iov[i as usize].type_0 = 9u32;
    iov[i as usize].buffer.value = 0 as *mut libc::c_void;
    iov[i as usize].buffer.length = 0usize;
    i += 1;
    iov[i as usize].type_0 = 7u32;
    iov[i as usize].buffer.value = 0 as *mut libc::c_void;
    iov[i as usize].buffer.length = 0usize;
    i += 1;
    iov_count = i;
    if (*mech).gss_wrap_iov_length.is_some() {
    } else {
        crate::stdlib::__assert_fail(b"mech->gss_wrap_iov_length\x00" as *const u8 as
                          *const i8,
                      b"g_wrap_aead.c\x00" as *const u8 as
                          *const i8,
                      110u32,
                      (*::std::mem::transmute::<&[u8; 143],
                                                &[i8; 143]>(b"OM_uint32 gssint_wrap_aead_iov_shim(gss_mechanism, OM_uint32 *, gss_ctx_id_t, int, gss_qop_t, gss_buffer_t, gss_buffer_t, int *, gss_buffer_t)\x00")).as_ptr());
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
        iov_count,
    );
    if status != 0u32 {
        *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
            *minor_status,
            &mut (*mech).mech_type,
        );
        return status;
    }
    /* Format output token (does not include associated data) */
    i = 0i32;
    (*output_message_buffer).length = 0usize;
    while i < iov_count {
        if !(iov[i as usize].type_0 & !(0xffff0000u32) == 11u32) {
            (*output_message_buffer).length =
                ((*output_message_buffer).length).wrapping_add(iov[i as usize].buffer.length)
        }
        i += 1
    }
    (*output_message_buffer).value = gssalloc_malloc((*output_message_buffer).length);
    if (*output_message_buffer).value.is_null() {
        *minor_status = 12u32;
        return (13u32) << 16i32;
    }
    i = 0i32;
    offset = 0usize;
    /* HEADER */
    iov[i as usize].buffer.value =
        ((*output_message_buffer).value as *mut u8).offset(offset as isize) as *mut libc::c_void;
    offset = (offset).wrapping_add(iov[i as usize].buffer.length);
    i += 1;
    /* SIGN_ONLY_DATA */
    if !input_assoc_buffer.is_null() {
        i += 1
    }
    /* DATA */
    iov[i as usize].buffer.value =
        ((*output_message_buffer).value as *mut u8).offset(offset as isize) as *mut libc::c_void;
    offset = (offset).wrapping_add(iov[i as usize].buffer.length);
    crate::stdlib::memcpy(
        iov[i as usize].buffer.value,
        (*input_payload_buffer).value,
        iov[i as usize].buffer.length,
    );
    i += 1;
    /* PADDING */
    iov[i as usize].buffer.value =
        ((*output_message_buffer).value as *mut u8).offset(offset as isize) as *mut libc::c_void;
    offset = (offset).wrapping_add(iov[i as usize].buffer.length);
    i += 1;
    /* TRAILER */
    iov[i as usize].buffer.value =
        ((*output_message_buffer).value as *mut u8).offset(offset as isize) as *mut libc::c_void;
    offset = (offset).wrapping_add(iov[i as usize].buffer.length);
    i += 1;
    if offset == (*output_message_buffer).length {
    } else {
        crate::stdlib::__assert_fail(b"offset == output_message_buffer->length\x00" as
                          *const u8 as *const i8,
                      b"g_wrap_aead.c\x00" as *const u8 as
                          *const i8,
                      162u32,
                      (*::std::mem::transmute::<&[u8; 143],
                                                &[i8; 143]>(b"OM_uint32 gssint_wrap_aead_iov_shim(gss_mechanism, OM_uint32 *, gss_ctx_id_t, int, gss_qop_t, gss_buffer_t, gss_buffer_t, int *, gss_buffer_t)\x00")).as_ptr());
    }
    if (*mech).gss_wrap_iov.is_some() {
    } else {
        crate::stdlib::__assert_fail(b"mech->gss_wrap_iov\x00" as *const u8 as
                          *const i8,
                      b"g_wrap_aead.c\x00" as *const u8 as
                          *const i8,
                      164u32,
                      (*::std::mem::transmute::<&[u8; 143],
                                                &[i8; 143]>(b"OM_uint32 gssint_wrap_aead_iov_shim(gss_mechanism, OM_uint32 *, gss_ctx_id_t, int, gss_qop_t, gss_buffer_t, gss_buffer_t, int *, gss_buffer_t)\x00")).as_ptr());
    }
    status = (*mech).gss_wrap_iov.expect("non-null function pointer")(
        minor_status,
        context_handle,
        conf_req_flag,
        qop_req,
        conf_state,
        iov.as_mut_ptr(),
        iov_count,
    );
    if status != 0u32 {
        let mut minor: crate::gssapi_h::OM_uint32 = 0;
        *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
            *minor_status,
            &mut (*mech).mech_type,
        );
        crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut minor, output_message_buffer);
    }
    return status;
}
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
/* minor_status */
/* oid set */
/* new oid set */
/* name_type */
/* minor_status */
/* name_type */
/* mech */
/*
 * Sun extensions to GSS-API v2
 */
/* buf */
/* buf_len */
/* bytes */
/* len */
/* length */
/* buf */
/* max_len */
#[no_mangle]

pub unsafe extern "C" fn gssint_wrap_aead(
    mut mech: crate::mglueP_h::gss_mechanism,
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut ctx: crate::mglueP_h::gss_union_ctx_id_t,
    mut conf_req_flag: i32,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut input_assoc_buffer: crate::gssapi_h::gss_buffer_t,
    mut input_payload_buffer: crate::gssapi_h::gss_buffer_t,
    mut conf_state: *mut i32,
    mut output_message_buffer: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    /* EXPORT DELETE START */
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    if !ctx.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"ctx != NULL\x00" as *const u8 as *const i8,
                      b"g_wrap_aead.c\x00" as *const u8 as
                          *const i8,
                      193u32,
                      (*::std::mem::transmute::<&[u8; 140],
                                                &[i8; 140]>(b"OM_uint32 gssint_wrap_aead(gss_mechanism, OM_uint32 *, gss_union_ctx_id_t, int, gss_qop_t, gss_buffer_t, gss_buffer_t, int *, gss_buffer_t)\x00")).as_ptr());
    }
    if !mech.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"mech != NULL\x00" as *const u8 as *const i8,
                      b"g_wrap_aead.c\x00" as *const u8 as
                          *const i8,
                      194u32,
                      (*::std::mem::transmute::<&[u8; 140],
                                                &[i8; 140]>(b"OM_uint32 gssint_wrap_aead(gss_mechanism, OM_uint32 *, gss_union_ctx_id_t, int, gss_qop_t, gss_buffer_t, gss_buffer_t, int *, gss_buffer_t)\x00")).as_ptr());
    }
    if (*mech).gss_wrap_aead.is_some() {
        status = (*mech).gss_wrap_aead.expect("non-null function pointer")(
            minor_status,
            (*ctx).internal_ctx_id,
            conf_req_flag,
            qop_req,
            input_assoc_buffer,
            input_payload_buffer,
            conf_state,
            output_message_buffer,
        );
        if status != 0u32 {
            *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                *minor_status,
                &mut (*mech).mech_type,
            )
        }
    } else if (*mech).gss_wrap_iov.is_some() && (*mech).gss_wrap_iov_length.is_some() {
        status = gssint_wrap_aead_iov_shim(
            mech,
            minor_status,
            (*ctx).internal_ctx_id,
            conf_req_flag,
            qop_req,
            input_assoc_buffer,
            input_payload_buffer,
            conf_state,
            output_message_buffer,
        )
    } else {
        status = (16u32) << 16i32
    }
    /* EXPORT DELETE END */
    return status;
}
/*
 * AEAD extensions
 */
#[no_mangle]

pub unsafe extern "C" fn gss_wrap_aead(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_req_flag: i32,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut input_assoc_buffer: crate::gssapi_h::gss_buffer_t,
    mut input_payload_buffer: crate::gssapi_h::gss_buffer_t,
    mut conf_state: *mut i32,
    mut output_message_buffer: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    let mut ctx = 0 as *mut crate::mglueP_h::gss_union_ctx_id_struct;
    status = val_wrap_aead_args(
        minor_status,
        context_handle,
        conf_req_flag,
        qop_req,
        input_assoc_buffer,
        input_payload_buffer,
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
    if mech.is_null() {
        return (1u32) << 16i32;
    }
    return gssint_wrap_aead(
        mech,
        minor_status,
        ctx,
        conf_req_flag,
        qop_req,
        input_assoc_buffer,
        input_payload_buffer,
        conf_state,
        output_message_buffer,
    );
}
