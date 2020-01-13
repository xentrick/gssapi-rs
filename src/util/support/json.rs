use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:73"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stdarg.h:73"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-json.h:75"]
pub mod k5_json_h {
    /*
 * The k5_json_value C type can represent any kind of JSON value.  It has no
 * static type safety since it is represented using a void pointer, so be
 * careful with it.  Its type can be checked dynamically with k5_json_get_tid()
 * and the above constants.
 */
    #[c2rust::src_loc = "86:1"]
    pub type k5_json_value = *mut libc::c_void;
    #[c2rust::src_loc = "87:1"]
    pub type k5_json_tid = libc::c_uint;
    /*
 * Object
 */
    #[c2rust::src_loc = "160:1"]
    pub type k5_json_object = *mut k5_json_object_st;
    /*
 * String
 */
    #[c2rust::src_loc = "186:1"]
    pub type k5_json_string = *mut k5_json_string_st;
    /*
 * Array
 */
    #[c2rust::src_loc = "128:1"]
    pub type k5_json_array = *mut k5_json_array_st;
    /*
 * Number
 */
    #[c2rust::src_loc = "206:1"]
    pub type k5_json_number = *mut k5_json_number_st;
    /*
 * If a k5_json_* function can fail, it returns 0 on success and an errno value
 * on failure.
 */
    /*
 * Null
 */
    #[c2rust::src_loc = "108:1"]
    pub type k5_json_null = *mut k5_json_null_st;
    /*
 * Boolean
 */
    #[c2rust::src_loc = "119:1"]
    pub type k5_json_bool = *mut k5_json_bool_st;
    #[c2rust::src_loc = "161:1"]
    pub type k5_json_object_iterator_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *const libc::c_char, _: k5_json_value)
                   -> ()>;
    use super::{k5_json_object_st, k5_json_array_st};
    extern "C" {
        #[c2rust::src_loc = "186:16"]
        pub type k5_json_string_st;
        #[c2rust::src_loc = "206:16"]
        pub type k5_json_number_st;
        #[c2rust::src_loc = "108:16"]
        pub type k5_json_null_st;
        #[c2rust::src_loc = "119:16"]
        pub type k5_json_bool_st;
    }
    /* K5_JSON_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-buf.h:76"]
pub mod k5_buf_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:8"]
    pub struct k5buf {
        pub buftype: k5buftype,
        pub data: *mut libc::c_void,
        pub space: size_t,
        pub len: size_t,
    }
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-buf.h - k5buf interface declarations */
/*
 * Copyright 2008 Massachusetts Institute of Technology.
 * All Rights Reserved.
 *
 * Export of this software from the United States of America may
 *   require a specific license from the United States Government.
 *   It is the responsibility of any person or organization contemplating
 *   export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of M.I.T. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 * M.I.T. makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 */
    /*
 * The k5buf module is intended to allow multi-step string construction in a
 * fixed or dynamic buffer without the need to check for a failure at each step
 * (and without aborting on malloc failure).  If an allocation failure occurs
 * or the fixed buffer runs out of room, the buffer will be set to an error
 * state which can be detected with k5_buf_status.  Data in a buffer is
 * terminated with a zero byte so that it can be used as a C string.
 *
 * k5buf structures are usually stack-allocated.  Do not put k5buf structure
 * pointers into public APIs.  It is okay to reference the data and len fields
 * of a buffer (they will be NULL/0 if the buffer is in an error state), but do
 * not change them.
 */
    /* Buffer type values */
    #[c2rust::src_loc = "48:1"]
    pub type k5buftype = libc::c_uint;
    #[c2rust::src_loc = "48:59"]
    pub const K5BUF_DYNAMIC_ZAP: k5buftype = 3;
    #[c2rust::src_loc = "48:44"]
    pub const K5BUF_DYNAMIC: k5buftype = 2;
    #[c2rust::src_loc = "48:31"]
    pub const K5BUF_FIXED: k5buftype = 1;
    #[c2rust::src_loc = "48:18"]
    pub const K5BUF_ERROR: k5buftype = 0;
    use super::stddef_h::size_t;
    extern "C" {
        /* Initialize a k5buf using an internally allocated dynamic buffer. */
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn k5_buf_init_dynamic(buf: *mut k5buf);
        /* Add a C string to BUF. */
        #[no_mangle]
        #[c2rust::src_loc = "71:1"]
        pub fn k5_buf_add(buf: *mut k5buf, data: *const libc::c_char);
        /* Add a counted series of bytes to BUF. */
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn k5_buf_add_len(buf: *mut k5buf, data: *const libc::c_void,
                              len: size_t);
        /* Add sprintf-style formatted data to BUF. */
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn k5_buf_add_fmt(buf: *mut k5buf, fmt: *const libc::c_char,
                              _: ...);
        /* Return ENOMEM if buf is in an error state, 0 otherwise. */
        #[no_mangle]
        #[c2rust::src_loc = "100:1"]
        pub fn k5_buf_status(buf: *mut k5buf) -> libc::c_int;
        /*
 * Free the storage used in the dynamic buffer BUF.  The caller may choose to
 * take responsibility for freeing the data pointer instead of using this
 * function.  If BUF is a fixed buffer, an assertion failure will result.
 * Freeing a buffer in the error state, a buffer initialized with EMPTY_K5BUF,
 * or a zeroed k5buf structure is a no-op.
 */
        #[no_mangle]
        #[c2rust::src_loc = "109:1"]
        pub fn k5_buf_free(buf: *mut k5buf);
    }
    /* K5_BUF_H */
}
#[c2rust::header_src = "/usr/include/string.h:73"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "225:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "272:15"]
        pub fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:73"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:73"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-base64.h:74"]
pub mod k5_base64_h {
    use super::stddef_h::size_t;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-base64.h - base64 declarations */
/*
 * Copyright (c) 1995, 1996, 1997 Kungliga Tekniska Högskolan
 * (Royal Institute of Technology, Stockholm, Sweden).
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * 3. Neither the name of the Institute nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE INSTITUTE AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE INSTITUTE OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
        /* base64-encode data and return it in an allocated buffer.  Return NULL if out
 * of memory. */
        #[no_mangle]
        #[c2rust::src_loc = "43:1"]
        pub fn k5_base64_encode(data: *const libc::c_void, len: size_t)
         -> *mut libc::c_char;
        /*
 * Decode str as base64 and return the result in an allocated buffer, setting
 * *len_out to the length.  Return NULL and *len_out == 0 if out of memory,
 * NULL and *len_out == SIZE_MAX on invalid input.
 */
        #[no_mangle]
        #[c2rust::src_loc = "50:1"]
        pub fn k5_base64_decode(str: *const libc::c_char,
                                len_out: *mut size_t) -> *mut libc::c_void;
    }
    /* K5_BASE64_H */
}
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::stddef_h::size_t;
pub use self::stdarg_h::va_list;
pub use self::k5_json_h::{k5_json_value, k5_json_tid, k5_json_object,
                          k5_json_string, k5_json_array, k5_json_number,
                          k5_json_null, k5_json_bool,
                          k5_json_object_iterator_fn, k5_json_string_st,
                          k5_json_number_st, k5_json_null_st,
                          k5_json_bool_st};
pub use self::k5_buf_h::{k5buf, k5buftype, K5BUF_DYNAMIC_ZAP, K5BUF_DYNAMIC,
                         K5BUF_FIXED, K5BUF_ERROR, k5_buf_init_dynamic,
                         k5_buf_add, k5_buf_add_len, k5_buf_add_fmt,
                         k5_buf_status, k5_buf_free};
use self::string_h::{memcpy, strcmp, strncmp, strdup, strchr, strcspn,
                     strlen};
use self::stdlib_h::{malloc, calloc, realloc, free, abort};
use self::assert_h::__assert_fail;
use self::k5_base64_h::{k5_base64_encode, k5_base64_decode};
#[c2rust::src_loc = "83:1"]
pub type json_type = *mut json_type_st;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "83:16"]
pub struct json_type_st {
    pub tid: k5_json_tid,
    pub name: *const libc::c_char,
    pub dealloc: type_dealloc_fn,
}
#[c2rust::src_loc = "81:1"]
pub type type_dealloc_fn
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "89:8"]
pub struct value_base {
    pub isa: json_type,
    pub ref_cnt: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "366:8"]
pub struct k5_json_object_st {
    pub entries: *mut entry,
    pub len: size_t,
    pub allocated: size_t,
}
/* ** Object type (string:value mapping) ***/
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "361:8"]
pub struct entry {
    pub key: *mut libc::c_char,
    pub value: k5_json_value,
}
/* ** Array type ***/
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "201:8"]
pub struct k5_json_array_st {
    pub values: *mut k5_json_value,
    pub len: size_t,
    pub allocated: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "608:8"]
pub struct obj_ctx {
    pub buf: *mut k5buf,
    pub ret: libc::c_int,
    pub first: libc::c_int,
}
/* ** JSON decoding ***/
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "707:8"]
pub struct decode_ctx {
    pub p: *const libc::c_uchar,
    pub depth: size_t,
}
/*
 * k5_json_value objects are reference-counted.  These functions increment and
 * decrement the refcount, possibly freeing the value.  k5_json_retain returns
 * its argument and always succeeds.  Both functions gracefully accept NULL.
 */
#[no_mangle]
#[c2rust::src_loc = "97:1"]
pub unsafe extern "C" fn k5_json_retain(mut val: k5_json_value)
 -> k5_json_value {
    let mut p: *mut value_base = 0 as *mut value_base;
    if val.is_null() { return val }
    p = (val as *mut value_base).offset(-(1 as libc::c_int as isize));
    if (*p).ref_cnt != 0 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"p->ref_cnt != 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"json.c\x00" as *const u8 as *const libc::c_char,
                      105 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 44],
                                                &[libc::c_char; 44]>(b"k5_json_value k5_json_retain(k5_json_value)\x00")).as_ptr());
    }
    (*p).ref_cnt = (*p).ref_cnt.wrapping_add(1);
    return val;
}
#[no_mangle]
#[c2rust::src_loc = "110:1"]
pub unsafe extern "C" fn k5_json_release(mut val: k5_json_value) {
    let mut p: *mut value_base = 0 as *mut value_base;
    if val.is_null() { return }
    p = (val as *mut value_base).offset(-(1 as libc::c_int as isize));
    if (*p).ref_cnt != 0 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"p->ref_cnt != 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"json.c\x00" as *const u8 as *const libc::c_char,
                      118 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 36],
                                                &[libc::c_char; 36]>(b"void k5_json_release(k5_json_value)\x00")).as_ptr());
    }
    (*p).ref_cnt = (*p).ref_cnt.wrapping_sub(1);
    if (*p).ref_cnt == 0 as libc::c_int as libc::c_uint {
        if (*(*p).isa).dealloc.is_some() {
            (*(*p).isa).dealloc.expect("non-null function pointer")(val);
        }
        free(p as *mut libc::c_void);
    };
}
/* Get the type description of a k5_json_value. */
#[c2rust::src_loc = "128:1"]
unsafe extern "C" fn get_isa(mut val: k5_json_value) -> json_type {
    let mut p: *mut value_base =
        (val as *mut value_base).offset(-(1 as libc::c_int as isize));
    return (*p).isa;
}
#[no_mangle]
#[c2rust::src_loc = "136:1"]
pub unsafe extern "C" fn k5_json_get_tid(mut val: k5_json_value)
 -> k5_json_tid {
    let mut isa: json_type = get_isa(val);
    return (*isa).tid;
}
#[c2rust::src_loc = "144:1"]
unsafe extern "C" fn alloc_value(mut type_0: json_type, mut size: size_t)
 -> k5_json_value {
    let mut p: *mut value_base =
        calloc(1 as libc::c_int as libc::c_ulong,
               size.wrapping_add(::std::mem::size_of::<value_base>() as
                                     libc::c_ulong)) as *mut value_base;
    if p.is_null() { return 0 as *mut libc::c_void }
    (*p).isa = type_0;
    (*p).ref_cnt = 1 as libc::c_int as libc::c_uint;
    return p.offset(1 as libc::c_int as isize) as *mut libc::c_void;
}
/* ** Null type ***/
#[c2rust::src_loc = "159:28"]
static mut null_type: json_type_st =
    {
        let mut init =
            json_type_st{tid: 1 as libc::c_int as k5_json_tid,
                         name:
                             b"null\x00" as *const u8 as *const libc::c_char,
                         dealloc: None,};
        init
    };
#[no_mangle]
#[c2rust::src_loc = "161:1"]
pub unsafe extern "C" fn k5_json_null_create(mut val_out: *mut k5_json_null)
 -> libc::c_int {
    *val_out =
        alloc_value(&mut null_type, 0 as libc::c_int as size_t) as
            k5_json_null;
    return if (*val_out).is_null() {
               12 as libc::c_int
           } else { 0 as libc::c_int };
}
/* Create a null value as a k5_json_value, for polymorphic convenience. */
#[no_mangle]
#[c2rust::src_loc = "168:1"]
pub unsafe extern "C" fn k5_json_null_create_val(mut val_out:
                                                     *mut k5_json_value)
 -> libc::c_int {
    *val_out = alloc_value(&mut null_type, 0 as libc::c_int as size_t);
    return if (*val_out).is_null() {
               12 as libc::c_int
           } else { 0 as libc::c_int };
}
/* ** Boolean type ***/
#[c2rust::src_loc = "177:28"]
static mut bool_type: json_type_st =
    {
        let mut init =
            json_type_st{tid: 2 as libc::c_int as k5_json_tid,
                         name:
                             b"bool\x00" as *const u8 as *const libc::c_char,
                         dealloc: None,};
        init
    };
#[no_mangle]
#[c2rust::src_loc = "179:1"]
pub unsafe extern "C" fn k5_json_bool_create(mut truth: libc::c_int,
                                             mut val_out: *mut k5_json_bool)
 -> libc::c_int {
    let mut b: k5_json_bool = 0 as *mut k5_json_bool_st;
    *val_out = 0 as k5_json_bool;
    b =
        alloc_value(&mut bool_type, 1 as libc::c_int as size_t) as
            k5_json_bool;
    if b.is_null() { return 12 as libc::c_int }
    *(b as *mut libc::c_uchar) = (truth != 0) as libc::c_int as libc::c_uchar;
    *val_out = b;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "193:1"]
pub unsafe extern "C" fn k5_json_bool_value(mut bval: k5_json_bool)
 -> libc::c_int {
    return *(bval as *mut libc::c_uchar) as libc::c_int;
}
#[c2rust::src_loc = "207:1"]
unsafe extern "C" fn array_dealloc(mut ptr: *mut libc::c_void) {
    let mut array: k5_json_array = ptr as k5_json_array;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*array).len {
        k5_json_release(*(*array).values.offset(i as isize));
        i = i.wrapping_add(1)
    }
    free((*array).values as *mut libc::c_void);
}
#[c2rust::src_loc = "218:28"]
static mut array_type: json_type_st =
    unsafe {
        {
            let mut init =
                json_type_st{tid: 129 as libc::c_int as k5_json_tid,
                             name:
                                 b"array\x00" as *const u8 as
                                     *const libc::c_char,
                             dealloc:
                                 Some(array_dealloc as
                                          unsafe extern "C" fn(_:
                                                                   *mut libc::c_void)
                                              -> ()),};
            init
        }
    };
#[no_mangle]
#[c2rust::src_loc = "222:1"]
pub unsafe extern "C" fn k5_json_array_create(mut val_out: *mut k5_json_array)
 -> libc::c_int {
    *val_out =
        alloc_value(&mut array_type,
                    ::std::mem::size_of::<k5_json_array_st>() as
                        libc::c_ulong) as k5_json_array;
    return if (*val_out).is_null() {
               12 as libc::c_int
           } else { 0 as libc::c_int };
}
/* Both of these functions increment the reference count on val. */
#[no_mangle]
#[c2rust::src_loc = "229:1"]
pub unsafe extern "C" fn k5_json_array_add(mut array: k5_json_array,
                                           mut val: k5_json_value)
 -> libc::c_int {
    let mut ptr: *mut k5_json_value = 0 as *mut k5_json_value;
    let mut new_alloc: size_t = 0;
    if (*array).len >= (*array).allocated {
        /* Increase the number of slots by 50% (MIN_ALLOC_SLOT minimum). */
        new_alloc =
            (*array).len.wrapping_add(1 as libc::c_int as
                                          libc::c_ulong).wrapping_add((*array).len
                                                                          >>
                                                                          1 as
                                                                              libc::c_int);
        if new_alloc < 16 as libc::c_int as libc::c_ulong {
            new_alloc = 16 as libc::c_int as size_t
        }
        ptr =
            realloc((*array).values as *mut libc::c_void,
                    new_alloc.wrapping_mul(::std::mem::size_of::<k5_json_value>()
                                               as libc::c_ulong)) as
                *mut k5_json_value;
        if ptr.is_null() { return 12 as libc::c_int }
        (*array).values = ptr;
        (*array).allocated = new_alloc
    }
    let fresh0 = (*array).len;
    (*array).len = (*array).len.wrapping_add(1);
    let ref mut fresh1 = *(*array).values.offset(fresh0 as isize);
    *fresh1 = k5_json_retain(val);
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "250:1"]
pub unsafe extern "C" fn k5_json_array_length(mut array: k5_json_array)
 -> size_t {
    return (*array).len;
}
/* Get an alias to the idx-th element of array, without incrementing the
 * reference count.  The caller must check idx against the array length. */
#[no_mangle]
#[c2rust::src_loc = "256:1"]
pub unsafe extern "C" fn k5_json_array_get(mut array: k5_json_array,
                                           mut idx: size_t) -> k5_json_value {
    if idx >= (*array).len { abort(); }
    return *(*array).values.offset(idx as isize);
}
#[no_mangle]
#[c2rust::src_loc = "264:1"]
pub unsafe extern "C" fn k5_json_array_set(mut array: k5_json_array,
                                           mut idx: size_t,
                                           mut val: k5_json_value) {
    if idx >= (*array).len { abort(); }
    k5_json_release(*(*array).values.offset(idx as isize));
    let ref mut fresh2 = *(*array).values.offset(idx as isize);
    *fresh2 = k5_json_retain(val);
}
/*
 * Create an array from a template and a variable argument list.  template
 * characters are:
 *   v: a k5_json_value argument is read, retained, and stored
 *   n: no argument is read; a null value is stored
 *   b: an int argument is read and stored as a boolean value
 *   i: an int argument is read and stored as a number value
 *   L: a long long argument is read and stored as a number value
 *   s: a const char * argument is read and stored as a null or string value
 *   B: const void * and size_t arguments are read and stored as a base64
 *      string value
 */
#[no_mangle]
#[c2rust::src_loc = "273:1"]
pub unsafe extern "C" fn k5_json_array_fmt(mut array_out: *mut k5_json_array,
                                           mut template: *const libc::c_char,
                                           mut args: ...) -> libc::c_int {
    let mut current_block: u64;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut ap: ::std::ffi::VaListImpl;
    let mut cstring: *const libc::c_char = 0 as *const libc::c_char;
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut len: size_t = 0;
    let mut nval: libc::c_longlong = 0;
    let mut array: k5_json_array = 0 as *mut k5_json_array_st;
    let mut val: k5_json_value = 0 as *mut libc::c_void;
    let mut num: k5_json_number = 0 as *mut k5_json_number_st;
    let mut str: k5_json_string = 0 as *mut k5_json_string_st;
    let mut b: k5_json_bool = 0 as *mut k5_json_bool_st;
    let mut null: k5_json_null = 0 as *mut k5_json_null_st;
    let mut truth: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    *array_out = 0 as k5_json_array;
    if k5_json_array_create(&mut array) != 0 { return 12 as libc::c_int }
    ap = args.clone();
    p = template;
    loop  {
        if !(*p as libc::c_int != '\u{0}' as i32) {
            current_block = 3160140712158701372;
            break ;
        }
        match *p as libc::c_int {
            118 => {
                val = k5_json_retain(ap.as_va_list().arg::<k5_json_value>())
            }
            110 => {
                if k5_json_null_create(&mut null) != 0 {
                    current_block = 14220266465818359136;
                    break ;
                }
                val = null as k5_json_value
            }
            98 => {
                truth = ap.as_va_list().arg::<libc::c_int>();
                if k5_json_bool_create(truth, &mut b) != 0 {
                    current_block = 14220266465818359136;
                    break ;
                }
                val = b as k5_json_value
            }
            105 => {
                nval =
                    ap.as_va_list().arg::<libc::c_int>() as libc::c_longlong;
                if k5_json_number_create(nval, &mut num) != 0 {
                    current_block = 14220266465818359136;
                    break ;
                }
                val = num as k5_json_value
            }
            76 => {
                nval = ap.as_va_list().arg::<libc::c_longlong>();
                if k5_json_number_create(nval, &mut num) != 0 {
                    current_block = 14220266465818359136;
                    break ;
                }
                val = num as k5_json_value
            }
            115 => {
                cstring = ap.as_va_list().arg::<*const libc::c_char>();
                if cstring.is_null() {
                    if k5_json_null_create(&mut null) != 0 {
                        current_block = 14220266465818359136;
                        break ;
                    }
                    val = null as k5_json_value
                } else {
                    if k5_json_string_create(cstring, &mut str) != 0 {
                        current_block = 14220266465818359136;
                        break ;
                    }
                    val = str as k5_json_value
                }
            }
            66 => {
                data = ap.as_va_list().arg::<*mut libc::c_uchar>();
                len = ap.as_va_list().arg::<size_t>();
                if k5_json_string_create_base64(data as *const libc::c_void,
                                                len, &mut str) != 0 {
                    current_block = 14220266465818359136;
                    break ;
                }
                val = str as k5_json_value
            }
            _ => { current_block = 14220266465818359136; break ; }
        }
        ret = k5_json_array_add(array, val);
        k5_json_release(val);
        if ret != 0 { current_block = 14220266465818359136; break ; }
        p = p.offset(1)
    }
    match current_block {
        14220266465818359136 => {
            k5_json_release(array as k5_json_value);
            return 12 as libc::c_int
        }
        _ => { *array_out = array; return 0 as libc::c_int }
    };
}
#[c2rust::src_loc = "372:1"]
unsafe extern "C" fn object_dealloc(mut ptr: *mut libc::c_void) {
    let mut obj: k5_json_object = ptr as k5_json_object;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*obj).len {
        free((*(*obj).entries.offset(i as isize)).key as *mut libc::c_void);
        k5_json_release((*(*obj).entries.offset(i as isize)).value);
        i = i.wrapping_add(1)
    }
    free((*obj).entries as *mut libc::c_void);
}
#[c2rust::src_loc = "385:28"]
static mut object_type: json_type_st =
    unsafe {
        {
            let mut init =
                json_type_st{tid: 130 as libc::c_int as k5_json_tid,
                             name:
                                 b"object\x00" as *const u8 as
                                     *const libc::c_char,
                             dealloc:
                                 Some(object_dealloc as
                                          unsafe extern "C" fn(_:
                                                                   *mut libc::c_void)
                                              -> ()),};
            init
        }
    };
#[no_mangle]
#[c2rust::src_loc = "389:1"]
pub unsafe extern "C" fn k5_json_object_create(mut val_out:
                                                   *mut k5_json_object)
 -> libc::c_int {
    *val_out =
        alloc_value(&mut object_type,
                    ::std::mem::size_of::<k5_json_object_st>() as
                        libc::c_ulong) as k5_json_object;
    return if (*val_out).is_null() {
               12 as libc::c_int
           } else { 0 as libc::c_int };
}
/* Return the number of mappings in an object. */
#[no_mangle]
#[c2rust::src_loc = "396:1"]
pub unsafe extern "C" fn k5_json_object_count(mut obj: k5_json_object)
 -> size_t {
    return (*obj).len;
}
/* Return the entry for key within obj, or NULL if none exists. */
#[c2rust::src_loc = "403:1"]
unsafe extern "C" fn object_search(mut obj: k5_json_object,
                                   mut key: *const libc::c_char)
 -> *mut entry {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*obj).len {
        if strcmp(key, (*(*obj).entries.offset(i as isize)).key) ==
               0 as libc::c_int {
            return &mut *(*obj).entries.offset(i as isize) as *mut entry
        }
        i = i.wrapping_add(1)
    }
    return 0 as *mut entry;
}
/* Get an alias to the object's value for key, without incrementing the
 * reference count.  Returns NULL if there is no value for key. */
#[no_mangle]
#[c2rust::src_loc = "415:1"]
pub unsafe extern "C" fn k5_json_object_get(mut obj: k5_json_object,
                                            mut key: *const libc::c_char)
 -> k5_json_value {
    let mut ent: *mut entry = 0 as *mut entry;
    ent = object_search(obj, key);
    if ent.is_null() { return 0 as *mut libc::c_void }
    return (*ent).value;
}
/*
 * Store val into object at key, incrementing val's reference count and
 * releasing any previous value at key.  If val is NULL, key is removed from
 * obj if it exists, and obj remains unchanged if it does not.
 */
#[no_mangle]
#[c2rust::src_loc = "426:1"]
pub unsafe extern "C" fn k5_json_object_set(mut obj: k5_json_object,
                                            mut key: *const libc::c_char,
                                            mut val: k5_json_value)
 -> libc::c_int {
    let mut ent: *mut entry = 0 as *mut entry;
    let mut ptr: *mut entry = 0 as *mut entry;
    let mut new_alloc: size_t = 0;
    let mut i: size_t = 0;
    ent = object_search(obj, key);
    if !ent.is_null() {
        k5_json_release((*ent).value);
        if val.is_null() {
            /* Remove this key. */
            free((*ent).key as *mut libc::c_void);
            i =
                ent.wrapping_offset_from((*obj).entries) as libc::c_long as
                    size_t;
            while i <
                      (*obj).len.wrapping_sub(1 as libc::c_int as
                                                  libc::c_ulong) {
                *(*obj).entries.offset(i as isize) =
                    *(*obj).entries.offset(i.wrapping_add(1 as libc::c_int as
                                                              libc::c_ulong)
                                               as isize);
                i = i.wrapping_add(1)
            }
            (*obj).len = (*obj).len.wrapping_sub(1)
        } else {
            /* Overwrite this key's value with the new one. */
            (*ent).value = k5_json_retain(val)
        }
        return 0 as libc::c_int
    }
    /* If didn't find a key the caller asked to remove, do nothing. */
    if val.is_null() { return 0 as libc::c_int }
    if (*obj).len >= (*obj).allocated {
        /* Increase the number of slots by 50% (MIN_ALLOC_SLOT minimum). */
        new_alloc =
            (*obj).len.wrapping_add(1 as libc::c_int as
                                        libc::c_ulong).wrapping_add((*obj).len
                                                                        >>
                                                                        1 as
                                                                            libc::c_int);
        if new_alloc < 16 as libc::c_int as libc::c_ulong {
            new_alloc = 16 as libc::c_int as size_t
        }
        ptr =
            realloc((*obj).entries as *mut libc::c_void,
                    new_alloc.wrapping_mul(::std::mem::size_of::<entry>() as
                                               libc::c_ulong)) as *mut entry;
        if ptr.is_null() { return 12 as libc::c_int }
        (*obj).entries = ptr;
        (*obj).allocated = new_alloc
    }
    let ref mut fresh3 = (*(*obj).entries.offset((*obj).len as isize)).key;
    *fresh3 = strdup(key);
    if (*(*obj).entries.offset((*obj).len as isize)).key.is_null() {
        return 12 as libc::c_int
    }
    let ref mut fresh4 = (*(*obj).entries.offset((*obj).len as isize)).value;
    *fresh4 = k5_json_retain(val);
    (*obj).len = (*obj).len.wrapping_add(1);
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "471:1"]
pub unsafe extern "C" fn k5_json_object_iterate(mut obj: k5_json_object,
                                                mut func:
                                                    k5_json_object_iterator_fn,
                                                mut arg: *mut libc::c_void) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*obj).len {
        func.expect("non-null function pointer")(arg,
                                                 (*(*obj).entries.offset(i as
                                                                             isize)).key,
                                                 (*(*obj).entries.offset(i as
                                                                             isize)).value);
        i = i.wrapping_add(1)
    };
}
/* ** String type ***/
#[c2rust::src_loc = "483:28"]
static mut string_type: json_type_st =
    {
        let mut init =
            json_type_st{tid: 131 as libc::c_int as k5_json_tid,
                         name:
                             b"string\x00" as *const u8 as
                                 *const libc::c_char,
                         dealloc: None,};
        init
    };
#[no_mangle]
#[c2rust::src_loc = "487:1"]
pub unsafe extern "C" fn k5_json_string_create(mut cstring:
                                                   *const libc::c_char,
                                               mut val_out:
                                                   *mut k5_json_string)
 -> libc::c_int {
    return k5_json_string_create_len(cstring as *const libc::c_void,
                                     strlen(cstring), val_out);
}
#[no_mangle]
#[c2rust::src_loc = "493:1"]
pub unsafe extern "C" fn k5_json_string_create_len(mut data:
                                                       *const libc::c_void,
                                                   mut len: size_t,
                                                   mut val_out:
                                                       *mut k5_json_string)
 -> libc::c_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    *val_out = 0 as k5_json_string;
    s =
        alloc_value(&mut string_type,
                    len.wrapping_add(1 as libc::c_int as libc::c_ulong)) as
            *mut libc::c_char;
    if s.is_null() { return 12 as libc::c_int }
    if len > 0 as libc::c_int as libc::c_ulong {
        memcpy(s as *mut libc::c_void, data, len);
    }
    *s.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    *val_out = s as k5_json_string;
    return 0 as libc::c_int;
}
/* Create a base64 string value from binary data. */
#[no_mangle]
#[c2rust::src_loc = "510:1"]
pub unsafe extern "C" fn k5_json_string_create_base64(mut data:
                                                          *const libc::c_void,
                                                      mut len: size_t,
                                                      mut val_out:
                                                          *mut k5_json_string)
 -> libc::c_int {
    let mut base64: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    *val_out = 0 as k5_json_string;
    base64 = k5_base64_encode(data, len);
    if base64.is_null() { return 12 as libc::c_int }
    ret = k5_json_string_create(base64, val_out);
    free(base64 as *mut libc::c_void);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "526:1"]
pub unsafe extern "C" fn k5_json_string_utf8(mut string: k5_json_string)
 -> *const libc::c_char {
    return string as *const libc::c_char;
}
/* Decode the base64 contents of string. */
#[no_mangle]
#[c2rust::src_loc = "532:1"]
pub unsafe extern "C" fn k5_json_string_unbase64(mut string: k5_json_string,
                                                 mut data_out:
                                                     *mut *mut libc::c_uchar,
                                                 mut len_out: *mut size_t)
 -> libc::c_int {
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut len: size_t = 0;
    *data_out = 0 as *mut libc::c_uchar;
    *len_out = 0 as libc::c_int as size_t;
    data =
        k5_base64_decode(string as *const libc::c_char, &mut len) as
            *mut libc::c_uchar;
    if data.is_null() {
        return if len == 0 as libc::c_int as libc::c_ulong {
                   12 as libc::c_int
               } else { 22 as libc::c_int }
    }
    *data_out = data;
    *len_out = len;
    return 0 as libc::c_int;
}
/* ** Number type ***/
#[c2rust::src_loc = "551:28"]
static mut number_type: json_type_st =
    {
        let mut init =
            json_type_st{tid: 0 as libc::c_int as k5_json_tid,
                         name:
                             b"number\x00" as *const u8 as
                                 *const libc::c_char,
                         dealloc: None,};
        init
    };
#[no_mangle]
#[c2rust::src_loc = "555:1"]
pub unsafe extern "C" fn k5_json_number_create(mut number: libc::c_longlong,
                                               mut val_out:
                                                   *mut k5_json_number)
 -> libc::c_int {
    let mut n: k5_json_number = 0 as *mut k5_json_number_st;
    *val_out = 0 as k5_json_number;
    n =
        alloc_value(&mut number_type,
                    ::std::mem::size_of::<libc::c_longlong>() as
                        libc::c_ulong) as k5_json_number;
    if n.is_null() { return 12 as libc::c_int }
    *(n as *mut libc::c_longlong) = number;
    *val_out = n;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "569:1"]
pub unsafe extern "C" fn k5_json_number_value(mut number: k5_json_number)
 -> libc::c_longlong {
    return *(number as *mut libc::c_longlong);
}
/* ** JSON encoding ***/
#[c2rust::src_loc = "577:19"]
static mut quotemap_json: [libc::c_char; 9] =
    [34, 92, 47, 98, 102, 110, 114, 116, 0];
#[c2rust::src_loc = "578:19"]
static mut quotemap_c: [libc::c_char; 9] = [34, 92, 47, 8, 12, 10, 13, 9, 0];
#[c2rust::src_loc = "579:19"]
static mut needs_quote: [libc::c_char; 34] =
    [92, 34, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
     19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0];
#[c2rust::src_loc = "584:1"]
unsafe extern "C" fn encode_string(mut buf: *mut k5buf,
                                   mut str: *const libc::c_char) {
    let mut n: size_t = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    k5_buf_add(buf, b"\"\x00" as *const u8 as *const libc::c_char);
    while *str as libc::c_int != '\u{0}' as i32 {
        n = strcspn(str, needs_quote.as_ptr());
        k5_buf_add_len(buf, str as *const libc::c_void, n);
        str = str.offset(n as isize);
        if *str as libc::c_int == '\u{0}' as i32 { break ; }
        k5_buf_add(buf, b"\\\x00" as *const u8 as *const libc::c_char);
        p = strchr(quotemap_c.as_ptr(), *str as libc::c_int);
        if !p.is_null() {
            k5_buf_add_len(buf,
                           quotemap_json.as_ptr().offset(p.wrapping_offset_from(quotemap_c.as_ptr())
                                                             as libc::c_long
                                                             as isize) as
                               *const libc::c_void,
                           1 as libc::c_int as size_t);
        } else {
            k5_buf_add_fmt(buf,
                           b"u00%02X\x00" as *const u8 as *const libc::c_char,
                           *str as libc::c_uint);
        }
        str = str.offset(1)
    }
    k5_buf_add(buf, b"\"\x00" as *const u8 as *const libc::c_char);
}
#[c2rust::src_loc = "614:1"]
unsafe extern "C" fn encode_obj_entry(mut ctx: *mut libc::c_void,
                                      mut key: *const libc::c_char,
                                      mut value: k5_json_value) {
    let mut j: *mut obj_ctx = ctx as *mut obj_ctx;
    if (*j).ret != 0 { return }
    if (*j).first != 0 {
        (*j).first = 0 as libc::c_int
    } else {
        k5_buf_add((*j).buf, b",\x00" as *const u8 as *const libc::c_char);
    }
    encode_string((*j).buf, key);
    k5_buf_add((*j).buf, b":\x00" as *const u8 as *const libc::c_char);
    (*j).ret = encode_value((*j).buf, value);
}
#[c2rust::src_loc = "630:1"]
unsafe extern "C" fn encode_value(mut buf: *mut k5buf, mut val: k5_json_value)
 -> libc::c_int {
    let mut type_0: k5_json_tid = 0;
    let mut ret: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut len: size_t = 0;
    let mut ctx: obj_ctx = obj_ctx{buf: 0 as *mut k5buf, ret: 0, first: 0,};
    if val.is_null() { return 22 as libc::c_int }
    type_0 = k5_json_get_tid(val);
    match type_0 {
        129 => {
            k5_buf_add(buf, b"[\x00" as *const u8 as *const libc::c_char);
            len = k5_json_array_length(val as k5_json_array);
            i = 0 as libc::c_int as size_t;
            while i < len {
                if i != 0 as libc::c_int as libc::c_ulong {
                    k5_buf_add(buf,
                               b",\x00" as *const u8 as *const libc::c_char);
                }
                ret =
                    encode_value(buf,
                                 k5_json_array_get(val as k5_json_array, i));
                if ret != 0 { return ret }
                i = i.wrapping_add(1)
            }
            k5_buf_add(buf, b"]\x00" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int
        }
        130 => {
            k5_buf_add(buf, b"{\x00" as *const u8 as *const libc::c_char);
            ctx.buf = buf;
            ctx.ret = 0 as libc::c_int;
            ctx.first = 1 as libc::c_int;
            k5_json_object_iterate(val as k5_json_object,
                                   Some(encode_obj_entry as
                                            unsafe extern "C" fn(_:
                                                                     *mut libc::c_void,
                                                                 _:
                                                                     *const libc::c_char,
                                                                 _:
                                                                     k5_json_value)
                                                -> ()),
                                   &mut ctx as *mut obj_ctx as
                                       *mut libc::c_void);
            k5_buf_add(buf, b"}\x00" as *const u8 as *const libc::c_char);
            return ctx.ret
        }
        131 => {
            encode_string(buf, k5_json_string_utf8(val as k5_json_string));
            return 0 as libc::c_int
        }
        0 => {
            k5_buf_add_fmt(buf,
                           b"%lld\x00" as *const u8 as *const libc::c_char,
                           k5_json_number_value(val as k5_json_number));
            return 0 as libc::c_int
        }
        1 => {
            k5_buf_add(buf, b"null\x00" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int
        }
        2 => {
            k5_buf_add(buf,
                       if k5_json_bool_value(val as k5_json_bool) != 0 {
                           b"true\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"false\x00" as *const u8 as *const libc::c_char
                       });
            return 0 as libc::c_int
        }
        _ => { return 22 as libc::c_int }
    };
}
/*
 * JSON encoding and decoding
 */
#[no_mangle]
#[c2rust::src_loc = "686:1"]
pub unsafe extern "C" fn k5_json_encode(mut val: k5_json_value,
                                        mut json_out: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut buf: k5buf =
        k5buf{buftype: K5BUF_ERROR,
              data: 0 as *mut libc::c_void,
              space: 0,
              len: 0,};
    let mut ret: libc::c_int = 0;
    *json_out = 0 as *mut libc::c_char;
    k5_buf_init_dynamic(&mut buf);
    ret = encode_value(&mut buf, val);
    if ret != 0 { k5_buf_free(&mut buf); return ret }
    if k5_buf_status(&mut buf) != 0 as libc::c_int {
        return 12 as libc::c_int
    }
    *json_out = buf.data as *mut libc::c_char;
    return 0 as libc::c_int;
}
/* Consume whitespace.  Return 0 if there is anything left to parse after the
 * whitespace, -1 if not. */
#[c2rust::src_loc = "716:1"]
unsafe extern "C" fn white_spaces(mut ctx: *mut decode_ctx) -> libc::c_int {
    let mut c: libc::c_uchar = 0;
    while *(*ctx).p as libc::c_int != '\u{0}' as i32 {
        c = *(*ctx).p;
        if c as libc::c_int != ' ' as i32 && c as libc::c_int != '\t' as i32
               && c as libc::c_int != '\r' as i32 &&
               c as libc::c_int != '\n' as i32 {
            return 0 as libc::c_int
        }
        (*ctx).p = (*ctx).p.offset(1)
    }
    return -(1 as libc::c_int);
}
/* Return true if c is a decimal digit. */
#[inline]
#[c2rust::src_loc = "730:1"]
unsafe extern "C" fn is_digit(mut c: libc::c_uchar) -> libc::c_int {
    return ('0' as i32 <= c as libc::c_int && c as libc::c_int <= '9' as i32)
               as libc::c_int;
}
/* Return true if c is a hexadecimal digit (per RFC 5234 HEXDIG). */
#[inline]
#[c2rust::src_loc = "737:1"]
unsafe extern "C" fn is_hex_digit(mut c: libc::c_uchar) -> libc::c_int {
    return (is_digit(c) != 0 ||
                'A' as i32 <= c as libc::c_int &&
                    c as libc::c_int <= 'F' as i32) as libc::c_int;
}
/* Return the numeric value of a hex digit; aborts if c is not a hex digit. */
#[inline]
#[c2rust::src_loc = "744:1"]
unsafe extern "C" fn hexval(mut c: libc::c_uchar) -> libc::c_uint {
    if is_digit(c) != 0 {
        return (c as libc::c_int - '0' as i32) as libc::c_uint
    } else {
        if 'A' as i32 <= c as libc::c_int && c as libc::c_int <= 'F' as i32 {
            return (c as libc::c_int - 'A' as i32 + 10 as libc::c_int) as
                       libc::c_uint
        }
    }
    abort();
}
/* Parse a JSON number (which must be an integer in the signed 64-bit range; we
 * do not allow floating-point numbers). */
#[c2rust::src_loc = "756:1"]
unsafe extern "C" fn parse_number(mut ctx: *mut decode_ctx,
                                  mut val_out: *mut k5_json_number)
 -> libc::c_int {
    let umax: libc::c_ulonglong = !(0 as libc::c_ulonglong);
    let smax: libc::c_ulonglong =
        ((1 as libc::c_ulonglong) <<
             63 as
                 libc::c_int).wrapping_sub(1 as libc::c_int as
                                               libc::c_ulonglong);
    let mut number: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    let mut neg: libc::c_int = 1 as libc::c_int;
    *val_out = 0 as k5_json_number;
    if *(*ctx).p as libc::c_int == '-' as i32 {
        neg = -(1 as libc::c_int);
        (*ctx).p = (*ctx).p.offset(1)
    }
    if is_digit(*(*ctx).p) == 0 { return 22 as libc::c_int }
    /* Read the number into an unsigned 64-bit container, ensuring that we
     * don't overflow it. */
    while is_digit(*(*ctx).p) != 0 {
        if number.wrapping_add(1 as libc::c_int as libc::c_ulonglong) >
               umax.wrapping_div(10 as libc::c_int as libc::c_ulonglong) {
            return 75 as libc::c_int
        }
        number =
            number.wrapping_mul(10 as libc::c_int as
                                    libc::c_ulonglong).wrapping_add((*(*ctx).p
                                                                         as
                                                                         libc::c_int
                                                                         -
                                                                         '0'
                                                                             as
                                                                             i32)
                                                                        as
                                                                        libc::c_ulonglong);
        (*ctx).p = (*ctx).p.offset(1)
    }
    /* Make sure the unsigned 64-bit value fits in the signed 64-bit range. */
    if number > smax.wrapping_add(1 as libc::c_int as libc::c_ulonglong) ||
           number > smax && neg == 1 as libc::c_int {
        return 75 as libc::c_int
    }
    return k5_json_number_create(number.wrapping_mul(neg as libc::c_ulonglong)
                                     as libc::c_longlong, val_out);
}
/* Parse a JSON string (which must not quote Unicode code points above 256). */
#[c2rust::src_loc = "790:1"]
unsafe extern "C" fn parse_string(mut ctx: *mut decode_ctx,
                                  mut str_out: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut start: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut end: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut code: libc::c_uint = 0;
    *str_out = 0 as *mut libc::c_char;
    /* Find the start and end of the string. */
    if *(*ctx).p as libc::c_int != '\"' as i32 { return 22 as libc::c_int }
    (*ctx).p = (*ctx).p.offset(1);
    start = (*ctx).p;
    while *(*ctx).p as libc::c_int != '\u{0}' as i32 {
        if *(*ctx).p as libc::c_int == '\\' as i32 {
            (*ctx).p = (*ctx).p.offset(1);
            if *(*ctx).p as libc::c_int == '\u{0}' as i32 {
                return 22 as libc::c_int
            }
        } else if *(*ctx).p as libc::c_int == '\"' as i32 {
            let fresh5 = (*ctx).p;
            (*ctx).p = (*ctx).p.offset(1);
            end = fresh5;
            break ;
        }
        (*ctx).p = (*ctx).p.offset(1)
    }
    if end.is_null() { return 22 as libc::c_int }
    buf =
        malloc((end.wrapping_offset_from(start) as libc::c_long +
                    1 as libc::c_int as libc::c_long) as libc::c_ulong) as
            *mut libc::c_char;
    pos = buf;
    if buf.is_null() { return 12 as libc::c_int }
    p = start;
    while p < end {
        if *p as libc::c_int == '\\' as i32 {
            p = p.offset(1);
            if *p as libc::c_int == 'u' as i32 &&
                   is_hex_digit(*p.offset(1 as libc::c_int as isize)) != 0 &&
                   is_hex_digit(*p.offset(2 as libc::c_int as isize)) != 0 &&
                   is_hex_digit(*p.offset(3 as libc::c_int as isize)) != 0 &&
                   is_hex_digit(*p.offset(4 as libc::c_int as isize)) != 0 {
                code =
                    hexval(*p.offset(1 as libc::c_int as isize)) <<
                        12 as libc::c_int |
                        hexval(*p.offset(2 as libc::c_int as isize)) <<
                            8 as libc::c_int |
                        hexval(*p.offset(3 as libc::c_int as isize)) <<
                            4 as libc::c_int |
                        hexval(*p.offset(4 as libc::c_int as isize));
                if code <= 0xff as libc::c_int as libc::c_uint {
                    let fresh6 = pos;
                    pos = pos.offset(1);
                    *fresh6 = code as libc::c_char
                } else {
                    /* Code points above 0xff don't need to be quoted, so we
                     * don't implement translating those into UTF-8. */
                    free(buf as *mut libc::c_void);
                    return 22 as libc::c_int
                }
                p = p.offset(5 as libc::c_int as isize)
            } else {
                q = strchr(quotemap_json.as_ptr(), *p as libc::c_int);
                if !q.is_null() {
                    let fresh7 = pos;
                    pos = pos.offset(1);
                    *fresh7 =
                        quotemap_c[q.wrapping_offset_from(quotemap_json.as_ptr())
                                       as libc::c_long as usize]
                } else {
                    free(buf as *mut libc::c_void);
                    return 22 as libc::c_int
                }
                p = p.offset(1)
            }
        } else {
            let fresh8 = p;
            p = p.offset(1);
            let fresh9 = pos;
            pos = pos.offset(1);
            *fresh9 = *fresh8 as libc::c_char
        }
    }
    *pos = '\u{0}' as i32 as libc::c_char;
    *str_out = buf;
    return 0 as libc::c_int;
}
/* Parse an object association and place it into obj. */
#[c2rust::src_loc = "856:1"]
unsafe extern "C" fn parse_object_association(mut obj: k5_json_object,
                                              mut ctx: *mut decode_ctx)
 -> libc::c_int {
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: k5_json_value = 0 as *mut libc::c_void;
    let mut ret: libc::c_int = 0;
    /* Parse the key and value. */
    ret = parse_string(ctx, &mut key);
    if ret != 0 { return ret }
    if !(white_spaces(ctx) != 0) {
        if !(*(*ctx).p as libc::c_int != ':' as i32) {
            (*ctx).p = (*ctx).p.offset(1);
            if !(white_spaces(ctx) != 0) {
                ret = parse_value(ctx, &mut val);
                if ret != 0 { free(key as *mut libc::c_void); return ret }
                /* Add the key and value to obj. */
                ret = k5_json_object_set(obj, key, val);
                free(key as *mut libc::c_void);
                k5_json_release(val);
                return ret
            }
        }
    }
    free(key as *mut libc::c_void);
    return 22 as libc::c_int;
}
/* Parse a JSON object. */
#[c2rust::src_loc = "892:1"]
unsafe extern "C" fn parse_object(mut ctx: *mut decode_ctx,
                                  mut val_out: *mut k5_json_object)
 -> libc::c_int {
    let mut current_block: u64;
    let mut obj: k5_json_object = 0 as k5_json_object;
    let mut ret: libc::c_int = 0;
    *val_out = 0 as k5_json_object;
    /* Parse past the opening brace. */
    if *(*ctx).p as libc::c_int != '{' as i32 { return 22 as libc::c_int }
    (*ctx).p = (*ctx).p.offset(1);
    if white_spaces(ctx) != 0 { return 22 as libc::c_int }
    ret = k5_json_object_create(&mut obj);
    if ret != 0 { return ret }
    /* Pairs associations until we reach the terminating brace. */
    if *(*ctx).p as libc::c_int != '}' as i32 {
        current_block = 11812396948646013369;
    } else { current_block = 5601891728916014340; }
    loop  {
        match current_block {
            11812396948646013369 => {
                ret = parse_object_association(obj, ctx);
                if ret != 0 {
                    k5_json_release(obj as k5_json_value);
                    return ret
                }
                if white_spaces(ctx) != 0 { break ; }
                if *(*ctx).p as libc::c_int == '}' as i32 {
                    current_block = 5601891728916014340;
                    continue ;
                }
                if *(*ctx).p as libc::c_int != ',' as i32 { break ; }
                (*ctx).p = (*ctx).p.offset(1);
                if white_spaces(ctx) != 0 {
                    break ;
                } else { current_block = 11812396948646013369; }
            }
            _ => {
                (*ctx).p = (*ctx).p.offset(1);
                *val_out = obj;
                return 0 as libc::c_int
            }
        }
    }
    k5_json_release(obj as k5_json_value);
    return 22 as libc::c_int;
}
/* Parse an value and place it into array. */
#[c2rust::src_loc = "940:1"]
unsafe extern "C" fn parse_array_item(mut array: k5_json_array,
                                      mut ctx: *mut decode_ctx)
 -> libc::c_int {
    let mut val: k5_json_value = 0 as *mut libc::c_void;
    let mut ret: libc::c_int = 0;
    ret = parse_value(ctx, &mut val);
    if ret != 0 { return ret }
    ret = k5_json_array_add(array, val);
    k5_json_release(val);
    return ret;
}
/* Parse a JSON array. */
#[c2rust::src_loc = "955:1"]
unsafe extern "C" fn parse_array(mut ctx: *mut decode_ctx,
                                 mut val_out: *mut k5_json_array)
 -> libc::c_int {
    let mut current_block: u64;
    let mut array: k5_json_array = 0 as k5_json_array;
    let mut ret: libc::c_int = 0;
    *val_out = 0 as k5_json_array;
    /* Parse past the opening bracket. */
    if *(*ctx).p as libc::c_int != '[' as i32 { return 22 as libc::c_int }
    (*ctx).p = (*ctx).p.offset(1);
    if white_spaces(ctx) != 0 { return 22 as libc::c_int }
    ret = k5_json_array_create(&mut array);
    if ret != 0 { return ret }
    /* Pairs values until we reach the terminating bracket. */
    if *(*ctx).p as libc::c_int != ']' as i32 {
        current_block = 11812396948646013369;
    } else { current_block = 5601891728916014340; }
    loop  {
        match current_block {
            11812396948646013369 => {
                ret = parse_array_item(array, ctx);
                if ret != 0 {
                    k5_json_release(array as k5_json_value);
                    return ret
                }
                if white_spaces(ctx) != 0 { break ; }
                if *(*ctx).p as libc::c_int == ']' as i32 {
                    current_block = 5601891728916014340;
                    continue ;
                }
                if *(*ctx).p as libc::c_int != ',' as i32 { break ; }
                (*ctx).p = (*ctx).p.offset(1);
                if white_spaces(ctx) != 0 {
                    break ;
                } else { current_block = 11812396948646013369; }
            }
            _ => {
                (*ctx).p = (*ctx).p.offset(1);
                *val_out = array;
                return 0 as libc::c_int
            }
        }
    }
    k5_json_release(array as k5_json_value);
    return 22 as libc::c_int;
}
/* Parse a JSON value of any type. */
#[c2rust::src_loc = "1003:1"]
unsafe extern "C" fn parse_value(mut ctx: *mut decode_ctx,
                                 mut val_out: *mut k5_json_value)
 -> libc::c_int {
    let mut null: k5_json_null = 0 as *mut k5_json_null_st;
    let mut bval: k5_json_bool = 0 as *mut k5_json_bool_st;
    let mut num: k5_json_number = 0 as *mut k5_json_number_st;
    let mut str: k5_json_string = 0 as *mut k5_json_string_st;
    let mut obj: k5_json_object = 0 as *mut k5_json_object_st;
    let mut array: k5_json_array = 0 as *mut k5_json_array_st;
    let mut cstring: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    *val_out = 0 as *mut libc::c_void;
    if white_spaces(ctx) != 0 { return 22 as libc::c_int }
    if *(*ctx).p as libc::c_int == '\"' as i32 {
        ret = parse_string(ctx, &mut cstring);
        if ret != 0 { return ret }
        ret = k5_json_string_create(cstring, &mut str);
        free(cstring as *mut libc::c_void);
        if ret != 0 { return ret }
        *val_out = str as k5_json_value
    } else if *(*ctx).p as libc::c_int == '{' as i32 {
        let fresh10 = (*ctx).depth;
        (*ctx).depth = (*ctx).depth.wrapping_sub(1);
        if fresh10 == 1 as libc::c_int as libc::c_ulong {
            return 22 as libc::c_int
        }
        ret = parse_object(ctx, &mut obj);
        if ret != 0 { return ret }
        (*ctx).depth = (*ctx).depth.wrapping_add(1);
        *val_out = obj as k5_json_value
    } else if *(*ctx).p as libc::c_int == '[' as i32 {
        let fresh11 = (*ctx).depth;
        (*ctx).depth = (*ctx).depth.wrapping_sub(1);
        if fresh11 == 1 as libc::c_int as libc::c_ulong {
            return 22 as libc::c_int
        }
        ret = parse_array(ctx, &mut array);
        (*ctx).depth = (*ctx).depth.wrapping_add(1);
        *val_out = array as k5_json_value
    } else if is_digit(*(*ctx).p) != 0 ||
                  *(*ctx).p as libc::c_int == '-' as i32 {
        ret = parse_number(ctx, &mut num);
        if ret != 0 { return ret }
        *val_out = num as k5_json_value
    } else if strncmp((*ctx).p as *mut libc::c_char,
                      b"null\x00" as *const u8 as *const libc::c_char,
                      4 as libc::c_int as libc::c_ulong) == 0 as libc::c_int {
        (*ctx).p = (*ctx).p.offset(4 as libc::c_int as isize);
        ret = k5_json_null_create(&mut null);
        if ret != 0 { return ret }
        *val_out = null as k5_json_value
    } else if strncmp((*ctx).p as *mut libc::c_char,
                      b"true\x00" as *const u8 as *const libc::c_char,
                      4 as libc::c_int as libc::c_ulong) == 0 as libc::c_int {
        (*ctx).p = (*ctx).p.offset(4 as libc::c_int as isize);
        ret = k5_json_bool_create(1 as libc::c_int, &mut bval);
        if ret != 0 { return ret }
        *val_out = bval as k5_json_value
    } else if strncmp((*ctx).p as *mut libc::c_char,
                      b"false\x00" as *const u8 as *const libc::c_char,
                      5 as libc::c_int as libc::c_ulong) == 0 as libc::c_int {
        (*ctx).p = (*ctx).p.offset(5 as libc::c_int as isize);
        ret = k5_json_bool_create(0 as libc::c_int, &mut bval);
        if ret != 0 { return ret }
        *val_out = bval as k5_json_value
    } else { return 22 as libc::c_int }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1073:1"]
pub unsafe extern "C" fn k5_json_decode(mut string: *const libc::c_char,
                                        mut val_out: *mut k5_json_value)
 -> libc::c_int {
    let mut ctx: decode_ctx =
        decode_ctx{p: 0 as *const libc::c_uchar, depth: 0,};
    let mut val: k5_json_value = 0 as *mut libc::c_void;
    let mut ret: libc::c_int = 0;
    *val_out = 0 as *mut libc::c_void;
    ctx.p = string as *mut libc::c_uchar;
    ctx.depth = 64 as libc::c_int as size_t;
    ret = parse_value(&mut ctx, &mut val);
    if ret != 0 { return ret }
    if white_spaces(&mut ctx) == 0 as libc::c_int {
        k5_json_release(val);
        return 22 as libc::c_int
    }
    *val_out = val;
    return 0 as libc::c_int;
}
