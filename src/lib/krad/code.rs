use ::libc;
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krad.h:30"]
pub mod krad_h {
    #[c2rust::src_loc = "63:1"]
    pub type krad_code = libc::c_uchar;
    /* KRAD_H_ */
}
#[c2rust::header_src = "/usr/include/string.h:30"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
pub use self::krad_h::krad_code;
use self::string_h::strcmp;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krad/code.c - RADIUS code name table for libkrad */
/*
 * Copyright 2013 Red Hat, Inc.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *    1. Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *
 *    2. Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
 * IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
 * OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
#[c2rust::src_loc = "34:20"]
static mut codes: [*const libc::c_char; 255] =
    [b"Access-Request\x00" as *const u8 as *const libc::c_char,
     b"Access-Accept\x00" as *const u8 as *const libc::c_char,
     b"Access-Reject\x00" as *const u8 as *const libc::c_char,
     b"Accounting-Request\x00" as *const u8 as *const libc::c_char,
     b"Accounting-Response\x00" as *const u8 as *const libc::c_char,
     b"Accounting-Status\x00" as *const u8 as *const libc::c_char,
     b"Password-Request\x00" as *const u8 as *const libc::c_char,
     b"Password-Ack\x00" as *const u8 as *const libc::c_char,
     b"Password-Reject\x00" as *const u8 as *const libc::c_char,
     b"Accounting-Message\x00" as *const u8 as *const libc::c_char,
     b"Access-Challenge\x00" as *const u8 as *const libc::c_char,
     b"Status-Server\x00" as *const u8 as *const libc::c_char,
     b"Status-Client\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char,
     b"Resource-Free-Request\x00" as *const u8 as *const libc::c_char,
     b"Resource-Free-Response\x00" as *const u8 as *const libc::c_char,
     b"Resource-Query-Request\x00" as *const u8 as *const libc::c_char,
     b"Resource-Query-Response\x00" as *const u8 as *const libc::c_char,
     b"Alternate-Resource-Reclaim-Request\x00" as *const u8 as
         *const libc::c_char,
     b"NAS-Reboot-Request\x00" as *const u8 as *const libc::c_char,
     b"NAS-Reboot-Response\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char,
     b"Next-Passcode\x00" as *const u8 as *const libc::c_char,
     b"New-Pin\x00" as *const u8 as *const libc::c_char,
     b"Terminate-Session\x00" as *const u8 as *const libc::c_char,
     b"Password-Expired\x00" as *const u8 as *const libc::c_char,
     b"Event-Request\x00" as *const u8 as *const libc::c_char,
     b"Event-Response\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char,
     b"Disconnect-Request\x00" as *const u8 as *const libc::c_char,
     b"Disconnect-Ack\x00" as *const u8 as *const libc::c_char,
     b"Disconnect-Nak\x00" as *const u8 as *const libc::c_char,
     b"Change-Filters-Request\x00" as *const u8 as *const libc::c_char,
     b"Change-Filters-Ack\x00" as *const u8 as *const libc::c_char,
     b"Change-Filters-Nak\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     b"IP-Address-Allocate\x00" as *const u8 as *const libc::c_char,
     b"IP-Address-Release\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char];
#[no_mangle]
#[c2rust::src_loc = "88:1"]
pub unsafe extern "C" fn krad_code_name2num(mut name: *const libc::c_char)
 -> krad_code {
    let mut i: libc::c_uchar = 0;
    i = 0 as libc::c_int as libc::c_uchar;
    while (i as libc::c_int) <
              127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int {
        if !codes[i as usize].is_null() {
            if strcmp(codes[i as usize], name) == 0 as libc::c_int {
                i = i.wrapping_add(1);
                return i
            }
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int as krad_code;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2013 Red Hat, Inc.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *    1. Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *
 *    2. Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
 * IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
 * OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/*
 * This API is not considered as stable as the main krb5 API.
 *
 * - We may make arbitrary incompatible changes between feature releases
 *   (e.g. from 1.12 to 1.13).
 * - We will make some effort to avoid making incompatible changes for
 *   bugfix releases, but will make them if necessary.
 */
/* Called when a response is received or the request times out. */
/*
 * Called to iterate over a set of requests.  Either the callback will be
 * called until it returns NULL, or it will be called with cancel = TRUE to
 * terminate in the middle of an iteration.
 */
/*
 * Code
 */
/* Convert a code name to its number. Only works for codes defined
 * by RFC 2875 or 2882. Returns 0 if the name was not found. */
/* Convert a code number to its name. Only works for attributes defined
 * by RFC 2865 or 2882. Returns NULL if the name was not found. */
#[no_mangle]
#[c2rust::src_loc = "104:1"]
pub unsafe extern "C" fn krad_code_num2name(mut code: krad_code)
 -> *const libc::c_char {
    if code as libc::c_int == 0 as libc::c_int {
        return 0 as *const libc::c_char
    }
    return codes[(code as libc::c_int - 1 as libc::c_int) as usize];
}
