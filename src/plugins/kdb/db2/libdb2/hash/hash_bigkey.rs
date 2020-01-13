use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:58"]
pub mod types_h {
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "34:1"]
    pub type __u_long = libc::c_ulong;
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = libc::c_schar;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/sys/types.h:58"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "36:1"]
    pub type u_long = __u_long;
    #[c2rust::src_loc = "158:1"]
    pub type u_int8_t = __uint8_t;
    #[c2rust::src_loc = "159:1"]
    pub type u_int16_t = __uint16_t;
    #[c2rust::src_loc = "160:1"]
    pub type u_int32_t = __uint32_t;
    use super::types_h::{__u_int, __u_long, __uint8_t, __uint16_t,
                         __uint32_t};
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:58"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:58"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int8_t, __int32_t};
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
    #[c2rust::src_loc = "83:9"]
    pub type DBTYPE = libc::c_uint;
    #[c2rust::src_loc = "83:35"]
    pub const DB_RECNO: DBTYPE = 2;
    #[c2rust::src_loc = "83:26"]
    pub const DB_HASH: DBTYPE = 1;
    #[c2rust::src_loc = "83:16"]
    pub const DB_BTREE: DBTYPE = 0;
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
    #[c2rust::src_loc = "143:1"]
    pub type db_pgno_t = u_int32_t;
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
    /*
 * There is no portable way to figure out the maximum value of a file
 * offset, so we put it here.
 */
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
    use super::sys_types_h::{u_int8_t, u_long};
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
    /* cursor structure */
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
    /* Hash Table Information */
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
    /* Disk resident portion */
    /* Magic NO for hash tables */
    /* Version ID */
    /* Byte Order */
    /* Bucket/Page Size */
    /* Bucket shift */
    /* Where overflow pages are being allocated */
    /* Last overflow page freed */
    /* ID of Maximum bucket in use */
    /* Mask to modulo into entire table */
    /* Mask to modulo into lower half of table */
    /* Fill factor */
    /* Number of keys in hash table */
    /* Size of table header */
    /* value of hash(CHARKEY) */
    /* number of bit maps and spare points */
    /* spare pages for overflow */
    /* address of overflow page bitmaps */
    /* Memory resident data structure */
    /* Header */
    /* Hash Function */
    /* Flag values */
    /* File pointer */
    /* File path */
    /* Temporary Buffer for BIG data */
    /* Temporary Buffer for BIG keys */
    /* Temporary buffer for splits */
    /* Cursor used for hash_seq */
    /* Error Number -- for DBM compatibility */
    /* Indicates if fd is backing store or no */
    /* Indicates whether we need to flush file at
				 * exit */
    /* Pointers to page maps */
    /* Initial number of bitmaps */
    /* mpool for buffer management */
    /*
 * Constants
 */
    /* 2^16 */
    /* log2(BUCKET) */
    /* log2(SEGSIZE)	 */
    /* Given the address of the beginning of a big map, clear/set the nth bit */
    /* Overflow management */
/*
 * Overflow page numbers are allocated per split point.  At each doubling of
 * the table, we can allocate extra pages.  So, an overflow page number has
 * the top 5 bits indicate which split point and the lower 11 bits indicate
 * which page at that split point is indicated (pages within split points are
 * numberered starting with 1).
 */
    /* Shorthands for accessing structure */
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
    /* for num_items */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/hash/page.h:69"]
pub mod page_h {
    #[c2rust::src_loc = "150:1"]
    pub type PAGE16 = libc::c_ushort;
    #[c2rust::src_loc = "151:1"]
    pub type PAGE8 = libc::c_uchar;
    /*
 * Overhead on header pages is just one word -- the length of the
 * header info stored on that page.
 */
    /*
 * Since these are all unsigned, we need to guarantee that we never go
 * negative.  Offset values are 0-based and overheads are one based (i.e.
 * one byte of overhead is 1, not 0), so we need to convert OFFSETs to
 * 1-based counting before subtraction.
 */
}
#[c2rust::header_src = "/usr/include/stdlib.h:60"]
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
#[c2rust::header_src = "/usr/include/string.h:61"]
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
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/hash/extern.h:70"]
pub mod extern_h {
    use super::hash_h::HTAB;
    use super::page_h::PAGE16;
    use super::db_int_h::indx_t;
    use super::sys_types_h::{u_int8_t, u_int32_t};
    use super::stdint_intn_h::int32_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "69:1"]
        pub fn __kdb2_add_bigpage(_: *mut HTAB, _: *mut PAGE16, _: indx_t,
                                  _: u_int8_t) -> *mut PAGE16;
        #[no_mangle]
        #[c2rust::src_loc = "80:1"]
        pub fn __kdb2_delete_page(_: *mut HTAB, _: *mut PAGE16, _: int32_t)
         -> int32_t;
        #[no_mangle]
        #[c2rust::src_loc = "92:1"]
        pub fn __kdb2_get_page(_: *mut HTAB, _: u_int32_t, _: int32_t)
         -> *mut PAGE16;
        #[no_mangle]
        #[c2rust::src_loc = "94:1"]
        pub fn __kdb2_log2(_: u_int32_t) -> u_int32_t;
        #[no_mangle]
        #[c2rust::src_loc = "99:1"]
        pub fn __kdb2_put_page(_: *mut HTAB, _: *mut PAGE16, _: int32_t,
                               _: int32_t) -> int32_t;
    }
}
pub use self::types_h::{__u_int, __u_long, __int8_t, __uint8_t, __uint16_t,
                        __int32_t, __uint32_t};
pub use self::sys_types_h::{u_int, u_long, u_int8_t, u_int16_t, u_int32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int8_t, int32_t};
pub use self::db_h::{DBT, DBTYPE, DB_RECNO, DB_HASH, DB_BTREE, __db, DB};
pub use self::db_int_h::{db_pgno_t, indx_t};
pub use self::mpool_h::{_bkt, C2RustUnnamed, C2RustUnnamed_0, MPOOL, _hqh,
                        _lqh};
pub use self::hash_h::{cursor_t, C2RustUnnamed_1, CURSOR, hashhdr, HASHHDR,
                       htab, _cursor_queue, HTAB, item_info, ITEM_INFO};
pub use self::page_h::{PAGE16, PAGE8};
use self::stdlib_h::{malloc, free};
use self::string_h::{memcpy, memmove, memcmp};
use self::extern_h::{__kdb2_add_bigpage, __kdb2_delete_page, __kdb2_get_page,
                     __kdb2_log2, __kdb2_put_page};
/*
 * Big_insert
 *
 * You need to do an insert and the key/data pair is greater than
 * MINFILL * the bucket size
 *
 * Returns:
 *	 0 ==> OK
 *	-1 ==> ERROR
 */
#[no_mangle]
#[c2rust::src_loc = "85:1"]
pub unsafe extern "C" fn __kdb2_big_insert(mut hashp: *mut HTAB,
                                           mut pagep: *mut PAGE16,
                                           mut key: *const DBT,
                                           mut val: *const DBT) -> int32_t {
    let mut key_size: size_t = 0;
    let mut val_size: size_t = 0;
    let mut key_move_bytes: indx_t = 0;
    let mut val_move_bytes: indx_t = 0;
    let mut key_data: *mut int8_t = 0 as *mut int8_t;
    let mut val_data: *mut int8_t = 0 as *mut int8_t;
    let mut base_page: int8_t = 0;
    key_data = (*key).data as *mut int8_t;
    key_size = (*key).size;
    val_data = (*val).data as *mut int8_t;
    val_size = (*val).size;
    *((pagep as *mut libc::c_void as
           *mut u_int8_t).offset(8 as libc::c_int as isize) as
          *mut libc::c_void as *mut indx_t).offset(0 as libc::c_int as isize)
        =
        (*((pagep as *mut libc::c_void as
                *mut u_int8_t).offset(8 as libc::c_int as isize) as
               *mut libc::c_void as
               *mut indx_t).offset(0 as libc::c_int as isize) as libc::c_int +
             1 as libc::c_int) as indx_t;
    base_page = 1 as libc::c_int as int8_t;
    while key_size.wrapping_add(val_size) != 0 {
        /* Add a page! */
        pagep =
            __kdb2_add_bigpage(hashp, pagep,
                               (*((pagep as *mut libc::c_void as
                                       *mut u_int8_t).offset(8 as libc::c_int
                                                                 as isize) as
                                      *mut libc::c_void as
                                      *mut indx_t).offset(0 as libc::c_int as
                                                              isize) as
                                    libc::c_int - 1 as libc::c_int) as indx_t,
                               base_page as u_int8_t);
        if pagep.is_null() { return -(1 as libc::c_int) }
        /* There's just going to be one entry on this page. */
        *((pagep as *mut libc::c_void as
               *mut u_int8_t).offset(8 as libc::c_int as isize) as
              *mut libc::c_void as
              *mut indx_t).offset(0 as libc::c_int as isize) =
            1 as libc::c_int as indx_t;
        /* Move the key's data. */
        key_move_bytes =
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
                   < key_size {
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
            } else { key_size } as indx_t;
        /* Mark the page as to how much key & data is on this page. */
        *((pagep as *mut libc::c_void as
               *mut u_int8_t).offset((12 as libc::c_int as
                                          libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                          as
                                                                          libc::c_ulong)
                                         as
                                         isize).offset((0 as libc::c_int as
                                                            libc::c_ulong).wrapping_mul((::std::mem::size_of::<indx_t>()
                                                                                             as
                                                                                             libc::c_ulong)
                                                                                            <<
                                                                                            1
                                                                                                as
                                                                                                libc::c_int)
                                                           as isize) as
              *mut libc::c_void as
              *mut indx_t).offset(0 as libc::c_int as isize) = key_move_bytes;
        val_move_bytes =
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
                                                                                                                                                           libc::c_int)).wrapping_sub(key_move_bytes
                                                                                                                                                                                          as
                                                                                                                                                                                          libc::c_ulong)
                   < val_size {
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
                                                                                                                                                            libc::c_int)).wrapping_sub(key_move_bytes
                                                                                                                                                                                           as
                                                                                                                                                                                           libc::c_ulong)
            } else { val_size } as indx_t;
        *((pagep as *mut libc::c_void as
               *mut u_int8_t).offset((12 as libc::c_int as
                                          libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                          as
                                                                          libc::c_ulong)
                                         as
                                         isize).offset((0 as libc::c_int as
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
              *mut indx_t).offset(0 as libc::c_int as isize) = val_move_bytes;
        /* Note big pages build beginning --> end, not vice versa. */
        if key_move_bytes != 0 {
            memmove((pagep as
                         *mut PAGE8).offset((12 as libc::c_int as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                 as
                                                                                 libc::c_ulong)
                                                as
                                                isize).offset(((::std::mem::size_of::<indx_t>()
                                                                    as
                                                                    libc::c_ulong)
                                                                   <<
                                                                   1 as
                                                                       libc::c_int)
                                                                  as isize) as
                        *mut libc::c_void, key_data as *const libc::c_void,
                    key_move_bytes as libc::c_ulong);
        }
        if val_move_bytes != 0 {
            memmove((pagep as
                         *mut PAGE8).offset((12 as libc::c_int as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                 as
                                                                                 libc::c_ulong)
                                                as
                                                isize).offset(((::std::mem::size_of::<indx_t>()
                                                                    as
                                                                    libc::c_ulong)
                                                                   <<
                                                                   1 as
                                                                       libc::c_int)
                                                                  as
                                                                  isize).offset(*((pagep
                                                                                       as
                                                                                       *mut libc::c_void
                                                                                       as
                                                                                       *mut u_int8_t).offset((12
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                  as
                                                                                                                                                  libc::c_ulong)
                                                                                                                 as
                                                                                                                 isize).offset((0
                                                                                                                                    as
                                                                                                                                    libc::c_int
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
                                                                                      as
                                                                                      *mut libc::c_void
                                                                                      as
                                                                                      *mut indx_t).offset(0
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              isize)
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize)
                        as *mut libc::c_void, val_data as *const libc::c_void,
                    val_move_bytes as libc::c_ulong);
        }
        key_size =
            (key_size as
                 libc::c_ulong).wrapping_sub(key_move_bytes as libc::c_ulong)
                as size_t as size_t;
        key_data = key_data.offset(key_move_bytes as libc::c_int as isize);
        val_size =
            (val_size as
                 libc::c_ulong).wrapping_sub(val_move_bytes as libc::c_ulong)
                as size_t as size_t;
        val_data = val_data.offset(val_move_bytes as libc::c_int as isize);
        base_page = 0 as libc::c_int as int8_t
    }
    __kdb2_put_page(hashp, pagep, 4 as libc::c_int, 1 as libc::c_int);
    return 0 as libc::c_int;
}
/*
 * Called when we need to delete a big pair.
 *
 * Returns:
 *	 0 => OK
 *	-1 => ERROR
 */
#[no_mangle]
#[c2rust::src_loc = "144:1"]
pub unsafe extern "C" fn __kdb2_big_delete(mut hashp: *mut HTAB,
                                           mut pagep: *mut PAGE16,
                                           mut ndx: indx_t) -> int32_t {
    let mut last_pagep: *mut PAGE16 = 0 as *mut PAGE16;
    /* Get first page with big key/data. */
    pagep =
        __kdb2_get_page(hashp,
                        ((((1 as libc::c_int) <<
                               (*((pagep as *mut libc::c_void as
                                       *mut u_int8_t).offset((12 as
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
                                      *mut indx_t).offset(0 as libc::c_int as
                                                              isize) as
                                    u_int32_t >> 11 as libc::c_int)) -
                              1 as libc::c_int) as
                             libc::c_uint).wrapping_add((*hashp).hdr.hdrpages).wrapping_add((if ((1
                                                                                                      as
                                                                                                      libc::c_int)
                                                                                                     <<
                                                                                                     (*((pagep
                                                                                                             as
                                                                                                             *mut libc::c_void
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
                                                                                                            as
                                                                                                            *mut libc::c_void
                                                                                                            as
                                                                                                            *mut indx_t).offset(0
                                                                                                                                    as
                                                                                                                                    libc::c_int
                                                                                                                                    as
                                                                                                                                    isize)
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
                                                                                                                                       (*((pagep
                                                                                                                                               as
                                                                                                                                               *mut libc::c_void
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
                                                                                                                                              as
                                                                                                                                              *mut libc::c_void
                                                                                                                                              as
                                                                                                                                              *mut indx_t).offset(0
                                                                                                                                                                      as
                                                                                                                                                                      libc::c_int
                                                                                                                                                                      as
                                                                                                                                                                      isize)
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
                                                                                             })).wrapping_add((*((pagep
                                                                                                                      as
                                                                                                                      *mut libc::c_void
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
                                                                                                                     as
                                                                                                                     *mut libc::c_void
                                                                                                                     as
                                                                                                                     *mut indx_t).offset(0
                                                                                                                                             as
                                                                                                                                             libc::c_int
                                                                                                                                             as
                                                                                                                                             isize)
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   &
                                                                                                                   0x7ff
                                                                                                                       as
                                                                                                                       libc::c_int)
                                                                                                                  as
                                                                                                                  libc::c_uint),
                        4 as libc::c_int);
    if pagep.is_null() { return -(1 as libc::c_int) }
    /*
	 * Traverse through the pages, freeing the previous one (except
	 * the first) at each new page.
	 */
    while *((pagep as *mut libc::c_void as
                 *mut u_int8_t).offset(4 as libc::c_int as isize) as
                *mut libc::c_void as
                *mut db_pgno_t).offset(0 as libc::c_int as isize) !=
              0xffffffff as libc::c_uint {
        last_pagep = pagep;
        pagep =
            __kdb2_get_page(hashp,
                            *((pagep as *mut libc::c_void as
                                   *mut u_int8_t).offset(4 as libc::c_int as
                                                             isize) as
                                  *mut libc::c_void as
                                  *mut db_pgno_t).offset(0 as libc::c_int as
                                                             isize),
                            4 as libc::c_int);
        if pagep.is_null() { return -(1 as libc::c_int) }
        __kdb2_delete_page(hashp, last_pagep, 1 as libc::c_int);
    }
    /* Free the last page in the chain. */
    __kdb2_delete_page(hashp, pagep, 1 as libc::c_int);
    return 0 as libc::c_int;
}
/*
 * Given a key, indicates whether the big key at cursorp matches the
 * given key.
 *
 * Returns:
 *	 1 = Found!
 *	 0 = Key not found
 *	-1 error
 */
#[no_mangle]
#[c2rust::src_loc = "187:1"]
pub unsafe extern "C" fn __kdb2_find_bigpair(mut hashp: *mut HTAB,
                                             mut cursorp: *mut CURSOR,
                                             mut key: *mut int8_t,
                                             mut size: int32_t) -> int32_t {
    let mut pagep: *mut PAGE16 = 0 as *mut PAGE16;
    let mut hold_pagep: *mut PAGE16 = 0 as *mut PAGE16;
    let mut next_pgno: db_pgno_t = 0;
    let mut ksize: int32_t = 0;
    let mut kkey: *mut int8_t = 0 as *mut int8_t;
    ksize = size;
    kkey = key;
    hold_pagep = 0 as *mut PAGE16;
    /* Chances are, hashp->cpage is the base page. */
    if !(*cursorp).pagep.is_null() {
        hold_pagep = (*cursorp).pagep;
        pagep = hold_pagep
    } else {
        pagep = __kdb2_get_page(hashp, (*cursorp).pgno, 4 as libc::c_int);
        if pagep.is_null() { return -(1 as libc::c_int) }
    }
    /*
	 * Now, get the first page with the big stuff on it.
	 *
	 * XXX
	 * KLUDGE: we know that cursor is looking at the _next_ item, so
	 * we have to look at pgndx - 1.
	 */
    next_pgno =
        ((((1 as libc::c_int) <<
               (*((pagep as *mut libc::c_void as
                       *mut u_int8_t).offset((12 as libc::c_int as
                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                  as
                                                                                  libc::c_ulong)
                                                 as
                                                 isize).offset((((*cursorp).pgndx
                                                                     as
                                                                     libc::c_int
                                                                     -
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
                    u_int32_t >> 11 as libc::c_int)) - 1 as libc::c_int) as
             libc::c_uint).wrapping_add((*hashp).hdr.hdrpages).wrapping_add((if ((1
                                                                                      as
                                                                                      libc::c_int)
                                                                                     <<
                                                                                     (*((pagep
                                                                                             as
                                                                                             *mut libc::c_void
                                                                                             as
                                                                                             *mut u_int8_t).offset((12
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                        as
                                                                                                                                                        libc::c_ulong)
                                                                                                                       as
                                                                                                                       isize).offset((((*cursorp).pgndx
                                                                                                                                           as
                                                                                                                                           libc::c_int
                                                                                                                                           -
                                                                                                                                           1
                                                                                                                                               as
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
                                                                                            as
                                                                                            *mut libc::c_void
                                                                                            as
                                                                                            *mut indx_t).offset(0
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    isize)
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
                                                                                                                       (*((pagep
                                                                                                                               as
                                                                                                                               *mut libc::c_void
                                                                                                                               as
                                                                                                                               *mut u_int8_t).offset((12
                                                                                                                                                          as
                                                                                                                                                          libc::c_int
                                                                                                                                                          as
                                                                                                                                                          libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                          as
                                                                                                                                                                                          libc::c_ulong)
                                                                                                                                                         as
                                                                                                                                                         isize).offset((((*cursorp).pgndx
                                                                                                                                                                             as
                                                                                                                                                                             libc::c_int
                                                                                                                                                                             -
                                                                                                                                                                             1
                                                                                                                                                                                 as
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
                                                                                                                              as
                                                                                                                              *mut libc::c_void
                                                                                                                              as
                                                                                                                              *mut indx_t).offset(0
                                                                                                                                                      as
                                                                                                                                                      libc::c_int
                                                                                                                                                      as
                                                                                                                                                      isize)
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
                                                                             })).wrapping_add((*((pagep
                                                                                                      as
                                                                                                      *mut libc::c_void
                                                                                                      as
                                                                                                      *mut u_int8_t).offset((12
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 as
                                                                                                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_ulong)
                                                                                                                                as
                                                                                                                                isize).offset((((*cursorp).pgndx
                                                                                                                                                    as
                                                                                                                                                    libc::c_int
                                                                                                                                                    -
                                                                                                                                                    1
                                                                                                                                                        as
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
                                                                                                     as
                                                                                                     *mut libc::c_void
                                                                                                     as
                                                                                                     *mut indx_t).offset(0
                                                                                                                             as
                                                                                                                             libc::c_int
                                                                                                                             as
                                                                                                                             isize)
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   &
                                                                                                   0x7ff
                                                                                                       as
                                                                                                       libc::c_int)
                                                                                                  as
                                                                                                  libc::c_uint);
    if hold_pagep.is_null() {
        __kdb2_put_page(hashp, pagep, 4 as libc::c_int, 0 as libc::c_int);
    }
    pagep = __kdb2_get_page(hashp, next_pgno, 4 as libc::c_int);
    if pagep.is_null() { return -(1 as libc::c_int) }
    /* While there are both keys to compare. */
    while ksize > 0 as libc::c_int &&
              *((pagep as *mut libc::c_void as
                     *mut u_int8_t).offset((12 as libc::c_int as
                                                libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                as
                                                                                libc::c_ulong)
                                               as
                                               isize).offset((0 as libc::c_int
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
                    *mut indx_t).offset(0 as libc::c_int as isize) as
                  libc::c_int != 0 {
        if ksize <
               *((pagep as *mut libc::c_void as
                      *mut u_int8_t).offset((12 as libc::c_int as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                 as
                                                                                 libc::c_ulong)
                                                as
                                                isize).offset((0 as
                                                                   libc::c_int
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
                     *mut indx_t).offset(0 as libc::c_int as isize) as
                   libc::c_int ||
               memcmp((pagep as
                           *mut PAGE8).offset((12 as libc::c_int as
                                                   libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                   as
                                                                                   libc::c_ulong)
                                                  as
                                                  isize).offset(((::std::mem::size_of::<indx_t>()
                                                                      as
                                                                      libc::c_ulong)
                                                                     <<
                                                                     1 as
                                                                         libc::c_int)
                                                                    as isize)
                          as *const libc::c_void, kkey as *const libc::c_void,
                      *((pagep as *mut libc::c_void as
                             *mut u_int8_t).offset((12 as libc::c_int as
                                                        libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                        as
                                                                                        libc::c_ulong)
                                                       as
                                                       isize).offset((0 as
                                                                          libc::c_int
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
                          libc::c_ulong) != 0 {
            __kdb2_put_page(hashp, pagep, 4 as libc::c_int, 0 as libc::c_int);
            return 0 as libc::c_int
        }
        kkey =
            kkey.offset(*((pagep as *mut libc::c_void as
                               *mut u_int8_t).offset((12 as libc::c_int as
                                                          libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                          as
                                                                                          libc::c_ulong)
                                                         as
                                                         isize).offset((0 as
                                                                            libc::c_int
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
                              *mut indx_t).offset(0 as libc::c_int as isize)
                            as libc::c_int as isize);
        ksize -=
            *((pagep as *mut libc::c_void as
                   *mut u_int8_t).offset((12 as libc::c_int as
                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                              as
                                                                              libc::c_ulong)
                                             as
                                             isize).offset((0 as libc::c_int
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
                  *mut indx_t).offset(0 as libc::c_int as isize) as
                libc::c_int;
        if *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset(4 as libc::c_int as isize) as
                 *mut libc::c_void as
                 *mut db_pgno_t).offset(0 as libc::c_int as isize) !=
               0xffffffff as libc::c_uint {
            next_pgno =
                *((pagep as *mut libc::c_void as
                       *mut u_int8_t).offset(4 as libc::c_int as isize) as
                      *mut libc::c_void as
                      *mut db_pgno_t).offset(0 as libc::c_int as isize);
            __kdb2_put_page(hashp, pagep, 4 as libc::c_int, 0 as libc::c_int);
            pagep = __kdb2_get_page(hashp, next_pgno, 4 as libc::c_int);
            if pagep.is_null() { return -(1 as libc::c_int) }
        }
    }
    __kdb2_put_page(hashp, pagep, 4 as libc::c_int, 0 as libc::c_int);
    if ksize != 0 as libc::c_int {
        return 0 as libc::c_int
    } else { return 1 as libc::c_int };
}
/*
 * Fill in the key and data for this big pair.
 */
#[no_mangle]
#[c2rust::src_loc = "259:1"]
pub unsafe extern "C" fn __kdb2_big_keydata(mut hashp: *mut HTAB,
                                            mut pagep: *mut PAGE16,
                                            mut key: *mut DBT,
                                            mut val: *mut DBT,
                                            mut ndx: int32_t) -> int32_t {
    let mut ii: ITEM_INFO =
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
    let mut key_pagep: *mut PAGE16 = 0 as *mut PAGE16;
    let mut last_page: db_pgno_t = 0;
    key_pagep =
        __kdb2_get_page(hashp,
                        ((((1 as libc::c_int) <<
                               (*((pagep as *mut libc::c_void as
                                       *mut u_int8_t).offset((12 as
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
                                      *mut indx_t).offset(0 as libc::c_int as
                                                              isize) as
                                    u_int32_t >> 11 as libc::c_int)) -
                              1 as libc::c_int) as
                             libc::c_uint).wrapping_add((*hashp).hdr.hdrpages).wrapping_add((if ((1
                                                                                                      as
                                                                                                      libc::c_int)
                                                                                                     <<
                                                                                                     (*((pagep
                                                                                                             as
                                                                                                             *mut libc::c_void
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
                                                                                                            as
                                                                                                            *mut libc::c_void
                                                                                                            as
                                                                                                            *mut indx_t).offset(0
                                                                                                                                    as
                                                                                                                                    libc::c_int
                                                                                                                                    as
                                                                                                                                    isize)
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
                                                                                                                                       (*((pagep
                                                                                                                                               as
                                                                                                                                               *mut libc::c_void
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
                                                                                                                                              as
                                                                                                                                              *mut libc::c_void
                                                                                                                                              as
                                                                                                                                              *mut indx_t).offset(0
                                                                                                                                                                      as
                                                                                                                                                                      libc::c_int
                                                                                                                                                                      as
                                                                                                                                                                      isize)
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
                                                                                             })).wrapping_add((*((pagep
                                                                                                                      as
                                                                                                                      *mut libc::c_void
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
                                                                                                                     as
                                                                                                                     *mut libc::c_void
                                                                                                                     as
                                                                                                                     *mut indx_t).offset(0
                                                                                                                                             as
                                                                                                                                             libc::c_int
                                                                                                                                             as
                                                                                                                                             isize)
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   &
                                                                                                                   0x7ff
                                                                                                                       as
                                                                                                                       libc::c_int)
                                                                                                                  as
                                                                                                                  libc::c_uint),
                        4 as libc::c_int);
    if key_pagep.is_null() { return -(1 as libc::c_int) }
    (*key).size =
        collect_key(hashp, key_pagep, 0 as libc::c_int, &mut last_page) as
            size_t;
    (*key).data = (*hashp).bigkey_buf as *mut libc::c_void;
    __kdb2_put_page(hashp, key_pagep, 4 as libc::c_int, 0 as libc::c_int);
    if (*key).size == -(1 as libc::c_int) as size_t {
        return -(1 as libc::c_int)
    }
    /* Create an item_info to direct __big_return to the beginning pgno. */
    ii.pgno = last_page;
    return __kdb2_big_return(hashp, &mut ii, val, 1 as libc::c_int);
}
/*
 * Return the big key on page, ndx.
 */
#[no_mangle]
#[c2rust::src_loc = "289:1"]
pub unsafe extern "C" fn __kdb2_get_bigkey(mut hashp: *mut HTAB,
                                           mut pagep: *mut PAGE16,
                                           mut ndx: indx_t, mut key: *mut DBT)
 -> int32_t {
    let mut key_pagep: *mut PAGE16 = 0 as *mut PAGE16;
    key_pagep =
        __kdb2_get_page(hashp,
                        ((((1 as libc::c_int) <<
                               (*((pagep as *mut libc::c_void as
                                       *mut u_int8_t).offset((12 as
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
                                      *mut indx_t).offset(0 as libc::c_int as
                                                              isize) as
                                    u_int32_t >> 11 as libc::c_int)) -
                              1 as libc::c_int) as
                             libc::c_uint).wrapping_add((*hashp).hdr.hdrpages).wrapping_add((if ((1
                                                                                                      as
                                                                                                      libc::c_int)
                                                                                                     <<
                                                                                                     (*((pagep
                                                                                                             as
                                                                                                             *mut libc::c_void
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
                                                                                                            as
                                                                                                            *mut libc::c_void
                                                                                                            as
                                                                                                            *mut indx_t).offset(0
                                                                                                                                    as
                                                                                                                                    libc::c_int
                                                                                                                                    as
                                                                                                                                    isize)
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
                                                                                                                                       (*((pagep
                                                                                                                                               as
                                                                                                                                               *mut libc::c_void
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
                                                                                                                                              as
                                                                                                                                              *mut libc::c_void
                                                                                                                                              as
                                                                                                                                              *mut indx_t).offset(0
                                                                                                                                                                      as
                                                                                                                                                                      libc::c_int
                                                                                                                                                                      as
                                                                                                                                                                      isize)
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
                                                                                             })).wrapping_add((*((pagep
                                                                                                                      as
                                                                                                                      *mut libc::c_void
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
                                                                                                                     as
                                                                                                                     *mut libc::c_void
                                                                                                                     as
                                                                                                                     *mut indx_t).offset(0
                                                                                                                                             as
                                                                                                                                             libc::c_int
                                                                                                                                             as
                                                                                                                                             isize)
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   &
                                                                                                                   0x7ff
                                                                                                                       as
                                                                                                                       libc::c_int)
                                                                                                                  as
                                                                                                                  libc::c_uint),
                        4 as libc::c_int);
    if key_pagep.is_null() { return -(1 as libc::c_int) }
    (*key).size =
        collect_key(hashp, key_pagep, 0 as libc::c_int, 0 as *mut db_pgno_t)
            as size_t;
    (*key).data = (*hashp).bigkey_buf as *mut libc::c_void;
    __kdb2_put_page(hashp, key_pagep, 4 as libc::c_int, 0 as libc::c_int);
    return 0 as libc::c_int;
}
/*
 * Return the big key and data indicated in item_info.
 */
#[no_mangle]
#[c2rust::src_loc = "317:1"]
pub unsafe extern "C" fn __kdb2_big_return(mut hashp: *mut HTAB,
                                           mut item_info: *mut ITEM_INFO,
                                           mut val: *mut DBT,
                                           mut on_bigkey_page: int32_t)
 -> int32_t {
    let mut pagep: *mut PAGE16 = 0 as *mut PAGE16;
    let mut next_pgno: db_pgno_t = 0;
    if on_bigkey_page == 0 {
        /* Get first page with big pair on it. */
        pagep =
            __kdb2_get_page(hashp,
                            ((((1 as libc::c_int) <<
                                   ((*item_info).data_off as u_int32_t >>
                                        11 as libc::c_int)) -
                                  1 as libc::c_int) as
                                 libc::c_uint).wrapping_add((*hashp).hdr.hdrpages).wrapping_add((if ((1
                                                                                                          as
                                                                                                          libc::c_int)
                                                                                                         <<
                                                                                                         ((*item_info).data_off
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
                                                                                                                                           ((*item_info).data_off
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
                                                                                                 })).wrapping_add(((*item_info).data_off
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       &
                                                                                                                       0x7ff
                                                                                                                           as
                                                                                                                           libc::c_int)
                                                                                                                      as
                                                                                                                      libc::c_uint),
                            4 as libc::c_int);
        if pagep.is_null() { return -(1 as libc::c_int) }
    } else {
        pagep = __kdb2_get_page(hashp, (*item_info).pgno, 4 as libc::c_int);
        if pagep.is_null() { return -(1 as libc::c_int) }
    }
    /* Traverse through the bigkey pages until a page with data is found. */
    while *((pagep as *mut libc::c_void as
                 *mut u_int8_t).offset((12 as libc::c_int as
                                            libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                            as
                                                                            libc::c_ulong)
                                           as
                                           isize).offset((0 as libc::c_int as
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
                *mut indx_t).offset(0 as libc::c_int as isize) == 0 {
        next_pgno =
            *((pagep as *mut libc::c_void as
                   *mut u_int8_t).offset(4 as libc::c_int as isize) as
                  *mut libc::c_void as
                  *mut db_pgno_t).offset(0 as libc::c_int as isize);
        __kdb2_put_page(hashp, pagep, 4 as libc::c_int, 0 as libc::c_int);
        pagep = __kdb2_get_page(hashp, next_pgno, 4 as libc::c_int);
        if pagep.is_null() { return -(1 as libc::c_int) }
    }
    (*val).size = collect_data(hashp, pagep, 0 as libc::c_int) as size_t;
    if (*val).size < 1 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int)
    }
    (*val).data = (*hashp).bigdata_buf as *mut libc::c_void;
    __kdb2_put_page(hashp, pagep, 4 as libc::c_int, 0 as libc::c_int);
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
 * PACKAGE: hash
 * DESCRIPTION:
 *	Big key/data handling for the hashing package.
 *
 * ROUTINES:
 * External
 *	__big_keydata
 *	__big_split
 *	__big_insert
 *	__big_return
 *	__big_delete
 *	__find_last_page
 * Internal
 *	collect_key
 *	collect_data
 */
/*
 * Given a page with a big key on it, traverse through the pages counting data
 * length, and collect all of the data on the way up.  Store the key in
 * hashp->bigkey_buf.  last_page indicates to the calling function what the
 * last page with key on it is; this will help if you later want to retrieve
 * the data portion.
 *
 * Does the work for __get_bigkey.
 *
 * Return total length of data; -1 if error.
 */
#[c2rust::src_loc = "368:1"]
unsafe extern "C" fn collect_key(mut hashp: *mut HTAB, mut pagep: *mut PAGE16,
                                 mut len: int32_t,
                                 mut last_page: *mut db_pgno_t) -> int32_t {
    let mut next_pagep: *mut PAGE16 = 0 as *mut PAGE16;
    let mut totlen: int32_t = 0;
    let mut retval: int32_t = 0;
    let mut next_pgno: db_pgno_t = 0;
    /* If this is the last page with key. */
    if *((pagep as *mut libc::c_void as
              *mut u_int8_t).offset((12 as libc::c_int as
                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                         as
                                                                         libc::c_ulong)
                                        as
                                        isize).offset((0 as libc::c_int as
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
             *mut indx_t).offset(0 as libc::c_int as isize) != 0 {
        totlen =
            len +
                *((pagep as *mut libc::c_void as
                       *mut u_int8_t).offset((12 as libc::c_int as
                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                  as
                                                                                  libc::c_ulong)
                                                 as
                                                 isize).offset((0 as
                                                                    libc::c_int
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
                    libc::c_int;
        if !(*hashp).bigkey_buf.is_null() {
            free((*hashp).bigkey_buf as *mut libc::c_void);
        }
        (*hashp).bigkey_buf =
            malloc(totlen as libc::c_ulong) as *mut u_int8_t;
        if (*hashp).bigkey_buf.is_null() { return -(1 as libc::c_int) }
        memcpy((*hashp).bigkey_buf.offset(len as isize) as *mut libc::c_void,
               (pagep as
                    *mut PAGE8).offset((12 as libc::c_int as
                                            libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                            as
                                                                            libc::c_ulong)
                                           as
                                           isize).offset(((::std::mem::size_of::<indx_t>()
                                                               as
                                                               libc::c_ulong)
                                                              <<
                                                              1 as
                                                                  libc::c_int)
                                                             as isize) as
                   *const libc::c_void,
               *((pagep as *mut libc::c_void as
                      *mut u_int8_t).offset((12 as libc::c_int as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                 as
                                                                                 libc::c_ulong)
                                                as
                                                isize).offset((0 as
                                                                   libc::c_int
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
                     *mut indx_t).offset(0 as libc::c_int as isize) as
                   libc::c_ulong);
        if !last_page.is_null() {
            *last_page =
                *((pagep as *mut libc::c_void as
                       *mut u_int8_t).offset(0 as libc::c_int as isize) as
                      *mut libc::c_void as
                      *mut db_pgno_t).offset(0 as libc::c_int as isize)
        }
        return totlen
    }
    /* Key filled up all of last key page, so we've gone 1 too far. */
    if *((pagep as *mut libc::c_void as
              *mut u_int8_t).offset((12 as libc::c_int as
                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                         as
                                                                         libc::c_ulong)
                                        as
                                        isize).offset((0 as libc::c_int as
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
        if !(*hashp).bigkey_buf.is_null() {
            free((*hashp).bigkey_buf as *mut libc::c_void);
        }
        (*hashp).bigkey_buf = malloc(len as libc::c_ulong) as *mut u_int8_t;
        return if !(*hashp).bigkey_buf.is_null() {
                   len
               } else { -(1 as libc::c_int) }
    }
    totlen =
        len +
            *((pagep as *mut libc::c_void as
                   *mut u_int8_t).offset((12 as libc::c_int as
                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                              as
                                                                              libc::c_ulong)
                                             as
                                             isize).offset((0 as libc::c_int
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
                  *mut indx_t).offset(0 as libc::c_int as isize) as
                libc::c_int;
    /* Set pagep to the next page in the chain. */
    if !last_page.is_null() {
        *last_page =
            *((pagep as *mut libc::c_void as
                   *mut u_int8_t).offset(0 as libc::c_int as isize) as
                  *mut libc::c_void as
                  *mut db_pgno_t).offset(0 as libc::c_int as isize)
    }
    next_pgno =
        *((pagep as *mut libc::c_void as
               *mut u_int8_t).offset(4 as libc::c_int as isize) as
              *mut libc::c_void as
              *mut db_pgno_t).offset(0 as libc::c_int as isize);
    next_pagep = __kdb2_get_page(hashp, next_pgno, 4 as libc::c_int);
    if next_pagep.is_null() { return -(1 as libc::c_int) }
    retval = collect_key(hashp, next_pagep, totlen, last_page);
    memcpy((*hashp).bigkey_buf.offset(len as isize) as *mut libc::c_void,
           (pagep as
                *mut PAGE8).offset((12 as libc::c_int as
                                        libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                        as
                                                                        libc::c_ulong)
                                       as
                                       isize).offset(((::std::mem::size_of::<indx_t>()
                                                           as libc::c_ulong)
                                                          << 1 as libc::c_int)
                                                         as isize) as
               *const libc::c_void,
           *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset((12 as libc::c_int as
                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                             as
                                                                             libc::c_ulong)
                                            as
                                            isize).offset((0 as libc::c_int as
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
               libc::c_ulong);
    __kdb2_put_page(hashp, next_pagep, 4 as libc::c_int, 0 as libc::c_int);
    return retval;
}
/*
 * Given a page with big data on it, recur through the pages counting data
 * length, and collect all of the data on the way up.  Store the data in
 * hashp->bigdata_buf.
 *
 * Does the work for __big_return.
 *
 * Return total length of data; -1 if error.
 */
#[c2rust::src_loc = "436:1"]
unsafe extern "C" fn collect_data(mut hashp: *mut HTAB,
                                  mut pagep: *mut PAGE16, mut len: int32_t)
 -> int32_t {
    let mut next_pagep: *mut PAGE16 = 0 as *mut PAGE16;
    let mut totlen: int32_t = 0;
    let mut retval: int32_t = 0;
    let mut next_pgno: db_pgno_t = 0;
    /* If there is no next page. */
    if *((pagep as *mut libc::c_void as
              *mut u_int8_t).offset(4 as libc::c_int as isize) as
             *mut libc::c_void as
             *mut db_pgno_t).offset(0 as libc::c_int as isize) ==
           0xffffffff as libc::c_uint {
        if !(*hashp).bigdata_buf.is_null() {
            free((*hashp).bigdata_buf as *mut libc::c_void);
        }
        totlen =
            len +
                *((pagep as *mut libc::c_void as
                       *mut u_int8_t).offset((12 as libc::c_int as
                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                  as
                                                                                  libc::c_ulong)
                                                 as
                                                 isize).offset((0 as
                                                                    libc::c_int
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
                    libc::c_int;
        (*hashp).bigdata_buf =
            malloc(totlen as libc::c_ulong) as *mut u_int8_t;
        if (*hashp).bigdata_buf.is_null() { return -(1 as libc::c_int) }
        memcpy((*hashp).bigdata_buf.offset(totlen as
                                               isize).offset(-(*((pagep as
                                                                      *mut libc::c_void
                                                                      as
                                                                      *mut u_int8_t).offset((12
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                 as
                                                                                                                                 libc::c_ulong)
                                                                                                as
                                                                                                isize).offset((0
                                                                                                                   as
                                                                                                                   libc::c_int
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
                                                                     as
                                                                     *mut libc::c_void
                                                                     as
                                                                     *mut indx_t).offset(0
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             isize)
                                                                   as
                                                                   libc::c_int
                                                                   as isize))
                   as *mut libc::c_void,
               (pagep as
                    *mut PAGE8).offset((12 as libc::c_int as
                                            libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                            as
                                                                            libc::c_ulong)
                                           as
                                           isize).offset(((::std::mem::size_of::<indx_t>()
                                                               as
                                                               libc::c_ulong)
                                                              <<
                                                              1 as
                                                                  libc::c_int)
                                                             as
                                                             isize).offset(*((pagep
                                                                                  as
                                                                                  *mut libc::c_void
                                                                                  as
                                                                                  *mut u_int8_t).offset((12
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                             as
                                                                                                                                             libc::c_ulong)
                                                                                                            as
                                                                                                            isize).offset((0
                                                                                                                               as
                                                                                                                               libc::c_int
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
                                                                                 as
                                                                                 *mut libc::c_void
                                                                                 as
                                                                                 *mut indx_t).offset(0
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         isize)
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                   as *const libc::c_void,
               *((pagep as *mut libc::c_void as
                      *mut u_int8_t).offset((12 as libc::c_int as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                 as
                                                                                 libc::c_ulong)
                                                as
                                                isize).offset((0 as
                                                                   libc::c_int
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
                   libc::c_ulong);
        return totlen
    }
    totlen =
        len +
            *((pagep as *mut libc::c_void as
                   *mut u_int8_t).offset((12 as libc::c_int as
                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                              as
                                                                              libc::c_ulong)
                                             as
                                             isize).offset((0 as libc::c_int
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
                libc::c_int;
    /* Set pagep to the next page in the chain. */
    next_pgno =
        *((pagep as *mut libc::c_void as
               *mut u_int8_t).offset(4 as libc::c_int as isize) as
              *mut libc::c_void as
              *mut db_pgno_t).offset(0 as libc::c_int as isize);
    next_pagep = __kdb2_get_page(hashp, next_pgno, 4 as libc::c_int);
    if next_pagep.is_null() { return -(1 as libc::c_int) }
    retval = collect_data(hashp, next_pagep, totlen);
    memcpy((*hashp).bigdata_buf.offset(totlen as
                                           isize).offset(-(*((pagep as
                                                                  *mut libc::c_void
                                                                  as
                                                                  *mut u_int8_t).offset((12
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                             as
                                                                                                                             libc::c_ulong)
                                                                                            as
                                                                                            isize).offset((0
                                                                                                               as
                                                                                                               libc::c_int
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
                                                                 as
                                                                 *mut libc::c_void
                                                                 as
                                                                 *mut indx_t).offset(0
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         isize)
                                                               as libc::c_int
                                                               as isize)) as
               *mut libc::c_void,
           (pagep as
                *mut PAGE8).offset((12 as libc::c_int as
                                        libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                        as
                                                                        libc::c_ulong)
                                       as
                                       isize).offset(((::std::mem::size_of::<indx_t>()
                                                           as libc::c_ulong)
                                                          << 1 as libc::c_int)
                                                         as
                                                         isize).offset(*((pagep
                                                                              as
                                                                              *mut libc::c_void
                                                                              as
                                                                              *mut u_int8_t).offset((12
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                         as
                                                                                                                                         libc::c_ulong)
                                                                                                        as
                                                                                                        isize).offset((0
                                                                                                                           as
                                                                                                                           libc::c_int
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
                                                                             as
                                                                             *mut libc::c_void
                                                                             as
                                                                             *mut indx_t).offset(0
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     isize)
                                                                           as
                                                                           libc::c_int
                                                                           as
                                                                           isize)
               as *const libc::c_void,
           *((pagep as *mut libc::c_void as
                  *mut u_int8_t).offset((12 as libc::c_int as
                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                             as
                                                                             libc::c_ulong)
                                            as
                                            isize).offset((0 as libc::c_int as
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
               libc::c_ulong);
    __kdb2_put_page(hashp, next_pagep, 4 as libc::c_int, 0 as libc::c_int);
    return retval;
}
