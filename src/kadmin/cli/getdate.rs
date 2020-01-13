use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:49"]
pub mod types_h {
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/ctype.h:50"]
pub mod ctype_h {
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
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed = 8192;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed = 4096;
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed = 2048;
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
        #[no_mangle]
        #[c2rust::src_loc = "122:12"]
        pub fn tolower(_: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:53"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_tm.h:72"]
pub mod struct_tm_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "7:8"]
    pub struct tm {
        pub tm_sec: libc::c_int,
        pub tm_min: libc::c_int,
        pub tm_hour: libc::c_int,
        pub tm_mday: libc::c_int,
        pub tm_mon: libc::c_int,
        pub tm_year: libc::c_int,
        pub tm_wday: libc::c_int,
        pub tm_yday: libc::c_int,
        pub tm_isdst: libc::c_int,
        pub tm_gmtoff: libc::c_long,
        pub tm_zone: *const libc::c_char,
    }
}
#[c2rust::header_src = "/usr/include/string.h:17"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:53"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
    }
}
#[c2rust::header_src = "/usr/include/time.h:72"]
pub mod time_h {
    use super::time_t_h::time_t;
    use super::struct_tm_h::tm;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "75:1"]
        pub fn time(__timer: *mut time_t) -> time_t;
        /* Field not used		*/
        /* defined(vms) */
        /* Some old versions of bison generate parsers that use bcopy.
   That loses on systems that don't provide the function, so we have
   to redefine it here.  */
        #[no_mangle]
        #[c2rust::src_loc = "119:1"]
        pub fn gmtime(__timer: *const time_t) -> *mut tm;
        #[no_mangle]
        #[c2rust::src_loc = "123:1"]
        pub fn localtime(__timer: *const time_t) -> *mut tm;
    }
}
pub use self::types_h::__time_t;
pub use self::ctype_h::{C2RustUnnamed, _ISalnum, _ISpunct, _IScntrl, _ISblank,
                        _ISgraph, _ISprint, _ISspace, _ISxdigit, _ISdigit,
                        _ISalpha, _ISlower, _ISupper, __ctype_b_loc, tolower};
pub use self::time_t_h::time_t;
pub use self::struct_tm_h::tm;
use self::string_h::{strcmp, strncmp, strlen};
use self::stdlib_h::{malloc, free, abort};
use self::time_h::{time, gmtime, localtime};
/* A Bison parser, made by GNU Bison 3.5.  */
/*
**  Originally written by Steven M. Bellovin <smb@research.att.com> while
**  at the University of North Carolina at Chapel Hill.  Later tweaked by
**  a couple of people on Usenet.  Completely overhauled by Rich $alz
**  <rsalz@bbn.com> and Jim Berets <jberets@bbn.com> in August, 1990;
**  send any email to Rich.
**
**  This grammar has four shift/reduce conflicts.
**
**  This code is in the public domain and has no copyright.
*/
/* SUPPRESS 287 on yaccpar_sccsid */
/* Bison implementation for Yacc-like parsers in C

   Copyright (C) 1984, 1989-1990, 2000-2015, 2018-2019 Free Software Foundation,
   Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.  */
/* Unusd static variable */
/* SUPPRESS 288 on yyerrlab */
/* Label unused */
/* Since the code of getdate.y is not included in the Emacs executable
   itself, there is no need to #define static in this file.  Even if
   the code were included in the Emacs executable, it probably
   wouldn't do any harm to #undef it here; this will only cause
   problems if we try to write to a static variable, which I don't
   think this code needs to do.  */
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
/* The following block of alloca-related preprocessor directives is here
   solely to allow compilation by non GNU-C compilers of the C parser
   produced from this file by old versions of bison.  Newer versions of
   bison include a block similar to this one in bison.simple.  */
/* C LALR(1) parser skeleton written by Richard Stallman, by
   simplifying the original so-called "semantic" parser.  */
/* All symbols defined below should begin with yy or YY, to avoid
   infringing on user name space.  This should be done even for local
   variables, as they might otherwise be expanded by user macros.
   There are some unavoidable exceptions within include files to
   define necessary library symbols; they are noted "INFRINGES ON
   USER NAME SPACE" below.  */
/* Undocumented macros, especially those whose name start with YY_,
   are private implementation details.  Do not rely on them.  */
/* Identify Bison output.  */
/* Bison version.  */
/* Skeleton name.  */
/* The code at the top of get_date which figures out the offset of the
   current time zone checks various CPP symbols to see if special
   tricks are need, but defaults to using the gettimeofday system call.
   Include <sys/time.h> if that will be used.  */
/* Pure parsers.  */
/* Push parsers.  */
/* Pull parsers.  */
/* First part of user prologue.  */
/*
** We use the obsolete `struct my_timeb' as part of our interface!
** Since the system doesn't have it, we define it here;
** our callers must do likewise.
*/
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "90:8"]
pub struct my_timeb {
    pub time: time_t,
    pub millitm: libc::c_ushort,
    pub timezone: libc::c_short,
    pub dstflag: libc::c_short,
}
/*
**  An entry in the lexical lookup table.
*/
#[c2rust::src_loc = "129:1"]
pub type TABLE = _TABLE;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "129:16"]
pub struct _TABLE {
    pub name: *mut libc::c_char,
    pub type_0: libc::c_int,
    pub value: time_t,
}
/* Value type.  */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "326:7"]
pub union YYSTYPE {
    pub Number: time_t,
    pub Meridian: _MERIDIAN,
}
#[c2rust::src_loc = "146:9"]
pub type _MERIDIAN = libc::c_uint;
#[c2rust::src_loc = "147:19"]
pub const MER24: _MERIDIAN = 2;
#[c2rust::src_loc = "147:12"]
pub const MERpm: _MERIDIAN = 1;
#[c2rust::src_loc = "147:5"]
pub const MERam: _MERIDIAN = 0;
#[c2rust::src_loc = "139:9"]
pub type _DSTMODE = libc::c_uint;
#[c2rust::src_loc = "140:20"]
pub const DSTmaybe: _DSTMODE = 2;
#[c2rust::src_loc = "140:12"]
pub const DSToff: _DSTMODE = 1;
#[c2rust::src_loc = "140:5"]
pub const DSTon: _DSTMODE = 0;
/*
**  Daylight-savings mode:  on, off, or not yet known.
*/
#[c2rust::src_loc = "139:1"]
pub type DSTMODE = _DSTMODE;
/*
**  Meridian:  am, pm, or 24-hour style.
*/
#[c2rust::src_loc = "146:1"]
pub type MERIDIAN = _MERIDIAN;
/* Stored state numbers (used for stacks). */
#[c2rust::src_loc = "447:1"]
pub type yy_state_t = yytype_int8;
/* On compilers that do not define __PTRDIFF_MAX__ etc., make sure
   <limits.h> and (if available) <stdint.h> are included
   so that the code can choose integer types of a good width.  */
/* Narrow types that promote to a signed type and that can represent a
   signed or unsigned integer of at least N bits.  In tables they can
   save space and decrease cache pressure.  Promoting to a signed type
   helps avoid bugs in integer arithmetic.  */
#[c2rust::src_loc = "372:1"]
pub type yytype_int8 = libc::c_schar;
/* The timezone table. */
/* Some of these are commented out because a time_t can't store a float. */
/* State numbers in computations.  */
#[c2rust::src_loc = "450:1"]
pub type yy_state_fast_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "594:7"]
pub union yyalloc {
    pub yyss_alloc: yy_state_t,
    pub yyvs_alloc: YYSTYPE,
}
/*
**  Global variables.  We could get rid of most of these by using a good
**  union as the yacc stack.  (This routine was originally written before
**  yacc had the %union construct.)  Maybe someday; right now we only use
**  the %union very rarely.
*/
#[c2rust::src_loc = "157:14"]
static mut yyInput: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "158:16"]
static mut yyDSTmode: DSTMODE = DSTon;
#[c2rust::src_loc = "159:15"]
static mut yyDayOrdinal: time_t = 0;
#[c2rust::src_loc = "160:15"]
static mut yyDayNumber: time_t = 0;
#[c2rust::src_loc = "161:12"]
static mut yyHaveDate: libc::c_int = 0;
#[c2rust::src_loc = "162:12"]
static mut yyHaveDay: libc::c_int = 0;
#[c2rust::src_loc = "163:12"]
static mut yyHaveRel: libc::c_int = 0;
#[c2rust::src_loc = "164:12"]
static mut yyHaveTime: libc::c_int = 0;
#[c2rust::src_loc = "165:12"]
static mut yyHaveZone: libc::c_int = 0;
#[c2rust::src_loc = "166:15"]
static mut yyTimezone: time_t = 0;
#[c2rust::src_loc = "167:15"]
static mut yyDay: time_t = 0;
#[c2rust::src_loc = "168:15"]
static mut yyHour: time_t = 0;
#[c2rust::src_loc = "169:15"]
static mut yyMinutes: time_t = 0;
#[c2rust::src_loc = "170:15"]
static mut yyMonth: time_t = 0;
#[c2rust::src_loc = "171:15"]
static mut yySeconds: time_t = 0;
#[c2rust::src_loc = "172:15"]
static mut yyYear: time_t = 0;
#[c2rust::src_loc = "173:17"]
static mut yyMeridian: MERIDIAN = MERam;
#[c2rust::src_loc = "174:15"]
static mut yyRelMonth: time_t = 0;
#[c2rust::src_loc = "175:15"]
static mut yyRelSeconds: time_t = 0;
/* Month and day table. */
#[c2rust::src_loc = "379:20"]
static mut MonthDayTable: [TABLE; 25] =
    [{
         let mut init =
             _TABLE{name:
                        b"january\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 264 as libc::c_int,
                    value: 1 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"february\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 264 as libc::c_int,
                    value: 2 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"march\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 264 as libc::c_int,
                    value: 3 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"april\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 264 as libc::c_int,
                    value: 4 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"may\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 264 as libc::c_int,
                    value: 5 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"june\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 264 as libc::c_int,
                    value: 6 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"july\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 264 as libc::c_int,
                    value: 7 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"august\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 264 as libc::c_int,
                    value: 8 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"september\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    type_0: 264 as libc::c_int,
                    value: 9 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"sept\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 264 as libc::c_int,
                    value: 9 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"october\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 264 as libc::c_int,
                    value: 10 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"november\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 264 as libc::c_int,
                    value: 11 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"december\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 264 as libc::c_int,
                    value: 12 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"sunday\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 259 as libc::c_int,
                    value: 0 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"monday\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 259 as libc::c_int,
                    value: 1 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"tuesday\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 259 as libc::c_int,
                    value: 2 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"tues\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 259 as libc::c_int,
                    value: 2 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"wednesday\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    type_0: 259 as libc::c_int,
                    value: 3 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"wednes\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 259 as libc::c_int,
                    value: 3 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"thursday\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 259 as libc::c_int,
                    value: 4 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"thur\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 259 as libc::c_int,
                    value: 4 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"thurs\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 259 as libc::c_int,
                    value: 4 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"friday\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 259 as libc::c_int,
                    value: 5 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"saturday\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 259 as libc::c_int,
                    value: 6 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name: 0 as *const libc::c_char as *mut libc::c_char,
                    type_0: 0,
                    value: 0,};
         init
     }];
/* Time units table. */
#[c2rust::src_loc = "408:20"]
static mut UnitsTable: [TABLE; 11] =
    [{
         let mut init =
             _TABLE{name:
                        b"year\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 265 as libc::c_int,
                    value: 12 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"month\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 265 as libc::c_int,
                    value: 1 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"fortnight\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    type_0: 263 as libc::c_int,
                    value:
                        (14 as libc::c_int * 24 as libc::c_int *
                             60 as libc::c_int) as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"week\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 263 as libc::c_int,
                    value:
                        (7 as libc::c_int * 24 as libc::c_int *
                             60 as libc::c_int) as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"day\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 263 as libc::c_int,
                    value:
                        (1 as libc::c_int * 24 as libc::c_int *
                             60 as libc::c_int) as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"hour\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 263 as libc::c_int,
                    value: 60 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"minute\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 263 as libc::c_int,
                    value: 1 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"min\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 263 as libc::c_int,
                    value: 1 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"second\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 266 as libc::c_int,
                    value: 1 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"sec\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 266 as libc::c_int,
                    value: 1 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name: 0 as *const libc::c_char as *mut libc::c_char,
                    type_0: 0,
                    value: 0,};
         init
     }];
/* Assorted relative-time words. */
#[c2rust::src_loc = "423:20"]
static mut OtherTable: [TABLE; 21] =
    [{
         let mut init =
             _TABLE{name:
                        b"tomorrow\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 263 as libc::c_int,
                    value:
                        (1 as libc::c_int * 24 as libc::c_int *
                             60 as libc::c_int) as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"yesterday\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    type_0: 263 as libc::c_int,
                    value:
                        (-(1 as libc::c_int) * 24 as libc::c_int *
                             60 as libc::c_int) as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"today\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 263 as libc::c_int,
                    value: 0 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"now\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 263 as libc::c_int,
                    value: 0 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"last\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 268 as libc::c_int,
                    value: -(1 as libc::c_int) as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"this\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 263 as libc::c_int,
                    value: 0 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"next\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 268 as libc::c_int,
                    value: 2 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"first\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 268 as libc::c_int,
                    value: 1 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"third\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 268 as libc::c_int,
                    value: 3 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"fourth\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 268 as libc::c_int,
                    value: 4 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"fifth\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 268 as libc::c_int,
                    value: 5 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"sixth\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 268 as libc::c_int,
                    value: 6 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"seventh\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 268 as libc::c_int,
                    value: 7 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"eighth\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 268 as libc::c_int,
                    value: 8 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"ninth\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 268 as libc::c_int,
                    value: 9 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"tenth\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 268 as libc::c_int,
                    value: 10 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"eleventh\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 268 as libc::c_int,
                    value: 11 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"twelfth\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 268 as libc::c_int,
                    value: 12 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"ago\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 258 as libc::c_int,
                    value: 1 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"never\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 271 as libc::c_int,
                    value: 0 as libc::c_int as time_t,};
         init
     },
     {
         let mut init =
             _TABLE{name: 0 as *const libc::c_char as *mut libc::c_char,
                    type_0: 0,
                    value: 0,};
         init
     }];
#[c2rust::src_loc = "450:20"]
static mut TimezoneTable: [TABLE; 52] =
    [{
         let mut init =
             _TABLE{name:
                        b"gmt\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        0 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"ut\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        0 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"utc\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        0 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"wet\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        0 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"bst\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 260 as libc::c_int,
                    value:
                        0 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"wat\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        1 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"at\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        2 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"ast\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        4 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"adt\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 260 as libc::c_int,
                    value:
                        4 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"est\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        5 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"edt\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 260 as libc::c_int,
                    value:
                        5 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"cst\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        6 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"cdt\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 260 as libc::c_int,
                    value:
                        6 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"mst\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        7 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"mdt\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 260 as libc::c_int,
                    value:
                        7 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"pst\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        8 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"pdt\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 260 as libc::c_int,
                    value:
                        8 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"yst\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        9 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"ydt\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 260 as libc::c_int,
                    value:
                        9 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"hst\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        10 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"hdt\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 260 as libc::c_int,
                    value:
                        10 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"cat\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        10 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"ahst\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        10 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"nt\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        11 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"idlw\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        12 as libc::c_int as time_t *
                            60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"cet\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        -(1 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"met\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        -(1 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"mewt\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        -(1 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"mest\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 260 as libc::c_int,
                    value:
                        -(1 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"swt\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        -(1 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"sst\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 260 as libc::c_int,
                    value:
                        -(1 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"fwt\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        -(1 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"fst\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 260 as libc::c_int,
                    value:
                        -(1 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"eet\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        -(2 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"bt\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        -(3 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"zp4\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        -(4 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"zp5\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        -(5 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"zp6\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        -(6 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"wast\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        -(7 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"wadt\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 260 as libc::c_int,
                    value:
                        -(7 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"cct\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        -(8 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"jst\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        -(9 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"kst\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        -(9 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"east\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        -(10 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"eadt\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 260 as libc::c_int,
                    value:
                        -(10 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"gst\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        -(10 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"kdt\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        -(10 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"nzt\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        -(12 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"nzst\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        -(12 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"nzdt\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 260 as libc::c_int,
                    value:
                        -(12 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name:
                        b"idle\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    type_0: 269 as libc::c_int,
                    value:
                        -(12 as libc::c_int as time_t *
                              60 as libc::c_int as libc::c_long),};
         init
     },
     {
         let mut init =
             _TABLE{name: 0 as *const libc::c_char as *mut libc::c_char,
                    type_0: 0,
                    value: 0,};
         init
     }];
/* ARGSUSED */
#[c2rust::src_loc = "536:1"]
unsafe extern "C" fn getdate_yyerror(mut s: *mut libc::c_char)
 -> libc::c_int {
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "543:1"]
unsafe extern "C" fn ToSeconds(mut Hours: time_t, mut Minutes: time_t,
                               mut Seconds: time_t, mut Meridian: MERIDIAN)
 -> time_t {
    if Minutes < 0 as libc::c_int as libc::c_long ||
           Minutes > 59 as libc::c_int as libc::c_long ||
           Seconds < 0 as libc::c_int as libc::c_long ||
           Seconds > 59 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int) as time_t
    }
    match Meridian as libc::c_uint {
        2 => {
            if Hours < 0 as libc::c_int as libc::c_long ||
                   Hours > 23 as libc::c_int as libc::c_long {
                return -(1 as libc::c_int) as time_t
            }
            return (Hours * 60 as libc::c_long + Minutes) * 60 as libc::c_long
                       + Seconds
        }
        0 => {
            if Hours < 1 as libc::c_int as libc::c_long ||
                   Hours > 12 as libc::c_int as libc::c_long {
                return -(1 as libc::c_int) as time_t
            }
            return (Hours * 60 as libc::c_long + Minutes) * 60 as libc::c_long
                       + Seconds
        }
        1 => {
            if Hours < 1 as libc::c_int as libc::c_long ||
                   Hours > 12 as libc::c_int as libc::c_long {
                return -(1 as libc::c_int) as time_t
            }
            return ((Hours + 12 as libc::c_int as libc::c_long) *
                        60 as libc::c_long + Minutes) * 60 as libc::c_long +
                       Seconds
        }
        _ => { abort(); }
    };
    /* NOTREACHED */
}
/*
 * From hh:mm:ss [am|pm] mm/dd/yy [tz], compute and return the number
 * of seconds since 00:00:00 1/1/70 GMT.
 */
#[c2rust::src_loc = "571:1"]
unsafe extern "C" fn Convert(mut Month: time_t, mut Day: time_t,
                             mut Year: time_t, mut Hours: time_t,
                             mut Minutes: time_t, mut Seconds: time_t,
                             mut Meridian: MERIDIAN, mut DSTmode: DSTMODE)
 -> time_t {
    static mut DaysInMonth: [libc::c_int; 12] =
        [31 as libc::c_int, 0 as libc::c_int, 31 as libc::c_int,
         30 as libc::c_int, 31 as libc::c_int, 30 as libc::c_int,
         31 as libc::c_int, 31 as libc::c_int, 30 as libc::c_int,
         31 as libc::c_int, 30 as libc::c_int, 31 as libc::c_int];
    let mut tod: time_t = 0;
    let mut Julian: time_t = 0;
    let mut i: libc::c_int = 0;
    let mut tm: *mut tm = 0 as *mut tm;
    if Year < 0 as libc::c_int as libc::c_long { Year = -Year }
    if Year < 1900 as libc::c_int as libc::c_long {
        Year += 1900 as libc::c_int as libc::c_long
    }
    DaysInMonth[1 as libc::c_int as usize] =
        if Year % 4 as libc::c_int as libc::c_long ==
               0 as libc::c_int as libc::c_long &&
               (Year % 100 as libc::c_int as libc::c_long !=
                    0 as libc::c_int as libc::c_long ||
                    Year % 400 as libc::c_int as libc::c_long ==
                        0 as libc::c_int as libc::c_long) {
            29 as libc::c_int
        } else { 28 as libc::c_int };
    if Year < 1970 as libc::c_int as libc::c_long ||
           Year > 2106 as libc::c_int as libc::c_long ||
           Month < 1 as libc::c_int as libc::c_long ||
           Month > 12 as libc::c_int as libc::c_long ||
           Day < 1 as libc::c_int as libc::c_long ||
           {
               Month -= 1;
               (Day) >
                   DaysInMonth[Month as libc::c_int as usize] as libc::c_long
           } {
        return -(1 as libc::c_int) as time_t
    }
    Julian = Day - 1 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while (i as libc::c_long) < Month {
        Julian += DaysInMonth[i as usize] as libc::c_long;
        i += 1
    }
    i = 1970 as libc::c_int;
    while (i as libc::c_long) < Year {
        Julian +=
            (365 as libc::c_int +
                 (i % 4 as libc::c_int == 0 as libc::c_int &&
                      (Year % 100 as libc::c_int as libc::c_long !=
                           0 as libc::c_int as libc::c_long ||
                           Year % 400 as libc::c_int as libc::c_long ==
                               0 as libc::c_int as libc::c_long)) as
                     libc::c_int) as libc::c_long;
        i += 1
    }
    Julian *= 24 as libc::c_long * 60 as libc::c_long * 60 as libc::c_long;
    Julian += yyTimezone * 60 as libc::c_long;
    /* The size of an array large to enough to hold all stacks, each with
   N elements.  */
    tod = ToSeconds(Hours, Minutes, Seconds, Meridian);
    if tod < 0 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int) as time_t
    }
    Julian += tod;
    if DSTmode as libc::c_uint == DSTon as libc::c_int as libc::c_uint {
        Julian -= (60 as libc::c_int * 60 as libc::c_int) as libc::c_long
    } else if DSTmode as libc::c_uint ==
                  DSTmaybe as libc::c_int as libc::c_uint {
        tm = localtime(&mut Julian);
        if tm.is_null() {
            /* Relocate STACK from its old location to the new one.  The
   local variables YYSIZE and YYSTACKSIZE give the old and new number of
   elements in the stack, and YYPTR gives the new location of the
   stack.  Advance YYPTR to a properly aligned location for the next
   stack.  */
            return -(1 as libc::c_int) as time_t
        } else {
            if (*tm).tm_isdst != 0 {
                Julian -=
                    (60 as libc::c_int * 60 as libc::c_int) as libc::c_long
            }
        }
    }
    return Julian;
}
#[c2rust::src_loc = "619:1"]
unsafe extern "C" fn DSTcorrect(mut Start: time_t, mut Future: time_t,
                                mut error: *mut libc::c_int) -> time_t {
    let mut StartDay: time_t = 0;
    let mut FutureDay: time_t = 0;
    let mut tm: *mut tm = 0 as *mut tm;
    tm = localtime(&mut Start);
    if tm.is_null() {
        *error = 1 as libc::c_int;
        return -(1 as libc::c_int) as time_t
        /* Copy COUNT objects from SRC to DST.  The source and destination do
   not overlap.  */
    }
    StartDay =
        (((*tm).tm_hour + 1 as libc::c_int) % 24 as libc::c_int) as time_t;
    tm = localtime(&mut Future);
    if tm.is_null() {
        *error = 1 as libc::c_int;
        return -(1 as libc::c_int) as time_t
    }
    FutureDay =
        (((*tm).tm_hour + 1 as libc::c_int) % 24 as libc::c_int) as time_t;
    *error = 0 as libc::c_int;
    return Future - Start +
               (StartDay - FutureDay) * 60 as libc::c_long *
                   60 as libc::c_long;
}
#[c2rust::src_loc = "643:1"]
unsafe extern "C" fn RelativeDate(mut Start: time_t, mut DayOrdinal: time_t,
                                  mut DayNumber: time_t,
                                  mut error: *mut libc::c_int) -> time_t {
    let mut tm: *mut tm = 0 as *mut tm;
    let mut now: time_t = 0;
    /* YYFINAL -- State number of the termination state.  */
    now = Start;
    tm = localtime(&mut now);
    /* YYLAST -- Last index in YYTABLE.  */
    if tm.is_null() {
        *error = 1 as libc::c_int;
        return -(1 as libc::c_int) as time_t
        /* YYNTOKENS -- Number of terminals.  */
    }
    now +=
        24 as libc::c_long * 60 as libc::c_long * 60 as libc::c_long *
            ((DayNumber - (*tm).tm_wday as libc::c_long +
                  7 as libc::c_int as libc::c_long) %
                 7 as libc::c_int as libc::c_long);
    /* YYNNTS -- Number of nonterminals.  */
    now +=
        7 as libc::c_int as libc::c_long *
            (24 as libc::c_long * 60 as libc::c_long * 60 as libc::c_long) *
            (if DayOrdinal <= 0 as libc::c_int as libc::c_long {
                 DayOrdinal
             } else { (DayOrdinal) - 1 as libc::c_int as libc::c_long });
    return DSTcorrect(Start, now, error);
}
/* YYNRULES -- Number of rules.  */
/* YYNSTATES -- Number of states.  */
#[c2rust::src_loc = "661:1"]
unsafe extern "C" fn RelativeMonth(mut Start: time_t, mut RelMonth: time_t)
 -> time_t {
    let mut tm: *mut tm = 0 as *mut tm;
    let mut Month: time_t = 0;
    let mut Year: time_t = 0;
    /* YYTRANSLATE(TOKEN-NUM) -- Symbol number corresponding to TOKEN-NUM
   as returned by yylex, with out-of-bounds checking.  */
    let mut ret: time_t = 0;
    let mut error: libc::c_int = 0;
    if RelMonth == 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as time_t
    }
    /* YYTRANSLATE[TOKEN-NUM] -- Symbol number corresponding to TOKEN-NUM
   as returned by yylex.  */
    tm = localtime(&mut Start);
    if tm.is_null() { return -(1 as libc::c_int) as time_t }
    Month =
        (12 as libc::c_int * (*tm).tm_year + (*tm).tm_mon) as libc::c_long +
            RelMonth;
    Year = Month / 12 as libc::c_int as libc::c_long;
    Month =
        Month % 12 as libc::c_int as libc::c_long +
            1 as libc::c_int as libc::c_long;
    ret =
        Convert(Month, (*tm).tm_mday as time_t, Year, (*tm).tm_hour as time_t,
                (*tm).tm_min as time_t, (*tm).tm_sec as time_t, MER24,
                DSTmaybe);
    if ret == -(1 as libc::c_int) as libc::c_long { return ret }
    ret = DSTcorrect(Start, ret, &mut error);
    if error != 0 { return -(1 as libc::c_int) as time_t }
    return ret;
}
#[c2rust::src_loc = "674:26"]
static mut yytranslate: [yytype_int8; 272] =
    [0 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     18 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 19 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     17 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     3 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     5 as libc::c_int as yytype_int8, 6 as libc::c_int as yytype_int8,
     7 as libc::c_int as yytype_int8, 8 as libc::c_int as yytype_int8,
     9 as libc::c_int as yytype_int8, 10 as libc::c_int as yytype_int8,
     11 as libc::c_int as yytype_int8, 12 as libc::c_int as yytype_int8,
     13 as libc::c_int as yytype_int8, 14 as libc::c_int as yytype_int8,
     15 as libc::c_int as yytype_int8, 16 as libc::c_int as yytype_int8];
#[c2rust::src_loc = "690:1"]
unsafe extern "C" fn LookupWord(mut buff: *mut libc::c_char) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tp: *const TABLE = 0 as *const TABLE;
    let mut i: libc::c_int = 0;
    let mut abbrev: libc::c_int = 0;
    p = buff;
    while *p != 0 {
        if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as
               libc::c_int &
               _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0 {
            *p = tolower(*p as libc::c_int) as libc::c_char
        }
        p = p.offset(1)
    }
    if strcmp(buff, b"am\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int ||
           strcmp(buff, b"a.m.\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        yylval.Meridian = MERam;
        return 262 as libc::c_int
    }
    if strcmp(buff, b"pm\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int ||
           strcmp(buff, b"p.m.\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        yylval.Meridian = MERpm;
        return 262 as libc::c_int
    }
    /* See if we have an abbreviation for a month. */
    if strlen(buff) == 3 as libc::c_int as libc::c_ulong {
        abbrev = 1 as libc::c_int
    } else if strlen(buff) == 4 as libc::c_int as libc::c_ulong &&
                  *buff.offset(3 as libc::c_int as isize) as libc::c_int ==
                      '.' as i32 {
        abbrev = 1 as libc::c_int;
        *buff.offset(3 as libc::c_int as isize) =
            '\u{0}' as i32 as libc::c_char
    } else { abbrev = 0 as libc::c_int }
    tp = MonthDayTable.as_ptr();
    while !(*tp).name.is_null() {
        if abbrev != 0 {
            if strncmp(buff, (*tp).name, 3 as libc::c_int as libc::c_ulong) ==
                   0 as libc::c_int {
                yylval.Number = (*tp).value;
                return (*tp).type_0
            }
        } else if strcmp(buff, (*tp).name) == 0 as libc::c_int {
            yylval.Number = (*tp).value;
            return (*tp).type_0
        }
        tp = tp.offset(1)
    }
    tp = TimezoneTable.as_ptr();
    while !(*tp).name.is_null() {
        if strcmp(buff, (*tp).name) == 0 as libc::c_int {
            yylval.Number = (*tp).value;
            return (*tp).type_0
        }
        tp = tp.offset(1)
    }
    if strcmp(buff, b"dst\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return 270 as libc::c_int
    }
    tp = UnitsTable.as_ptr();
    while !(*tp).name.is_null() {
        if strcmp(buff, (*tp).name) == 0 as libc::c_int {
            yylval.Number = (*tp).value;
            return (*tp).type_0
        }
        tp = tp.offset(1)
    }
    /* YYPACT[STATE-NUM] -- Index in YYTABLE of the portion describing
     STATE-NUM.  */
    /* Strip off any plural and try the units table again. */
    i =
        strlen(buff).wrapping_sub(1 as libc::c_int as libc::c_ulong) as
            libc::c_int;
    if *buff.offset(i as isize) as libc::c_int == 's' as i32 {
        *buff.offset(i as isize) = '\u{0}' as i32 as libc::c_char;
        tp = UnitsTable.as_ptr();
        while !(*tp).name.is_null() {
            if strcmp(buff, (*tp).name) == 0 as libc::c_int {
                yylval.Number = (*tp).value;
                return (*tp).type_0
            }
            tp = tp.offset(1)
        }
        *buff.offset(i as isize) = 's' as i32 as libc::c_char
        /* Put back for "this" in OtherTable. */
    }
    /* YYDEFACT[STATE-NUM] -- Default reduction number in state STATE-NUM.
     Performed when YYTABLE does not specify something else to do.  Zero
     means the default is an error.  */
    tp = OtherTable.as_ptr();
    while !(*tp).name.is_null() {
        if strcmp(buff, (*tp).name) == 0 as libc::c_int {
            yylval.Number = (*tp).value;
            return (*tp).type_0
        }
        tp = tp.offset(1)
    }
    /* Drop out any periods and try the timezone table again. */
    i = 0 as libc::c_int;
    q = buff;
    p = q;
    while *q != 0 {
        if *q as libc::c_int != '.' as i32 {
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = *q
        } else { i += 1 }
        q = q.offset(1)
    }
    *p = '\u{0}' as i32 as libc::c_char;
    /* YYPGOTO[NTERM-NUM].  */
    if i != 0 {
        tp = TimezoneTable.as_ptr();
        while !(*tp).name.is_null() {
            if strcmp(buff, (*tp).name) == 0 as libc::c_int {
                yylval.Number = (*tp).value;
                return (*tp).type_0
            }
            tp = tp.offset(1)
        }
    }
    /* YYDEFGOTO[NTERM-NUM].  */
    return 261 as libc::c_int;
}
#[c2rust::src_loc = "753:26"]
static mut yypact: [yytype_int8; 52] =
    [-(6 as libc::c_int) as yytype_int8, -(13 as libc::c_int) as yytype_int8,
     12 as libc::c_int as yytype_int8, -(13 as libc::c_int) as yytype_int8,
     -(7 as libc::c_int) as yytype_int8, -(13 as libc::c_int) as yytype_int8,
     -(13 as libc::c_int) as yytype_int8, 5 as libc::c_int as yytype_int8,
     -(13 as libc::c_int) as yytype_int8, -(13 as libc::c_int) as yytype_int8,
     23 as libc::c_int as yytype_int8, -(4 as libc::c_int) as yytype_int8,
     13 as libc::c_int as yytype_int8, -(13 as libc::c_int) as yytype_int8,
     -(13 as libc::c_int) as yytype_int8, -(13 as libc::c_int) as yytype_int8,
     -(13 as libc::c_int) as yytype_int8, -(13 as libc::c_int) as yytype_int8,
     -(13 as libc::c_int) as yytype_int8, 26 as libc::c_int as yytype_int8,
     -(13 as libc::c_int) as yytype_int8, 17 as libc::c_int as yytype_int8,
     -(13 as libc::c_int) as yytype_int8, -(13 as libc::c_int) as yytype_int8,
     -(13 as libc::c_int) as yytype_int8, -(13 as libc::c_int) as yytype_int8,
     -(13 as libc::c_int) as yytype_int8, -(13 as libc::c_int) as yytype_int8,
     -(11 as libc::c_int) as yytype_int8, -(13 as libc::c_int) as yytype_int8,
     -(13 as libc::c_int) as yytype_int8, 18 as libc::c_int as yytype_int8,
     24 as libc::c_int as yytype_int8, 25 as libc::c_int as yytype_int8,
     -(13 as libc::c_int) as yytype_int8, -(13 as libc::c_int) as yytype_int8,
     27 as libc::c_int as yytype_int8, -(13 as libc::c_int) as yytype_int8,
     -(13 as libc::c_int) as yytype_int8, -(13 as libc::c_int) as yytype_int8,
     2 as libc::c_int as yytype_int8, 22 as libc::c_int as yytype_int8,
     -(13 as libc::c_int) as yytype_int8, -(13 as libc::c_int) as yytype_int8,
     -(13 as libc::c_int) as yytype_int8, 29 as libc::c_int as yytype_int8,
     -(13 as libc::c_int) as yytype_int8, 30 as libc::c_int as yytype_int8,
     20 as libc::c_int as yytype_int8, -(13 as libc::c_int) as yytype_int8,
     -(13 as libc::c_int) as yytype_int8,
     -(13 as libc::c_int) as yytype_int8];
#[c2rust::src_loc = "766:26"]
static mut yydefact: [yytype_int8; 52] =
    [2 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 1 as libc::c_int as yytype_int8,
     18 as libc::c_int as yytype_int8, 16 as libc::c_int as yytype_int8,
     33 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     39 as libc::c_int as yytype_int8, 36 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     15 as libc::c_int as yytype_int8, 3 as libc::c_int as yytype_int8,
     5 as libc::c_int as yytype_int8, 6 as libc::c_int as yytype_int8,
     8 as libc::c_int as yytype_int8, 7 as libc::c_int as yytype_int8,
     9 as libc::c_int as yytype_int8, 30 as libc::c_int as yytype_int8,
     19 as libc::c_int as yytype_int8, 25 as libc::c_int as yytype_int8,
     32 as libc::c_int as yytype_int8, 37 as libc::c_int as yytype_int8,
     34 as libc::c_int as yytype_int8, 20 as libc::c_int as yytype_int8,
     10 as libc::c_int as yytype_int8, 31 as libc::c_int as yytype_int8,
     27 as libc::c_int as yytype_int8, 38 as libc::c_int as yytype_int8,
     35 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     17 as libc::c_int as yytype_int8, 29 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 24 as libc::c_int as yytype_int8,
     28 as libc::c_int as yytype_int8, 23 as libc::c_int as yytype_int8,
     40 as libc::c_int as yytype_int8, 21 as libc::c_int as yytype_int8,
     26 as libc::c_int as yytype_int8, 41 as libc::c_int as yytype_int8,
     12 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     11 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     40 as libc::c_int as yytype_int8, 22 as libc::c_int as yytype_int8,
     14 as libc::c_int as yytype_int8, 13 as libc::c_int as yytype_int8];
#[c2rust::src_loc = "777:26"]
static mut yypgoto: [yytype_int8; 10] =
    [-(13 as libc::c_int) as yytype_int8, -(13 as libc::c_int) as yytype_int8,
     -(13 as libc::c_int) as yytype_int8, -(13 as libc::c_int) as yytype_int8,
     -(13 as libc::c_int) as yytype_int8, -(13 as libc::c_int) as yytype_int8,
     -(13 as libc::c_int) as yytype_int8, -(13 as libc::c_int) as yytype_int8,
     -(13 as libc::c_int) as yytype_int8,
     -(12 as libc::c_int) as yytype_int8];
#[c2rust::src_loc = "783:26"]
static mut yydefgoto: [yytype_int8; 10] =
    [-(1 as libc::c_int) as yytype_int8, 2 as libc::c_int as yytype_int8,
     13 as libc::c_int as yytype_int8, 14 as libc::c_int as yytype_int8,
     15 as libc::c_int as yytype_int8, 16 as libc::c_int as yytype_int8,
     17 as libc::c_int as yytype_int8, 18 as libc::c_int as yytype_int8,
     19 as libc::c_int as yytype_int8, 46 as libc::c_int as yytype_int8];
#[c2rust::src_loc = "787:1"]
unsafe extern "C" fn getdate_yylex() -> libc::c_int 
 /* YYTABLE[YYPACT[STATE-NUM]] -- What to do in state STATE-NUM.  If
     positive, shift that token.  If negative, reduce the rule whose
     number is the opposite.  If YYTABLE_NINF, syntax error.  */
 {
    let mut c: libc::c_char = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buff: [libc::c_char; 20] = [0; 20];
    let mut Count: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    loop 
         /* skip the '-' sign */
         {
        while *(*__ctype_b_loc()).offset(*yyInput as libc::c_int as isize) as
                  libc::c_int &
                  _ISspace as libc::c_int as libc::c_ushort as libc::c_int !=
                  0 {
            yyInput = yyInput.offset(1)
        }
        c = *yyInput;
        if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as
               libc::c_int &
               _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
               || c as libc::c_int == '-' as i32 ||
               c as libc::c_int == '+' as i32 {
            if c as libc::c_int == '-' as i32 ||
                   c as libc::c_int == '+' as i32 {
                sign =
                    if c as libc::c_int == '-' as i32 {
                        -(1 as libc::c_int)
                    } else { 1 as libc::c_int };
                yyInput = yyInput.offset(1);
                if *(*__ctype_b_loc()).offset(*yyInput as libc::c_int as
                                                  isize) as libc::c_int &
                       _ISdigit as libc::c_int as libc::c_ushort as
                           libc::c_int == 0 {
                    continue ;
                }
            } else { sign = 0 as libc::c_int }
            yylval.Number = 0 as libc::c_int as time_t;
            loop  {
                let fresh1 = yyInput;
                yyInput = yyInput.offset(1);
                c = *fresh1;
                if !(*(*__ctype_b_loc()).offset(c as libc::c_int as isize) as
                         libc::c_int &
                         _ISdigit as libc::c_int as libc::c_ushort as
                             libc::c_int != 0) {
                    break ;
                }
                yylval.Number =
                    10 as libc::c_int as libc::c_long * yylval.Number +
                        c as libc::c_long - '0' as i32 as libc::c_long
            }
            yyInput = yyInput.offset(-1);
            if sign < 0 as libc::c_int { yylval.Number = -yylval.Number }
            return if sign != 0 {
                       267 as libc::c_int
                   } else { 268 as libc::c_int }
        } else {
            if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as
                   libc::c_int &
                   _ISalpha as libc::c_int as libc::c_ushort as libc::c_int !=
                   0 {
                p = buff.as_mut_ptr();
                loop  {
                    let fresh2 = yyInput;
                    yyInput = yyInput.offset(1);
                    c = *fresh2;
                    if !(*(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                             as libc::c_int &
                             _ISalpha as libc::c_int as libc::c_ushort as
                                 libc::c_int != 0 ||
                             c as libc::c_int == '.' as i32) {
                        break ;
                    }
                    if p <
                           &mut *buff.as_mut_ptr().offset((::std::mem::size_of::<[libc::c_char; 20]>()
                                                               as
                                                               libc::c_ulong).wrapping_sub(1
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_ulong)
                                                              as isize) as
                               *mut libc::c_char {
                        let fresh3 = p;
                        p = p.offset(1);
                        *fresh3 = c
                    }
                }
                /* YYR1[YYN] -- Symbol number of symbol that rule YYN derives.  */
                *p = '\u{0}' as i32 as libc::c_char;
                yyInput = yyInput.offset(-1);
                return LookupWord(buff.as_mut_ptr())
            }
            if c as libc::c_int != '(' as i32 {
                let fresh4 = yyInput;
                yyInput = yyInput.offset(1);
                return *fresh4 as libc::c_int
            }
            Count = 0 as libc::c_int;
            loop  {
                let fresh5 = yyInput;
                yyInput = yyInput.offset(1);
                c = *fresh5;
                if c as libc::c_int == '\u{0}' as i32 {
                    return c as libc::c_int
                }
                if c as libc::c_int == '(' as i32 {
                    Count += 1
                } else if c as libc::c_int == ')' as i32 { Count -= 1 }
                if !(Count > 0 as libc::c_int) { break ; }
            }
        }
    };
}
#[c2rust::src_loc = "791:26"]
static mut yytable: [yytype_int8; 44] =
    [25 as libc::c_int as yytype_int8, 37 as libc::c_int as yytype_int8,
     38 as libc::c_int as yytype_int8, 26 as libc::c_int as yytype_int8,
     27 as libc::c_int as yytype_int8, 28 as libc::c_int as yytype_int8,
     29 as libc::c_int as yytype_int8, 30 as libc::c_int as yytype_int8,
     31 as libc::c_int as yytype_int8, 43 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 20 as libc::c_int as yytype_int8,
     3 as libc::c_int as yytype_int8, 32 as libc::c_int as yytype_int8,
     44 as libc::c_int as yytype_int8, 33 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 5 as libc::c_int as yytype_int8,
     21 as libc::c_int as yytype_int8, 45 as libc::c_int as yytype_int8,
     6 as libc::c_int as yytype_int8, 7 as libc::c_int as yytype_int8,
     8 as libc::c_int as yytype_int8, 9 as libc::c_int as yytype_int8,
     10 as libc::c_int as yytype_int8, 11 as libc::c_int as yytype_int8,
     12 as libc::c_int as yytype_int8, 43 as libc::c_int as yytype_int8,
     34 as libc::c_int as yytype_int8, 35 as libc::c_int as yytype_int8,
     39 as libc::c_int as yytype_int8, 22 as libc::c_int as yytype_int8,
     50 as libc::c_int as yytype_int8, 23 as libc::c_int as yytype_int8,
     24 as libc::c_int as yytype_int8, 36 as libc::c_int as yytype_int8,
     51 as libc::c_int as yytype_int8, 40 as libc::c_int as yytype_int8,
     41 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     42 as libc::c_int as yytype_int8, 47 as libc::c_int as yytype_int8,
     48 as libc::c_int as yytype_int8, 49 as libc::c_int as yytype_int8];
#[c2rust::src_loc = "800:26"]
static mut yycheck: [yytype_int8; 44] =
    [4 as libc::c_int as yytype_int8, 12 as libc::c_int as yytype_int8,
     13 as libc::c_int as yytype_int8, 7 as libc::c_int as yytype_int8,
     8 as libc::c_int as yytype_int8, 9 as libc::c_int as yytype_int8,
     10 as libc::c_int as yytype_int8, 11 as libc::c_int as yytype_int8,
     12 as libc::c_int as yytype_int8, 7 as libc::c_int as yytype_int8,
     16 as libc::c_int as yytype_int8, 18 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 17 as libc::c_int as yytype_int8,
     12 as libc::c_int as yytype_int8, 19 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 5 as libc::c_int as yytype_int8,
     13 as libc::c_int as yytype_int8, 17 as libc::c_int as yytype_int8,
     8 as libc::c_int as yytype_int8, 9 as libc::c_int as yytype_int8,
     10 as libc::c_int as yytype_int8, 11 as libc::c_int as yytype_int8,
     12 as libc::c_int as yytype_int8, 13 as libc::c_int as yytype_int8,
     14 as libc::c_int as yytype_int8, 7 as libc::c_int as yytype_int8,
     15 as libc::c_int as yytype_int8, 3 as libc::c_int as yytype_int8,
     12 as libc::c_int as yytype_int8, 8 as libc::c_int as yytype_int8,
     12 as libc::c_int as yytype_int8, 10 as libc::c_int as yytype_int8,
     11 as libc::c_int as yytype_int8, 18 as libc::c_int as yytype_int8,
     48 as libc::c_int as yytype_int8, 13 as libc::c_int as yytype_int8,
     13 as libc::c_int as yytype_int8, -(1 as libc::c_int) as yytype_int8,
     13 as libc::c_int as yytype_int8, 19 as libc::c_int as yytype_int8,
     13 as libc::c_int as yytype_int8, 13 as libc::c_int as yytype_int8];
#[c2rust::src_loc = "811:26"]
static mut yystos: [yytype_int8; 52] =
    [0 as libc::c_int as yytype_int8, 16 as libc::c_int as yytype_int8,
     21 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 5 as libc::c_int as yytype_int8,
     8 as libc::c_int as yytype_int8, 9 as libc::c_int as yytype_int8,
     10 as libc::c_int as yytype_int8, 11 as libc::c_int as yytype_int8,
     12 as libc::c_int as yytype_int8, 13 as libc::c_int as yytype_int8,
     14 as libc::c_int as yytype_int8, 22 as libc::c_int as yytype_int8,
     23 as libc::c_int as yytype_int8, 24 as libc::c_int as yytype_int8,
     25 as libc::c_int as yytype_int8, 26 as libc::c_int as yytype_int8,
     27 as libc::c_int as yytype_int8, 28 as libc::c_int as yytype_int8,
     18 as libc::c_int as yytype_int8, 13 as libc::c_int as yytype_int8,
     8 as libc::c_int as yytype_int8, 10 as libc::c_int as yytype_int8,
     11 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     7 as libc::c_int as yytype_int8, 8 as libc::c_int as yytype_int8,
     9 as libc::c_int as yytype_int8, 10 as libc::c_int as yytype_int8,
     11 as libc::c_int as yytype_int8, 12 as libc::c_int as yytype_int8,
     17 as libc::c_int as yytype_int8, 19 as libc::c_int as yytype_int8,
     15 as libc::c_int as yytype_int8, 3 as libc::c_int as yytype_int8,
     18 as libc::c_int as yytype_int8, 12 as libc::c_int as yytype_int8,
     13 as libc::c_int as yytype_int8, 12 as libc::c_int as yytype_int8,
     13 as libc::c_int as yytype_int8, 13 as libc::c_int as yytype_int8,
     13 as libc::c_int as yytype_int8, 7 as libc::c_int as yytype_int8,
     12 as libc::c_int as yytype_int8, 17 as libc::c_int as yytype_int8,
     29 as libc::c_int as yytype_int8, 19 as libc::c_int as yytype_int8,
     13 as libc::c_int as yytype_int8, 13 as libc::c_int as yytype_int8,
     12 as libc::c_int as yytype_int8, 29 as libc::c_int as yytype_int8];
#[c2rust::src_loc = "822:26"]
static mut yyr1: [yytype_int8; 42] =
    [0 as libc::c_int as yytype_int8, 20 as libc::c_int as yytype_int8,
     21 as libc::c_int as yytype_int8, 21 as libc::c_int as yytype_int8,
     21 as libc::c_int as yytype_int8, 22 as libc::c_int as yytype_int8,
     22 as libc::c_int as yytype_int8, 22 as libc::c_int as yytype_int8,
     22 as libc::c_int as yytype_int8, 22 as libc::c_int as yytype_int8,
     23 as libc::c_int as yytype_int8, 23 as libc::c_int as yytype_int8,
     23 as libc::c_int as yytype_int8, 23 as libc::c_int as yytype_int8,
     23 as libc::c_int as yytype_int8, 24 as libc::c_int as yytype_int8,
     24 as libc::c_int as yytype_int8, 24 as libc::c_int as yytype_int8,
     25 as libc::c_int as yytype_int8, 25 as libc::c_int as yytype_int8,
     25 as libc::c_int as yytype_int8, 26 as libc::c_int as yytype_int8,
     26 as libc::c_int as yytype_int8, 26 as libc::c_int as yytype_int8,
     26 as libc::c_int as yytype_int8, 26 as libc::c_int as yytype_int8,
     26 as libc::c_int as yytype_int8, 26 as libc::c_int as yytype_int8,
     26 as libc::c_int as yytype_int8, 27 as libc::c_int as yytype_int8,
     27 as libc::c_int as yytype_int8, 28 as libc::c_int as yytype_int8,
     28 as libc::c_int as yytype_int8, 28 as libc::c_int as yytype_int8,
     28 as libc::c_int as yytype_int8, 28 as libc::c_int as yytype_int8,
     28 as libc::c_int as yytype_int8, 28 as libc::c_int as yytype_int8,
     28 as libc::c_int as yytype_int8, 28 as libc::c_int as yytype_int8,
     29 as libc::c_int as yytype_int8, 29 as libc::c_int as yytype_int8];
#[c2rust::src_loc = "832:26"]
static mut yyr2: [yytype_int8; 42] =
    [0 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 1 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 1 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 1 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 6 as libc::c_int as yytype_int8,
     6 as libc::c_int as yytype_int8, 1 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 3 as libc::c_int as yytype_int8,
     5 as libc::c_int as yytype_int8, 3 as libc::c_int as yytype_int8,
     3 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     3 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 1 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 1 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 1 as libc::c_int as yytype_int8];
/* Yield A - B, measured in seconds.  */
#[c2rust::src_loc = "844:1"]
unsafe extern "C" fn difftm(mut a: *mut tm, mut b: *mut tm) -> time_t {
    let mut ay: libc::c_int =
        (*a).tm_year + (1900 as libc::c_int - 1 as libc::c_int);
    let mut by: libc::c_int =
        (*b).tm_year + (1900 as libc::c_int - 1 as libc::c_int);
    return (((((*a).tm_yday - (*b).tm_yday +
                   ((ay >> 2 as libc::c_int) - (by >> 2 as libc::c_int)) -
                   (ay / 100 as libc::c_int - by / 100 as libc::c_int) +
                   ((ay / 100 as libc::c_int >> 2 as libc::c_int) -
                        (by / 100 as libc::c_int >> 2 as libc::c_int))) as
                  libc::c_long +
                  (ay - by) as time_t * 365 as libc::c_int as libc::c_long) *
                 24 as libc::c_int as libc::c_long +
                 ((*a).tm_hour - (*b).tm_hour) as libc::c_long) *
                60 as libc::c_int as libc::c_long +
                ((*a).tm_min - (*b).tm_min) as libc::c_long) *
               60 as libc::c_int as libc::c_long +
               ((*a).tm_sec - (*b).tm_sec) as libc::c_long;
}
#[no_mangle]
#[c2rust::src_loc = "873:1"]
pub unsafe extern "C" fn get_date_rel(mut p: *mut libc::c_char,
                                      mut nowtime: time_t) -> time_t {
    let mut now: *mut my_timeb = 0 as *mut my_timeb;
    /* Enable debugging if requested.  */
    let mut tm: *mut tm =
        0 as *mut tm; /* Make a copy, in case localtime modifies *tm.  */
    let mut gmt: tm =
        tm{tm_sec: 0,
           tm_min: 0,
           tm_hour: 0,
           tm_mday: 0,
           tm_mon: 0,
           tm_year: 0,
           tm_wday: 0,
           tm_yday: 0,
           tm_isdst: 0,
           tm_gmtoff: 0,
           tm_zone: 0 as *const libc::c_char,};
    let mut ftz: my_timeb =
        my_timeb{time: 0, millitm: 0, timezone: 0, dstflag: 0,};
    let mut Start: time_t = 0;
    let mut tod: time_t = 0;
    let mut delta: time_t = 0;
    let mut error: libc::c_int = 0;
    yyInput = p;
    if now.is_null() {
        now = &mut ftz;
        ftz.time = nowtime;
        tm = gmtime(&mut ftz.time);
        if tm.is_null() { return -(1 as libc::c_int) as time_t }
        gmt = *tm;
        tm = localtime(&mut ftz.time);
        if tm.is_null() { return -(1 as libc::c_int) as time_t }
        ftz.timezone =
            (difftm(&mut gmt, tm) / 60 as libc::c_int as libc::c_long) as
                libc::c_short
    }
    tm = localtime(&mut (*now).time);
    if tm.is_null() { return -(1 as libc::c_int) as time_t }
    yyYear = (*tm).tm_year as time_t;
    yyMonth = ((*tm).tm_mon + 1 as libc::c_int) as time_t;
    yyDay = (*tm).tm_mday as time_t;
    yyTimezone = (*now).timezone as time_t;
    yyDSTmode = DSTmaybe;
    yyHour = 0 as libc::c_int as time_t;
    yyMinutes = 0 as libc::c_int as time_t;
    yySeconds = 0 as libc::c_int as time_t;
    yyMeridian = MER24;
    yyRelSeconds = 0 as libc::c_int as time_t;
    yyRelMonth = 0 as libc::c_int as time_t;
    yyHaveDate = 0 as libc::c_int;
    yyHaveDay = 0 as libc::c_int;
    yyHaveRel = 0 as libc::c_int;
    yyHaveTime = 0 as libc::c_int;
    yyHaveZone = 0 as libc::c_int;
    /*
     * When yyparse returns, zero or more of yyHave{Time,Zone,Date,Day,Rel} 
     * will have been incremented.  The value is number of items of
     * that type that were found; for all but Rel, more than one is
     * illegal.
     *
     * For each yyHave indicator, the following values are set:
     *
     * yyHaveTime:
     *	yyHour, yyMinutes, yySeconds: hh:mm:ss specified, initialized
     *				      to zeros above
     *	yyMeridian: MERam, MERpm, or MER24
     *	yyTimeZone: time zone specified in minutes
     *  yyDSTmode: DSToff if yyTimeZone is set, otherwise unchanged
     *		   (initialized above to DSTmaybe)
     *
     * yyHaveZone:
     *  yyTimezone: as above
     *  yyDSTmode: DSToff if a non-DST zone is specified, otherwise DSTon
     *	XXX don't understand interaction with yyHaveTime zone info
     *
     * yyHaveDay:
     *	yyDayNumber: 0-6 for Sunday-Saturday
     *  yyDayOrdinal: val specified with day ("second monday",
     *		      Ordinal=2), otherwise 1
     *
     * yyHaveDate:
     *	yyMonth, yyDay, yyYear: mm/dd/yy specified, initialized to
     *				today above
     *
     * yyHaveRel:
     *	yyRelSeconds: seconds specified with MINUTE_UNITs ("3 hours") or
     *		      SEC_UNITs ("30 seconds")
     *  yyRelMonth: months specified with MONTH_UNITs ("3 months", "1
     *		     year")
     *
     * The code following yyparse turns these values into a single
     * date stamp.
     */
    if getdate_yyparse() != 0 || yyHaveTime > 1 as libc::c_int ||
           yyHaveZone > 1 as libc::c_int || yyHaveDate > 1 as libc::c_int ||
           yyHaveDay > 1 as libc::c_int {
        return -(1 as libc::c_int) as time_t
    }
    /*
     * If an absolute time specified, set Start to the equivalent Unix
     * timestamp.  Otherwise, set Start to now, and if we do not have
     * a relatime time (ie: only yyHaveZone), decrement Start to the
     * beginning of today.
     *
     * By having yyHaveDay in the "absolute" list, "next Monday" means
     * midnight next Monday.  Otherwise, "next Monday" would mean the
     * time right now, next Monday.  It's not clear to me why the
     * current behavior is preferred.
     */
    if yyHaveDate != 0 || yyHaveTime != 0 || yyHaveDay != 0 {
        Start =
            Convert(yyMonth, yyDay, yyYear, yyHour, yyMinutes, yySeconds,
                    yyMeridian, yyDSTmode);
        if Start < 0 as libc::c_int as libc::c_long {
            return -(1 as libc::c_int) as time_t
        }
    } else {
        Start = (*now).time;
        if yyHaveRel == 0 {
            Start -=
                ((*tm).tm_hour as libc::c_long * 60 as libc::c_long +
                     (*tm).tm_min as libc::c_long) * 60 as libc::c_long +
                    (*tm).tm_sec as libc::c_long
        }
    }
    /*
     * Add in the relative time specified.  RelativeMonth adds in the
     * months, accounting for the fact that the actual length of "3
     * months" depends on where you start counting.
     *
     * XXX By having this separate from the previous block, we are
     * allowing dates like "10:00am 3 months", which means 3 months
     * from 10:00am today, or even "1/1/99 two days" which means two
     * days after 1/1/99.
     *
     * XXX Shouldn't this only be done if yyHaveRel, just for
     * thoroughness?
     */
    Start += yyRelSeconds;
    delta = RelativeMonth(Start, yyRelMonth);
    if delta == -(1 as libc::c_int) as time_t {
        return -(1 as libc::c_int) as time_t
    }
    Start += delta;
    /*
     * Now, if you specified a day of week and counter, add it in.  By
     * disallowing Date but allowing Time, you can say "5pm next
     * monday".
     *
     * XXX The yyHaveDay && !yyHaveDate restriction should be enforced
     * above and be able to cause failure.
     */
    /* !YYDEBUG */
    /* YYINITDEPTH -- initial size of the parser's stacks.  */
    if yyHaveDay != 0 && yyHaveDate == 0 {
        tod = RelativeDate(Start, yyDayOrdinal, yyDayNumber, &mut error);
        /* YYMAXDEPTH -- maximum size the stacks can grow to (effective only
   if the built-in stack extension method is used).

   Do not make this value too large; the results are undefined if
   YYSTACK_ALLOC_MAXIMUM < YYSTACK_BYTES (YYMAXDEPTH)
   evaluated with infinite-precision integer arithmetic.  */
        if error != 0 as libc::c_int { return -(1 as libc::c_int) as time_t }
        Start += tod
    }
    /* Have to do *something* with a legitimate -1 so it's distinguishable
     * from the error return value.  (Alternately could set errno on error.) */
    return if Start == -(1 as libc::c_int) as libc::c_long {
               0 as libc::c_int as libc::c_long
           } else { Start };
}
#[no_mangle]
#[c2rust::src_loc = "1025:1"]
pub unsafe extern "C" fn get_date(mut p: *mut libc::c_char) -> time_t {
    return get_date_rel(p, time(0 as *mut time_t));
}
/* defined(TEST) */
/* YYERROR_VERBOSE */
/*-----------------------------------------------.
| Release the memory associated to this symbol.  |
`-----------------------------------------------*/
#[c2rust::src_loc = "1262:1"]
unsafe extern "C" fn yydestruct(mut yymsg: *const libc::c_char,
                                mut yytype: libc::c_int,
                                mut yyvaluep: *mut YYSTYPE) {
    if yymsg.is_null() {
        yymsg = b"Deleting\x00" as *const u8 as *const libc::c_char
    };
}
/* The lookahead symbol.  */
#[no_mangle]
#[c2rust::src_loc = "1279:5"]
pub static mut yychar: libc::c_int = 0;
/* The semantic value of the lookahead symbol.  */
#[no_mangle]
#[c2rust::src_loc = "1282:9"]
pub static mut yylval: YYSTYPE = YYSTYPE{Number: 0,};
/* Number of syntax errors so far.  */
#[no_mangle]
#[c2rust::src_loc = "1284:5"]
pub static mut yynerrs: libc::c_int = 0;
/* For get_date extern declaration compatibility check... yuck.  */
/*----------.
| yyparse.  |
`----------*/
#[no_mangle]
#[c2rust::src_loc = "1291:1"]
pub unsafe extern "C" fn getdate_yyparse() -> libc::c_int {
    let mut current_block: u64;
    let mut yystate: yy_state_fast_t = 0;
    /* Number of tokens to shift before error messages enabled.  */
    let mut yyerrstatus: libc::c_int = 0;
    /* The stacks and their tools:
       'yyss': related to states.
       'yyvs': related to semantic values.

       Refer to the stacks through separate pointers, to allow yyoverflow
       to reallocate them elsewhere.  */
    /* The state stack.  */
    let mut yyssa: [yy_state_t; 200] = [0; 200];
    let mut yyss: *mut yy_state_t = 0 as *mut yy_state_t;
    let mut yyssp: *mut yy_state_t = 0 as *mut yy_state_t;
    /* The semantic value stack.  */
    let mut yyvsa: [YYSTYPE; 200] = [YYSTYPE{Number: 0,}; 200];
    let mut yyvs: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yyvsp: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yystacksize: libc::c_long = 0;
    let mut yyn: libc::c_int = 0;
    let mut yyresult: libc::c_int = 0;
    /* Lookahead token as an internal (translated) token number.  */
    let mut yytoken: libc::c_int = 0 as libc::c_int;
    /* The variables used to return semantic value and location from the
     action routines.  */
    let mut yyval: YYSTYPE = YYSTYPE{Number: 0,};
    /* The number of symbols on the RHS of the reduced rule.
     Keep to zero when no symbol should be popped.  */
    let mut yylen: libc::c_int =
        0 as libc::c_int; /* Cause a token to be read.  */
    yyss = yyssa.as_mut_ptr();
    yyssp = yyss;
    yyvs = yyvsa.as_mut_ptr();
    yyvsp = yyvs;
    yystacksize = 200 as libc::c_int as libc::c_long;
    yystate = 0 as libc::c_int;
    yyerrstatus = 0 as libc::c_int;
    yynerrs = 0 as libc::c_int;
    yychar = -(2 as libc::c_int);
    's_88:
        loop 
             /*--------------------------------------------------------------------.
| yysetstate -- set current state (the top of the stack) to yystate.  |
`--------------------------------------------------------------------*/
             {
            (0 as libc::c_int != 0 &&
                 (0 as libc::c_int <= yystate && yystate < 52 as libc::c_int))
                as libc::c_int;
            *yyssp = yystate as yy_state_t;
            if yyss.offset(yystacksize as
                               isize).offset(-(1 as libc::c_int as isize)) <=
                   yyssp {
                /* Get the current used size of the three stacks, in elements.  */
                let mut yysize: libc::c_long =
                    yyssp.wrapping_offset_from(yyss) as libc::c_long +
                        1 as libc::c_int as libc::c_long;
                /* defined YYSTACK_RELOCATE */
                /* Extend the stack our own way.  */
                if 10000 as libc::c_int as libc::c_long <= yystacksize {
                    current_block = 16403834700202643003;
                    break ;
                }
                yystacksize *= 2 as libc::c_int as libc::c_long;
                if (10000 as libc::c_int as libc::c_long) < yystacksize {
                    yystacksize = 10000 as libc::c_int as libc::c_long
                }
                let mut yyss1: *mut yy_state_t = yyss;
                let mut yyptr: *mut yyalloc =
                    malloc((yystacksize *
                                (::std::mem::size_of::<yy_state_t>() as
                                     libc::c_ulong as libc::c_long +
                                     ::std::mem::size_of::<YYSTYPE>() as
                                         libc::c_ulong as libc::c_long) +
                                (::std::mem::size_of::<yyalloc>() as
                                     libc::c_ulong as libc::c_long -
                                     1 as libc::c_int as libc::c_long)) as
                               libc::c_ulong) as *mut yyalloc;
                if yyptr.is_null() {
                    current_block = 16403834700202643003;
                    break ;
                }
                let mut yynewbytes: libc::c_long = 0;
                libc::memcpy(&mut (*yyptr).yyss_alloc as *mut yy_state_t as
                                 *mut libc::c_void,
                             yyss as *const libc::c_void,
                             (yysize as
                                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<yy_state_t>()
                                                                  as
                                                                  libc::c_ulong)
                                 as libc::size_t);
                yyss = &mut (*yyptr).yyss_alloc;
                yynewbytes =
                    yystacksize *
                        ::std::mem::size_of::<yy_state_t>() as libc::c_ulong
                            as libc::c_long +
                        (::std::mem::size_of::<yyalloc>() as libc::c_ulong as
                             libc::c_long - 1 as libc::c_int as libc::c_long);
                yyptr =
                    yyptr.offset((yynewbytes /
                                      ::std::mem::size_of::<yyalloc>() as
                                          libc::c_ulong as libc::c_long) as
                                     isize);
                let mut yynewbytes_0: libc::c_long = 0;
                libc::memcpy(&mut (*yyptr).yyvs_alloc as *mut YYSTYPE as
                                 *mut libc::c_void,
                             yyvs as *const libc::c_void,
                             (yysize as
                                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<YYSTYPE>()
                                                                  as
                                                                  libc::c_ulong)
                                 as libc::size_t);
                yyvs = &mut (*yyptr).yyvs_alloc;
                yynewbytes_0 =
                    yystacksize *
                        ::std::mem::size_of::<YYSTYPE>() as libc::c_ulong as
                            libc::c_long +
                        (::std::mem::size_of::<yyalloc>() as libc::c_ulong as
                             libc::c_long - 1 as libc::c_int as libc::c_long);
                yyptr =
                    yyptr.offset((yynewbytes_0 /
                                      ::std::mem::size_of::<yyalloc>() as
                                          libc::c_ulong as libc::c_long) as
                                     isize);
                if yyss1 != yyssa.as_mut_ptr() {
                    free(yyss1 as *mut libc::c_void);
                }
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
                    current_block = 12916222515858309620;
                    break ;
                }
            }
            /* !defined yyoverflow && !defined YYSTACK_RELOCATE */
            if yystate == 3 as libc::c_int {
                /*-------------------------------------.
| yyacceptlab -- YYACCEPT comes here.  |
`-------------------------------------*/
                yyresult = 0 as libc::c_int;
                current_block = 3900539361213381026;
                break ;
            } else {
                /*-----------.
| yybackup.  |
`-----------*/
                /* Do appropriate processing given the current state.  Read a
     lookahead token if we need one and don't already have one.  */
                /* First try to decide what to do without reference to lookahead token.  */
                yyn = yypact[yystate as usize] as libc::c_int;
                if yyn == -(13 as libc::c_int) {
                    current_block = 10465836181876106334;
                } else {
                    /* Not known => get a lookahead token if don't already have one.  */
                    /* YYCHAR is either YYEMPTY or YYEOF or a valid lookahead symbol.  */
                    if yychar == -(2 as libc::c_int) {
                        yychar = getdate_yylex()
                    }
                    if yychar <= 0 as libc::c_int {
                        yytoken = 0 as libc::c_int;
                        yychar = yytoken
                    } else {
                        yytoken =
                            if 0 as libc::c_int <= yychar &&
                                   yychar <= 271 as libc::c_int {
                                yytranslate[yychar as usize] as libc::c_int
                            } else { 2 as libc::c_int }
                    }
                    /* If the proper action on seeing token YYTOKEN is to reduce or to
     detect an error, take that action.  */
                    yyn += yytoken;
                    if yyn < 0 as libc::c_int || (43 as libc::c_int) < yyn ||
                           yycheck[yyn as usize] as libc::c_int != yytoken {
                        current_block = 10465836181876106334;
                    } else {
                        yyn = yytable[yyn as usize] as libc::c_int;
                        if yyn <= 0 as libc::c_int {
                            yyn = -yyn;
                            current_block = 676744132399875722;
                        } else {
                            /* Count tokens shifted since error; after three, turn off error
     status.  */
                            if yyerrstatus != 0 { yyerrstatus -= 1 }
                            /* Shift the lookahead token.  */
                            yystate = yyn;
                            yyvsp = yyvsp.offset(1);
                            *yyvsp = yylval;
                            /* Discard the shifted token.  */
                            yychar = -(2 as libc::c_int);
                            current_block = 1265369698245557087;
                        }
                    }
                }
                match current_block {
                    10465836181876106334 =>
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
                                } else if 0 as libc::c_int <= yychar &&
                                              yychar <= 271 as libc::c_int {
                                    yytranslate[yychar as usize] as
                                        libc::c_int
                                } else { 2 as libc::c_int };
                            /* If not already recovering from an error, report this error.  */
                            if yyerrstatus == 0 {
                                yynerrs += 1;
                                getdate_yyerror(b"syntax error\x00" as
                                                    *const u8 as
                                                    *const libc::c_char as
                                                    *mut libc::c_char);
                            }
                            if yyerrstatus == 3 as libc::c_int {
                                /* If just tried and failed to reuse lookahead token after an
         error, discard it.  */
                                if yychar <= 0 as libc::c_int {
                                    /* Return failure if at end of input.  */
                                    if yychar == 0 as libc::c_int {
                                        current_block = 12916222515858309620;
                                        break ;
                                    }
                                } else {
                                    yydestruct(b"Error: discarding\x00" as
                                                   *const u8 as
                                                   *const libc::c_char,
                                               yytoken, &mut yylval);
                                    yychar = -(2 as libc::c_int)
                                }
                            }
                            /* Else will try to reuse lookahead token after shifting the error
     token.  */
                            /*-------------------------------------------------------------.
| yyerrlab1 -- common code for both syntax error and YYERROR.  |
`-------------------------------------------------------------*/
                            yyerrstatus =
                                3 as
                                    libc::c_int; /* Each real token shifted decrements this.  */
                            loop  {
                                yyn = yypact[yystate as usize] as libc::c_int;
                                if !(yyn == -(13 as libc::c_int)) {
                                    yyn += 1 as libc::c_int;
                                    if 0 as libc::c_int <= yyn &&
                                           yyn <= 43 as libc::c_int &&
                                           yycheck[yyn as usize] as
                                               libc::c_int == 1 as libc::c_int
                                       {
                                        yyn =
                                            yytable[yyn as usize] as
                                                libc::c_int;
                                        if (0 as libc::c_int) < yyn {
                                            break ;
                                        }
                                    }
                                }
                                /* Pop the current state because it cannot handle the error token.  */
                                if yyssp == yyss {
                                    current_block = 12916222515858309620;
                                    break 's_88 ;
                                }
                                yydestruct(b"Error: popping\x00" as *const u8
                                               as *const libc::c_char,
                                           yystos[yystate as usize] as
                                               libc::c_int, yyvsp);
                                yyvsp =
                                    yyvsp.offset(-(1 as libc::c_int as
                                                       isize));
                                yyssp =
                                    yyssp.offset(-(1 as libc::c_int as
                                                       isize));
                                yystate = *yyssp as yy_state_fast_t
                            }
                            yyvsp = yyvsp.offset(1);
                            *yyvsp = yylval;
                            /* Shift the error token.  */
                            yystate = yyn;
                            current_block = 1265369698245557087;
                        } else { current_block = 676744132399875722; }
                    }
                    _ => { }
                }
                match current_block {
                    676744132399875722 =>
                    /*-----------------------------.
| yyreduce -- do a reduction.  |
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
                            4 => {
                                yyYear = 1970 as libc::c_int as time_t;
                                yyMonth = 1 as libc::c_int as time_t;
                                yyDay = 1 as libc::c_int as time_t;
                                yySeconds = 0 as libc::c_int as time_t;
                                yyMinutes = yySeconds;
                                yyHour = yyMinutes;
                                yyDSTmode = DSToff;
                                yyTimezone = 0 as libc::c_int as time_t;
                                yyHaveDate += 1
                            }
                            5 => { yyHaveTime += 1 }
                            6 => { yyHaveZone += 1 }
                            7 => { yyHaveDate += 1 }
                            8 => { yyHaveDay += 1 }
                            9 => { yyHaveRel += 1 }
                            10 => {
                                yyHour =
                                    (*yyvsp.offset(-(1 as libc::c_int) as
                                                       isize)).Number;
                                yyMinutes = 0 as libc::c_int as time_t;
                                yySeconds = 0 as libc::c_int as time_t;
                                yyMeridian =
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).Meridian
                            }
                            11 => {
                                yyHour =
                                    (*yyvsp.offset(-(3 as libc::c_int) as
                                                       isize)).Number;
                                yyMinutes =
                                    (*yyvsp.offset(-(1 as libc::c_int) as
                                                       isize)).Number;
                                yySeconds = 0 as libc::c_int as time_t;
                                yyMeridian =
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).Meridian
                            }
                            12 => {
                                yyHour =
                                    (*yyvsp.offset(-(3 as libc::c_int) as
                                                       isize)).Number;
                                yyMinutes =
                                    (*yyvsp.offset(-(1 as libc::c_int) as
                                                       isize)).Number;
                                yyMeridian = MER24;
                                yyDSTmode = DSToff;
                                yyTimezone =
                                    -((*yyvsp.offset(0 as libc::c_int as
                                                         isize)).Number %
                                          100 as libc::c_int as libc::c_long +
                                          (*yyvsp.offset(0 as libc::c_int as
                                                             isize)).Number /
                                              100 as libc::c_int as
                                                  libc::c_long *
                                              60 as libc::c_int as
                                                  libc::c_long)
                            }
                            13 => {
                                yyHour =
                                    (*yyvsp.offset(-(5 as libc::c_int) as
                                                       isize)).Number;
                                yyMinutes =
                                    (*yyvsp.offset(-(3 as libc::c_int) as
                                                       isize)).Number;
                                yySeconds =
                                    (*yyvsp.offset(-(1 as libc::c_int) as
                                                       isize)).Number;
                                yyMeridian =
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).Meridian
                            }
                            14 => {
                                yyHour =
                                    (*yyvsp.offset(-(5 as libc::c_int) as
                                                       isize)).Number;
                                yyMinutes =
                                    (*yyvsp.offset(-(3 as libc::c_int) as
                                                       isize)).Number;
                                yySeconds =
                                    (*yyvsp.offset(-(1 as libc::c_int) as
                                                       isize)).Number;
                                yyMeridian = MER24;
                                yyDSTmode = DSToff;
                                yyTimezone =
                                    -((*yyvsp.offset(0 as libc::c_int as
                                                         isize)).Number %
                                          100 as libc::c_int as libc::c_long +
                                          (*yyvsp.offset(0 as libc::c_int as
                                                             isize)).Number /
                                              100 as libc::c_int as
                                                  libc::c_long *
                                              60 as libc::c_int as
                                                  libc::c_long)
                            }
                            15 => {
                                yyTimezone =
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).Number;
                                yyDSTmode = DSToff
                            }
                            16 => {
                                yyTimezone =
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).Number;
                                yyDSTmode = DSTon
                            }
                            17 => {
                                yyTimezone =
                                    (*yyvsp.offset(-(1 as libc::c_int) as
                                                       isize)).Number;
                                yyDSTmode = DSTon
                            }
                            18 => {
                                yyDayOrdinal = 1 as libc::c_int as time_t;
                                yyDayNumber =
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).Number
                            }
                            19 => {
                                yyDayOrdinal = 1 as libc::c_int as time_t;
                                yyDayNumber =
                                    (*yyvsp.offset(-(1 as libc::c_int) as
                                                       isize)).Number
                            }
                            20 => {
                                yyDayOrdinal =
                                    (*yyvsp.offset(-(1 as libc::c_int) as
                                                       isize)).Number;
                                yyDayNumber =
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).Number
                            }
                            21 => {
                                yyMonth =
                                    (*yyvsp.offset(-(2 as libc::c_int) as
                                                       isize)).Number;
                                yyDay =
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).Number
                            }
                            22 => {
                                yyMonth =
                                    (*yyvsp.offset(-(4 as libc::c_int) as
                                                       isize)).Number;
                                yyDay =
                                    (*yyvsp.offset(-(2 as libc::c_int) as
                                                       isize)).Number;
                                yyYear =
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).Number
                            }
                            23 => {
                                yyYear =
                                    (*yyvsp.offset(-(2 as libc::c_int) as
                                                       isize)).Number;
                                yyMonth =
                                    -(*yyvsp.offset(-(1 as libc::c_int) as
                                                        isize)).Number;
                                yyDay =
                                    -(*yyvsp.offset(0 as libc::c_int as
                                                        isize)).Number
                            }
                            24 => {
                                yyDay =
                                    (*yyvsp.offset(-(2 as libc::c_int) as
                                                       isize)).Number;
                                yyMonth =
                                    (*yyvsp.offset(-(1 as libc::c_int) as
                                                       isize)).Number;
                                yyYear =
                                    -(*yyvsp.offset(0 as libc::c_int as
                                                        isize)).Number
                            }
                            25 => {
                                yyMonth =
                                    (*yyvsp.offset(-(1 as libc::c_int) as
                                                       isize)).Number;
                                yyDay =
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).Number
                            }
                            26 => {
                                yyMonth =
                                    (*yyvsp.offset(-(3 as libc::c_int) as
                                                       isize)).Number;
                                yyDay =
                                    (*yyvsp.offset(-(2 as libc::c_int) as
                                                       isize)).Number;
                                yyYear =
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).Number
                            }
                            27 => {
                                yyMonth =
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).Number;
                                yyDay =
                                    (*yyvsp.offset(-(1 as libc::c_int) as
                                                       isize)).Number
                            }
                            28 => {
                                yyMonth =
                                    (*yyvsp.offset(-(1 as libc::c_int) as
                                                       isize)).Number;
                                yyDay =
                                    (*yyvsp.offset(-(2 as libc::c_int) as
                                                       isize)).Number;
                                yyYear =
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).Number
                            }
                            29 => {
                                yyRelSeconds = -yyRelSeconds;
                                yyRelMonth = -yyRelMonth
                            }
                            31 => {
                                yyRelSeconds +=
                                    (*yyvsp.offset(-(1 as libc::c_int) as
                                                       isize)).Number *
                                        (*yyvsp.offset(0 as libc::c_int as
                                                           isize)).Number *
                                        60 as libc::c_long
                            }
                            32 => {
                                yyRelSeconds +=
                                    (*yyvsp.offset(-(1 as libc::c_int) as
                                                       isize)).Number *
                                        (*yyvsp.offset(0 as libc::c_int as
                                                           isize)).Number *
                                        60 as libc::c_long
                            }
                            33 => {
                                yyRelSeconds +=
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).Number *
                                        60 as libc::c_long
                            }
                            34 => {
                                yyRelSeconds +=
                                    (*yyvsp.offset(-(1 as libc::c_int) as
                                                       isize)).Number
                            }
                            35 => {
                                yyRelSeconds +=
                                    (*yyvsp.offset(-(1 as libc::c_int) as
                                                       isize)).Number
                            }
                            36 => { yyRelSeconds += 1 }
                            37 => {
                                yyRelMonth +=
                                    (*yyvsp.offset(-(1 as libc::c_int) as
                                                       isize)).Number *
                                        (*yyvsp.offset(0 as libc::c_int as
                                                           isize)).Number
                            }
                            38 => {
                                yyRelMonth +=
                                    (*yyvsp.offset(-(1 as libc::c_int) as
                                                       isize)).Number *
                                        (*yyvsp.offset(0 as libc::c_int as
                                                           isize)).Number
                            }
                            39 => {
                                yyRelMonth +=
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).Number
                            }
                            40 => { yyval.Meridian = MER24 }
                            41 => {
                                yyval.Meridian =
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).Meridian
                            }
                            _ => { }
                        }
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
                        yyvsp = yyvsp.offset(-(yylen as isize));
                        yyssp = yyssp.offset(-(yylen as isize));
                        yylen = 0 as libc::c_int;
                        yyvsp = yyvsp.offset(1);
                        *yyvsp = yyval;
                        /* Now 'shift' the result of the reduction.  Determine what state
     that goes to, based on the state we popped back to and the rule
     number reduced by.  */
                        let yylhs: libc::c_int =
                            yyr1[yyn as usize] as libc::c_int -
                                20 as libc::c_int;
                        let yyi: libc::c_int =
                            yypgoto[yylhs as usize] as libc::c_int +
                                *yyssp as libc::c_int;
                        yystate =
                            if 0 as libc::c_int <= yyi &&
                                   yyi <= 43 as libc::c_int &&
                                   yycheck[yyi as usize] as libc::c_int ==
                                       *yyssp as libc::c_int {
                                yytable[yyi as usize] as libc::c_int
                            } else {
                                yydefgoto[yylhs as usize] as libc::c_int
                            }
                    }
                    _ => { }
                }
                /*------------------------------------------------------------.
| yynewstate -- push a new state, which is found in yystate.  |
`------------------------------------------------------------*/
                /* In all cases, when you get here, the value and location stacks
     have just been pushed.  So pushing a state here evens the stacks.  */
                yyssp = yyssp.offset(1)
            }
        }
    match current_block {
        16403834700202643003 =>
        /*-------------------------------------------------.
| yyexhaustedlab -- memory exhaustion comes here.  |
`-------------------------------------------------*/
        {
            getdate_yyerror(b"memory exhausted\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char);
            yyresult = 2 as libc::c_int
        }
        12916222515858309620 =>
        /*-----------------------------------.
| yyabortlab -- YYABORT comes here.  |
`-----------------------------------*/
        {
            yyresult = 1 as libc::c_int
        }
        _ => { }
    }
    /* Fall through.  */
    /*-----------------------------------------------------.
| yyreturn -- parsing is finished, return the result.  |
`-----------------------------------------------------*/
    if yychar != -(2 as libc::c_int) {
        /* Make sure we have latest lookahead translation.  See comments at
         user semantic actions for why this is necessary.  */
        yytoken =
            if 0 as libc::c_int <= yychar && yychar <= 271 as libc::c_int {
                yytranslate[yychar as usize] as libc::c_int
            } else { 2 as libc::c_int };
        yydestruct(b"Cleanup: discarding lookahead\x00" as *const u8 as
                       *const libc::c_char, yytoken, &mut yylval);
    }
    /* Do not reclaim the symbols of the rule whose action triggered
     this YYABORT or YYACCEPT.  */
    yyvsp = yyvsp.offset(-(yylen as isize));
    yyssp = yyssp.offset(-(yylen as isize));
    while yyssp != yyss {
        yydestruct(b"Cleanup: popping\x00" as *const u8 as
                       *const libc::c_char,
                   yystos[*yyssp as usize] as libc::c_int, yyvsp);
        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
        yyssp = yyssp.offset(-(1 as libc::c_int as isize))
    }
    if yyss != yyssa.as_mut_ptr() { free(yyss as *mut libc::c_void); }
    return yyresult;
}
