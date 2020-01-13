use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/sys/types.h:27"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:27"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:27"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:27"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
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
    use super::stdint_intn_h::int32_t;
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/int-proto.h:28"]
pub mod int_proto_h {
    #[c2rust::src_loc = "32:1"]
    pub type k5_response_items = k5_response_items_st;
    use super::k5_response_items_st;
    /* KRB5_INT_FUNC_PROTO__ */
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
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
    }
}
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "435:1"]
        pub fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:27"]
pub mod k5_int_h {
    #[inline]
    #[c2rust::src_loc = "666:1"]
    pub unsafe extern "C" fn zapfreestr(mut str: *mut libc::c_void) {
        if !str.is_null() {
            explicit_bzero(str, strlen(str as *mut libc::c_char));
            free(str);
        };
    }
    use super::string_h::{explicit_bzero, strlen};
    use super::stdlib_h::free;
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
pub use self::types_h::{__int32_t, __ssize_t};
pub use self::sys_types_h::ssize_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::krb5_h::{krb5_int32, krb5_boolean, krb5_error_code, krb5_magic,
                       _krb5_data, krb5_data};
pub use self::int_proto_h::k5_response_items;
use self::stdlib_h::{calloc, realloc, free};
use self::string_h::{explicit_bzero, strlen, strdup, strcmp};
pub use self::k5_int_h::zapfreestr;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/krb/response_items.c - Response items */
/*
 * Copyright 2012 Red Hat, Inc.
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
 * the name of Red Hat not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original Red Hat software.
 * Red Hat makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "30:8"]
pub struct k5_response_items_st {
    pub count: size_t,
    pub questions: *mut *mut libc::c_char,
    pub challenges: *mut *mut libc::c_char,
    pub answers: *mut *mut libc::c_char,
}
#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn k5_response_items_new(mut ri_out:
                                                   *mut *mut k5_response_items)
 -> krb5_error_code {
    *ri_out =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<k5_response_items>() as libc::c_ulong) as
            *mut k5_response_items;
    return if (*ri_out).is_null() {
               12 as libc::c_int
           } else { 0 as libc::c_int };
}
#[no_mangle]
#[c2rust::src_loc = "44:1"]
pub unsafe extern "C" fn k5_response_items_free(mut ri:
                                                    *mut k5_response_items) {
    k5_response_items_reset(ri);
    free(ri as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "51:1"]
pub unsafe extern "C" fn k5_response_items_reset(mut ri:
                                                     *mut k5_response_items) {
    let mut i: size_t = 0;
    if ri.is_null() { return }
    i = 0 as libc::c_int as size_t;
    while i < (*ri).count {
        free(*(*ri).questions.offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    free((*ri).questions as *mut libc::c_void);
    (*ri).questions = 0 as *mut *mut libc::c_char;
    i = 0 as libc::c_int as size_t;
    while i < (*ri).count {
        zapfreestr(*(*ri).challenges.offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    free((*ri).challenges as *mut libc::c_void);
    (*ri).challenges = 0 as *mut *mut libc::c_char;
    i = 0 as libc::c_int as size_t;
    while i < (*ri).count {
        zapfreestr(*(*ri).answers.offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    free((*ri).answers as *mut libc::c_void);
    (*ri).answers = 0 as *mut *mut libc::c_char;
    (*ri).count = 0 as libc::c_int as size_t;
}
#[no_mangle]
#[c2rust::src_loc = "77:1"]
pub unsafe extern "C" fn k5_response_items_empty(mut ri:
                                                     *const k5_response_items)
 -> krb5_boolean {
    return if ri.is_null() {
               1 as libc::c_int
           } else {
               ((*ri).count == 0 as libc::c_int as libc::c_ulong) as
                   libc::c_int
           } as krb5_boolean;
}
#[no_mangle]
#[c2rust::src_loc = "83:1"]
pub unsafe extern "C" fn k5_response_items_list_questions(mut ri:
                                                              *const k5_response_items)
 -> *const *const libc::c_char {
    if ri.is_null() { return 0 as *const *const libc::c_char }
    return (*ri).questions as *const *const libc::c_char;
}
#[c2rust::src_loc = "91:1"]
unsafe extern "C" fn find_question(mut ri: *const k5_response_items,
                                   mut question: *const libc::c_char)
 -> ssize_t {
    let mut i: size_t = 0;
    if ri.is_null() { return -(1 as libc::c_int) as ssize_t }
    i = 0 as libc::c_int as size_t;
    while i < (*ri).count {
        if strcmp(*(*ri).questions.offset(i as isize), question) ==
               0 as libc::c_int {
            return i as ssize_t
        }
        i = i.wrapping_add(1)
    }
    return -(1 as libc::c_int) as ssize_t;
}
#[c2rust::src_loc = "107:1"]
unsafe extern "C" fn push_question(mut ri: *mut k5_response_items,
                                   mut question: *const libc::c_char,
                                   mut challenge: *const libc::c_char)
 -> krb5_error_code {
    let mut tmp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut size: size_t = 0;
    if ri.is_null() { return 22 as libc::c_int }
    size =
        (::std::mem::size_of::<*mut libc::c_char>() as
             libc::c_ulong).wrapping_mul((*ri).count.wrapping_add(2 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong));
    tmp =
        realloc((*ri).questions as *mut libc::c_void, size) as
            *mut *mut libc::c_char;
    if tmp.is_null() { return 12 as libc::c_int }
    (*ri).questions = tmp;
    let ref mut fresh0 = *(*ri).questions.offset((*ri).count as isize);
    *fresh0 = 0 as *mut libc::c_char;
    let ref mut fresh1 =
        *(*ri).questions.offset((*ri).count.wrapping_add(1 as libc::c_int as
                                                             libc::c_ulong) as
                                    isize);
    *fresh1 = 0 as *mut libc::c_char;
    tmp =
        realloc((*ri).challenges as *mut libc::c_void, size) as
            *mut *mut libc::c_char;
    if tmp.is_null() { return 12 as libc::c_int }
    (*ri).challenges = tmp;
    let ref mut fresh2 = *(*ri).challenges.offset((*ri).count as isize);
    *fresh2 = 0 as *mut libc::c_char;
    let ref mut fresh3 =
        *(*ri).challenges.offset((*ri).count.wrapping_add(1 as libc::c_int as
                                                              libc::c_ulong)
                                     as isize);
    *fresh3 = 0 as *mut libc::c_char;
    tmp =
        realloc((*ri).answers as *mut libc::c_void, size) as
            *mut *mut libc::c_char;
    if tmp.is_null() { return 12 as libc::c_int }
    (*ri).answers = tmp;
    let ref mut fresh4 = *(*ri).answers.offset((*ri).count as isize);
    *fresh4 = 0 as *mut libc::c_char;
    let ref mut fresh5 =
        *(*ri).answers.offset((*ri).count.wrapping_add(1 as libc::c_int as
                                                           libc::c_ulong) as
                                  isize);
    *fresh5 = 0 as *mut libc::c_char;
    let ref mut fresh6 = *(*ri).questions.offset((*ri).count as isize);
    *fresh6 = strdup(question);
    if (*(*ri).questions.offset((*ri).count as isize)).is_null() {
        return 12 as libc::c_int
    }
    if !challenge.is_null() {
        let ref mut fresh7 = *(*ri).challenges.offset((*ri).count as isize);
        *fresh7 = strdup(challenge);
        if (*(*ri).challenges.offset((*ri).count as isize)).is_null() {
            free(*(*ri).questions.offset((*ri).count as isize) as
                     *mut libc::c_void);
            let ref mut fresh8 =
                *(*ri).questions.offset((*ri).count as isize);
            *fresh8 = 0 as *mut libc::c_char;
            return 12 as libc::c_int
        }
    }
    (*ri).count = (*ri).count.wrapping_add(1);
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "156:1"]
pub unsafe extern "C" fn k5_response_items_ask_question(mut ri:
                                                            *mut k5_response_items,
                                                        mut question:
                                                            *const libc::c_char,
                                                        mut challenge:
                                                            *const libc::c_char)
 -> krb5_error_code {
    let mut i: ssize_t = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    i = find_question(ri, question);
    if i < 0 as libc::c_int as libc::c_long {
        return push_question(ri, question, challenge)
    }
    if !challenge.is_null() {
        tmp = strdup(challenge);
        if tmp.is_null() { return 12 as libc::c_int }
    }
    zapfreestr(*(*ri).challenges.offset(i as isize) as *mut libc::c_void);
    let ref mut fresh9 = *(*ri).challenges.offset(i as isize);
    *fresh9 = tmp;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "178:1"]
pub unsafe extern "C" fn k5_response_items_get_challenge(mut ri:
                                                             *const k5_response_items,
                                                         mut question:
                                                             *const libc::c_char)
 -> *const libc::c_char {
    let mut i: ssize_t = 0;
    i = find_question(ri, question);
    if i < 0 as libc::c_int as libc::c_long {
        return 0 as *const libc::c_char
    }
    return *(*ri).challenges.offset(i as isize);
}
#[no_mangle]
#[c2rust::src_loc = "191:1"]
pub unsafe extern "C" fn k5_response_items_set_answer(mut ri:
                                                          *mut k5_response_items,
                                                      mut question:
                                                          *const libc::c_char,
                                                      mut answer:
                                                          *const libc::c_char)
 -> krb5_error_code {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: ssize_t = 0;
    i = find_question(ri, question);
    if i < 0 as libc::c_int as libc::c_long { return 22 as libc::c_int }
    if !answer.is_null() {
        tmp = strdup(answer);
        if tmp.is_null() { return 12 as libc::c_int }
    }
    zapfreestr(*(*ri).answers.offset(i as isize) as *mut libc::c_void);
    let ref mut fresh10 = *(*ri).answers.offset(i as isize);
    *fresh10 = tmp;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "213:1"]
pub unsafe extern "C" fn k5_response_items_get_answer(mut ri:
                                                          *const k5_response_items,
                                                      mut question:
                                                          *const libc::c_char)
 -> *const libc::c_char {
    let mut i: ssize_t = 0;
    i = find_question(ri, question);
    if i < 0 as libc::c_int as libc::c_long {
        return 0 as *const libc::c_char
    }
    return *(*ri).answers.offset(i as isize);
}
