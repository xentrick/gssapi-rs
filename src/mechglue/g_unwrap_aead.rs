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
pub use crate::src::mechglue::g_rel_buffer::gss_release_buffer;

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
 *  glue routine for gss_unwrap_aead
 */

unsafe extern "C" fn val_unwrap_aead_args(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut input_message_buffer: crate::gssapi_h::gss_buffer_t,
    mut _input_assoc_buffer: crate::gssapi_h::gss_buffer_t,
    mut output_payload_buffer: crate::gssapi_h::gss_buffer_t,
    mut _conf_state: *mut i32,
    mut _qop_state: *mut crate::gssapi_h::gss_qop_t,
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
    if input_message_buffer.is_null() {
        return (1u32) << 24i32;
    }
    if output_payload_buffer.is_null() {
        return (2u32) << 24i32;
    }
    return 0u32;
}

unsafe extern "C" fn gssint_unwrap_aead_iov_shim(
    mut mech: crate::mglueP_h::gss_mechanism,
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut input_message_buffer: crate::gssapi_h::gss_buffer_t,
    mut input_assoc_buffer: crate::gssapi_h::gss_buffer_t,
    mut output_payload_buffer: crate::gssapi_h::gss_buffer_t,
    mut conf_state: *mut i32,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut iov: [crate::gssapi_ext_h::gss_iov_buffer_desc; 3] =
        [crate::gssapi_ext_h::gss_iov_buffer_desc {
            type_0: 0,
            buffer: crate::gssapi_h::gss_buffer_desc {
                length: 0,
                value: 0 as *mut libc::c_void,
            },
        }; 3];
    let mut i = 0i32;
    iov[i as usize].type_0 = 10u32;
    iov[i as usize].buffer = *input_message_buffer;
    i += 1;
    if !input_assoc_buffer.is_null() {
        iov[i as usize].type_0 = 11u32;
        iov[i as usize].buffer = *input_assoc_buffer;
        i += 1
    }
    iov[i as usize].type_0 = (1i32 | 0x10000i32) as crate::gssapi_h::OM_uint32;
    iov[i as usize].buffer.value = 0 as *mut libc::c_void;
    iov[i as usize].buffer.length = 0usize;
    i += 1;
    if (*mech).gss_unwrap_iov.is_some() {
    } else {
        crate::stdlib::__assert_fail(b"mech->gss_unwrap_iov\x00" as *const u8 as
                          *const i8,
                      b"g_unwrap_aead.c\x00" as *const u8 as
                          *const i8,
                      93u32,
                      (*::std::mem::transmute::<&[u8; 142],
                                                &[i8; 142]>(b"OM_uint32 gssint_unwrap_aead_iov_shim(gss_mechanism, OM_uint32 *, gss_ctx_id_t, gss_buffer_t, gss_buffer_t, gss_buffer_t, int *, gss_qop_t *)\x00")).as_ptr());
    }
    status = (*mech).gss_unwrap_iov.expect("non-null function pointer")(
        minor_status,
        context_handle,
        conf_state,
        qop_state,
        iov.as_mut_ptr(),
        i,
    );
    if status == 0u32 {
        *output_payload_buffer = iov[(i - 1i32) as usize].buffer
    } else {
        let mut minor: crate::gssapi_h::OM_uint32 = 0;
        *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
            *minor_status,
            &mut (*mech).mech_type,
        );
        if iov[(i - 1i32) as usize].type_0 & 0x20000u32 != 0 {
            crate::src::mechglue::g_rel_buffer::gss_release_buffer(
                &mut minor,
                &mut (*iov.as_mut_ptr().offset((i - 1i32) as isize)).buffer,
            );
            iov[(i - 1i32) as usize].type_0 &= !(0x20000i32) as u32
        }
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
/* mech */
/* minor_status */
/* ctx */
/* conf_req_flag */
/* qop_req_flag */
/* input_assoc_buffer */
/* input_payload_buffer */
/* conf_state */
/* output_message_buffer */
#[no_mangle]

pub unsafe extern "C" fn gssint_unwrap_aead(
    mut mech: crate::mglueP_h::gss_mechanism,
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut ctx: crate::mglueP_h::gss_union_ctx_id_t,
    mut input_message_buffer: crate::gssapi_h::gss_buffer_t,
    mut input_assoc_buffer: crate::gssapi_h::gss_buffer_t,
    mut output_payload_buffer: crate::gssapi_h::gss_buffer_t,
    mut conf_state: *mut i32,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    if !mech.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"mech != NULL\x00" as *const u8 as *const i8,
                      b"g_unwrap_aead.c\x00" as *const u8 as
                          *const i8,
                      125u32,
                      (*::std::mem::transmute::<&[u8; 139],
                                                &[i8; 139]>(b"OM_uint32 gssint_unwrap_aead(gss_mechanism, OM_uint32 *, gss_union_ctx_id_t, gss_buffer_t, gss_buffer_t, gss_buffer_t, int *, gss_qop_t *)\x00")).as_ptr());
    }
    if !ctx.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"ctx != NULL\x00" as *const u8 as *const i8,
                      b"g_unwrap_aead.c\x00" as *const u8 as
                          *const i8,
                      126u32,
                      (*::std::mem::transmute::<&[u8; 139],
                                                &[i8; 139]>(b"OM_uint32 gssint_unwrap_aead(gss_mechanism, OM_uint32 *, gss_union_ctx_id_t, gss_buffer_t, gss_buffer_t, gss_buffer_t, int *, gss_qop_t *)\x00")).as_ptr());
    }
    /* EXPORT DELETE START */
    if (*mech).gss_unwrap_aead.is_some() {
        status = (*mech).gss_unwrap_aead.expect("non-null function pointer")(
            minor_status,
            (*ctx).internal_ctx_id,
            input_message_buffer,
            input_assoc_buffer,
            output_payload_buffer,
            conf_state,
            qop_state,
        );
        if status != 0u32 {
            *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                *minor_status,
                &mut (*mech).mech_type,
            )
        }
    } else if (*mech).gss_unwrap_iov.is_some() {
        status = gssint_unwrap_aead_iov_shim(
            mech,
            minor_status,
            (*ctx).internal_ctx_id,
            input_message_buffer,
            input_assoc_buffer,
            output_payload_buffer,
            conf_state,
            qop_state,
        )
    } else {
        status = (16u32) << 16i32
    }
    /* EXPORT DELETE END */
    return status;
}
#[no_mangle]

pub unsafe extern "C" fn gss_unwrap_aead(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut input_message_buffer: crate::gssapi_h::gss_buffer_t,
    mut input_assoc_buffer: crate::gssapi_h::gss_buffer_t,
    mut output_payload_buffer: crate::gssapi_h::gss_buffer_t,
    mut conf_state: *mut i32,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut ctx = 0 as *mut crate::mglueP_h::gss_union_ctx_id_struct;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    status = val_unwrap_aead_args(
        minor_status,
        context_handle,
        input_message_buffer,
        input_assoc_buffer,
        output_payload_buffer,
        conf_state,
        qop_state,
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
    return gssint_unwrap_aead(
        mech,
        minor_status,
        ctx,
        input_message_buffer,
        input_assoc_buffer,
        output_payload_buffer,
        conf_state,
        qop_state,
    );
}
