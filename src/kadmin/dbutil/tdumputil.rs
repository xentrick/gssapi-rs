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
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
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
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stdarg.h:33"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:33"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    #[c2rust::src_loc = "43:1"]
    pub type _IO_lock_t = ();
    use super::types_h::{__off_t, __off64_t};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "38:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "37:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "36:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:33"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:33"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-buf.h:33"]
pub mod k5_buf_h {
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:8"]
    pub struct k5buf {
        pub buftype: k5buftype,
        pub data: *mut libc::c_void,
        pub space: size_t,
        pub len: size_t,
    }
    use super::stddef_h::size_t;
    extern "C" {
        /* Initialize a k5buf using an internally allocated dynamic buffer. */
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn k5_buf_init_dynamic(buf: *mut k5buf);
        /* Add a counted series of bytes to BUF. */
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn k5_buf_add_len(buf: *mut k5buf, data: *const libc::c_void,
                              len: size_t);
    }
    /* K5_BUF_H */
}
#[c2rust::header_src = "/usr/include/stdlib.h:33"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:33"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    use super::internal::__va_list_tag;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "341:12"]
        pub fn vfprintf(_: *mut FILE, _: *const libc::c_char,
                        _: ::std::ffi::VaList) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "366:1"]
        pub fn vasprintf(__ptr: *mut *mut libc::c_char,
                         __f: *const libc::c_char, __arg: ::std::ffi::VaList)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "521:1"]
        pub fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "626:1"]
        pub fn fputs(__s: *const libc::c_char, __stream: *mut FILE)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:33"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "225:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
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
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::types_h::{__int32_t, __off_t, __off64_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdarg_h::va_list;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::krb5_h::{krb5_int32, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data};
pub use self::k5_buf_h::{k5buftype, K5BUF_DYNAMIC_ZAP, K5BUF_DYNAMIC,
                         K5BUF_FIXED, K5BUF_ERROR, k5buf, k5_buf_init_dynamic,
                         k5_buf_add_len};
use self::stdlib_h::{calloc, free};
use self::stdio_h::{fprintf, vfprintf, vasprintf, fputc, fputs};
use self::string_h::strchr;
use self::assert_h::__assert_fail;
/* quote character */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "61:8"]
pub struct rechandle {
    pub fh: *mut FILE,
    pub rectype: *const libc::c_char,
    pub do_sep: libc::c_int,
    pub flavor: flavor,
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kdc/tdumputil.c - utilities for tab-separated, etc. files */
/*
 * Copyright (C) 2015 by the Massachusetts Institute of Technology.
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
/* for vasprintf */
/*
 * Structure describing flavor of a tabular output format.
 *
 * fieldsep is the field separator
 *
 * recordsep is the record/line separator
 *
 * quotechar begins and ends a quoted field.  If an instance of quotechar
 * occurs within a quoted field value, it is doubled.
 *
 * Values are only quoted if they contain fieldsep, recordsep, or quotechar.
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "55:8"]
pub struct flavor {
    pub fieldsep: libc::c_int,
    pub recordsep: libc::c_int,
    pub quotechar: libc::c_int,
}
#[c2rust::src_loc = "68:28"]
static mut tabsep: flavor =
    {
        let mut init =
            flavor{fieldsep: '\t' as i32,
                   recordsep: '\n' as i32,
                   quotechar: '\u{0}' as i32,};
        init
    };
#[c2rust::src_loc = "74:28"]
static mut csv: flavor =
    {
        let mut init =
            flavor{fieldsep: ',' as i32,
                   recordsep: '\n' as i32,
                   quotechar: '\"' as i32,};
        init
    };
/*
 * Double any quote characters present in a quoted field.
 */
#[c2rust::src_loc = "83:1"]
unsafe extern "C" fn qquote(mut fl: *mut flavor, mut s: *const libc::c_char)
 -> *mut libc::c_char {
    let mut sp: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: k5buf =
        k5buf{buftype: K5BUF_ERROR,
              data: 0 as *mut libc::c_void,
              space: 0,
              len: 0,};
    k5_buf_init_dynamic(&mut buf);
    sp = s;
    while *sp as libc::c_int != '\u{0}' as i32 {
        k5_buf_add_len(&mut buf, sp as *const libc::c_void,
                       1 as libc::c_int as size_t);
        if *sp as libc::c_int == (*fl).quotechar {
            k5_buf_add_len(&mut buf, sp as *const libc::c_void,
                           1 as libc::c_int as size_t);
        }
        sp = sp.offset(1)
    }
    return buf.data as *mut libc::c_char;
}
/*
 * Write an optionally quoted field.
 */
#[c2rust::src_loc = "101:1"]
unsafe extern "C" fn writequoted(mut h: *mut rechandle,
                                 mut fmt: *const libc::c_char,
                                 mut ap: ::std::ffi::VaList) -> libc::c_int {
    let mut doquote: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut qs: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fl: flavor = (*h).flavor;
    if fl.quotechar != '\u{0}' as i32 {
    } else {
        __assert_fail(b"fl.quotechar != \'\\0\'\x00" as *const u8 as
                          *const libc::c_char,
                      b"tdumputil.c\x00" as *const u8 as *const libc::c_char,
                      108 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 74],
                                                &[libc::c_char; 74]>(b"int writequoted(struct rechandle *, const char *, struct __va_list_tag *)\x00")).as_ptr());
    }
    ret = vasprintf(&mut s, fmt, ap.as_va_list());
    if ret < 0 as libc::c_int { return ret }
    if !strchr(s, fl.fieldsep).is_null() { doquote = 1 as libc::c_int }
    if !strchr(s, fl.recordsep).is_null() { doquote = 1 as libc::c_int }
    if !strchr(s, fl.quotechar).is_null() { doquote = 1 as libc::c_int }
    if doquote != 0 {
        qs = qquote(&mut fl, s);
        if qs.is_null() {
            ret = -(1 as libc::c_int)
        } else {
            ret =
                fprintf((*h).fh,
                        b"%c%s%c\x00" as *const u8 as *const libc::c_char,
                        fl.quotechar, qs, fl.quotechar)
        }
    } else {
        ret =
            fprintf((*h).fh, b"%s\x00" as *const u8 as *const libc::c_char, s)
    }
    free(s as *mut libc::c_void);
    free(qs as *mut libc::c_void);
    return ret;
}
/*
 * Return a rechandle with the requested file handle and rectype.
 *
 * rectype must be a valid pointer for the entire lifetime of the rechandle (or
 * null)
 */
#[c2rust::src_loc = "141:1"]
unsafe extern "C" fn rechandle_common(mut fh: *mut FILE,
                                      mut rectype: *const libc::c_char)
 -> *mut rechandle {
    let mut h: *mut rechandle =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<rechandle>() as libc::c_ulong) as
            *mut rechandle;
    if h.is_null() { return 0 as *mut rechandle }
    (*h).fh = fh;
    (*h).rectype = rectype;
    (*h).do_sep = 0 as libc::c_int;
    return h;
}
/*
 * Return a rechandle for tab-separated output.
 */
#[no_mangle]
#[c2rust::src_loc = "157:1"]
pub unsafe extern "C" fn rechandle_tabsep(mut fh: *mut FILE,
                                          mut rectype: *const libc::c_char)
 -> *mut rechandle {
    let mut h: *mut rechandle = rechandle_common(fh, rectype);
    if h.is_null() { return 0 as *mut rechandle }
    (*h).flavor = tabsep;
    return h;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kdc/tdumputil.h - utilities for tab-separated, etc. files */
/*
 * Copyright (C) 2015 by the Massachusetts Institute of Technology.
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
 * Return a rechandle for CSV output.
 */
#[no_mangle]
#[c2rust::src_loc = "171:1"]
pub unsafe extern "C" fn rechandle_csv(mut fh: *mut FILE,
                                       mut rectype: *const libc::c_char)
 -> *mut rechandle {
    let mut h: *mut rechandle = rechandle_common(fh, rectype);
    if h.is_null() { return 0 as *mut rechandle }
    (*h).flavor = csv;
    return h;
}
/*
 * Free a rechandle.
 */
#[no_mangle]
#[c2rust::src_loc = "185:1"]
pub unsafe extern "C" fn rechandle_free(mut h: *mut rechandle) {
    free(h as *mut libc::c_void);
}
/*
 * Start a record.  This includes writing a record type prefix (rectype) if
 * specified.
 */
#[no_mangle]
#[c2rust::src_loc = "195:1"]
pub unsafe extern "C" fn startrec(mut h: *mut rechandle) -> libc::c_int {
    if (*h).rectype.is_null() {
        (*h).do_sep = 0 as libc::c_int;
        return 0 as libc::c_int
    }
    (*h).do_sep = 1 as libc::c_int;
    return fputs((*h).rectype, (*h).fh);
}
/*
 * Write a single field of a record.  This includes writing a separator
 * character, if appropriate.
 */
#[no_mangle]
#[c2rust::src_loc = "210:1"]
pub unsafe extern "C" fn writefield(mut h: *mut rechandle,
                                    mut fmt: *const libc::c_char,
                                    mut args: ...) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut ap: ::std::ffi::VaListImpl;
    let mut fl: flavor = (*h).flavor;
    if (*h).do_sep != 0 {
        ret = fputc(fl.fieldsep, (*h).fh);
        if ret < 0 as libc::c_int { return ret }
    }
    (*h).do_sep = 1 as libc::c_int;
    ap = args.clone();
    if fl.quotechar == '\u{0}' as i32 {
        ret = vfprintf((*h).fh, fmt, ap.as_va_list())
    } else { ret = writequoted(h, fmt, ap.as_va_list()) }
    return ret;
}
/*
 * Finish a record (line).
 */
#[no_mangle]
#[c2rust::src_loc = "235:1"]
pub unsafe extern "C" fn endrec(mut h: *mut rechandle) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut fl: flavor = (*h).flavor;
    ret = fputc(fl.recordsep, (*h).fh);
    (*h).do_sep = 0 as libc::c_int;
    return ret;
}
/*
 * Write a header line if h->rectype is null.  (If rectype is set, it will be
 * prefixed to output lines, most likely in a mixed record type output file, so
 * it doesn't make sense to output a header line in that case.)
 */
#[no_mangle]
#[c2rust::src_loc = "251:1"]
pub unsafe extern "C" fn writeheader(mut h: *mut rechandle,
                                     mut a: *const *mut libc::c_char)
 -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut p: *const *mut libc::c_char = 0 as *const *mut libc::c_char;
    if !(*h).rectype.is_null() { return 0 as libc::c_int }
    p = a;
    while !(*p).is_null() {
        ret =
            writefield(h, b"%s\x00" as *const u8 as *const libc::c_char, *p);
        if ret < 0 as libc::c_int { return ret }
        p = p.offset(1)
    }
    ret = endrec(h);
    return ret;
}
