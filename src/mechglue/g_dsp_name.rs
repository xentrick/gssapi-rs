use ::libc;

pub mod gssapi_alloc_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
    /* To the extent possible under law, Painless Security, LLC has waived
     * all copyright and related or neighboring rights to GSS-API Memory
     * Management Header. This work is published from: United States.
     */
    /* not _WIN32 or DEBUG_GSSALLOC */
    /* Normal Unix case, just use free/malloc/calloc/realloc. */
    #[inline]

    pub unsafe extern "C" fn gssalloc_malloc(
        mut size: crate::stddef_h::size_t,
    ) -> *mut libc::c_void {
        return crate::stdlib::malloc(size);
    }
}

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
pub use crate::src::mechglue::g_dsp_name::gssapi_alloc_h::gssalloc_malloc;
pub use crate::src::mechglue::g_glue::gssint_display_internal_name;

/* #pragma ident	"@(#)g_dsp_name.c	1.13	04/02/23 SMI" */
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
 *  glue routine for gss_display_name()
 *
 */

unsafe extern "C" fn val_dsp_name_args(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut input_name: crate::gssapi_h::gss_name_t,
    mut output_name_buffer: crate::gssapi_h::gss_buffer_t,
    mut output_name_type: *mut crate::gssapi_h::gss_OID,
) -> crate::gssapi_h::OM_uint32 {
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !output_name_buffer.is_null() {
        (*output_name_buffer).length = 0usize;
        (*output_name_buffer).value = 0 as *mut libc::c_void
    }
    if !output_name_type.is_null() {
        *output_name_type = 0 as crate::gssapi_h::gss_OID
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
    return 0u32;
}
/* name_equal */
#[no_mangle]

pub unsafe extern "C" fn gss_display_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut input_name: crate::gssapi_h::gss_name_t,
    mut output_name_buffer: crate::gssapi_h::gss_buffer_t,
    mut output_name_type: *mut crate::gssapi_h::gss_OID,
) -> crate::gssapi_h::OM_uint32 {
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut union_name = 0 as *mut crate::mglueP_h::gss_name_struct;
    major_status = val_dsp_name_args(
        minor_status,
        input_name,
        output_name_buffer,
        output_name_type,
    );
    if major_status != 0u32 {
        return major_status;
    }
    union_name = input_name;
    if !(*union_name).mech_type.is_null() {
        /*
         * OK, we have a mechanism-specific name; let's use it!
         */
        return crate::src::mechglue::g_glue::gssint_display_internal_name(
            minor_status,
            (*union_name).mech_type,
            (*union_name).mech_name,
            output_name_buffer,
            output_name_type,
        );
    }
    (*output_name_buffer).value =
        gssalloc_malloc((*(*union_name).external_name).length.wrapping_add(1usize));
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
    if !output_name_type.is_null() {
        *output_name_type = (*union_name).name_type
    }
    return 0u32;
}
