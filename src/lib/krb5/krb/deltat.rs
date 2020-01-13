use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:49"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:49"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:49"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "197:1"]
    pub type krb5_deltat = krb5_int32;
    /* *
 * Used to convey an operation status.  The value 0 indicates success; any
 * other values are com_err codes.  Use krb5_get_error_message() to obtain a
 * string describing the error.
 */
    #[c2rust::src_loc = "204:1"]
    pub type krb5_error_code = krb5_int32;
    use super::stdint_intn_h::int32_t;
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/ctype.h:50"]
pub mod ctype_h {
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed = 8192;
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed = 2048;
    #[c2rust::src_loc = "46:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed = 8;
    #[c2rust::src_loc = "58:3"]
    pub const _ISpunct: C2RustUnnamed = 4;
    #[c2rust::src_loc = "57:3"]
    pub const _IScntrl: C2RustUnnamed = 2;
    #[c2rust::src_loc = "56:3"]
    pub const _ISblank: C2RustUnnamed = 1;
    #[c2rust::src_loc = "55:3"]
    pub const _ISgraph: C2RustUnnamed = 32768;
    #[c2rust::src_loc = "54:3"]
    pub const _ISprint: C2RustUnnamed = 16384;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed = 4096;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed = 256;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:49"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
pub use self::types_h::__int32_t;
pub use self::stdint_intn_h::int32_t;
pub use self::krb5_h::{krb5_int32, krb5_deltat, krb5_error_code};
pub use self::ctype_h::{_ISspace, _ISdigit, C2RustUnnamed, _ISalnum, _ISpunct,
                        _IScntrl, _ISblank, _ISgraph, _ISprint, _ISxdigit,
                        _ISalpha, _ISlower, _ISupper, __ctype_b_loc};
use self::stdlib_h::{malloc, free};
/* A Bison parser, made by GNU Bison 3.0.4.  */
/* Bison implementation for Yacc-like parsers in C

   Copyright (C) 1984, 1989-1990, 2000-2015 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */
/* As a special exception, you may create a larger work that contains
   part or all of the Bison parser skeleton and distribute that work
   under terms of your choice, so long as that work isn't itself a
   parser generator using the skeleton or a modified version thereof
   as a parser skeleton.  Alternatively, if you modify or redistribute
   the parser skeleton itself, you may (at your option) remove this
   special exception, which will cause the skeleton and the resulting
   Bison output files to be licensed under the GNU General Public
   License without this special exception.

   This special exception was added by the Free Software Foundation in
   version 2.2 of Bison.  */
/* C LALR(1) parser skeleton written by Richard Stallman, by
   simplifying the original so-called "semantic" parser.  */
/* All symbols defined below should begin with yy or YY, to avoid
   infringing on user name space.  This should be done even for local
   variables, as they might otherwise be expanded by user macros.
   There are some unavoidable exceptions within include files to
   define necessary library symbols; they are noted "INFRINGES ON
   USER NAME SPACE" below.  */
/* yacc.c:339  */
/*
 * GCC optimizer will detect a variable used without being set in a YYERROR
 * path.  As this is generated code, suppress the complaint.
 */
/* Identify Bison output.  */
/* Bison version.  */
/* Skeleton name.  */
/* Pure parsers.  */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "52:8"]
pub struct param {
    pub delta: krb5_int32,
    pub p: *mut libc::c_char,
}
#[c2rust::src_loc = "245:1"]
pub type yytype_int16 = libc::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "197:7"]
pub union YYSTYPE {
    pub val: libc::c_int,
}
#[c2rust::src_loc = "227:1"]
pub type yytype_uint8 = libc::c_uchar;
#[c2rust::src_loc = "233:1"]
pub type yytype_int8 = libc::c_schar;
#[c2rust::src_loc = "190:5"]
pub const tok_WS: yytokentype = 261;
#[c2rust::src_loc = "187:5"]
pub const tok_NUM: yytokentype = 258;
#[c2rust::src_loc = "188:5"]
pub const tok_LONGNUM: yytokentype = 259;
#[c2rust::src_loc = "189:5"]
pub const tok_OVERFLOW: yytokentype = 260;
/* ! defined yyoverflow || YYERROR_VERBOSE */
/* A type that is properly aligned for any stack member.  */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "402:7"]
pub union yyalloc {
    pub yyss_alloc: yytype_int16,
    pub yyvs_alloc: YYSTYPE,
}
#[c2rust::src_loc = "185:3"]
pub type yytokentype = libc::c_uint;
/* An explanation of the tests being performed.
   We do not want to overflow a 32 bit integer with out manipulations,
   even for testing for overflow. Therefore we rely on the following:

   The lex parser will not return a number > MAX_TIME (which is out 32
   bit limit).

   Therefore, seconds (s) will require
       MIN_TIME < s < MAX_TIME

   For subsequent tests, the logic is as follows:

      If A < MAX_TIME and  B < MAX_TIME

      If we want to test if A+B < MAX_TIME, there are two cases
        if (A > 0)
         then A + B < MAX_TIME if B < MAX_TIME - A
	else A + B < MAX_TIME  always.

      if we want to test if MIN_TIME < A + B
          if A > 0 - then nothing to test
          otherwise, we test if MIN_TIME - A < B.

   We of course are testing for:
          MIN_TIME < A + B < MAX_TIME
*/
/* Overflow testing - this does not handle negative values well.. */
/* yacc.c:355  */
/* yacc.c:1646  */
/* yacc.c:1646  */
/* yacc.c:1646  */
/* yacc.c:1646  */
/* yacc.c:1646  */
/* yacc.c:1646  */
/* yacc.c:1646  */
/* yacc.c:1646  */
/* yacc.c:1646  */
/* yacc.c:1646  */
/* yacc.c:339  */
/* yacc.c:1646  */
/* yacc.c:1646  */
/* Enabling verbose error messages.  */
/* yacc.c:1646  */
/* yacc.c:1646  */
/* yacc.c:1906  */
/* Debug traces.  */
#[c2rust::src_loc = "178:1"]
unsafe extern "C" fn mylex(mut intp: *mut libc::c_int, mut tmv: *mut param)
 -> libc::c_int {
    let mut num: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    /* Token type.  */
    let mut orig_p: *mut libc::c_char = (*tmv).p;
    if !(*(*tmv).p as libc::c_int & !(0x7f as libc::c_int) ==
             0 as libc::c_int) {
        return 0 as libc::c_int
    }
    let fresh0 = (*tmv).p;
    (*tmv).p = (*tmv).p.offset(1);
    c = *fresh0 as libc::c_int;
    match c {
        45 | 58 | 100 | 104 | 109 | 115 => {
            /* Value type.  */
            return c
        }
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
            /* yacc.c:355  */
            /* XXX assumes ASCII */
            num = c - '0' as i32;
            while *(*__ctype_b_loc()).offset(*(*tmv).p as libc::c_int as
                                                 isize) as libc::c_int &
                      _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                      != 0 {
                if num > 2147483647 as libc::c_int / 10 as libc::c_int {
                    return tok_OVERFLOW as libc::c_int
                }
                num *= 10 as libc::c_int;
                if num >
                       2147483647 as libc::c_int -
                           (*(*tmv).p as libc::c_int - '0' as i32) {
                    return tok_OVERFLOW as libc::c_int
                }
                let fresh1 = (*tmv).p;
                (*tmv).p = (*tmv).p.offset(1);
                num += *fresh1 as libc::c_int - '0' as i32
                /* Copy the second part of user declarations.  */
            }
            *intp = num;
            return if (*tmv).p.wrapping_offset_from(orig_p) as libc::c_long >
                          2 as libc::c_int as libc::c_long {
                       tok_LONGNUM as libc::c_int
                   } else { tok_NUM as libc::c_int }
        }
        32 | 9 | 10 => {
            while *(*__ctype_b_loc()).offset(*(*tmv).p as libc::c_int as
                                                 isize) as libc::c_int &
                      _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                      != 0 {
                (*tmv).p = (*tmv).p.offset(1)
            }
            return tok_WS as libc::c_int
        }
        _ => { return 0 as libc::c_int }
    };
}
/* *
 * Convert a string to a checksum type.
 *
 * @param [in]  string          String to be converted
 * @param [out] cksumtypep      Checksum type to be filled in
 *
 * @retval 0  Success; otherwise - EINVAL
 */
/* *
 * Convert a string to a timestamp.
 *
 * @param [in]  string          String to be converted
 * @param [out] timestampp      Pointer to timestamp
 *
 * @retval 0  Success; otherwise - EINVAL
 */
/* *
 * Convert a string to a delta time value.
 *
 * @param [in]  string          String to be converted
 * @param [out] deltatp         Delta time to be filled in
 *
 * @retval 0  Success; otherwise - KRB5_DELTAT_BADFORMAT
 */
#[no_mangle]
#[c2rust::src_loc = "230:1"]
pub unsafe extern "C" fn krb5_string_to_deltat(mut string: *mut libc::c_char,
                                               mut deltatp: *mut krb5_deltat)
 -> krb5_error_code {
    let mut p: param = param{delta: 0, p: 0 as *mut libc::c_char,};
    p.delta = 0 as libc::c_int;
    p.p = string;
    if yyparse(&mut p) != 0 {
        return -(1765328136 as libc::c_long) as krb5_error_code
    }
    *deltatp = p.delta;
    return 0 as libc::c_int;
}
/* YYTRANSLATE[TOKEN-NUM] -- Symbol number corresponding to TOKEN-NUM
   as returned by yylex, without out-of-bounds checking.  */
#[c2rust::src_loc = "481:27"]
static mut yytranslate: [yytype_uint8; 262] =
    [0 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 6 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     7 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     8 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     9 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 10 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 11 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 4 as libc::c_int as yytype_uint8,
     5 as libc::c_int as yytype_uint8, 12 as libc::c_int as yytype_uint8];
/* YYPACT[STATE-NUM] -- Index in YYTABLE of the portion describing
     STATE-NUM.  */
#[c2rust::src_loc = "555:26"]
static mut yypact: [yytype_int8; 42] =
    [-(10 as libc::c_int) as yytype_int8, -(16 as libc::c_int) as yytype_int8,
     14 as libc::c_int as yytype_int8, 7 as libc::c_int as yytype_int8,
     -(2 as libc::c_int) as yytype_int8, -(16 as libc::c_int) as yytype_int8,
     -(16 as libc::c_int) as yytype_int8, -(16 as libc::c_int) as yytype_int8,
     -(16 as libc::c_int) as yytype_int8, -(16 as libc::c_int) as yytype_int8,
     21 as libc::c_int as yytype_int8, -(16 as libc::c_int) as yytype_int8,
     -(16 as libc::c_int) as yytype_int8, 13 as libc::c_int as yytype_int8,
     25 as libc::c_int as yytype_int8, -(10 as libc::c_int) as yytype_int8,
     -(10 as libc::c_int) as yytype_int8, -(10 as libc::c_int) as yytype_int8,
     -(16 as libc::c_int) as yytype_int8, -(16 as libc::c_int) as yytype_int8,
     22 as libc::c_int as yytype_int8, 23 as libc::c_int as yytype_int8,
     7 as libc::c_int as yytype_int8, 12 as libc::c_int as yytype_int8,
     -(16 as libc::c_int) as yytype_int8, -(16 as libc::c_int) as yytype_int8,
     -(16 as libc::c_int) as yytype_int8, 16 as libc::c_int as yytype_int8,
     -(16 as libc::c_int) as yytype_int8, 8 as libc::c_int as yytype_int8,
     -(16 as libc::c_int) as yytype_int8, 28 as libc::c_int as yytype_int8,
     29 as libc::c_int as yytype_int8, -(10 as libc::c_int) as yytype_int8,
     -(10 as libc::c_int) as yytype_int8, -(16 as libc::c_int) as yytype_int8,
     26 as libc::c_int as yytype_int8, -(16 as libc::c_int) as yytype_int8,
     -(16 as libc::c_int) as yytype_int8, -(16 as libc::c_int) as yytype_int8,
     32 as libc::c_int as yytype_int8, -(16 as libc::c_int) as yytype_int8];
/* YYDEFACT[STATE-NUM] -- Default reduction number in state STATE-NUM.
     Performed when YYTABLE does not specify something else to do.  Zero
     means the default is an error.  */
#[c2rust::src_loc = "567:27"]
static mut yydefact: [yytype_uint8; 42] =
    [7 as libc::c_int as yytype_uint8, 8 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     18 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     4 as libc::c_int as yytype_uint8, 10 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 5 as libc::c_int as yytype_uint8,
     9 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 7 as libc::c_int as yytype_uint8,
     7 as libc::c_int as yytype_uint8, 7 as libc::c_int as yytype_uint8,
     14 as libc::c_int as yytype_uint8, 6 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 17 as libc::c_int as yytype_uint8,
     23 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     11 as libc::c_int as yytype_uint8, 19 as libc::c_int as yytype_uint8,
     21 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     12 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     13 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 7 as libc::c_int as yytype_uint8,
     7 as libc::c_int as yytype_uint8, 24 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 16 as libc::c_int as yytype_uint8,
     20 as libc::c_int as yytype_uint8, 22 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 15 as libc::c_int as yytype_uint8];
/* YYPGOTO[NTERM-NUM].  */
#[c2rust::src_loc = "577:26"]
static mut yypgoto: [yytype_int8; 10] =
    [-(16 as libc::c_int) as yytype_int8, -(16 as libc::c_int) as yytype_int8,
     27 as libc::c_int as yytype_int8, -(16 as libc::c_int) as yytype_int8,
     36 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     -(16 as libc::c_int) as yytype_int8, -(16 as libc::c_int) as yytype_int8,
     -(15 as libc::c_int) as yytype_int8,
     -(14 as libc::c_int) as yytype_int8];
/* YYDEFGOTO[NTERM-NUM].  */
#[c2rust::src_loc = "583:26"]
static mut yydefgoto: [yytype_int8; 10] =
    [-(1 as libc::c_int) as yytype_int8, 2 as libc::c_int as yytype_int8,
     11 as libc::c_int as yytype_int8, 12 as libc::c_int as yytype_int8,
     22 as libc::c_int as yytype_int8, 27 as libc::c_int as yytype_int8,
     5 as libc::c_int as yytype_int8, 24 as libc::c_int as yytype_int8,
     25 as libc::c_int as yytype_int8, 26 as libc::c_int as yytype_int8];
/* YYTABLE[YYPACT[STATE-NUM]] -- What to do in state STATE-NUM.  If
     positive, shift that token.  If negative, reduce the rule whose
     number is the opposite.  If YYTABLE_NINF, syntax error.  */
#[c2rust::src_loc = "591:27"]
static mut yytable: [yytype_uint8; 38] =
    [4 as libc::c_int as yytype_uint8, 28 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 30 as libc::c_int as yytype_uint8,
     13 as libc::c_int as yytype_uint8, 14 as libc::c_int as yytype_uint8,
     15 as libc::c_int as yytype_uint8, 16 as libc::c_int as yytype_uint8,
     17 as libc::c_int as yytype_uint8, 18 as libc::c_int as yytype_uint8,
     7 as libc::c_int as yytype_uint8, 8 as libc::c_int as yytype_uint8,
     9 as libc::c_int as yytype_uint8, 10 as libc::c_int as yytype_uint8,
     6 as libc::c_int as yytype_uint8, 23 as libc::c_int as yytype_uint8,
     20 as libc::c_int as yytype_uint8, 29 as libc::c_int as yytype_uint8,
     38 as libc::c_int as yytype_uint8, 35 as libc::c_int as yytype_uint8,
     39 as libc::c_int as yytype_uint8, 33 as libc::c_int as yytype_uint8,
     34 as libc::c_int as yytype_uint8, 35 as libc::c_int as yytype_uint8,
     7 as libc::c_int as yytype_uint8, 8 as libc::c_int as yytype_uint8,
     34 as libc::c_int as yytype_uint8, 35 as libc::c_int as yytype_uint8,
     21 as libc::c_int as yytype_uint8, 31 as libc::c_int as yytype_uint8,
     32 as libc::c_int as yytype_uint8, 36 as libc::c_int as yytype_uint8,
     37 as libc::c_int as yytype_uint8, 40 as libc::c_int as yytype_uint8,
     29 as libc::c_int as yytype_uint8, 41 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 19 as libc::c_int as yytype_uint8];
#[c2rust::src_loc = "599:27"]
static mut yycheck: [yytype_uint8; 38] =
    [0 as libc::c_int as yytype_uint8, 16 as libc::c_int as yytype_uint8,
     12 as libc::c_int as yytype_uint8, 17 as libc::c_int as yytype_uint8,
     6 as libc::c_int as yytype_uint8, 7 as libc::c_int as yytype_uint8,
     8 as libc::c_int as yytype_uint8, 9 as libc::c_int as yytype_uint8,
     10 as libc::c_int as yytype_uint8, 11 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 4 as libc::c_int as yytype_uint8,
     5 as libc::c_int as yytype_uint8, 6 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 15 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 17 as libc::c_int as yytype_uint8,
     33 as libc::c_int as yytype_uint8, 11 as libc::c_int as yytype_uint8,
     34 as libc::c_int as yytype_uint8, 9 as libc::c_int as yytype_uint8,
     10 as libc::c_int as yytype_uint8, 11 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 4 as libc::c_int as yytype_uint8,
     10 as libc::c_int as yytype_uint8, 11 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 7 as libc::c_int as yytype_uint8,
     7 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 7 as libc::c_int as yytype_uint8,
     34 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 10 as libc::c_int as yytype_uint8];
/* YYSTOS[STATE-NUM] -- The (internal number of the) accessing
     symbol of state STATE-NUM.  */
#[c2rust::src_loc = "609:27"]
static mut yystos: [yytype_uint8; 42] =
    [0 as libc::c_int as yytype_uint8, 12 as libc::c_int as yytype_uint8,
     14 as libc::c_int as yytype_uint8, 17 as libc::c_int as yytype_uint8,
     18 as libc::c_int as yytype_uint8, 19 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     4 as libc::c_int as yytype_uint8, 5 as libc::c_int as yytype_uint8,
     6 as libc::c_int as yytype_uint8, 15 as libc::c_int as yytype_uint8,
     16 as libc::c_int as yytype_uint8, 6 as libc::c_int as yytype_uint8,
     7 as libc::c_int as yytype_uint8, 8 as libc::c_int as yytype_uint8,
     9 as libc::c_int as yytype_uint8, 10 as libc::c_int as yytype_uint8,
     11 as libc::c_int as yytype_uint8, 15 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     17 as libc::c_int as yytype_uint8, 18 as libc::c_int as yytype_uint8,
     20 as libc::c_int as yytype_uint8, 21 as libc::c_int as yytype_uint8,
     22 as libc::c_int as yytype_uint8, 18 as libc::c_int as yytype_uint8,
     21 as libc::c_int as yytype_uint8, 18 as libc::c_int as yytype_uint8,
     22 as libc::c_int as yytype_uint8, 7 as libc::c_int as yytype_uint8,
     7 as libc::c_int as yytype_uint8, 9 as libc::c_int as yytype_uint8,
     10 as libc::c_int as yytype_uint8, 11 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     21 as libc::c_int as yytype_uint8, 22 as libc::c_int as yytype_uint8,
     7 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8];
/* YYR1[YYN] -- Symbol number of symbol that rule YYN derives.  */
#[c2rust::src_loc = "619:27"]
static mut yyr1: [yytype_uint8; 25] =
    [0 as libc::c_int as yytype_uint8, 13 as libc::c_int as yytype_uint8,
     14 as libc::c_int as yytype_uint8, 15 as libc::c_int as yytype_uint8,
     15 as libc::c_int as yytype_uint8, 16 as libc::c_int as yytype_uint8,
     16 as libc::c_int as yytype_uint8, 17 as libc::c_int as yytype_uint8,
     17 as libc::c_int as yytype_uint8, 18 as libc::c_int as yytype_uint8,
     18 as libc::c_int as yytype_uint8, 19 as libc::c_int as yytype_uint8,
     19 as libc::c_int as yytype_uint8, 19 as libc::c_int as yytype_uint8,
     19 as libc::c_int as yytype_uint8, 19 as libc::c_int as yytype_uint8,
     19 as libc::c_int as yytype_uint8, 19 as libc::c_int as yytype_uint8,
     19 as libc::c_int as yytype_uint8, 20 as libc::c_int as yytype_uint8,
     20 as libc::c_int as yytype_uint8, 21 as libc::c_int as yytype_uint8,
     21 as libc::c_int as yytype_uint8, 22 as libc::c_int as yytype_uint8,
     22 as libc::c_int as yytype_uint8];
/* YYR2[YYN] -- Number of symbols on the right hand side of rule YYN.  */
#[c2rust::src_loc = "627:27"]
static mut yyr2: [yytype_uint8; 25] =
    [0 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 7 as libc::c_int as yytype_uint8,
     5 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8];
/* YYERROR_VERBOSE */
/*-----------------------------------------------.
| Release the memory associated to this symbol.  |
`-----------------------------------------------*/
#[c2rust::src_loc = "1044:1"]
unsafe extern "C" fn yydestruct(mut yymsg: *const libc::c_char,
                                mut yytype: libc::c_int,
                                mut yyvaluep: *mut YYSTYPE,
                                mut tmv: *mut param) {
    if yymsg.is_null() {
        yymsg = b"Deleting\x00" as *const u8 as *const libc::c_char
    };
}
/*----------.
| yyparse.  |
`----------*/
#[c2rust::src_loc = "1065:1"]
unsafe extern "C" fn yyparse(mut tmv: *mut param) -> libc::c_int {
    let mut current_block: u64;
    /* The lookahead symbol.  */
    let mut yychar: libc::c_int = 0;
    /* The semantic value of the lookahead symbol.  */
/* Default value used for initialization, for pacifying older GCCs
   or non-GCC compilers.  */
    static mut yyval_default: YYSTYPE = YYSTYPE{val: 0,};
    let mut yylval: YYSTYPE = yyval_default;
    /* Number of syntax errors so far.  */
    let mut yynerrs: libc::c_int = 0;
    let mut yystate: libc::c_int = 0;
    /* Number of tokens to shift before error messages enabled.  */
    let mut yyerrstatus: libc::c_int = 0;
    /* The stacks and their tools:
       'yyss': related to states.
       'yyvs': related to semantic values.

       Refer to the stacks through separate pointers, to allow yyoverflow
       to reallocate them elsewhere.  */
    /* The state stack.  */
    let mut yyssa: [yytype_int16; 200] = [0; 200];
    let mut yyss: *mut yytype_int16 = 0 as *mut yytype_int16;
    let mut yyssp: *mut yytype_int16 = 0 as *mut yytype_int16;
    /* The semantic value stack.  */
    let mut yyvsa: [YYSTYPE; 200] = [YYSTYPE{val: 0,}; 200];
    let mut yyvs: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yyvsp: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yystacksize: libc::c_ulong = 0;
    let mut yyn: libc::c_int = 0;
    let mut yyresult: libc::c_int = 0;
    /* Lookahead token as an internal (translated) token number.  */
    let mut yytoken: libc::c_int = 0 as libc::c_int;
    /* The variables used to return semantic value and location from the
     action routines.  */
    let mut yyval: YYSTYPE = YYSTYPE{val: 0,};
    /* The number of symbols on the RHS of the reduced rule.
     Keep to zero when no symbol should be popped.  */
    let mut yylen: libc::c_int =
        0 as libc::c_int; /* Cause a token to be read.  */
    yyss = yyssa.as_mut_ptr();
    yyssp = yyss;
    yyvs = yyvsa.as_mut_ptr();
    yyvsp = yyvs;
    yystacksize = 200 as libc::c_int as libc::c_ulong;
    yystate = 0 as libc::c_int;
    yyerrstatus = 0 as libc::c_int;
    yynerrs = 0 as libc::c_int;
    yychar = -(2 as libc::c_int);
    'c_6913:
        loop  {
            *yyssp = yystate as yytype_int16;
            if yyss.offset(yystacksize as
                               isize).offset(-(1 as libc::c_int as isize)) <=
                   yyssp {
                /* Get the current used size of the three stacks, in elements.  */
                let mut yysize: libc::c_ulong =
                    (yyssp.wrapping_offset_from(yyss) as libc::c_long +
                         1 as libc::c_int as libc::c_long) as libc::c_ulong;
                /* no yyoverflow */
                /* Extend the stack our own way.  */
                if 10000 as libc::c_int as libc::c_ulong <= yystacksize {
                    current_block = 3244376785761602731;
                    break ;
                }
                yystacksize =
                    yystacksize.wrapping_mul(2 as libc::c_int as
                                                 libc::c_ulong);
                if (10000 as libc::c_int as libc::c_ulong) < yystacksize {
                    yystacksize = 10000 as libc::c_int as libc::c_ulong
                }
                let mut yyss1: *mut yytype_int16 = yyss;
                let mut yyptr: *mut yyalloc =
                    malloc(yystacksize.wrapping_mul((::std::mem::size_of::<yytype_int16>()
                                                         as
                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<YYSTYPE>()
                                                                                         as
                                                                                         libc::c_ulong)).wrapping_add((::std::mem::size_of::<yyalloc>()
                                                                                                                           as
                                                                                                                           libc::c_ulong).wrapping_sub(1
                                                                                                                                                           as
                                                                                                                                                           libc::c_int
                                                                                                                                                           as
                                                                                                                                                           libc::c_ulong)))
                        as *mut yyalloc;
                if yyptr.is_null() {
                    current_block = 3244376785761602731;
                    break ;
                }
                let mut yynewbytes: libc::c_ulong = 0;
                libc::memcpy(&mut (*yyptr).yyss_alloc as *mut yytype_int16 as
                                 *mut libc::c_void,
                             yyss as *const libc::c_void,
                             yysize.wrapping_mul(::std::mem::size_of::<yytype_int16>()
                                                     as libc::c_ulong) as
                                 libc::size_t);
                yyss = &mut (*yyptr).yyss_alloc;
                yynewbytes =
                    yystacksize.wrapping_mul(::std::mem::size_of::<yytype_int16>()
                                                 as
                                                 libc::c_ulong).wrapping_add((::std::mem::size_of::<yyalloc>()
                                                                                  as
                                                                                  libc::c_ulong).wrapping_sub(1
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_ulong));
                yyptr =
                    yyptr.offset(yynewbytes.wrapping_div(::std::mem::size_of::<yyalloc>()
                                                             as libc::c_ulong)
                                     as isize);
                let mut yynewbytes_0: libc::c_ulong = 0;
                libc::memcpy(&mut (*yyptr).yyvs_alloc as *mut YYSTYPE as
                                 *mut libc::c_void,
                             yyvs as *const libc::c_void,
                             yysize.wrapping_mul(::std::mem::size_of::<YYSTYPE>()
                                                     as libc::c_ulong) as
                                 libc::size_t);
                yyvs = &mut (*yyptr).yyvs_alloc;
                yynewbytes_0 =
                    yystacksize.wrapping_mul(::std::mem::size_of::<YYSTYPE>()
                                                 as
                                                 libc::c_ulong).wrapping_add((::std::mem::size_of::<yyalloc>()
                                                                                  as
                                                                                  libc::c_ulong).wrapping_sub(1
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_ulong));
                yyptr =
                    yyptr.offset(yynewbytes_0.wrapping_div(::std::mem::size_of::<yyalloc>()
                                                               as
                                                               libc::c_ulong)
                                     as isize);
                if yyss1 != yyssa.as_mut_ptr() {
                    free(yyss1 as *mut libc::c_void);
                }
                /* no yyoverflow */
                yyssp =
                    yyss.offset(yysize as
                                    isize).offset(-(1 as libc::c_int as
                                                        isize));
                yyvsp =
                    yyvs.offset(yysize as
                                    isize).offset(-(1 as libc::c_int as
                                                        isize));
                if yyss.offset(yystacksize as
                                   isize).offset(-(1 as libc::c_int as isize))
                       <= yyssp {
                    current_block = 9910440355210482038;
                    break ;
                }
            }
            if yystate == 6 as libc::c_int {
                /*-------------------------------------.
| yyacceptlab -- YYACCEPT comes here.  |
`-------------------------------------*/
                yyresult = 0 as libc::c_int;
                current_block = 335923787858262782;
                break ;
            } else {
                /*-----------.
| yybackup.  |
`-----------*/
                /* Do appropriate processing given the current state.  Read a
     lookahead token if we need one and don't already have one.  */
                /* First try to decide what to do without reference to lookahead token.  */
                yyn = yypact[yystate as usize] as libc::c_int;
                if yyn == -(16 as libc::c_int) {
                    current_block = 5853511527084183066;
                } else {
                    /* Not known => get a lookahead token if don't already have one.  */
                    /* YYCHAR is either YYEMPTY or YYEOF or a valid lookahead symbol.  */
                    if yychar == -(2 as libc::c_int) {
                        yychar = mylex(&mut yylval.val, tmv)
                    }
                    if yychar <= 0 as libc::c_int {
                        yytoken = 0 as libc::c_int;
                        yychar = yytoken
                    } else {
                        yytoken =
                            if yychar as libc::c_uint <=
                                   261 as libc::c_int as libc::c_uint {
                                yytranslate[yychar as usize] as libc::c_int
                            } else { 2 as libc::c_int }
                    }
                    /* If the proper action on seeing token YYTOKEN is to reduce or to
     detect an error, take that action.  */
                    yyn += yytoken;
                    if yyn < 0 as libc::c_int || (37 as libc::c_int) < yyn ||
                           yycheck[yyn as usize] as libc::c_int != yytoken {
                        current_block = 5853511527084183066;
                    } else {
                        yyn = yytable[yyn as usize] as libc::c_int;
                        if yyn <= 0 as libc::c_int {
                            yyn = -yyn;
                            current_block = 9607599782166492281;
                        } else {
                            /* Count tokens shifted since error; after three, turn off error
     status.  */
                            if yyerrstatus != 0 { yyerrstatus -= 1 }
                            /* Shift the lookahead token.  */
                            /* Discard the shifted token.  */
                            yychar = -(2 as libc::c_int);
                            yystate = yyn;
                            yyvsp = yyvsp.offset(1);
                            *yyvsp = yylval;
                            current_block = 1885938290812153035;
                        }
                    }
                }
                match current_block {
                    5853511527084183066 =>
                    /*-----------------------------------------------------------.
| yydefault -- do the default action for the current state.  |
`-----------------------------------------------------------*/
                    {
                        yyn = yydefact[yystate as usize] as libc::c_int;
                        if yyn == 0 as libc::c_int {
                            /*--------------------------------------.
| yyerrlab -- here on detecting error.  |
`--------------------------------------*/
                            /* Make sure we have latest lookahead translation.  See comments at
     user semantic actions for why this is necessary.  */
                            yytoken =
                                if yychar == -(2 as libc::c_int) {
                                    -(2 as libc::c_int)
                                } else if yychar as libc::c_uint <=
                                              261 as libc::c_int as
                                                  libc::c_uint {
                                    yytranslate[yychar as usize] as
                                        libc::c_int
                                } else { 2 as libc::c_int };
                            /* If not already recovering from an error, report this error.  */
                            if yyerrstatus == 0 { yynerrs += 1 }
                            if yyerrstatus == 3 as libc::c_int {
                                /* If just tried and failed to reuse lookahead token after an
         error, discard it.  */
                                if yychar <= 0 as libc::c_int {
                                    /* Return failure if at end of input.  */
                                    if yychar == 0 as libc::c_int {
                                        current_block = 9910440355210482038;
                                        break ;
                                    }
                                } else {
                                    yydestruct(b"Error: discarding\x00" as
                                                   *const u8 as
                                                   *const libc::c_char,
                                               yytoken, &mut yylval, tmv);
                                    yychar = -(2 as libc::c_int)
                                }
                            }
                            /* Else will try to reuse lookahead token after shifting the error
     token.  */
                            current_block = 17367163409548420750;
                        } else { current_block = 9607599782166492281; }
                    }
                    _ => { }
                }
                match current_block {
                    9607599782166492281 =>
                    /*-----------------------------.
| yyreduce -- Do a reduction.  |
`-----------------------------*/
                    /* yyn is the number of a rule to reduce with.  */
                    {
                        yylen = yyr2[yyn as usize] as libc::c_int;
                        /* If YYLEN is nonzero, implement the default value of the action:
     '$$ = $1'.

     Otherwise, the following line sets YYVAL to garbage.
     This behavior is undocumented and Bison
     users should not rely upon it.  Assigning to YYVAL
     unconditionally makes the parser a bit smaller, and it avoids a
     GCC warning that YYVAL may be used uninitialized.  */
                        yyval =
                            *yyvsp.offset((1 as libc::c_int - yylen) as
                                              isize);
                        match yyn {
                            6 => {
                                yyval.val =
                                    -(*yyvsp.offset(0 as libc::c_int as
                                                        isize)).val;
                                /* yacc.c:1646  */
                                current_block = 5127850565928544452;
                            }
                            9 => {
                                yyval.val =
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).val;
                                /* yacc.c:1646  */
                                current_block = 5127850565928544452;
                            }
                            10 => { current_block = 11510301253690344784; }
                            11 => {
                                if (*yyvsp.offset(-(2 as libc::c_int) as
                                                      isize)).val >
                                       2147483647 as libc::c_int /
                                           (24 as libc::c_int *
                                                3600 as libc::c_int) ||
                                       (*yyvsp.offset(-(2 as libc::c_int) as
                                                          isize)).val <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               (24 as libc::c_int *
                                                    3600 as libc::c_int) ||
                                       0 as libc::c_int >
                                           2147483647 as libc::c_int /
                                               3600 as libc::c_int ||
                                       (0 as libc::c_int) <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               3600 as libc::c_int ||
                                       0 as libc::c_int >
                                           2147483647 as libc::c_int /
                                               60 as libc::c_int ||
                                       (0 as libc::c_int) <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               60 as libc::c_int {
                                    current_block = 11510301253690344784;
                                } else {
                                    (*tmv).delta =
                                        (*yyvsp.offset(-(2 as libc::c_int) as
                                                           isize)).val *
                                            (24 as libc::c_int *
                                                 3600 as libc::c_int);
                                    if if (*tmv).delta > 0 as libc::c_int {
                                           (0 as libc::c_int *
                                                3600 as libc::c_int <=
                                                2147483647 as libc::c_int -
                                                    (*tmv).delta) as
                                               libc::c_int
                                       } else {
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int -
                                                (*tmv).delta <=
                                                0 as libc::c_int *
                                                    3600 as libc::c_int) as
                                               libc::c_int
                                       } == 0 {
                                        current_block = 11510301253690344784;
                                    } else {
                                        (*tmv).delta =
                                            (*tmv).delta +
                                                0 as libc::c_int *
                                                    3600 as libc::c_int;
                                        if if (*tmv).delta > 0 as libc::c_int
                                              {
                                               (0 as libc::c_int *
                                                    60 as libc::c_int <=
                                                    2147483647 as libc::c_int
                                                        - (*tmv).delta) as
                                                   libc::c_int
                                           } else {
                                               (-(2147483647 as libc::c_int) -
                                                    1 as libc::c_int -
                                                    (*tmv).delta <=
                                                    0 as libc::c_int *
                                                        60 as libc::c_int) as
                                                   libc::c_int
                                           } == 0 {
                                            current_block =
                                                11510301253690344784;
                                        } else {
                                            (*tmv).delta =
                                                (*tmv).delta +
                                                    0 as libc::c_int *
                                                        60 as libc::c_int;
                                            if if (*tmv).delta >
                                                      0 as libc::c_int {
                                                   ((*yyvsp.offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)).val
                                                        <=
                                                        2147483647 as
                                                            libc::c_int -
                                                            (*tmv).delta) as
                                                       libc::c_int
                                               } else {
                                                   (-(2147483647 as
                                                          libc::c_int) -
                                                        1 as libc::c_int -
                                                        (*tmv).delta <=
                                                        (*yyvsp.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)).val)
                                                       as libc::c_int
                                               } == 0 {
                                                current_block =
                                                    11510301253690344784;
                                            } else {
                                                (*tmv).delta =
                                                    (*tmv).delta +
                                                        (*yyvsp.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)).val;
                                                /* yacc.c:1646  */
                                                current_block =
                                                    5127850565928544452;
                                            }
                                        }
                                    }
                                }
                            }
                            12 => {
                                if 0 as libc::c_int >
                                       2147483647 as libc::c_int /
                                           (24 as libc::c_int *
                                                3600 as libc::c_int) ||
                                       (0 as libc::c_int) <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               (24 as libc::c_int *
                                                    3600 as libc::c_int) ||
                                       (*yyvsp.offset(-(2 as libc::c_int) as
                                                          isize)).val >
                                           2147483647 as libc::c_int /
                                               3600 as libc::c_int ||
                                       (*yyvsp.offset(-(2 as libc::c_int) as
                                                          isize)).val <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               3600 as libc::c_int ||
                                       0 as libc::c_int >
                                           2147483647 as libc::c_int /
                                               60 as libc::c_int ||
                                       (0 as libc::c_int) <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               60 as libc::c_int {
                                    current_block = 11510301253690344784;
                                } else {
                                    (*tmv).delta =
                                        0 as libc::c_int *
                                            (24 as libc::c_int *
                                                 3600 as libc::c_int);
                                    if if (*tmv).delta > 0 as libc::c_int {
                                           ((*yyvsp.offset(-(2 as libc::c_int)
                                                               as isize)).val
                                                * 3600 as libc::c_int <=
                                                2147483647 as libc::c_int -
                                                    (*tmv).delta) as
                                               libc::c_int
                                       } else {
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int -
                                                (*tmv).delta <=
                                                (*yyvsp.offset(-(2 as
                                                                     libc::c_int)
                                                                   as
                                                                   isize)).val
                                                    * 3600 as libc::c_int) as
                                               libc::c_int
                                       } == 0 {
                                        current_block = 11510301253690344784;
                                    } else {
                                        (*tmv).delta =
                                            (*tmv).delta +
                                                (*yyvsp.offset(-(2 as
                                                                     libc::c_int)
                                                                   as
                                                                   isize)).val
                                                    * 3600 as libc::c_int;
                                        if if (*tmv).delta > 0 as libc::c_int
                                              {
                                               (0 as libc::c_int *
                                                    60 as libc::c_int <=
                                                    2147483647 as libc::c_int
                                                        - (*tmv).delta) as
                                                   libc::c_int
                                           } else {
                                               (-(2147483647 as libc::c_int) -
                                                    1 as libc::c_int -
                                                    (*tmv).delta <=
                                                    0 as libc::c_int *
                                                        60 as libc::c_int) as
                                                   libc::c_int
                                           } == 0 {
                                            current_block =
                                                11510301253690344784;
                                        } else {
                                            (*tmv).delta =
                                                (*tmv).delta +
                                                    0 as libc::c_int *
                                                        60 as libc::c_int;
                                            if if (*tmv).delta >
                                                      0 as libc::c_int {
                                                   ((*yyvsp.offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)).val
                                                        <=
                                                        2147483647 as
                                                            libc::c_int -
                                                            (*tmv).delta) as
                                                       libc::c_int
                                               } else {
                                                   (-(2147483647 as
                                                          libc::c_int) -
                                                        1 as libc::c_int -
                                                        (*tmv).delta <=
                                                        (*yyvsp.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)).val)
                                                       as libc::c_int
                                               } == 0 {
                                                current_block =
                                                    11510301253690344784;
                                            } else {
                                                (*tmv).delta =
                                                    (*tmv).delta +
                                                        (*yyvsp.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)).val;
                                                /* yacc.c:1646  */
                                                current_block =
                                                    5127850565928544452;
                                            }
                                        }
                                    }
                                }
                            }
                            13 => {
                                if 0 as libc::c_int >
                                       2147483647 as libc::c_int /
                                           (24 as libc::c_int *
                                                3600 as libc::c_int) ||
                                       (0 as libc::c_int) <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               (24 as libc::c_int *
                                                    3600 as libc::c_int) ||
                                       0 as libc::c_int >
                                           2147483647 as libc::c_int /
                                               3600 as libc::c_int ||
                                       (0 as libc::c_int) <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               3600 as libc::c_int ||
                                       (*yyvsp.offset(-(2 as libc::c_int) as
                                                          isize)).val >
                                           2147483647 as libc::c_int /
                                               60 as libc::c_int ||
                                       (*yyvsp.offset(-(2 as libc::c_int) as
                                                          isize)).val <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               60 as libc::c_int {
                                    current_block = 11510301253690344784;
                                } else {
                                    (*tmv).delta =
                                        0 as libc::c_int *
                                            (24 as libc::c_int *
                                                 3600 as libc::c_int);
                                    if if (*tmv).delta > 0 as libc::c_int {
                                           (0 as libc::c_int *
                                                3600 as libc::c_int <=
                                                2147483647 as libc::c_int -
                                                    (*tmv).delta) as
                                               libc::c_int
                                       } else {
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int -
                                                (*tmv).delta <=
                                                0 as libc::c_int *
                                                    3600 as libc::c_int) as
                                               libc::c_int
                                       } == 0 {
                                        current_block = 11510301253690344784;
                                    } else {
                                        (*tmv).delta =
                                            (*tmv).delta +
                                                0 as libc::c_int *
                                                    3600 as libc::c_int;
                                        if if (*tmv).delta > 0 as libc::c_int
                                              {
                                               ((*yyvsp.offset(-(2 as
                                                                     libc::c_int)
                                                                   as
                                                                   isize)).val
                                                    * 60 as libc::c_int <=
                                                    2147483647 as libc::c_int
                                                        - (*tmv).delta) as
                                                   libc::c_int
                                           } else {
                                               (-(2147483647 as libc::c_int) -
                                                    1 as libc::c_int -
                                                    (*tmv).delta <=
                                                    (*yyvsp.offset(-(2 as
                                                                         libc::c_int)
                                                                       as
                                                                       isize)).val
                                                        * 60 as libc::c_int)
                                                   as libc::c_int
                                           } == 0 {
                                            current_block =
                                                11510301253690344784;
                                        } else {
                                            (*tmv).delta =
                                                (*tmv).delta +
                                                    (*yyvsp.offset(-(2 as
                                                                         libc::c_int)
                                                                       as
                                                                       isize)).val
                                                        * 60 as libc::c_int;
                                            if if (*tmv).delta >
                                                      0 as libc::c_int {
                                                   ((*yyvsp.offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)).val
                                                        <=
                                                        2147483647 as
                                                            libc::c_int -
                                                            (*tmv).delta) as
                                                       libc::c_int
                                               } else {
                                                   (-(2147483647 as
                                                          libc::c_int) -
                                                        1 as libc::c_int -
                                                        (*tmv).delta <=
                                                        (*yyvsp.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)).val)
                                                       as libc::c_int
                                               } == 0 {
                                                current_block =
                                                    11510301253690344784;
                                            } else {
                                                (*tmv).delta =
                                                    (*tmv).delta +
                                                        (*yyvsp.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)).val;
                                                /* yacc.c:1646  */
                                                current_block =
                                                    5127850565928544452;
                                            }
                                        }
                                    }
                                }
                            }
                            14 => {
                                if 0 as libc::c_int >
                                       2147483647 as libc::c_int /
                                           (24 as libc::c_int *
                                                3600 as libc::c_int) ||
                                       (0 as libc::c_int) <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               (24 as libc::c_int *
                                                    3600 as libc::c_int) ||
                                       0 as libc::c_int >
                                           2147483647 as libc::c_int /
                                               3600 as libc::c_int ||
                                       (0 as libc::c_int) <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               3600 as libc::c_int ||
                                       0 as libc::c_int >
                                           2147483647 as libc::c_int /
                                               60 as libc::c_int ||
                                       (0 as libc::c_int) <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               60 as libc::c_int {
                                    current_block = 11510301253690344784;
                                } else {
                                    (*tmv).delta =
                                        0 as libc::c_int *
                                            (24 as libc::c_int *
                                                 3600 as libc::c_int);
                                    if if (*tmv).delta > 0 as libc::c_int {
                                           (0 as libc::c_int *
                                                3600 as libc::c_int <=
                                                2147483647 as libc::c_int -
                                                    (*tmv).delta) as
                                               libc::c_int
                                       } else {
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int -
                                                (*tmv).delta <=
                                                0 as libc::c_int *
                                                    3600 as libc::c_int) as
                                               libc::c_int
                                       } == 0 {
                                        current_block = 11510301253690344784;
                                    } else {
                                        (*tmv).delta =
                                            (*tmv).delta +
                                                0 as libc::c_int *
                                                    3600 as libc::c_int;
                                        if if (*tmv).delta > 0 as libc::c_int
                                              {
                                               (0 as libc::c_int *
                                                    60 as libc::c_int <=
                                                    2147483647 as libc::c_int
                                                        - (*tmv).delta) as
                                                   libc::c_int
                                           } else {
                                               (-(2147483647 as libc::c_int) -
                                                    1 as libc::c_int -
                                                    (*tmv).delta <=
                                                    0 as libc::c_int *
                                                        60 as libc::c_int) as
                                                   libc::c_int
                                           } == 0 {
                                            current_block =
                                                11510301253690344784;
                                        } else {
                                            (*tmv).delta =
                                                (*tmv).delta +
                                                    0 as libc::c_int *
                                                        60 as libc::c_int;
                                            if if (*tmv).delta >
                                                      0 as libc::c_int {
                                                   ((*yyvsp.offset(-(1 as
                                                                         libc::c_int)
                                                                       as
                                                                       isize)).val
                                                        <=
                                                        2147483647 as
                                                            libc::c_int -
                                                            (*tmv).delta) as
                                                       libc::c_int
                                               } else {
                                                   (-(2147483647 as
                                                          libc::c_int) -
                                                        1 as libc::c_int -
                                                        (*tmv).delta <=
                                                        (*yyvsp.offset(-(1 as
                                                                             libc::c_int)
                                                                           as
                                                                           isize)).val)
                                                       as libc::c_int
                                               } == 0 {
                                                current_block =
                                                    11510301253690344784;
                                            } else {
                                                (*tmv).delta =
                                                    (*tmv).delta +
                                                        (*yyvsp.offset(-(1 as
                                                                             libc::c_int)
                                                                           as
                                                                           isize)).val;
                                                /* yacc.c:1646  */
                                                current_block =
                                                    5127850565928544452;
                                            }
                                        }
                                    }
                                }
                            }
                            15 => {
                                if (*yyvsp.offset(-(6 as libc::c_int) as
                                                      isize)).val >
                                       2147483647 as libc::c_int /
                                           (24 as libc::c_int *
                                                3600 as libc::c_int) ||
                                       (*yyvsp.offset(-(6 as libc::c_int) as
                                                          isize)).val <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               (24 as libc::c_int *
                                                    3600 as libc::c_int) ||
                                       (*yyvsp.offset(-(4 as libc::c_int) as
                                                          isize)).val >
                                           2147483647 as libc::c_int /
                                               3600 as libc::c_int ||
                                       (*yyvsp.offset(-(4 as libc::c_int) as
                                                          isize)).val <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               3600 as libc::c_int ||
                                       (*yyvsp.offset(-(2 as libc::c_int) as
                                                          isize)).val >
                                           2147483647 as libc::c_int /
                                               60 as libc::c_int ||
                                       (*yyvsp.offset(-(2 as libc::c_int) as
                                                          isize)).val <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               60 as libc::c_int {
                                    current_block = 11510301253690344784;
                                } else {
                                    (*tmv).delta =
                                        (*yyvsp.offset(-(6 as libc::c_int) as
                                                           isize)).val *
                                            (24 as libc::c_int *
                                                 3600 as libc::c_int);
                                    if if (*tmv).delta > 0 as libc::c_int {
                                           ((*yyvsp.offset(-(4 as libc::c_int)
                                                               as isize)).val
                                                * 3600 as libc::c_int <=
                                                2147483647 as libc::c_int -
                                                    (*tmv).delta) as
                                               libc::c_int
                                       } else {
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int -
                                                (*tmv).delta <=
                                                (*yyvsp.offset(-(4 as
                                                                     libc::c_int)
                                                                   as
                                                                   isize)).val
                                                    * 3600 as libc::c_int) as
                                               libc::c_int
                                       } == 0 {
                                        current_block = 11510301253690344784;
                                    } else {
                                        (*tmv).delta =
                                            (*tmv).delta +
                                                (*yyvsp.offset(-(4 as
                                                                     libc::c_int)
                                                                   as
                                                                   isize)).val
                                                    * 3600 as libc::c_int;
                                        if if (*tmv).delta > 0 as libc::c_int
                                              {
                                               ((*yyvsp.offset(-(2 as
                                                                     libc::c_int)
                                                                   as
                                                                   isize)).val
                                                    * 60 as libc::c_int <=
                                                    2147483647 as libc::c_int
                                                        - (*tmv).delta) as
                                                   libc::c_int
                                           } else {
                                               (-(2147483647 as libc::c_int) -
                                                    1 as libc::c_int -
                                                    (*tmv).delta <=
                                                    (*yyvsp.offset(-(2 as
                                                                         libc::c_int)
                                                                       as
                                                                       isize)).val
                                                        * 60 as libc::c_int)
                                                   as libc::c_int
                                           } == 0 {
                                            current_block =
                                                11510301253690344784;
                                        } else {
                                            (*tmv).delta =
                                                (*tmv).delta +
                                                    (*yyvsp.offset(-(2 as
                                                                         libc::c_int)
                                                                       as
                                                                       isize)).val
                                                        * 60 as libc::c_int;
                                            if if (*tmv).delta >
                                                      0 as libc::c_int {
                                                   ((*yyvsp.offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)).val
                                                        <=
                                                        2147483647 as
                                                            libc::c_int -
                                                            (*tmv).delta) as
                                                       libc::c_int
                                               } else {
                                                   (-(2147483647 as
                                                          libc::c_int) -
                                                        1 as libc::c_int -
                                                        (*tmv).delta <=
                                                        (*yyvsp.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)).val)
                                                       as libc::c_int
                                               } == 0 {
                                                current_block =
                                                    11510301253690344784;
                                            } else {
                                                (*tmv).delta =
                                                    (*tmv).delta +
                                                        (*yyvsp.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)).val;
                                                /* yacc.c:1646  */
                                                current_block =
                                                    5127850565928544452;
                                            }
                                        }
                                    }
                                }
                            }
                            16 => {
                                if 0 as libc::c_int >
                                       2147483647 as libc::c_int /
                                           (24 as libc::c_int *
                                                3600 as libc::c_int) ||
                                       (0 as libc::c_int) <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               (24 as libc::c_int *
                                                    3600 as libc::c_int) ||
                                       (*yyvsp.offset(-(4 as libc::c_int) as
                                                          isize)).val >
                                           2147483647 as libc::c_int /
                                               3600 as libc::c_int ||
                                       (*yyvsp.offset(-(4 as libc::c_int) as
                                                          isize)).val <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               3600 as libc::c_int ||
                                       (*yyvsp.offset(-(2 as libc::c_int) as
                                                          isize)).val >
                                           2147483647 as libc::c_int /
                                               60 as libc::c_int ||
                                       (*yyvsp.offset(-(2 as libc::c_int) as
                                                          isize)).val <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               60 as libc::c_int {
                                    current_block = 11510301253690344784;
                                } else {
                                    (*tmv).delta =
                                        0 as libc::c_int *
                                            (24 as libc::c_int *
                                                 3600 as libc::c_int);
                                    if if (*tmv).delta > 0 as libc::c_int {
                                           ((*yyvsp.offset(-(4 as libc::c_int)
                                                               as isize)).val
                                                * 3600 as libc::c_int <=
                                                2147483647 as libc::c_int -
                                                    (*tmv).delta) as
                                               libc::c_int
                                       } else {
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int -
                                                (*tmv).delta <=
                                                (*yyvsp.offset(-(4 as
                                                                     libc::c_int)
                                                                   as
                                                                   isize)).val
                                                    * 3600 as libc::c_int) as
                                               libc::c_int
                                       } == 0 {
                                        current_block = 11510301253690344784;
                                    } else {
                                        (*tmv).delta =
                                            (*tmv).delta +
                                                (*yyvsp.offset(-(4 as
                                                                     libc::c_int)
                                                                   as
                                                                   isize)).val
                                                    * 3600 as libc::c_int;
                                        if if (*tmv).delta > 0 as libc::c_int
                                              {
                                               ((*yyvsp.offset(-(2 as
                                                                     libc::c_int)
                                                                   as
                                                                   isize)).val
                                                    * 60 as libc::c_int <=
                                                    2147483647 as libc::c_int
                                                        - (*tmv).delta) as
                                                   libc::c_int
                                           } else {
                                               (-(2147483647 as libc::c_int) -
                                                    1 as libc::c_int -
                                                    (*tmv).delta <=
                                                    (*yyvsp.offset(-(2 as
                                                                         libc::c_int)
                                                                       as
                                                                       isize)).val
                                                        * 60 as libc::c_int)
                                                   as libc::c_int
                                           } == 0 {
                                            current_block =
                                                11510301253690344784;
                                        } else {
                                            (*tmv).delta =
                                                (*tmv).delta +
                                                    (*yyvsp.offset(-(2 as
                                                                         libc::c_int)
                                                                       as
                                                                       isize)).val
                                                        * 60 as libc::c_int;
                                            if if (*tmv).delta >
                                                      0 as libc::c_int {
                                                   ((*yyvsp.offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)).val
                                                        <=
                                                        2147483647 as
                                                            libc::c_int -
                                                            (*tmv).delta) as
                                                       libc::c_int
                                               } else {
                                                   (-(2147483647 as
                                                          libc::c_int) -
                                                        1 as libc::c_int -
                                                        (*tmv).delta <=
                                                        (*yyvsp.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)).val)
                                                       as libc::c_int
                                               } == 0 {
                                                current_block =
                                                    11510301253690344784;
                                            } else {
                                                (*tmv).delta =
                                                    (*tmv).delta +
                                                        (*yyvsp.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)).val;
                                                /* yacc.c:1646  */
                                                current_block =
                                                    5127850565928544452;
                                            }
                                        }
                                    }
                                }
                            }
                            17 => {
                                if 0 as libc::c_int >
                                       2147483647 as libc::c_int /
                                           (24 as libc::c_int *
                                                3600 as libc::c_int) ||
                                       (0 as libc::c_int) <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               (24 as libc::c_int *
                                                    3600 as libc::c_int) ||
                                       (*yyvsp.offset(-(2 as libc::c_int) as
                                                          isize)).val >
                                           2147483647 as libc::c_int /
                                               3600 as libc::c_int ||
                                       (*yyvsp.offset(-(2 as libc::c_int) as
                                                          isize)).val <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               3600 as libc::c_int ||
                                       (*yyvsp.offset(0 as libc::c_int as
                                                          isize)).val >
                                           2147483647 as libc::c_int /
                                               60 as libc::c_int ||
                                       (*yyvsp.offset(0 as libc::c_int as
                                                          isize)).val <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               60 as libc::c_int {
                                    current_block = 11510301253690344784;
                                } else {
                                    (*tmv).delta =
                                        0 as libc::c_int *
                                            (24 as libc::c_int *
                                                 3600 as libc::c_int);
                                    if if (*tmv).delta > 0 as libc::c_int {
                                           ((*yyvsp.offset(-(2 as libc::c_int)
                                                               as isize)).val
                                                * 3600 as libc::c_int <=
                                                2147483647 as libc::c_int -
                                                    (*tmv).delta) as
                                               libc::c_int
                                       } else {
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int -
                                                (*tmv).delta <=
                                                (*yyvsp.offset(-(2 as
                                                                     libc::c_int)
                                                                   as
                                                                   isize)).val
                                                    * 3600 as libc::c_int) as
                                               libc::c_int
                                       } == 0 {
                                        current_block = 11510301253690344784;
                                    } else {
                                        (*tmv).delta =
                                            (*tmv).delta +
                                                (*yyvsp.offset(-(2 as
                                                                     libc::c_int)
                                                                   as
                                                                   isize)).val
                                                    * 3600 as libc::c_int;
                                        if if (*tmv).delta > 0 as libc::c_int
                                              {
                                               ((*yyvsp.offset(0 as
                                                                   libc::c_int
                                                                   as
                                                                   isize)).val
                                                    * 60 as libc::c_int <=
                                                    2147483647 as libc::c_int
                                                        - (*tmv).delta) as
                                                   libc::c_int
                                           } else {
                                               (-(2147483647 as libc::c_int) -
                                                    1 as libc::c_int -
                                                    (*tmv).delta <=
                                                    (*yyvsp.offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)).val
                                                        * 60 as libc::c_int)
                                                   as libc::c_int
                                           } == 0 {
                                            current_block =
                                                11510301253690344784;
                                        } else {
                                            (*tmv).delta =
                                                (*tmv).delta +
                                                    (*yyvsp.offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)).val
                                                        * 60 as libc::c_int;
                                            if if (*tmv).delta >
                                                      0 as libc::c_int {
                                                   (0 as libc::c_int <=
                                                        2147483647 as
                                                            libc::c_int -
                                                            (*tmv).delta) as
                                                       libc::c_int
                                               } else {
                                                   (-(2147483647 as
                                                          libc::c_int) -
                                                        1 as libc::c_int -
                                                        (*tmv).delta <=
                                                        0 as libc::c_int) as
                                                       libc::c_int
                                               } == 0 {
                                                current_block =
                                                    11510301253690344784;
                                            } else {
                                                (*tmv).delta =
                                                    (*tmv).delta +
                                                        0 as libc::c_int;
                                                /* yacc.c:1646  */
                                                current_block =
                                                    5127850565928544452;
                                            }
                                        }
                                    }
                                }
                            }
                            18 => {
                                if 0 as libc::c_int >
                                       2147483647 as libc::c_int /
                                           (24 as libc::c_int *
                                                3600 as libc::c_int) ||
                                       (0 as libc::c_int) <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               (24 as libc::c_int *
                                                    3600 as libc::c_int) ||
                                       0 as libc::c_int >
                                           2147483647 as libc::c_int /
                                               3600 as libc::c_int ||
                                       (0 as libc::c_int) <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               3600 as libc::c_int ||
                                       0 as libc::c_int >
                                           2147483647 as libc::c_int /
                                               60 as libc::c_int ||
                                       (0 as libc::c_int) <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               60 as libc::c_int {
                                    current_block = 11510301253690344784;
                                } else {
                                    (*tmv).delta =
                                        0 as libc::c_int *
                                            (24 as libc::c_int *
                                                 3600 as libc::c_int);
                                    if if (*tmv).delta > 0 as libc::c_int {
                                           (0 as libc::c_int *
                                                3600 as libc::c_int <=
                                                2147483647 as libc::c_int -
                                                    (*tmv).delta) as
                                               libc::c_int
                                       } else {
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int -
                                                (*tmv).delta <=
                                                0 as libc::c_int *
                                                    3600 as libc::c_int) as
                                               libc::c_int
                                       } == 0 {
                                        current_block = 11510301253690344784;
                                    } else {
                                        (*tmv).delta =
                                            (*tmv).delta +
                                                0 as libc::c_int *
                                                    3600 as libc::c_int;
                                        if if (*tmv).delta > 0 as libc::c_int
                                              {
                                               (0 as libc::c_int *
                                                    60 as libc::c_int <=
                                                    2147483647 as libc::c_int
                                                        - (*tmv).delta) as
                                                   libc::c_int
                                           } else {
                                               (-(2147483647 as libc::c_int) -
                                                    1 as libc::c_int -
                                                    (*tmv).delta <=
                                                    0 as libc::c_int *
                                                        60 as libc::c_int) as
                                                   libc::c_int
                                           } == 0 {
                                            current_block =
                                                11510301253690344784;
                                        } else {
                                            (*tmv).delta =
                                                (*tmv).delta +
                                                    0 as libc::c_int *
                                                        60 as libc::c_int;
                                            if if (*tmv).delta >
                                                      0 as libc::c_int {
                                                   ((*yyvsp.offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)).val
                                                        <=
                                                        2147483647 as
                                                            libc::c_int -
                                                            (*tmv).delta) as
                                                       libc::c_int
                                               } else {
                                                   (-(2147483647 as
                                                          libc::c_int) -
                                                        1 as libc::c_int -
                                                        (*tmv).delta <=
                                                        (*yyvsp.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)).val)
                                                       as libc::c_int
                                               } == 0 {
                                                current_block =
                                                    11510301253690344784;
                                            } else {
                                                (*tmv).delta =
                                                    (*tmv).delta +
                                                        (*yyvsp.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)).val;
                                                /* yacc.c:1646  */
                                                current_block =
                                                    5127850565928544452;
                                            }
                                        }
                                    }
                                }
                            }
                            20 => {
                                if (*yyvsp.offset(-(2 as libc::c_int) as
                                                      isize)).val >
                                       2147483647 as libc::c_int /
                                           3600 as libc::c_int ||
                                       (*yyvsp.offset(-(2 as libc::c_int) as
                                                          isize)).val <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               3600 as libc::c_int {
                                    current_block = 11510301253690344784;
                                } else if if (*yyvsp.offset(-(2 as
                                                                  libc::c_int)
                                                                as isize)).val
                                                 * 3600 as libc::c_int >
                                                 0 as libc::c_int {
                                              ((*yyvsp.offset(0 as libc::c_int
                                                                  as
                                                                  isize)).val
                                                   <=
                                                   2147483647 as libc::c_int -
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).val
                                                           *
                                                           3600 as
                                                               libc::c_int) as
                                                  libc::c_int
                                          } else {
                                              (-(2147483647 as libc::c_int) -
                                                   1 as libc::c_int -
                                                   (*yyvsp.offset(-(2 as
                                                                        libc::c_int)
                                                                      as
                                                                      isize)).val
                                                       * 3600 as libc::c_int
                                                   <=
                                                   (*yyvsp.offset(0 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)).val)
                                                  as libc::c_int
                                          } == 0 {
                                    current_block = 11510301253690344784;
                                } else {
                                    yyval.val =
                                        (*yyvsp.offset(-(2 as libc::c_int) as
                                                           isize)).val *
                                            3600 as libc::c_int +
                                            (*yyvsp.offset(0 as libc::c_int as
                                                               isize)).val;
                                    /* yacc.c:1646  */
                                    current_block = 5127850565928544452;
                                }
                            }
                            22 => {
                                if (*yyvsp.offset(-(2 as libc::c_int) as
                                                      isize)).val >
                                       2147483647 as libc::c_int /
                                           60 as libc::c_int ||
                                       (*yyvsp.offset(-(2 as libc::c_int) as
                                                          isize)).val <
                                           (-(2147483647 as libc::c_int) -
                                                1 as libc::c_int) /
                                               60 as libc::c_int {
                                    current_block = 11510301253690344784;
                                } else if if (*yyvsp.offset(-(2 as
                                                                  libc::c_int)
                                                                as isize)).val
                                                 * 60 as libc::c_int >
                                                 0 as libc::c_int {
                                              ((*yyvsp.offset(0 as libc::c_int
                                                                  as
                                                                  isize)).val
                                                   <=
                                                   2147483647 as libc::c_int -
                                                       (*yyvsp.offset(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          isize)).val
                                                           *
                                                           60 as libc::c_int)
                                                  as libc::c_int
                                          } else {
                                              (-(2147483647 as libc::c_int) -
                                                   1 as libc::c_int -
                                                   (*yyvsp.offset(-(2 as
                                                                        libc::c_int)
                                                                      as
                                                                      isize)).val
                                                       * 60 as libc::c_int <=
                                                   (*yyvsp.offset(0 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)).val)
                                                  as libc::c_int
                                          } == 0 {
                                    current_block = 11510301253690344784;
                                } else {
                                    yyval.val =
                                        (*yyvsp.offset(-(2 as libc::c_int) as
                                                           isize)).val *
                                            60 as libc::c_int +
                                            (*yyvsp.offset(0 as libc::c_int as
                                                               isize)).val;
                                    /* yacc.c:1646  */
                                    current_block = 5127850565928544452;
                                }
                            }
                            23 => {
                                yyval.val = 0 as libc::c_int;
                                /* yacc.c:1646  */
                                current_block = 5127850565928544452;
                            }
                            _ => { current_block = 5127850565928544452; }
                        }
                        match current_block {
                            11510301253690344784 =>
                            /* Pacify compilers like GCC when the user code never invokes
     YYERROR and the label yyerrorlab therefore never appears in user
     code.  */
                            /* Do not reclaim the symbols of the rule whose action triggered
     this YYERROR.  */
                            {
                                yyvsp = yyvsp.offset(-(yylen as isize));
                                yyssp = yyssp.offset(-(yylen as isize));
                                yylen = 0 as libc::c_int;
                                yystate = *yyssp as libc::c_int;
                                current_block = 17367163409548420750;
                            }
                            _ =>
                            /* User semantic actions sometimes alter yychar, and that requires
     that yytoken be updated with the new translation.  We take the
     approach of translating immediately before every use of yytoken.
     One alternative is translating here after every semantic action,
     but that translation would be missed if the semantic action invokes
     YYABORT, YYACCEPT, or YYERROR immediately after altering yychar or
     if it invokes YYBACKUP.  In the case of YYABORT or YYACCEPT, an
     incorrect destructor might then be invoked immediately.  In the
     case of YYERROR or YYBACKUP, subsequent parser actions might lead
     to an incorrect destructor call or verbose syntax error message
     before the lookahead is translated.  */
                            {
                                yyvsp = yyvsp.offset(-(yylen as isize));
                                yyssp = yyssp.offset(-(yylen as isize));
                                yylen = 0 as libc::c_int;
                                yyvsp = yyvsp.offset(1);
                                *yyvsp = yyval;
                                /* Now 'shift' the result of the reduction.  Determine what state
     that goes to, based on the state we popped back to and the rule
     number reduced by.  */
                                yyn = yyr1[yyn as usize] as libc::c_int;
                                yystate =
                                    yypgoto[(yyn - 13 as libc::c_int) as
                                                usize] as libc::c_int +
                                        *yyssp as libc::c_int;
                                if 0 as libc::c_int <= yystate &&
                                       yystate <= 37 as libc::c_int &&
                                       yycheck[yystate as usize] as
                                           libc::c_int ==
                                           *yyssp as libc::c_int {
                                    yystate =
                                        yytable[yystate as usize] as
                                            libc::c_int
                                } else {
                                    yystate =
                                        yydefgoto[(yyn - 13 as libc::c_int) as
                                                      usize] as libc::c_int
                                }
                                current_block = 1885938290812153035;
                            }
                        }
                    }
                    _ => { }
                }
                match current_block {
                    17367163409548420750 =>
                    /*-------------------------------------------------------------.
| yyerrlab1 -- common code for both syntax error and YYERROR.  |
`-------------------------------------------------------------*/
                    {
                        yyerrstatus =
                            3 as
                                libc::c_int; /* Each real token shifted decrements this.  */
                        loop  {
                            yyn = yypact[yystate as usize] as libc::c_int;
                            if !(yyn == -(16 as libc::c_int)) {
                                yyn += 1 as libc::c_int;
                                if 0 as libc::c_int <= yyn &&
                                       yyn <= 37 as libc::c_int &&
                                       yycheck[yyn as usize] as libc::c_int ==
                                           1 as libc::c_int {
                                    yyn =
                                        yytable[yyn as usize] as libc::c_int;
                                    if (0 as libc::c_int) < yyn { break ; }
                                }
                            }
                            /* Pop the current state because it cannot handle the error token.  */
                            if yyssp == yyss {
                                current_block = 9910440355210482038;
                                break 'c_6913 ;
                            }
                            yydestruct(b"Error: popping\x00" as *const u8 as
                                           *const libc::c_char,
                                       yystos[yystate as usize] as
                                           libc::c_int, yyvsp, tmv);
                            yyvsp =
                                yyvsp.offset(-(1 as libc::c_int as isize));
                            yyssp =
                                yyssp.offset(-(1 as libc::c_int as isize));
                            yystate = *yyssp as libc::c_int
                        }
                        yyvsp = yyvsp.offset(1);
                        *yyvsp = yylval;
                        /* Shift the error token.  */
                        yystate = yyn
                    }
                    _ => { }
                }
                /*------------------------------------------------------------.
| yynewstate -- Push a new state, which is found in yystate.  |
`------------------------------------------------------------*/
                /* In all cases, when you get here, the value and location stacks
     have just been pushed.  So pushing a state here evens the stacks.  */
                yyssp = yyssp.offset(1)
            }
        }
    match current_block {
        9910440355210482038 =>
        /*-----------------------------------.
| yyabortlab -- YYABORT comes here.  |
`-----------------------------------*/
        {
            yyresult = 1 as libc::c_int
        }
        3244376785761602731 =>
        /*-------------------------------------------------.
| yyexhaustedlab -- memory exhaustion comes here.  |
`-------------------------------------------------*/
        {
            yyresult = 2 as libc::c_int
        }
        _ => { }
    }
    /* Fall through.  */
    if yychar != -(2 as libc::c_int) {
        /* Make sure we have latest lookahead translation.  See comments at
         user semantic actions for why this is necessary.  */
        yytoken =
            if yychar as libc::c_uint <= 261 as libc::c_int as libc::c_uint {
                yytranslate[yychar as usize] as libc::c_int
            } else { 2 as libc::c_int };
        yydestruct(b"Cleanup: discarding lookahead\x00" as *const u8 as
                       *const libc::c_char, yytoken, &mut yylval, tmv);
    }
    /* Do not reclaim the symbols of the rule whose action triggered
     this YYABORT or YYACCEPT.  */
    yyvsp = yyvsp.offset(-(yylen as isize));
    yyssp = yyssp.offset(-(yylen as isize));
    while yyssp != yyss {
        yydestruct(b"Cleanup: popping\x00" as *const u8 as
                       *const libc::c_char,
                   yystos[*yyssp as usize] as libc::c_int, yyvsp, tmv);
        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
        yyssp = yyssp.offset(-(1 as libc::c_int as isize))
    }
    if yyss != yyssa.as_mut_ptr() { free(yyss as *mut libc::c_void); }
    return yyresult;
}
