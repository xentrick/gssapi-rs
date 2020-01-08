use ::libc;

pub use crate::stddef_h::size_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::uint32_t;

pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_ctx_id_struct;
pub use crate::gssapi_h::gss_ctx_id_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::mglueP_h::gss_union_ctx_id_struct;
pub use crate::mglueP_h::gss_union_ctx_id_t;
pub use crate::src::mechglue::g_glue::gssint_delete_internal_sec_context;

/* #pragma ident	"@(#)g_delete_sec_context.c	1.11	97/11/09 SMI" */
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
 *  glue routine for gss_delete_sec_context
 */

unsafe extern "C" fn val_del_sec_ctx_args(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    mut output_token: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !output_token.is_null() {
        (*output_token).length = 0usize;
        (*output_token).value = 0 as *mut libc::c_void
    }
    /* Validate arguments. */
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    if context_handle.is_null() || (*context_handle).is_null() {
        return (2u32) << 24i32 | (8u32) << 16i32;
    }
    return 0u32;
}
#[no_mangle]

pub unsafe extern "C" fn gss_delete_sec_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    mut output_token: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut ctx = 0 as *mut crate::mglueP_h::gss_union_ctx_id_struct;
    status = val_del_sec_ctx_args(minor_status, context_handle, output_token);
    if status != 0u32 {
        return status;
    }
    /*
     * select the approprate underlying mechanism routine and
     * call it.
     */
    ctx = *context_handle as crate::mglueP_h::gss_union_ctx_id_t;
    if !(!ctx.is_null() && (*ctx).loopback == ctx) {
        return (1u32) << 24i32 | (8u32) << 16i32;
    }
    if !(*ctx).internal_ctx_id.is_null() {
        status = crate::src::mechglue::g_glue::gssint_delete_internal_sec_context(
            minor_status,
            (*ctx).mech_type,
            &mut (*ctx).internal_ctx_id,
            output_token,
        );
        if status != 0 {
            return status;
        }
    }
    /* now free up the space for the union context structure */
    crate::stdlib::free((*(*ctx).mech_type).elements);
    crate::stdlib::free((*ctx).mech_type as *mut libc::c_void);
    crate::stdlib::free(*context_handle as *mut libc::c_void);
    *context_handle = 0 as crate::gssapi_h::gss_ctx_id_t;
    return 0u32;
}
