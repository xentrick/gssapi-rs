use ::libc;

pub use crate::stddef_h::size_t;

pub use crate::gssapiP_generic_h::g_seqnum_state;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint64_t;

pub use crate::stdlib::uint32_t;
pub use crate::stdlib::uint64_t;
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct g_seqnum_state_st {
    pub do_replay: i32,
    pub do_sequence: i32,
    pub seqmask: crate::stdlib::uint64_t,
    pub base: crate::stdlib::uint64_t,
    pub next: crate::stdlib::uint64_t,
    pub recvmap: crate::stdlib::uint64_t,
}
#[no_mangle]

pub unsafe extern "C" fn gssint_g_seqstate_init(
    mut state_out: *mut crate::gssapiP_generic_h::g_seqnum_state,
    mut seqnum: crate::stdlib::uint64_t,
    mut do_replay: i32,
    mut do_sequence: i32,
    mut wide: i32,
) -> isize {
    let mut state = 0 as *mut g_seqnum_state_st;
    *state_out = 0 as crate::gssapiP_generic_h::g_seqnum_state;
    state = crate::stdlib::malloc(::std::mem::size_of::<g_seqnum_state_st>())
        as crate::gssapiP_generic_h::g_seqnum_state;
    if state.is_null() {
        return 12isize;
    }
    (*state).do_replay = do_replay;
    (*state).do_sequence = do_sequence;
    (*state).seqmask = if wide != 0 {
        18446744073709551615 as usize
    } else {
        4294967295u32 as usize
    };
    (*state).base = seqnum;
    (*state).recvmap = 0usize;
    (*state).next = (*state).recvmap;
    *state_out = state;
    return 0isize;
}
#[no_mangle]

pub unsafe extern "C" fn gssint_g_seqstate_check(
    mut state: crate::gssapiP_generic_h::g_seqnum_state,
    mut seqnum: crate::stdlib::uint64_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut rel_seqnum: crate::stdlib::uint64_t = 0;
    let mut offset: crate::stdlib::uint64_t = 0;
    let mut bit: crate::stdlib::uint64_t = 0;
    if (*state).do_replay == 0 && (*state).do_sequence == 0 {
        return 0u32;
    }
    /* Use the difference from the base seqnum, to simplify wraparound. */
    rel_seqnum = seqnum.wrapping_sub((*state).base) & (*state).seqmask;
    if rel_seqnum >= (*state).next {
        /* seqnum is the expected sequence number or in the future.  Update the
         * received bitmap and expected next sequence number. */
        offset = rel_seqnum.wrapping_sub((*state).next);
        (*state).recvmap = (*state).recvmap << offset.wrapping_add(1usize) | 1usize;
        (*state).next = rel_seqnum.wrapping_add(1usize) & (*state).seqmask;
        return if offset > 0usize && (*state).do_sequence != 0 {
            (1i32) << 0i32 + 4i32
        } else {
            0i32
        } as crate::gssapi_h::OM_uint32;
    }
    /* seqnum is in the past.  Check if it's too old for replay detection. */
    offset = (*state).next.wrapping_sub(rel_seqnum);
    if offset > 64usize {
        return if (*state).do_sequence != 0 {
            (1i32) << 0i32 + 3i32
        } else {
            (1i32) << 0i32 + 2i32
        } as crate::gssapi_h::OM_uint32;
    }
    /* Check for replay and mark as received. */
    bit = (1usize) << offset.wrapping_sub(1usize);
    if (*state).do_replay != 0 && (*state).recvmap & bit != 0 {
        return ((1i32) << 0i32 + 1i32) as crate::gssapi_h::OM_uint32;
    }
    (*state).recvmap |= bit;
    return if (*state).do_sequence != 0 {
        (1i32) << 0i32 + 3i32
    } else {
        0i32
    } as crate::gssapi_h::OM_uint32;
}
#[no_mangle]

pub unsafe extern "C" fn gssint_g_seqstate_free(
    mut state: crate::gssapiP_generic_h::g_seqnum_state,
) {
    crate::stdlib::free(state as *mut libc::c_void);
}
/*
 * These support functions are for the serialization routines
 */
#[no_mangle]

pub unsafe extern "C" fn gssint_g_seqstate_size(
    mut _state: crate::gssapiP_generic_h::g_seqnum_state,
    mut sizep: *mut crate::stddef_h::size_t,
) {
    *sizep = (*sizep).wrapping_add(::std::mem::size_of::<g_seqnum_state_st>());
}
#[no_mangle]

pub unsafe extern "C" fn gssint_g_seqstate_externalize(
    mut state: crate::gssapiP_generic_h::g_seqnum_state,
    mut buf: *mut *mut u8,
    mut lenremain: *mut crate::stddef_h::size_t,
) -> isize {
    if *lenremain < ::std::mem::size_of::<g_seqnum_state_st>() {
        return 12isize;
    }
    crate::stdlib::memcpy(
        *buf as *mut libc::c_void,
        state as *const libc::c_void,
        ::std::mem::size_of::<g_seqnum_state_st>(),
    );
    *buf = (*buf).offset(::std::mem::size_of::<g_seqnum_state_st>() as isize);
    *lenremain = (*lenremain).wrapping_sub(::std::mem::size_of::<g_seqnum_state_st>());
    return 0isize;
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

pub unsafe extern "C" fn gssint_g_seqstate_internalize(
    mut state_out: *mut crate::gssapiP_generic_h::g_seqnum_state,
    mut buf: *mut *mut u8,
    mut lenremain: *mut crate::stddef_h::size_t,
) -> isize {
    let mut state = 0 as *mut g_seqnum_state_st;
    *state_out = 0 as crate::gssapiP_generic_h::g_seqnum_state;
    if *lenremain < ::std::mem::size_of::<g_seqnum_state_st>() {
        return 22isize;
    }
    state = crate::stdlib::malloc(::std::mem::size_of::<g_seqnum_state_st>())
        as crate::gssapiP_generic_h::g_seqnum_state;
    if state.is_null() {
        return 12isize;
    }
    crate::stdlib::memcpy(
        state as *mut libc::c_void,
        *buf as *const libc::c_void,
        ::std::mem::size_of::<g_seqnum_state_st>(),
    );
    *buf = (*buf).offset(::std::mem::size_of::<g_seqnum_state_st>() as isize);
    *lenremain = (*lenremain).wrapping_sub(::std::mem::size_of::<g_seqnum_state_st>());
    *state_out = state;
    return 0isize;
}
