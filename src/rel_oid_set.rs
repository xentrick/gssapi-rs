use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:30"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/types.h:30"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = u32;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:30"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:30"]
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
    #[c2rust::src_loc = "106:16"]
    #[derive(Copy, Clone)]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    /* OM_STRING */

    #[repr(C)]
    #[c2rust::src_loc = "112:16"]
    #[derive(Copy, Clone)]
    pub struct gss_OID_set_desc_struct {
        pub count: usize,
        pub elements: gss_OID,
    }
    #[c2rust::src_loc = "112:1"]
    pub type gss_OID_set = *mut gss_OID_set_desc_struct;
    use super::stddef_h::size_t;
    use super::stdint_uintn_h::uint32_t;
    /* _GSSAPI_H_ */
}
#[c2rust::header_src = "/usr/include/stdlib.h:30"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_alloc.h:30"]
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
    use super::stdlib_h::free;
}
pub use self::gssapi_alloc_h::gssalloc_free;
pub use self::gssapi_h::gss_OID;
pub use self::gssapi_h::gss_OID_desc_struct;
pub use self::gssapi_h::gss_OID_set;
pub use self::gssapi_h::gss_OID_set_desc_struct;
pub use self::gssapi_h::gss_uint32;
pub use self::gssapi_h::OM_uint32;
pub use self::stddef_h::size_t;
pub use self::stdint_uintn_h::uint32_t;
use self::stdlib_h::free;
pub use self::types_h::__uint32_t;
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
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* #ident  "@(#)gss_release_oid_set.c 1.12     95/08/23 SMI" */
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
 *  glue routine for gss_release_oid_set
 */
#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn generic_gss_release_oid_set(
    mut minor_status: *mut OM_uint32,
    mut set: *mut gss_OID_set,
) -> OM_uint32 {
    let mut i = 0;
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if set.is_null() {
        return 0u32;
    }
    if (*set).is_null() {
        return 0u32;
    }
    i = 0usize;
    while i < (**set).count {
        gssalloc_free((*(**set).elements.offset(i as isize)).elements);
        i = i.wrapping_add(1)
    }
    gssalloc_free((**set).elements as *mut libc::c_void);
    gssalloc_free(*set as *mut libc::c_void);
    *set = 0 as gss_OID_set;
    return 0u32;
}
