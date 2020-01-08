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
pub use crate::mglueP_h::gss_union_name_t;

pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;

/* -*- mode: c; indent-tabs-mode: nil -*- */
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
 *  glue routine for gss_display_name_ext()
 *
 */

unsafe extern "C" fn val_dsp_name_ext_args(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut input_name: crate::gssapi_h::gss_name_t,
    mut display_as_name_type: crate::gssapi_h::gss_OID,
    mut output_name_buffer: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !output_name_buffer.is_null() {
        (*output_name_buffer).length = 0usize;
        (*output_name_buffer).value = 0 as *mut libc::c_void
    }
    /* Validate arguments. */
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    if output_name_buffer.is_null() {
        return (2u32) << 24i32;
    }
    if input_name.is_null() {
        return (1u32) << 24i32 | (2u32) << 16i32;
    }
    if display_as_name_type.is_null() {
        return (1u32) << 24i32 | (3u32) << 16i32;
    }
    return 0u32;
}
#[no_mangle]

pub unsafe extern "C" fn gss_display_name_ext(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut input_name: crate::gssapi_h::gss_name_t,
    mut display_as_name_type: crate::gssapi_h::gss_OID,
    mut output_name_buffer: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut union_name = 0 as *mut crate::mglueP_h::gss_name_struct;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    status = val_dsp_name_ext_args(
        minor_status,
        input_name,
        display_as_name_type,
        output_name_buffer,
    );
    if status != 0u32 {
        return status;
    }
    union_name = input_name;
    if !(*union_name).mech_type.is_null() {
        mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
            (*union_name).mech_type as crate::gssapi_h::gss_const_OID,
        );
        if mech.is_null() {
            status = (2u32) << 16i32
        } else if (*mech).gss_display_name_ext.is_none() {
            if (*mech).gss_display_name.is_some()
                && !(*union_name).name_type.is_null()
                && ((*display_as_name_type).length == (*(*union_name).name_type).length
                    && crate::stdlib::memcmp(
                        (*display_as_name_type).elements,
                        (*(*union_name).name_type).elements,
                        (*display_as_name_type).length as usize,
                    ) == 0i32)
            {
                status = Some((*mech).gss_display_name.expect("non-null function pointer"))
                    .expect("non-null function pointer")(
                    minor_status,
                    (*union_name).mech_name,
                    output_name_buffer,
                    0 as *mut crate::gssapi_h::gss_OID,
                );
                if status != 0u32 {
                    *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                        *minor_status,
                        &mut (*mech).mech_type,
                    )
                }
            } else {
                status = (16u32) << 16i32
            }
        } else {
            status = Some(
                (*mech)
                    .gss_display_name_ext
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                minor_status,
                (*union_name).mech_name,
                display_as_name_type,
                output_name_buffer,
            );
            if status != 0u32 {
                *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                    *minor_status,
                    &mut (*mech).mech_type,
                )
            }
        }
        return status;
    }
    if (*union_name).name_type.is_null()
        || !((*display_as_name_type).length == (*(*union_name).name_type).length
            && crate::stdlib::memcmp(
                (*display_as_name_type).elements,
                (*(*union_name).name_type).elements,
                (*display_as_name_type).length as usize,
            ) == 0i32)
    {
        return (16u32) << 16i32;
    }
    (*output_name_buffer).value =
        crate::stdlib::malloc((*(*union_name).external_name).length.wrapping_add(1usize));
    if (*output_name_buffer).value.is_null() {
        return (13u32) << 16i32;
    }
    (*output_name_buffer).length = (*(*union_name).external_name).length;
    crate::stdlib::memcpy(
        (*output_name_buffer).value,
        (*(*union_name).external_name).value,
        (*(*union_name).external_name).length,
    );
    *((*output_name_buffer).value as *mut i8).offset((*output_name_buffer).length as isize) =
        '\u{0}' as i8;
    return 0u32;
}
