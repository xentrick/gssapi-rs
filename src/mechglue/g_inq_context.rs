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

pub use crate::src::mechglue::g_glue::gssint_convert_name_to_union_name;
pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;
pub use crate::src::mechglue::g_initialize::gssint_get_public_oid;
pub use crate::src::mechglue::g_rel_name::gss_release_name;
/* #pragma ident	"@(#)g_inquire_context.c	1.15	04/02/23 SMI" */
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
 *  glue routine for gss_inquire_context
 */

unsafe extern "C" fn val_inq_ctx_args(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut src_name: *mut crate::gssapi_h::gss_name_t,
    mut targ_name: *mut crate::gssapi_h::gss_name_t,
    mut _lifetime_rec: *mut crate::gssapi_h::OM_uint32,
    mut mech_type: *mut crate::gssapi_h::gss_OID,
    mut _ctx_flags: *mut crate::gssapi_h::OM_uint32,
    mut _locally_initiated: *mut i32,
    mut _opened: *mut i32,
) -> crate::gssapi_h::OM_uint32 {
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !src_name.is_null() {
        *src_name = 0 as crate::gssapi_h::gss_name_t
    }
    if !targ_name.is_null() {
        *targ_name = 0 as crate::gssapi_h::gss_name_t
    }
    if !mech_type.is_null() {
        *mech_type = 0 as crate::gssapi_h::gss_OID
    }
    /* Validate arguments. */
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    if context_handle.is_null() {
        return (1u32) << 24i32 | (8u32) << 16i32;
    }
    return 0u32;
}
/* Last argument new for V2 */
#[no_mangle]

pub unsafe extern "C" fn gss_inquire_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut src_name: *mut crate::gssapi_h::gss_name_t,
    mut targ_name: *mut crate::gssapi_h::gss_name_t,
    mut lifetime_rec: *mut crate::gssapi_h::OM_uint32,
    mut mech_type: *mut crate::gssapi_h::gss_OID,
    mut ctx_flags: *mut crate::gssapi_h::OM_uint32,
    mut locally_initiated: *mut i32,
    mut opened: *mut i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = 0 as *mut crate::mglueP_h::gss_union_ctx_id_struct;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut temp_minor: crate::gssapi_h::OM_uint32 = 0;
    let mut actual_mech = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut localTargName = 0 as crate::gssapi_h::gss_name_t;
    let mut localSourceName = 0 as crate::gssapi_h::gss_name_t;
    status = val_inq_ctx_args(
        minor_status,
        context_handle,
        src_name,
        targ_name,
        lifetime_rec,
        mech_type,
        ctx_flags,
        locally_initiated,
        opened,
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
    if mech.is_null()
        || (*mech).gss_inquire_context.is_none()
        || (*mech).gss_display_name.is_none()
        || (*mech).gss_release_name.is_none()
    {
        return (16u32) << 16i32;
    }
    status = (*mech)
        .gss_inquire_context
        .expect("non-null function pointer")(
        minor_status,
        (*ctx).internal_ctx_id,
        if !src_name.is_null() {
            &mut localSourceName
        } else {
            0 as *mut crate::gssapi_h::gss_name_t
        },
        if !targ_name.is_null() {
            &mut localTargName
        } else {
            0 as *mut crate::gssapi_h::gss_name_t
        },
        lifetime_rec,
        &mut actual_mech,
        ctx_flags,
        locally_initiated,
        opened,
    );
    if status != 0u32 {
        *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
            *minor_status,
            &mut (*mech).mech_type,
        );
        return status;
    }
    /* need to convert names */
    if !src_name.is_null() {
        if !localSourceName.is_null() {
            status = crate::src::mechglue::g_glue::gssint_convert_name_to_union_name(
                minor_status,
                mech,
                localSourceName,
                src_name,
            );
            if status != 0u32 {
                if !localTargName.is_null() {
                    (*mech).gss_release_name.expect("non-null function pointer")(
                        &mut temp_minor,
                        &mut localTargName,
                    );
                }
                return status;
            }
        } else {
            *src_name = 0 as crate::gssapi_h::gss_name_t
        }
    }
    if !targ_name.is_null() {
        if !localTargName.is_null() {
            status = crate::src::mechglue::g_glue::gssint_convert_name_to_union_name(
                minor_status,
                mech,
                localTargName,
                targ_name,
            );
            if status != 0u32 {
                if !src_name.is_null() {
                    crate::src::mechglue::g_rel_name::gss_release_name(&mut temp_minor, src_name);
                }
                return status;
            }
        } else {
            *targ_name = 0 as crate::gssapi_h::gss_name_t
        }
    }
    if !mech_type.is_null() {
        *mech_type = crate::src::mechglue::g_initialize::gssint_get_public_oid(
            actual_mech as crate::gssapi_h::gss_const_OID,
        )
    }
    return 0u32;
}
