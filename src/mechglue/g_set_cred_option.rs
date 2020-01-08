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
pub use crate::src::mechglue::g_rel_cred::gss_release_cred;

/*
 * Copyright 2008-2010 by the Massachusetts Institute of Technology.
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
/* Glue routine for gssspi_set_cred_option */

unsafe extern "C" fn alloc_union_cred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut mech: crate::mglueP_h::gss_mechanism,
    mut mech_cred: crate::gssapi_h::gss_cred_id_t,
    mut pcred: *mut crate::mglueP_h::gss_union_cred_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut temp_minor_status: crate::gssapi_h::OM_uint32 = 0;
    let mut cred = 0 as crate::mglueP_h::gss_union_cred_t;
    *pcred = 0 as crate::mglueP_h::gss_union_cred_t;
    status = (13u32) << 16i32;
    cred = crate::stdlib::calloc(
        1usize,
        ::std::mem::size_of::<crate::mglueP_h::gss_cred_id_struct>(),
    ) as crate::mglueP_h::gss_union_cred_t;
    if cred.is_null() {
        *minor_status = 12u32
    } else {
        (*cred).loopback = cred;
        (*cred).count = 1i32;
        (*cred).cred_array = crate::stdlib::calloc(
            (*cred).count as usize,
            ::std::mem::size_of::<crate::gssapi_h::gss_cred_id_t>(),
        ) as *mut crate::gssapi_h::gss_cred_id_t;
        if (*cred).cred_array.is_null() {
            *minor_status = 12u32
        } else {
            let ref mut fresh0 = *(*cred).cred_array.offset(0isize);
            *fresh0 = mech_cred;
            status = crate::src::generic::oid_ops::generic_gss_copy_oid(
                minor_status,
                &mut (*mech).mech_type,
                &mut (*cred).mechs_array,
            );
            if !(status != 0u32) {
                status = 0u32;
                *pcred = cred
            }
        }
    }
    if status != 0u32 {
        crate::src::mechglue::g_rel_cred::gss_release_cred(
            &mut temp_minor_status,
            &mut cred as *mut crate::mglueP_h::gss_union_cred_t,
        );
    }
    return status;
}
/*
 * Heimdal extension
 */
/*
 * This differs from gssspi_set_cred_option() as shipped in 1.7, in that
 * it can return a cred handle. To denote this change we have changed the
 * name of the function from gssspi_set_cred_option() to gss_set_cred_option().
 * However, the dlsym() entry point is still gssspi_set_cred_option(). This
 * fixes a separate issue, namely that a dynamically loaded mechanism could
 * not itself call set_cred_option() without calling its own implementation
 * instead of the mechanism glue's. (This is useful where a mechanism wishes
 * to export a mechanism-specific API that is a wrapper around this function.)
 */
#[no_mangle]

pub unsafe extern "C" fn gss_set_cred_option(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    desired_object: crate::gssapi_h::gss_OID,
    value: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut union_cred = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    let mut i: i32 = 0;
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut mech_status: crate::gssapi_h::OM_uint32 = 0;
    let mut mech_minor_status: crate::gssapi_h::OM_uint32 = 0;
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    *minor_status = 0u32;
    if cred_handle.is_null() {
        return (2u32) << 24i32;
    }
    status = (16u32) << 16i32;
    if (*cred_handle).is_null() {
        let mut mech_cred = 0 as crate::gssapi_h::gss_cred_id_t;
        /*
         * We need to give a mechanism the opportunity to allocate a
         * credentials handle. Unfortunately this does mean that only
         * the default mechanism can allocate a credentials handle.
         */
        mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
            0 as crate::gssapi_h::gss_const_OID,
        );
        if mech.is_null() {
            return (1u32) << 16i32;
        }
        if (*mech).gssspi_set_cred_option.is_none() {
            return (16u32) << 16i32;
        }
        status = (*mech)
            .gssspi_set_cred_option
            .expect("non-null function pointer")(
            minor_status,
            &mut mech_cred,
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
        if !mech_cred.is_null() {
            status = alloc_union_cred(minor_status, mech, mech_cred, &mut union_cred);
            if status != 0u32 {
                return status;
            }
            *cred_handle = union_cred
        }
    } else {
        union_cred = *cred_handle;
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
                if !(*mech).gssspi_set_cred_option.is_none() {
                    mech_status = (*mech)
                        .gssspi_set_cred_option
                        .expect("non-null function pointer")(
                        &mut mech_minor_status,
                        &mut *(*union_cred).cred_array.offset(i as isize),
                        desired_object,
                        value,
                    );
                    if !(mech_status == (16u32) << 16i32) {
                        status = mech_status;
                        *minor_status = mech_minor_status;
                        if status != 0u32 {
                            *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                                *minor_status,
                                &mut (*mech).mech_type,
                            );
                            break;
                        }
                    }
                }
                i += 1
            }
        }
    }
    return status;
}
/*
 * Provide this for backward ABI compatibility, but remove it from the
 * header.
 */
#[no_mangle]

pub unsafe extern "C" fn gssspi_set_cred_option(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut cred: crate::gssapi_h::gss_cred_id_t,
    desired_object: crate::gssapi_h::gss_OID,
    value: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    return gss_set_cred_option(minor_status, &mut cred, desired_object, value);
}
