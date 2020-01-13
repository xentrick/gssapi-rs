use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:28"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:28"]
pub mod gssapi_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    use super::stddef_h::size_t;
    /* _GSSAPI_H_ */
}
#[c2rust::header_src = "/usr/include/stdlib.h:28"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/string.h:28"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_alloc.h:28"]
pub mod gssapi_alloc_h {
    #[inline]
    #[c2rust::src_loc = "99:1"]
    pub unsafe extern "C" fn gssalloc_malloc(mut size: size_t)
     -> *mut libc::c_void {
        return malloc(size);
    }
    /* not _WIN32 or DEBUG_GSSALLOC */
    #[inline]
    #[c2rust::src_loc = "119:1"]
    pub unsafe extern "C" fn gssalloc_strdup(mut str: *const libc::c_char)
     -> *mut libc::c_char {
        let mut size: size_t =
            strlen(str).wrapping_add(1 as libc::c_int as libc::c_ulong);
        let mut copy: *mut libc::c_char =
            gssalloc_malloc(size) as *mut libc::c_char;
        if !copy.is_null() {
            memcpy(copy as *mut libc::c_void, str as *const libc::c_void,
                   size);
            *copy.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                             as isize) = '\u{0}' as i32 as libc::c_char
        }
        return copy;
    }
    use super::stddef_h::size_t;
    use super::stdlib_h::malloc;
    use super::string_h::{strlen, memcpy};
}
pub use self::stddef_h::size_t;
pub use self::gssapi_h::{gss_buffer_desc_struct, gss_buffer_t};
use self::stdlib_h::malloc;
use self::string_h::{memcpy, strlen};
pub use self::gssapi_alloc_h::{gssalloc_malloc, gssalloc_strdup};
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
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
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
/* return nonzero on success, 0 on failure
   make sure that buffer is consistent (release'able) when this
   function exits, no matter what the exit value */
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn gssint_g_make_string_buffer(mut str:
                                                         *const libc::c_char,
                                                     mut buffer: gss_buffer_t)
 -> libc::c_int {
    if buffer.is_null() { return 1 as libc::c_int }
    (*buffer).length = strlen(str);
    (*buffer).value = gssalloc_strdup(str) as *mut libc::c_void;
    if (*buffer).value.is_null() {
        (*buffer).length = 0 as libc::c_int as size_t;
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
