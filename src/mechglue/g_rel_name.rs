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

    pub unsafe extern "C" fn gssalloc_free(mut value: *mut libc::c_void) {
        crate::stdlib::free(value);
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
pub use crate::src::mechglue::g_glue::gssint_release_internal_name;
pub use crate::src::mechglue::g_initialize::gss_release_oid;
pub use crate::src::mechglue::g_rel_name::gssapi_alloc_h::gssalloc_free;

/* #pragma ident	"@(#)g_rel_name.c	1.11	04/02/23 SMI" */
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
 *  glue routine for gss_release_name
 */
#[no_mangle]

pub unsafe extern "C" fn gss_release_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut input_name: *mut crate::gssapi_h::gss_name_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut union_name = 0 as *mut crate::mglueP_h::gss_name_struct;
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    *minor_status = 0u32;
    /* if input_name is NULL, return error */
    if input_name.is_null() {
        return (1u32) << 24i32 | (2u32) << 16i32;
    }
    if (*input_name).is_null() {
        return 0u32;
    }
    /*
     * free up the space for the external_name and then
     * free the union_name descriptor
     */
    union_name = *input_name;
    if !(!union_name.is_null() && (*union_name).loopback == union_name) {
        return (1u32) << 24i32 | (2u32) << 16i32;
    }
    *input_name = 0 as crate::gssapi_h::gss_name_t;
    *minor_status = 0u32;
    if !(*union_name).name_type.is_null() {
        crate::src::mechglue::g_initialize::gss_release_oid(
            minor_status,
            &mut (*union_name).name_type,
        );
    }
    if !(*union_name).external_name.is_null() {
        if !(*(*union_name).external_name).value.is_null() {
            gssalloc_free((*(*union_name).external_name).value);
        }
        crate::stdlib::free((*union_name).external_name as *mut libc::c_void);
    }
    if !(*union_name).mech_type.is_null() {
        crate::src::mechglue::g_glue::gssint_release_internal_name(
            minor_status,
            (*union_name).mech_type,
            &mut (*union_name).mech_name,
        );
        crate::src::mechglue::g_initialize::gss_release_oid(
            minor_status,
            &mut (*union_name).mech_type,
        );
    }
    crate::stdlib::free(union_name as *mut libc::c_void);
    return 0u32;
}
