extern "C" {
    pub type k5_json_array_st;

    pub type k5_json_string_st;

    pub type k5_json_number_st;

    pub type k5_json_bool_st;

    #[no_mangle]
    pub fn k5_json_release(val: crate::k5_json_h::k5_json_value);
    /* Create a null value as a k5_json_value, for polymorphic convenience. */
    #[no_mangle]
    pub fn k5_json_null_create_val(val_out: *mut crate::k5_json_h::k5_json_value) -> i32;

    #[no_mangle]
    pub fn k5_json_array_create(val_out: *mut crate::k5_json_h::k5_json_array) -> i32;
    /* Both of these functions increment the reference count on val. */
    #[no_mangle]
    pub fn k5_json_array_add(
        array: crate::k5_json_h::k5_json_array,
        val: crate::k5_json_h::k5_json_value,
    ) -> i32;
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
    pub fn k5_json_array_fmt(
        array_out: *mut crate::k5_json_h::k5_json_array,
        template: *const i8,
        _: ...
    ) -> i32;

    #[no_mangle]
    pub fn k5_json_string_create(
        cstring: *const i8,
        val_out: *mut crate::k5_json_h::k5_json_string,
    ) -> i32;

    #[no_mangle]
    pub fn k5_json_number_create(
        number: i64,
        val_out: *mut crate::k5_json_h::k5_json_number,
    ) -> i32;

    #[no_mangle]
    pub fn k5_json_encode(val: crate::k5_json_h::k5_json_value, json_out: *mut *mut i8) -> i32;

    #[no_mangle]
    pub fn k5_json_get_tid(val: crate::k5_json_h::k5_json_value) -> crate::k5_json_h::k5_json_tid;

    #[no_mangle]
    pub fn k5_json_bool_value(bval: crate::k5_json_h::k5_json_bool) -> i32;

    #[no_mangle]
    pub fn k5_json_array_length(array: crate::k5_json_h::k5_json_array) -> crate::stddef_h::size_t;
    /* Get an alias to the idx-th element of array, without incrementing the
     * reference count.  The caller must check idx against the array length. */
    #[no_mangle]
    pub fn k5_json_array_get(
        array: crate::k5_json_h::k5_json_array,
        idx: crate::stddef_h::size_t,
    ) -> crate::k5_json_h::k5_json_value;

    #[no_mangle]
    pub fn k5_json_string_utf8(string: crate::k5_json_h::k5_json_string) -> *const i8;
    /* Decode the base64 contents of string. */
    #[no_mangle]
    pub fn k5_json_string_unbase64(
        string: crate::k5_json_h::k5_json_string,
        data_out: *mut *mut u8,
        len_out: *mut crate::stddef_h::size_t,
    ) -> i32;

    #[no_mangle]
    pub fn k5_json_number_value(number: crate::k5_json_h::k5_json_number) -> i64;

    #[no_mangle]
    pub fn k5_json_decode(str: *const i8, val_out: *mut crate::k5_json_h::k5_json_value) -> i32;
}
// =============== BEGIN k5_json_h ================

/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */

/* include/k5-json.h - JSON declarations */

/*
 * Copyright (c) 2010 Kungliga Tekniska Högskolan
 * (Royal Institute of Technology, Stockholm, Sweden).
 * All rights reserved.
 *
 * Portions Copyright (c) 2010 Apple Inc. All rights reserved.
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

/*
 * Copyright (C) 2012 by the Massachusetts Institute of Technology.
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

/*
 * The k5_json_value C type can represent any kind of JSON value.  It has no
 * static type safety since it is represented using a void pointer, so be
 * careful with it.  Its type can be checked dynamically with k5_json_get_tid()
 * and the above constants.
 */
pub type k5_json_value = *mut libc::c_void;
/*
 * Array
 */
pub type k5_json_array = *mut crate::k5_json_h::k5_json_array_st;
/*
 * String
 */
pub type k5_json_string = *mut crate::k5_json_h::k5_json_string_st;
pub type k5_json_number = *mut crate::k5_json_h::k5_json_number_st;
pub type k5_json_tid = u32;
/*
 * Boolean
 */
pub type k5_json_bool = *mut crate::k5_json_h::k5_json_bool_st;
