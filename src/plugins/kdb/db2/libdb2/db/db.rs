use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:38"]
pub mod types_h {
    #[c2rust::src_loc = "31:1"]
    pub type __u_char = libc::c_uchar;
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "34:1"]
    pub type __u_long = libc::c_ulong;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/sys/types.h:38"]
pub mod sys_types_h {
    #[c2rust::src_loc = "33:1"]
    pub type u_char = __u_char;
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "36:1"]
    pub type u_long = __u_long;
    #[c2rust::src_loc = "160:1"]
    pub type u_int32_t = __uint32_t;
    use super::types_h::{__u_char, __u_int, __u_long, __uint32_t};
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:38"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/db.h:45"]
pub mod db_h {
    /*-
 * Copyright (c) 1990, 1993, 1994
 *	The Regents of the University of California.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *	This product includes software developed by the University of
 *	California, Berkeley and its contributors.
 * 4. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 *	@(#)db.h	8.8 (Berkeley) 11/2/95
 */
    /* Return values. */
    /* Key/data structure -- a Data-Base Thang. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "48:9"]
    pub struct DBT {
        pub data: *mut libc::c_void,
        pub size: size_t,
    }
    #[c2rust::src_loc = "83:9"]
    pub type DBTYPE = libc::c_uint;
    #[c2rust::src_loc = "83:35"]
    pub const DB_RECNO: DBTYPE = 2;
    #[c2rust::src_loc = "83:26"]
    pub const DB_HASH: DBTYPE = 1;
    #[c2rust::src_loc = "83:16"]
    pub const DB_BTREE: DBTYPE = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "119:16"]
    pub struct __db {
        pub type_0: DBTYPE,
        pub close: Option<unsafe extern "C" fn(_: *mut __db) -> libc::c_int>,
        pub del: Option<unsafe extern "C" fn(_: *const __db, _: *const DBT,
                                             _: u_int) -> libc::c_int>,
        pub get: Option<unsafe extern "C" fn(_: *const __db, _: *const DBT,
                                             _: *mut DBT, _: u_int)
                            -> libc::c_int>,
        pub put: Option<unsafe extern "C" fn(_: *const __db, _: *mut DBT,
                                             _: *const DBT, _: u_int)
                            -> libc::c_int>,
        pub seq: Option<unsafe extern "C" fn(_: *const __db, _: *mut DBT,
                                             _: *mut DBT, _: u_int)
                            -> libc::c_int>,
        pub sync: Option<unsafe extern "C" fn(_: *const __db, _: u_int)
                             -> libc::c_int>,
        pub internal: *mut libc::c_void,
        pub fd: Option<unsafe extern "C" fn(_: *const __db) -> libc::c_int>,
    }
    #[c2rust::src_loc = "119:1"]
    pub type DB = __db;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "135:9"]
    pub struct BTREEINFO {
        pub flags: u_long,
        pub cachesize: u_int,
        pub maxkeypage: libc::c_int,
        pub minkeypage: libc::c_int,
        pub psize: u_int,
        pub compare: Option<unsafe extern "C" fn(_: *const DBT, _: *const DBT)
                                -> libc::c_int>,
        pub prefix: Option<unsafe extern "C" fn(_: *const DBT, _: *const DBT)
                               -> size_t>,
        pub lorder: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "153:9"]
    pub struct HASHINFO {
        pub bsize: u_int,
        pub ffactor: u_int,
        pub nelem: u_int,
        pub cachesize: u_int,
        pub hash: Option<unsafe extern "C" fn(_: *const libc::c_void,
                                              _: size_t) -> u_int32_t>,
        pub lorder: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "164:9"]
    pub struct RECNOINFO {
        pub flags: u_long,
        pub cachesize: u_int,
        pub psize: u_int,
        pub lorder: libc::c_int,
        pub reclen: size_t,
        pub bval: u_char,
        pub bfname: *mut libc::c_char,
    }
    use super::stddef_h::size_t;
    use super::sys_types_h::{u_int, u_long, u_int32_t, u_char};
    /* !_DB_H_ */
}
#[c2rust::header_src = "/usr/include/errno.h:40"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/include/db-int.h:45"]
pub mod db_int_h {
    use super::db_h::{RECNOINFO, DB, HASHINFO, BTREEINFO};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "212:1"]
        pub fn __kdb2_rec_open(_: *const libc::c_char, _: libc::c_int,
                               _: libc::c_int, _: *const RECNOINFO,
                               _: libc::c_int) -> *mut DB;
        #[no_mangle]
        #[c2rust::src_loc = "211:1"]
        pub fn __kdb2_hash_open(_: *const libc::c_char, _: libc::c_int,
                                _: libc::c_int, _: *const HASHINFO,
                                _: libc::c_int) -> *mut DB;
        #[no_mangle]
        #[c2rust::src_loc = "210:1"]
        pub fn __kdb2_bt_open(_: *const libc::c_char, _: libc::c_int,
                              _: libc::c_int, _: *const BTREEINFO,
                              _: libc::c_int) -> *mut DB;
    }
    /* _DB_INT_H_ */
    /* Needed for Win32 compiles */
}
pub use self::types_h::{__u_char, __u_int, __u_long, __uint32_t};
pub use self::sys_types_h::{u_char, u_int, u_long, u_int32_t};
pub use self::stddef_h::size_t;
pub use self::db_h::{DBT, DBTYPE, DB_RECNO, DB_HASH, DB_BTREE, __db, DB,
                     BTREEINFO, HASHINFO, RECNOINFO};
use self::errno_h::__errno_location;
use self::db_int_h::{__kdb2_rec_open, __kdb2_hash_open, __kdb2_bt_open};
/* data length */
/* Routine flags. */
/* del, put, seq */
/* UNUSED */
/* seq */
/* put (RECNO) */
/* put (RECNO) */
/* seq (BTREE, RECNO) */
/* seq */
/* put */
/* seq (BTREE, RECNO) */
/* put (RECNO) */
/* sync (RECNO) */
/*
 * Recursive sequential scan.
 *
 * This avoids using sibling pointers, permitting (possibly partial)
 * recovery from some kinds of btree corruption.  Start a sequential
 * scan as usual, but use R_RNEXT or R_RPREV to move forward or
 * backward.
 *
 * This probably doesn't work with btrees that allow duplicate keys.
 * Database modifications during the scan can also modify the parent
 * page stack needed for correct functioning.  Intermixing
 * non-recursive traversal by using R_NEXT or R_PREV can also make the
 * page stack inconsistent with the cursor and cause problems.
 */
/* seq (BTREE, RECNO) */
/* seq (BTREE, RECNO) */
/*
 * !!!
 * The following flags are included in the dbopen(3) call as part of the
 * open(2) flags.  In order to avoid conflicts with the open flags, start
 * at the top of the 16 or 32-bit number space and work our way down.  If
 * the open flags were significantly expanded in the future, it could be
 * a problem.  Wish I'd left another flags word in the dbopen call.
 *
 * !!!
 * None of this stuff is implemented yet.  The only reason that it's here
 * is so that the access methods can skip copying the key/data pair when
 * the DB_LOCK flag isn't set.
 */
/* Do locking. */
/* Use shared memory. */
/* Do transactions. */
/* deal with turning prototypes on and off */
/* no __P from system */
/* Access method description structure. */
/* Underlying db type. */
/* Access method private. */
/* Structure used to pass parameters to the btree routines. */
/* duplicate keys */
/* bytes to cache */
/* maximum keys per page */
/* minimum keys per page */
/* page size */
/* comparison function */
/* prefix function */
/* byte order */
/* Structure used to pass parameters to the hashing routines. */
/* bucket size */
/* fill factor */
/* number of elements */
/* bytes to cache */
/* hash function */
/* byte order */
/* Structure used to pass parameters to the record routines. */
/* fixed-length records */
/* key not required */
/* snapshot the input */
/* bytes to cache */
/* page size */
/* byte order */
/* record length (fixed-length records) */
/* delimiting byte (variable-length records */
/* btree file name */
/*-
 * Copyright (c) 1991, 1993
 *	The Regents of the University of California.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *	This product includes software developed by the University of
 *	California, Berkeley and its contributors.
 * 4. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
/* LIBC_SCCS and not lint */
#[no_mangle]
#[c2rust::src_loc = "47:1"]
pub unsafe extern "C" fn kdb2_dbopen(mut fname: *const libc::c_char,
                                     mut flags: libc::c_int,
                                     mut mode: libc::c_int,
                                     mut type_0: DBTYPE,
                                     mut openinfo: *const libc::c_void)
 -> *mut DB {
    if flags &
           !(0o100 as libc::c_int | 0o200 as libc::c_int | 0 as libc::c_int |
                 0o4000 as libc::c_int | 0 as libc::c_int | 0o2 as libc::c_int
                 | 0 as libc::c_int | 0o1000 as libc::c_int | 0 as libc::c_int
                 |
                 (0x2000 as libc::c_int | 0x4000 as libc::c_int |
                      0x8000 as libc::c_int)) == 0 as libc::c_int {
        match type_0 as libc::c_uint {
            0 => {
                return __kdb2_bt_open(fname,
                                      flags &
                                          (0o100 as libc::c_int |
                                               0o200 as libc::c_int |
                                               0 as libc::c_int |
                                               0o4000 as libc::c_int |
                                               0 as libc::c_int |
                                               0o2 as libc::c_int |
                                               0 as libc::c_int |
                                               0o1000 as libc::c_int |
                                               0 as libc::c_int), mode,
                                      openinfo as *const BTREEINFO,
                                      flags &
                                          (0x2000 as libc::c_int |
                                               0x4000 as libc::c_int |
                                               0x8000 as libc::c_int))
            }
            1 => {
                return __kdb2_hash_open(fname,
                                        flags &
                                            (0o100 as libc::c_int |
                                                 0o200 as libc::c_int |
                                                 0 as libc::c_int |
                                                 0o4000 as libc::c_int |
                                                 0 as libc::c_int |
                                                 0o2 as libc::c_int |
                                                 0 as libc::c_int |
                                                 0o1000 as libc::c_int |
                                                 0 as libc::c_int), mode,
                                        openinfo as *const HASHINFO,
                                        flags &
                                            (0x2000 as libc::c_int |
                                                 0x4000 as libc::c_int |
                                                 0x8000 as libc::c_int))
            }
            2 => {
                return __kdb2_rec_open(fname,
                                       flags &
                                           (0o100 as libc::c_int |
                                                0o200 as libc::c_int |
                                                0 as libc::c_int |
                                                0o4000 as libc::c_int |
                                                0 as libc::c_int |
                                                0o2 as libc::c_int |
                                                0 as libc::c_int |
                                                0o1000 as libc::c_int |
                                                0 as libc::c_int), mode,
                                       openinfo as *const RECNOINFO,
                                       flags &
                                           (0x2000 as libc::c_int |
                                                0x4000 as libc::c_int |
                                                0x8000 as libc::c_int))
            }
            _ => { }
        }
    }
    *__errno_location() = 22 as libc::c_int;
    return 0 as *mut DB;
}
#[c2rust::src_loc = "76:1"]
unsafe extern "C" fn __dberr() -> libc::c_int { return -(1 as libc::c_int); }
/*
 * __DBPANIC -- Stop.
 *
 * Parameters:
 *	dbp:	pointer to the DB structure.
 */
#[no_mangle]
#[c2rust::src_loc = "88:1"]
pub unsafe extern "C" fn __kdb2_dbpanic(mut dbp: *mut DB) {
    /* The only thing that can succeed is a close. */
    (*dbp).del =
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> libc::c_int>,
                                Option<unsafe extern "C" fn(_: *const __db,
                                                            _: *const DBT,
                                                            _: u_int)
                                           ->
                                               libc::c_int>>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                ->
                                                                                                    libc::c_int>,
                                                                                     Option<unsafe extern "C" fn()
                                                                                                ->
                                                                                                    libc::c_int>>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                   ->
                                                                                                                                                       libc::c_int,
                                                                                                                                               unsafe extern "C" fn()
                                                                                                                                                   ->
                                                                                                                                                       libc::c_int>(__dberr))));
    (*dbp).fd =
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> libc::c_int>,
                                Option<unsafe extern "C" fn(_: *const __db)
                                           ->
                                               libc::c_int>>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                ->
                                                                                                    libc::c_int>,
                                                                                     Option<unsafe extern "C" fn()
                                                                                                ->
                                                                                                    libc::c_int>>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                   ->
                                                                                                                                                       libc::c_int,
                                                                                                                                               unsafe extern "C" fn()
                                                                                                                                                   ->
                                                                                                                                                       libc::c_int>(__dberr))));
    (*dbp).get =
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> libc::c_int>,
                                Option<unsafe extern "C" fn(_: *const __db,
                                                            _: *const DBT,
                                                            _: *mut DBT,
                                                            _: u_int)
                                           ->
                                               libc::c_int>>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                ->
                                                                                                    libc::c_int>,
                                                                                     Option<unsafe extern "C" fn()
                                                                                                ->
                                                                                                    libc::c_int>>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                   ->
                                                                                                                                                       libc::c_int,
                                                                                                                                               unsafe extern "C" fn()
                                                                                                                                                   ->
                                                                                                                                                       libc::c_int>(__dberr))));
    (*dbp).put =
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> libc::c_int>,
                                Option<unsafe extern "C" fn(_: *const __db,
                                                            _: *mut DBT,
                                                            _: *const DBT,
                                                            _: u_int)
                                           ->
                                               libc::c_int>>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                ->
                                                                                                    libc::c_int>,
                                                                                     Option<unsafe extern "C" fn()
                                                                                                ->
                                                                                                    libc::c_int>>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                   ->
                                                                                                                                                       libc::c_int,
                                                                                                                                               unsafe extern "C" fn()
                                                                                                                                                   ->
                                                                                                                                                       libc::c_int>(__dberr))));
    (*dbp).seq =
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> libc::c_int>,
                                Option<unsafe extern "C" fn(_: *const __db,
                                                            _: *mut DBT,
                                                            _: *mut DBT,
                                                            _: u_int)
                                           ->
                                               libc::c_int>>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                ->
                                                                                                    libc::c_int>,
                                                                                     Option<unsafe extern "C" fn()
                                                                                                ->
                                                                                                    libc::c_int>>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                   ->
                                                                                                                                                       libc::c_int,
                                                                                                                                               unsafe extern "C" fn()
                                                                                                                                                   ->
                                                                                                                                                       libc::c_int>(__dberr))));
    (*dbp).sync =
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> libc::c_int>,
                                Option<unsafe extern "C" fn(_: *const __db,
                                                            _: u_int)
                                           ->
                                               libc::c_int>>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                ->
                                                                                                    libc::c_int>,
                                                                                     Option<unsafe extern "C" fn()
                                                                                                ->
                                                                                                    libc::c_int>>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                                                   ->
                                                                                                                                                       libc::c_int,
                                                                                                                                               unsafe extern "C" fn()
                                                                                                                                                   ->
                                                                                                                                                       libc::c_int>(__dberr))));
}
