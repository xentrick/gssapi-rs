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
pub use crate::mglueP_h::gss_union_cred_t;

pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;
/* cred_usage_stored */
/* lib/gssapi/mechglue/g_set_neg_mechs.c - Glue for gss_set_neg_mechs */
/*
 * Copyright (C) 2010 by the Massachusetts Institute of Technology.
 * All rights reserved.
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
 * permission.	Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 * M.I.T. makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 */
#[no_mangle]

pub unsafe extern "C" fn gss_set_neg_mechs(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut cred_handle: crate::gssapi_h::gss_cred_id_t,
    mech_set: crate::gssapi_h::gss_OID_set,
) -> crate::gssapi_h::OM_uint32 {
    let mut union_cred = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    let mut i: i32 = 0;
    let mut avail: i32 = 0;
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    *minor_status = 0u32;
    if cred_handle.is_null() {
        return (1u32) << 24i32 | (7u32) << 16i32;
    }
    union_cred = cred_handle;
    avail = 0i32;
    status = 0u32;
    i = 0i32;
    while i < (*union_cred).count {
        mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
            &mut *(*union_cred).mechs_array.offset(i as isize)
                as *mut crate::gssapi_h::gss_OID_desc_struct
                as crate::gssapi_h::gss_const_OID,
        );
        if mech.is_null() {
            status = (1u32) << 16i32;
            break;
        } else {
            if !(*mech).gss_set_neg_mechs.is_none() {
                avail = 1i32;
                status = (*mech)
                    .gss_set_neg_mechs
                    .expect("non-null function pointer")(
                    minor_status,
                    *(*union_cred).cred_array.offset(i as isize),
                    mech_set,
                );
                if status != 0u32 {
                    *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                        *minor_status,
                        &mut (*mech).mech_type,
                    );
                    break;
                }
            }
            i += 1
        }
    }
    if status == 0u32 && avail == 0 {
        return (16u32) << 16i32;
    }
    return status;
}
