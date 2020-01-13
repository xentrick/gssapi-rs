use ::libc;
#[c2rust::header_src = "/usr/include/stdlib.h:31"]
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
#[c2rust::header_src = "/usr/include/errno.h:31"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/util/ss/ss.h:31"]
pub mod ss_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "53:1"]
        pub fn ss_error(_: libc::c_int, _: libc::c_long,
                        _: *const libc::c_char, _: ...);
    }
    /* _ss_h */
}
use self::stdlib_h::{malloc, realloc, free};
use self::errno_h::__errno_location;
use self::ss_h::ss_error;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2007 Massachusetts Institute of Technology.
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
 * Copyright 1987, 1988 by MIT Student Information Processing Board
 *
 * For copyright info, see copyright.h.
 */
#[c2rust::src_loc = "35:1"]
pub type parse_mode = libc::c_uint;
#[c2rust::src_loc = "35:38"]
pub const QUOTED_STRING: parse_mode = 2;
#[c2rust::src_loc = "35:31"]
pub const TOKEN: parse_mode = 1;
#[c2rust::src_loc = "35:19"]
pub const WHITESPACE: parse_mode = 0;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1987, 1988 by MIT Student Information Processing Board
 *
 * For copyright information, see copyright.h.
 */
/* LOCAL_ALLOC stuff */
/* abbrev name */
/* new tokens to insert */
/*    char *path; */
/* init values */
/* this subsystem */
/* current request info */
/* arg list */
/* primary name */
/* info directory for 'help' */
/* to be extracted by subroutines */
/* (void *) NULL */
/* for ss_listen processing */
/* to get out */
/* exit subsystem */
/*
 * parse(line_ptr, argc_ptr)
 *
 * Function:
 *      Parses line, dividing at whitespace, into tokens, returns
 *      the "argc" and "argv" values.
 * Arguments:
 *      line_ptr (char *)
 *              Pointer to text string to be parsed.
 *      argc_ptr (int *)
 *              Where to put the "argc" (number of tokens) value.
 * Returns:
 *      argv (char **)
 *              Series of pointers to parsed tokens in the original string.
 */
#[no_mangle]
#[c2rust::src_loc = "56:1"]
pub unsafe extern "C" fn ss_parse(mut sci_idx: libc::c_int,
                                  mut line_ptr: *mut libc::c_char,
                                  mut argc_ptr: *mut libc::c_int)
 -> *mut *mut libc::c_char {
    let mut argv: *mut *mut libc::c_char =
        0 as *mut *mut libc::c_char; /* flushing whitespace */
    let mut cp: *mut libc::c_char =
        0 as *mut libc::c_char; /* cp is for output */
    let mut newargv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut argc: libc::c_int = 0;
    let mut parse_mode: parse_mode = WHITESPACE;
    argv =
        malloc(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) as
            *mut *mut libc::c_char;
    if argv.is_null() {
        ss_error(sci_idx, *__errno_location() as libc::c_long,
                 b"Can\'t allocate storage\x00" as *const u8 as
                     *const libc::c_char);
        *argc_ptr = 0 as libc::c_int;
        return argv
    }
    *argv = 0 as *mut libc::c_void as *mut libc::c_char;
    argc = 0 as libc::c_int;
    parse_mode = WHITESPACE;
    cp = line_ptr;
    's_59:
        loop  {
            if parse_mode as libc::c_uint ==
                   WHITESPACE as libc::c_int as libc::c_uint {
                if *line_ptr as libc::c_int == '\u{0}' as i32 { break ; }
                if *line_ptr as libc::c_int == ' ' as i32 ||
                       *line_ptr as libc::c_int == '\t' as i32 {
                    line_ptr = line_ptr.offset(1)
                } else {
                    let mut current_block_26: u64;
                    if *line_ptr as libc::c_int == '\"' as i32 {
                        /* go to quoted-string mode */
                        parse_mode = QUOTED_STRING;
                        let fresh0 = line_ptr;
                        line_ptr = line_ptr.offset(1);
                        cp = fresh0;
                        newargv =
                            realloc(argv as *mut libc::c_char as
                                        *mut libc::c_void,
                                    ((argc + 2 as libc::c_int) as libc::c_uint
                                         as
                                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                                         as
                                                                         libc::c_ulong))
                                as *mut *mut libc::c_char;
                        if newargv.is_null() {
                            current_block_26 = 9581682943260839966;
                        } else {
                            argv = newargv;
                            let fresh1 = argc;
                            argc = argc + 1;
                            let ref mut fresh2 =
                                *argv.offset(fresh1 as isize);
                            *fresh2 = cp;
                            let ref mut fresh3 = *argv.offset(argc as isize);
                            *fresh3 = 0 as *mut libc::c_char;
                            current_block_26 = 1118134448028020070;
                        }
                    } else {
                        /* random-token mode */
                        parse_mode = TOKEN;
                        cp = line_ptr;
                        newargv =
                            realloc(argv as *mut libc::c_char as
                                        *mut libc::c_void,
                                    ((argc + 2 as libc::c_int) as libc::c_uint
                                         as
                                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                                         as
                                                                         libc::c_ulong))
                                as *mut *mut libc::c_char;
                        if newargv.is_null() {
                            current_block_26 = 9581682943260839966;
                        } else {
                            argv = newargv;
                            let fresh4 = argc;
                            argc = argc + 1;
                            let ref mut fresh5 =
                                *argv.offset(fresh4 as isize);
                            *fresh5 = line_ptr;
                            let ref mut fresh6 = *argv.offset(argc as isize);
                            *fresh6 = 0 as *mut libc::c_char;
                            current_block_26 = 1118134448028020070;
                        }
                    }
                    match current_block_26 {
                        1118134448028020070 => { }
                        _ => {
                            free(argv as *mut libc::c_void);
                            ss_error(sci_idx,
                                     *__errno_location() as libc::c_long,
                                     b"Can\'t allocate storage\x00" as
                                         *const u8 as *const libc::c_char);
                            *argc_ptr = 0 as libc::c_int;
                            return 0 as *mut *mut libc::c_char
                        }
                    }
                }
            } else {
                while parse_mode as libc::c_uint ==
                          TOKEN as libc::c_int as libc::c_uint {
                    if *line_ptr as libc::c_int == '\u{0}' as i32 {
                        let fresh7 = cp;
                        cp = cp.offset(1);
                        *fresh7 = '\u{0}' as i32 as libc::c_char;
                        break 's_59 ;
                    } else if *line_ptr as libc::c_int == ' ' as i32 ||
                                  *line_ptr as libc::c_int == '\t' as i32 {
                        let fresh8 = cp;
                        cp = cp.offset(1);
                        *fresh8 = '\u{0}' as i32 as libc::c_char;
                        line_ptr = line_ptr.offset(1);
                        parse_mode = WHITESPACE
                    } else if *line_ptr as libc::c_int == '\"' as i32 {
                        line_ptr = line_ptr.offset(1);
                        parse_mode = QUOTED_STRING
                    } else {
                        let fresh9 = line_ptr;
                        line_ptr = line_ptr.offset(1);
                        let fresh10 = cp;
                        cp = cp.offset(1);
                        *fresh10 = *fresh9
                    }
                }
                while parse_mode as libc::c_uint ==
                          QUOTED_STRING as libc::c_int as libc::c_uint {
                    if *line_ptr as libc::c_int == '\u{0}' as i32 {
                        ss_error(sci_idx, 0 as libc::c_int as libc::c_long,
                                 b"Unbalanced quotes in command line\x00" as
                                     *const u8 as *const libc::c_char);
                        free(argv as *mut libc::c_void);
                        *argc_ptr = 0 as libc::c_int;
                        return 0 as *mut *mut libc::c_char
                    } else {
                        if *line_ptr as libc::c_int == '\"' as i32 {
                            line_ptr = line_ptr.offset(1);
                            if *line_ptr as libc::c_int == '\"' as i32 {
                                let fresh11 = cp;
                                cp = cp.offset(1);
                                *fresh11 = '\"' as i32 as libc::c_char;
                                line_ptr = line_ptr.offset(1)
                            } else { parse_mode = TOKEN }
                        } else {
                            let fresh12 = line_ptr;
                            line_ptr = line_ptr.offset(1);
                            let fresh13 = cp;
                            cp = cp.offset(1);
                            *fresh13 = *fresh12
                        }
                    }
                }
            }
        }
    *argc_ptr = argc;
    return argv;
}
