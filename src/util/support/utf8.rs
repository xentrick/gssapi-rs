use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:51"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:51"]
pub mod types_h {
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:51"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-utf8.h:52"]
pub mod k5_utf8_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2008 by the Massachusetts Institute of Technology,
 * Cambridge, MA, USA.  All Rights Reserved.
 *
 * This software is being provided to you, the LICENSEE, by the
 * Massachusetts Institute of Technology (M.I.T.) under the following
 * license.  By obtaining, using and/or copying this software, you agree
 * that you have read, understood, and will comply with these terms and
 * conditions:
 *
 * Export of this software from the United States of America may
 * require a specific license from the United States Government.
 * It is the responsibility of any person or organization contemplating
 * export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify and distribute
 * this software and its documentation for any purpose and without fee or
 * royalty is hereby granted, provided that you agree to comply with the
 * following copyright notice and statements, including the disclaimer, and
 * that the same appear on ALL copies of the software and documentation,
 * including modifications that you make for internal use or for
 * distribution:
 *
 * THIS SOFTWARE IS PROVIDED "AS IS", AND M.I.T. MAKES NO REPRESENTATIONS
 * OR WARRANTIES, EXPRESS OR IMPLIED.  By way of example, but not
 * limitation, M.I.T. MAKES NO REPRESENTATIONS OR WARRANTIES OF
 * MERCHANTABILITY OR FITNESS FOR ANY PARTICULAR PURPOSE OR THAT THE USE OF
 * THE LICENSED SOFTWARE OR DOCUMENTATION WILL NOT INFRINGE ANY THIRD PARTY
 * PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
 *
 * The name of the Massachusetts Institute of Technology or M.I.T. may NOT
 * be used in advertising or publicity pertaining to distribution of the
 * software.  Title to copyright in this software and any associated
 * documentation shall at all times remain with M.I.T., and USER agrees to
 * preserve same.
 *
 * Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 */
/*
 * Copyright 1998-2008 The OpenLDAP Foundation.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted only as authorized by the OpenLDAP
 * Public License.
 *
 * A copy of this license is available in file LICENSE in the
 * top-level directory of the distribution or, alternatively, at
 * <https://www.OpenLDAP.org/license.html>.
 */
/*
 * Copyright (C) 2000 Novell, Inc. All Rights Reserved.
 *
 * THIS WORK IS SUBJECT TO U.S. AND INTERNATIONAL COPYRIGHT LAWS AND TREATIES.
 * USE, MODIFICATION, AND REDISTRIBUTION OF THIS WORK IS SUBJECT TO VERSION
 * 2.0.1 OF THE OPENLDAP PUBLIC LICENSE, A COPY OF WHICH IS AVAILABLE AT
 * HTTPS://WWW.OPENLDAP.ORG/LICENSE.HTML OR IN THE FILE "LICENSE" IN THE
 * TOP-LEVEL DIRECTORY OF THE DISTRIBUTION. ANY USE OR EXPLOITATION OF THIS
 * WORK OTHER THAN AS AUTHORIZED IN VERSION 2.0.1 OF THE OPENLDAP PUBLIC
 * LICENSE, OR OTHER PRIOR WRITTEN CONSENT FROM NOVELL, COULD SUBJECT THE
 * PERPETRATOR TO CRIMINAL AND CIVIL LIABILITY.
 */
/* This work is part of OpenLDAP Software <https://www.openldap.org/>. */
    #[c2rust::src_loc = "73:1"]
    pub type krb5_ucs2 = uint16_t;
    #[c2rust::src_loc = "74:1"]
    pub type krb5_ucs4 = uint32_t;
    use super::stdint_uintn_h::{uint16_t, uint32_t};
    /* K5_UTF8_H */
    /*
 * these macros assume 'x' is an ASCII x
 * and assume the "C" locale
 */
    /* For symmetry */
    /* This is like CHARLEN but additionally validates to make sure
 * the char used the shortest possible encoding.
 * 'l' is used to temporarily hold the result of CHARLEN.
 */
    /* Optimizations */
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint16_t, __uint32_t};
pub use self::stdint_uintn_h::{uint16_t, uint32_t};
pub use self::k5_utf8_h::{krb5_ucs2, krb5_ucs4};
/* returns the number of bytes in the UTF-8 string */
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* util/support/utf8.c */
/*
 * Copyright 2008 by the Massachusetts Institute of Technology.
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
 * Copyright 1998-2008 The OpenLDAP Foundation.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted only as authorized by the OpenLDAP
 * Public License.
 *
 * A copy of this license is available in the file LICENSE in the
 * top-level directory of the distribution or, alternatively, at
 * <https://www.OpenLDAP.org/license.html>.
 */
/* This work is part of OpenLDAP Software <https://www.openldap.org/>. */
/* Basic UTF-8 routines
 *
 * These routines are "dumb".  Though they understand UTF-8,
 * they don't grok Unicode.  That is, they can push bits,
 * but don't have a clue what the bits represent.  That's
 * good enough for use with the KRB5 Client SDK.
 *
 * These routines are not optimized.
 */
/*
 * return the number of bytes required to hold the
 * NULL-terminated UTF-8 string NOT INCLUDING the
 * termination.
 */
#[no_mangle]
#[c2rust::src_loc = "60:1"]
pub unsafe extern "C" fn krb5int_utf8_bytes(mut p: *const libc::c_char)
 -> size_t {
    let mut bytes: size_t = 0;
    bytes = 0 as libc::c_int as size_t;
    while *p.offset(bytes as isize) != 0 { bytes = bytes.wrapping_add(1) }
    return bytes;
}
/* returns the number of UTF-8 characters in the string */
#[no_mangle]
#[c2rust::src_loc = "70:1"]
pub unsafe extern "C" fn krb5int_utf8_chars(mut p: *const libc::c_char)
 -> size_t {
    /* could be optimized and could check for invalid sequences */
    let mut chars: size_t = 0 as libc::c_int as size_t;
    while *p != 0 {
        chars = chars.wrapping_add(1);
        p =
            if *(p as *const libc::c_uchar) as libc::c_int &
                   0x80 as libc::c_int == 0 {
                (p as *mut libc::c_char).offset(1 as libc::c_int as isize)
            } else { krb5int_utf8_next(p) }
    }
    return chars;
}
/* returns the number of UTF-8 characters in the counted string */
#[no_mangle]
#[c2rust::src_loc = "81:1"]
pub unsafe extern "C" fn krb5int_utf8c_chars(mut p: *const libc::c_char,
                                             mut length: size_t) -> size_t {
    /* could be optimized and could check for invalid sequences */
    let mut chars: size_t = 0 as libc::c_int as size_t;
    let mut end: *const libc::c_char = p.offset(length as isize);
    while p < end {
        chars = chars.wrapping_add(1);
        p =
            if *(p as *const libc::c_uchar) as libc::c_int &
                   0x80 as libc::c_int == 0 {
                (p as *mut libc::c_char).offset(1 as libc::c_int as isize)
            } else { krb5int_utf8_next(p) }
    }
    return chars;
}
/* returns the length (in bytes) of the UTF-8 character */
/* return offset to next character */
#[no_mangle]
#[c2rust::src_loc = "94:1"]
pub unsafe extern "C" fn krb5int_utf8_offset(mut p: *const libc::c_char)
 -> libc::c_int {
    return (if *(p as *const libc::c_uchar) as libc::c_int &
                   0x80 as libc::c_int == 0 {
                (p as *mut libc::c_char).offset(1 as libc::c_int as isize)
            } else { krb5int_utf8_next(p) }).wrapping_offset_from(p) as
               libc::c_long as libc::c_int;
}
/*
 * Returns length indicated by first byte.
 */
#[no_mangle]
#[c2rust::src_loc = "102:12"]
pub static mut krb5int_utf8_lentab: [libc::c_char; 128] =
    [0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     4 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char,
     4 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char,
     4 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char];
/* returns the length (in bytes) indicated by the UTF-8 character */
#[no_mangle]
#[c2rust::src_loc = "112:1"]
pub unsafe extern "C" fn krb5int_utf8_charlen(mut p: *const libc::c_char)
 -> libc::c_int {
    if *p as libc::c_int & 0x80 as libc::c_int == 0 {
        return 1 as libc::c_int
    }
    return krb5int_utf8_lentab[(*(p as *const libc::c_uchar) as libc::c_int ^
                                    0x80 as libc::c_int) as usize] as
               libc::c_int;
}
/*
 * Make sure the UTF-8 char used the shortest possible encoding
 * returns charlen if valid, 0 if not.
 *
 * Here are the valid UTF-8 encodings, taken from RFC 3629 page 4.
 * The table is slightly modified from that of the RFC.
 *
 * UCS-4 range (hex)      UTF-8 sequence (binary)
 * 0000 0000-0000 007F   0.......
 * 0000 0080-0000 07FF   110++++. 10......
 * 0000 0800-0000 FFFF   1110++++ 10+..... 10......
 * 0001 0000-0010 FFFF   11110+++ 10++.... 10...... 10......
 *
 * The '.' bits are "don't cares". When validating a UTF-8 sequence,
 * at least one of the '+' bits must be set, otherwise the character
 * should have been encoded in fewer octets. Note that in the two-octet
 * case, only the first octet needs to be validated, and this is done
 * in the krb5int_utf8_lentab[] above.
 */
/* mask of required bits in second octet */
#[no_mangle]
#[c2rust::src_loc = "143:3"]
pub static mut krb5int_utf8_mintab: [libc::c_char; 32] =
    [0x20 as libc::c_int as libc::c_char, 0x80 as libc::c_int as libc::c_char,
     0x80 as libc::c_int as libc::c_char, 0x80 as libc::c_int as libc::c_char,
     0x80 as libc::c_int as libc::c_char, 0x80 as libc::c_int as libc::c_char,
     0x80 as libc::c_int as libc::c_char, 0x80 as libc::c_int as libc::c_char,
     0x80 as libc::c_int as libc::c_char, 0x80 as libc::c_int as libc::c_char,
     0x80 as libc::c_int as libc::c_char, 0x80 as libc::c_int as libc::c_char,
     0x80 as libc::c_int as libc::c_char, 0x80 as libc::c_int as libc::c_char,
     0x80 as libc::c_int as libc::c_char, 0x80 as libc::c_int as libc::c_char,
     0x30 as libc::c_int as libc::c_char, 0x80 as libc::c_int as libc::c_char,
     0x80 as libc::c_int as libc::c_char, 0x80 as libc::c_int as libc::c_char,
     0x80 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char];
/* returns the length (in bytes) indicated by the UTF-8 character
 * also checks that shortest possible encoding was used
 */
#[no_mangle]
#[c2rust::src_loc = "150:1"]
pub unsafe extern "C" fn krb5int_utf8_charlen2(mut p: *const libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int =
        if *(p as *const libc::c_uchar) as libc::c_int & 0x80 as libc::c_int
               == 0 {
            1 as libc::c_int
        } else {
            krb5int_utf8_lentab[(*(p as *const libc::c_uchar) as libc::c_int ^
                                     0x80 as libc::c_int) as usize] as
                libc::c_int
        };
    if i > 2 as libc::c_int {
        if krb5int_utf8_mintab[(*p as libc::c_int & 0x1f as libc::c_int) as
                                   usize] as libc::c_int &
               *p.offset(1 as libc::c_int as isize) as libc::c_int == 0 {
            i = 0 as libc::c_int
        }
    }
    return i;
}
/*
 * Convert a UTF8 character to a UCS4 character.  Return 0 on success,
 * -1 on failure.
 */
#[no_mangle]
#[c2rust::src_loc = "166:1"]
pub unsafe extern "C" fn krb5int_utf8_to_ucs4(mut p: *const libc::c_char,
                                              mut out: *mut krb5_ucs4)
 -> libc::c_int {
    let mut c: *const libc::c_uchar = p as *const libc::c_uchar;
    let mut ch: krb5_ucs4 = 0;
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    static mut mask: [libc::c_uchar; 5] =
        [0 as libc::c_int as libc::c_uchar,
         0x7f as libc::c_int as libc::c_uchar,
         0x1f as libc::c_int as libc::c_uchar,
         0xf as libc::c_int as libc::c_uchar,
         0x7 as libc::c_int as libc::c_uchar];
    *out = 0 as libc::c_int as krb5_ucs4;
    len =
        (if *(p as *const libc::c_uchar) as libc::c_int & 0x80 as libc::c_int
                == 0 {
             1 as libc::c_int
         } else {
             krb5int_utf8_lentab[(*(p as *const libc::c_uchar) as libc::c_int
                                      ^ 0x80 as libc::c_int) as usize] as
                 libc::c_int
         });
    len =
        if len < 3 as libc::c_int ||
               krb5int_utf8_mintab[(*(p as *const libc::c_uchar) as
                                        libc::c_int & 0x1f as libc::c_int) as
                                       usize] as libc::c_int &
                   *p.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
            len
        } else { 0 as libc::c_int };
    if len == 0 as libc::c_int { return -(1 as libc::c_int) }
    ch =
        (*c.offset(0 as libc::c_int as isize) as libc::c_int &
             mask[len as usize] as libc::c_int) as krb5_ucs4;
    i = 1 as libc::c_int;
    while i < len {
        if *c.offset(i as isize) as libc::c_int & 0xc0 as libc::c_int !=
               0x80 as libc::c_int {
            return -(1 as libc::c_int)
        }
        ch <<= 6 as libc::c_int;
        ch |=
            (*c.offset(i as isize) as libc::c_int & 0x3f as libc::c_int) as
                libc::c_uint;
        i += 1
    }
    if ch > 0x10ffff as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int)
    }
    *out = ch;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "197:1"]
pub unsafe extern "C" fn krb5int_utf8_to_ucs2(mut p: *const libc::c_char,
                                              mut out: *mut krb5_ucs2)
 -> libc::c_int {
    let mut ch: krb5_ucs4 = 0;
    *out = 0 as libc::c_int as krb5_ucs2;
    if krb5int_utf8_to_ucs4(p, &mut ch) == -(1 as libc::c_int) ||
           ch > 0xffff as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int)
    }
    *out = ch as krb5_ucs2;
    return 0 as libc::c_int;
}
/* conv UCS-4 to UTF-8 */
#[no_mangle]
#[c2rust::src_loc = "209:1"]
pub unsafe extern "C" fn krb5int_ucs4_to_utf8(mut c: krb5_ucs4,
                                              mut buf: *mut libc::c_char)
 -> size_t {
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut p: *mut libc::c_uchar = buf as *mut libc::c_uchar;
    /* not a valid Unicode character */
    if c > 0x10ffff as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as size_t
    }
    /* Just return length, don't convert */
    if buf.is_null() {
        if c < 0x80 as libc::c_int as libc::c_uint {
            return 1 as libc::c_int as size_t
        } else if c < 0x800 as libc::c_int as libc::c_uint {
            return 2 as libc::c_int as size_t
        } else if c < 0x10000 as libc::c_int as libc::c_uint {
            return 3 as libc::c_int as size_t
        } else { return 4 as libc::c_int as size_t }
    }
    if c < 0x80 as libc::c_int as libc::c_uint {
        let fresh0 = len;
        len = len.wrapping_add(1);
        *p.offset(fresh0 as isize) = c as libc::c_uchar
    } else if c < 0x800 as libc::c_int as libc::c_uint {
        let fresh1 = len;
        len = len.wrapping_add(1);
        *p.offset(fresh1 as isize) =
            (0xc0 as libc::c_int as libc::c_uint | c >> 6 as libc::c_int) as
                libc::c_uchar;
        let fresh2 = len;
        len = len.wrapping_add(1);
        *p.offset(fresh2 as isize) =
            (0x80 as libc::c_int as libc::c_uint |
                 c & 0x3f as libc::c_int as libc::c_uint) as libc::c_uchar
    } else if c < 0x10000 as libc::c_int as libc::c_uint {
        let fresh3 = len;
        len = len.wrapping_add(1);
        *p.offset(fresh3 as isize) =
            (0xe0 as libc::c_int as libc::c_uint | c >> 12 as libc::c_int) as
                libc::c_uchar;
        let fresh4 = len;
        len = len.wrapping_add(1);
        *p.offset(fresh4 as isize) =
            (0x80 as libc::c_int as libc::c_uint |
                 c >> 6 as libc::c_int & 0x3f as libc::c_int as libc::c_uint)
                as libc::c_uchar;
        let fresh5 = len;
        len = len.wrapping_add(1);
        *p.offset(fresh5 as isize) =
            (0x80 as libc::c_int as libc::c_uint |
                 c & 0x3f as libc::c_int as libc::c_uint) as libc::c_uchar
    } else {
        /* if (c < 0x110000) */
        let fresh6 = len;
        len = len.wrapping_add(1);
        *p.offset(fresh6 as isize) =
            (0xf0 as libc::c_int as libc::c_uint | c >> 18 as libc::c_int) as
                libc::c_uchar;
        let fresh7 = len;
        len = len.wrapping_add(1);
        *p.offset(fresh7 as isize) =
            (0x80 as libc::c_int as libc::c_uint |
                 c >> 12 as libc::c_int & 0x3f as libc::c_int as libc::c_uint)
                as libc::c_uchar;
        let fresh8 = len;
        len = len.wrapping_add(1);
        *p.offset(fresh8 as isize) =
            (0x80 as libc::c_int as libc::c_uint |
                 c >> 6 as libc::c_int & 0x3f as libc::c_int as libc::c_uint)
                as libc::c_uchar;
        let fresh9 = len;
        len = len.wrapping_add(1);
        *p.offset(fresh9 as isize) =
            (0x80 as libc::c_int as libc::c_uint |
                 c & 0x3f as libc::c_int as libc::c_uint) as libc::c_uchar
    }
    return len;
}
#[no_mangle]
#[c2rust::src_loc = "245:1"]
pub unsafe extern "C" fn krb5int_ucs2_to_utf8(mut c: krb5_ucs2,
                                              mut buf: *mut libc::c_char)
 -> size_t {
    return krb5int_ucs4_to_utf8(c as krb5_ucs4, buf);
}
/* returns pointer of next UTF-8 character in string */
/*
 * Advance to the next UTF-8 character
 *
 * Ignores length of multibyte character, instead rely on
 * continuation markers to find start of next character.
 * This allows for "resyncing" of when invalid characters
 * are provided provided the start of the next character
 * is appears within the 6 bytes examined.
 */
#[no_mangle]
#[c2rust::src_loc = "259:1"]
pub unsafe extern "C" fn krb5int_utf8_next(mut p: *const libc::c_char)
 -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut u: *const libc::c_uchar = p as *const libc::c_uchar;
    if *u as libc::c_int & 0x80 as libc::c_int == 0 {
        return &*p.offset(1 as libc::c_int as isize) as *const libc::c_char as
                   *mut libc::c_char
    }
    i = 1 as libc::c_int;
    while i < 6 as libc::c_int {
        if *u.offset(i as isize) as libc::c_int & 0xc0 as libc::c_int !=
               0x80 as libc::c_int {
            return &*p.offset(i as isize) as *const libc::c_char as
                       *mut libc::c_char
        }
        i += 1
    }
    return &*p.offset(i as isize) as *const libc::c_char as *mut libc::c_char;
}
/* returns pointer of previous UTF-8 character in string */
/*
 * Advance to the previous UTF-8 character
 *
 * Ignores length of multibyte character, instead rely on
 * continuation markers to find start of next character.
 * This allows for "resyncing" of when invalid characters
 * are provided provided the start of the next character
 * is appears within the 6 bytes examined.
 */
#[no_mangle]
#[c2rust::src_loc = "286:1"]
pub unsafe extern "C" fn krb5int_utf8_prev(mut p: *const libc::c_char)
 -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut u: *const libc::c_uchar = p as *const libc::c_uchar;
    i = -(1 as libc::c_int);
    while i > -(6 as libc::c_int) {
        if *u.offset(i as isize) as libc::c_int & 0xc0 as libc::c_int !=
               0x80 as libc::c_int {
            return &*p.offset(i as isize) as *const libc::c_char as
                       *mut libc::c_char
        }
        i -= 1
    }
    return &*p.offset(i as isize) as *const libc::c_char as *mut libc::c_char;
}
/* copies a UTF-8 character and returning number of bytes copied */
/*
 * Copy one UTF-8 character from src to dst returning
 * number of bytes copied.
 *
 * Ignores length of multibyte character, instead rely on
 * continuation markers to find start of next character.
 * This allows for "resyncing" of when invalid characters
 * are provided provided the start of the next character
 * is appears within the 6 bytes examined.
 */
#[no_mangle]
#[c2rust::src_loc = "310:1"]
pub unsafe extern "C" fn krb5int_utf8_copy(mut dst: *mut libc::c_char,
                                           mut src: *const libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut u: *const libc::c_uchar = src as *const libc::c_uchar;
    *dst.offset(0 as libc::c_int as isize) =
        *src.offset(0 as libc::c_int as isize);
    if *u as libc::c_int & 0x80 as libc::c_int == 0 {
        return 1 as libc::c_int
    }
    i = 1 as libc::c_int;
    while i < 6 as libc::c_int {
        if *u.offset(i as isize) as libc::c_int & 0xc0 as libc::c_int !=
               0x80 as libc::c_int {
            return i
        }
        *dst.offset(i as isize) = *src.offset(i as isize);
        i += 1
    }
    return i;
}
/* primitive ctype routines -- not aware of non-ascii characters */
/*
 * UTF-8 ctype routines
 * Only deals with characters < 0x80 (ie: US-ASCII)
 */
#[no_mangle]
#[c2rust::src_loc = "337:1"]
pub unsafe extern "C" fn krb5int_utf8_isascii(mut p: *const libc::c_char)
 -> libc::c_int {
    let mut c: libc::c_uint = *(p as *const libc::c_uchar) as libc::c_uint;
    return (c & 0x80 as libc::c_int as libc::c_uint == 0) as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "344:1"]
pub unsafe extern "C" fn krb5int_utf8_isdigit(mut p: *const libc::c_char)
 -> libc::c_int {
    let mut c: libc::c_uint = *(p as *const libc::c_uchar) as libc::c_uint;
    if c & 0x80 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int
    }
    return (c >= '0' as i32 as libc::c_uint &&
                c <= '9' as i32 as libc::c_uint) as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "354:1"]
pub unsafe extern "C" fn krb5int_utf8_isxdigit(mut p: *const libc::c_char)
 -> libc::c_int {
    let mut c: libc::c_uint = *(p as *const libc::c_uchar) as libc::c_uint;
    if c & 0x80 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int
    }
    return (c >= '0' as i32 as libc::c_uint && c <= '9' as i32 as libc::c_uint
                ||
                c >= 'a' as i32 as libc::c_uint &&
                    c <= 'f' as i32 as libc::c_uint ||
                c >= 'A' as i32 as libc::c_uint &&
                    c <= 'F' as i32 as libc::c_uint) as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "364:1"]
pub unsafe extern "C" fn krb5int_utf8_isspace(mut p: *const libc::c_char)
 -> libc::c_int {
    let mut c: libc::c_uint = *(p as *const libc::c_uchar) as libc::c_uint;
    if c & 0x80 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int
    }
    match c {
        32 | 9 | 10 | 13 | 11 | 12 => { return 1 as libc::c_int }
        _ => { }
    }
    return 0 as libc::c_int;
}
/*
 * These are not needed by the C SDK and are
 * not "good enough" for general use.
 */
#[no_mangle]
#[c2rust::src_loc = "388:1"]
pub unsafe extern "C" fn krb5int_utf8_isalpha(mut p: *const libc::c_char)
 -> libc::c_int {
    let mut c: libc::c_uint = *(p as *const libc::c_uchar) as libc::c_uint;
    if c & 0x80 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int
    }
    return (c >= 'a' as i32 as libc::c_uint && c <= 'z' as i32 as libc::c_uint
                ||
                c >= 'A' as i32 as libc::c_uint &&
                    c <= 'Z' as i32 as libc::c_uint) as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "398:1"]
pub unsafe extern "C" fn krb5int_utf8_isalnum(mut p: *const libc::c_char)
 -> libc::c_int {
    let mut c: libc::c_uint = *(p as *const libc::c_uchar) as libc::c_uint;
    if c & 0x80 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int
    }
    return (c >= 'a' as i32 as libc::c_uint && c <= 'z' as i32 as libc::c_uint
                ||
                c >= 'A' as i32 as libc::c_uint &&
                    c <= 'Z' as i32 as libc::c_uint ||
                c >= '0' as i32 as libc::c_uint &&
                    c <= '9' as i32 as libc::c_uint) as libc::c_int;
}
/* return first occurance of character in string */
/*
 * UTF-8 string routines
 */
/* like strchr() */
#[no_mangle]
#[c2rust::src_loc = "415:1"]
pub unsafe extern "C" fn krb5int_utf8_strchr(mut str: *const libc::c_char,
                                             mut chr: *const libc::c_char)
 -> *mut libc::c_char {
    let mut chs: krb5_ucs4 = 0;
    let mut ch: krb5_ucs4 = 0;
    if krb5int_utf8_to_ucs4(chr, &mut ch) == -(1 as libc::c_int) {
        return 0 as *mut libc::c_char
    }
    while *str as libc::c_int != '\u{0}' as i32 {
        if krb5int_utf8_to_ucs4(str, &mut chs) == 0 as libc::c_int &&
               chs == ch {
            return str as *mut libc::c_char
        }
        str =
            if *(str as *const libc::c_uchar) as libc::c_int &
                   0x80 as libc::c_int == 0 {
                (str as *mut libc::c_char).offset(1 as libc::c_int as isize)
            } else { krb5int_utf8_next(str) }
    }
    return 0 as *mut libc::c_char;
}
/* span characters not in set, return bytes spanned */
/* like strcspn() but returns number of bytes, not characters */
#[no_mangle]
#[c2rust::src_loc = "430:1"]
pub unsafe extern "C" fn krb5int_utf8_strcspn(mut str: *const libc::c_char,
                                              mut set: *const libc::c_char)
 -> size_t {
    let mut cstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut cset: *const libc::c_char = 0 as *const libc::c_char;
    let mut chstr: krb5_ucs4 = 0;
    let mut chset: krb5_ucs4 = 0;
    cstr = str;
    while *cstr as libc::c_int != '\u{0}' as i32 {
        cset = set;
        while *cset as libc::c_int != '\u{0}' as i32 {
            if krb5int_utf8_to_ucs4(cstr, &mut chstr) == 0 as libc::c_int &&
                   krb5int_utf8_to_ucs4(cset, &mut chset) == 0 as libc::c_int
                   && chstr == chset {
                return cstr.wrapping_offset_from(str) as libc::c_long as
                           size_t
            }
            cset =
                if *(cset as *const libc::c_uchar) as libc::c_int &
                       0x80 as libc::c_int == 0 {
                    (cset as
                         *mut libc::c_char).offset(1 as libc::c_int as isize)
                } else { krb5int_utf8_next(cset) }
        }
        cstr =
            if *(cstr as *const libc::c_uchar) as libc::c_int &
                   0x80 as libc::c_int == 0 {
                (cstr as *mut libc::c_char).offset(1 as libc::c_int as isize)
            } else { krb5int_utf8_next(cstr) }
    }
    return cstr.wrapping_offset_from(str) as libc::c_long as size_t;
}
/* span characters in set, return bytes spanned */
/* like strspn() but returns number of bytes, not characters */
#[no_mangle]
#[c2rust::src_loc = "447:1"]
pub unsafe extern "C" fn krb5int_utf8_strspn(mut str: *const libc::c_char,
                                             mut set: *const libc::c_char)
 -> size_t {
    let mut cstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut cset: *const libc::c_char = 0 as *const libc::c_char;
    let mut chstr: krb5_ucs4 = 0;
    let mut chset: krb5_ucs4 = 0;
    cstr = str;
    while *cstr as libc::c_int != '\u{0}' as i32 {
        cset = set;
        loop  {
            if *cset as libc::c_int == '\u{0}' as i32 {
                return cstr.wrapping_offset_from(str) as libc::c_long as
                           size_t
            }
            if krb5int_utf8_to_ucs4(cstr, &mut chstr) == 0 as libc::c_int &&
                   krb5int_utf8_to_ucs4(cset, &mut chset) == 0 as libc::c_int
                   && chstr == chset {
                break ;
            }
            cset =
                if *(cset as *const libc::c_uchar) as libc::c_int &
                       0x80 as libc::c_int == 0 {
                    (cset as
                         *mut libc::c_char).offset(1 as libc::c_int as isize)
                } else { krb5int_utf8_next(cset) }
        }
        cstr =
            if *(cstr as *const libc::c_uchar) as libc::c_int &
                   0x80 as libc::c_int == 0 {
                (cstr as *mut libc::c_char).offset(1 as libc::c_int as isize)
            } else { krb5int_utf8_next(cstr) }
    }
    return cstr.wrapping_offset_from(str) as libc::c_long as size_t;
}
/* return first character of set in string */
/* like strpbrk(), replaces strchr() as well */
#[no_mangle]
#[c2rust::src_loc = "466:1"]
pub unsafe extern "C" fn krb5int_utf8_strpbrk(mut str: *const libc::c_char,
                                              mut set: *const libc::c_char)
 -> *mut libc::c_char {
    let mut cset: *const libc::c_char = 0 as *const libc::c_char;
    let mut chstr: krb5_ucs4 = 0;
    let mut chset: krb5_ucs4 = 0;
    while *str as libc::c_int != '\u{0}' as i32 {
        cset = set;
        while *cset as libc::c_int != '\u{0}' as i32 {
            if krb5int_utf8_to_ucs4(str, &mut chstr) == 0 as libc::c_int &&
                   krb5int_utf8_to_ucs4(cset, &mut chset) == 0 as libc::c_int
                   && chstr == chset {
                return str as *mut libc::c_char
            }
            cset =
                if *(cset as *const libc::c_uchar) as libc::c_int &
                       0x80 as libc::c_int == 0 {
                    (cset as
                         *mut libc::c_char).offset(1 as libc::c_int as isize)
                } else { krb5int_utf8_next(cset) }
        }
        str =
            if *(str as *const libc::c_uchar) as libc::c_int &
                   0x80 as libc::c_int == 0 {
                (str as *mut libc::c_char).offset(1 as libc::c_int as isize)
            } else { krb5int_utf8_next(str) }
    }
    return 0 as *mut libc::c_char;
}
/* reentrant tokenizer */
/* like strtok_r(), not strtok() */
#[no_mangle]
#[c2rust::src_loc = "483:1"]
pub unsafe extern "C" fn krb5int_utf8_strtok(mut str: *mut libc::c_char,
                                             mut sep: *const libc::c_char,
                                             mut last: *mut *mut libc::c_char)
 -> *mut libc::c_char {
    let mut begin: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    if last.is_null() { return 0 as *mut libc::c_char }
    begin = if !str.is_null() { str } else { *last };
    begin = begin.offset(krb5int_utf8_strspn(begin, sep) as isize);
    if *begin as libc::c_int == '\u{0}' as i32 {
        *last = 0 as *mut libc::c_char;
        return 0 as *mut libc::c_char
    }
    end =
        &mut *begin.offset((krb5int_utf8_strcspn as
                                unsafe extern "C" fn(_: *const libc::c_char,
                                                     _: *const libc::c_char)
                                    -> size_t)(begin, sep) as isize) as
            *mut libc::c_char;
    if *end as libc::c_int != '\u{0}' as i32 {
        let mut next: *mut libc::c_char =
            if *(end as *const libc::c_uchar) as libc::c_int &
                   0x80 as libc::c_int == 0 {
                end.offset(1 as libc::c_int as isize)
            } else { krb5int_utf8_next(end) };
        *end = '\u{0}' as i32 as libc::c_char;
        end = next
    }
    *last = end;
    return begin;
}
