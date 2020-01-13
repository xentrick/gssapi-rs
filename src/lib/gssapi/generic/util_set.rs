use ::libc;
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapiP_generic.h:28"]
pub mod gssapiP_generic_h {
    #[c2rust::src_loc = "122:1"]
    pub type g_set_elt = *mut _g_set_elt;
    use super::_g_set_elt;
    /* _GSSAPIP_GENERIC_H_ */
}
#[c2rust::header_src = "/usr/include/stdlib.h:28"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
pub use self::gssapiP_generic_h::g_set_elt;
use self::stdlib_h::{malloc, free};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1995 by OpenVision Technologies, Inc.
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
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "30:8"]
pub struct _g_set_elt {
    pub key: *mut libc::c_void,
    pub value: *mut libc::c_void,
    pub next: *mut _g_set_elt,
}
#[no_mangle]
#[c2rust::src_loc = "36:1"]
pub unsafe extern "C" fn gssint_g_set_init(mut s: *mut g_set_elt)
 -> libc::c_int {
    *s = 0 as g_set_elt;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "43:1"]
pub unsafe extern "C" fn gssint_g_set_entry_add(mut s: *mut g_set_elt,
                                                mut key: *mut libc::c_void,
                                                mut value: *mut libc::c_void)
 -> libc::c_int {
    let mut first: g_set_elt = 0 as *mut _g_set_elt;
    first =
        malloc(::std::mem::size_of::<_g_set_elt>() as libc::c_ulong) as
            *mut _g_set_elt;
    if first.is_null() { return 12 as libc::c_int }
    (*first).key = key;
    (*first).value = value;
    (*first).next = *s;
    *s = first;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "59:1"]
pub unsafe extern "C" fn gssint_g_set_entry_delete(mut s: *mut g_set_elt,
                                                   mut key: *mut libc::c_void)
 -> libc::c_int {
    let mut p: *mut g_set_elt = 0 as *mut g_set_elt;
    p = s;
    while !(*p).is_null() {
        if (**p).key == key {
            let mut next: g_set_elt = (**p).next;
            free(*p as *mut libc::c_void);
            *p = next;
            return 0 as libc::c_int
        }
        p = &mut (**p).next
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "76:1"]
pub unsafe extern "C" fn gssint_g_set_entry_get(mut s: *mut g_set_elt,
                                                mut key: *mut libc::c_void,
                                                mut value:
                                                    *mut *mut libc::c_void)
 -> libc::c_int {
    let mut p: g_set_elt = 0 as *mut _g_set_elt;
    p = *s;
    while !p.is_null() {
        if (*p).key == key { *value = (*p).value; return 0 as libc::c_int }
        p = (*p).next
    }
    *value = 0 as *mut libc::c_void;
    return -(1 as libc::c_int);
}
