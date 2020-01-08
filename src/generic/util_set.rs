use ::libc;

pub use crate::gssapiP_generic_h::g_set_elt;

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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _g_set_elt {
    pub key: *mut libc::c_void,
    pub value: *mut libc::c_void,
    pub next: *mut _g_set_elt,
}
#[no_mangle]

pub unsafe extern "C" fn gssint_g_set_init(mut s: *mut crate::gssapiP_generic_h::g_set_elt) -> i32 {
    *s = 0 as crate::gssapiP_generic_h::g_set_elt;
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn gssint_g_set_entry_add(
    mut s: *mut crate::gssapiP_generic_h::g_set_elt,
    mut key: *mut libc::c_void,
    mut value: *mut libc::c_void,
) -> i32 {
    let mut first = 0 as *mut _g_set_elt;
    first = crate::stdlib::malloc(::std::mem::size_of::<_g_set_elt>()) as *mut _g_set_elt;
    if first.is_null() {
        return 12i32;
    }
    (*first).key = key;
    (*first).value = value;
    (*first).next = *s;
    *s = first;
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn gssint_g_set_entry_delete(
    mut s: *mut crate::gssapiP_generic_h::g_set_elt,
    mut key: *mut libc::c_void,
) -> i32 {
    let mut p = 0 as *mut crate::gssapiP_generic_h::g_set_elt;
    p = s;
    while !(*p).is_null() {
        if (**p).key == key {
            let mut next = (**p).next;
            crate::stdlib::free(*p as *mut libc::c_void);
            *p = next;
            return 0i32;
        }
        p = &mut (**p).next
    }
    return -(1i32);
}
#[no_mangle]

pub unsafe extern "C" fn gssint_g_set_entry_get(
    mut s: *mut crate::gssapiP_generic_h::g_set_elt,
    mut key: *mut libc::c_void,
    mut value: *mut *mut libc::c_void,
) -> i32 {
    let mut p = 0 as *mut _g_set_elt;
    p = *s;
    while !p.is_null() {
        if (*p).key == key {
            *value = (*p).value;
            return 0i32;
        }
        p = (*p).next
    }
    *value = 0 as *mut libc::c_void;
    return -(1i32);
}
