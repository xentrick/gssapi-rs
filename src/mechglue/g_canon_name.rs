pub use crate::stddef_h::size_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::uint32_t;

pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_const_OID;
pub use crate::gssapi_h::gss_name_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::mglueP_h::gss_name_struct;
pub use crate::mglueP_h::gss_union_name_desc;
pub use crate::mglueP_h::gss_union_name_t;

pub use crate::src::mechglue::g_glue::gssint_create_copy_buffer;
pub use crate::src::mechglue::g_glue::gssint_import_internal_name;
pub use crate::src::mechglue::g_glue::gssint_release_internal_name;
pub use crate::src::mechglue::g_initialize::gss_release_oid;
pub use crate::src::mechglue::g_initialize::gssint_select_mech_type;
pub use crate::src::mechglue::g_rel_name::gss_release_name;

/*
 * Copyright 2004 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
/* #pragma ident	"@(#)g_canon_name.c	1.15	04/02/23 SMI" */
/*
 * routine gss_canonicalize_name
 *
 * This routine is used to produce a mechanism specific
 * representation of name that has been previously
 * imported with gss_import_name.  The routine uses the mechanism
 * specific implementation of gss_import_name to implement this
 * function.
 *
 * We allow a NULL output_name, in which case we modify the
 * input_name to include the mechanism specific name.
 */

unsafe extern "C" fn val_canon_name_args(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    input_name: crate::gssapi_h::gss_name_t,
    mech_type: crate::gssapi_h::gss_OID,
    mut output_name: *mut crate::gssapi_h::gss_name_t,
) -> crate::gssapi_h::OM_uint32 {
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !output_name.is_null() {
        *output_name = 0 as crate::gssapi_h::gss_name_t
    }
    /* Validate arguments. */
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    if input_name.is_null() || mech_type.is_null() {
        return (1u32) << 24i32;
    }
    return 0u32;
}
/* dest_name */
/* New for V2 */
#[no_mangle]

pub unsafe extern "C" fn gss_canonicalize_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    input_name: crate::gssapi_h::gss_name_t,
    mech_type: crate::gssapi_h::gss_OID,
    mut output_name: *mut crate::gssapi_h::gss_name_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut in_union = 0 as *mut crate::mglueP_h::gss_name_struct;
    let mut out_union = 0 as crate::mglueP_h::gss_union_name_t;
    let mut dest_union = 0 as crate::mglueP_h::gss_union_name_t;
    let mut major_status = (13u32) << 16i32;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut selected_mech = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    major_status = val_canon_name_args(minor_status, input_name, mech_type, output_name);
    if major_status != 0u32 {
        return major_status;
    }
    major_status = crate::src::mechglue::g_initialize::gssint_select_mech_type(
        minor_status,
        mech_type as crate::gssapi_h::gss_const_OID,
        &mut selected_mech,
    );
    if major_status != 0u32 {
        return major_status;
    }
    /* Initial value needed below. */
    major_status = (13u32) << 16i32;
    in_union = input_name;
    /*
     * If the caller wants to reuse the name, and the name has already
     * been converted, then there is nothing for us to do.
     */
    if output_name.is_null()
        && !(*in_union).mech_type.is_null()
        && ((*(*in_union).mech_type).length == (*selected_mech).length
            && crate::stdlib::memcmp(
                (*(*in_union).mech_type).elements,
                (*selected_mech).elements,
                (*(*in_union).mech_type).length as usize,
            ) == 0i32)
    {
        return 0u32;
    }
    /* ok, then we need to do something - start by creating data struct */
    if !output_name.is_null() {
        out_union =
            crate::stdlib::malloc(::std::mem::size_of::<crate::mglueP_h::gss_union_name_desc>())
                as crate::mglueP_h::gss_union_name_t;
        if out_union.is_null() {
            current_block = 5332791911318817343;
        } else {
            (*out_union).mech_type = 0 as crate::gssapi_h::gss_OID;
            (*out_union).mech_name = 0 as crate::gssapi_h::gss_name_t;
            (*out_union).name_type = 0 as crate::gssapi_h::gss_OID;
            (*out_union).external_name = 0 as crate::gssapi_h::gss_buffer_t;
            (*out_union).loopback = out_union;
            /* Allocate the buffer for the user specified representation */
            if crate::src::mechglue::g_glue::gssint_create_copy_buffer(
                (*in_union).external_name,
                &mut (*out_union).external_name,
                1i32,
            ) != 0
            {
                current_block = 5332791911318817343;
            } else if !(*in_union).name_type.is_null() {
                major_status = crate::src::generic::oid_ops::generic_gss_copy_oid(
                    minor_status,
                    (*in_union).name_type as *const crate::gssapi_h::gss_OID_desc,
                    &mut (*out_union).name_type,
                );
                if major_status != 0 {
                    *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(
                        *minor_status,
                    );
                    current_block = 5332791911318817343;
                } else {
                    current_block = 11307063007268554308;
                }
            } else {
                current_block = 11307063007268554308;
            }
        }
    } else {
        current_block = 11307063007268554308;
    }
    match current_block {
        11307063007268554308 => {
            /*
             * might need to delete any old mechanism names if we are
             * reusing the buffer.
             */
            if output_name.is_null() {
                if !(*in_union).mech_type.is_null() {
                    crate::src::mechglue::g_glue::gssint_release_internal_name(
                        minor_status,
                        (*in_union).mech_type,
                        &mut (*in_union).mech_name,
                    );
                    crate::src::mechglue::g_initialize::gss_release_oid(
                        minor_status,
                        &mut (*in_union).mech_type,
                    );
                    (*in_union).mech_type = 0 as crate::gssapi_h::gss_OID
                }
                dest_union = in_union
            } else {
                dest_union = out_union
            }
            /* now let's create the new mech name */
            major_status = crate::src::generic::oid_ops::generic_gss_copy_oid(
                minor_status,
                selected_mech as *const crate::gssapi_h::gss_OID_desc,
                &mut (*dest_union).mech_type,
            );
            if major_status != 0 {
                *minor_status =
                    crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status)
            } else {
                major_status = crate::src::mechglue::g_glue::gssint_import_internal_name(
                    minor_status,
                    selected_mech,
                    in_union,
                    &mut (*dest_union).mech_name,
                );
                if !(major_status != 0) {
                    if !output_name.is_null() {
                        *output_name = dest_union
                    }
                    return 0u32;
                }
            }
        }
        _ => {}
    }
    if !out_union.is_null() {
        /* Release the partly constructed out_union. */
        let mut name = out_union;
        crate::src::mechglue::g_rel_name::gss_release_name(&mut tmpmin, &mut name);
    } else if output_name.is_null() {
        /* Release only the mech name fields in in_union. */
        if !(*in_union).mech_name.is_null() {
            crate::src::mechglue::g_glue::gssint_release_internal_name(
                &mut tmpmin,
                (*dest_union).mech_type,
                &mut (*dest_union).mech_name,
            );
        }
        if !(*in_union).mech_type.is_null() {
            crate::src::mechglue::g_initialize::gss_release_oid(
                &mut tmpmin,
                &mut (*dest_union).mech_type,
            );
        }
    }
    return major_status;
}
/* *********  gss_canonicalize_name ********/
