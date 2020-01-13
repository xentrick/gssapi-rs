use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:33"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint32_t, __uint64_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:33"]
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
    use super::stdint_uintn_h::uint32_t;
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapiP_generic.h:33"]
pub mod gssapiP_generic_h {
    #[c2rust::src_loc = "129:1"]
    pub type g_seqnum_state = *mut g_seqnum_state_st;
    use super::g_seqnum_state_st;
    /* _GSSAPIP_GENERIC_H_ */
}
#[c2rust::header_src = "/usr/include/stdlib.h:33"]
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
#[c2rust::header_src = "/usr/include/string.h:33"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint32_t, __uint64_t};
pub use self::stdint_uintn_h::{uint32_t, uint64_t};
pub use self::gssapi_h::{gss_uint32, OM_uint32};
pub use self::gssapiP_generic_h::g_seqnum_state;
use self::stdlib_h::{malloc, free};
use self::string_h::memcpy;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/gssapi/generic/util_seqstate.c - sequence number checking */
/*
 * Copyright (C) 2014 by the Massachusetts Institute of Technology.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * * Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.
 *
 * * Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in
 *   the documentation and/or other materials provided with the
 *   distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "36:8"]
pub struct g_seqnum_state_st {
    pub do_replay: libc::c_int,
    pub do_sequence: libc::c_int,
    pub seqmask: uint64_t,
    pub base: uint64_t,
    pub next: uint64_t,
    pub recvmap: uint64_t,
}
#[no_mangle]
#[c2rust::src_loc = "64:1"]
pub unsafe extern "C" fn gssint_g_seqstate_init(mut state_out:
                                                    *mut g_seqnum_state,
                                                mut seqnum: uint64_t,
                                                mut do_replay: libc::c_int,
                                                mut do_sequence: libc::c_int,
                                                mut wide: libc::c_int)
 -> libc::c_long {
    let mut state: g_seqnum_state = 0 as *mut g_seqnum_state_st;
    *state_out = 0 as g_seqnum_state;
    state =
        malloc(::std::mem::size_of::<g_seqnum_state_st>() as libc::c_ulong) as
            g_seqnum_state;
    if state.is_null() { return 12 as libc::c_int as libc::c_long }
    (*state).do_replay = do_replay;
    (*state).do_sequence = do_sequence;
    (*state).seqmask =
        if wide != 0 {
            18446744073709551615 as libc::c_ulong
        } else { 4294967295 as libc::c_uint as libc::c_ulong };
    (*state).base = seqnum;
    (*state).recvmap = 0 as libc::c_int as uint64_t;
    (*state).next = (*state).recvmap;
    *state_out = state;
    return 0 as libc::c_int as libc::c_long;
}
#[no_mangle]
#[c2rust::src_loc = "83:1"]
pub unsafe extern "C" fn gssint_g_seqstate_check(mut state: g_seqnum_state,
                                                 mut seqnum: uint64_t)
 -> OM_uint32 {
    let mut rel_seqnum: uint64_t = 0;
    let mut offset: uint64_t = 0;
    let mut bit: uint64_t = 0;
    if (*state).do_replay == 0 && (*state).do_sequence == 0 {
        return 0 as libc::c_int as OM_uint32
    }
    /* Use the difference from the base seqnum, to simplify wraparound. */
    rel_seqnum = seqnum.wrapping_sub((*state).base) & (*state).seqmask;
    if rel_seqnum >= (*state).next {
        /* seqnum is the expected sequence number or in the future.  Update the
         * received bitmap and expected next sequence number. */
        offset = rel_seqnum.wrapping_sub((*state).next);
        (*state).recvmap =
            (*state).recvmap <<
                offset.wrapping_add(1 as libc::c_int as libc::c_ulong) |
                1 as libc::c_int as libc::c_ulong;
        (*state).next =
            rel_seqnum.wrapping_add(1 as libc::c_int as libc::c_ulong) &
                (*state).seqmask;
        return if offset > 0 as libc::c_int as libc::c_ulong &&
                      (*state).do_sequence != 0 {
                   ((1 as libc::c_int)) << 0 as libc::c_int + 4 as libc::c_int
               } else { 0 as libc::c_int } as OM_uint32
    }
    /* seqnum is in the past.  Check if it's too old for replay detection. */
    offset = (*state).next.wrapping_sub(rel_seqnum);
    if offset > 64 as libc::c_int as libc::c_ulong {
        return if (*state).do_sequence != 0 {
                   ((1 as libc::c_int)) << 0 as libc::c_int + 3 as libc::c_int
               } else {
                   ((1 as libc::c_int)) << 0 as libc::c_int + 2 as libc::c_int
               } as OM_uint32
    }
    /* Check for replay and mark as received. */
    bit =
        (1 as libc::c_int as uint64_t) <<
            offset.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if (*state).do_replay != 0 && (*state).recvmap & bit != 0 {
        return ((1 as libc::c_int) << 0 as libc::c_int + 1 as libc::c_int) as
                   OM_uint32
    }
    (*state).recvmap |= bit;
    return if (*state).do_sequence != 0 {
               ((1 as libc::c_int)) << 0 as libc::c_int + 3 as libc::c_int
           } else { 0 as libc::c_int } as OM_uint32;
}
#[no_mangle]
#[c2rust::src_loc = "119:1"]
pub unsafe extern "C" fn gssint_g_seqstate_free(mut state: g_seqnum_state) {
    free(state as *mut libc::c_void);
}
/*
 * These support functions are for the serialization routines
 */
#[no_mangle]
#[c2rust::src_loc = "128:1"]
pub unsafe extern "C" fn gssint_g_seqstate_size(mut state: g_seqnum_state,
                                                mut sizep: *mut size_t) {
    *sizep =
        (*sizep as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<g_seqnum_state_st>()
                                             as libc::c_ulong) as size_t as
            size_t;
}
#[no_mangle]
#[c2rust::src_loc = "134:1"]
pub unsafe extern "C" fn gssint_g_seqstate_externalize(mut state:
                                                           g_seqnum_state,
                                                       mut buf:
                                                           *mut *mut libc::c_uchar,
                                                       mut lenremain:
                                                           *mut size_t)
 -> libc::c_long {
    if *lenremain <
           ::std::mem::size_of::<g_seqnum_state_st>() as libc::c_ulong {
        return 12 as libc::c_int as libc::c_long
    }
    memcpy(*buf as *mut libc::c_void, state as *const libc::c_void,
           ::std::mem::size_of::<g_seqnum_state_st>() as libc::c_ulong);
    *buf =
        (*buf).offset(::std::mem::size_of::<g_seqnum_state_st>() as
                          libc::c_ulong as isize);
    *lenremain =
        (*lenremain as
             libc::c_ulong).wrapping_sub(::std::mem::size_of::<g_seqnum_state_st>()
                                             as libc::c_ulong) as size_t as
            size_t;
    return 0 as libc::c_int as libc::c_long;
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
#[no_mangle]
#[c2rust::src_loc = "146:1"]
pub unsafe extern "C" fn gssint_g_seqstate_internalize(mut state_out:
                                                           *mut g_seqnum_state,
                                                       mut buf:
                                                           *mut *mut libc::c_uchar,
                                                       mut lenremain:
                                                           *mut size_t)
 -> libc::c_long {
    let mut state: g_seqnum_state = 0 as *mut g_seqnum_state_st;
    *state_out = 0 as g_seqnum_state;
    if *lenremain <
           ::std::mem::size_of::<g_seqnum_state_st>() as libc::c_ulong {
        return 22 as libc::c_int as libc::c_long
    }
    state =
        malloc(::std::mem::size_of::<g_seqnum_state_st>() as libc::c_ulong) as
            g_seqnum_state;
    if state.is_null() { return 12 as libc::c_int as libc::c_long }
    memcpy(state as *mut libc::c_void, *buf as *const libc::c_void,
           ::std::mem::size_of::<g_seqnum_state_st>() as libc::c_ulong);
    *buf =
        (*buf).offset(::std::mem::size_of::<g_seqnum_state_st>() as
                          libc::c_ulong as isize);
    *lenremain =
        (*lenremain as
             libc::c_ulong).wrapping_sub(::std::mem::size_of::<g_seqnum_state_st>()
                                             as libc::c_ulong) as size_t as
            size_t;
    *state_out = state;
    return 0 as libc::c_int as libc::c_long;
}
