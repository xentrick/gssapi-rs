use ::libc;

/* _GSSAPIP_GENERIC_H_ */
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::uint32_t;

pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_name_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::mglueP_h::gss_name_struct;
pub use crate::mglueP_h::gss_union_name_desc;
pub use crate::mglueP_h::gss_union_name_t;

pub use crate::src::mechglue::g_glue::gssint_create_copy_buffer;
pub use crate::src::mechglue::g_glue::gssint_import_internal_name;
pub use crate::src::mechglue::g_glue::gssint_release_internal_name;

/*
 * Copyright 2004 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
/* #pragma ident	"@(#)g_dup_name.c	1.14	04/02/23 SMI" */
/*
 *  routine gss_duplicate_name
 *
 * This routine does not rely on mechanism implementation of this
 * name, but instead uses mechanism specific gss_import_name routine.
 */

unsafe extern "C" fn val_dup_name_args(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    src_name: crate::gssapi_h::gss_name_t,
    mut dest_name: *mut crate::gssapi_h::gss_name_t,
) -> crate::gssapi_h::OM_uint32 {
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !dest_name.is_null() {
        *dest_name = 0 as crate::gssapi_h::gss_name_t
    }
    /* Validate arguments. */
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    /* if output_name is NULL, simply return */
    if dest_name.is_null() {
        return (2u32) << 24i32;
    }
    if src_name.is_null() {
        return (1u32) << 24i32 | (2u32) << 16i32;
    }
    return 0u32;
}
/* exported_name */
/* New for V2 */
#[no_mangle]

pub unsafe extern "C" fn gss_duplicate_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    src_name: crate::gssapi_h::gss_name_t,
    mut dest_name: *mut crate::gssapi_h::gss_name_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut src_union = 0 as *mut crate::mglueP_h::gss_name_struct;
    let mut dest_union = 0 as *mut crate::mglueP_h::gss_name_struct;
    let mut major_status = (13u32) << 16i32;
    major_status = val_dup_name_args(minor_status, src_name, dest_name);
    if major_status != 0u32 {
        return major_status;
    }
    src_union = src_name;
    /*
     * First create the union name struct that will hold the external
     * name and the name type.
     */
    dest_union = crate::stdlib::malloc(::std::mem::size_of::<crate::mglueP_h::gss_union_name_desc>())
        as crate::mglueP_h::gss_union_name_t;
    if !dest_union.is_null() {
        (*dest_union).loopback = 0 as *mut crate::mglueP_h::gss_name_struct;
        (*dest_union).mech_type = 0 as crate::gssapi_h::gss_OID;
        (*dest_union).mech_name = 0 as crate::gssapi_h::gss_name_t;
        (*dest_union).name_type = 0 as crate::gssapi_h::gss_OID;
        (*dest_union).external_name = 0 as crate::gssapi_h::gss_buffer_t;
        /* Now copy the external representaion */
        if !(crate::src::mechglue::g_glue::gssint_create_copy_buffer(
            (*src_union).external_name,
            &mut (*dest_union).external_name,
            0i32,
        ) != 0)
        {
            if !(*src_union).name_type.is_null() {
                major_status = crate::src::generic::oid_ops::generic_gss_copy_oid(
                    minor_status,
                    (*src_union).name_type as *const crate::gssapi_h::gss_OID_desc,
                    &mut (*dest_union).name_type,
                );
                if major_status != 0u32 {
                    *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(
                        *minor_status,
                    );
                    current_block = 4125999606161253044;
                } else {
                    current_block = 12039483399334584727;
                }
            } else {
                current_block = 12039483399334584727;
            }
            match current_block {
                4125999606161253044 => {}
                _ =>
                /*
                 * See if source name is mechanim specific, if so then need to import it
                 */
                {
                    if !(*src_union).mech_type.is_null() {
                        major_status = crate::src::generic::oid_ops::generic_gss_copy_oid(
                            minor_status,
                            (*src_union).mech_type as *const crate::gssapi_h::gss_OID_desc,
                            &mut (*dest_union).mech_type,
                        );
                        if major_status != 0u32 {
                            *minor_status =
                                crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(
                                    *minor_status,
                                );
                            current_block = 4125999606161253044;
                        } else {
                            major_status =
                                crate::src::mechglue::g_glue::gssint_import_internal_name(
                                    minor_status,
                                    (*src_union).mech_type,
                                    src_union,
                                    &mut (*dest_union).mech_name,
                                );
                            if major_status != 0u32 {
                                current_block = 4125999606161253044;
                            } else {
                                current_block = 11307063007268554308;
                            }
                        }
                    } else {
                        current_block = 11307063007268554308;
                    }
                    match current_block {
                        4125999606161253044 => {}
                        _ => {
                            (*dest_union).loopback = dest_union;
                            *dest_name = dest_union;
                            return 0u32;
                        }
                    }
                }
            }
        }
    }
    if !dest_union.is_null() {
        if !(*dest_union).external_name.is_null() {
            if !(*(*dest_union).external_name).value.is_null() {
                crate::stdlib::free((*(*dest_union).external_name).value);
            }
            crate::stdlib::free((*dest_union).external_name as *mut libc::c_void);
        }
        if !(*dest_union).name_type.is_null() {
            crate::src::generic::oid_ops::generic_gss_release_oid(
                minor_status,
                &mut (*dest_union).name_type,
            );
        }
        if !(*dest_union).mech_name.is_null() {
            crate::src::mechglue::g_glue::gssint_release_internal_name(
                minor_status,
                (*dest_union).mech_type,
                &mut (*dest_union).mech_name,
            );
        }
        if !(*dest_union).mech_type.is_null() {
            crate::src::generic::oid_ops::generic_gss_release_oid(
                minor_status,
                &mut (*dest_union).mech_type,
            );
        }
        crate::stdlib::free(dest_union as *mut libc::c_void);
    }
    return major_status;
}
/*	gss_duplicate_name	*/
