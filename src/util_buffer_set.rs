use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:27"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = u32;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:27"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:27"]
pub mod gssapi_h {
    /*
 * The following type must be defined as the smallest natural unsigned integer
 * supported by the platform that has at least 32 bits of precision.
 */
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    /* OM_STRING */
    /*
 * We can't use X/Open definitions, so roll our own.
 */
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]#[derive(Copy, Clone)]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_desc = gss_buffer_desc_struct;
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapi_ext.h:27"]
pub mod gssapi_ext_h {
    /* acceptor_time_rec */
    /*
 * GGF extensions
 */
    
    #[repr(C)]
    #[c2rust::src_loc = "134:16"]#[derive(Copy, Clone)]
    pub struct gss_buffer_set_desc_struct {
        pub count: size_t,
        pub elements: *mut gss_buffer_desc,
    }
    #[c2rust::src_loc = "134:1"]
    pub type gss_buffer_set_desc = gss_buffer_set_desc_struct;
    #[c2rust::src_loc = "134:1"]
    pub type gss_buffer_set_t = *mut gss_buffer_set_desc_struct;
    use super::stddef_h::size_t;
    use super::gssapi_h::gss_buffer_desc;
    /* GSSAPI_EXT_H_ */
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: usize) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: usize)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: usize) -> *mut libc::c_void;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_alloc.h:27"]
pub mod gssapi_alloc_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* To the extent possible under law, Painless Security, LLC has waived
 * all copyright and related or neighboring rights to GSS-API Memory
 * Management Header. This work is published from: United States.
 */
    /* not _WIN32 or DEBUG_GSSALLOC */
    /* Normal Unix case, just use free/malloc/calloc/realloc. */
    #[inline]
    #[c2rust::src_loc = "93:1"]
    pub unsafe extern "C" fn gssalloc_free(mut value: *mut libc::c_void) {
        free(value);
    }
    #[inline]
    #[c2rust::src_loc = "99:1"]
    pub unsafe extern "C" fn gssalloc_malloc(mut size: size_t)
     -> *mut libc::c_void {
        return malloc(size);
    }
    #[inline]
    #[c2rust::src_loc = "111:1"]
    pub unsafe extern "C" fn gssalloc_realloc(mut value: *mut libc::c_void,
                                              mut size: size_t)
     -> *mut libc::c_void {
        return realloc(value, size);
    }
    use super::stdlib_h::{free, malloc, realloc};
    use super::stddef_h::size_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapiP_generic.h:27"]
pub mod gssapiP_generic_h {
    use super::gssapi_h::{OM_uint32, gss_buffer_desc_struct, gss_buffer_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "191:1"]
        pub fn generic_gss_release_buffer(_: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
    }
    /* _GSSAPIP_GENERIC_H_ */
}
pub use self::stddef_h::size_t;
pub use self::types_h::__uint32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssapi_h::{gss_uint32, OM_uint32, gss_buffer_desc_struct,
                         gss_buffer_desc, gss_buffer_t};
pub use self::gssapi_ext_h::{gss_buffer_set_desc_struct, gss_buffer_set_desc,
                             gss_buffer_set_t};
use self::stdlib_h::{malloc, realloc, free};
use self::string_h::memcpy;
pub use self::gssapi_alloc_h::{gssalloc_free, gssalloc_malloc,
                               gssalloc_realloc};
use self::gssapiP_generic_h::generic_gss_release_buffer;
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
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn generic_gss_create_empty_buffer_set(mut minor_status:
                                                                 *mut OM_uint32,
                                                             mut buffer_set:
                                                                 *mut gss_buffer_set_t)
 -> OM_uint32 {
    let mut set = 0 as *mut gss_buffer_set_desc_struct;
    set =
        gssalloc_malloc(::std::mem::size_of::<gss_buffer_set_desc_struct>() as
                            usize) as *mut gss_buffer_set_desc;
    if set.is_null() {
        *minor_status = 12 as i32 as OM_uint32;
        return (13 as usize as OM_uint32) << 16 as i32
    }
    (*set).count = 0 as i32 as size_t;
    (*set).elements = 0 as *mut gss_buffer_desc;
    *buffer_set = set;
    *minor_status = 0 as i32 as OM_uint32;
    return 0 as i32 as OM_uint32;
}
#[no_mangle]
#[c2rust::src_loc = "56:1"]
pub unsafe extern "C" fn generic_gss_add_buffer_set_member(mut minor_status:
                                                               *mut OM_uint32,
                                                           member_buffer:
                                                               gss_buffer_t,
                                                           mut buffer_set:
                                                               *mut gss_buffer_set_t)
 -> OM_uint32 {
    let mut set = 0 as *mut gss_buffer_set_desc_struct;
    let mut p = 0 as *mut gss_buffer_desc_struct;
    let mut ret: OM_uint32 = 0;
    if (*buffer_set).is_null() {
        ret = generic_gss_create_empty_buffer_set(minor_status, buffer_set);
        if ret != 0 { return ret }
    }
    set = *buffer_set;
    (*set).elements =
        gssalloc_realloc((*set).elements as *mut libc::c_void,
                         (*set).count.wrapping_add(1 as i32 as
                                                       usize).wrapping_mul(::std::mem::size_of::<gss_buffer_desc>()
                                                                                       as
                                                                                       usize))
            as *mut gss_buffer_desc;
    if (*set).elements.is_null() {
        *minor_status = 12 as i32 as OM_uint32;
        return (13 as usize as OM_uint32) << 16 as i32
    }
    p =
        &mut *(*set).elements.offset((*set).count as isize) as
            *mut gss_buffer_desc;
    (*p).value = gssalloc_malloc((*member_buffer).length);
    if (*p).value.is_null() {
        *minor_status = 12 as i32 as OM_uint32;
        return (13 as usize as OM_uint32) << 16 as i32
    }
    memcpy((*p).value, (*member_buffer).value, (*member_buffer).length);
    (*p).length = (*member_buffer).length;
    (*set).count = (*set).count.wrapping_add(1);
    *minor_status = 0 as i32 as OM_uint32;
    return 0 as i32 as OM_uint32;
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
#[c2rust::src_loc = "98:1"]
pub unsafe extern "C" fn generic_gss_release_buffer_set(mut minor_status:
                                                            *mut OM_uint32,
                                                        mut buffer_set:
                                                            *mut gss_buffer_set_t)
 -> OM_uint32 {
    let mut i: size_t = 0;
    let mut minor: OM_uint32 = 0;
    *minor_status = 0 as i32 as OM_uint32;
    if (*buffer_set).is_null() { return 0 as i32 as OM_uint32 }
    i = 0 as i32 as size_t;
    while i < (**buffer_set).count {
        generic_gss_release_buffer(&mut minor,
                                   &mut *(**buffer_set).elements.offset(i as
                                                                            isize));
        i = i.wrapping_add(1)
    }
    if !(**buffer_set).elements.is_null() {
        gssalloc_free((**buffer_set).elements as *mut libc::c_void);
        (**buffer_set).elements = 0 as *mut gss_buffer_desc
    }
    (**buffer_set).count = 0 as i32 as size_t;
    gssalloc_free(*buffer_set as *mut libc::c_void);
    *buffer_set = 0 as gss_buffer_set_t;
    return 0 as i32 as OM_uint32;
}
