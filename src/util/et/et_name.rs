use ::libc;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1997 by Massachusetts Institute of Technology
 *
 * Copyright 1987 by MIT Student Information Processing Board
 *
 * Permission to use, copy, modify, and distribute this software
 * and its documentation for any purpose and without fee is
 * hereby granted, provided that the above copyright notice
 * appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation,
 * and that the names of M.I.T. and the M.I.T. S.I.P.B. not be
 * used in advertising or publicity pertaining to distribution
 * of the software without specific, written prior permission.
 * Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 * M.I.T. and the M.I.T. S.I.P.B. make no representations about
 * the suitability of this software for any purpose.  It is
 * provided "as is" without express or implied warranty.
 */
#[c2rust::src_loc = "26:19"]
static mut char_set: [libc::c_char; 64] =
    [65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82,
     83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105,
     106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119,
     120, 121, 122, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 95, 0];
#[no_mangle]
#[c2rust::src_loc = "29:1"]
pub unsafe extern "C" fn error_table_name_r(mut num: libc::c_ulong,
                                            mut outbuf: *mut libc::c_char)
 -> *const libc::c_char 
 /*@modifies outbuf@*/
 {
    let mut ch: libc::c_long = 0;
    let mut i: libc::c_int = 0;
    /*@out@*/
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = outbuf;
    num >>= 8 as libc::c_int;
    i = 3 as libc::c_int;
    while i >= 0 as libc::c_int {
        ch =
            (num >> 6 as libc::c_int * i &
                 (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int)
                     as libc::c_ulong) as libc::c_long;
        if ch != 0 as libc::c_int as libc::c_long {
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 =
                char_set[(ch - 1 as libc::c_int as libc::c_long) as usize]
        }
        i -= 1
    }
    *p = '\u{0}' as i32 as libc::c_char;
    return outbuf;
}
/* # bits to shift per character in name */
/* Mask for maximum error table */
/*@observer@*/
#[no_mangle]
#[c2rust::src_loc = "51:1"]
pub unsafe extern "C" fn error_table_name(mut num: libc::c_ulong)
 -> *const libc::c_char 
 /*@modifies internalState@*/
 {
    static mut buf: [libc::c_char; 6] = [0; 6];
    return error_table_name_r(num, buf.as_mut_ptr());
}
