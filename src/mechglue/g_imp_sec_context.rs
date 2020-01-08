use ::libc;

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

pub use crate::src::mechglue::g_glue::gssint_create_union_context;
pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;
pub use crate::src::mechglue::g_initialize::gssint_get_public_oid;
pub use crate::src::mechglue::g_initialize::gssint_select_mech_type;

/* #pragma ident	"@(#)g_imp_sec_context.c	1.18	04/02/23 SMI" */
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
 *  glue routine gss_export_sec_context
 */

unsafe extern "C" fn val_imp_sec_ctx_args(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut interprocess_token: crate::gssapi_h::gss_buffer_t,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
) -> crate::gssapi_h::OM_uint32 {
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !context_handle.is_null() {
        *context_handle = 0 as crate::gssapi_h::gss_ctx_id_t
    }
    /* Validate arguments. */
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    if context_handle.is_null() {
        return (2u32) << 24i32;
    }
    if interprocess_token.is_null() {
        return (1u32) << 24i32 | (9u32) << 16i32;
    }
    if interprocess_token.is_null()
        || (*interprocess_token).value.is_null()
        || (*interprocess_token).length == 0usize
    {
        return (1u32) << 24i32 | (9u32) << 16i32;
    }
    return 0u32;
}
/* interprocess_token */
/* New for V2 */
#[no_mangle]

pub unsafe extern "C" fn gss_import_sec_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut interprocess_token: crate::gssapi_h::gss_buffer_t,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut length = 0u32;
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut p = 0 as *mut i8;
    let mut ctx = 0 as *mut crate::mglueP_h::gss_union_ctx_id_struct;
    let mut mctx = 0 as *mut crate::gssapi_h::gss_ctx_id_struct;
    let mut token = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    let mut token_mech = crate::gssapi_h::gss_OID_desc {
        length: 0,
        elements: 0 as *mut libc::c_void,
    };
    let mut selected_mech = 0 as crate::gssapi_h::gss_OID;
    let mut public_mech = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    status = val_imp_sec_ctx_args(minor_status, interprocess_token, context_handle);
    if status != 0u32 {
        return status;
    }
    /* Initial value needed below. */
    status = (13u32) << 16i32;
    if (*interprocess_token).length >= ::std::mem::size_of::<crate::gssapi_h::OM_uint32>() {
        p = (*interprocess_token).value as *mut i8;
        let fresh0 = p;
        p = p.offset(1);
        length = *fresh0 as crate::gssapi_h::OM_uint32;
        let fresh1 = p;
        p = p.offset(1);
        length = (length << 8i32).wrapping_add(*fresh1 as u32);
        let fresh2 = p;
        p = p.offset(1);
        length = (length << 8i32).wrapping_add(*fresh2 as u32);
        let fresh3 = p;
        p = p.offset(1);
        length = (length << 8i32).wrapping_add(*fresh3 as u32)
    }
    if length == 0u32
        || length as usize
            > (*interprocess_token)
                .length
                .wrapping_sub(::std::mem::size_of::<crate::gssapi_h::OM_uint32>())
    {
        return (3u32) << 24i32 | (9u32) << 16i32;
    }
    token_mech.length = length;
    token_mech.elements = p as *mut libc::c_void;
    p = p.offset(length as isize);
    token.length = (*interprocess_token)
        .length
        .wrapping_sub(::std::mem::size_of::<crate::gssapi_h::OM_uint32>())
        .wrapping_sub(length as usize);
    token.value = p as *mut libc::c_void;
    /*
     * select the approprate underlying mechanism routine and
     * call it.
     */
    status = crate::src::mechglue::g_initialize::gssint_select_mech_type(
        minor_status,
        &mut token_mech as *mut crate::gssapi_h::gss_OID_desc as crate::gssapi_h::gss_const_OID,
        &mut selected_mech,
    );
    if status != 0u32 {
        return status;
    }
    mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
        selected_mech as crate::gssapi_h::gss_const_OID,
    );
    if mech.is_null() {
        return (1u32) << 16i32;
    }
    if (*mech).gssspi_import_sec_context_by_mech.is_none()
        && (*mech).gss_import_sec_context.is_none()
    {
        return (16u32) << 16i32;
    }
    status = crate::src::mechglue::g_glue::gssint_create_union_context(
        minor_status,
        selected_mech as crate::gssapi_h::gss_const_OID,
        &mut ctx,
    );
    if status != 0u32 {
        return status;
    }
    if (*mech).gssspi_import_sec_context_by_mech.is_some() {
        public_mech = crate::src::mechglue::g_initialize::gssint_get_public_oid(
            selected_mech as crate::gssapi_h::gss_const_OID,
        );
        status = (*mech)
            .gssspi_import_sec_context_by_mech
            .expect("non-null function pointer")(
            minor_status, public_mech, &mut token, &mut mctx
        )
    } else {
        status =
            (*mech)
                .gss_import_sec_context
                .expect("non-null function pointer")(minor_status, &mut token, &mut mctx)
    }
    if status == 0u32 {
        (*ctx).internal_ctx_id = mctx;
        *context_handle = ctx as crate::gssapi_h::gss_ctx_id_t;
        return 0u32;
    }
    *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
        *minor_status,
        &mut (*mech).mech_type,
    );
    crate::stdlib::free((*(*ctx).mech_type).elements);
    crate::stdlib::free((*ctx).mech_type as *mut libc::c_void);
    crate::stdlib::free(ctx as *mut libc::c_void);
    return status;
}
/* LEAN_CLIENT */
