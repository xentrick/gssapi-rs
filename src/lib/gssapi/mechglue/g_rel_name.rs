use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:29"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:29"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:29"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/mechglue/mglueP.h:29"]
pub mod mglueP_h {
    /* lib/gssapi/mechglue/mglueP.h */
    /*
 * Copyright (c) 1995, by Sun Microsystems, Inc.
 * All rights reserved.
 */
    /* This header contains the private mechglue definitions. */
    /*
 * Array of context IDs typed by mechanism OID
 */
    /*
 * Generic GSSAPI names.  A name can either be a generic name, or a
 * mechanism specific name....
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:16"]
    pub struct gss_name_struct {
        pub loopback: *mut gss_name_struct,
        pub name_type: gss_OID,
        pub external_name: gss_buffer_t,
        pub mech_type: gss_OID,
        pub mech_name: gss_name_t,
    }
    #[c2rust::src_loc = "36:1"]
    pub type gss_union_name_t = *mut gss_name_struct;
    use super::gssapi_h::{gss_OID, gss_buffer_t, gss_name_t, OM_uint32,
                          gss_OID_desc_struct};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "779:1"]
        pub fn gssint_release_internal_name(_: *mut OM_uint32, _: gss_OID,
                                            _: *mut gss_name_t) -> OM_uint32;
    }
    /* _GSS_MECHGLUEP_H */
    /* Use this to map an errno value or com_err error code being
   generated within the mechglue code (e.g., by calling generic oid
   ops).  Any errno or com_err values produced by mech operations
   should be processed with map_error.  This means they'll be stored
   separately even if the mech uses com_err, because we can't assume
   that it will use com_err.  */
    /* Use this to map an error code that was returned from a mech
   operation; the mech will be asked to produce the associated error
   messages.

   Remember that if the minor status code cannot be returned to the
   caller (e.g., if it's stuffed in an automatic variable and then
   ignored), then we don't care about producing a mapping.  */
    /* qop_state */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:29"]
pub mod gssapi_h {
    #[c2rust::src_loc = "79:1"]
    pub type gss_name_t = *mut gss_name_struct;
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "106:16"]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    use super::mglueP_h::gss_name_struct;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "673:1"]
        pub fn gss_release_oid(_: *mut OM_uint32, _: *mut gss_OID)
         -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src = "/usr/include/stdlib.h:29"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_alloc.h:29"]
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
pub use self::types_h::__uint32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::stddef_h::size_t;
pub use self::mglueP_h::{gss_name_struct, gss_union_name_t,
                         gssint_release_internal_name};
pub use self::gssapi_h::{gss_name_t, gss_OID, gss_OID_desc_struct, OM_uint32,
                         gss_uint32, gss_buffer_t, gss_buffer_desc_struct,
                         gss_release_oid};
use self::stdlib_h::free;
pub use self::gssapi_alloc_h::gssalloc_free;
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
#[c2rust::src_loc = "36:1"]
pub unsafe extern "C" fn gss_release_name(mut minor_status: *mut OM_uint32,
                                          mut input_name: *mut gss_name_t)
 -> OM_uint32 {
    let mut union_name: gss_union_name_t = 0 as *mut gss_name_struct;
    if minor_status.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    *minor_status = 0 as libc::c_int as OM_uint32;
    /* if input_name is NULL, return error */
    if input_name.is_null() {
        return (1 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                   (2 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if (*input_name).is_null() { return 0 as libc::c_int as OM_uint32 }
    /*
     * free up the space for the external_name and then
     * free the union_name descriptor
     */
    union_name = *input_name as gss_union_name_t;
    if !(!union_name.is_null() && (*union_name).loopback == union_name) {
        return (1 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                   (2 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    *input_name = 0 as gss_name_t;
    *minor_status = 0 as libc::c_int as OM_uint32;
    if !(*union_name).name_type.is_null() {
        gss_release_oid(minor_status, &mut (*union_name).name_type);
    }
    if !(*union_name).external_name.is_null() {
        if !(*(*union_name).external_name).value.is_null() {
            gssalloc_free((*(*union_name).external_name).value);
        }
        free((*union_name).external_name as *mut libc::c_void);
    }
    if !(*union_name).mech_type.is_null() {
        gssint_release_internal_name(minor_status, (*union_name).mech_type,
                                     &mut (*union_name).mech_name);
        gss_release_oid(minor_status, &mut (*union_name).mech_type);
    }
    free(union_name as *mut libc::c_void);
    return 0 as libc::c_int as OM_uint32;
}
