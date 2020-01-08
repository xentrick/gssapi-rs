use ::libc;

/* _GSSAPI_H_ */
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::uint32_t;

pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_name_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::mglueP_h::gss_name_struct;
pub use crate::mglueP_h::gss_union_name_t;
pub use crate::src::mechglue::g_glue::gssint_export_internal_name;
/* qop_state */
/* New for V2 */
/*
 * Copyright (c) 1996,1997, by Sun Microsystems, Inc.
 * All rights reserved.
 */
/* #pragma ident	"@(#)g_export_name.c	1.11	00/07/17 SMI" */
/*
 * glue routine gss_export_name
 *
 * Will either call the mechanism defined gss_export_name, or if one is
 * not defined will call a generic_gss_export_name routine.
 */
#[no_mangle]

pub unsafe extern "C" fn gss_export_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    input_name: crate::gssapi_h::gss_name_t,
    mut exported_name: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut union_name = 0 as *mut crate::mglueP_h::gss_name_struct;
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !exported_name.is_null() {
        (*exported_name).value = 0 as *mut libc::c_void;
        (*exported_name).length = 0usize
    }
    /* Validate arguments. */
    if minor_status.is_null() || exported_name.is_null() {
        return (2u32) << 24i32;
    }
    if input_name.is_null() {
        return (1u32) << 24i32 | (2u32) << 16i32;
    }
    union_name = input_name;
    /* the name must be in mechanism specific format */
    if (*union_name).mech_type.is_null() {
        return (18u32) << 16i32;
    }
    return crate::src::mechglue::g_glue::gssint_export_internal_name(
        minor_status,
        (*union_name).mech_type,
        (*union_name).mech_name,
        exported_name,
    );
}
