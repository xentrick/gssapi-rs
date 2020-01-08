use ::libc;

pub mod gssapi_alloc_h {
    #[inline]

    pub unsafe extern "C" fn gssalloc_malloc(
        mut size: crate::stddef_h::size_t,
    ) -> *mut libc::c_void {
        return crate::stdlib::malloc(size);
    }
    /* not _WIN32 or DEBUG_GSSALLOC */
    #[inline]

    pub unsafe extern "C" fn gssalloc_strdup(mut str: *const i8) -> *mut i8 {
        let mut size = crate::stdlib::strlen(str).wrapping_add(1usize);
        let mut copy = gssalloc_malloc(size) as *mut i8;
        if !copy.is_null() {
            crate::stdlib::memcpy(copy as *mut libc::c_void, str as *const libc::c_void, size);
            *copy.offset(size.wrapping_sub(1usize) as isize) = '\u{0}' as i8
        }
        return copy;
    }
}
pub use crate::stddef_h::size_t;

pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::src::generic::util_buffer::gssapi_alloc_h::gssalloc_malloc;
pub use crate::src::generic::util_buffer::gssapi_alloc_h::gssalloc_strdup;

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

pub unsafe extern "C" fn gssint_g_make_string_buffer(
    mut str: *const i8,
    mut buffer: crate::gssapi_h::gss_buffer_t,
) -> i32 {
    if buffer.is_null() {
        return 1i32;
    }
    (*buffer).length = crate::stdlib::strlen(str);
    (*buffer).value = gssalloc_strdup(str) as *mut libc::c_void;
    if (*buffer).value.is_null() {
        (*buffer).length = 0usize;
        return 0i32;
    }
    return 1i32;
}
