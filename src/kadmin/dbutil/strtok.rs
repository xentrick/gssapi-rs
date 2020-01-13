use ::libc;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* Prototype for nstrtok */
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved
 *
 */
/*
 * Copyright (c) 1988 Regents of the University of California.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms are permitted
 * provided that: (1) source distributions retain this entire copyright
 * notice and comment, and (2) distributions including binaries display
 * the following acknowledgement:  ``This product includes software
 * developed by the University of California, Berkeley and its contributors''
 * in the documentation or other materials provided with the distribution
 * and in all advertising materials mentioning features or use of this
 * software. Neither the name of the University nor the names of its
 * contributors may be used to endorse or promote products derived
 * from this software without specific prior written permission.
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND WITHOUT ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE.
 */
/*
 * Function: nstrtok
 *
 * Purpose: the same as strtok ... just different. does not deal with
 *          multiple tokens in row.
 *
 * Arguments:
 *      s           (input) string to scan
 *      delim       (input) list of delimiters
 *      <return value> string or null on error.
 *
 * Requires:
 *      nuttin
 *
 * Effects:
 *      sets last to string
 *
 * Modifies:
 *      last
 *
 */
#[no_mangle]
#[c2rust::src_loc = "52:1"]
pub unsafe extern "C" fn nstrtok(mut s: *mut libc::c_char,
                                 mut delim: *const libc::c_char)
 -> *mut libc::c_char {
    let mut spanp: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: libc::c_int = 0;
    let mut sc: libc::c_int = 0;
    let mut tok: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut last: *mut libc::c_char =
        0 as *const libc::c_char as *mut libc::c_char;
    if s.is_null() && { s = last; s.is_null() } {
        return 0 as *mut libc::c_char
    }
    /*
     * Skip (span) leading delimiters (s += strspn(s, delim), sort of).
     */
    tok = s;
    loop 
         /*
     * Scan token (scan for delimiters: s += strcspn(s, delim), sort of).
     * Note that delim must have one NUL; we stop if we see that, too.
     */
         {
        let fresh0 = s;
        s = s.offset(1);
        c = *fresh0 as libc::c_int;
        spanp = delim;
        loop  {
            let fresh1 = spanp;
            spanp = spanp.offset(1);
            sc = *fresh1 as libc::c_int;
            if sc == c {
                if c == 0 as libc::c_int {
                    s = 0 as *mut libc::c_char
                } else {
                    *s.offset(-(1 as libc::c_int) as isize) =
                        0 as libc::c_int as libc::c_char
                }
                last = s;
                return tok
            }
            if !(sc != 0 as libc::c_int) { break ; }
        }
    };
    /* NOTREACHED */
}
