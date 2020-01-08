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
pub use crate::mglueP_h::gss_union_name_t;

pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;
/* -*- mode: c; indent-tabs-mode: nil -*- */
/*
 * Copyright 2009 by the Massachusetts Institute of Technology.
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
/* Glue routine for gss_set_name_attribute */
#[no_mangle]

pub unsafe extern "C" fn gss_set_name_attribute(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut name: crate::gssapi_h::gss_name_t,
    mut complete: i32,
    mut attr: crate::gssapi_h::gss_buffer_t,
    mut value: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut union_name = 0 as *mut crate::mglueP_h::gss_name_struct;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    *minor_status = 0u32;
    if name.is_null() {
        return (1u32) << 24i32 | (2u32) << 16i32;
    }
    union_name = name;
    if (*union_name).mech_type.is_null() {
        return (16u32) << 16i32;
    }
    mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
        (*name).mech_type as crate::gssapi_h::gss_const_OID,
    );
    if mech.is_null() {
        return (2u32) << 16i32;
    }
    if (*mech).gss_set_name_attribute.is_none() {
        return (16u32) << 16i32;
    }
    status = Some(
        (*mech)
            .gss_set_name_attribute
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        minor_status,
        (*union_name).mech_name,
        complete,
        attr,
        value,
    );
    if status != 0u32 {
        *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
            *minor_status,
            &mut (*mech).mech_type,
        )
    }
    return status;
}
