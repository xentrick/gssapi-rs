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
    #[inline]

    pub unsafe extern "C" fn gssalloc_malloc(
        mut size: crate::stddef_h::size_t,
    ) -> *mut libc::c_void {
        return crate::stdlib::malloc(size);
    }
    #[inline]

    pub unsafe extern "C" fn gssalloc_realloc(
        mut value: *mut libc::c_void,
        mut size: crate::stddef_h::size_t,
    ) -> *mut libc::c_void {
        return crate::stdlib::realloc(value, size);
    }
}

/* _GSSAPIP_GENERIC_H_ */
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::uint32_t;

pub use crate::gssapi_ext_h::gss_buffer_set_desc;
pub use crate::gssapi_ext_h::gss_buffer_set_desc_struct;
pub use crate::gssapi_ext_h::gss_buffer_set_t;
pub use crate::gssapi_h::gss_buffer_desc;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;

pub use crate::src::generic::util_buffer_set::gssapi_alloc_h::gssalloc_free;
pub use crate::src::generic::util_buffer_set::gssapi_alloc_h::gssalloc_malloc;
pub use crate::src::generic::util_buffer_set::gssapi_alloc_h::gssalloc_realloc;

/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2008 by the Massachusetts Institute of Technology.
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
 *
 */
#[no_mangle]

pub unsafe extern "C" fn generic_gss_create_empty_buffer_set(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut buffer_set: *mut crate::gssapi_ext_h::gss_buffer_set_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut set = 0 as *mut crate::gssapi_ext_h::gss_buffer_set_desc_struct;
    set = gssalloc_malloc(::std::mem::size_of::<
        crate::gssapi_ext_h::gss_buffer_set_desc_struct,
    >()) as *mut crate::gssapi_ext_h::gss_buffer_set_desc;
    if set.is_null() {
        *minor_status = 12u32;
        return (13u32) << 16i32;
    }
    (*set).count = 0usize;
    (*set).elements = 0 as *mut crate::gssapi_h::gss_buffer_desc;
    *buffer_set = set;
    *minor_status = 0u32;
    return 0u32;
}
#[no_mangle]

pub unsafe extern "C" fn generic_gss_add_buffer_set_member(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    member_buffer: crate::gssapi_h::gss_buffer_t,
    mut buffer_set: *mut crate::gssapi_ext_h::gss_buffer_set_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut set = 0 as *mut crate::gssapi_ext_h::gss_buffer_set_desc_struct;
    let mut p = 0 as *mut crate::gssapi_h::gss_buffer_desc_struct;
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    if (*buffer_set).is_null() {
        ret = generic_gss_create_empty_buffer_set(minor_status, buffer_set);
        if ret != 0 {
            return ret;
        }
    }
    set = *buffer_set;
    (*set).elements = gssalloc_realloc(
        (*set).elements as *mut libc::c_void,
        (*set)
            .count
            .wrapping_add(1usize)
            .wrapping_mul(::std::mem::size_of::<crate::gssapi_h::gss_buffer_desc>()),
    ) as *mut crate::gssapi_h::gss_buffer_desc;
    if (*set).elements.is_null() {
        *minor_status = 12u32;
        return (13u32) << 16i32;
    }
    p = &mut *(*set).elements.offset((*set).count as isize)
        as *mut crate::gssapi_h::gss_buffer_desc;
    (*p).value = gssalloc_malloc((*member_buffer).length);
    if (*p).value.is_null() {
        *minor_status = 12u32;
        return (13u32) << 16i32;
    }
    crate::stdlib::memcpy((*p).value, (*member_buffer).value, (*member_buffer).length);
    (*p).length = (*member_buffer).length;
    (*set).count = (*set).count.wrapping_add(1);
    *minor_status = 0u32;
    return 0u32;
}
/* -*- mode: c; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 by OpenVision Technologies, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
/*
 * $Id$
 */
/* * helper macros **/
/* this code knows that an int on the wire is 32 bits.  The type of
num should be at least this big, or the extra shifts may do weird
things */
/* * malloc wrappers; these may actually do something later */
/* * helper functions **/
/* hide names from applications, especially glib applications */
/* flags for g_verify_token_header() */
/* * declarations of internal name mechanism functions **/
/* minor_status */
/* buffer */
/* minor_status */
/* set */
/* minor_status */
/* set */
/* minor_status */
/* oid */
/* new_oid */
/* minor_status */
/* oid_set */
/* minor_status */
/* member_oid */
/* oid_set */
/* minor_status */
/* member */
/* set */
/* present */
/* minor_status */
/* oid */
/* oid_str */
/* minor_status */
/* oid_str */
/* oid */
/* minor_status */
/* prefix */
/* prefix_len */
/* suffix */
/* oid */
/* minor_status */
/*prefix */
/* prefix_len */
/* oid */
/* suffix */
/*
 * Transfer contents of a k5buf to a gss_buffer and invalidate the source
 * On unix, this is a simple pointer copy
 * On windows, memory is reallocated and copied.
 */
/*minor_status*/
/*buffer_set*/
/*minor_status*/
/*member_buffer*/
/*buffer_set*/
#[no_mangle]

pub unsafe extern "C" fn generic_gss_release_buffer_set(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut buffer_set: *mut crate::gssapi_ext_h::gss_buffer_set_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut i: crate::stddef_h::size_t = 0;
    let mut minor: crate::gssapi_h::OM_uint32 = 0;
    *minor_status = 0u32;
    if (*buffer_set).is_null() {
        return 0u32;
    }
    i = 0usize;
    while i < (**buffer_set).count {
        crate::src::generic::rel_buffer::generic_gss_release_buffer(
            &mut minor,
            &mut *(**buffer_set).elements.offset(i as isize),
        );
        i = i.wrapping_add(1)
    }
    if !(**buffer_set).elements.is_null() {
        gssalloc_free((**buffer_set).elements as *mut libc::c_void);
        (**buffer_set).elements = 0 as *mut crate::gssapi_h::gss_buffer_desc
    }
    (**buffer_set).count = 0usize;
    gssalloc_free(*buffer_set as *mut libc::c_void);
    *buffer_set = 0 as crate::gssapi_ext_h::gss_buffer_set_t;
    return 0u32;
}
