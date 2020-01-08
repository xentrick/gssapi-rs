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

pub use crate::src::mechglue::g_dsp_name::gss_display_name;
pub use crate::src::mechglue::g_initialize::gss_indicate_mechs;
pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;
pub use crate::src::mechglue::g_initialize::gssint_get_public_oid;
pub use crate::src::mechglue::g_initialize::gssint_select_mech_type;
pub use crate::src::mechglue::g_oid_ops::gss_add_oid_set_member;
pub use crate::src::mechglue::g_oid_ops::gss_create_empty_oid_set;
pub use crate::src::mechglue::g_oid_ops::gss_test_oid_set_member;
pub use crate::src::mechglue::g_rel_buffer::gss_release_buffer;
pub use crate::src::mechglue::g_rel_oid_set::gss_release_oid_set;
/* #pragma ident	"@(#)g_inquire_names.c	1.16	04/02/23 SMI" */
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
/* Last argument new for V2 */
#[no_mangle]

pub unsafe extern "C" fn gss_inquire_names_for_mech(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut mechanism: crate::gssapi_h::gss_OID,
    mut name_types: *mut crate::gssapi_h::gss_OID_set,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut selected_mech = 0 as crate::gssapi_h::gss_OID;
    let mut public_mech = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !name_types.is_null() {
        *name_types = 0 as crate::gssapi_h::gss_OID_set
    }
    /* Validate arguments. */
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    if name_types.is_null() {
        return (2u32) << 24i32;
    }
    /*
     * select the approprate underlying mechanism routine and
     * call it.
     */
    status = crate::src::mechglue::g_initialize::gssint_select_mech_type(
        minor_status,
        mechanism as crate::gssapi_h::gss_const_OID,
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
    } else {
        if (*mech).gss_inquire_names_for_mech.is_none() {
            return (16u32) << 16i32;
        }
    }
    public_mech = crate::src::mechglue::g_initialize::gssint_get_public_oid(
        selected_mech as crate::gssapi_h::gss_const_OID,
    );
    status = (*mech)
        .gss_inquire_names_for_mech
        .expect("non-null function pointer")(minor_status, public_mech, name_types);
    if status != 0u32 {
        *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
            *minor_status,
            &mut (*mech).mech_type,
        )
    }
    return status;
}

unsafe extern "C" fn val_inq_mechs4name_args(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    input_name: crate::gssapi_h::gss_name_t,
    mut mech_set: *mut crate::gssapi_h::gss_OID_set,
) -> crate::gssapi_h::OM_uint32 {
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !mech_set.is_null() {
        *mech_set = 0 as crate::gssapi_h::gss_OID_set
    }
    /* Validate arguments.e
     */
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    if input_name.is_null() {
        return (2u32) << 16i32;
    }
    return 0u32;
}

unsafe extern "C" fn mech_supports_nametype(
    mut mech_oid: crate::gssapi_h::gss_OID,
    mut name_type: crate::gssapi_h::gss_OID,
) -> i32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut minor: crate::gssapi_h::OM_uint32 = 0;
    let mut types = 0 as crate::gssapi_h::gss_OID_set;
    let mut present: i32 = 0;
    status = gss_inquire_names_for_mech(&mut minor, mech_oid, &mut types);
    if status != 0u32 {
        return 0i32;
    }
    status = crate::src::mechglue::g_oid_ops::gss_test_oid_set_member(
        &mut minor,
        name_type,
        types,
        &mut present,
    );
    crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(&mut minor, &mut types);
    return (status == 0u32 && present != 0) as i32;
}
/* oid_str */
/* New for V2 */
/* minor_status */
/* mechanism */
/* name_types */
/* New for V2 */
#[no_mangle]

pub unsafe extern "C" fn gss_inquire_mechs_for_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    input_name: crate::gssapi_h::gss_name_t,
    mut mech_set: *mut crate::gssapi_h::gss_OID_set,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut all_mechs = 0 as crate::gssapi_h::gss_OID_set;
    let mut mechs = 0 as crate::gssapi_h::gss_OID_set;
    let mut mech_oid = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut name_type = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut name_buffer = {
        let mut init = crate::gssapi_h::gss_buffer_desc_struct {
            length: 0usize,
            value: 0 as *mut libc::c_void,
        };
        init
    };
    let mut i: crate::stddef_h::size_t = 0;
    status = val_inq_mechs4name_args(minor_status, input_name, mech_set);
    if status != 0u32 {
        return status;
    }
    status = crate::src::mechglue::g_dsp_name::gss_display_name(
        minor_status,
        input_name,
        &mut name_buffer,
        &mut name_type,
    );
    if !(status != 0u32) {
        status =
            crate::src::mechglue::g_initialize::gss_indicate_mechs(minor_status, &mut all_mechs);
        if !(status != 0u32) {
            status =
                crate::src::mechglue::g_oid_ops::gss_create_empty_oid_set(minor_status, &mut mechs);
            if !(status != 0u32) {
                i = 0usize;
                loop {
                    if !(i < (*all_mechs).count) {
                        current_block = 2370887241019905314;
                        break;
                    }
                    mech_oid = &mut *(*all_mechs).elements.offset(i as isize)
                        as *mut crate::gssapi_h::gss_OID_desc_struct;
                    if mech_supports_nametype(mech_oid, name_type) != 0 {
                        status = crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(
                            minor_status,
                            mech_oid,
                            &mut mechs,
                        );
                        if status != 0u32 {
                            current_block = 14279826438816034313;
                            break;
                        }
                    }
                    i = i.wrapping_add(1)
                }
                match current_block {
                    14279826438816034313 => {}
                    _ => {
                        *mech_set = mechs;
                        mechs = 0 as crate::gssapi_h::gss_OID_set
                    }
                }
            }
        }
    }
    crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmpmin, &mut name_buffer);
    crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(&mut tmpmin, &mut all_mechs);
    crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(&mut tmpmin, &mut mechs);
    return status;
}
