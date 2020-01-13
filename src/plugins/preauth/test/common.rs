use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:33"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:33"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:33"]
pub mod krb5_h {
    /* typedef struct _profile_t *profile_t; */
    /*
 * begin wordsize.h
 */
    /*
 * Word-size related definition.
 */
    #[c2rust::src_loc = "136:1"]
    pub type krb5_octet = uint8_t;
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "185:1"]
    pub type krb5_preauthtype = krb5_int32;
    /* *
 * Used to convey an operation status.  The value 0 indicates success; any
 * other values are com_err codes.  Use krb5_get_error_message() to obtain a
 * string describing the error.
 */
    #[c2rust::src_loc = "204:1"]
    pub type krb5_error_code = krb5_int32;
    #[c2rust::src_loc = "206:1"]
    pub type krb5_magic = krb5_error_code;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "208:16"]
    pub struct _krb5_data {
        pub magic: krb5_magic,
        pub length: libc::c_uint,
        pub data: *mut libc::c_char,
    }
    #[c2rust::src_loc = "208:1"]
    pub type krb5_data = _krb5_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2038:16"]
    pub struct _krb5_pa_data {
        pub magic: krb5_magic,
        pub pa_type: krb5_preauthtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* * Pre-authentication data */
    #[c2rust::src_loc = "2038:1"]
    pub type krb5_pa_data = _krb5_pa_data;
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
    /* *< Preauthentication data type */
    /* *< Length of data */
    /* *< Data */
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:33"]
pub mod k5_int_h {
    #[inline]
    #[c2rust::src_loc = "2315:1"]
    pub unsafe extern "C" fn k5memdup(mut in_0: *const libc::c_void,
                                      mut len: size_t,
                                      mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void = k5alloc(len, code);
        if !ptr.is_null() && len > 0 as libc::c_int as libc::c_ulong {
            memcpy(ptr, in_0, len);
        }
        return ptr;
    }
    #[inline]
    #[c2rust::src_loc = "2308:1"]
    pub unsafe extern "C" fn k5alloc(mut size: size_t,
                                     mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        return k5calloc(1 as libc::c_int as size_t, size, code);
    }
    #[inline]
    #[c2rust::src_loc = "2296:1"]
    pub unsafe extern "C" fn k5calloc(mut nmemb: size_t, mut size: size_t,
                                      mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
        ptr =
            calloc(if nmemb != 0 {
                       nmemb
                   } else { 1 as libc::c_int as libc::c_ulong },
                   if size != 0 {
                       size
                   } else { 1 as libc::c_int as libc::c_ulong });
        *code =
            if ptr.is_null() { 12 as libc::c_int } else { 0 as libc::c_int };
        return ptr;
    }
    use super::stddef_h::size_t;
    use super::krb5_h::krb5_error_code;
    use super::string_h::memcpy;
    use super::stdlib_h::calloc;
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
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
#[c2rust::header_src = "/usr/include/assert.h:33"]
pub mod assert_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn __assert_fail(__assertion: *const libc::c_char,
                             __file: *const libc::c_char,
                             __line: libc::c_uint,
                             __function: *const libc::c_char) -> !;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:33"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
pub use self::types_h::{__uint8_t, __int32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint8_t;
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_preauthtype,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       _krb5_pa_data, krb5_pa_data};
pub use self::k5_int_h::{k5memdup, k5alloc, k5calloc};
use self::string_h::memcpy;
use self::assert_h::__assert_fail;
use self::stdlib_h::calloc;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* plugins/preauth/test/common.c - common functions for test preauth module */
/*
 * Copyright (C) 2017 by the Massachusetts Institute of Technology.
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
#[no_mangle]
#[c2rust::src_loc = "36:1"]
pub unsafe extern "C" fn make_pa(mut contents: *const libc::c_char,
                                 mut len: size_t) -> *mut krb5_pa_data {
    let mut ret: krb5_error_code = 0;
    let mut pa: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    pa =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<krb5_pa_data>() as libc::c_ulong) as
            *mut krb5_pa_data;
    if !pa.is_null() {
    } else {
        __assert_fail(b"pa != NULL\x00" as *const u8 as *const libc::c_char,
                      b"common.c\x00" as *const u8 as *const libc::c_char,
                      43 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 44],
                                                &[libc::c_char; 44]>(b"krb5_pa_data *make_pa(const char *, size_t)\x00")).as_ptr());
    }
    (*pa).pa_type = -(123 as libc::c_int);
    (*pa).contents =
        k5memdup(contents as *const libc::c_void, len, &mut ret) as
            *mut krb5_octet;
    if ret == 0 {
    } else {
        __assert_fail(b"!ret\x00" as *const u8 as *const libc::c_char,
                      b"common.c\x00" as *const u8 as *const libc::c_char,
                      46 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 44],
                                                &[libc::c_char; 44]>(b"krb5_pa_data *make_pa(const char *, size_t)\x00")).as_ptr());
    }
    (*pa).length = len as libc::c_uint;
    return pa;
}
/* Make a one-element padata list of type TEST_PA_TYPE. */
#[no_mangle]
#[c2rust::src_loc = "52:1"]
pub unsafe extern "C" fn make_pa_list(mut contents: *const libc::c_char,
                                      mut len: size_t)
 -> *mut *mut krb5_pa_data {
    let mut list: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    list =
        calloc(2 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<*mut krb5_pa_data>() as libc::c_ulong) as
            *mut *mut krb5_pa_data;
    if !list.is_null() {
    } else {
        __assert_fail(b"list != NULL\x00" as *const u8 as *const libc::c_char,
                      b"common.c\x00" as *const u8 as *const libc::c_char,
                      58 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 50],
                                                &[libc::c_char; 50]>(b"krb5_pa_data **make_pa_list(const char *, size_t)\x00")).as_ptr());
    }
    let ref mut fresh0 = *list.offset(0 as libc::c_int as isize);
    *fresh0 = make_pa(contents, len);
    return list;
}
