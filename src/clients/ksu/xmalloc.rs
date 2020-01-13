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
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:27"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stdarg.h:27"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
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
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:27"]
pub mod stdio_h {
    use super::internal::__va_list_tag;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "366:1"]
        pub fn vasprintf(__ptr: *mut *mut libc::c_char,
                         __f: *const libc::c_char, __arg: ::std::ffi::VaList)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "775:1"]
        pub fn perror(__s: *const libc::c_char);
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/clients/ksu/ksu.h:28"]
pub mod ksu_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "64:15"]
        pub static mut prog_name: *mut libc::c_char;
    }
}
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::stddef_h::size_t;
pub use self::stdarg_h::va_list;
use self::string_h::{strlen, memcpy};
use self::stdlib_h::{malloc, calloc, realloc, exit};
use self::stdio_h::{vasprintf, perror};
use self::ksu_h::prog_name;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* clients/ksu/xmalloc.c - Exit-on-failure allocation wrappers */
/*
 * Copyright 1999 by the Massachusetts Institute of Technology.
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
#[no_mangle]
#[c2rust::src_loc = "30:1"]
pub unsafe extern "C" fn xmalloc(mut sz: size_t) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = malloc(sz);
    if ret.is_null() && sz != 0 as libc::c_int as libc::c_ulong {
        perror(prog_name);
        exit(1 as libc::c_int);
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn xrealloc(mut old: *mut libc::c_void,
                                  mut newsz: size_t) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = realloc(old, newsz);
    if ret.is_null() && newsz != 0 as libc::c_int as libc::c_ulong {
        perror(prog_name);
        exit(1 as libc::c_int);
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn xcalloc(mut nelts: size_t, mut eltsz: size_t)
 -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = calloc(nelts, eltsz);
    if ret.is_null() && nelts != 0 as libc::c_int as libc::c_ulong &&
           eltsz != 0 as libc::c_int as libc::c_ulong {
        perror(prog_name);
        exit(1 as libc::c_int);
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "60:1"]
pub unsafe extern "C" fn xstrdup(mut src: *const libc::c_char)
 -> *mut libc::c_char {
    let mut len: size_t =
        strlen(src).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut dst: *mut libc::c_char = xmalloc(len) as *mut libc::c_char;
    memcpy(dst as *mut libc::c_void, src as *const libc::c_void, len);
    return dst;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (c) 1994 by the University of Southern California
 *
 * EXPORT OF THIS SOFTWARE from the United States of America may
 *     require a specific license from the United States Government.
 *     It is the responsibility of any person or organization contemplating
 *     export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to copy, modify, and distribute
 *     this software and its documentation in source and binary forms is
 *     hereby granted, provided that any documentation or other materials
 *     related to such distribution or use acknowledge that the software
 *     was developed by the University of Southern California.
 *
 * DISCLAIMER OF WARRANTY.  THIS SOFTWARE IS PROVIDED "AS IS".  The
 *     University of Southern California MAKES NO REPRESENTATIONS OR
 *     WARRANTIES, EXPRESS OR IMPLIED.  By way of example, but not
 *     limitation, the University of Southern California MAKES NO
 *     REPRESENTATIONS OR WARRANTIES OF MERCHANTABILITY OR FITNESS FOR ANY
 *     PARTICULAR PURPOSE. The University of Southern
 *     California shall not be held liable for any liability nor for any
 *     direct, indirect, or consequential damages with respect to any
 *     claim by the user or distributor of the ksu software.
 *
 * KSU was writen by:  Ari Medvinsky, ari@isi.edu
 */
/* <stdarg.h> or <varargs.h> is already included by com_err.h.  */
/* 12 hours */
/* this is temp, should use realloc instead,
                        as done in most of the code */
/* globals */
/* **********/
/* krb_auth_su.c */
/* ccache.c */
/* authorization.c */
/* main.c */
/* heuristic.c */
/* min */
/* Note: print this out just be sure
                                  that it gets set */
#[no_mangle]
#[c2rust::src_loc = "68:1"]
pub unsafe extern "C" fn xasprintf(mut format: *const libc::c_char,
                                   mut args: ...) -> *mut libc::c_char {
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    if vasprintf(&mut out, format, args_0.as_va_list()) < 0 as libc::c_int {
        perror(prog_name);
        exit(1 as libc::c_int);
    }
    return out;
}
