use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:41"]
pub mod types_h {
    #[c2rust::src_loc = "31:1"]
    pub type __u_char = libc::c_uchar;
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "34:1"]
    pub type __u_long = libc::c_ulong;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
}
#[c2rust::header_src = "/usr/include/sys/types.h:41"]
pub mod sys_types_h {
    #[c2rust::src_loc = "33:1"]
    pub type u_char = __u_char;
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "36:1"]
    pub type u_long = __u_long;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    #[c2rust::src_loc = "158:1"]
    pub type u_int8_t = __uint8_t;
    #[c2rust::src_loc = "159:1"]
    pub type u_int16_t = __uint16_t;
    #[c2rust::src_loc = "160:1"]
    pub type u_int32_t = __uint32_t;
    use super::types_h::{__u_char, __u_int, __u_long, __caddr_t, __uint8_t,
                         __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:41"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:44"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    #[c2rust::src_loc = "43:1"]
    pub type _IO_lock_t = ();
    use super::types_h::{__off_t, __off64_t};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "38:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "37:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "36:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:44"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/db.h:47"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/include/db-int.h:47"]
pub mod db_int_h {
    #[c2rust::src_loc = "143:1"]
    pub type db_pgno_t = u_int32_t;
    #[c2rust::src_loc = "145:1"]
    pub type indx_t = u_int16_t;
    #[c2rust::src_loc = "147:1"]
    pub type recno_t = u_int32_t;
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
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/mpool/mpool.h:48"]
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
        #[c2rust::src_loc = "109:1"]
        pub fn kdb2_mpool_get(_: *mut MPOOL, _: db_pgno_t, _: u_int)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "111:1"]
        pub fn kdb2_mpool_put(_: *mut MPOOL, _: *mut libc::c_void, _: u_int)
         -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/btree/btree.h:48"]
pub mod btree_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "67:16"]
    pub struct _page {
        pub pgno: db_pgno_t,
        pub prevpg: db_pgno_t,
        pub nextpg: db_pgno_t,
        pub flags: u_int32_t,
        pub lower: indx_t,
        pub upper: indx_t,
        pub linp: [indx_t; 1],
    }
    /* Tree root page number. */
    /*
 * There are five page layouts in the btree: btree internal pages (BINTERNAL),
 * btree leaf pages (BLEAF), recno internal pages (RINTERNAL), recno leaf pages
 * (RLEAF) and overflow pages.  All five page types have a page header (PAGE).
 * This implementation requires that values within structures NOT be padded.
 * (ANSI C permits random padding.)  If your compiler pads randomly you'll have
 * to do some work to get this package to run.
 */
    #[c2rust::src_loc = "67:1"]
    pub type PAGE = _page;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "120:16"]
    pub struct _binternal {
        pub ksize: u_int32_t,
        pub pgno: db_pgno_t,
        pub flags: u_char,
        pub bytes: [libc::c_char; 1],
    }
    /* this page's page number */
    /* left sibling */
    /* right sibling */
    /* btree internal page */
    /* leaf page */
    /* overflow page */
    /* recno internal page */
    /* leaf page */
    /* type mask */
    /* never delete this chain of pages */
    /* lower bound of free space on page */
    /* upper bound of free space on page */
    /* indx_t-aligned VAR. LENGTH DATA */
    /*
 * For pages other than overflow pages, there is an array of offsets into the
 * rest of the page immediately following the page header.  Each offset is to
 * an item which is unique to the type of page.  The h_lower offset is just
 * past the last filled-in index.  The h_upper offset is the first item on the
 * page.  Offsets are from the beginning of the page.
 *
 * If an item is too big to store on a single page, a flag is set and the item
 * is a { page, size } pair such that the page is the first page of an overflow
 * chain with size bytes of item.  Overflow pages are simply bytes without any
 * external structure.
 *
 * The page number and size fields in the items are db_pgno_t-aligned so they can
 * be manipulated without copying.  (This presumes that 32 bit items can be
 * manipulated on this system.)
 */
    /*
 * For the btree internal pages, the item is a key.  BINTERNALs are {key, pgno}
 * pairs, such that the key compares less than or equal to all of the records
 * on that page.  For a tree without duplicate keys, an internal page with two
 * consecutive keys, a and b, will have all records greater than or equal to a
 * and less than b stored on the page associated with a.  Duplicate keys are
 * somewhat special and can cause duplicate internal and leaf page records and
 * some minor modifications of the above rule.
 */
    #[c2rust::src_loc = "120:1"]
    pub type BINTERNAL = _binternal;
    /* key size */
    /* page number stored on */
    /* overflow data */
    /* overflow key */
    /* data */
    /* Get the page's RINTERNAL structure at index indx. */
    /* Get the number of bytes in the entry. */
    /* Copy a RINTERAL entry to the page. */
    /* For the btree leaf pages, the item is a key and data pair. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "172:16"]
    pub struct _bleaf {
        pub ksize: u_int32_t,
        pub dsize: u_int32_t,
        pub flags: u_char,
        pub bytes: [libc::c_char; 1],
    }
    #[c2rust::src_loc = "172:1"]
    pub type BLEAF = _bleaf;
    /* data */
    /* Get the page's RLEAF structure at index indx. */
    /* Get the number of bytes in the entry. */
    /* Get the number of bytes from the user's data. */
    /* Copy a RLEAF entry to the page. */
    /*
 * A record in the tree is either a pointer to a page and an index in the page
 * or a page number and an index.  These structures are used as a cursor, stack
 * entry and search returns as well as to pass records to other routines.
 *
 * One comment about searches.  Internal page searches must find the largest
 * record less than key in the tree so that descents work.  Leaf page searches
 * must find the smallest record greater than key so that the returned index
 * is the record's correct position for insertion.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "241:16"]
    pub struct _epgno {
        pub pgno: db_pgno_t,
        pub index: indx_t,
    }
    #[c2rust::src_loc = "241:1"]
    pub type EPGNO = _epgno;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "246:16"]
    pub struct _epg {
        pub page: *mut PAGE,
        pub index: indx_t,
    }
    #[c2rust::src_loc = "246:1"]
    pub type EPG = _epg;
    /* the index on the page */
    /* the index on the page */
    /*
 * About cursors.  The cursor (and the page that contained the key/data pair
 * that it referenced) can be deleted, which makes things a bit tricky.  If
 * there are no duplicates of the cursor key in the tree (i.e. B_NODUPS is set
 * or there simply aren't any duplicates of the key) we copy the key that it
 * referenced when it's deleted, and reacquire a new cursor key if the cursor
 * is used again.  If there are duplicates keys, we move to the next/previous
 * key, and set a flag so that we know what happened.  NOTE: if duplicate (to
 * the cursor) keys are added to the tree during this process, it is undefined
 * if they will be returned or not in a cursor scan.
 *
 * The flags determine the possible states of the cursor:
 *
 * CURS_INIT	The cursor references *something*.
 * CURS_ACQUIRE	The cursor was deleted, and a key has been saved so that
 *		we can reacquire the right position in the tree.
 * CURS_AFTER, CURS_BEFORE
 *		The cursor was deleted, and now references a key/data pair
 *		that has not yet been returned, either before or after the
 *		deleted key/data pair.
 * XXX
 * This structure is broken out so that we can eventually offer multiple
 * cursors as part of the DB interface.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "275:16"]
    pub struct _cursor {
        pub pg: EPGNO,
        pub key: DBT,
        pub rcursor: recno_t,
        pub flags: u_int8_t,
    }
    #[c2rust::src_loc = "275:1"]
    pub type CURSOR = _cursor;
    /* The in-memory btree/recno data structure. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "304:16"]
    pub struct _btree {
        pub bt_mp: *mut MPOOL,
        pub bt_dbp: *mut DB,
        pub bt_cur: EPG,
        pub bt_pinned: *mut PAGE,
        pub bt_cursor: CURSOR,
        pub bt_stack: [EPGNO; 50],
        pub bt_sp: *mut EPGNO,
        pub bt_rkey: DBT,
        pub bt_rdata: DBT,
        pub bt_fd: libc::c_int,
        pub bt_free: db_pgno_t,
        pub bt_psize: u_int32_t,
        pub bt_ovflsize: indx_t,
        pub bt_lorder: libc::c_int,
        pub bt_order: C2RustUnnamed_1,
        pub bt_last: EPGNO,
        pub bt_cmp: Option<unsafe extern "C" fn(_: *const DBT, _: *const DBT)
                               -> libc::c_int>,
        pub bt_pfx: Option<unsafe extern "C" fn(_: *const DBT, _: *const DBT)
                               -> size_t>,
        pub bt_irec: Option<unsafe extern "C" fn(_: *mut _btree, _: recno_t)
                                -> libc::c_int>,
        pub bt_rfp: *mut FILE,
        pub bt_rfd: libc::c_int,
        pub bt_cmap: caddr_t,
        pub bt_smap: caddr_t,
        pub bt_emap: caddr_t,
        pub bt_msize: size_t,
        pub bt_nrecs: recno_t,
        pub bt_reclen: size_t,
        pub bt_bval: u_char,
        pub flags: u_int32_t,
    }
    #[c2rust::src_loc = "334:2"]
    pub type C2RustUnnamed_1 = libc::c_uint;
    #[c2rust::src_loc = "334:20"]
    pub const FORWARD: C2RustUnnamed_1 = 2;
    #[c2rust::src_loc = "334:14"]
    pub const BACK: C2RustUnnamed_1 = 1;
    #[c2rust::src_loc = "334:9"]
    pub const NOT: C2RustUnnamed_1 = 0;
    #[c2rust::src_loc = "304:1"]
    pub type BTREE = _btree;
    use super::db_int_h::{db_pgno_t, indx_t, recno_t};
    use super::sys_types_h::{u_int32_t, u_char, u_int8_t, caddr_t};
    use super::db_h::{DBT, DB};
    use super::mpool_h::MPOOL;
    use super::stddef_h::size_t;
    use super::FILE_h::FILE;
}
#[c2rust::header_src = "/usr/include/errno.h:43"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:45"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "46:14"]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/btree/extern.h:48"]
pub mod extern_h {
    use super::btree_h::{BTREE, EPG, PAGE};
    use super::db_h::DBT;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "68:1"]
        pub fn __kdb2_bt_cmp(_: *mut BTREE, _: *const DBT, _: *mut EPG)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "75:1"]
        pub fn __kdb2_bt_free(_: *mut BTREE, _: *mut PAGE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "90:1"]
        pub fn __kdb2_ovfl_delete(_: *mut BTREE, _: *mut libc::c_void)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "82:1"]
        pub fn __kdb2_bt_ret(_: *mut BTREE, _: *mut EPG, _: *mut DBT,
                             _: *mut DBT, _: *mut DBT, _: *mut DBT,
                             _: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "83:1"]
        pub fn __kdb2_bt_search(_: *mut BTREE, _: *const DBT,
                                _: *mut libc::c_int) -> *mut EPG;
    }
}
pub use self::types_h::{__u_char, __u_int, __u_long, __uint8_t, __uint16_t,
                        __uint32_t, __off_t, __off64_t, __caddr_t};
pub use self::sys_types_h::{u_char, u_int, u_long, caddr_t, u_int8_t,
                            u_int16_t, u_int32_t};
pub use self::stddef_h::size_t;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::db_h::{DBT, DBTYPE, DB_RECNO, DB_HASH, DB_BTREE, __db, DB};
pub use self::db_int_h::{db_pgno_t, indx_t, recno_t};
pub use self::mpool_h::{_bkt, C2RustUnnamed, C2RustUnnamed_0, MPOOL, _hqh,
                        _lqh, kdb2_mpool_get, kdb2_mpool_put};
pub use self::btree_h::{_page, PAGE, _binternal, BINTERNAL, _bleaf, BLEAF,
                        _epgno, EPGNO, _epg, EPG, _cursor, CURSOR, _btree,
                        C2RustUnnamed_1, FORWARD, BACK, NOT, BTREE};
use self::errno_h::__errno_location;
use self::string_h::memmove;
use self::extern_h::{__kdb2_bt_cmp, __kdb2_bt_free, __kdb2_ovfl_delete,
                     __kdb2_bt_ret, __kdb2_bt_search};
/*
 * __bt_delete
 *	Delete the item(s) referenced by a key.
 *
 * Return RET_SPECIAL if the key is not found.
 */
#[no_mangle]
#[c2rust::src_loc = "61:1"]
pub unsafe extern "C" fn __kdb2_bt_delete(mut dbp: *const DB,
                                          mut key: *const DBT,
                                          mut flags: u_int) -> libc::c_int {
    let mut t: *mut BTREE = 0 as *mut BTREE;
    let mut c: *mut CURSOR = 0 as *mut CURSOR;
    let mut h: *mut PAGE = 0 as *mut PAGE;
    let mut status: libc::c_int = 0;
    t = (*dbp).internal as *mut BTREE;
    /* Toss any page pinned across calls. */
    if !(*t).bt_pinned.is_null() {
        kdb2_mpool_put((*t).bt_mp, (*t).bt_pinned as *mut libc::c_void,
                       0 as libc::c_int as u_int);
        (*t).bt_pinned = 0 as *mut PAGE
    }
    /* Check for change to a read-only tree. */
    if (*t).flags & 0x10 as libc::c_int as libc::c_uint != 0 {
        *__errno_location() = 1 as libc::c_int;
        return -(1 as libc::c_int)
    }
    let mut current_block_25: u64;
    match flags {
        0 => {
            status = __bt_bdelete(t, key);
            current_block_25 = 3437258052017859086;
        }
        1 => {
            /*
		 * If flags is R_CURSOR, delete the cursor.  Must already
		 * have started a scan and not have already deleted it.
		 */
            c = &mut (*t).bt_cursor;
            if (*c).flags as libc::c_int & 0x8 as libc::c_int != 0 {
                if (*c).flags as libc::c_int &
                       (0x1 as libc::c_int | 0x2 as libc::c_int |
                            0x4 as libc::c_int) != 0 {
                    return 1 as libc::c_int
                }
                h =
                    kdb2_mpool_get((*t).bt_mp, (*c).pg.pgno,
                                   0 as libc::c_int as u_int) as *mut PAGE;
                if h.is_null() { return -(1 as libc::c_int) }
                /*
			 * If the page is about to be emptied, we'll need to
			 * delete it, which means we have to acquire a stack.
			 */
                if ((*h).lower as
                        libc::c_ulong).wrapping_sub((::std::mem::size_of::<db_pgno_t>()
                                                         as
                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                         as
                                                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                                                         as
                                                                                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                                                                                                         as
                                                                                                                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                         as
                                                                                                                                                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                         as
                                                                                                                                                                                                                         libc::c_ulong)).wrapping_div(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                                                          as
                                                                                                                                                                                                                                                          libc::c_ulong)
                       == 1 as libc::c_int as libc::c_ulong {
                    if __bt_stkacq(t, &mut h, &mut (*t).bt_cursor) != 0 {
                        return -(1 as libc::c_int)
                    }
                }
                status =
                    __kdb2_bt_deleaf(t, 0 as *const DBT, h,
                                     (*c).pg.index as u_int);
                if ((*h).lower as
                        libc::c_ulong).wrapping_sub((::std::mem::size_of::<db_pgno_t>()
                                                         as
                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                         as
                                                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                                                         as
                                                                                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                                                                                                         as
                                                                                                                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                         as
                                                                                                                                                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                         as
                                                                                                                                                                                                                         libc::c_ulong)).wrapping_div(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                                                          as
                                                                                                                                                                                                                                                          libc::c_ulong)
                       == 0 as libc::c_int as libc::c_ulong &&
                       status == 0 as libc::c_int {
                    if __bt_pdelete(t, h) != 0 { return -(1 as libc::c_int) }
                } else {
                    kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                                   if status == 0 as libc::c_int {
                                       0x1 as libc::c_int
                                   } else { 0 as libc::c_int } as u_int);
                }
                current_block_25 = 3437258052017859086;
            } else { current_block_25 = 13504760517129887221; }
        }
        _ => { current_block_25 = 13504760517129887221; }
    }
    match current_block_25 {
        3437258052017859086 => { }
        _ =>
        /* FALLTHROUGH */
        {
            *__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int)
        }
    }
    if status == 0 as libc::c_int {
        (*t).flags |= 0x4 as libc::c_int as libc::c_uint
    }
    return status;
}
/*
 * __bt_stkacq --
 *	Acquire a stack so we can delete a cursor entry.
 *
 * Parameters:
 *	  t:	tree
 *	 hp:	pointer to current, pinned PAGE pointer
 *	  c:	pointer to the cursor
 *
 * Returns:
 *	0 on success, 1 on failure
 */
#[c2rust::src_loc = "142:1"]
unsafe extern "C" fn __bt_stkacq(mut t: *mut BTREE, mut hp: *mut *mut PAGE,
                                 mut c: *mut CURSOR) -> libc::c_int {
    let mut bi: *mut BINTERNAL = 0 as *mut BINTERNAL;
    let mut e: *mut EPG = 0 as *mut EPG;
    let mut parent: *mut EPGNO = 0 as *mut EPGNO;
    let mut h: *mut PAGE = 0 as *mut PAGE;
    let mut idx: indx_t = 0 as libc::c_int as indx_t;
    let mut pgno: db_pgno_t = 0;
    let mut nextpg: recno_t = 0;
    let mut prevpg: recno_t = 0;
    let mut exact: libc::c_int = 0;
    let mut level: libc::c_uint = 0;
    /*
	 * Find the first occurrence of the key in the tree.  Toss the
	 * currently locked page so we don't hit an already-locked page.
	 */
    h = *hp;
    kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                   0 as libc::c_int as u_int);
    e = __kdb2_bt_search(t, &mut (*c).key, &mut exact);
    if e.is_null() { return 1 as libc::c_int }
    h = (*e).page;
    /* See if we got it in one shot. */
    if !((*h).pgno == (*c).pg.pgno) {
        /*
	 * Move right, looking for the page.  At each move we have to move
	 * up the stack until we don't have to move to the next page.  If
	 * we have to change pages at an internal level, we have to fix the
	 * stack back up.
	 */
        while (*h).pgno != (*c).pg.pgno {
            nextpg = (*h).nextpg;
            if nextpg == 0 as libc::c_int as libc::c_uint { break ; }
            kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                           0 as libc::c_int as u_int);
            /* Move up the stack. */
            level = 0 as libc::c_int as libc::c_uint;
            loop  {
                parent =
                    if (*t).bt_sp == (*t).bt_stack.as_mut_ptr() {
                        0 as *mut EPGNO
                    } else { (*t).bt_sp = (*t).bt_sp.offset(-1); (*t).bt_sp };
                if parent.is_null() { break ; }
                /* Get the parent page. */
                h =
                    kdb2_mpool_get((*t).bt_mp, (*parent).pgno,
                                   0 as libc::c_int as u_int) as *mut PAGE;
                if h.is_null() { return 1 as libc::c_int }
                /* Move to the next index. */
                if (*parent).index as libc::c_ulong !=
                       ((*h).lower as
                            libc::c_ulong).wrapping_sub((::std::mem::size_of::<db_pgno_t>()
                                                             as
                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                             as
                                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                                                             as
                                                                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                                                                                                             as
                                                                                                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                             as
                                                                                                                                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                             as
                                                                                                                                                                                                                             libc::c_ulong)).wrapping_div(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                                                              as
                                                                                                                                                                                                                                                              libc::c_ulong).wrapping_sub(1
                                                                                                                                                                                                                                                                                              as
                                                                                                                                                                                                                                                                                              libc::c_int
                                                                                                                                                                                                                                                                                              as
                                                                                                                                                                                                                                                                                              libc::c_ulong)
                   {
                    idx =
                        ((*parent).index as libc::c_int + 1 as libc::c_int) as
                            indx_t;
                    (*(*t).bt_sp).pgno = (*h).pgno;
                    (*(*t).bt_sp).index = idx;
                    (*t).bt_sp = (*t).bt_sp.offset(1);
                    break ;
                } else {
                    kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                                   0 as libc::c_int as u_int);
                    level = level.wrapping_add(1)
                }
            }
            loop 
                 /* Restore the stack. */
                 {
                let fresh0 = level;
                level = level.wrapping_sub(1);
                if !(fresh0 != 0) { break ; }
                /* Push the next level down onto the stack. */
                bi =
                    (h as
                         *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(idx
                                                                                      as
                                                                                      isize)
                                                       as libc::c_int as
                                                       isize) as
                        *mut libc::c_void as *mut BINTERNAL;
                pgno = (*bi).pgno;
                (*(*t).bt_sp).pgno = pgno;
                (*(*t).bt_sp).index = 0 as libc::c_int as indx_t;
                (*t).bt_sp = (*t).bt_sp.offset(1);
                /* Lose the currently pinned page. */
                kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                               0 as libc::c_int as u_int);
                /* Get the next level down. */
                h =
                    kdb2_mpool_get((*t).bt_mp, pgno,
                                   0 as libc::c_int as u_int) as *mut PAGE;
                if h.is_null() { return 1 as libc::c_int }
                idx = 0 as libc::c_int as indx_t
            }
            kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                           0 as libc::c_int as u_int);
            h =
                kdb2_mpool_get((*t).bt_mp, nextpg, 0 as libc::c_int as u_int)
                    as *mut PAGE;
            if h.is_null() { return 1 as libc::c_int }
        }
        if !((*h).pgno == (*c).pg.pgno) {
            /* Reacquire the original stack. */
            kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                           0 as libc::c_int as u_int);
            e = __kdb2_bt_search(t, &mut (*c).key, &mut exact);
            if e.is_null() { return 1 as libc::c_int }
            h = (*e).page;
            /*
	 * Move left, looking for the page.  At each move we have to move
	 * up the stack until we don't have to change pages to move to the
	 * next page.  If we have to change pages at an internal level, we
	 * have to fix the stack back up.
	 */
            while (*h).pgno != (*c).pg.pgno {
                prevpg = (*h).prevpg;
                if prevpg == 0 as libc::c_int as libc::c_uint { break ; }
                kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                               0 as libc::c_int as u_int);
                /* Move up the stack. */
                level = 0 as libc::c_int as libc::c_uint;
                loop  {
                    parent =
                        if (*t).bt_sp == (*t).bt_stack.as_mut_ptr() {
                            0 as *mut EPGNO
                        } else {
                            (*t).bt_sp = (*t).bt_sp.offset(-1);
                            (*t).bt_sp
                        };
                    if parent.is_null() { break ; }
                    /* Get the parent page. */
                    h =
                        kdb2_mpool_get((*t).bt_mp, (*parent).pgno,
                                       0 as libc::c_int as u_int) as
                            *mut PAGE;
                    if h.is_null() { return 1 as libc::c_int }
                    /* Move to the next index. */
                    if (*parent).index as libc::c_int != 0 as libc::c_int {
                        idx =
                            ((*parent).index as libc::c_int -
                                 1 as libc::c_int) as indx_t;
                        (*(*t).bt_sp).pgno = (*h).pgno;
                        (*(*t).bt_sp).index = idx;
                        (*t).bt_sp = (*t).bt_sp.offset(1);
                        break ;
                    } else {
                        kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                                       0 as libc::c_int as u_int);
                        level = level.wrapping_add(1)
                    }
                }
                loop 
                     /* Restore the stack. */
                     {
                    let fresh1 = level;
                    level = level.wrapping_sub(1);
                    if !(fresh1 != 0) { break ; }
                    /* Push the next level down onto the stack. */
                    bi =
                        (h as
                             *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(idx
                                                                                          as
                                                                                          isize)
                                                           as libc::c_int as
                                                           isize) as
                            *mut libc::c_void as *mut BINTERNAL;
                    pgno = (*bi).pgno;
                    /* Lose the currently pinned page. */
                    kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                                   0 as libc::c_int as u_int);
                    /* Get the next level down. */
                    h =
                        kdb2_mpool_get((*t).bt_mp, pgno,
                                       0 as libc::c_int as u_int) as
                            *mut PAGE;
                    if h.is_null() { return 1 as libc::c_int }
                    idx =
                        ((*h).lower as
                             libc::c_ulong).wrapping_sub((::std::mem::size_of::<db_pgno_t>()
                                                              as
                                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                              as
                                                                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                                                              as
                                                                                                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                                                                                                              as
                                                                                                                                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                              as
                                                                                                                                                                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              libc::c_ulong)).wrapping_div(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                                                               as
                                                                                                                                                                                                                                                               libc::c_ulong).wrapping_sub(1
                                                                                                                                                                                                                                                                                               as
                                                                                                                                                                                                                                                                                               libc::c_int
                                                                                                                                                                                                                                                                                               as
                                                                                                                                                                                                                                                                                               libc::c_ulong)
                            as indx_t;
                    (*(*t).bt_sp).pgno = pgno;
                    (*(*t).bt_sp).index = idx;
                    (*t).bt_sp = (*t).bt_sp.offset(1)
                }
                kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                               0 as libc::c_int as u_int);
                h =
                    kdb2_mpool_get((*t).bt_mp, prevpg,
                                   0 as libc::c_int as u_int) as *mut PAGE;
                if h.is_null() { return 1 as libc::c_int }
            }
        }
    }
    kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                   0 as libc::c_int as u_int);
    *hp =
        kdb2_mpool_get((*t).bt_mp, (*c).pg.pgno, 0 as libc::c_int as u_int) as
            *mut PAGE;
    return (*hp == 0 as *mut libc::c_void as *mut PAGE) as libc::c_int;
}
/*-
 * Copyright (c) 1990, 1993, 1994
 *	The Regents of the University of California.  All rights reserved.
 *
 * This code is derived from software contributed to Berkeley by
 * Mike Olson.
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
 * __bt_bdelete --
 *	Delete all key/data pairs matching the specified key.
 *
 * Parameters:
 *	  t:	tree
 *	key:	key to delete
 *
 * Returns:
 *	RET_ERROR, RET_SUCCESS and RET_SPECIAL if the key not found.
 */
#[c2rust::src_loc = "290:1"]
unsafe extern "C" fn __bt_bdelete(mut t: *mut BTREE, mut key: *const DBT)
 -> libc::c_int {
    let mut e: *mut EPG = 0 as *mut EPG;
    let mut h: *mut PAGE = 0 as *mut PAGE;
    let mut deleted: libc::c_int = 0;
    let mut exact: libc::c_int = 0;
    let mut redo: libc::c_int = 0;
    deleted = 0 as libc::c_int;
    loop 
         /* Find any matching record; __bt_search pins the page. */
         {
        e = __kdb2_bt_search(t, key, &mut exact);
        if e.is_null() {
            return if deleted != 0 {
                       0 as libc::c_int
                   } else { -(1 as libc::c_int) }
        }
        if exact == 0 {
            kdb2_mpool_put((*t).bt_mp, (*e).page as *mut libc::c_void,
                           0 as libc::c_int as u_int);
            return if deleted != 0 {
                       0 as libc::c_int
                   } else { 1 as libc::c_int }
        }
        /*
	 * Delete forward, then delete backward, from the found key.  If
	 * there are duplicates and we reach either side of the page, do
	 * the key search again, so that we get them all.
	 */
        redo = 0 as libc::c_int;
        h = (*e).page;
        loop  {
            if __kdb2_bt_deleaf(t, key, h, (*e).index as u_int) != 0 {
                kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                               0 as libc::c_int as u_int);
                return -(1 as libc::c_int)
            }
            if (*t).flags & 0x20 as libc::c_int as libc::c_uint != 0 {
                if ((*h).lower as
                        libc::c_ulong).wrapping_sub((::std::mem::size_of::<db_pgno_t>()
                                                         as
                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                         as
                                                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                                                         as
                                                                                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                                                                                                         as
                                                                                                                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                         as
                                                                                                                                                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                         as
                                                                                                                                                                                                                         libc::c_ulong)).wrapping_div(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                                                          as
                                                                                                                                                                                                                                                          libc::c_ulong)
                       == 0 as libc::c_int as libc::c_ulong {
                    if __bt_pdelete(t, h) != 0 { return -(1 as libc::c_int) }
                } else {
                    kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                                   0x1 as libc::c_int as u_int);
                }
                return 0 as libc::c_int
            }
            deleted = 1 as libc::c_int;
            if !(((*e).index as libc::c_ulong) <
                     ((*h).lower as
                          libc::c_ulong).wrapping_sub((::std::mem::size_of::<db_pgno_t>()
                                                           as
                                                           libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                           as
                                                                                           libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                                                           as
                                                                                                                           libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                                                                                                           as
                                                                                                                                                           libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                           as
                                                                                                                                                                                           libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           libc::c_ulong)).wrapping_div(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                                                            as
                                                                                                                                                                                                                                                            libc::c_ulong)
                     && __kdb2_bt_cmp(t, key, e) == 0 as libc::c_int) {
                break ;
            }
        }
        /* Check for right-hand edge of the page. */
        if (*e).index as libc::c_ulong ==
               ((*h).lower as
                    libc::c_ulong).wrapping_sub((::std::mem::size_of::<db_pgno_t>()
                                                     as
                                                     libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                     as
                                                                                     libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                                                     as
                                                                                                                     libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                                                                                                     as
                                                                                                                                                     libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                     as
                                                                                                                                                                                     libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                     as
                                                                                                                                                                                                                     libc::c_ulong)).wrapping_div(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                                      libc::c_ulong)
           {
            redo = 1 as libc::c_int
        }
        loop 
             /* Delete from the key to the beginning of the page. */
             {
            let fresh2 = (*e).index;
            (*e).index = (*e).index.wrapping_sub(1);
            if !(fresh2 as libc::c_int > 0 as libc::c_int) { break ; }
            if __kdb2_bt_cmp(t, key, e) != 0 as libc::c_int { break ; }
            if __kdb2_bt_deleaf(t, key, h, (*e).index as u_int) ==
                   -(1 as libc::c_int) {
                kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                               0 as libc::c_int as u_int);
                return -(1 as libc::c_int)
            }
            if (*e).index as libc::c_int == 0 as libc::c_int {
                redo = 1 as libc::c_int
            }
        }
        /* Check for an empty page. */
        if ((*h).lower as
                libc::c_ulong).wrapping_sub((::std::mem::size_of::<db_pgno_t>()
                                                 as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                 as
                                                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                                                 as
                                                                                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                                                                                                 as
                                                                                                                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                 as
                                                                                                                                                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                 as
                                                                                                                                                                                                                 libc::c_ulong)).wrapping_div(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                                                  as
                                                                                                                                                                                                                                                  libc::c_ulong)
               == 0 as libc::c_int as libc::c_ulong {
            if __bt_pdelete(t, h) != 0 { return -(1 as libc::c_int) }
        } else {
            /* Put the page. */
            kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                           0x1 as libc::c_int as u_int);
            if !(redo != 0) { break ; }
        }
    }
    return 0 as libc::c_int;
}
/*
 * __bt_pdelete --
 *	Delete a single page from the tree.
 *
 * Parameters:
 *	t:	tree
 *	h:	leaf page
 *
 * Returns:
 *	RET_SUCCESS, RET_ERROR.
 *
 * Side-effects:
 *	mpool_put's the page
 */
#[c2rust::src_loc = "377:1"]
unsafe extern "C" fn __bt_pdelete(mut t: *mut BTREE, mut h: *mut PAGE)
 -> libc::c_int {
    let mut bi: *mut BINTERNAL = 0 as *mut BINTERNAL;
    let mut pg: *mut PAGE = 0 as *mut PAGE;
    let mut parent: *mut EPGNO = 0 as *mut EPGNO;
    let mut cnt: indx_t = 0;
    let mut idx: indx_t = 0;
    let mut ip: *mut indx_t = 0 as *mut indx_t;
    let mut offset: indx_t = 0;
    let mut nksize: u_int32_t = 0;
    let mut from: *mut libc::c_char = 0 as *mut libc::c_char;
    loop 
         /*
	 * Walk the parent page stack -- a LIFO stack of the pages that were
	 * traversed when we searched for the page where the delete occurred.
	 * Each stack entry is a page number and a page index offset.  The
	 * offset is for the page traversed on the search.  We've just deleted
	 * a page, so we have to delete the key from the parent page.
	 *
	 * If the delete from the parent page makes it empty, this process may
	 * continue all the way up the tree.  We stop if we reach the root page
	 * (which is never deleted, it's just not worth the effort) or if the
	 * delete does not empty the page.
	 */
         {
        parent =
            if (*t).bt_sp == (*t).bt_stack.as_mut_ptr() {
                0 as *mut EPGNO
            } else { (*t).bt_sp = (*t).bt_sp.offset(-1); (*t).bt_sp };
        if parent.is_null() { break ; }
        /* Get the parent page. */
        pg =
            kdb2_mpool_get((*t).bt_mp, (*parent).pgno,
                           0 as libc::c_int as u_int) as *mut PAGE;
        if pg.is_null() { return -(1 as libc::c_int) }
        idx = (*parent).index;
        bi =
            (pg as
                 *mut libc::c_char).offset(*(*pg).linp.as_mut_ptr().offset(idx
                                                                               as
                                                                               isize)
                                               as libc::c_int as isize) as
                *mut libc::c_void as *mut BINTERNAL;
        /* Free any overflow pages. */
        if (*bi).flags as libc::c_int & 0x2 as libc::c_int != 0 &&
               __kdb2_ovfl_delete(t,
                                  (*bi).bytes.as_mut_ptr() as
                                      *mut libc::c_void) ==
                   -(1 as libc::c_int) {
            kdb2_mpool_put((*t).bt_mp, pg as *mut libc::c_void,
                           0 as libc::c_int as u_int);
            return -(1 as libc::c_int)
        }
        /*
		 * Free the parent if it has only the one key and it's not the
		 * root page. If it's the rootpage, turn it back into an empty
		 * leaf page.
		 */
        if ((*pg).lower as
                libc::c_ulong).wrapping_sub((::std::mem::size_of::<db_pgno_t>()
                                                 as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                 as
                                                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                                                 as
                                                                                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                                                                                                 as
                                                                                                                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                 as
                                                                                                                                                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                 as
                                                                                                                                                                                                                 libc::c_ulong)).wrapping_div(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                                                  as
                                                                                                                                                                                                                                                  libc::c_ulong)
               == 1 as libc::c_int as libc::c_ulong {
            if (*pg).pgno == 1 as libc::c_int as libc::c_uint {
                (*pg).lower =
                    (::std::mem::size_of::<db_pgno_t>() as
                         libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                         as
                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                         as
                                                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                                                                         as
                                                                                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                         as
                                                                                                                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                         as
                                                                                                                                                                                         libc::c_ulong)
                        as indx_t;
                (*pg).upper = (*t).bt_psize as indx_t;
                (*pg).flags = 0x2 as libc::c_int as u_int32_t
            } else {
                if __kdb2_bt_relink(t, pg) != 0 || __kdb2_bt_free(t, pg) != 0
                   {
                    return -(1 as libc::c_int)
                }
                continue ;
            }
        } else {
            /* Pack remaining key items at the end of the page. */
            nksize =
                ((::std::mem::size_of::<u_int32_t>() as
                      libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                      as
                                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<u_char>()
                                                                                      as
                                                                                      libc::c_ulong).wrapping_add((*bi).ksize
                                                                                                                      as
                                                                                                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                                                                                      as
                                                                                                                                                      libc::c_ulong).wrapping_sub(1
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_int
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_ulong)
                     &
                     !(::std::mem::size_of::<db_pgno_t>() as
                           libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                           libc::c_ulong)) as
                    u_int32_t;
            from =
                (pg as
                     *mut libc::c_char).offset((*pg).upper as libc::c_int as
                                                   isize);
            memmove(from.offset(nksize as isize) as *mut libc::c_void,
                    from as *const libc::c_void,
                    (bi as *mut libc::c_char).wrapping_offset_from(from) as
                        libc::c_long as libc::c_ulong);
            (*pg).upper =
                ((*pg).upper as libc::c_uint).wrapping_add(nksize) as indx_t
                    as indx_t;
            /* Adjust indices' offsets, shift the indices down. */
            offset = *(*pg).linp.as_mut_ptr().offset(idx as isize);
            cnt = idx;
            ip =
                &mut *(*pg).linp.as_mut_ptr().offset(0 as libc::c_int as
                                                         isize) as
                    *mut indx_t;
            loop  {
                let fresh3 = cnt;
                cnt = cnt.wrapping_sub(1);
                if !(fresh3 != 0) { break ; }
                if (*ip.offset(0 as libc::c_int as isize) as libc::c_int) <
                       offset as libc::c_int {
                    let ref mut fresh4 =
                        *ip.offset(0 as libc::c_int as isize);
                    *fresh4 =
                        (*fresh4 as libc::c_uint).wrapping_add(nksize) as
                            indx_t as indx_t
                }
                ip = ip.offset(1)
            }
            cnt =
                ((*pg).lower as
                     libc::c_ulong).wrapping_sub((::std::mem::size_of::<db_pgno_t>()
                                                      as
                                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                      as
                                                                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                                                      as
                                                                                                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                                                                                                      as
                                                                                                                                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                      as
                                                                                                                                                                                                                      libc::c_ulong)).wrapping_div(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                                       libc::c_ulong).wrapping_sub(idx
                                                                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                                                                       libc::c_ulong)
                    as indx_t;
            loop  {
                cnt = cnt.wrapping_sub(1);
                if !(cnt != 0) { break ; }
                *ip.offset(0 as libc::c_int as isize) =
                    if (*ip.offset(1 as libc::c_int as isize) as libc::c_int)
                           < offset as libc::c_int {
                        (*ip.offset(1 as libc::c_int as isize) as
                             libc::c_uint).wrapping_add(nksize)
                    } else {
                        *ip.offset(1 as libc::c_int as isize) as libc::c_uint
                    } as indx_t;
                ip = ip.offset(1)
            }
            (*pg).lower =
                ((*pg).lower as
                     libc::c_ulong).wrapping_sub(::std::mem::size_of::<indx_t>()
                                                     as libc::c_ulong) as
                    indx_t as indx_t
        }
        kdb2_mpool_put((*t).bt_mp, pg as *mut libc::c_void,
                       0x1 as libc::c_int as u_int);
        break ;
    }
    /* Free the leaf page, as long as it wasn't the root. */
    if (*h).pgno == 1 as libc::c_int as libc::c_uint {
        kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                       0x1 as libc::c_int as u_int);
        return 0 as libc::c_int
    }
    return (__kdb2_bt_relink(t, h) != 0 || __kdb2_bt_free(t, h) != 0) as
               libc::c_int;
}
/*
 * __bt_dleaf --
 *	Delete a single record from a leaf page.
 *
 * Parameters:
 *	t:	tree
 *    key:	referenced key
 *	h:	page
 *	idx:	index on page to delete
 *
 * Returns:
 *	RET_SUCCESS, RET_ERROR.
 */
#[no_mangle]
#[c2rust::src_loc = "473:1"]
pub unsafe extern "C" fn __kdb2_bt_deleaf(mut t: *mut BTREE,
                                          mut key: *const DBT,
                                          mut h: *mut PAGE, mut idx: u_int)
 -> libc::c_int {
    let mut bl: *mut BLEAF = 0 as *mut BLEAF;
    let mut cnt: indx_t = 0;
    let mut ip: *mut indx_t = 0 as *mut indx_t;
    let mut offset: indx_t = 0;
    let mut nbytes: u_int32_t = 0;
    let mut to: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut from: *mut libc::c_char = 0 as *mut libc::c_char;
    /* If this record is referenced by the cursor, delete the cursor. */
    if (*t).bt_cursor.flags as libc::c_int & 0x8 as libc::c_int != 0 &&
           (*t).bt_cursor.flags as libc::c_int & 0x1 as libc::c_int == 0 &&
           (*t).bt_cursor.pg.pgno == (*h).pgno &&
           (*t).bt_cursor.pg.index as libc::c_uint == idx &&
           __bt_curdel(t, key, h, idx) != 0 {
        return -(1 as libc::c_int)
    }
    /* If the entry uses overflow pages, make them available for reuse. */
    bl =
        (h as
             *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(idx as
                                                                          isize)
                                           as libc::c_int as isize) as
            *mut libc::c_void as *mut BLEAF;
    to = bl as *mut libc::c_void;
    if (*bl).flags as libc::c_int & 0x2 as libc::c_int != 0 &&
           __kdb2_ovfl_delete(t,
                              (*bl).bytes.as_mut_ptr() as *mut libc::c_void)
               == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    }
    if (*bl).flags as libc::c_int & 0x1 as libc::c_int != 0 &&
           __kdb2_ovfl_delete(t,
                              (*bl).bytes.as_mut_ptr().offset((*bl).ksize as
                                                                  isize) as
                                  *mut libc::c_void) == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    }
    /* Pack the remaining key/data items at the end of the page. */
    nbytes =
        ((::std::mem::size_of::<u_int32_t>() as
              libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                              as
                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<u_char>()
                                                                              as
                                                                              libc::c_ulong).wrapping_add((*bl).ksize
                                                                                                              as
                                                                                                              libc::c_ulong).wrapping_add((*bl).dsize
                                                                                                                                              as
                                                                                                                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                                                                                                              as
                                                                                                                                                                              libc::c_ulong).wrapping_sub(1
                                                                                                                                                                                                              as
                                                                                                                                                                                                              libc::c_int
                                                                                                                                                                                                              as
                                                                                                                                                                                                              libc::c_ulong)
             &
             !(::std::mem::size_of::<db_pgno_t>() as
                   libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                   libc::c_ulong)) as
            u_int32_t;
    from =
        (h as *mut libc::c_char).offset((*h).upper as libc::c_int as isize);
    memmove(from.offset(nbytes as isize) as *mut libc::c_void,
            from as *const libc::c_void,
            (to as *mut libc::c_char).wrapping_offset_from(from) as
                libc::c_long as libc::c_ulong);
    (*h).upper =
        ((*h).upper as libc::c_uint).wrapping_add(nbytes) as indx_t as indx_t;
    /* Adjust the indices' offsets, shift the indices down. */
    offset = *(*h).linp.as_mut_ptr().offset(idx as isize);
    cnt = idx as indx_t;
    ip =
        &mut *(*h).linp.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut indx_t;
    loop  {
        let fresh5 = cnt;
        cnt = cnt.wrapping_sub(1);
        if !(fresh5 != 0) { break ; }
        if (*ip.offset(0 as libc::c_int as isize) as libc::c_int) <
               offset as libc::c_int {
            let ref mut fresh6 = *ip.offset(0 as libc::c_int as isize);
            *fresh6 =
                (*fresh6 as libc::c_uint).wrapping_add(nbytes) as indx_t as
                    indx_t
        }
        ip = ip.offset(1)
    }
    cnt =
        ((*h).lower as
             libc::c_ulong).wrapping_sub((::std::mem::size_of::<db_pgno_t>()
                                              as
                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                              as
                                                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                                              as
                                                                                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                                                                                              as
                                                                                                                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                              as
                                                                                                                                                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                              as
                                                                                                                                                                                                              libc::c_ulong)).wrapping_div(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                                               as
                                                                                                                                                                                                                                               libc::c_ulong).wrapping_sub(idx
                                                                                                                                                                                                                                                                               as
                                                                                                                                                                                                                                                                               libc::c_ulong)
            as indx_t;
    loop  {
        cnt = cnt.wrapping_sub(1);
        if !(cnt != 0) { break ; }
        *ip.offset(0 as libc::c_int as isize) =
            if (*ip.offset(1 as libc::c_int as isize) as libc::c_int) <
                   offset as libc::c_int {
                (*ip.offset(1 as libc::c_int as isize) as
                     libc::c_uint).wrapping_add(nbytes)
            } else { *ip.offset(1 as libc::c_int as isize) as libc::c_uint }
                as indx_t;
        ip = ip.offset(1)
    }
    (*h).lower =
        ((*h).lower as
             libc::c_ulong).wrapping_sub(::std::mem::size_of::<indx_t>() as
                                             libc::c_ulong) as indx_t as
            indx_t;
    /* If the cursor is on this page, adjust it as necessary. */
    if (*t).bt_cursor.flags as libc::c_int & 0x8 as libc::c_int != 0 &&
           (*t).bt_cursor.flags as libc::c_int & 0x1 as libc::c_int == 0 &&
           (*t).bt_cursor.pg.pgno == (*h).pgno &&
           (*t).bt_cursor.pg.index as libc::c_uint > idx {
        (*t).bt_cursor.pg.index = (*t).bt_cursor.pg.index.wrapping_sub(1)
    }
    return 0 as libc::c_int;
}
/*
 * __bt_curdel --
 *	Delete the cursor.
 *
 * Parameters:
 *	t:	tree
 *    key:	referenced key (or NULL)
 *	h:	page
 *  idx:	idx on page to delete
 *
 * Returns:
 *	RET_SUCCESS, RET_ERROR.
 */
#[c2rust::src_loc = "538:1"]
unsafe extern "C" fn __bt_curdel(mut t: *mut BTREE, mut key: *const DBT,
                                 mut h: *mut PAGE, mut idx: u_int)
 -> libc::c_int {
    let mut current_block: u64;
    let mut c: *mut CURSOR = 0 as *mut CURSOR;
    let mut e: EPG = EPG{page: 0 as *mut PAGE, index: 0,};
    let mut pg: *mut PAGE = 0 as *mut PAGE;
    let mut curcopy: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    /*
	 * If there are duplicates, move forward or backward to one.
	 * Otherwise, copy the key into the cursor area.
	 */
    c = &mut (*t).bt_cursor;
    (*c).flags =
        ((*c).flags as libc::c_int &
             !(0x2 as libc::c_int | 0x4 as libc::c_int | 0x1 as libc::c_int))
            as u_int8_t;
    curcopy = 0 as libc::c_int;
    if (*t).flags & 0x20 as libc::c_int as libc::c_uint == 0 {
        /*
		 * We're going to have to do comparisons.  If we weren't
		 * provided a copy of the key, i.e. the user is deleting
		 * the current cursor position, get one.
		 */
        if key.is_null() {
            e.page = h;
            e.index = idx as indx_t;
            status =
                __kdb2_bt_ret(t, &mut e, &mut (*c).key, &mut (*c).key,
                              0 as *mut DBT, 0 as *mut DBT, 1 as libc::c_int);
            if status != 0 as libc::c_int { return status }
            curcopy = 1 as libc::c_int;
            key = &mut (*c).key
        }
        /* Check previous key, if not at the beginning of the page. */
        if idx > 0 as libc::c_int as libc::c_uint {
            e.page = h;
            e.index =
                idx.wrapping_sub(1 as libc::c_int as libc::c_uint) as indx_t;
            if __kdb2_bt_cmp(t, key, &mut e) == 0 as libc::c_int {
                (*c).flags =
                    ((*c).flags as libc::c_int | 0x4 as libc::c_int) as
                        u_int8_t;
                current_block = 8931291464137740265;
            } else { current_block = 6009453772311597924; }
        } else { current_block = 6009453772311597924; }
        match current_block {
            6009453772311597924 =>
            /* Check next key, if not at the end of the page. */
            {
                if (idx as libc::c_ulong) <
                       ((*h).lower as
                            libc::c_ulong).wrapping_sub((::std::mem::size_of::<db_pgno_t>()
                                                             as
                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                             as
                                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                                                             as
                                                                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                                                                                                             as
                                                                                                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                             as
                                                                                                                                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                             as
                                                                                                                                                                                                                             libc::c_ulong)).wrapping_div(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                                                              as
                                                                                                                                                                                                                                                              libc::c_ulong).wrapping_sub(1
                                                                                                                                                                                                                                                                                              as
                                                                                                                                                                                                                                                                                              libc::c_int
                                                                                                                                                                                                                                                                                              as
                                                                                                                                                                                                                                                                                              libc::c_ulong)
                   {
                    e.page = h;
                    e.index =
                        idx.wrapping_add(1 as libc::c_int as libc::c_uint) as
                            indx_t;
                    if __kdb2_bt_cmp(t, key, &mut e) == 0 as libc::c_int {
                        (*c).flags =
                            ((*c).flags as libc::c_int | 0x2 as libc::c_int)
                                as u_int8_t;
                        current_block = 8931291464137740265;
                    } else { current_block = 10652014663920648156; }
                } else { current_block = 10652014663920648156; }
                match current_block {
                    8931291464137740265 => { }
                    _ =>
                    /* Check previous key if at the beginning of the page. */
                    {
                        if idx == 0 as libc::c_int as libc::c_uint &&
                               (*h).prevpg != 0 as libc::c_int as libc::c_uint
                           {
                            pg =
                                kdb2_mpool_get((*t).bt_mp, (*h).prevpg,
                                               0 as libc::c_int as u_int) as
                                    *mut PAGE;
                            if pg.is_null() { return -(1 as libc::c_int) }
                            e.page = pg;
                            e.index =
                                ((*pg).lower as
                                     libc::c_ulong).wrapping_sub((::std::mem::size_of::<db_pgno_t>()
                                                                      as
                                                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                                      as
                                                                                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                                                                      as
                                                                                                                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                                                                                                                      as
                                                                                                                                                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                      as
                                                                                                                                                                                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                      libc::c_ulong)).wrapping_div(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                                                       libc::c_ulong).wrapping_sub(1
                                                                                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                                                                                       libc::c_int
                                                                                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                                                                                       libc::c_ulong)
                                    as indx_t;
                            if __kdb2_bt_cmp(t, key, &mut e) ==
                                   0 as libc::c_int {
                                (*c).flags =
                                    ((*c).flags as libc::c_int |
                                         0x4 as libc::c_int) as u_int8_t;
                                current_block = 8926429821588478829;
                            } else {
                                kdb2_mpool_put((*t).bt_mp,
                                               pg as *mut libc::c_void,
                                               0 as libc::c_int as u_int);
                                current_block = 2891135413264362348;
                            }
                        } else { current_block = 2891135413264362348; }
                        match current_block {
                            2891135413264362348 =>
                            /* Check next key if at the end of the page. */
                            {
                                if idx as libc::c_ulong ==
                                       ((*h).lower as
                                            libc::c_ulong).wrapping_sub((::std::mem::size_of::<db_pgno_t>()
                                                                             as
                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                                             as
                                                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                                                                             as
                                                                                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                                                                                                                             as
                                                                                                                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                             as
                                                                                                                                                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                                             as
                                                                                                                                                                                                                                             libc::c_ulong)).wrapping_div(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                                                                              as
                                                                                                                                                                                                                                                                              libc::c_ulong).wrapping_sub(1
                                                                                                                                                                                                                                                                                                              as
                                                                                                                                                                                                                                                                                                              libc::c_int
                                                                                                                                                                                                                                                                                                              as
                                                                                                                                                                                                                                                                                                              libc::c_ulong)
                                       &&
                                       (*h).nextpg !=
                                           0 as libc::c_int as libc::c_uint {
                                    pg =
                                        kdb2_mpool_get((*t).bt_mp,
                                                       (*h).nextpg,
                                                       0 as libc::c_int as
                                                           u_int) as
                                            *mut PAGE;
                                    if pg.is_null() {
                                        return -(1 as libc::c_int)
                                    }
                                    e.page = pg;
                                    e.index = 0 as libc::c_int as indx_t;
                                    if __kdb2_bt_cmp(t, key, &mut e) ==
                                           0 as libc::c_int {
                                        (*c).flags =
                                            ((*c).flags as libc::c_int |
                                                 0x2 as libc::c_int) as
                                                u_int8_t;
                                        current_block = 8926429821588478829;
                                    } else {
                                        kdb2_mpool_put((*t).bt_mp,
                                                       pg as
                                                           *mut libc::c_void,
                                                       0 as libc::c_int as
                                                           u_int);
                                        current_block = 1356832168064818221;
                                    }
                                } else {
                                    current_block = 1356832168064818221;
                                }
                            }
                            _ => { }
                        }
                        match current_block {
                            1356832168064818221 => { }
                            _ => {
                                kdb2_mpool_put((*t).bt_mp,
                                               pg as *mut libc::c_void,
                                               0 as libc::c_int as u_int);
                                current_block = 8931291464137740265;
                            }
                        }
                    }
                }
            }
            _ => { }
        }
        match current_block {
            1356832168064818221 => { }
            _ => {
                (*c).pg.pgno = (*e.page).pgno;
                (*c).pg.index = e.index;
                return 0 as libc::c_int
            }
        }
    }
    e.page = h;
    e.index = idx as indx_t;
    if curcopy != 0 ||
           {
               status =
                   __kdb2_bt_ret(t, &mut e, &mut (*c).key, &mut (*c).key,
                                 0 as *mut DBT, 0 as *mut DBT,
                                 1 as libc::c_int);
               (status) == 0 as libc::c_int
           } {
        (*c).flags =
            ((*c).flags as libc::c_int | 0x1 as libc::c_int) as u_int8_t;
        return 0 as libc::c_int
    }
    return status;
}
/*
 * __bt_relink --
 *	Link around a deleted page.
 *
 * Parameters:
 *	t:	tree
 *	h:	page to be deleted
 */
#[no_mangle]
#[c2rust::src_loc = "637:1"]
pub unsafe extern "C" fn __kdb2_bt_relink(mut t: *mut BTREE, mut h: *mut PAGE)
 -> libc::c_int {
    let mut pg: *mut PAGE = 0 as *mut PAGE;
    if (*h).nextpg != 0 as libc::c_int as libc::c_uint {
        pg =
            kdb2_mpool_get((*t).bt_mp, (*h).nextpg, 0 as libc::c_int as u_int)
                as *mut PAGE;
        if pg.is_null() { return -(1 as libc::c_int) }
        (*pg).prevpg = (*h).prevpg;
        kdb2_mpool_put((*t).bt_mp, pg as *mut libc::c_void,
                       0x1 as libc::c_int as u_int);
    }
    if (*h).prevpg != 0 as libc::c_int as libc::c_uint {
        pg =
            kdb2_mpool_get((*t).bt_mp, (*h).prevpg, 0 as libc::c_int as u_int)
                as *mut PAGE;
        if pg.is_null() { return -(1 as libc::c_int) }
        (*pg).nextpg = (*h).nextpg;
        kdb2_mpool_put((*t).bt_mp, pg as *mut libc::c_void,
                       0x1 as libc::c_int as u_int);
    }
    return 0 as libc::c_int;
}
