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
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stdarg.h:33"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-buf.h:34"]
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
    /* K5_BUF_H */
}
#[c2rust::header_src = "/usr/include/stdio.h:33"]
pub mod stdio_h {
    use super::internal::__va_list_tag;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "358:12"]
        pub fn vsnprintf(_: *mut libc::c_char, _: libc::c_ulong,
                         _: *const libc::c_char, _: ::std::ffi::VaList)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "366:1"]
        pub fn vasprintf(__ptr: *mut *mut libc::c_char,
                         __f: *const libc::c_char, __arg: ::std::ffi::VaList)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:33"]
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
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:33"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
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
pub use self::stddef_h::size_t;
pub use self::stdarg_h::va_list;
pub use self::k5_buf_h::{k5buftype, K5BUF_DYNAMIC_ZAP, K5BUF_DYNAMIC,
                         K5BUF_FIXED, K5BUF_ERROR, k5buf};
use self::stdio_h::{vsnprintf, vasprintf};
use self::string_h::{explicit_bzero, strlen, memcpy};
use self::stdlib_h::{malloc, realloc, free};
use self::assert_h::__assert_fail;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* util/support/k5buf.c - string buffer functions */
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
 * Can't include krb5.h here, or k5-int.h which includes it, because krb5.h
 * needs to be generated with error tables, after util/et, which builds after
 * this directory.
 */
/*
 * Structure invariants:
 *
 * buftype is K5BUF_FIXED, K5BUF_DYNAMIC, K5BUF_DYNAMIC_ZAP, or K5BUF_ERROR
 * if buftype is K5BUF_ERROR, the other fields are NULL or 0
 * if buftype is not K5BUF_ERROR:
 *   space > 0
 *   len < space
 *   data[len] = '\0'
 */
/* Return a character pointer to the current end of buf. */
#[inline]
#[c2rust::src_loc = "49:1"]
unsafe extern "C" fn endptr(mut buf: *mut k5buf) -> *mut libc::c_char {
    return ((*buf).data as *mut libc::c_char).offset((*buf).len as isize);
}
#[inline]
#[c2rust::src_loc = "55:1"]
unsafe extern "C" fn set_error(mut buf: *mut k5buf) {
    (*buf).buftype = K5BUF_ERROR;
    (*buf).data = 0 as *mut libc::c_void;
    (*buf).len = 0 as libc::c_int as size_t;
    (*buf).space = (*buf).len;
}
/*
 * Make sure there is room for LEN more characters in BUF, in addition to the
 * null terminator and what's already in there.  Return true on success.  On
 * failure, set the error flag and return false.
 */
#[c2rust::src_loc = "68:1"]
unsafe extern "C" fn ensure_space(mut buf: *mut k5buf, mut len: size_t)
 -> libc::c_int {
    let mut current_block: u64;
    let mut new_space: size_t = 0;
    let mut new_data: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*buf).buftype as libc::c_uint ==
           K5BUF_ERROR as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    if (*buf).space.wrapping_sub(1 as libc::c_int as
                                     libc::c_ulong).wrapping_sub((*buf).len)
           >= len {
        /* Enough room already. */
        return 1 as libc::c_int
    }
    if !((*buf).buftype as libc::c_uint ==
             K5BUF_FIXED as libc::c_int as libc::c_uint) {
        if (*buf).buftype as libc::c_uint ==
               K5BUF_DYNAMIC as libc::c_int as libc::c_uint ||
               (*buf).buftype as libc::c_uint ==
                   K5BUF_DYNAMIC_ZAP as libc::c_int as libc::c_uint {
        } else {
            __assert_fail(b"buf->buftype == K5BUF_DYNAMIC || buf->buftype == K5BUF_DYNAMIC_ZAP\x00"
                              as *const u8 as *const libc::c_char,
                          b"k5buf.c\x00" as *const u8 as *const libc::c_char,
                          80 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 41],
                                                    &[libc::c_char; 41]>(b"int ensure_space(struct k5buf *, size_t)\x00")).as_ptr());
        }
        new_space =
            (*buf).space.wrapping_mul(2 as libc::c_int as libc::c_ulong);
        loop  {
            if !(new_space.wrapping_sub((*buf).len).wrapping_sub(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
                     < len) {
                current_block = 11650488183268122163;
                break ;
            }
            if new_space >
                   (18446744073709551615 as
                        libc::c_ulong).wrapping_div(2 as libc::c_int as
                                                        libc::c_ulong) {
                current_block = 11305580990006235441;
                break ;
            }
            new_space =
                (new_space as
                     libc::c_ulong).wrapping_mul(2 as libc::c_int as
                                                     libc::c_ulong) as size_t
                    as size_t
        }
        match current_block {
            11305580990006235441 => { }
            _ => {
                if (*buf).buftype as libc::c_uint ==
                       K5BUF_DYNAMIC_ZAP as libc::c_int as libc::c_uint {
                    /* realloc() could leave behind a partial copy of sensitive data. */
                    new_data = malloc(new_space) as *mut libc::c_char;
                    if new_data.is_null() {
                        current_block = 11305580990006235441;
                    } else {
                        memcpy(new_data as *mut libc::c_void, (*buf).data,
                               (*buf).len);
                        *new_data.offset((*buf).len as isize) =
                            '\u{0}' as i32 as libc::c_char;
                        explicit_bzero((*buf).data, (*buf).len);
                        free((*buf).data);
                        current_block = 26972500619410423;
                    }
                } else {
                    new_data =
                        realloc((*buf).data, new_space) as *mut libc::c_char;
                    if new_data.is_null() {
                        current_block = 11305580990006235441;
                    } else { current_block = 26972500619410423; }
                }
                match current_block {
                    11305580990006235441 => { }
                    _ => {
                        (*buf).data = new_data as *mut libc::c_void;
                        (*buf).space = new_space;
                        return 1 as libc::c_int
                    }
                }
            }
        }
    }
    /* Can't resize a fixed buffer. */
    if (*buf).buftype as libc::c_uint ==
           K5BUF_DYNAMIC_ZAP as libc::c_int as libc::c_uint {
        explicit_bzero((*buf).data, (*buf).len);
    }
    if (*buf).buftype as libc::c_uint ==
           K5BUF_DYNAMIC_ZAP as libc::c_int as libc::c_uint ||
           (*buf).buftype as libc::c_uint ==
               K5BUF_DYNAMIC as libc::c_int as libc::c_uint {
        free((*buf).data);
    }
    set_error(buf);
    return 0 as libc::c_int;
}
/* Initialize a k5buf using a fixed-sized, existing buffer.  SPACE must be
 * more than zero, or an assertion failure will result. */
#[no_mangle]
#[c2rust::src_loc = "114:1"]
pub unsafe extern "C" fn k5_buf_init_fixed(mut buf: *mut k5buf,
                                           mut data: *mut libc::c_char,
                                           mut space: size_t) {
    if space > 0 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"space > 0\x00" as *const u8 as *const libc::c_char,
                      b"k5buf.c\x00" as *const u8 as *const libc::c_char,
                      117 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 55],
                                                &[libc::c_char; 55]>(b"void k5_buf_init_fixed(struct k5buf *, char *, size_t)\x00")).as_ptr());
    }
    (*buf).buftype = K5BUF_FIXED;
    (*buf).data = data as *mut libc::c_void;
    (*buf).space = space;
    (*buf).len = 0 as libc::c_int as size_t;
    *endptr(buf) = '\u{0}' as i32 as libc::c_char;
}
/* Initialize a k5buf using an internally allocated dynamic buffer. */
#[no_mangle]
#[c2rust::src_loc = "125:1"]
pub unsafe extern "C" fn k5_buf_init_dynamic(mut buf: *mut k5buf) {
    (*buf).buftype = K5BUF_DYNAMIC;
    (*buf).space = 128 as libc::c_int as size_t;
    (*buf).data = malloc((*buf).space);
    if (*buf).data.is_null() { set_error(buf); return }
    (*buf).len = 0 as libc::c_int as size_t;
    *endptr(buf) = '\u{0}' as i32 as libc::c_char;
}
/* Initialize a k5buf using an internally allocated dynamic buffer, zeroing
 * memory when reallocating or freeing. */
#[no_mangle]
#[c2rust::src_loc = "139:1"]
pub unsafe extern "C" fn k5_buf_init_dynamic_zap(mut buf: *mut k5buf) {
    k5_buf_init_dynamic(buf);
    if (*buf).buftype as libc::c_uint ==
           K5BUF_DYNAMIC as libc::c_int as libc::c_uint {
        (*buf).buftype = K5BUF_DYNAMIC_ZAP
    };
}
/* Add a C string to BUF. */
#[no_mangle]
#[c2rust::src_loc = "147:1"]
pub unsafe extern "C" fn k5_buf_add(mut buf: *mut k5buf,
                                    mut data: *const libc::c_char) {
    k5_buf_add_len(buf, data as *const libc::c_void, strlen(data));
}
/* Add a counted series of bytes to BUF. */
#[no_mangle]
#[c2rust::src_loc = "153:1"]
pub unsafe extern "C" fn k5_buf_add_len(mut buf: *mut k5buf,
                                        mut data: *const libc::c_void,
                                        mut len: size_t) {
    if ensure_space(buf, len) == 0 { return }
    if len > 0 as libc::c_int as libc::c_ulong {
        memcpy(endptr(buf) as *mut libc::c_void, data, len);
    }
    (*buf).len =
        ((*buf).len as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    *endptr(buf) = '\u{0}' as i32 as libc::c_char;
}
/* Add sprintf-style formatted data to BUF, with a va_list.  The value of ap is
 * undefined after the call. */
#[no_mangle]
#[c2rust::src_loc = "164:1"]
pub unsafe extern "C" fn k5_buf_add_vfmt(mut buf: *mut k5buf,
                                         mut fmt: *const libc::c_char,
                                         mut ap: ::std::ffi::VaList) {
    let mut apcopy: ::std::ffi::VaListImpl;
    let mut r: libc::c_int = 0;
    let mut remaining: size_t = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*buf).buftype as libc::c_uint ==
           K5BUF_ERROR as libc::c_int as libc::c_uint {
        return
    }
    remaining = (*buf).space.wrapping_sub((*buf).len);
    if (*buf).buftype as libc::c_uint ==
           K5BUF_FIXED as libc::c_int as libc::c_uint {
        /* Format the data directly into the fixed buffer. */
        r = vsnprintf(endptr(buf), remaining, fmt, ap.as_va_list());
        if r as libc::c_uint as libc::c_ulong >= remaining {
            set_error(buf);
        } else {
            (*buf).len =
                ((*buf).len as
                     libc::c_ulong).wrapping_add(r as libc::c_uint as
                                                     libc::c_ulong) as size_t
                    as size_t
        }
        return
    }
    /* Optimistically format the data directly into the dynamic buffer. */
    if (*buf).buftype as libc::c_uint ==
           K5BUF_DYNAMIC as libc::c_int as libc::c_uint ||
           (*buf).buftype as libc::c_uint ==
               K5BUF_DYNAMIC_ZAP as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"buf->buftype == K5BUF_DYNAMIC || buf->buftype == K5BUF_DYNAMIC_ZAP\x00"
                          as *const u8 as *const libc::c_char,
                      b"k5buf.c\x00" as *const u8 as *const libc::c_char,
                      187 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 75],
                                                &[libc::c_char; 75]>(b"void k5_buf_add_vfmt(struct k5buf *, const char *, struct __va_list_tag *)\x00")).as_ptr());
    }
    apcopy = ap.clone();
    r = vsnprintf(endptr(buf), remaining, fmt, apcopy.as_va_list());
    if !(r as libc::c_uint as libc::c_ulong >= remaining) {
        (*buf).len =
            ((*buf).len as
                 libc::c_ulong).wrapping_add(r as libc::c_uint as
                                                 libc::c_ulong) as size_t as
                size_t;
        return
    }
    if r >= 0 as libc::c_int {
        /* snprintf correctly told us how much space is required. */
        if ensure_space(buf, r as size_t) == 0 { return }
        remaining = (*buf).space.wrapping_sub((*buf).len);
        r = vsnprintf(endptr(buf), remaining, fmt, ap.as_va_list());
        if r as libc::c_uint as libc::c_ulong >= remaining {
            /* Shouldn't ever happen. */
            k5_buf_free(buf);
        } else {
            (*buf).len =
                ((*buf).len as
                     libc::c_ulong).wrapping_add(r as libc::c_uint as
                                                     libc::c_ulong) as size_t
                    as size_t
        }
        return
    }
    /* It's a pre-C99 snprintf implementation, or something else went wrong.
     * Fall back to asprintf. */
    r = vasprintf(&mut tmp, fmt, ap.as_va_list());
    if r < 0 as libc::c_int { k5_buf_free(buf); return }
    if ensure_space(buf, r as size_t) != 0 {
        /* Copy the temporary string into buf, including terminator. */
        memcpy(endptr(buf) as *mut libc::c_void, tmp as *const libc::c_void,
               (r + 1 as libc::c_int) as libc::c_ulong);
        (*buf).len =
            ((*buf).len as libc::c_ulong).wrapping_add(r as libc::c_ulong) as
                size_t as size_t
    }
    if (*buf).buftype as libc::c_uint ==
           K5BUF_DYNAMIC_ZAP as libc::c_int as libc::c_uint {
        explicit_bzero(tmp as *mut libc::c_void, strlen(tmp));
    }
    free(tmp as *mut libc::c_void);
}
/* Add sprintf-style formatted data to BUF. */
#[no_mangle]
#[c2rust::src_loc = "226:1"]
pub unsafe extern "C" fn k5_buf_add_fmt(mut buf: *mut k5buf,
                                        mut fmt: *const libc::c_char,
                                        mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    k5_buf_add_vfmt(buf, fmt, ap.as_va_list());
}
/* Extend the length of buf by len and return a pointer to the reserved space,
 * to be filled in by the caller.  Return NULL on error. */
#[no_mangle]
#[c2rust::src_loc = "236:1"]
pub unsafe extern "C" fn k5_buf_get_space(mut buf: *mut k5buf,
                                          mut len: size_t)
 -> *mut libc::c_void {
    if ensure_space(buf, len) == 0 { return 0 as *mut libc::c_void }
    (*buf).len =
        ((*buf).len as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    *endptr(buf) = '\u{0}' as i32 as libc::c_char;
    return endptr(buf).offset(-(len as isize)) as *mut libc::c_void;
}
/* Truncate BUF.  LEN must be between 0 and the existing buffer
 * length, or an assertion failure will result. */
#[no_mangle]
#[c2rust::src_loc = "246:1"]
pub unsafe extern "C" fn k5_buf_truncate(mut buf: *mut k5buf,
                                         mut len: size_t) {
    if (*buf).buftype as libc::c_uint ==
           K5BUF_ERROR as libc::c_int as libc::c_uint {
        return
    }
    if len <= (*buf).len {
    } else {
        __assert_fail(b"len <= buf->len\x00" as *const u8 as
                          *const libc::c_char,
                      b"k5buf.c\x00" as *const u8 as *const libc::c_char,
                      251 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 45],
                                                &[libc::c_char; 45]>(b"void k5_buf_truncate(struct k5buf *, size_t)\x00")).as_ptr());
    }
    (*buf).len = len;
    *endptr(buf) = '\u{0}' as i32 as libc::c_char;
}
/* Return ENOMEM if buf is in an error state, 0 otherwise. */
#[no_mangle]
#[c2rust::src_loc = "256:1"]
pub unsafe extern "C" fn k5_buf_status(mut buf: *mut k5buf) -> libc::c_int {
    return if (*buf).buftype as libc::c_uint ==
                  K5BUF_ERROR as libc::c_int as libc::c_uint {
               12 as libc::c_int
           } else { 0 as libc::c_int };
}
/*
 * Free the storage used in the dynamic buffer BUF.  The caller may choose to
 * take responsibility for freeing the data pointer instead of using this
 * function.  If BUF is a fixed buffer, an assertion failure will result.
 * Freeing a buffer in the error state, a buffer initialized with EMPTY_K5BUF,
 * or a zeroed k5buf structure is a no-op.
 */
#[no_mangle]
#[c2rust::src_loc = "262:1"]
pub unsafe extern "C" fn k5_buf_free(mut buf: *mut k5buf) {
    if (*buf).buftype as libc::c_uint ==
           K5BUF_ERROR as libc::c_int as libc::c_uint {
        return
    }
    if (*buf).buftype as libc::c_uint ==
           K5BUF_DYNAMIC as libc::c_int as libc::c_uint ||
           (*buf).buftype as libc::c_uint ==
               K5BUF_DYNAMIC_ZAP as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"buf->buftype == K5BUF_DYNAMIC || buf->buftype == K5BUF_DYNAMIC_ZAP\x00"
                          as *const u8 as *const libc::c_char,
                      b"k5buf.c\x00" as *const u8 as *const libc::c_char,
                      267 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"void k5_buf_free(struct k5buf *)\x00")).as_ptr());
    }
    if (*buf).buftype as libc::c_uint ==
           K5BUF_DYNAMIC_ZAP as libc::c_int as libc::c_uint {
        explicit_bzero((*buf).data, (*buf).len);
    }
    free((*buf).data);
    set_error(buf);
}
