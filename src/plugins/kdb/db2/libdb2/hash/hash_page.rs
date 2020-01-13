use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:57"]
pub mod types_h {
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "34:1"]
    pub type __u_long = libc::c_ulong;
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = libc::c_schar;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/sys/types.h:57"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "36:1"]
    pub type u_long = __u_long;
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    #[c2rust::src_loc = "158:1"]
    pub type u_int8_t = __uint8_t;
    #[c2rust::src_loc = "159:1"]
    pub type u_int16_t = __uint16_t;
    #[c2rust::src_loc = "160:1"]
    pub type u_int32_t = __uint32_t;
    use super::types_h::{__u_int, __u_long, __ssize_t, __uint8_t, __uint16_t,
                         __uint32_t};
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:57"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:57"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int8_t, __int16_t, __int32_t};
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/db.h:67"]
pub mod db_h {
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
    /* data */
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
    use super::stddef_h::size_t;
    use super::sys_types_h::u_int;
    /* !_DB_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/include/db-int.h:67"]
pub mod db_int_h {
    /*-
 * Copyright (c) 1991, 1993, 2007
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
 *	@(#)compat.h	8.13 (Berkeley) 2/21/94
 */
    /* deal with autoconf-based stuff */
    /* Handle both BIG and LITTLE defined and BYTE_ORDER matches one, or
   just one defined; both with and without leading underscores.

   Ignore "PDP endian" machines, this code doesn't support them
   anyways.  */
    /* end autoconf-based stuff */
    /* include necessary system header files */
    /* types and constants used for database structure */
    /* >= # of pages in a file */
    #[c2rust::src_loc = "143:1"]
    pub type db_pgno_t = u_int32_t;
    /* >= # of bytes in a page */
    #[c2rust::src_loc = "145:1"]
    pub type indx_t = u_int16_t;
    use super::sys_types_h::{u_int32_t, u_int16_t};
    /* _DB_INT_H_ */
    /* Needed for Win32 compiles */
    /* BSD POSIX 1003.1 extensions */
    /* POSIX 1003.1 file type tests. */
    /* Usually found in <sys/param.h>. */
    /* Usually found in <sys/param.h>. */
    /* ANSI C #defines NULL everywhere. */
    /* POSIX 1003.1 format errno. */
    /* 4.4BSD extension. */
    /* 4.4BSD extension. */
    /*
 * If you can't provide lock values in the open(2) call.  Note, this
 * allows races to happen.
 */
    /* POSIX 1003.1 access mode mask. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/mpool/mpool.h:68"]
pub mod mpool_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:16"]
    pub struct _bkt {
        pub hq: C2RustUnnamed_0,
        pub q: C2RustUnnamed,
        pub page: *mut libc::c_void,
        pub pgno: db_pgno_t,
        pub flags: u_int8_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "51:2"]
    pub struct C2RustUnnamed {
        pub tqe_next: *mut _bkt,
        pub tqe_prev: *mut *mut _bkt,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:2"]
    pub struct C2RustUnnamed_0 {
        pub tqe_next: *mut _bkt,
        pub tqe_prev: *mut *mut _bkt,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "61:16"]
    pub struct MPOOL {
        pub lqh: _lqh,
        pub hqh: [_hqh; 128],
        pub curcache: db_pgno_t,
        pub maxcache: db_pgno_t,
        pub npages: db_pgno_t,
        pub pagesize: u_long,
        pub fd: libc::c_int,
        pub pgin: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                              _: db_pgno_t,
                                              _: *mut libc::c_void) -> ()>,
        pub pgout: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                               _: db_pgno_t,
                                               _: *mut libc::c_void) -> ()>,
        pub pgcookie: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "64:13"]
    pub struct _hqh {
        pub tqh_first: *mut _bkt,
        pub tqh_last: *mut *mut _bkt,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:13"]
    pub struct _lqh {
        pub tqh_first: *mut _bkt,
        pub tqh_last: *mut *mut _bkt,
    }
    use super::db_int_h::db_pgno_t;
    use super::sys_types_h::{u_int8_t, u_long, u_int};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "108:1"]
        pub fn kdb2_mpool_new(_: *mut MPOOL, _: *mut db_pgno_t, _: u_int)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "111:1"]
        pub fn kdb2_mpool_put(_: *mut MPOOL, _: *mut libc::c_void, _: u_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "110:1"]
        pub fn kdb2_mpool_delete(_: *mut MPOOL, _: *mut libc::c_void)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "109:1"]
        pub fn kdb2_mpool_get(_: *mut MPOOL, _: db_pgno_t, _: u_int)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/hash/hash.h:68"]
pub mod hash_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "48:16"]
    pub struct cursor_t {
        pub queue: C2RustUnnamed_1,
        pub get: Option<unsafe extern "C" fn(_: *const DB, _: *mut cursor_t,
                                             _: *mut DBT, _: *mut DBT,
                                             _: u_int32_t) -> libc::c_int>,
        pub delete: Option<unsafe extern "C" fn(_: *const DB,
                                                _: *mut cursor_t,
                                                _: u_int32_t) -> libc::c_int>,
        pub bucket: db_pgno_t,
        pub pgno: db_pgno_t,
        pub ndx: indx_t,
        pub pgndx: indx_t,
        pub pagep: *mut u_int16_t,
        pub internal: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:2"]
    pub struct C2RustUnnamed_1 {
        pub tqe_next: *mut cursor_t,
        pub tqe_prev: *mut *mut cursor_t,
    }
    #[c2rust::src_loc = "48:1"]
    pub type CURSOR = cursor_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "66:16"]
    pub struct hashhdr {
        pub magic: int32_t,
        pub version: int32_t,
        pub lorder: int32_t,
        pub bsize: u_int32_t,
        pub bshift: int32_t,
        pub ovfl_point: int32_t,
        pub last_freed: u_int32_t,
        pub max_bucket: u_int32_t,
        pub high_mask: u_int32_t,
        pub low_mask: u_int32_t,
        pub ffactor: u_int32_t,
        pub nkeys: int32_t,
        pub hdrpages: u_int32_t,
        pub h_charkey: u_int32_t,
        pub spares: [u_int32_t; 32],
        pub bitmaps: [u_int16_t; 32],
    }
    #[c2rust::src_loc = "66:1"]
    pub type HASHHDR = hashhdr;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "86:16"]
    pub struct htab {
        pub curs_queue: _cursor_queue,
        pub hdr: HASHHDR,
        pub hash: Option<unsafe extern "C" fn(_: *const libc::c_void,
                                              _: size_t) -> u_int32_t>,
        pub flags: int32_t,
        pub fp: int32_t,
        pub fname: *const libc::c_char,
        pub bigdata_buf: *mut u_int8_t,
        pub bigkey_buf: *mut u_int8_t,
        pub split_buf: *mut u_int16_t,
        pub seq_cursor: *mut CURSOR,
        pub local_errno: int32_t,
        pub new_file: int32_t,
        pub save_file: int32_t,
        pub mapp: [*mut u_int32_t; 32],
        pub nmaps: int32_t,
        pub mp: *mut MPOOL,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "87:13"]
    pub struct _cursor_queue {
        pub tqh_first: *mut cursor_t,
        pub tqh_last: *mut *mut cursor_t,
    }
    #[c2rust::src_loc = "86:1"]
    pub type HTAB = htab;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "171:16"]
    pub struct item_info {
        pub pgno: db_pgno_t,
        pub bucket: db_pgno_t,
        pub ndx: indx_t,
        pub pgndx: indx_t,
        pub status: u_int8_t,
        pub seek_size: u_int32_t,
        pub seek_found_page: db_pgno_t,
        pub key_off: indx_t,
        pub data_off: indx_t,
        pub caused_expand: u_int8_t,
    }
    #[c2rust::src_loc = "171:1"]
    pub type ITEM_INFO = item_info;
    use super::db_h::{DB, DBT};
    use super::sys_types_h::{u_int32_t, u_int16_t, u_int8_t};
    use super::db_int_h::{db_pgno_t, indx_t};
    use super::stdint_intn_h::int32_t;
    use super::stddef_h::size_t;
    use super::mpool_h::MPOOL;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/hash/page.h:69"]
pub mod page_h {
    #[c2rust::src_loc = "150:1"]
    pub type PAGE16 = libc::c_ushort;
    #[c2rust::src_loc = "151:1"]
    pub type PAGE8 = libc::c_uchar;
}
#[c2rust::header_src = "/usr/include/string.h:64"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "46:14"]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:65"]
pub mod unistd_h {
    use super::stddef_h::size_t;
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "366:1"]
        pub fn write(__fd: libc::c_int, __buf: *const libc::c_void,
                     __n: size_t) -> ssize_t;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/hash/extern.h:70"]
pub mod extern_h {
    use super::hash_h::HTAB;
    use super::page_h::PAGE16;
    use super::db_int_h::indx_t;
    use super::stdint_intn_h::{int32_t, int8_t};
    use super::db_h::DBT;
    use super::sys_types_h::u_int32_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn __kdb2_big_delete(_: *mut HTAB, _: *mut PAGE16, _: indx_t)
         -> int32_t;
        #[no_mangle]
        #[c2rust::src_loc = "75:1"]
        pub fn __kdb2_big_insert(_: *mut HTAB, _: *mut PAGE16, _: *const DBT,
                                 _: *const DBT) -> int32_t;
        #[no_mangle]
        #[c2rust::src_loc = "78:1"]
        pub fn __kdb2_call_hash(_: *mut HTAB, _: *mut int8_t, _: int32_t)
         -> u_int32_t;
        #[no_mangle]
        #[c2rust::src_loc = "85:1"]
        pub fn __kdb2_get_bigkey(_: *mut HTAB, _: *mut PAGE16, _: indx_t,
                                 _: *mut DBT) -> int32_t;
        #[no_mangle]
        #[c2rust::src_loc = "94:1"]
        pub fn __kdb2_log2(_: u_int32_t) -> u_int32_t;
    }
}
pub use self::types_h::{__u_int, __u_long, __int8_t, __uint8_t, __int16_t,
                        __uint16_t, __int32_t, __uint32_t, __ssize_t};
pub use self::sys_types_h::{u_int, u_long, ssize_t, u_int8_t, u_int16_t,
                            u_int32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int8_t, int16_t, int32_t};
pub use self::db_h::{DBT, DBTYPE, DB_RECNO, DB_HASH, DB_BTREE, __db, DB};
pub use self::db_int_h::{db_pgno_t, indx_t};
pub use self::mpool_h::{_bkt, C2RustUnnamed, C2RustUnnamed_0, MPOOL, _hqh,
                        _lqh, kdb2_mpool_new, kdb2_mpool_put,
                        kdb2_mpool_delete, kdb2_mpool_get};
pub use self::hash_h::{cursor_t, C2RustUnnamed_1, CURSOR, hashhdr, HASHHDR,
                       htab, _cursor_queue, HTAB, item_info, ITEM_INFO};
pub use self::page_h::{PAGE16, PAGE8};
use self::string_h::{memcpy, memmove, memset};
use self::unistd_h::write;
use self::extern_h::{__kdb2_big_delete, __kdb2_big_insert, __kdb2_call_hash,
                     __kdb2_get_bigkey, __kdb2_log2};
#[no_mangle]
#[c2rust::src_loc = "87:1"]
pub unsafe extern "C" fn __kdb2_get_item(mut hashp: *mut HTAB,
                                         mut cursorp: *mut CURSOR,
                                         mut key: *mut DBT, mut val: *mut DBT,
                                         mut item_info: *mut ITEM_INFO)
 -> u_int32_t {
    let mut next_pgno: db_pgno_t = 0;
    let mut i: int32_t = 0;
    /* Check if we need to get a page. */
    if (*cursorp).pagep.is_null() {
        if (*cursorp).pgno == 0xffffffff as libc::c_uint {
            (*cursorp).pagep =
                __kdb2_get_page(hashp, (*cursorp).bucket, 0 as libc::c_int);
            (*cursorp).pgno =
                *(((*cursorp).pagep as *mut libc::c_void as
                       *mut u_int8_t).offset(0 as libc::c_int as isize) as
                      *mut libc::c_void as
                      *mut db_pgno_t).offset(0 as libc::c_int as isize);
            (*cursorp).ndx = 0 as libc::c_int as indx_t;
            (*cursorp).pgndx = 0 as libc::c_int as indx_t
        } else {
            (*cursorp).pagep =
                __kdb2_get_page(hashp, (*cursorp).pgno, 4 as libc::c_int)
        }
        if (*cursorp).pagep.is_null() {
            (*item_info).status = 0 as libc::c_int as u_int8_t;
            return -(1 as libc::c_int) as u_int32_t
        }
    }
    if (*item_info).seek_size != 0 &&
           ((*(((*cursorp).pagep as *mut libc::c_void as
                    *mut u_int8_t).offset(12 as libc::c_int as isize) as
                   *mut libc::c_void as
                   *mut indx_t).offset(0 as libc::c_int as isize) as
                 libc::c_int + 1 as libc::c_int) as
                libc::c_ulong).wrapping_sub((12 as libc::c_int as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                 as
                                                                                 libc::c_ulong)).wrapping_sub((*(((*cursorp).pagep
                                                                                                                      as
                                                                                                                      *mut libc::c_void
                                                                                                                      as
                                                                                                                      *mut u_int8_t).offset(8
                                                                                                                                                as
                                                                                                                                                libc::c_int
                                                                                                                                                as
                                                                                                                                                isize)
                                                                                                                     as
                                                                                                                     *mut libc::c_void
                                                                                                                     as
                                                                                                                     *mut indx_t).offset(0
                                                                                                                                             as
                                                                                                                                             libc::c_int
                                                                                                                                             as
                                                                                                                                             isize)
                                                                                                                   as
                                                                                                                   libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                                                                    as
                                                                                                                                                    libc::c_ulong)
                                                                                                                                                   <<
                                                                                                                                                   1
                                                                                                                                                       as
                                                                                                                                                       libc::c_int))
               > (*item_info).seek_size as libc::c_ulong {
        (*item_info).seek_found_page = (*cursorp).pgno
    }
    if (*cursorp).pgndx as libc::c_int ==
           *(((*cursorp).pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(8 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut indx_t).offset(0 as libc::c_int as isize) as libc::c_int
       {
        /* Fetch next page. */
        if *(((*cursorp).pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(4 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut db_pgno_t).offset(0 as libc::c_int as isize) ==
               0xffffffff as libc::c_uint {
            (*item_info).status = 2 as libc::c_int as u_int8_t;
            return -(1 as libc::c_int) as u_int32_t
        }
        next_pgno =
            *(((*cursorp).pagep as *mut libc::c_void as
                   *mut u_int8_t).offset(4 as libc::c_int as isize) as
                  *mut libc::c_void as
                  *mut db_pgno_t).offset(0 as libc::c_int as isize);
        (*cursorp).pgndx = 0 as libc::c_int as indx_t;
        __kdb2_put_page(hashp, (*cursorp).pagep, 4 as libc::c_int,
                        0 as libc::c_int);
        (*cursorp).pagep =
            __kdb2_get_page(hashp, next_pgno, 4 as libc::c_int);
        if (*cursorp).pagep.is_null() {
            (*item_info).status = 0 as libc::c_int as u_int8_t;
            return -(1 as libc::c_int) as u_int32_t
        }
        (*cursorp).pgno = next_pgno
    }
    if *(((*cursorp).pagep as *mut libc::c_void as
              *mut u_int8_t).offset((12 as libc::c_int as
                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                         as
                                                                         libc::c_ulong)
                                        as
                                        isize).offset(((*cursorp).pgndx as
                                                           libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                            as
                                                                                            libc::c_ulong)
                                                                                           <<
                                                                                           1
                                                                                               as
                                                                                               libc::c_int)
                                                          as isize) as
             *mut libc::c_void as
             *mut indx_t).offset(0 as libc::c_int as isize) as libc::c_int !=
           0 as libc::c_int {
        i = prev_realkey((*cursorp).pagep, (*cursorp).pgndx) as int32_t;
        if i == (*cursorp).pgndx as libc::c_int {
            (*key).size =
                (*hashp).hdr.bsize.wrapping_sub(*(((*cursorp).pagep as
                                                       *mut libc::c_void as
                                                       *mut u_int8_t).offset((12
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                  as
                                                                                                                  libc::c_ulong)
                                                                                 as
                                                                                 isize).offset(((*cursorp).pgndx
                                                                                                    as
                                                                                                    libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                                                     as
                                                                                                                                     libc::c_ulong)
                                                                                                                                    <<
                                                                                                                                    1
                                                                                                                                        as
                                                                                                                                        libc::c_int)
                                                                                                   as
                                                                                                   isize)
                                                      as *mut libc::c_void as
                                                      *mut indx_t).offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)
                                                    as libc::c_uint) as size_t
        } else {
            (*key).size =
                (*(((*cursorp).pagep as *mut libc::c_void as
                        *mut u_int8_t).offset((12 as libc::c_int as
                                                   libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                   as
                                                                                   libc::c_ulong)
                                                  as
                                                  isize).offset((i as
                                                                     libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                      as
                                                                                                      libc::c_ulong)
                                                                                                     <<
                                                                                                     1
                                                                                                         as
                                                                                                         libc::c_int)
                                                                    as
                                                                    isize).offset(::std::mem::size_of::<indx_t>()
                                                                                      as
                                                                                      libc::c_ulong
                                                                                      as
                                                                                      isize)
                       as *mut libc::c_void as
                       *mut indx_t).offset(0 as libc::c_int as isize) as
                     libc::c_int -
                     *(((*cursorp).pagep as *mut libc::c_void as
                            *mut u_int8_t).offset((12 as libc::c_int as
                                                       libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                       as
                                                                                       libc::c_ulong)
                                                      as
                                                      isize).offset(((*cursorp).pgndx
                                                                         as
                                                                         libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                          as
                                                                                                          libc::c_ulong)
                                                                                                         <<
                                                                                                         1
                                                                                                             as
                                                                                                             libc::c_int)
                                                                        as
                                                                        isize)
                           as *mut libc::c_void as
                           *mut indx_t).offset(0 as libc::c_int as isize) as
                         libc::c_int) as size_t
        }
    }
    /*
	 * All of this information will be set incorrectly for big keys, but
	 * it will be ignored anyway.
	 */
    (*val).size =
        (*(((*cursorp).pagep as *mut libc::c_void as
                *mut u_int8_t).offset((12 as libc::c_int as
                                           libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                           as
                                                                           libc::c_ulong)
                                          as
                                          isize).offset(((*cursorp).pgndx as
                                                             libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                              as
                                                                                              libc::c_ulong)
                                                                                             <<
                                                                                             1
                                                                                                 as
                                                                                                 libc::c_int)
                                                            as isize) as
               *mut libc::c_void as
               *mut indx_t).offset(0 as libc::c_int as isize) as libc::c_int -
             *(((*cursorp).pagep as *mut libc::c_void as
                    *mut u_int8_t).offset((12 as libc::c_int as
                                               libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                               as
                                                                               libc::c_ulong)
                                              as
                                              isize).offset(((*cursorp).pgndx
                                                                 as
                                                                 libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                  as
                                                                                                  libc::c_ulong)
                                                                                                 <<
                                                                                                 1
                                                                                                     as
                                                                                                     libc::c_int)
                                                                as
                                                                isize).offset(::std::mem::size_of::<indx_t>()
                                                                                  as
                                                                                  libc::c_ulong
                                                                                  as
                                                                                  isize)
                   as *mut libc::c_void as
                   *mut indx_t).offset(0 as libc::c_int as isize) as
                 libc::c_int) as size_t;
    (*key).data =
        ((*cursorp).pagep as
             *mut PAGE8).offset(*(((*cursorp).pagep as *mut libc::c_void as
                                       *mut u_int8_t).offset((12 as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                  as
                                                                                                  libc::c_ulong)
                                                                 as
                                                                 isize).offset(((*cursorp).pgndx
                                                                                    as
                                                                                    libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                                     as
                                                                                                                     libc::c_ulong)
                                                                                                                    <<
                                                                                                                    1
                                                                                                                        as
                                                                                                                        libc::c_int)
                                                                                   as
                                                                                   isize)
                                      as *mut libc::c_void as
                                      *mut indx_t).offset(0 as libc::c_int as
                                                              isize) as
                                    libc::c_int as isize) as
            *mut libc::c_void;
    (*val).data =
        ((*cursorp).pagep as
             *mut PAGE8).offset(*(((*cursorp).pagep as *mut libc::c_void as
                                       *mut u_int8_t).offset((12 as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                  as
                                                                                                  libc::c_ulong)
                                                                 as
                                                                 isize).offset(((*cursorp).pgndx
                                                                                    as
                                                                                    libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                                     as
                                                                                                                     libc::c_ulong)
                                                                                                                    <<
                                                                                                                    1
                                                                                                                        as
                                                                                                                        libc::c_int)
                                                                                   as
                                                                                   isize).offset(::std::mem::size_of::<indx_t>()
                                                                                                     as
                                                                                                     libc::c_ulong
                                                                                                     as
                                                                                                     isize)
                                      as *mut libc::c_void as
                                      *mut indx_t).offset(0 as libc::c_int as
                                                              isize) as
                                    libc::c_int as isize) as
            *mut libc::c_void;
    (*item_info).pgno = (*cursorp).pgno;
    (*item_info).bucket = (*cursorp).bucket;
    (*item_info).ndx = (*cursorp).ndx;
    (*item_info).pgndx = (*cursorp).pgndx;
    (*item_info).key_off =
        *(((*cursorp).pagep as *mut libc::c_void as
               *mut u_int8_t).offset((12 as libc::c_int as
                                          libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                          as
                                                                          libc::c_ulong)
                                         as
                                         isize).offset(((*cursorp).pgndx as
                                                            libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                             as
                                                                                             libc::c_ulong)
                                                                                            <<
                                                                                            1
                                                                                                as
                                                                                                libc::c_int)
                                                           as isize) as
              *mut libc::c_void as
              *mut indx_t).offset(0 as libc::c_int as isize);
    (*item_info).data_off =
        *(((*cursorp).pagep as *mut libc::c_void as
               *mut u_int8_t).offset((12 as libc::c_int as
                                          libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                          as
                                                                          libc::c_ulong)
                                         as
                                         isize).offset(((*cursorp).pgndx as
                                                            libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                             as
                                                                                             libc::c_ulong)
                                                                                            <<
                                                                                            1
                                                                                                as
                                                                                                libc::c_int)
                                                           as
                                                           isize).offset(::std::mem::size_of::<indx_t>()
                                                                             as
                                                                             libc::c_ulong
                                                                             as
                                                                             isize)
              as *mut libc::c_void as
              *mut indx_t).offset(0 as libc::c_int as isize);
    (*item_info).status = 1 as libc::c_int as u_int8_t;
    return 0 as libc::c_int as u_int32_t;
}
#[no_mangle]
#[c2rust::src_loc = "162:1"]
pub unsafe extern "C" fn __kdb2_get_item_reset(mut hashp: *mut HTAB,
                                               mut cursorp: *mut CURSOR)
 -> u_int32_t {
    if !(*cursorp).pagep.is_null() {
        __kdb2_put_page(hashp, (*cursorp).pagep, 4 as libc::c_int,
                        0 as libc::c_int);
    }
    (*cursorp).pagep = 0 as *mut u_int16_t;
    (*cursorp).bucket = -(1 as libc::c_int) as db_pgno_t;
    (*cursorp).ndx = 0 as libc::c_int as indx_t;
    (*cursorp).pgndx = 0 as libc::c_int as indx_t;
    (*cursorp).pgno = 0xffffffff as libc::c_uint;
    return 0 as libc::c_int as u_int32_t;
}
#[no_mangle]
#[c2rust::src_loc = "177:1"]
pub unsafe extern "C" fn __kdb2_get_item_done(mut hashp: *mut HTAB,
                                              mut cursorp: *mut CURSOR)
 -> u_int32_t {
    if !(*cursorp).pagep.is_null() {
        __kdb2_put_page(hashp, (*cursorp).pagep, 4 as libc::c_int,
                        0 as libc::c_int);
    }
    (*cursorp).pagep = 0 as *mut u_int16_t;
    /*
	 * We don't throw out the page number since we might want to
	 * continue getting on this page.
	 */
    return 0 as libc::c_int as u_int32_t;
}
#[no_mangle]
#[c2rust::src_loc = "193:1"]
pub unsafe extern "C" fn __kdb2_get_item_first(mut hashp: *mut HTAB,
                                               mut cursorp: *mut CURSOR,
                                               mut key: *mut DBT,
                                               mut val: *mut DBT,
                                               mut item_info: *mut ITEM_INFO)
 -> u_int32_t {
    __kdb2_get_item_reset(hashp, cursorp);
    (*cursorp).bucket = 0 as libc::c_int as db_pgno_t;
    return __kdb2_get_item_next(hashp, cursorp, key, val, item_info);
}
/*
 * Returns a pointer to key/data pair on a page.  In the case of bigkeys,
 * just returns the page number and index of the bigkey pointer pair.
 */
#[no_mangle]
#[c2rust::src_loc = "209:1"]
pub unsafe extern "C" fn __kdb2_get_item_next(mut hashp: *mut HTAB,
                                              mut cursorp: *mut CURSOR,
                                              mut key: *mut DBT,
                                              mut val: *mut DBT,
                                              mut item_info: *mut ITEM_INFO)
 -> u_int32_t {
    let mut status: libc::c_int = 0;
    status =
        __kdb2_get_item(hashp, cursorp, key, val, item_info) as libc::c_int;
    (*cursorp).ndx = (*cursorp).ndx.wrapping_add(1);
    (*cursorp).pgndx = (*cursorp).pgndx.wrapping_add(1);
    return status as u_int32_t;
}
/*
 * Put a non-big pair on a page.
 */
#[c2rust::src_loc = "227:1"]
unsafe extern "C" fn putpair(mut p: *mut PAGE8, mut key: *const DBT,
                             mut val: *const DBT) {
    let mut pagep: *mut u_int16_t = 0 as *mut u_int16_t;
    let mut n: u_int16_t = 0;
    let mut off: u_int16_t = 0;
    pagep = p as *mut libc::c_void as *mut PAGE16;
    /* Items on the page are 0-indexed. */
    n =
        *((pagep as *mut libc::c_void as
               *mut u_int8_t).offset(8 as libc::c_int as isize) as
              *mut libc::c_void as
              *mut indx_t).offset(0 as libc::c_int as isize);
    off =
        (*((pagep as *mut libc::c_void as
                *mut u_int8_t).offset(12 as libc::c_int as isize) as
               *mut libc::c_void as
               *mut indx_t).offset(0 as libc::c_int as isize) as
             libc::c_ulong).wrapping_sub((*key).size).wrapping_add(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulong)
            as u_int16_t;
    memmove(p.offset(off as libc::c_int as isize) as *mut libc::c_void,
            (*key).data, (*key).size);
    *((pagep as *mut libc::c_void as
           *mut u_int8_t).offset((12 as libc::c_int as
                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                      as
                                                                      libc::c_ulong)
                                     as
                                     isize).offset((n as
                                                        libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                         as
                                                                                         libc::c_ulong)
                                                                                        <<
                                                                                        1
                                                                                            as
                                                                                            libc::c_int)
                                                       as isize) as
          *mut libc::c_void as *mut indx_t).offset(0 as libc::c_int as isize)
        = off;
    off =
        (off as libc::c_ulong).wrapping_sub((*val).size) as u_int16_t as
            u_int16_t;
    memmove(p.offset(off as libc::c_int as isize) as *mut libc::c_void,
            (*val).data, (*val).size);
    *((pagep as *mut libc::c_void as
           *mut u_int8_t).offset((12 as libc::c_int as
                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                      as
                                                                      libc::c_ulong)
                                     as
                                     isize).offset((n as
                                                        libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                         as
                                                                                         libc::c_ulong)
                                                                                        <<
                                                                                        1
                                                                                            as
                                                                                            libc::c_int)
                                                       as
                                                       isize).offset(::std::mem::size_of::<indx_t>()
                                                                         as
                                                                         libc::c_ulong
                                                                         as
                                                                         isize)
          as *mut libc::c_void as
          *mut indx_t).offset(0 as libc::c_int as isize) = off;
    /* Adjust page info. */
    *((pagep as *mut libc::c_void as
           *mut u_int8_t).offset(8 as libc::c_int as isize) as
          *mut libc::c_void as *mut indx_t).offset(0 as libc::c_int as isize)
        = (n as libc::c_int + 1 as libc::c_int) as indx_t;
    *((pagep as *mut libc::c_void as
           *mut u_int8_t).offset(12 as libc::c_int as isize) as
          *mut libc::c_void as *mut indx_t).offset(0 as libc::c_int as isize)
        = (off as libc::c_int - 1 as libc::c_int) as indx_t;
}
/*
 * Returns the index of the next non-bigkey pair after n on the page.
 * Returns -1 if there are no more non-big things on the page.
 */
#[c2rust::src_loc = "255:1"]
unsafe extern "C" fn next_realkey(mut pagep: *mut PAGE16, mut n: indx_t)
 -> indx_t {
    let mut i: indx_t = 0;
    i = (n as libc::c_int + 1 as libc::c_int) as indx_t;
    while (i as libc::c_int) <
              *((pagep as *mut libc::c_void as
                     *mut u_int8_t).offset(8 as libc::c_int as isize) as
                    *mut libc::c_void as
                    *mut indx_t).offset(0 as libc::c_int as isize) as
                  libc::c_int {
        if *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset((12 as libc::c_int as
                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                             as
                                                                             libc::c_ulong)
                                            as
                                            isize).offset((i as
                                                               libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                as
                                                                                                libc::c_ulong)
                                                                                               <<
                                                                                               1
                                                                                                   as
                                                                                                   libc::c_int)
                                                              as isize) as
                 *mut libc::c_void as
                 *mut indx_t).offset(0 as libc::c_int as isize) as libc::c_int
               != 0 as libc::c_int {
            return i
        }
        i = i.wrapping_add(1)
    }
    return -(1 as libc::c_int) as indx_t;
}
/*
 * Returns the index of the previous non-bigkey pair after n on the page.
 * Returns n if there are no previous non-big things on the page.
 */
#[c2rust::src_loc = "276:1"]
unsafe extern "C" fn prev_realkey(mut pagep: *mut PAGE16, mut n: indx_t)
 -> indx_t {
    let mut i: int32_t = 0;
    /* Need a signed value to do the compare properly. */
    i = n as libc::c_int - 1 as libc::c_int;
    while i > -(1 as libc::c_int) {
        if *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset((12 as libc::c_int as
                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                             as
                                                                             libc::c_ulong)
                                            as
                                            isize).offset((i as
                                                               libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                as
                                                                                                libc::c_ulong)
                                                                                               <<
                                                                                               1
                                                                                                   as
                                                                                                   libc::c_int)
                                                              as isize) as
                 *mut libc::c_void as
                 *mut indx_t).offset(0 as libc::c_int as isize) as libc::c_int
               != 0 as libc::c_int {
            return i as indx_t
        }
        i -= 1
    }
    return n;
}
/*
 * Returns:
 *       0 OK
 *      -1 error
 */
#[no_mangle]
#[c2rust::src_loc = "299:1"]
pub unsafe extern "C" fn __kdb2_delpair(mut hashp: *mut HTAB,
                                        mut cursorp: *mut CURSOR,
                                        mut item_info: *mut ITEM_INFO)
 -> int32_t {
    let mut pagep: *mut PAGE16 = 0 as *mut PAGE16;
    let mut ndx: indx_t = 0;
    let mut check_ndx: libc::c_short = 0;
    let mut delta: int16_t = 0;
    let mut len: int16_t = 0;
    let mut next_key: int16_t = 0;
    let mut n: int32_t = 0;
    let mut src: *mut u_int8_t = 0 as *mut u_int8_t;
    let mut dest: *mut u_int8_t = 0 as *mut u_int8_t;
    ndx = (*cursorp).pgndx;
    if (*cursorp).pagep.is_null() {
        pagep = __kdb2_get_page(hashp, (*cursorp).pgno, 4 as libc::c_int);
        if pagep.is_null() { return -(1 as libc::c_int) }
        /*
		 * KLUGE: pgndx has gone one too far, because cursor points
		 * to the _next_ item.  Use pgndx - 1.
		 */
        ndx = ndx.wrapping_sub(1)
    } else { pagep = (*cursorp).pagep }
    if *((pagep as *mut libc::c_void as
              *mut u_int8_t).offset((12 as libc::c_int as
                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                         as
                                                                         libc::c_ulong)
                                        as
                                        isize).offset((ndx as
                                                           libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                            as
                                                                                            libc::c_ulong)
                                                                                           <<
                                                                                           1
                                                                                               as
                                                                                               libc::c_int)
                                                          as isize) as
             *mut libc::c_void as
             *mut indx_t).offset(0 as libc::c_int as isize) as libc::c_int ==
           0 as libc::c_int {
        delta = 0 as libc::c_int as int16_t;
        __kdb2_big_delete(hashp, pagep, ndx);
    } else {
        /*
		 * Compute "delta", the amount we have to shift all of the
		 * offsets.  To find the delta, we need to make sure that
		 * we aren't looking at the DATA_OFF of a big/keydata pair.
		 */
        check_ndx = (ndx as libc::c_int - 1 as libc::c_int) as libc::c_short;
        while check_ndx as libc::c_int >= 0 as libc::c_int &&
                  *((pagep as *mut libc::c_void as
                         *mut u_int8_t).offset((12 as libc::c_int as
                                                    libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                    as
                                                                                    libc::c_ulong)
                                                   as
                                                   isize).offset((check_ndx as
                                                                      libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                       as
                                                                                                       libc::c_ulong)
                                                                                                      <<
                                                                                                      1
                                                                                                          as
                                                                                                          libc::c_int)
                                                                     as isize)
                        as *mut libc::c_void as
                        *mut indx_t).offset(0 as libc::c_int as isize) as
                      libc::c_int == 0 as libc::c_int {
            check_ndx -= 1
        }
        if (check_ndx as libc::c_int) < 0 as libc::c_int {
            delta =
                (*hashp).hdr.bsize.wrapping_sub(*((pagep as *mut libc::c_void
                                                       as
                                                       *mut u_int8_t).offset((12
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                  as
                                                                                                                  libc::c_ulong)
                                                                                 as
                                                                                 isize).offset((ndx
                                                                                                    as
                                                                                                    libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                                                     as
                                                                                                                                     libc::c_ulong)
                                                                                                                                    <<
                                                                                                                                    1
                                                                                                                                        as
                                                                                                                                        libc::c_int)
                                                                                                   as
                                                                                                   isize).offset(::std::mem::size_of::<indx_t>()
                                                                                                                     as
                                                                                                                     libc::c_ulong
                                                                                                                     as
                                                                                                                     isize)
                                                      as *mut libc::c_void as
                                                      *mut indx_t).offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)
                                                    as libc::c_uint) as
                    int16_t
        } else {
            delta =
                (*((pagep as *mut libc::c_void as
                        *mut u_int8_t).offset((12 as libc::c_int as
                                                   libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                   as
                                                                                   libc::c_ulong)
                                                  as
                                                  isize).offset((check_ndx as
                                                                     libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                      as
                                                                                                      libc::c_ulong)
                                                                                                     <<
                                                                                                     1
                                                                                                         as
                                                                                                         libc::c_int)
                                                                    as
                                                                    isize).offset(::std::mem::size_of::<indx_t>()
                                                                                      as
                                                                                      libc::c_ulong
                                                                                      as
                                                                                      isize)
                       as *mut libc::c_void as
                       *mut indx_t).offset(0 as libc::c_int as isize) as
                     libc::c_int -
                     *((pagep as *mut libc::c_void as
                            *mut u_int8_t).offset((12 as libc::c_int as
                                                       libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                       as
                                                                                       libc::c_ulong)
                                                      as
                                                      isize).offset((ndx as
                                                                         libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                          as
                                                                                                          libc::c_ulong)
                                                                                                         <<
                                                                                                         1
                                                                                                             as
                                                                                                             libc::c_int)
                                                                        as
                                                                        isize).offset(::std::mem::size_of::<indx_t>()
                                                                                          as
                                                                                          libc::c_ulong
                                                                                          as
                                                                                          isize)
                           as *mut libc::c_void as
                           *mut indx_t).offset(0 as libc::c_int as isize) as
                         libc::c_int) as int16_t
        }
        /*
		 * The hard case: we want to remove something other than
		 * the last item on the page.  We need to shift data and
		 * offsets down.
		 */
        if ndx as libc::c_int !=
               *((pagep as *mut libc::c_void as
                      *mut u_int8_t).offset(8 as libc::c_int as isize) as
                     *mut libc::c_void as
                     *mut indx_t).offset(0 as libc::c_int as isize) as
                   libc::c_int - 1 as libc::c_int {
            /*
			 * Move the data: src is the address of the last data
			 * item on the page.
			 */
            src =
                (pagep as
                     *mut u_int8_t).offset(*((pagep as *mut libc::c_void as
                                                  *mut u_int8_t).offset(12 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                                 as *mut libc::c_void as
                                                 *mut indx_t).offset(0 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                               as libc::c_int as
                                               isize).offset(1 as libc::c_int
                                                                 as isize);
            /*
			 * Length is the distance between where to start
			 * deleting and end of the data on the page.
			 */
            len =
                (*((pagep as *mut libc::c_void as
                        *mut u_int8_t).offset((12 as libc::c_int as
                                                   libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                   as
                                                                                   libc::c_ulong)
                                                  as
                                                  isize).offset((ndx as
                                                                     libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                      as
                                                                                                      libc::c_ulong)
                                                                                                     <<
                                                                                                     1
                                                                                                         as
                                                                                                         libc::c_int)
                                                                    as
                                                                    isize).offset(::std::mem::size_of::<indx_t>()
                                                                                      as
                                                                                      libc::c_ulong
                                                                                      as
                                                                                      isize)
                       as *mut libc::c_void as
                       *mut indx_t).offset(0 as libc::c_int as isize) as
                     libc::c_int -
                     (*((pagep as *mut libc::c_void as
                             *mut u_int8_t).offset(12 as libc::c_int as isize)
                            as *mut libc::c_void as
                            *mut indx_t).offset(0 as libc::c_int as isize) as
                          libc::c_int + 1 as libc::c_int)) as int16_t;
            /*
			 * Dest is the location of the to-be-deleted item
			 * occupied - length.
			 */
            if (check_ndx as libc::c_int) < 0 as libc::c_int {
                dest =
                    (pagep as
                         *mut u_int8_t).offset((*hashp).hdr.bsize as
                                                   isize).offset(-(len as
                                                                       libc::c_int
                                                                       as
                                                                       isize))
            } else {
                dest =
                    (pagep as
                         *mut u_int8_t).offset(*((pagep as *mut libc::c_void
                                                      as
                                                      *mut u_int8_t).offset((12
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                 as
                                                                                                                 libc::c_ulong)
                                                                                as
                                                                                isize).offset((check_ndx
                                                                                                   as
                                                                                                   libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                                                    as
                                                                                                                                    libc::c_ulong)
                                                                                                                                   <<
                                                                                                                                   1
                                                                                                                                       as
                                                                                                                                       libc::c_int)
                                                                                                  as
                                                                                                  isize).offset(::std::mem::size_of::<indx_t>()
                                                                                                                    as
                                                                                                                    libc::c_ulong
                                                                                                                    as
                                                                                                                    isize)
                                                     as *mut libc::c_void as
                                                     *mut indx_t).offset(0 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                                                   as libc::c_int as
                                                   isize).offset(-(len as
                                                                       libc::c_int
                                                                       as
                                                                       isize))
            }
            memmove(dest as *mut libc::c_void, src as *const libc::c_void,
                    len as libc::c_ulong);
        }
    }
    /* Adjust the offsets. */
    n = ndx as int32_t;
    while n <
              *((pagep as *mut libc::c_void as
                     *mut u_int8_t).offset(8 as libc::c_int as isize) as
                    *mut libc::c_void as
                    *mut indx_t).offset(0 as libc::c_int as isize) as
                  libc::c_int - 1 as libc::c_int {
        if *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset((12 as libc::c_int as
                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                             as
                                                                             libc::c_ulong)
                                            as
                                            isize).offset(((n +
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                as
                                                                                                libc::c_ulong)
                                                                                               <<
                                                                                               1
                                                                                                   as
                                                                                                   libc::c_int)
                                                              as isize) as
                 *mut libc::c_void as
                 *mut indx_t).offset(0 as libc::c_int as isize) as libc::c_int
               != 0 as libc::c_int {
            next_key = next_realkey(pagep, n as indx_t) as int16_t;
            *((pagep as *mut libc::c_void as
                   *mut u_int8_t).offset((12 as libc::c_int as
                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                              as
                                                                              libc::c_ulong)
                                             as
                                             isize).offset((n as
                                                                libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                 as
                                                                                                 libc::c_ulong)
                                                                                                <<
                                                                                                1
                                                                                                    as
                                                                                                    libc::c_int)
                                                               as isize) as
                  *mut libc::c_void as
                  *mut indx_t).offset(0 as libc::c_int as isize) =
                (*((pagep as *mut libc::c_void as
                        *mut u_int8_t).offset((12 as libc::c_int as
                                                   libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                   as
                                                                                   libc::c_ulong)
                                                  as
                                                  isize).offset(((n +
                                                                      1 as
                                                                          libc::c_int)
                                                                     as
                                                                     libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                      as
                                                                                                      libc::c_ulong)
                                                                                                     <<
                                                                                                     1
                                                                                                         as
                                                                                                         libc::c_int)
                                                                    as isize)
                       as *mut libc::c_void as
                       *mut indx_t).offset(0 as libc::c_int as isize) as
                     libc::c_int + delta as libc::c_int) as indx_t;
            *((pagep as *mut libc::c_void as
                   *mut u_int8_t).offset((12 as libc::c_int as
                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                              as
                                                                              libc::c_ulong)
                                             as
                                             isize).offset((n as
                                                                libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                 as
                                                                                                 libc::c_ulong)
                                                                                                <<
                                                                                                1
                                                                                                    as
                                                                                                    libc::c_int)
                                                               as
                                                               isize).offset(::std::mem::size_of::<indx_t>()
                                                                                 as
                                                                                 libc::c_ulong
                                                                                 as
                                                                                 isize)
                  as *mut libc::c_void as
                  *mut indx_t).offset(0 as libc::c_int as isize) =
                (*((pagep as *mut libc::c_void as
                        *mut u_int8_t).offset((12 as libc::c_int as
                                                   libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                   as
                                                                                   libc::c_ulong)
                                                  as
                                                  isize).offset(((n +
                                                                      1 as
                                                                          libc::c_int)
                                                                     as
                                                                     libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                      as
                                                                                                      libc::c_ulong)
                                                                                                     <<
                                                                                                     1
                                                                                                         as
                                                                                                         libc::c_int)
                                                                    as
                                                                    isize).offset(::std::mem::size_of::<indx_t>()
                                                                                      as
                                                                                      libc::c_ulong
                                                                                      as
                                                                                      isize)
                       as *mut libc::c_void as
                       *mut indx_t).offset(0 as libc::c_int as isize) as
                     libc::c_int + delta as libc::c_int) as indx_t
        } else {
            *((pagep as *mut libc::c_void as
                   *mut u_int8_t).offset((12 as libc::c_int as
                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                              as
                                                                              libc::c_ulong)
                                             as
                                             isize).offset((n as
                                                                libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                 as
                                                                                                 libc::c_ulong)
                                                                                                <<
                                                                                                1
                                                                                                    as
                                                                                                    libc::c_int)
                                                               as isize) as
                  *mut libc::c_void as
                  *mut indx_t).offset(0 as libc::c_int as isize) =
                *((pagep as *mut libc::c_void as
                       *mut u_int8_t).offset((12 as libc::c_int as
                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                  as
                                                                                  libc::c_ulong)
                                                 as
                                                 isize).offset(((n +
                                                                     1 as
                                                                         libc::c_int)
                                                                    as
                                                                    libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                     as
                                                                                                     libc::c_ulong)
                                                                                                    <<
                                                                                                    1
                                                                                                        as
                                                                                                        libc::c_int)
                                                                   as isize)
                      as *mut libc::c_void as
                      *mut indx_t).offset(0 as libc::c_int as isize);
            *((pagep as *mut libc::c_void as
                   *mut u_int8_t).offset((12 as libc::c_int as
                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                              as
                                                                              libc::c_ulong)
                                             as
                                             isize).offset((n as
                                                                libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                 as
                                                                                                 libc::c_ulong)
                                                                                                <<
                                                                                                1
                                                                                                    as
                                                                                                    libc::c_int)
                                                               as
                                                               isize).offset(::std::mem::size_of::<indx_t>()
                                                                                 as
                                                                                 libc::c_ulong
                                                                                 as
                                                                                 isize)
                  as *mut libc::c_void as
                  *mut indx_t).offset(0 as libc::c_int as isize) =
                *((pagep as *mut libc::c_void as
                       *mut u_int8_t).offset((12 as libc::c_int as
                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                  as
                                                                                  libc::c_ulong)
                                                 as
                                                 isize).offset(((n +
                                                                     1 as
                                                                         libc::c_int)
                                                                    as
                                                                    libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                     as
                                                                                                     libc::c_ulong)
                                                                                                    <<
                                                                                                    1
                                                                                                        as
                                                                                                        libc::c_int)
                                                                   as
                                                                   isize).offset(::std::mem::size_of::<indx_t>()
                                                                                     as
                                                                                     libc::c_ulong
                                                                                     as
                                                                                     isize)
                      as *mut libc::c_void as
                      *mut indx_t).offset(0 as libc::c_int as isize)
        }
        n += 1
    }
    /* Adjust page metadata. */
    *((pagep as *mut libc::c_void as
           *mut u_int8_t).offset(12 as libc::c_int as isize) as
          *mut libc::c_void as *mut indx_t).offset(0 as libc::c_int as isize)
        =
        (*((pagep as *mut libc::c_void as
                *mut u_int8_t).offset(12 as libc::c_int as isize) as
               *mut libc::c_void as
               *mut indx_t).offset(0 as libc::c_int as isize) as libc::c_int +
             delta as libc::c_int) as indx_t;
    *((pagep as *mut libc::c_void as
           *mut u_int8_t).offset(8 as libc::c_int as isize) as
          *mut libc::c_void as *mut indx_t).offset(0 as libc::c_int as isize)
        =
        (*((pagep as *mut libc::c_void as
                *mut u_int8_t).offset(8 as libc::c_int as isize) as
               *mut libc::c_void as
               *mut indx_t).offset(0 as libc::c_int as isize) as libc::c_int -
             1 as libc::c_int) as indx_t;
    (*hashp).hdr.nkeys -= 1;
    /* Is this page now an empty overflow page?  If so, free it. */
    if *((pagep as *mut libc::c_void as
              *mut u_int8_t).offset(10 as libc::c_int as isize) as
             *mut libc::c_void as
             *mut u_int8_t).offset(0 as libc::c_int as isize) as libc::c_int
           == 4 as libc::c_int &&
           *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(8 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut indx_t).offset(0 as libc::c_int as isize) as libc::c_int
               == 0 as libc::c_int {
        let mut empty_page: *mut PAGE16 = 0 as *mut PAGE16;
        let mut to_find: db_pgno_t = 0;
        let mut next_pgno: db_pgno_t = 0;
        let mut link_page: db_pgno_t = 0;
        /*
		 * We need to go back to the first page in the chain and
		 * look for this page so that we can update the previous
		 * page's NEXT_PGNO field.
		 */
        to_find =
            *((pagep as *mut libc::c_void as
                   *mut u_int8_t).offset(0 as libc::c_int as isize) as
                  *mut libc::c_void as
                  *mut db_pgno_t).offset(0 as libc::c_int as isize);
        empty_page = pagep;
        link_page =
            *((empty_page as *mut libc::c_void as
                   *mut u_int8_t).offset(4 as libc::c_int as isize) as
                  *mut libc::c_void as
                  *mut db_pgno_t).offset(0 as libc::c_int as isize);
        pagep = __kdb2_get_page(hashp, (*item_info).bucket, 0 as libc::c_int);
        if pagep.is_null() { return -(1 as libc::c_int) }
        while *((pagep as *mut libc::c_void as
                     *mut u_int8_t).offset(4 as libc::c_int as isize) as
                    *mut libc::c_void as
                    *mut db_pgno_t).offset(0 as libc::c_int as isize) !=
                  to_find {
            next_pgno =
                *((pagep as *mut libc::c_void as
                       *mut u_int8_t).offset(4 as libc::c_int as isize) as
                      *mut libc::c_void as
                      *mut db_pgno_t).offset(0 as libc::c_int as isize);
            __kdb2_put_page(hashp, pagep, 4 as libc::c_int, 0 as libc::c_int);
            pagep = __kdb2_get_page(hashp, next_pgno, 4 as libc::c_int);
            if pagep.is_null() { return -(1 as libc::c_int) }
        }
        /*
		 * At this point, pagep should point to the page before the
		 * page to be deleted.
		 */
        *((pagep as *mut libc::c_void as
               *mut u_int8_t).offset(4 as libc::c_int as isize) as
              *mut libc::c_void as
              *mut db_pgno_t).offset(0 as libc::c_int as isize) = link_page;
        if (*item_info).pgno == to_find {
            (*item_info).pgno =
                *((pagep as *mut libc::c_void as
                       *mut u_int8_t).offset(0 as libc::c_int as isize) as
                      *mut libc::c_void as
                      *mut db_pgno_t).offset(0 as libc::c_int as isize);
            (*item_info).pgndx =
                *((pagep as *mut libc::c_void as
                       *mut u_int8_t).offset(8 as libc::c_int as isize) as
                      *mut libc::c_void as
                      *mut indx_t).offset(0 as libc::c_int as isize);
            (*item_info).seek_found_page =
                *((pagep as *mut libc::c_void as
                       *mut u_int8_t).offset(0 as libc::c_int as isize) as
                      *mut libc::c_void as
                      *mut db_pgno_t).offset(0 as libc::c_int as isize)
        }
        __kdb2_delete_page(hashp, empty_page, 1 as libc::c_int);
    }
    __kdb2_put_page(hashp, pagep, 4 as libc::c_int, 1 as libc::c_int);
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "440:1"]
pub unsafe extern "C" fn __kdb2_split_page(mut hashp: *mut HTAB,
                                           mut obucket: u_int32_t,
                                           mut nbucket: u_int32_t)
 -> int32_t {
    let mut key: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    let mut val: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    let mut old_ii: ITEM_INFO =
        ITEM_INFO{pgno: 0,
                  bucket: 0,
                  ndx: 0,
                  pgndx: 0,
                  status: 0,
                  seek_size: 0,
                  seek_found_page: 0,
                  key_off: 0,
                  data_off: 0,
                  caused_expand: 0,};
    let mut new_ii: ITEM_INFO =
        ITEM_INFO{pgno: 0,
                  bucket: 0,
                  ndx: 0,
                  pgndx: 0,
                  status: 0,
                  seek_size: 0,
                  seek_found_page: 0,
                  key_off: 0,
                  data_off: 0,
                  caused_expand: 0,};
    let mut old_pagep: *mut PAGE16 = 0 as *mut PAGE16;
    let mut temp_pagep: *mut PAGE16 = 0 as *mut PAGE16;
    let mut next_pgno: db_pgno_t = 0;
    let mut off: int32_t = 0;
    let mut n: u_int16_t = 0;
    let mut base_page: int8_t = 0;
    off = (*hashp).hdr.bsize as int32_t;
    old_pagep = __kdb2_get_page(hashp, obucket, 0 as libc::c_int);
    base_page = 1 as libc::c_int as int8_t;
    temp_pagep = (*hashp).split_buf;
    memcpy(temp_pagep as *mut libc::c_void, old_pagep as *const libc::c_void,
           (*hashp).hdr.bsize as libc::c_ulong);
    page_init(hashp, old_pagep,
              *((old_pagep as *mut libc::c_void as
                     *mut u_int8_t).offset(0 as libc::c_int as isize) as
                    *mut libc::c_void as
                    *mut db_pgno_t).offset(0 as libc::c_int as isize),
              2 as libc::c_int as u_int8_t);
    __kdb2_put_page(hashp, old_pagep, 4 as libc::c_int, 1 as libc::c_int);
    old_ii.pgno =
        obucket.wrapping_add((*hashp).hdr.hdrpages).wrapping_add((if obucket
                                                                         != 0
                                                                     {
                                                                      (*hashp).hdr.spares[__kdb2_log2(obucket.wrapping_add(1
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               libc::c_uint)).wrapping_sub(1
                                                                                                                                                               as
                                                                                                                                                               libc::c_int
                                                                                                                                                               as
                                                                                                                                                               libc::c_uint)
                                                                                              as
                                                                                              usize]
                                                                  } else {
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint
                                                                  }));
    new_ii.pgno =
        nbucket.wrapping_add((*hashp).hdr.hdrpages).wrapping_add((if nbucket
                                                                         != 0
                                                                     {
                                                                      (*hashp).hdr.spares[__kdb2_log2(nbucket.wrapping_add(1
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               libc::c_uint)).wrapping_sub(1
                                                                                                                                                               as
                                                                                                                                                               libc::c_int
                                                                                                                                                               as
                                                                                                                                                               libc::c_uint)
                                                                                              as
                                                                                              usize]
                                                                  } else {
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint
                                                                  }));
    old_ii.bucket = obucket;
    new_ii.bucket = nbucket;
    new_ii.seek_found_page = 0 as libc::c_int as db_pgno_t;
    old_ii.seek_found_page = new_ii.seek_found_page;
    while !temp_pagep.is_null() {
        off = (*hashp).hdr.bsize as int32_t;
        n = 0 as libc::c_int as u_int16_t;
        while (n as libc::c_int) <
                  *((temp_pagep as *mut libc::c_void as
                         *mut u_int8_t).offset(8 as libc::c_int as isize) as
                        *mut libc::c_void as
                        *mut indx_t).offset(0 as libc::c_int as isize) as
                      libc::c_int {
            if *((temp_pagep as *mut libc::c_void as
                      *mut u_int8_t).offset((12 as libc::c_int as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                 as
                                                                                 libc::c_ulong)
                                                as
                                                isize).offset((n as
                                                                   libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                    as
                                                                                                    libc::c_ulong)
                                                                                                   <<
                                                                                                   1
                                                                                                       as
                                                                                                       libc::c_int)
                                                                  as isize) as
                     *mut libc::c_void as
                     *mut indx_t).offset(0 as libc::c_int as isize) as
                   libc::c_int == 0 as libc::c_int {
                __kdb2_get_bigkey(hashp, temp_pagep, n, &mut key);
                if __kdb2_call_hash(hashp, key.data as *mut int8_t,
                                    key.size as int32_t) == obucket {
                    add_bigptr(hashp, &mut old_ii,
                               *((temp_pagep as *mut libc::c_void as
                                      *mut u_int8_t).offset((12 as libc::c_int
                                                                 as
                                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                 as
                                                                                                 libc::c_ulong)
                                                                as
                                                                isize).offset((n
                                                                                   as
                                                                                   libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                                    as
                                                                                                                    libc::c_ulong)
                                                                                                                   <<
                                                                                                                   1
                                                                                                                       as
                                                                                                                       libc::c_int)
                                                                                  as
                                                                                  isize).offset(::std::mem::size_of::<indx_t>()
                                                                                                    as
                                                                                                    libc::c_ulong
                                                                                                    as
                                                                                                    isize)
                                     as *mut libc::c_void as
                                     *mut indx_t).offset(0 as libc::c_int as
                                                             isize));
                } else {
                    add_bigptr(hashp, &mut new_ii,
                               *((temp_pagep as *mut libc::c_void as
                                      *mut u_int8_t).offset((12 as libc::c_int
                                                                 as
                                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                 as
                                                                                                 libc::c_ulong)
                                                                as
                                                                isize).offset((n
                                                                                   as
                                                                                   libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                                    as
                                                                                                                    libc::c_ulong)
                                                                                                                   <<
                                                                                                                   1
                                                                                                                       as
                                                                                                                       libc::c_int)
                                                                                  as
                                                                                  isize).offset(::std::mem::size_of::<indx_t>()
                                                                                                    as
                                                                                                    libc::c_ulong
                                                                                                    as
                                                                                                    isize)
                                     as *mut libc::c_void as
                                     *mut indx_t).offset(0 as libc::c_int as
                                                             isize));
                }
            } else {
                key.size =
                    (off -
                         *((temp_pagep as *mut libc::c_void as
                                *mut u_int8_t).offset((12 as libc::c_int as
                                                           libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                           as
                                                                                           libc::c_ulong)
                                                          as
                                                          isize).offset((n as
                                                                             libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                              as
                                                                                                              libc::c_ulong)
                                                                                                             <<
                                                                                                             1
                                                                                                                 as
                                                                                                                 libc::c_int)
                                                                            as
                                                                            isize)
                               as *mut libc::c_void as
                               *mut indx_t).offset(0 as libc::c_int as isize)
                             as libc::c_int) as size_t;
                key.data =
                    (temp_pagep as
                         *mut PAGE8).offset(*((temp_pagep as *mut libc::c_void
                                                   as
                                                   *mut u_int8_t).offset((12
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                              as
                                                                                                              libc::c_ulong)
                                                                             as
                                                                             isize).offset((n
                                                                                                as
                                                                                                libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                                                 as
                                                                                                                                 libc::c_ulong)
                                                                                                                                <<
                                                                                                                                1
                                                                                                                                    as
                                                                                                                                    libc::c_int)
                                                                                               as
                                                                                               isize)
                                                  as *mut libc::c_void as
                                                  *mut indx_t).offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
                                                as libc::c_int as isize) as
                        *mut libc::c_void;
                off =
                    *((temp_pagep as *mut libc::c_void as
                           *mut u_int8_t).offset((12 as libc::c_int as
                                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                      as
                                                                                      libc::c_ulong)
                                                     as
                                                     isize).offset((n as
                                                                        libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                         as
                                                                                                         libc::c_ulong)
                                                                                                        <<
                                                                                                        1
                                                                                                            as
                                                                                                            libc::c_int)
                                                                       as
                                                                       isize)
                          as *mut libc::c_void as
                          *mut indx_t).offset(0 as libc::c_int as isize) as
                        int32_t;
                val.size =
                    (off -
                         *((temp_pagep as *mut libc::c_void as
                                *mut u_int8_t).offset((12 as libc::c_int as
                                                           libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                           as
                                                                                           libc::c_ulong)
                                                          as
                                                          isize).offset((n as
                                                                             libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                              as
                                                                                                              libc::c_ulong)
                                                                                                             <<
                                                                                                             1
                                                                                                                 as
                                                                                                                 libc::c_int)
                                                                            as
                                                                            isize).offset(::std::mem::size_of::<indx_t>()
                                                                                              as
                                                                                              libc::c_ulong
                                                                                              as
                                                                                              isize)
                               as *mut libc::c_void as
                               *mut indx_t).offset(0 as libc::c_int as isize)
                             as libc::c_int) as size_t;
                val.data =
                    (temp_pagep as
                         *mut PAGE8).offset(*((temp_pagep as *mut libc::c_void
                                                   as
                                                   *mut u_int8_t).offset((12
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                              as
                                                                                                              libc::c_ulong)
                                                                             as
                                                                             isize).offset((n
                                                                                                as
                                                                                                libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                                                 as
                                                                                                                                 libc::c_ulong)
                                                                                                                                <<
                                                                                                                                1
                                                                                                                                    as
                                                                                                                                    libc::c_int)
                                                                                               as
                                                                                               isize).offset(::std::mem::size_of::<indx_t>()
                                                                                                                 as
                                                                                                                 libc::c_ulong
                                                                                                                 as
                                                                                                                 isize)
                                                  as *mut libc::c_void as
                                                  *mut indx_t).offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
                                                as libc::c_int as isize) as
                        *mut libc::c_void;
                if __kdb2_call_hash(hashp, key.data as *mut int8_t,
                                    key.size as int32_t) == obucket {
                    __kdb2_addel(hashp, &mut old_ii, &mut key, &mut val,
                                 0xfffffffe as libc::c_uint,
                                 1 as libc::c_int as u_int8_t);
                } else {
                    __kdb2_addel(hashp, &mut new_ii, &mut key, &mut val,
                                 0xfffffffe as libc::c_uint,
                                 1 as libc::c_int as u_int8_t);
                }
                off =
                    *((temp_pagep as *mut libc::c_void as
                           *mut u_int8_t).offset((12 as libc::c_int as
                                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                      as
                                                                                      libc::c_ulong)
                                                     as
                                                     isize).offset((n as
                                                                        libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                         as
                                                                                                         libc::c_ulong)
                                                                                                        <<
                                                                                                        1
                                                                                                            as
                                                                                                            libc::c_int)
                                                                       as
                                                                       isize).offset(::std::mem::size_of::<indx_t>()
                                                                                         as
                                                                                         libc::c_ulong
                                                                                         as
                                                                                         isize)
                          as *mut libc::c_void as
                          *mut indx_t).offset(0 as libc::c_int as isize) as
                        int32_t
            }
            n = n.wrapping_add(1)
        }
        next_pgno =
            *((temp_pagep as *mut libc::c_void as
                   *mut u_int8_t).offset(4 as libc::c_int as isize) as
                  *mut libc::c_void as
                  *mut db_pgno_t).offset(0 as libc::c_int as isize);
        /* Clear temp_page; if it's an overflow page, free it. */
        if base_page == 0 {
            __kdb2_delete_page(hashp, temp_pagep, 1 as libc::c_int);
        } else { base_page = 0 as libc::c_int as int8_t }
        if !(next_pgno != 0xffffffff as libc::c_uint) { break ; }
        temp_pagep = __kdb2_get_page(hashp, next_pgno, 4 as libc::c_int)
    }
    return 0 as libc::c_int;
}
/*
 * Add the given pair to the page.
 *
 *
 * Returns:
 *       0 ==> OK
 *	-1 ==> failure
 */
#[no_mangle]
#[c2rust::src_loc = "521:1"]
pub unsafe extern "C" fn __kdb2_addel(mut hashp: *mut HTAB,
                                      mut item_info: *mut ITEM_INFO,
                                      mut key: *const DBT,
                                      mut val: *const DBT,
                                      mut num_items: u_int32_t,
                                      expanding: u_int8_t) -> int32_t {
    let mut pagep: *mut PAGE16 = 0 as *mut PAGE16;
    let mut do_expand: int32_t = 0;
    let mut next_pgno: db_pgno_t = 0;
    do_expand = 0 as libc::c_int;
    pagep =
        __kdb2_get_page(hashp,
                        if (*item_info).seek_found_page !=
                               0 as libc::c_int as libc::c_uint {
                            (*item_info).seek_found_page
                        } else { (*item_info).pgno }, 4 as libc::c_int);
    if pagep.is_null() { return -(1 as libc::c_int) }
    /* Advance to first page in chain with room for item. */
    while *((pagep as *mut libc::c_void as
                 *mut u_int8_t).offset(8 as libc::c_int as isize) as
                *mut libc::c_void as
                *mut indx_t).offset(0 as libc::c_int as isize) as libc::c_int
              != 0 &&
              *((pagep as *mut libc::c_void as
                     *mut u_int8_t).offset(4 as libc::c_int as isize) as
                    *mut libc::c_void as
                    *mut db_pgno_t).offset(0 as libc::c_int as isize) !=
                  0xffffffff as libc::c_uint {
        /*
		 * This may not be the end of the chain, but the pair may fit
		 * anyway.
		 */
        if (if ((::std::mem::size_of::<indx_t>() as libc::c_ulong) <<
                    1 as
                        libc::c_int).wrapping_add((*key).size).wrapping_add((*val).size)
                   as libc::c_double >
                   (*hashp).hdr.bsize as libc::c_double * 0.75f64 {
                1 as libc::c_int
            } else { 0 as libc::c_int }) != 0 &&
               ((*((pagep as *mut libc::c_void as
                        *mut u_int8_t).offset(12 as libc::c_int as isize) as
                       *mut libc::c_void as
                       *mut indx_t).offset(0 as libc::c_int as isize) as
                     libc::c_int + 1 as libc::c_int) as
                    libc::c_ulong).wrapping_sub((12 as libc::c_int as
                                                     libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                     as
                                                                                     libc::c_ulong)).wrapping_sub((*((pagep
                                                                                                                          as
                                                                                                                          *mut libc::c_void
                                                                                                                          as
                                                                                                                          *mut u_int8_t).offset(8
                                                                                                                                                    as
                                                                                                                                                    libc::c_int
                                                                                                                                                    as
                                                                                                                                                    isize)
                                                                                                                         as
                                                                                                                         *mut libc::c_void
                                                                                                                         as
                                                                                                                         *mut indx_t).offset(0
                                                                                                                                                 as
                                                                                                                                                 libc::c_int
                                                                                                                                                 as
                                                                                                                                                 isize)
                                                                                                                       as
                                                                                                                       libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                                                                        as
                                                                                                                                                        libc::c_ulong)
                                                                                                                                                       <<
                                                                                                                                                       1
                                                                                                                                                           as
                                                                                                                                                           libc::c_int))
                   >=
                   (::std::mem::size_of::<indx_t>() as libc::c_ulong) <<
                       1 as libc::c_int {
            break ;
        }
        if ((::std::mem::size_of::<indx_t>() as libc::c_ulong) <<
                1 as
                    libc::c_int).wrapping_add((*key).size).wrapping_add((*val).size)
               <=
               ((*((pagep as *mut libc::c_void as
                        *mut u_int8_t).offset(12 as libc::c_int as isize) as
                       *mut libc::c_void as
                       *mut indx_t).offset(0 as libc::c_int as isize) as
                     libc::c_int + 1 as libc::c_int) as
                    libc::c_ulong).wrapping_sub((12 as libc::c_int as
                                                     libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                     as
                                                                                     libc::c_ulong)).wrapping_sub((*((pagep
                                                                                                                          as
                                                                                                                          *mut libc::c_void
                                                                                                                          as
                                                                                                                          *mut u_int8_t).offset(8
                                                                                                                                                    as
                                                                                                                                                    libc::c_int
                                                                                                                                                    as
                                                                                                                                                    isize)
                                                                                                                         as
                                                                                                                         *mut libc::c_void
                                                                                                                         as
                                                                                                                         *mut indx_t).offset(0
                                                                                                                                                 as
                                                                                                                                                 libc::c_int
                                                                                                                                                 as
                                                                                                                                                 isize)
                                                                                                                       as
                                                                                                                       libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                                                                        as
                                                                                                                                                        libc::c_ulong)
                                                                                                                                                       <<
                                                                                                                                                       1
                                                                                                                                                           as
                                                                                                                                                           libc::c_int))
           {
            break ;
        }
        next_pgno =
            *((pagep as *mut libc::c_void as
                   *mut u_int8_t).offset(4 as libc::c_int as isize) as
                  *mut libc::c_void as
                  *mut db_pgno_t).offset(0 as libc::c_int as isize);
        __kdb2_put_page(hashp, pagep, 4 as libc::c_int, 0 as libc::c_int);
        pagep = __kdb2_get_page(hashp, next_pgno, 4 as libc::c_int);
        if pagep.is_null() { return -(1 as libc::c_int) }
    }
    if (if ((::std::mem::size_of::<indx_t>() as libc::c_ulong) <<
                1 as
                    libc::c_int).wrapping_add((*key).size).wrapping_add((*val).size)
               as libc::c_double >
               (*hashp).hdr.bsize as libc::c_double * 0.75f64 {
            1 as libc::c_int
        } else { 0 as libc::c_int }) != 0 &&
           !(((*((pagep as *mut libc::c_void as
                      *mut u_int8_t).offset(12 as libc::c_int as isize) as
                     *mut libc::c_void as
                     *mut indx_t).offset(0 as libc::c_int as isize) as
                   libc::c_int + 1 as libc::c_int) as
                  libc::c_ulong).wrapping_sub((12 as libc::c_int as
                                                   libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                   as
                                                                                   libc::c_ulong)).wrapping_sub((*((pagep
                                                                                                                        as
                                                                                                                        *mut libc::c_void
                                                                                                                        as
                                                                                                                        *mut u_int8_t).offset(8
                                                                                                                                                  as
                                                                                                                                                  libc::c_int
                                                                                                                                                  as
                                                                                                                                                  isize)
                                                                                                                       as
                                                                                                                       *mut libc::c_void
                                                                                                                       as
                                                                                                                       *mut indx_t).offset(0
                                                                                                                                               as
                                                                                                                                               libc::c_int
                                                                                                                                               as
                                                                                                                                               isize)
                                                                                                                     as
                                                                                                                     libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                                                                      as
                                                                                                                                                      libc::c_ulong)
                                                                                                                                                     <<
                                                                                                                                                     1
                                                                                                                                                         as
                                                                                                                                                         libc::c_int))
                 >=
                 (::std::mem::size_of::<indx_t>() as libc::c_ulong) <<
                     1 as libc::c_int) ||
           (if ((::std::mem::size_of::<indx_t>() as libc::c_ulong) <<
                    1 as
                        libc::c_int).wrapping_add((*key).size).wrapping_add((*val).size)
                   as libc::c_double >
                   (*hashp).hdr.bsize as libc::c_double * 0.75f64 {
                1 as libc::c_int
            } else { 0 as libc::c_int }) == 0 &&
               !(((::std::mem::size_of::<indx_t>() as libc::c_ulong) <<
                      1 as
                          libc::c_int).wrapping_add((*key).size).wrapping_add((*val).size)
                     <=
                     ((*((pagep as *mut libc::c_void as
                              *mut u_int8_t).offset(12 as libc::c_int as
                                                        isize) as
                             *mut libc::c_void as
                             *mut indx_t).offset(0 as libc::c_int as isize) as
                           libc::c_int + 1 as libc::c_int) as
                          libc::c_ulong).wrapping_sub((12 as libc::c_int as
                                                           libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                           as
                                                                                           libc::c_ulong)).wrapping_sub((*((pagep
                                                                                                                                as
                                                                                                                                *mut libc::c_void
                                                                                                                                as
                                                                                                                                *mut u_int8_t).offset(8
                                                                                                                                                          as
                                                                                                                                                          libc::c_int
                                                                                                                                                          as
                                                                                                                                                          isize)
                                                                                                                               as
                                                                                                                               *mut libc::c_void
                                                                                                                               as
                                                                                                                               *mut indx_t).offset(0
                                                                                                                                                       as
                                                                                                                                                       libc::c_int
                                                                                                                                                       as
                                                                                                                                                       isize)
                                                                                                                             as
                                                                                                                             libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                                                                              as
                                                                                                                                                              libc::c_ulong)
                                                                                                                                                             <<
                                                                                                                                                             1
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_int)))
       {
        do_expand = 1 as libc::c_int;
        pagep = __kdb2_add_ovflpage(hashp, pagep);
        if pagep.is_null() { return -(1 as libc::c_int) }
        if (if ((::std::mem::size_of::<indx_t>() as libc::c_ulong) <<
                    1 as
                        libc::c_int).wrapping_add((*key).size).wrapping_add((*val).size)
                   as libc::c_double >
                   (*hashp).hdr.bsize as libc::c_double * 0.75f64 {
                1 as libc::c_int
            } else { 0 as libc::c_int }) != 0 &&
               !(((*((pagep as *mut libc::c_void as
                          *mut u_int8_t).offset(12 as libc::c_int as isize) as
                         *mut libc::c_void as
                         *mut indx_t).offset(0 as libc::c_int as isize) as
                       libc::c_int + 1 as libc::c_int) as
                      libc::c_ulong).wrapping_sub((12 as libc::c_int as
                                                       libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                       as
                                                                                       libc::c_ulong)).wrapping_sub((*((pagep
                                                                                                                            as
                                                                                                                            *mut libc::c_void
                                                                                                                            as
                                                                                                                            *mut u_int8_t).offset(8
                                                                                                                                                      as
                                                                                                                                                      libc::c_int
                                                                                                                                                      as
                                                                                                                                                      isize)
                                                                                                                           as
                                                                                                                           *mut libc::c_void
                                                                                                                           as
                                                                                                                           *mut indx_t).offset(0
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   as
                                                                                                                                                   isize)
                                                                                                                         as
                                                                                                                         libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                                                                          as
                                                                                                                                                          libc::c_ulong)
                                                                                                                                                         <<
                                                                                                                                                         1
                                                                                                                                                             as
                                                                                                                                                             libc::c_int))
                     >=
                     (::std::mem::size_of::<indx_t>() as libc::c_ulong) <<
                         1 as libc::c_int) ||
               (if ((::std::mem::size_of::<indx_t>() as libc::c_ulong) <<
                        1 as
                            libc::c_int).wrapping_add((*key).size).wrapping_add((*val).size)
                       as libc::c_double >
                       (*hashp).hdr.bsize as libc::c_double * 0.75f64 {
                    1 as libc::c_int
                } else { 0 as libc::c_int }) == 0 &&
                   !(((::std::mem::size_of::<indx_t>() as libc::c_ulong) <<
                          1 as
                              libc::c_int).wrapping_add((*key).size).wrapping_add((*val).size)
                         <=
                         ((*((pagep as *mut libc::c_void as
                                  *mut u_int8_t).offset(12 as libc::c_int as
                                                            isize) as
                                 *mut libc::c_void as
                                 *mut indx_t).offset(0 as libc::c_int as
                                                         isize) as libc::c_int
                               + 1 as libc::c_int) as
                              libc::c_ulong).wrapping_sub((12 as libc::c_int
                                                               as
                                                               libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                               as
                                                                                               libc::c_ulong)).wrapping_sub((*((pagep
                                                                                                                                    as
                                                                                                                                    *mut libc::c_void
                                                                                                                                    as
                                                                                                                                    *mut u_int8_t).offset(8
                                                                                                                                                              as
                                                                                                                                                              libc::c_int
                                                                                                                                                              as
                                                                                                                                                              isize)
                                                                                                                                   as
                                                                                                                                   *mut libc::c_void
                                                                                                                                   as
                                                                                                                                   *mut indx_t).offset(0
                                                                                                                                                           as
                                                                                                                                                           libc::c_int
                                                                                                                                                           as
                                                                                                                                                           isize)
                                                                                                                                 as
                                                                                                                                 libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                                                                                  as
                                                                                                                                                                  libc::c_ulong)
                                                                                                                                                                 <<
                                                                                                                                                                 1
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_int)))
           {
            __kdb2_put_page(hashp, pagep, 4 as libc::c_int, 0 as libc::c_int);
            return -(1 as libc::c_int)
        }
    }
    /* At this point, we know the page fits, so we just add it */
    if if ((::std::mem::size_of::<indx_t>() as libc::c_ulong) <<
               1 as
                   libc::c_int).wrapping_add((*key).size).wrapping_add((*val).size)
              as libc::c_double >
              (*hashp).hdr.bsize as libc::c_double * 0.75f64 {
           1 as libc::c_int
       } else { 0 as libc::c_int } != 0 {
        if __kdb2_big_insert(hashp, pagep, key, val) != 0 {
            return -(1 as libc::c_int)
        }
    } else { putpair(pagep as *mut PAGE8, key, val); }
    /*
	 * For splits, we are going to update item_info's page number
	 * field, so that we can easily return to the same page the
	 * next time we come in here.  For other operations, this shouldn't
	 * matter, since adds are the last thing that happens before we
	 * return to the user program.
	 */
    (*item_info).pgno =
        *((pagep as *mut libc::c_void as
               *mut u_int8_t).offset(0 as libc::c_int as isize) as
              *mut libc::c_void as
              *mut db_pgno_t).offset(0 as libc::c_int as isize);
    if expanding == 0 { (*hashp).hdr.nkeys += 1 }
    /* Kludge: if this is a big page, then it's already been put. */
    if if ((::std::mem::size_of::<indx_t>() as libc::c_ulong) <<
               1 as
                   libc::c_int).wrapping_add((*key).size).wrapping_add((*val).size)
              as libc::c_double >
              (*hashp).hdr.bsize as libc::c_double * 0.75f64 {
           1 as libc::c_int
       } else { 0 as libc::c_int } == 0 {
        __kdb2_put_page(hashp, pagep, 4 as libc::c_int, 1 as libc::c_int);
    }
    if expanding != 0 {
        (*item_info).caused_expand = 0 as libc::c_int as u_int8_t
    } else {
        match num_items {
            4294967294 => {
                (*item_info).caused_expand = 0 as libc::c_int as u_int8_t
            }
            4294967295 => {
                (*item_info).caused_expand =
                    ((*item_info).caused_expand as libc::c_int |
                         (((*hashp).hdr.nkeys as
                               libc::c_uint).wrapping_div((*hashp).hdr.max_bucket)
                              > (*hashp).hdr.ffactor ||
                              (*item_info).pgndx as libc::c_uint >
                                  (*hashp).hdr.ffactor) as libc::c_int) as
                        u_int8_t
            }
            _ => {
                (*item_info).caused_expand =
                    if num_items > (*hashp).hdr.ffactor {
                        1 as libc::c_int
                    } else { do_expand } as u_int8_t
            }
        }
    }
    return 0 as libc::c_int;
}
/*-
 * Copyright (c) 1990, 1993, 1994
 *	The Regents of the University of California.  All rights reserved.
 *
 * This code is derived from software contributed to Berkeley by
 * Margo Seltzer.
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
/*
 * PACKAGE:  hashing
 *
 * DESCRIPTION:
 *      Page manipulation for hashing package.
 *
 * ROUTINES:
 *
 * External
 *      __get_page
 *      __add_ovflpage
 * Internal
 *      overflow_page
 *      open_temp
 */
/*
 * Special __addel used in big splitting; this one just puts the pointer
 * to an already-allocated big page in the appropriate bucket.
 */
#[c2rust::src_loc = "631:1"]
unsafe extern "C" fn add_bigptr(mut hashp: *mut HTAB,
                                mut item_info: *mut ITEM_INFO,
                                mut big_pgno: indx_t) -> int32_t {
    let mut pagep: *mut PAGE16 = 0 as *mut PAGE16;
    let mut next_pgno: db_pgno_t = 0;
    pagep = __kdb2_get_page(hashp, (*item_info).bucket, 0 as libc::c_int);
    if pagep.is_null() { return -(1 as libc::c_int) }
    /*
	 * Note: in __addel(), we used item_info->pgno for the beginning of
	 * our search for space.  Now, we use item_info->bucket, since we
	 * know that the space required by a big pair on the base page is
	 * quite small, and we may very well find that space early in the
	 * chain.
	 */
    /* Find first page in chain that has space for a big pair. */
    while *((pagep as *mut libc::c_void as
                 *mut u_int8_t).offset(8 as libc::c_int as isize) as
                *mut libc::c_void as
                *mut indx_t).offset(0 as libc::c_int as isize) as libc::c_int
              != 0 &&
              *((pagep as *mut libc::c_void as
                     *mut u_int8_t).offset(4 as libc::c_int as isize) as
                    *mut libc::c_void as
                    *mut db_pgno_t).offset(0 as libc::c_int as isize) !=
                  0xffffffff as libc::c_uint {
        if ((*((pagep as *mut libc::c_void as
                    *mut u_int8_t).offset(12 as libc::c_int as isize) as
                   *mut libc::c_void as
                   *mut indx_t).offset(0 as libc::c_int as isize) as
                 libc::c_int + 1 as libc::c_int) as
                libc::c_ulong).wrapping_sub((12 as libc::c_int as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                 as
                                                                                 libc::c_ulong)).wrapping_sub((*((pagep
                                                                                                                      as
                                                                                                                      *mut libc::c_void
                                                                                                                      as
                                                                                                                      *mut u_int8_t).offset(8
                                                                                                                                                as
                                                                                                                                                libc::c_int
                                                                                                                                                as
                                                                                                                                                isize)
                                                                                                                     as
                                                                                                                     *mut libc::c_void
                                                                                                                     as
                                                                                                                     *mut indx_t).offset(0
                                                                                                                                             as
                                                                                                                                             libc::c_int
                                                                                                                                             as
                                                                                                                                             isize)
                                                                                                                   as
                                                                                                                   libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                                                                    as
                                                                                                                                                    libc::c_ulong)
                                                                                                                                                   <<
                                                                                                                                                   1
                                                                                                                                                       as
                                                                                                                                                       libc::c_int))
               >=
               (::std::mem::size_of::<indx_t>() as libc::c_ulong) <<
                   1 as libc::c_int {
            break ;
        }
        next_pgno =
            *((pagep as *mut libc::c_void as
                   *mut u_int8_t).offset(4 as libc::c_int as isize) as
                  *mut libc::c_void as
                  *mut db_pgno_t).offset(0 as libc::c_int as isize);
        __kdb2_put_page(hashp, pagep, 4 as libc::c_int, 0 as libc::c_int);
        pagep = __kdb2_get_page(hashp, next_pgno, 4 as libc::c_int);
        if pagep.is_null() { return -(1 as libc::c_int) }
    }
    if !(((*((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(12 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut indx_t).offset(0 as libc::c_int as isize) as libc::c_int
               + 1 as libc::c_int) as
              libc::c_ulong).wrapping_sub((12 as libc::c_int as
                                               libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                               as
                                                                               libc::c_ulong)).wrapping_sub((*((pagep
                                                                                                                    as
                                                                                                                    *mut libc::c_void
                                                                                                                    as
                                                                                                                    *mut u_int8_t).offset(8
                                                                                                                                              as
                                                                                                                                              libc::c_int
                                                                                                                                              as
                                                                                                                                              isize)
                                                                                                                   as
                                                                                                                   *mut libc::c_void
                                                                                                                   as
                                                                                                                   *mut indx_t).offset(0
                                                                                                                                           as
                                                                                                                                           libc::c_int
                                                                                                                                           as
                                                                                                                                           isize)
                                                                                                                 as
                                                                                                                 libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                                                                  as
                                                                                                                                                  libc::c_ulong)
                                                                                                                                                 <<
                                                                                                                                                 1
                                                                                                                                                     as
                                                                                                                                                     libc::c_int))
             >=
             (::std::mem::size_of::<indx_t>() as libc::c_ulong) <<
                 1 as libc::c_int) {
        pagep = __kdb2_add_ovflpage(hashp, pagep);
        if pagep.is_null() { return -(1 as libc::c_int) }
    }
    *((pagep as *mut libc::c_void as
           *mut u_int8_t).offset((12 as libc::c_int as
                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                      as
                                                                      libc::c_ulong)
                                     as
                                     isize).offset((*((pagep as
                                                           *mut libc::c_void
                                                           as
                                                           *mut u_int8_t).offset(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)
                                                          as *mut libc::c_void
                                                          as
                                                          *mut indx_t).offset(0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)
                                                        as
                                                        libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                         as
                                                                                         libc::c_ulong)
                                                                                        <<
                                                                                        1
                                                                                            as
                                                                                            libc::c_int)
                                                       as isize) as
          *mut libc::c_void as *mut indx_t).offset(0 as libc::c_int as isize)
        = 0 as libc::c_int as indx_t;
    *((pagep as *mut libc::c_void as
           *mut u_int8_t).offset((12 as libc::c_int as
                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                      as
                                                                      libc::c_ulong)
                                     as
                                     isize).offset((*((pagep as
                                                           *mut libc::c_void
                                                           as
                                                           *mut u_int8_t).offset(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)
                                                          as *mut libc::c_void
                                                          as
                                                          *mut indx_t).offset(0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)
                                                        as
                                                        libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                         as
                                                                                         libc::c_ulong)
                                                                                        <<
                                                                                        1
                                                                                            as
                                                                                            libc::c_int)
                                                       as
                                                       isize).offset(::std::mem::size_of::<indx_t>()
                                                                         as
                                                                         libc::c_ulong
                                                                         as
                                                                         isize)
          as *mut libc::c_void as
          *mut indx_t).offset(0 as libc::c_int as isize) = big_pgno;
    *((pagep as *mut libc::c_void as
           *mut u_int8_t).offset(8 as libc::c_int as isize) as
          *mut libc::c_void as *mut indx_t).offset(0 as libc::c_int as isize)
        =
        (*((pagep as *mut libc::c_void as
                *mut u_int8_t).offset(8 as libc::c_int as isize) as
               *mut libc::c_void as
               *mut indx_t).offset(0 as libc::c_int as isize) as libc::c_int +
             1 as libc::c_int) as indx_t;
    __kdb2_put_page(hashp, pagep, 4 as libc::c_int, 1 as libc::c_int);
    return 0 as libc::c_int;
}
/*
 *
 * Returns:
 *      pointer on success
 *      NULL on error
 */
#[no_mangle]
#[c2rust::src_loc = "689:1"]
pub unsafe extern "C" fn __kdb2_add_ovflpage(mut hashp: *mut HTAB,
                                             mut pagep: *mut PAGE16)
 -> *mut PAGE16 {
    let mut new_pagep: *mut PAGE16 = 0 as *mut PAGE16;
    let mut ovfl_num: u_int16_t = 0;
    /* Check if we are dynamically determining the fill factor. */
    if (*hashp).hdr.ffactor == 65536 as libc::c_int as libc::c_uint {
        (*hashp).hdr.ffactor =
            (*((pagep as *mut libc::c_void as
                    *mut u_int8_t).offset(8 as libc::c_int as isize) as
                   *mut libc::c_void as
                   *mut indx_t).offset(0 as libc::c_int as isize) as
                 libc::c_int >> 1 as libc::c_int) as u_int32_t;
        if (*hashp).hdr.ffactor < 4 as libc::c_int as libc::c_uint {
            (*hashp).hdr.ffactor = 4 as libc::c_int as u_int32_t
        }
    }
    ovfl_num = overflow_page(hashp);
    if ovfl_num == 0 { return 0 as *mut PAGE16 }
    if __kdb2_new_page(hashp, ovfl_num as u_int32_t, 1 as libc::c_int) !=
           0 as libc::c_int {
        return 0 as *mut PAGE16
    }
    if ovfl_num == 0 ||
           {
               new_pagep =
                   __kdb2_get_page(hashp, ovfl_num as u_int32_t,
                                   1 as libc::c_int);
               new_pagep.is_null()
           } {
        return 0 as *mut PAGE16
    }
    *((pagep as *mut libc::c_void as
           *mut u_int8_t).offset(4 as libc::c_int as isize) as
          *mut libc::c_void as
          *mut db_pgno_t).offset(0 as libc::c_int as isize) =
        ((((1 as libc::c_int) << (ovfl_num as u_int32_t >> 11 as libc::c_int))
              - 1 as libc::c_int) as
             libc::c_uint).wrapping_add((*hashp).hdr.hdrpages).wrapping_add((if ((1
                                                                                      as
                                                                                      libc::c_int)
                                                                                     <<
                                                                                     (ovfl_num
                                                                                          as
                                                                                          u_int32_t
                                                                                          >>
                                                                                          11
                                                                                              as
                                                                                              libc::c_int))
                                                                                    -
                                                                                    1
                                                                                        as
                                                                                        libc::c_int
                                                                                    !=
                                                                                    0
                                                                                {
                                                                                 (*hashp).hdr.spares[__kdb2_log2((((1
                                                                                                                        as
                                                                                                                        libc::c_int)
                                                                                                                       <<
                                                                                                                       (ovfl_num
                                                                                                                            as
                                                                                                                            u_int32_t
                                                                                                                            >>
                                                                                                                            11
                                                                                                                                as
                                                                                                                                libc::c_int))
                                                                                                                      -
                                                                                                                      1
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                      +
                                                                                                                      1
                                                                                                                          as
                                                                                                                          libc::c_int)
                                                                                                                     as
                                                                                                                     u_int32_t).wrapping_sub(1
                                                                                                                                                 as
                                                                                                                                                 libc::c_int
                                                                                                                                                 as
                                                                                                                                                 libc::c_uint)
                                                                                                         as
                                                                                                         usize]
                                                                             } else {
                                                                                 0
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint
                                                                             })).wrapping_add((ovfl_num
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   &
                                                                                                   0x7ff
                                                                                                       as
                                                                                                       libc::c_int)
                                                                                                  as
                                                                                                  libc::c_uint);
    *((new_pagep as *mut libc::c_void as
           *mut u_int8_t).offset(10 as libc::c_int as isize) as
          *mut libc::c_void as
          *mut u_int8_t).offset(0 as libc::c_int as isize) =
        4 as libc::c_int as u_int8_t;
    __kdb2_put_page(hashp, pagep, 4 as libc::c_int, 1 as libc::c_int);
    return new_pagep;
}
/*
 *
 * Returns:
 *      pointer on success
 *      NULL on error
 */
#[no_mangle]
#[c2rust::src_loc = "730:1"]
pub unsafe extern "C" fn __kdb2_add_bigpage(mut hashp: *mut HTAB,
                                            mut pagep: *mut PAGE16,
                                            mut ndx: indx_t,
                                            is_basepage: u_int8_t)
 -> *mut PAGE16 {
    let mut new_pagep: *mut PAGE16 = 0 as *mut PAGE16;
    let mut ovfl_num: u_int16_t = 0;
    ovfl_num = overflow_page(hashp);
    if ovfl_num == 0 { return 0 as *mut PAGE16 }
    if __kdb2_new_page(hashp, ovfl_num as u_int32_t, 1 as libc::c_int) !=
           0 as libc::c_int {
        return 0 as *mut PAGE16
    }
    if ovfl_num == 0 ||
           {
               new_pagep =
                   __kdb2_get_page(hashp, ovfl_num as u_int32_t,
                                   1 as libc::c_int);
               new_pagep.is_null()
           } {
        return 0 as *mut PAGE16
    }
    if is_basepage != 0 {
        *((pagep as *mut libc::c_void as
               *mut u_int8_t).offset((12 as libc::c_int as
                                          libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                          as
                                                                          libc::c_ulong)
                                         as
                                         isize).offset((ndx as
                                                            libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                             as
                                                                                             libc::c_ulong)
                                                                                            <<
                                                                                            1
                                                                                                as
                                                                                                libc::c_int)
                                                           as isize) as
              *mut libc::c_void as
              *mut indx_t).offset(0 as libc::c_int as isize) =
            0 as libc::c_int as indx_t;
        *((pagep as *mut libc::c_void as
               *mut u_int8_t).offset((12 as libc::c_int as
                                          libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                          as
                                                                          libc::c_ulong)
                                         as
                                         isize).offset((ndx as
                                                            libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                             as
                                                                                             libc::c_ulong)
                                                                                            <<
                                                                                            1
                                                                                                as
                                                                                                libc::c_int)
                                                           as
                                                           isize).offset(::std::mem::size_of::<indx_t>()
                                                                             as
                                                                             libc::c_ulong
                                                                             as
                                                                             isize)
              as *mut libc::c_void as
              *mut indx_t).offset(0 as libc::c_int as isize) = ovfl_num
    } else {
        *((pagep as *mut libc::c_void as
               *mut u_int8_t).offset(4 as libc::c_int as isize) as
              *mut libc::c_void as
              *mut db_pgno_t).offset(0 as libc::c_int as isize) =
            *((new_pagep as *mut libc::c_void as
                   *mut u_int8_t).offset(0 as libc::c_int as isize) as
                  *mut libc::c_void as
                  *mut db_pgno_t).offset(0 as libc::c_int as isize)
    }
    __kdb2_put_page(hashp, pagep, 4 as libc::c_int, 1 as libc::c_int);
    *((new_pagep as *mut libc::c_void as
           *mut u_int8_t).offset(10 as libc::c_int as isize) as
          *mut libc::c_void as
          *mut u_int8_t).offset(0 as libc::c_int as isize) =
        3 as libc::c_int as u_int8_t;
    return new_pagep;
}
#[c2rust::src_loc = "771:1"]
unsafe extern "C" fn page_init(mut hashp: *mut HTAB, mut pagep: *mut PAGE16,
                               mut pgno: db_pgno_t, mut type_0: u_int8_t) {
    *((pagep as *mut libc::c_void as
           *mut u_int8_t).offset(8 as libc::c_int as isize) as
          *mut libc::c_void as *mut indx_t).offset(0 as libc::c_int as isize)
        = 0 as libc::c_int as indx_t;
    let ref mut fresh0 =
        *((pagep as *mut libc::c_void as
               *mut u_int8_t).offset(4 as libc::c_int as isize) as
              *mut libc::c_void as
              *mut db_pgno_t).offset(0 as libc::c_int as isize);
    *fresh0 = 0xffffffff as libc::c_uint;
    *((pagep as *mut libc::c_void as
           *mut u_int8_t).offset(0 as libc::c_int as isize) as
          *mut libc::c_void as
          *mut db_pgno_t).offset(0 as libc::c_int as isize) = *fresh0;
    *((pagep as *mut libc::c_void as
           *mut u_int8_t).offset(10 as libc::c_int as isize) as
          *mut libc::c_void as
          *mut u_int8_t).offset(0 as libc::c_int as isize) = type_0;
    *((pagep as *mut libc::c_void as
           *mut u_int8_t).offset(12 as libc::c_int as isize) as
          *mut libc::c_void as *mut indx_t).offset(0 as libc::c_int as isize)
        =
        (*hashp).hdr.bsize.wrapping_sub(1 as libc::c_int as libc::c_uint) as
            indx_t;
    /*
	 * Note: since in the current version ADDR(pagep) == PREV_PGNO(pagep),
	 * make sure that ADDR(pagep) is set after resetting PREV_PGNO(pagep).
	 * We reset PREV_PGNO(pagep) just in case the macros are changed.
	 */
    *((pagep as *mut libc::c_void as
           *mut u_int8_t).offset(0 as libc::c_int as isize) as
          *mut libc::c_void as
          *mut db_pgno_t).offset(0 as libc::c_int as isize) = pgno;
}
#[no_mangle]
#[c2rust::src_loc = "796:1"]
pub unsafe extern "C" fn __kdb2_new_page(mut hashp: *mut HTAB,
                                         mut addr: u_int32_t,
                                         mut addr_type: int32_t) -> int32_t {
    let mut paddr: db_pgno_t = 0;
    let mut pagep: *mut PAGE16 = 0 as *mut PAGE16;
    match addr_type {
        0 => {
            /* Convert page number. */
            paddr =
                addr.wrapping_add((*hashp).hdr.hdrpages).wrapping_add((if addr
                                                                              !=
                                                                              0
                                                                          {
                                                                           (*hashp).hdr.spares[__kdb2_log2(addr.wrapping_add(1
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 as
                                                                                                                                 libc::c_uint)).wrapping_sub(1
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_int
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_uint)
                                                                                                   as
                                                                                                   usize]
                                                                       } else {
                                                                           0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint
                                                                       }))
        }
        1 | 2 => {
            paddr =
                ((((1 as libc::c_int) << (addr >> 11 as libc::c_int)) -
                      1 as libc::c_int) as
                     libc::c_uint).wrapping_add((*hashp).hdr.hdrpages).wrapping_add((if ((1
                                                                                              as
                                                                                              libc::c_int)
                                                                                             <<
                                                                                             (addr
                                                                                                  >>
                                                                                                  11
                                                                                                      as
                                                                                                      libc::c_int))
                                                                                            -
                                                                                            1
                                                                                                as
                                                                                                libc::c_int
                                                                                            !=
                                                                                            0
                                                                                        {
                                                                                         (*hashp).hdr.spares[__kdb2_log2((((1
                                                                                                                                as
                                                                                                                                libc::c_int)
                                                                                                                               <<
                                                                                                                               (addr
                                                                                                                                    >>
                                                                                                                                    11
                                                                                                                                        as
                                                                                                                                        libc::c_int))
                                                                                                                              -
                                                                                                                              1
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                              +
                                                                                                                              1
                                                                                                                                  as
                                                                                                                                  libc::c_int)
                                                                                                                             as
                                                                                                                             u_int32_t).wrapping_sub(1
                                                                                                                                                         as
                                                                                                                                                         libc::c_int
                                                                                                                                                         as
                                                                                                                                                         libc::c_uint)
                                                                                                                 as
                                                                                                                 usize]
                                                                                     } else {
                                                                                         0
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_uint
                                                                                     })).wrapping_add(addr
                                                                                                          &
                                                                                                          0x7ff
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint)
        }
        _ => { paddr = addr }
    }
    pagep =
        kdb2_mpool_new((*hashp).mp, &mut paddr, 0x1 as libc::c_int as u_int)
            as *mut PAGE16;
    if pagep.is_null() { return -(1 as libc::c_int) }
    if addr_type != 2 as libc::c_int {
        page_init(hashp, pagep, paddr, 2 as libc::c_int as u_int8_t);
    }
    __kdb2_put_page(hashp, pagep, addr_type, 1 as libc::c_int);
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "832:1"]
pub unsafe extern "C" fn __kdb2_delete_page(mut hashp: *mut HTAB,
                                            mut pagep: *mut PAGE16,
                                            mut page_type: int32_t)
 -> int32_t {
    if page_type == 1 as libc::c_int { __kdb2_free_ovflpage(hashp, pagep); }
    return kdb2_mpool_delete((*hashp).mp, pagep as *mut libc::c_void);
}
#[c2rust::src_loc = "843:1"]
unsafe extern "C" fn is_bitmap_pgno(mut hashp: *mut HTAB, mut pgno: db_pgno_t)
 -> u_int8_t {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i < (*hashp).nmaps {
        if ((((1 as libc::c_int) <<
                  ((*hashp).hdr.bitmaps[i as usize] as u_int32_t >>
                       11 as libc::c_int)) - 1 as libc::c_int) as
                libc::c_uint).wrapping_add((*hashp).hdr.hdrpages).wrapping_add((if ((1
                                                                                         as
                                                                                         libc::c_int)
                                                                                        <<
                                                                                        ((*hashp).hdr.bitmaps[i
                                                                                                                  as
                                                                                                                  usize]
                                                                                             as
                                                                                             u_int32_t
                                                                                             >>
                                                                                             11
                                                                                                 as
                                                                                                 libc::c_int))
                                                                                       -
                                                                                       1
                                                                                           as
                                                                                           libc::c_int
                                                                                       !=
                                                                                       0
                                                                                   {
                                                                                    (*hashp).hdr.spares[__kdb2_log2((((1
                                                                                                                           as
                                                                                                                           libc::c_int)
                                                                                                                          <<
                                                                                                                          ((*hashp).hdr.bitmaps[i
                                                                                                                                                    as
                                                                                                                                                    usize]
                                                                                                                               as
                                                                                                                               u_int32_t
                                                                                                                               >>
                                                                                                                               11
                                                                                                                                   as
                                                                                                                                   libc::c_int))
                                                                                                                         -
                                                                                                                         1
                                                                                                                             as
                                                                                                                             libc::c_int
                                                                                                                         +
                                                                                                                         1
                                                                                                                             as
                                                                                                                             libc::c_int)
                                                                                                                        as
                                                                                                                        u_int32_t).wrapping_sub(1
                                                                                                                                                    as
                                                                                                                                                    libc::c_int
                                                                                                                                                    as
                                                                                                                                                    libc::c_uint)
                                                                                                            as
                                                                                                            usize]
                                                                                } else {
                                                                                    0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint
                                                                                })).wrapping_add(((*hashp).hdr.bitmaps[i
                                                                                                                           as
                                                                                                                           usize]
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      &
                                                                                                      0x7ff
                                                                                                          as
                                                                                                          libc::c_int)
                                                                                                     as
                                                                                                     libc::c_uint)
               == pgno {
            return 1 as libc::c_int as u_int8_t
        }
        i += 1
    }
    return 0 as libc::c_int as u_int8_t;
}
#[no_mangle]
#[c2rust::src_loc = "856:1"]
pub unsafe extern "C" fn __kdb2_pgin_routine(mut pg_cookie: *mut libc::c_void,
                                             mut pgno: db_pgno_t,
                                             mut page: *mut libc::c_void) {
    let mut hashp: *mut HTAB = 0 as *mut HTAB;
    let mut pagep: *mut PAGE16 = 0 as *mut PAGE16;
    let mut max: int32_t = 0;
    let mut i: int32_t = 0;
    pagep = page as *mut PAGE16;
    hashp = pg_cookie as *mut HTAB;
    /*
	 * There are the following cases for swapping:
	 * 0) New page that may be unitialized.
	 * 1) Bucket page or overflow page.  Either swap
	 *	the header or initialize the page.
	 * 2) Bitmap page.  Swap the whole page!
	 * 3) Header pages.  Not handled here; these are written directly
	 *    to the file.
	 */
    if *((pagep as *mut libc::c_void as
              *mut u_int8_t).offset(8 as libc::c_int as isize) as
             *mut libc::c_void as
             *mut indx_t).offset(0 as libc::c_int as isize) as libc::c_int ==
           0 as libc::c_int &&
           *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(4 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut db_pgno_t).offset(0 as libc::c_int as isize) ==
               0 as libc::c_int as libc::c_uint &&
           is_bitmap_pgno(hashp, pgno) == 0 {
        /* XXX check for !0 LSN */
        page_init(hashp, pagep, pgno,
                  2 as libc::c_int as u_int8_t); /* divide by 4 bytes */
        return
    }
    if (*hashp).hdr.lorder == 1234 as libc::c_int { return }
    if is_bitmap_pgno(hashp, pgno) != 0 {
        max = ((*hashp).hdr.bsize >> 2 as libc::c_int) as int32_t;
        i = 0 as libc::c_int;
        while i < max {
            let mut _tmp: u_int32_t =
                *(pagep as *mut libc::c_void as
                      *mut int32_t).offset(i as isize) as u_int32_t;
            *(&mut *(pagep as *mut libc::c_void as
                         *mut int32_t).offset(i as isize) as *mut int32_t as
                  *mut libc::c_char).offset(0 as libc::c_int as isize) =
                *(&mut _tmp as *mut u_int32_t as
                      *mut libc::c_char).offset(3 as libc::c_int as isize);
            *(&mut *(pagep as *mut libc::c_void as
                         *mut int32_t).offset(i as isize) as *mut int32_t as
                  *mut libc::c_char).offset(1 as libc::c_int as isize) =
                *(&mut _tmp as *mut u_int32_t as
                      *mut libc::c_char).offset(2 as libc::c_int as isize);
            *(&mut *(pagep as *mut libc::c_void as
                         *mut int32_t).offset(i as isize) as *mut int32_t as
                  *mut libc::c_char).offset(2 as libc::c_int as isize) =
                *(&mut _tmp as *mut u_int32_t as
                      *mut libc::c_char).offset(1 as libc::c_int as isize);
            *(&mut *(pagep as *mut libc::c_void as
                         *mut int32_t).offset(i as isize) as *mut int32_t as
                  *mut libc::c_char).offset(3 as libc::c_int as isize) =
                *(&mut _tmp as *mut u_int32_t as
                      *mut libc::c_char).offset(0 as libc::c_int as isize);
            i += 1
        }
    } else { swap_page_header_in(pagep); };
}
#[no_mangle]
#[c2rust::src_loc = "896:1"]
pub unsafe extern "C" fn __kdb2_pgout_routine(mut pg_cookie:
                                                  *mut libc::c_void,
                                              mut pgno: db_pgno_t,
                                              mut page: *mut libc::c_void) {
    let mut hashp: *mut HTAB = 0 as *mut HTAB;
    let mut pagep: *mut PAGE16 = 0 as *mut PAGE16;
    let mut i: int32_t = 0;
    let mut max: int32_t = 0;
    pagep = page as *mut PAGE16;
    hashp = pg_cookie as *mut HTAB;
    /*
	 * There are the following cases for swapping:
	 * 1) Bucket page or overflow page.  Just swap the header.
	 * 2) Bitmap page.  Swap the whole page!
	 * 3) Header pages.  Not handled here; these are written directly
	 *    to the file.
	 */
    if (*hashp).hdr.lorder == 1234 as libc::c_int {
        return
    } /* divide by 4 bytes */
    if is_bitmap_pgno(hashp, pgno) != 0 {
        max = ((*hashp).hdr.bsize >> 2 as libc::c_int) as int32_t;
        i = 0 as libc::c_int;
        while i < max {
            let mut _tmp: u_int32_t =
                *(pagep as *mut libc::c_void as
                      *mut int32_t).offset(i as isize) as u_int32_t;
            *(&mut *(pagep as *mut libc::c_void as
                         *mut int32_t).offset(i as isize) as *mut int32_t as
                  *mut libc::c_char).offset(0 as libc::c_int as isize) =
                *(&mut _tmp as *mut u_int32_t as
                      *mut libc::c_char).offset(3 as libc::c_int as isize);
            *(&mut *(pagep as *mut libc::c_void as
                         *mut int32_t).offset(i as isize) as *mut int32_t as
                  *mut libc::c_char).offset(1 as libc::c_int as isize) =
                *(&mut _tmp as *mut u_int32_t as
                      *mut libc::c_char).offset(2 as libc::c_int as isize);
            *(&mut *(pagep as *mut libc::c_void as
                         *mut int32_t).offset(i as isize) as *mut int32_t as
                  *mut libc::c_char).offset(2 as libc::c_int as isize) =
                *(&mut _tmp as *mut u_int32_t as
                      *mut libc::c_char).offset(1 as libc::c_int as isize);
            *(&mut *(pagep as *mut libc::c_void as
                         *mut int32_t).offset(i as isize) as *mut int32_t as
                  *mut libc::c_char).offset(3 as libc::c_int as isize) =
                *(&mut _tmp as *mut u_int32_t as
                      *mut libc::c_char).offset(0 as libc::c_int as isize);
            i += 1
        }
    } else { swap_page_header_out(pagep); };
}
/*
 *
 * Returns:
 *       0 ==> OK
 *      -1 ==>failure
 */
#[no_mangle]
#[c2rust::src_loc = "933:1"]
pub unsafe extern "C" fn __kdb2_put_page(mut hashp: *mut HTAB,
                                         mut pagep: *mut PAGE16,
                                         mut addr_type: int32_t,
                                         mut is_dirty: int32_t) -> int32_t {
    return kdb2_mpool_put((*hashp).mp, pagep as *mut libc::c_void,
                          if is_dirty != 0 {
                              0x1 as libc::c_int
                          } else { 0 as libc::c_int } as u_int);
}
/*
 * Returns:
 *       0 indicates SUCCESS
 *      -1 indicates FAILURE
 */
#[no_mangle]
#[c2rust::src_loc = "952:1"]
pub unsafe extern "C" fn __kdb2_get_page(mut hashp: *mut HTAB,
                                         mut addr: u_int32_t,
                                         mut addr_type: int32_t)
 -> *mut PAGE16 {
    let mut pagep: *mut PAGE16 = 0 as *mut PAGE16;
    let mut paddr: db_pgno_t = 0;
    match addr_type {
        0 => {
            /* Convert page number. */
            paddr =
                addr.wrapping_add((*hashp).hdr.hdrpages).wrapping_add((if addr
                                                                              !=
                                                                              0
                                                                          {
                                                                           (*hashp).hdr.spares[__kdb2_log2(addr.wrapping_add(1
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 as
                                                                                                                                 libc::c_uint)).wrapping_sub(1
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_int
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_uint)
                                                                                                   as
                                                                                                   usize]
                                                                       } else {
                                                                           0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint
                                                                       }))
        }
        1 | 2 => {
            paddr =
                ((((1 as libc::c_int) << (addr >> 11 as libc::c_int)) -
                      1 as libc::c_int) as
                     libc::c_uint).wrapping_add((*hashp).hdr.hdrpages).wrapping_add((if ((1
                                                                                              as
                                                                                              libc::c_int)
                                                                                             <<
                                                                                             (addr
                                                                                                  >>
                                                                                                  11
                                                                                                      as
                                                                                                      libc::c_int))
                                                                                            -
                                                                                            1
                                                                                                as
                                                                                                libc::c_int
                                                                                            !=
                                                                                            0
                                                                                        {
                                                                                         (*hashp).hdr.spares[__kdb2_log2((((1
                                                                                                                                as
                                                                                                                                libc::c_int)
                                                                                                                               <<
                                                                                                                               (addr
                                                                                                                                    >>
                                                                                                                                    11
                                                                                                                                        as
                                                                                                                                        libc::c_int))
                                                                                                                              -
                                                                                                                              1
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                              +
                                                                                                                              1
                                                                                                                                  as
                                                                                                                                  libc::c_int)
                                                                                                                             as
                                                                                                                             u_int32_t).wrapping_sub(1
                                                                                                                                                         as
                                                                                                                                                         libc::c_int
                                                                                                                                                         as
                                                                                                                                                         libc::c_uint)
                                                                                                                 as
                                                                                                                 usize]
                                                                                     } else {
                                                                                         0
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_uint
                                                                                     })).wrapping_add(addr
                                                                                                          &
                                                                                                          0x7ff
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint)
        }
        _ => { paddr = addr }
    }
    pagep =
        kdb2_mpool_get((*hashp).mp, paddr, 0 as libc::c_int as u_int) as
            *mut PAGE16;
    return pagep;
}
#[c2rust::src_loc = "986:1"]
unsafe extern "C" fn swap_page_header_in(mut pagep: *mut PAGE16) {
    let mut i: u_int32_t = 0;
    /* can leave type and filler alone, since they're 1-byte quantities */
    let mut _tmp: u_int32_t =
        *((pagep as *mut libc::c_void as
               *mut u_int8_t).offset(0 as libc::c_int as isize) as
              *mut libc::c_void as
              *mut db_pgno_t).offset(0 as libc::c_int as isize);
    *(&mut *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(0 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut db_pgno_t).offset(0 as libc::c_int as isize) as
          *mut db_pgno_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(0 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut db_pgno_t).offset(0 as libc::c_int as isize) as
          *mut db_pgno_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(0 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut db_pgno_t).offset(0 as libc::c_int as isize) as
          *mut db_pgno_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(0 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut db_pgno_t).offset(0 as libc::c_int as isize) as
          *mut db_pgno_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_0: u_int32_t =
        *((pagep as *mut libc::c_void as
               *mut u_int8_t).offset(4 as libc::c_int as isize) as
              *mut libc::c_void as
              *mut db_pgno_t).offset(0 as libc::c_int as isize);
    *(&mut *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(4 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut db_pgno_t).offset(0 as libc::c_int as isize) as
          *mut db_pgno_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_0 as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(4 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut db_pgno_t).offset(0 as libc::c_int as isize) as
          *mut db_pgno_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_0 as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(4 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut db_pgno_t).offset(0 as libc::c_int as isize) as
          *mut db_pgno_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp_0 as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(4 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut db_pgno_t).offset(0 as libc::c_int as isize) as
          *mut db_pgno_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp_0 as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_1: u_int16_t =
        *((pagep as *mut libc::c_void as
               *mut u_int8_t).offset(8 as libc::c_int as isize) as
              *mut libc::c_void as
              *mut indx_t).offset(0 as libc::c_int as isize);
    *(&mut *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(8 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut indx_t).offset(0 as libc::c_int as isize) as *mut indx_t
          as *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_1 as *mut u_int16_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(8 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut indx_t).offset(0 as libc::c_int as isize) as *mut indx_t
          as *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_1 as *mut u_int16_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_2: u_int16_t =
        *((pagep as *mut libc::c_void as
               *mut u_int8_t).offset(12 as libc::c_int as isize) as
              *mut libc::c_void as
              *mut indx_t).offset(0 as libc::c_int as isize);
    *(&mut *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(12 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut indx_t).offset(0 as libc::c_int as isize) as *mut indx_t
          as *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_2 as *mut u_int16_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(12 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut indx_t).offset(0 as libc::c_int as isize) as *mut indx_t
          as *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_2 as *mut u_int16_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int as u_int32_t;
    while i <
              *((pagep as *mut libc::c_void as
                     *mut u_int8_t).offset(8 as libc::c_int as isize) as
                    *mut libc::c_void as
                    *mut indx_t).offset(0 as libc::c_int as isize) as
                  libc::c_uint {
        let mut _tmp_3: u_int16_t =
            *((pagep as *mut libc::c_void as
                   *mut u_int8_t).offset((12 as libc::c_int as
                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                              as
                                                                              libc::c_ulong)
                                             as
                                             isize).offset((i as
                                                                libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                 as
                                                                                                 libc::c_ulong)
                                                                                                <<
                                                                                                1
                                                                                                    as
                                                                                                    libc::c_int)
                                                               as isize) as
                  *mut libc::c_void as
                  *mut indx_t).offset(0 as libc::c_int as isize);
        *(&mut *((pagep as *mut libc::c_void as
                      *mut u_int8_t).offset((12 as libc::c_int as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                 as
                                                                                 libc::c_ulong)
                                                as
                                                isize).offset((i as
                                                                   libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                    as
                                                                                                    libc::c_ulong)
                                                                                                   <<
                                                                                                   1
                                                                                                       as
                                                                                                       libc::c_int)
                                                                  as isize) as
                     *mut libc::c_void as
                     *mut indx_t).offset(0 as libc::c_int as isize) as
              *mut indx_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize) =
            *(&mut _tmp_3 as *mut u_int16_t as
                  *mut libc::c_char).offset(1 as libc::c_int as isize);
        *(&mut *((pagep as *mut libc::c_void as
                      *mut u_int8_t).offset((12 as libc::c_int as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                 as
                                                                                 libc::c_ulong)
                                                as
                                                isize).offset((i as
                                                                   libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                    as
                                                                                                    libc::c_ulong)
                                                                                                   <<
                                                                                                   1
                                                                                                       as
                                                                                                       libc::c_int)
                                                                  as isize) as
                     *mut libc::c_void as
                     *mut indx_t).offset(0 as libc::c_int as isize) as
              *mut indx_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize) =
            *(&mut _tmp_3 as *mut u_int16_t as
                  *mut libc::c_char).offset(0 as libc::c_int as isize);
        let mut _tmp_4: u_int16_t =
            *((pagep as *mut libc::c_void as
                   *mut u_int8_t).offset((12 as libc::c_int as
                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                              as
                                                                              libc::c_ulong)
                                             as
                                             isize).offset((i as
                                                                libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                 as
                                                                                                 libc::c_ulong)
                                                                                                <<
                                                                                                1
                                                                                                    as
                                                                                                    libc::c_int)
                                                               as
                                                               isize).offset(::std::mem::size_of::<indx_t>()
                                                                                 as
                                                                                 libc::c_ulong
                                                                                 as
                                                                                 isize)
                  as *mut libc::c_void as
                  *mut indx_t).offset(0 as libc::c_int as isize);
        *(&mut *((pagep as *mut libc::c_void as
                      *mut u_int8_t).offset((12 as libc::c_int as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                 as
                                                                                 libc::c_ulong)
                                                as
                                                isize).offset((i as
                                                                   libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                    as
                                                                                                    libc::c_ulong)
                                                                                                   <<
                                                                                                   1
                                                                                                       as
                                                                                                       libc::c_int)
                                                                  as
                                                                  isize).offset(::std::mem::size_of::<indx_t>()
                                                                                    as
                                                                                    libc::c_ulong
                                                                                    as
                                                                                    isize)
                     as *mut libc::c_void as
                     *mut indx_t).offset(0 as libc::c_int as isize) as
              *mut indx_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize) =
            *(&mut _tmp_4 as *mut u_int16_t as
                  *mut libc::c_char).offset(1 as libc::c_int as isize);
        *(&mut *((pagep as *mut libc::c_void as
                      *mut u_int8_t).offset((12 as libc::c_int as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                 as
                                                                                 libc::c_ulong)
                                                as
                                                isize).offset((i as
                                                                   libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                    as
                                                                                                    libc::c_ulong)
                                                                                                   <<
                                                                                                   1
                                                                                                       as
                                                                                                       libc::c_int)
                                                                  as
                                                                  isize).offset(::std::mem::size_of::<indx_t>()
                                                                                    as
                                                                                    libc::c_ulong
                                                                                    as
                                                                                    isize)
                     as *mut libc::c_void as
                     *mut indx_t).offset(0 as libc::c_int as isize) as
              *mut indx_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize) =
            *(&mut _tmp_4 as *mut u_int16_t as
                  *mut libc::c_char).offset(0 as libc::c_int as isize);
        i = i.wrapping_add(1)
    };
}
#[c2rust::src_loc = "1005:1"]
unsafe extern "C" fn swap_page_header_out(mut pagep: *mut PAGE16) {
    let mut i: u_int32_t = 0;
    i = 0 as libc::c_int as u_int32_t;
    while i <
              *((pagep as *mut libc::c_void as
                     *mut u_int8_t).offset(8 as libc::c_int as isize) as
                    *mut libc::c_void as
                    *mut indx_t).offset(0 as libc::c_int as isize) as
                  libc::c_uint {
        let mut _tmp: u_int16_t =
            *((pagep as *mut libc::c_void as
                   *mut u_int8_t).offset((12 as libc::c_int as
                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                              as
                                                                              libc::c_ulong)
                                             as
                                             isize).offset((i as
                                                                libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                 as
                                                                                                 libc::c_ulong)
                                                                                                <<
                                                                                                1
                                                                                                    as
                                                                                                    libc::c_int)
                                                               as isize) as
                  *mut libc::c_void as
                  *mut indx_t).offset(0 as libc::c_int as isize);
        *(&mut *((pagep as *mut libc::c_void as
                      *mut u_int8_t).offset((12 as libc::c_int as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                 as
                                                                                 libc::c_ulong)
                                                as
                                                isize).offset((i as
                                                                   libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                    as
                                                                                                    libc::c_ulong)
                                                                                                   <<
                                                                                                   1
                                                                                                       as
                                                                                                       libc::c_int)
                                                                  as isize) as
                     *mut libc::c_void as
                     *mut indx_t).offset(0 as libc::c_int as isize) as
              *mut indx_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize) =
            *(&mut _tmp as *mut u_int16_t as
                  *mut libc::c_char).offset(1 as libc::c_int as isize);
        *(&mut *((pagep as *mut libc::c_void as
                      *mut u_int8_t).offset((12 as libc::c_int as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                 as
                                                                                 libc::c_ulong)
                                                as
                                                isize).offset((i as
                                                                   libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                    as
                                                                                                    libc::c_ulong)
                                                                                                   <<
                                                                                                   1
                                                                                                       as
                                                                                                       libc::c_int)
                                                                  as isize) as
                     *mut libc::c_void as
                     *mut indx_t).offset(0 as libc::c_int as isize) as
              *mut indx_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize) =
            *(&mut _tmp as *mut u_int16_t as
                  *mut libc::c_char).offset(0 as libc::c_int as isize);
        let mut _tmp_0: u_int16_t =
            *((pagep as *mut libc::c_void as
                   *mut u_int8_t).offset((12 as libc::c_int as
                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                              as
                                                                              libc::c_ulong)
                                             as
                                             isize).offset((i as
                                                                libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                 as
                                                                                                 libc::c_ulong)
                                                                                                <<
                                                                                                1
                                                                                                    as
                                                                                                    libc::c_int)
                                                               as
                                                               isize).offset(::std::mem::size_of::<indx_t>()
                                                                                 as
                                                                                 libc::c_ulong
                                                                                 as
                                                                                 isize)
                  as *mut libc::c_void as
                  *mut indx_t).offset(0 as libc::c_int as isize);
        *(&mut *((pagep as *mut libc::c_void as
                      *mut u_int8_t).offset((12 as libc::c_int as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                 as
                                                                                 libc::c_ulong)
                                                as
                                                isize).offset((i as
                                                                   libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                    as
                                                                                                    libc::c_ulong)
                                                                                                   <<
                                                                                                   1
                                                                                                       as
                                                                                                       libc::c_int)
                                                                  as
                                                                  isize).offset(::std::mem::size_of::<indx_t>()
                                                                                    as
                                                                                    libc::c_ulong
                                                                                    as
                                                                                    isize)
                     as *mut libc::c_void as
                     *mut indx_t).offset(0 as libc::c_int as isize) as
              *mut indx_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize) =
            *(&mut _tmp_0 as *mut u_int16_t as
                  *mut libc::c_char).offset(1 as libc::c_int as isize);
        *(&mut *((pagep as *mut libc::c_void as
                      *mut u_int8_t).offset((12 as libc::c_int as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                 as
                                                                                 libc::c_ulong)
                                                as
                                                isize).offset((i as
                                                                   libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                                    as
                                                                                                    libc::c_ulong)
                                                                                                   <<
                                                                                                   1
                                                                                                       as
                                                                                                       libc::c_int)
                                                                  as
                                                                  isize).offset(::std::mem::size_of::<indx_t>()
                                                                                    as
                                                                                    libc::c_ulong
                                                                                    as
                                                                                    isize)
                     as *mut libc::c_void as
                     *mut indx_t).offset(0 as libc::c_int as isize) as
              *mut indx_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize) =
            *(&mut _tmp_0 as *mut u_int16_t as
                  *mut libc::c_char).offset(0 as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
    /* can leave type and filler alone, since they're 1-byte quantities */
    let mut _tmp_1: u_int32_t =
        *((pagep as *mut libc::c_void as
               *mut u_int8_t).offset(0 as libc::c_int as isize) as
              *mut libc::c_void as
              *mut db_pgno_t).offset(0 as libc::c_int as isize);
    *(&mut *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(0 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut db_pgno_t).offset(0 as libc::c_int as isize) as
          *mut db_pgno_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_1 as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(0 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut db_pgno_t).offset(0 as libc::c_int as isize) as
          *mut db_pgno_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_1 as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(0 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut db_pgno_t).offset(0 as libc::c_int as isize) as
          *mut db_pgno_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp_1 as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(0 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut db_pgno_t).offset(0 as libc::c_int as isize) as
          *mut db_pgno_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp_1 as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_2: u_int32_t =
        *((pagep as *mut libc::c_void as
               *mut u_int8_t).offset(4 as libc::c_int as isize) as
              *mut libc::c_void as
              *mut db_pgno_t).offset(0 as libc::c_int as isize);
    *(&mut *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(4 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut db_pgno_t).offset(0 as libc::c_int as isize) as
          *mut db_pgno_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_2 as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(4 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut db_pgno_t).offset(0 as libc::c_int as isize) as
          *mut db_pgno_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_2 as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(4 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut db_pgno_t).offset(0 as libc::c_int as isize) as
          *mut db_pgno_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp_2 as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(4 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut db_pgno_t).offset(0 as libc::c_int as isize) as
          *mut db_pgno_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp_2 as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_3: u_int16_t =
        *((pagep as *mut libc::c_void as
               *mut u_int8_t).offset(8 as libc::c_int as isize) as
              *mut libc::c_void as
              *mut indx_t).offset(0 as libc::c_int as isize);
    *(&mut *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(8 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut indx_t).offset(0 as libc::c_int as isize) as *mut indx_t
          as *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_3 as *mut u_int16_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(8 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut indx_t).offset(0 as libc::c_int as isize) as *mut indx_t
          as *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_3 as *mut u_int16_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_4: u_int16_t =
        *((pagep as *mut libc::c_void as
               *mut u_int8_t).offset(12 as libc::c_int as isize) as
              *mut libc::c_void as
              *mut indx_t).offset(0 as libc::c_int as isize);
    *(&mut *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(12 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut indx_t).offset(0 as libc::c_int as isize) as *mut indx_t
          as *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_4 as *mut u_int16_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(12 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut indx_t).offset(0 as libc::c_int as isize) as *mut indx_t
          as *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_4 as *mut u_int16_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
}
/*
 * Initialize a new bitmap page.  Bitmap pages are left in memory
 * once they are read in.
 */
#[no_mangle]
#[c2rust::src_loc = "1029:1"]
pub unsafe extern "C" fn __kdb2_ibitmap(mut hashp: *mut HTAB,
                                        mut pnum: int32_t, mut nbits: int32_t,
                                        mut ndx: int32_t) -> int32_t {
    let mut ip: *mut u_int32_t = 0 as *mut u_int32_t;
    let mut clearbytes: int32_t = 0;
    let mut clearints: int32_t = 0;
    /* make a new bitmap page */
    if __kdb2_new_page(hashp, pnum as u_int32_t, 2 as libc::c_int) !=
           0 as libc::c_int {
        return 1 as libc::c_int
    }
    ip =
        __kdb2_get_page(hashp, pnum as u_int32_t, 2 as libc::c_int) as
            *mut libc::c_void as *mut u_int32_t;
    if ip.is_null() { return 1 as libc::c_int }
    (*hashp).nmaps += 1;
    clearints =
        (nbits - 1 as libc::c_int >> 5 as libc::c_int) + 1 as libc::c_int;
    clearbytes = clearints << 2 as libc::c_int;
    memset(ip as *mut int8_t as *mut libc::c_void, 0 as libc::c_int,
           clearbytes as libc::c_ulong);
    memset((ip as *mut int8_t).offset(clearbytes as isize) as
               *mut libc::c_void, 0xff as libc::c_int,
           (*hashp).hdr.bsize.wrapping_sub(clearbytes as libc::c_uint) as
               libc::c_ulong);
    *ip.offset((clearints - 1 as libc::c_int) as isize) =
        (0xffffffff as libc::c_uint) <<
            (nbits &
                 ((1 as libc::c_int) << 5 as libc::c_int) - 1 as libc::c_int);
    let ref mut fresh1 =
        *ip.offset((0 as libc::c_int / 32 as libc::c_int) as isize);
    *fresh1 |=
        ((1 as libc::c_int) << 0 as libc::c_int % 32 as libc::c_int) as
            libc::c_uint;
    (*hashp).hdr.bitmaps[ndx as usize] = pnum as u_int16_t;
    (*hashp).mapp[ndx as usize] = ip;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1055:1"]
unsafe extern "C" fn first_free(mut map: u_int32_t) -> u_int32_t {
    let mut i: u_int32_t = 0;
    let mut mask: u_int32_t = 0;
    mask = 0x1 as libc::c_int as u_int32_t;
    i = 0 as libc::c_int as u_int32_t;
    while i < 32 as libc::c_int as libc::c_uint {
        if mask & map == 0 { return i }
        mask = mask << 1 as libc::c_int;
        i = i.wrapping_add(1)
    }
    return i;
}
/*
 * returns 0 on error
 */
#[c2rust::src_loc = "1072:1"]
unsafe extern "C" fn overflow_page(mut hashp: *mut HTAB) -> u_int16_t {
    let mut current_block: u64;
    let mut freep: *mut u_int32_t = 0 as *mut u_int32_t;
    let mut bit: u_int32_t = 0;
    let mut first_page: u_int32_t = 0;
    let mut free_bit: u_int32_t = 0;
    let mut free_page: u_int32_t = 0;
    let mut i: u_int32_t = 0;
    let mut in_use_bits: u_int32_t = 0;
    let mut j: u_int32_t = 0;
    let mut max_free: u_int32_t = 0;
    let mut offset: u_int32_t = 0;
    let mut splitnum: u_int32_t = 0;
    let mut addr: u_int16_t = 0;
    splitnum = (*hashp).hdr.ovfl_point as u_int32_t;
    max_free = (*hashp).hdr.spares[splitnum as usize];
    free_page =
        max_free.wrapping_sub(1 as libc::c_int as libc::c_uint) >>
            (*hashp).hdr.bshift + 3 as libc::c_int;
    free_bit =
        max_free.wrapping_sub(1 as libc::c_int as libc::c_uint) &
            ((*hashp).hdr.bsize <<
                 3 as
                     libc::c_int).wrapping_sub(1 as libc::c_int as
                                                   libc::c_uint);
    /*
	 * Look through all the free maps to find the first free block.
 	 * The compiler under -Wall will complain that freep may be used
	 * before being set, however, this loop will ALWAYS get executed
	 * at least once, so freep is guaranteed to be set.
	 */
    freep = 0 as *mut u_int32_t;
    first_page =
        (*hashp).hdr.last_freed >> (*hashp).hdr.bshift + 3 as libc::c_int;
    i = first_page;
    's_39:
        loop  {
            if !(i <= free_page) {
                current_block = 11298138898191919651;
                break ;
            }
            freep = fetch_bitmap(hashp, i as int32_t);
            if freep.is_null() { return 0 as libc::c_int as u_int16_t }
            if i == free_page {
                in_use_bits = free_bit
            } else {
                in_use_bits =
                    ((*hashp).hdr.bsize <<
                         3 as
                             libc::c_int).wrapping_sub(1 as libc::c_int as
                                                           libc::c_uint)
            }
            if i == first_page {
                bit =
                    (*hashp).hdr.last_freed &
                        ((*hashp).hdr.bsize <<
                             3 as
                                 libc::c_int).wrapping_sub(1 as libc::c_int as
                                                               libc::c_uint);
                j = bit.wrapping_div(32 as libc::c_int as libc::c_uint);
                bit =
                    bit &
                        !(32 as libc::c_int - 1 as libc::c_int) as
                            libc::c_uint
            } else {
                bit = 0 as libc::c_int as u_int32_t;
                j = 0 as libc::c_int as u_int32_t
            }
            while bit <= in_use_bits {
                if *freep.offset(j as isize) != 0xffffffff as libc::c_uint {
                    current_block = 1403743547856234815;
                    break 's_39 ;
                }
                j = j.wrapping_add(1);
                bit =
                    (bit as
                         libc::c_uint).wrapping_add(32 as libc::c_int as
                                                        libc::c_uint) as
                        u_int32_t as u_int32_t
            }
            i = i.wrapping_add(1)
        }
    match current_block {
        1403743547856234815 => {
            bit = bit.wrapping_add(first_free(*freep.offset(j as isize)));
            let ref mut fresh3 =
                *freep.offset(bit.wrapping_div(32 as libc::c_int as
                                                   libc::c_uint) as isize);
            *fresh3 |=
                ((1 as libc::c_int) <<
                     bit.wrapping_rem(32 as libc::c_int as libc::c_uint)) as
                    libc::c_uint;
            /*
	 * Bits are addressed starting with 0, but overflow pages are addressed
	 * beginning at 1. Bit is a bit address number, so we need to increment
	 * it to convert it to a page number.
	 */
            bit =
                (1 as libc::c_int as
                     libc::c_uint).wrapping_add(bit).wrapping_add(i.wrapping_mul((*hashp).hdr.bsize
                                                                                     <<
                                                                                     3
                                                                                         as
                                                                                         libc::c_int));
            if bit >= (*hashp).hdr.last_freed {
                (*hashp).hdr.last_freed =
                    bit.wrapping_sub(1 as libc::c_int as libc::c_uint)
            }
            /* Calculate the split number for this page */
            i = 0 as libc::c_int as u_int32_t; /* Out of overflow pages */
            while i < splitnum && bit > (*hashp).hdr.spares[i as usize] {
                i = i.wrapping_add(1)
            }
            offset =
                if i != 0 {
                    bit.wrapping_sub((*hashp).hdr.spares[i.wrapping_sub(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
                                                             as usize])
                } else { bit };
            if offset >= 0x7ff as libc::c_int as libc::c_uint {
                return 0 as libc::c_int as u_int16_t
            }
            addr = (i << 11 as libc::c_int).wrapping_add(offset) as u_int16_t;
            if ((((1 as libc::c_int) <<
                      (addr as u_int32_t >> 11 as libc::c_int)) -
                     1 as libc::c_int) as
                    libc::c_uint).wrapping_add((*hashp).hdr.hdrpages).wrapping_add((if ((1
                                                                                             as
                                                                                             libc::c_int)
                                                                                            <<
                                                                                            (addr
                                                                                                 as
                                                                                                 u_int32_t
                                                                                                 >>
                                                                                                 11
                                                                                                     as
                                                                                                     libc::c_int))
                                                                                           -
                                                                                           1
                                                                                               as
                                                                                               libc::c_int
                                                                                           !=
                                                                                           0
                                                                                       {
                                                                                        (*hashp).hdr.spares[__kdb2_log2((((1
                                                                                                                               as
                                                                                                                               libc::c_int)
                                                                                                                              <<
                                                                                                                              (addr
                                                                                                                                   as
                                                                                                                                   u_int32_t
                                                                                                                                   >>
                                                                                                                                   11
                                                                                                                                       as
                                                                                                                                       libc::c_int))
                                                                                                                             -
                                                                                                                             1
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                             +
                                                                                                                             1
                                                                                                                                 as
                                                                                                                                 libc::c_int)
                                                                                                                            as
                                                                                                                            u_int32_t).wrapping_sub(1
                                                                                                                                                        as
                                                                                                                                                        libc::c_int
                                                                                                                                                        as
                                                                                                                                                        libc::c_uint)
                                                                                                                as
                                                                                                                usize]
                                                                                    } else {
                                                                                        0
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint
                                                                                    })).wrapping_add((addr
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          &
                                                                                                          0x7ff
                                                                                                              as
                                                                                                              libc::c_int)
                                                                                                         as
                                                                                                         libc::c_uint)
                   as libc::c_long >
                   9223372036854775807 as libc::c_long /
                       (*hashp).hdr.bsize as libc::c_long {
                write(2 as libc::c_int,
                      b"HASH: Out of overflow pages.  Increase page size\n\x00"
                          as *const u8 as *const libc::c_char as
                          *const libc::c_void,
                      (::std::mem::size_of::<[libc::c_char; 50]>() as
                           libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                           libc::c_ulong));
                return 0 as libc::c_int as u_int16_t
            }
            /* Allocate and return the overflow page */
            return addr
        }
        _ => {
            /* No Free Page Found */
            (*hashp).hdr.last_freed = (*hashp).hdr.spares[splitnum as usize];
            (*hashp).hdr.spares[splitnum as usize] =
                (*hashp).hdr.spares[splitnum as usize].wrapping_add(1);
            offset =
                (*hashp).hdr.spares[splitnum as
                                        usize].wrapping_sub((if splitnum != 0
                                                                {
                                                                 (*hashp).hdr.spares[splitnum.wrapping_sub(1
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_uint)
                                                                                         as
                                                                                         usize]
                                                             } else {
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint
                                                             }));
            if offset > 0x7ff as libc::c_int as libc::c_uint {
                splitnum = splitnum.wrapping_add(1);
                if splitnum >= 32 as libc::c_int as libc::c_uint {
                    write(2 as libc::c_int,
                          b"HASH: Out of overflow pages.  Increase page size\n\x00"
                              as *const u8 as *const libc::c_char as
                              *const libc::c_void,
                          (::std::mem::size_of::<[libc::c_char; 50]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong));
                    return 0 as libc::c_int as u_int16_t
                }
                (*hashp).hdr.ovfl_point = splitnum as int32_t;
                (*hashp).hdr.spares[splitnum as usize] =
                    (*hashp).hdr.spares[splitnum.wrapping_sub(1 as libc::c_int
                                                                  as
                                                                  libc::c_uint)
                                            as usize];
                (*hashp).hdr.spares[splitnum.wrapping_sub(1 as libc::c_int as
                                                              libc::c_uint) as
                                        usize] =
                    (*hashp).hdr.spares[splitnum.wrapping_sub(1 as libc::c_int
                                                                  as
                                                                  libc::c_uint)
                                            as usize].wrapping_sub(1);
                offset = 1 as libc::c_int as u_int32_t
            }
            /* Check if we need to allocate a new bitmap page. */
            if free_bit ==
                   ((*hashp).hdr.bsize <<
                        3 as
                            libc::c_int).wrapping_sub(1 as libc::c_int as
                                                          libc::c_uint) {
                free_page = free_page.wrapping_add(1);
                if free_page >= 32 as libc::c_int as libc::c_uint {
                    write(2 as libc::c_int,
                          b"HASH: Out of overflow pages.  Increase page size\n\x00"
                              as *const u8 as *const libc::c_char as
                              *const libc::c_void,
                          (::std::mem::size_of::<[libc::c_char; 50]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong));
                    return 0 as libc::c_int as u_int16_t
                }
                /*
		 * This is tricky.  The 1 indicates that you want the new page
		 * allocated with 1 clear bit.  Actually, you are going to
		 * allocate 2 pages from this map.  The first is going to be
		 * the map page, the second is the overflow page we were
		 * looking for.  The __ibitmap routine automatically, sets
		 * the first bit of itself to indicate that the bitmap itself
		 * is in use.  We would explicitly set the second bit, but
		 * don't have to if we tell __ibitmap not to leave it clear
		 * in the first place.
		 */
                if __kdb2_ibitmap(hashp,
                                  (splitnum <<
                                       11 as libc::c_int).wrapping_add(offset)
                                      as int32_t, 1 as libc::c_int,
                                  free_page as int32_t) != 0 {
                    return 0 as libc::c_int as u_int16_t
                }
                (*hashp).hdr.spares[splitnum as usize] =
                    (*hashp).hdr.spares[splitnum as usize].wrapping_add(1);
                offset = offset.wrapping_add(1);
                if offset > 0x7ff as libc::c_int as libc::c_uint {
                    splitnum = splitnum.wrapping_add(1);
                    if splitnum >= 32 as libc::c_int as libc::c_uint {
                        write(2 as libc::c_int,
                              b"HASH: Out of overflow pages.  Increase page size\n\x00"
                                  as *const u8 as *const libc::c_char as
                                  *const libc::c_void,
                              (::std::mem::size_of::<[libc::c_char; 50]>() as
                                   libc::c_ulong).wrapping_sub(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong));
                        return 0 as libc::c_int as u_int16_t
                    }
                    (*hashp).hdr.ovfl_point = splitnum as int32_t;
                    (*hashp).hdr.spares[splitnum as usize] =
                        (*hashp).hdr.spares[splitnum.wrapping_sub(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                                                as usize];
                    (*hashp).hdr.spares[splitnum.wrapping_sub(1 as libc::c_int
                                                                  as
                                                                  libc::c_uint)
                                            as usize] =
                        (*hashp).hdr.spares[splitnum.wrapping_sub(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                                                as usize].wrapping_sub(1);
                    offset = 0 as libc::c_int as u_int32_t
                }
            } else {
                /*
		 * Free_bit addresses the last used bit.  Bump it to address
		 * the first available bit.
		 */
                free_bit = free_bit.wrapping_add(1);
                let ref mut fresh2 =
                    *freep.offset(free_bit.wrapping_div(32 as libc::c_int as
                                                            libc::c_uint) as
                                      isize);
                *fresh2 |=
                    ((1 as libc::c_int) <<
                         free_bit.wrapping_rem(32 as libc::c_int as
                                                   libc::c_uint)) as
                        libc::c_uint
            }
            /* Calculate address of the new overflow page */
            addr =
                (splitnum << 11 as libc::c_int).wrapping_add(offset) as
                    u_int16_t;
            if ((((1 as libc::c_int) <<
                      (addr as u_int32_t >> 11 as libc::c_int)) -
                     1 as libc::c_int) as
                    libc::c_uint).wrapping_add((*hashp).hdr.hdrpages).wrapping_add((if ((1
                                                                                             as
                                                                                             libc::c_int)
                                                                                            <<
                                                                                            (addr
                                                                                                 as
                                                                                                 u_int32_t
                                                                                                 >>
                                                                                                 11
                                                                                                     as
                                                                                                     libc::c_int))
                                                                                           -
                                                                                           1
                                                                                               as
                                                                                               libc::c_int
                                                                                           !=
                                                                                           0
                                                                                       {
                                                                                        (*hashp).hdr.spares[__kdb2_log2((((1
                                                                                                                               as
                                                                                                                               libc::c_int)
                                                                                                                              <<
                                                                                                                              (addr
                                                                                                                                   as
                                                                                                                                   u_int32_t
                                                                                                                                   >>
                                                                                                                                   11
                                                                                                                                       as
                                                                                                                                       libc::c_int))
                                                                                                                             -
                                                                                                                             1
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                             +
                                                                                                                             1
                                                                                                                                 as
                                                                                                                                 libc::c_int)
                                                                                                                            as
                                                                                                                            u_int32_t).wrapping_sub(1
                                                                                                                                                        as
                                                                                                                                                        libc::c_int
                                                                                                                                                        as
                                                                                                                                                        libc::c_uint)
                                                                                                                as
                                                                                                                usize]
                                                                                    } else {
                                                                                        0
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint
                                                                                    })).wrapping_add((addr
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          &
                                                                                                          0x7ff
                                                                                                              as
                                                                                                              libc::c_int)
                                                                                                         as
                                                                                                         libc::c_uint)
                   as libc::c_long >
                   9223372036854775807 as libc::c_long /
                       (*hashp).hdr.bsize as libc::c_long {
                write(2 as libc::c_int,
                      b"HASH: Out of overflow pages.  Increase page size\n\x00"
                          as *const u8 as *const libc::c_char as
                          *const libc::c_void,
                      (::std::mem::size_of::<[libc::c_char; 50]>() as
                           libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                           libc::c_ulong));
                return 0 as libc::c_int as u_int16_t
            }
            return addr
        }
    };
}
/* DEBUG */
#[c2rust::src_loc = "1262:1"]
unsafe extern "C" fn page_to_oaddr(mut hashp: *mut HTAB, mut pgno: db_pgno_t)
 -> indx_t {
    let mut sp: int32_t = 0;
    let mut ret_val: int32_t = 0;
    /*
	 * To convert page number to overflow address:
	 *
	 * 1.  Find a starting split point -- use 0 since there are only
	 *     32 split points.
	 * 2.  Find the split point s.t. 2^sp + hdr.spares[sp] < pgno and
	 *     2^(sp+1) = hdr.spares[sp+1] > pgno.  The overflow address will
	 *     be located at sp.
	 * 3.  return...
	 */
    pgno =
        (pgno as libc::c_uint).wrapping_sub((*hashp).hdr.hdrpages) as
            db_pgno_t as db_pgno_t;
    sp = 0 as libc::c_int;
    while sp < 32 as libc::c_int - 1 as libc::c_int {
        if (((1 as libc::c_int) << sp) as
                libc::c_uint).wrapping_add((*hashp).hdr.spares[sp as usize]) <
               pgno &&
               (((1 as libc::c_int) << sp + 1 as libc::c_int) as
                    libc::c_uint).wrapping_add((*hashp).hdr.spares[(sp +
                                                                        1 as
                                                                            libc::c_int)
                                                                       as
                                                                       usize])
                   > pgno {
            break ;
        }
        sp += 1
    }
    ret_val =
        (((sp + 1 as libc::c_int) as u_int32_t) <<
             11 as
                 libc::c_int).wrapping_add(pgno.wrapping_sub(((((1 as
                                                                     libc::c_int)
                                                                    <<
                                                                    sp +
                                                                        1 as
                                                                            libc::c_int)
                                                                   -
                                                                   1 as
                                                                       libc::c_int)
                                                                  as
                                                                  libc::c_uint).wrapping_add((*hashp).hdr.spares[sp
                                                                                                                     as
                                                                                                                     usize])))
            as int32_t;
    return ret_val as indx_t;
}
/*
 * Mark this overflow page as free.
 */
#[no_mangle]
#[c2rust::src_loc = "1296:1"]
pub unsafe extern "C" fn __kdb2_free_ovflpage(mut hashp: *mut HTAB,
                                              mut pagep: *mut PAGE16) {
    let mut freep: *mut u_int32_t = 0 as *mut u_int32_t;
    let mut bit_address: u_int32_t = 0;
    let mut free_page: u_int32_t = 0;
    let mut free_bit: u_int32_t = 0;
    let mut addr: u_int16_t = 0;
    let mut ndx: u_int16_t = 0;
    addr =
        page_to_oaddr(hashp,
                      *((pagep as *mut libc::c_void as
                             *mut u_int8_t).offset(0 as libc::c_int as isize)
                            as *mut libc::c_void as
                            *mut db_pgno_t).offset(0 as libc::c_int as
                                                       isize));
    ndx = (addr as libc::c_int >> 11 as libc::c_int) as u_int16_t;
    bit_address =
        (if ndx as libc::c_int != 0 {
             (*hashp).hdr.spares[(ndx as libc::c_int - 1 as libc::c_int) as
                                     usize]
         } else {
             0 as libc::c_int as libc::c_uint
         }).wrapping_add((addr as libc::c_int & 0x7ff as libc::c_int) as
                             libc::c_uint).wrapping_sub(1 as libc::c_int as
                                                            libc::c_uint);
    if bit_address < (*hashp).hdr.last_freed {
        (*hashp).hdr.last_freed = bit_address
    }
    free_page = bit_address >> (*hashp).hdr.bshift + 3 as libc::c_int;
    free_bit =
        bit_address &
            ((*hashp).hdr.bsize <<
                 3 as
                     libc::c_int).wrapping_sub(1 as libc::c_int as
                                                   libc::c_uint);
    freep = fetch_bitmap(hashp, free_page as int32_t);
    let ref mut fresh4 =
        *freep.offset(free_bit.wrapping_div(32 as libc::c_int as libc::c_uint)
                          as isize);
    *fresh4 &=
        !((1 as libc::c_int) <<
              free_bit.wrapping_rem(32 as libc::c_int as libc::c_uint)) as
            libc::c_uint;
}
#[c2rust::src_loc = "1335:1"]
unsafe extern "C" fn fetch_bitmap(mut hashp: *mut HTAB, mut ndx: int32_t)
 -> *mut u_int32_t {
    if ndx >= (*hashp).nmaps { return 0 as *mut u_int32_t }
    if (*hashp).mapp[ndx as usize].is_null() {
        (*hashp).mapp[ndx as usize] =
            __kdb2_get_page(hashp,
                            (*hashp).hdr.bitmaps[ndx as usize] as u_int32_t,
                            2 as libc::c_int) as *mut libc::c_void as
                *mut u_int32_t
    }
    return (*hashp).mapp[ndx as usize];
}
/* DEBUG_SLOW */
