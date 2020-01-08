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
pub use crate::src::mechglue::g_glue::gssint_delete_internal_sec_context;
pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;
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
/* Glue routine for gss_set_sec_context_option */
#[no_mangle]

pub unsafe extern "C" fn gss_set_sec_context_option(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    desired_object: crate::gssapi_h::gss_OID,
    value: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut minor: crate::gssapi_h::OM_uint32 = 0;
    let mut ctx = 0 as *mut crate::mglueP_h::gss_union_ctx_id_struct;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    let mut internal_ctx = 0 as crate::gssapi_h::gss_ctx_id_t;
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    *minor_status = 0u32;
    if context_handle.is_null() {
        return (2u32) << 24i32;
    }
    /*
     * select the approprate underlying mechanism routine and
     * call it.
     */
    ctx = *context_handle as crate::mglueP_h::gss_union_ctx_id_t;
    if ctx.is_null() {
        mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
            0 as crate::gssapi_h::gss_const_OID,
        )
    } else {
        mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
            (*ctx).mech_type as crate::gssapi_h::gss_const_OID,
        )
    }
    if mech.is_null() {
        return (1u32) << 16i32;
    }
    if (*mech).gss_set_sec_context_option.is_none() {
        return (16u32) << 16i32;
    }
    status = (*mech)
        .gss_set_sec_context_option
        .expect("non-null function pointer")(
        minor_status,
        if !ctx.is_null() {
            &mut (*ctx).internal_ctx_id
        } else {
            &mut internal_ctx
        },
        desired_object,
        value,
    );
    if status != 0u32 {
        *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
            *minor_status,
            &mut (*mech).mech_type,
        );
        return status;
    }
    if ctx.is_null() && !internal_ctx.is_null() {
        status = crate::src::mechglue::g_glue::gssint_create_union_context(
            minor_status,
            &mut (*mech).mech_type as *mut crate::gssapi_h::gss_OID_desc
                as crate::gssapi_h::gss_const_OID,
            &mut ctx,
        );
        if status != 0u32 {
            crate::src::mechglue::g_glue::gssint_delete_internal_sec_context(
                &mut minor,
                (*ctx).mech_type,
                &mut internal_ctx,
                0 as crate::gssapi_h::gss_buffer_t,
            );
            return status;
        }
        (*ctx).internal_ctx_id = internal_ctx;
        *context_handle = ctx as crate::gssapi_h::gss_ctx_id_t
    }
    return 0u32;
}
