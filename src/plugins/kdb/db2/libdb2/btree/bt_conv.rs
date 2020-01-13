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
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:43"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:43"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/db.h:46"]
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
    use super::stddef_h::size_t;
    use super::sys_types_h::u_int;
    /* !_DB_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/include/db-int.h:46"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/mpool/mpool.h:47"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/btree/btree.h:47"]
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
    /* Tree metadata page number. */
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
#[c2rust::header_src = "/usr/include/string.h:44"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
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
                        _lqh};
pub use self::btree_h::{_page, PAGE, _binternal, BINTERNAL, _bleaf, BLEAF,
                        _epgno, EPGNO, _epg, EPG, _cursor, CURSOR, _btree,
                        C2RustUnnamed_1, FORWARD, BACK, NOT, BTREE};
use self::string_h::memcpy;
/*
 * __BT_BPGIN, __BT_BPGOUT --
 *	Convert host-specific number layout to/from the host-independent
 *	format stored on disk.
 *
 * Parameters:
 *	t:	tree
 *	pg:	page number
 *	h:	page to convert
 */
#[no_mangle]
#[c2rust::src_loc = "61:1"]
pub unsafe extern "C" fn __kdb2_bt_pgin(mut t: *mut libc::c_void,
                                        mut pg: db_pgno_t,
                                        mut pp: *mut libc::c_void) {
    let mut h: *mut PAGE = 0 as *mut PAGE;
    let mut i: indx_t = 0;
    let mut top: indx_t = 0;
    let mut flags: u_char = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ksize: u_int32_t = 0;
    if (*(t as *mut BTREE)).flags & 0x8 as libc::c_int as libc::c_uint == 0 {
        return
    }
    if pg == 0 as libc::c_int as libc::c_uint {
        mswap(pp as *mut PAGE);
        return
    }
    h = pp as *mut PAGE;
    let mut _tmp: u_int32_t = (*h).pgno;
    *(&mut (*h).pgno as *mut db_pgno_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*h).pgno as *mut db_pgno_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*h).pgno as *mut db_pgno_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*h).pgno as *mut db_pgno_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_0: u_int32_t = (*h).prevpg;
    *(&mut (*h).prevpg as *mut db_pgno_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_0 as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*h).prevpg as *mut db_pgno_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_0 as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*h).prevpg as *mut db_pgno_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp_0 as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*h).prevpg as *mut db_pgno_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp_0 as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_1: u_int32_t = (*h).nextpg;
    *(&mut (*h).nextpg as *mut db_pgno_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_1 as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*h).nextpg as *mut db_pgno_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_1 as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*h).nextpg as *mut db_pgno_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp_1 as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*h).nextpg as *mut db_pgno_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp_1 as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_2: u_int32_t = (*h).flags;
    *(&mut (*h).flags as *mut u_int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_2 as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*h).flags as *mut u_int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_2 as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*h).flags as *mut u_int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp_2 as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*h).flags as *mut u_int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp_2 as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_3: u_int16_t = (*h).lower;
    *(&mut (*h).lower as *mut indx_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_3 as *mut u_int16_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*h).lower as *mut indx_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_3 as *mut u_int16_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_4: u_int16_t = (*h).upper;
    *(&mut (*h).upper as *mut indx_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_4 as *mut u_int16_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*h).upper as *mut indx_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_4 as *mut u_int16_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    top =
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
            as indx_t;
    if (*h).flags & 0x1f as libc::c_int as libc::c_uint ==
           0x1 as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as indx_t;
        while (i as libc::c_int) < top as libc::c_int {
            let mut _tmp_5: u_int16_t =
                *(*h).linp.as_mut_ptr().offset(i as isize);
            *(&mut *(*h).linp.as_mut_ptr().offset(i as isize) as *mut indx_t
                  as *mut libc::c_char).offset(0 as libc::c_int as isize) =
                *(&mut _tmp_5 as *mut u_int16_t as
                      *mut libc::c_char).offset(1 as libc::c_int as isize);
            *(&mut *(*h).linp.as_mut_ptr().offset(i as isize) as *mut indx_t
                  as *mut libc::c_char).offset(1 as libc::c_int as isize) =
                *(&mut _tmp_5 as *mut u_int16_t as
                      *mut libc::c_char).offset(0 as libc::c_int as isize);
            p =
                (h as
                     *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(i
                                                                                  as
                                                                                  isize)
                                                   as libc::c_int as isize) as
                    *mut libc::c_void as *mut BINTERNAL as *mut libc::c_char;
            let mut _tmp_6: [libc::c_char; 4] = [0; 4];
            _tmp_6[0 as libc::c_int as usize] =
                *p.offset(0 as libc::c_int as isize);
            _tmp_6[1 as libc::c_int as usize] =
                *p.offset(1 as libc::c_int as isize);
            _tmp_6[2 as libc::c_int as usize] =
                *p.offset(2 as libc::c_int as isize);
            _tmp_6[3 as libc::c_int as usize] =
                *p.offset(3 as libc::c_int as isize);
            *p.offset(0 as libc::c_int as isize) =
                _tmp_6[3 as libc::c_int as usize];
            *p.offset(1 as libc::c_int as isize) =
                _tmp_6[2 as libc::c_int as usize];
            *p.offset(2 as libc::c_int as isize) =
                _tmp_6[1 as libc::c_int as usize];
            *p.offset(3 as libc::c_int as isize) =
                _tmp_6[0 as libc::c_int as usize];
            p =
                p.offset(::std::mem::size_of::<u_int32_t>() as libc::c_ulong
                             as isize);
            let mut _tmp_7: [libc::c_char; 4] = [0; 4];
            _tmp_7[0 as libc::c_int as usize] =
                *p.offset(0 as libc::c_int as isize);
            _tmp_7[1 as libc::c_int as usize] =
                *p.offset(1 as libc::c_int as isize);
            _tmp_7[2 as libc::c_int as usize] =
                *p.offset(2 as libc::c_int as isize);
            _tmp_7[3 as libc::c_int as usize] =
                *p.offset(3 as libc::c_int as isize);
            *p.offset(0 as libc::c_int as isize) =
                _tmp_7[3 as libc::c_int as usize];
            *p.offset(1 as libc::c_int as isize) =
                _tmp_7[2 as libc::c_int as usize];
            *p.offset(2 as libc::c_int as isize) =
                _tmp_7[1 as libc::c_int as usize];
            *p.offset(3 as libc::c_int as isize) =
                _tmp_7[0 as libc::c_int as usize];
            p =
                p.offset(::std::mem::size_of::<db_pgno_t>() as libc::c_ulong
                             as isize);
            if *(p as *mut u_char) as libc::c_int & 0x2 as libc::c_int != 0 {
                p =
                    p.offset(::std::mem::size_of::<u_char>() as libc::c_ulong
                                 as isize);
                let mut _tmp_8: [libc::c_char; 4] = [0; 4];
                _tmp_8[0 as libc::c_int as usize] =
                    *p.offset(0 as libc::c_int as isize);
                _tmp_8[1 as libc::c_int as usize] =
                    *p.offset(1 as libc::c_int as isize);
                _tmp_8[2 as libc::c_int as usize] =
                    *p.offset(2 as libc::c_int as isize);
                _tmp_8[3 as libc::c_int as usize] =
                    *p.offset(3 as libc::c_int as isize);
                *p.offset(0 as libc::c_int as isize) =
                    _tmp_8[3 as libc::c_int as usize];
                *p.offset(1 as libc::c_int as isize) =
                    _tmp_8[2 as libc::c_int as usize];
                *p.offset(2 as libc::c_int as isize) =
                    _tmp_8[1 as libc::c_int as usize];
                *p.offset(3 as libc::c_int as isize) =
                    _tmp_8[0 as libc::c_int as usize];
                p =
                    p.offset(::std::mem::size_of::<db_pgno_t>() as
                                 libc::c_ulong as isize);
                let mut _tmp_9: [libc::c_char; 4] = [0; 4];
                _tmp_9[0 as libc::c_int as usize] =
                    *p.offset(0 as libc::c_int as isize);
                _tmp_9[1 as libc::c_int as usize] =
                    *p.offset(1 as libc::c_int as isize);
                _tmp_9[2 as libc::c_int as usize] =
                    *p.offset(2 as libc::c_int as isize);
                _tmp_9[3 as libc::c_int as usize] =
                    *p.offset(3 as libc::c_int as isize);
                *p.offset(0 as libc::c_int as isize) =
                    _tmp_9[3 as libc::c_int as usize];
                *p.offset(1 as libc::c_int as isize) =
                    _tmp_9[2 as libc::c_int as usize];
                *p.offset(2 as libc::c_int as isize) =
                    _tmp_9[1 as libc::c_int as usize];
                *p.offset(3 as libc::c_int as isize) =
                    _tmp_9[0 as libc::c_int as usize]
            }
            i = i.wrapping_add(1)
        }
    } else if (*h).flags & 0x1f as libc::c_int as libc::c_uint ==
                  0x2 as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as indx_t;
        while (i as libc::c_int) < top as libc::c_int {
            let mut _tmp_10: u_int16_t =
                *(*h).linp.as_mut_ptr().offset(i as isize);
            *(&mut *(*h).linp.as_mut_ptr().offset(i as isize) as *mut indx_t
                  as *mut libc::c_char).offset(0 as libc::c_int as isize) =
                *(&mut _tmp_10 as *mut u_int16_t as
                      *mut libc::c_char).offset(1 as libc::c_int as isize);
            *(&mut *(*h).linp.as_mut_ptr().offset(i as isize) as *mut indx_t
                  as *mut libc::c_char).offset(1 as libc::c_int as isize) =
                *(&mut _tmp_10 as *mut u_int16_t as
                      *mut libc::c_char).offset(0 as libc::c_int as isize);
            p =
                (h as
                     *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(i
                                                                                  as
                                                                                  isize)
                                                   as libc::c_int as isize) as
                    *mut libc::c_void as *mut BLEAF as *mut libc::c_char;
            let mut _tmp_11: [libc::c_char; 4] = [0; 4];
            _tmp_11[0 as libc::c_int as usize] =
                *p.offset(0 as libc::c_int as isize);
            _tmp_11[1 as libc::c_int as usize] =
                *p.offset(1 as libc::c_int as isize);
            _tmp_11[2 as libc::c_int as usize] =
                *p.offset(2 as libc::c_int as isize);
            _tmp_11[3 as libc::c_int as usize] =
                *p.offset(3 as libc::c_int as isize);
            *p.offset(0 as libc::c_int as isize) =
                _tmp_11[3 as libc::c_int as usize];
            *p.offset(1 as libc::c_int as isize) =
                _tmp_11[2 as libc::c_int as usize];
            *p.offset(2 as libc::c_int as isize) =
                _tmp_11[1 as libc::c_int as usize];
            *p.offset(3 as libc::c_int as isize) =
                _tmp_11[0 as libc::c_int as usize];
            memcpy(&mut ksize as *mut u_int32_t as *mut libc::c_void,
                   p as *const libc::c_void,
                   ::std::mem::size_of::<u_int32_t>() as libc::c_ulong);
            p =
                p.offset(::std::mem::size_of::<u_int32_t>() as libc::c_ulong
                             as isize);
            let mut _tmp_12: [libc::c_char; 4] = [0; 4];
            _tmp_12[0 as libc::c_int as usize] =
                *p.offset(0 as libc::c_int as isize);
            _tmp_12[1 as libc::c_int as usize] =
                *p.offset(1 as libc::c_int as isize);
            _tmp_12[2 as libc::c_int as usize] =
                *p.offset(2 as libc::c_int as isize);
            _tmp_12[3 as libc::c_int as usize] =
                *p.offset(3 as libc::c_int as isize);
            *p.offset(0 as libc::c_int as isize) =
                _tmp_12[3 as libc::c_int as usize];
            *p.offset(1 as libc::c_int as isize) =
                _tmp_12[2 as libc::c_int as usize];
            *p.offset(2 as libc::c_int as isize) =
                _tmp_12[1 as libc::c_int as usize];
            *p.offset(3 as libc::c_int as isize) =
                _tmp_12[0 as libc::c_int as usize];
            p =
                p.offset(::std::mem::size_of::<u_int32_t>() as libc::c_ulong
                             as isize);
            flags = *(p as *mut u_char);
            if flags as libc::c_int &
                   (0x2 as libc::c_int | 0x1 as libc::c_int) != 0 {
                p =
                    p.offset(::std::mem::size_of::<u_char>() as libc::c_ulong
                                 as isize);
                if flags as libc::c_int & 0x2 as libc::c_int != 0 {
                    let mut _tmp_13: [libc::c_char; 4] = [0; 4];
                    _tmp_13[0 as libc::c_int as usize] =
                        *p.offset(0 as libc::c_int as isize);
                    _tmp_13[1 as libc::c_int as usize] =
                        *p.offset(1 as libc::c_int as isize);
                    _tmp_13[2 as libc::c_int as usize] =
                        *p.offset(2 as libc::c_int as isize);
                    _tmp_13[3 as libc::c_int as usize] =
                        *p.offset(3 as libc::c_int as isize);
                    *p.offset(0 as libc::c_int as isize) =
                        _tmp_13[3 as libc::c_int as usize];
                    *p.offset(1 as libc::c_int as isize) =
                        _tmp_13[2 as libc::c_int as usize];
                    *p.offset(2 as libc::c_int as isize) =
                        _tmp_13[1 as libc::c_int as usize];
                    *p.offset(3 as libc::c_int as isize) =
                        _tmp_13[0 as libc::c_int as usize];
                    let mut _tmp_14: [libc::c_char; 4] = [0; 4];
                    _tmp_14[0 as libc::c_int as usize] =
                        *p.offset(::std::mem::size_of::<db_pgno_t>() as
                                      libc::c_ulong as
                                      isize).offset(0 as libc::c_int as
                                                        isize);
                    _tmp_14[1 as libc::c_int as usize] =
                        *p.offset(::std::mem::size_of::<db_pgno_t>() as
                                      libc::c_ulong as
                                      isize).offset(1 as libc::c_int as
                                                        isize);
                    _tmp_14[2 as libc::c_int as usize] =
                        *p.offset(::std::mem::size_of::<db_pgno_t>() as
                                      libc::c_ulong as
                                      isize).offset(2 as libc::c_int as
                                                        isize);
                    _tmp_14[3 as libc::c_int as usize] =
                        *p.offset(::std::mem::size_of::<db_pgno_t>() as
                                      libc::c_ulong as
                                      isize).offset(3 as libc::c_int as
                                                        isize);
                    *p.offset(::std::mem::size_of::<db_pgno_t>() as
                                  libc::c_ulong as
                                  isize).offset(0 as libc::c_int as isize) =
                        _tmp_14[3 as libc::c_int as usize];
                    *p.offset(::std::mem::size_of::<db_pgno_t>() as
                                  libc::c_ulong as
                                  isize).offset(1 as libc::c_int as isize) =
                        _tmp_14[2 as libc::c_int as usize];
                    *p.offset(::std::mem::size_of::<db_pgno_t>() as
                                  libc::c_ulong as
                                  isize).offset(2 as libc::c_int as isize) =
                        _tmp_14[1 as libc::c_int as usize];
                    *p.offset(::std::mem::size_of::<db_pgno_t>() as
                                  libc::c_ulong as
                                  isize).offset(3 as libc::c_int as isize) =
                        _tmp_14[0 as libc::c_int as usize]
                }
                if flags as libc::c_int & 0x1 as libc::c_int != 0 {
                    p = p.offset(ksize as isize);
                    let mut _tmp_15: [libc::c_char; 4] = [0; 4];
                    _tmp_15[0 as libc::c_int as usize] =
                        *p.offset(0 as libc::c_int as isize);
                    _tmp_15[1 as libc::c_int as usize] =
                        *p.offset(1 as libc::c_int as isize);
                    _tmp_15[2 as libc::c_int as usize] =
                        *p.offset(2 as libc::c_int as isize);
                    _tmp_15[3 as libc::c_int as usize] =
                        *p.offset(3 as libc::c_int as isize);
                    *p.offset(0 as libc::c_int as isize) =
                        _tmp_15[3 as libc::c_int as usize];
                    *p.offset(1 as libc::c_int as isize) =
                        _tmp_15[2 as libc::c_int as usize];
                    *p.offset(2 as libc::c_int as isize) =
                        _tmp_15[1 as libc::c_int as usize];
                    *p.offset(3 as libc::c_int as isize) =
                        _tmp_15[0 as libc::c_int as usize];
                    p =
                        p.offset(::std::mem::size_of::<db_pgno_t>() as
                                     libc::c_ulong as isize);
                    let mut _tmp_16: [libc::c_char; 4] = [0; 4];
                    _tmp_16[0 as libc::c_int as usize] =
                        *p.offset(0 as libc::c_int as isize);
                    _tmp_16[1 as libc::c_int as usize] =
                        *p.offset(1 as libc::c_int as isize);
                    _tmp_16[2 as libc::c_int as usize] =
                        *p.offset(2 as libc::c_int as isize);
                    _tmp_16[3 as libc::c_int as usize] =
                        *p.offset(3 as libc::c_int as isize);
                    *p.offset(0 as libc::c_int as isize) =
                        _tmp_16[3 as libc::c_int as usize];
                    *p.offset(1 as libc::c_int as isize) =
                        _tmp_16[2 as libc::c_int as usize];
                    *p.offset(2 as libc::c_int as isize) =
                        _tmp_16[1 as libc::c_int as usize];
                    *p.offset(3 as libc::c_int as isize) =
                        _tmp_16[0 as libc::c_int as usize]
                }
            }
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "130:1"]
pub unsafe extern "C" fn __kdb2_bt_pgout(mut t: *mut libc::c_void,
                                         mut pg: db_pgno_t,
                                         mut pp: *mut libc::c_void) {
    let mut h: *mut PAGE = 0 as *mut PAGE;
    let mut i: indx_t = 0;
    let mut top: indx_t = 0;
    let mut flags: u_char = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ksize: u_int32_t = 0;
    if (*(t as *mut BTREE)).flags & 0x8 as libc::c_int as libc::c_uint == 0 {
        return
    }
    if pg == 0 as libc::c_int as libc::c_uint {
        mswap(pp as *mut PAGE);
        return
    }
    h = pp as *mut PAGE;
    top =
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
            as indx_t;
    if (*h).flags & 0x1f as libc::c_int as libc::c_uint ==
           0x1 as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as indx_t;
        while (i as libc::c_int) < top as libc::c_int {
            p =
                (h as
                     *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(i
                                                                                  as
                                                                                  isize)
                                                   as libc::c_int as isize) as
                    *mut libc::c_void as *mut BINTERNAL as *mut libc::c_char;
            let mut _tmp: [libc::c_char; 4] = [0; 4];
            _tmp[0 as libc::c_int as usize] =
                *p.offset(0 as libc::c_int as isize);
            _tmp[1 as libc::c_int as usize] =
                *p.offset(1 as libc::c_int as isize);
            _tmp[2 as libc::c_int as usize] =
                *p.offset(2 as libc::c_int as isize);
            _tmp[3 as libc::c_int as usize] =
                *p.offset(3 as libc::c_int as isize);
            *p.offset(0 as libc::c_int as isize) =
                _tmp[3 as libc::c_int as usize];
            *p.offset(1 as libc::c_int as isize) =
                _tmp[2 as libc::c_int as usize];
            *p.offset(2 as libc::c_int as isize) =
                _tmp[1 as libc::c_int as usize];
            *p.offset(3 as libc::c_int as isize) =
                _tmp[0 as libc::c_int as usize];
            p =
                p.offset(::std::mem::size_of::<u_int32_t>() as libc::c_ulong
                             as isize);
            let mut _tmp_0: [libc::c_char; 4] = [0; 4];
            _tmp_0[0 as libc::c_int as usize] =
                *p.offset(0 as libc::c_int as isize);
            _tmp_0[1 as libc::c_int as usize] =
                *p.offset(1 as libc::c_int as isize);
            _tmp_0[2 as libc::c_int as usize] =
                *p.offset(2 as libc::c_int as isize);
            _tmp_0[3 as libc::c_int as usize] =
                *p.offset(3 as libc::c_int as isize);
            *p.offset(0 as libc::c_int as isize) =
                _tmp_0[3 as libc::c_int as usize];
            *p.offset(1 as libc::c_int as isize) =
                _tmp_0[2 as libc::c_int as usize];
            *p.offset(2 as libc::c_int as isize) =
                _tmp_0[1 as libc::c_int as usize];
            *p.offset(3 as libc::c_int as isize) =
                _tmp_0[0 as libc::c_int as usize];
            p =
                p.offset(::std::mem::size_of::<db_pgno_t>() as libc::c_ulong
                             as isize);
            if *(p as *mut u_char) as libc::c_int & 0x2 as libc::c_int != 0 {
                p =
                    p.offset(::std::mem::size_of::<u_char>() as libc::c_ulong
                                 as isize);
                let mut _tmp_1: [libc::c_char; 4] = [0; 4];
                _tmp_1[0 as libc::c_int as usize] =
                    *p.offset(0 as libc::c_int as isize);
                _tmp_1[1 as libc::c_int as usize] =
                    *p.offset(1 as libc::c_int as isize);
                _tmp_1[2 as libc::c_int as usize] =
                    *p.offset(2 as libc::c_int as isize);
                _tmp_1[3 as libc::c_int as usize] =
                    *p.offset(3 as libc::c_int as isize);
                *p.offset(0 as libc::c_int as isize) =
                    _tmp_1[3 as libc::c_int as usize];
                *p.offset(1 as libc::c_int as isize) =
                    _tmp_1[2 as libc::c_int as usize];
                *p.offset(2 as libc::c_int as isize) =
                    _tmp_1[1 as libc::c_int as usize];
                *p.offset(3 as libc::c_int as isize) =
                    _tmp_1[0 as libc::c_int as usize];
                p =
                    p.offset(::std::mem::size_of::<db_pgno_t>() as
                                 libc::c_ulong as isize);
                let mut _tmp_2: [libc::c_char; 4] = [0; 4];
                _tmp_2[0 as libc::c_int as usize] =
                    *p.offset(0 as libc::c_int as isize);
                _tmp_2[1 as libc::c_int as usize] =
                    *p.offset(1 as libc::c_int as isize);
                _tmp_2[2 as libc::c_int as usize] =
                    *p.offset(2 as libc::c_int as isize);
                _tmp_2[3 as libc::c_int as usize] =
                    *p.offset(3 as libc::c_int as isize);
                *p.offset(0 as libc::c_int as isize) =
                    _tmp_2[3 as libc::c_int as usize];
                *p.offset(1 as libc::c_int as isize) =
                    _tmp_2[2 as libc::c_int as usize];
                *p.offset(2 as libc::c_int as isize) =
                    _tmp_2[1 as libc::c_int as usize];
                *p.offset(3 as libc::c_int as isize) =
                    _tmp_2[0 as libc::c_int as usize]
            }
            let mut _tmp_3: u_int16_t =
                *(*h).linp.as_mut_ptr().offset(i as isize);
            *(&mut *(*h).linp.as_mut_ptr().offset(i as isize) as *mut indx_t
                  as *mut libc::c_char).offset(0 as libc::c_int as isize) =
                *(&mut _tmp_3 as *mut u_int16_t as
                      *mut libc::c_char).offset(1 as libc::c_int as isize);
            *(&mut *(*h).linp.as_mut_ptr().offset(i as isize) as *mut indx_t
                  as *mut libc::c_char).offset(1 as libc::c_int as isize) =
                *(&mut _tmp_3 as *mut u_int16_t as
                      *mut libc::c_char).offset(0 as libc::c_int as isize);
            i = i.wrapping_add(1)
        }
    } else if (*h).flags & 0x1f as libc::c_int as libc::c_uint ==
                  0x2 as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as indx_t;
        while (i as libc::c_int) < top as libc::c_int {
            ksize =
                (*((h as
                        *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(i
                                                                                     as
                                                                                     isize)
                                                      as libc::c_int as isize)
                       as *mut libc::c_void as *mut BLEAF)).ksize;
            p =
                (h as
                     *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(i
                                                                                  as
                                                                                  isize)
                                                   as libc::c_int as isize) as
                    *mut libc::c_void as *mut BLEAF as *mut libc::c_char;
            let mut _tmp_4: [libc::c_char; 4] = [0; 4];
            _tmp_4[0 as libc::c_int as usize] =
                *p.offset(0 as libc::c_int as isize);
            _tmp_4[1 as libc::c_int as usize] =
                *p.offset(1 as libc::c_int as isize);
            _tmp_4[2 as libc::c_int as usize] =
                *p.offset(2 as libc::c_int as isize);
            _tmp_4[3 as libc::c_int as usize] =
                *p.offset(3 as libc::c_int as isize);
            *p.offset(0 as libc::c_int as isize) =
                _tmp_4[3 as libc::c_int as usize];
            *p.offset(1 as libc::c_int as isize) =
                _tmp_4[2 as libc::c_int as usize];
            *p.offset(2 as libc::c_int as isize) =
                _tmp_4[1 as libc::c_int as usize];
            *p.offset(3 as libc::c_int as isize) =
                _tmp_4[0 as libc::c_int as usize];
            p =
                p.offset(::std::mem::size_of::<u_int32_t>() as libc::c_ulong
                             as isize);
            let mut _tmp_5: [libc::c_char; 4] = [0; 4];
            _tmp_5[0 as libc::c_int as usize] =
                *p.offset(0 as libc::c_int as isize);
            _tmp_5[1 as libc::c_int as usize] =
                *p.offset(1 as libc::c_int as isize);
            _tmp_5[2 as libc::c_int as usize] =
                *p.offset(2 as libc::c_int as isize);
            _tmp_5[3 as libc::c_int as usize] =
                *p.offset(3 as libc::c_int as isize);
            *p.offset(0 as libc::c_int as isize) =
                _tmp_5[3 as libc::c_int as usize];
            *p.offset(1 as libc::c_int as isize) =
                _tmp_5[2 as libc::c_int as usize];
            *p.offset(2 as libc::c_int as isize) =
                _tmp_5[1 as libc::c_int as usize];
            *p.offset(3 as libc::c_int as isize) =
                _tmp_5[0 as libc::c_int as usize];
            p =
                p.offset(::std::mem::size_of::<u_int32_t>() as libc::c_ulong
                             as isize);
            flags = *(p as *mut u_char);
            if flags as libc::c_int &
                   (0x2 as libc::c_int | 0x1 as libc::c_int) != 0 {
                p =
                    p.offset(::std::mem::size_of::<u_char>() as libc::c_ulong
                                 as isize);
                if flags as libc::c_int & 0x2 as libc::c_int != 0 {
                    let mut _tmp_6: [libc::c_char; 4] = [0; 4];
                    _tmp_6[0 as libc::c_int as usize] =
                        *p.offset(0 as libc::c_int as isize);
                    _tmp_6[1 as libc::c_int as usize] =
                        *p.offset(1 as libc::c_int as isize);
                    _tmp_6[2 as libc::c_int as usize] =
                        *p.offset(2 as libc::c_int as isize);
                    _tmp_6[3 as libc::c_int as usize] =
                        *p.offset(3 as libc::c_int as isize);
                    *p.offset(0 as libc::c_int as isize) =
                        _tmp_6[3 as libc::c_int as usize];
                    *p.offset(1 as libc::c_int as isize) =
                        _tmp_6[2 as libc::c_int as usize];
                    *p.offset(2 as libc::c_int as isize) =
                        _tmp_6[1 as libc::c_int as usize];
                    *p.offset(3 as libc::c_int as isize) =
                        _tmp_6[0 as libc::c_int as usize];
                    let mut _tmp_7: [libc::c_char; 4] = [0; 4];
                    _tmp_7[0 as libc::c_int as usize] =
                        *p.offset(::std::mem::size_of::<db_pgno_t>() as
                                      libc::c_ulong as
                                      isize).offset(0 as libc::c_int as
                                                        isize);
                    _tmp_7[1 as libc::c_int as usize] =
                        *p.offset(::std::mem::size_of::<db_pgno_t>() as
                                      libc::c_ulong as
                                      isize).offset(1 as libc::c_int as
                                                        isize);
                    _tmp_7[2 as libc::c_int as usize] =
                        *p.offset(::std::mem::size_of::<db_pgno_t>() as
                                      libc::c_ulong as
                                      isize).offset(2 as libc::c_int as
                                                        isize);
                    _tmp_7[3 as libc::c_int as usize] =
                        *p.offset(::std::mem::size_of::<db_pgno_t>() as
                                      libc::c_ulong as
                                      isize).offset(3 as libc::c_int as
                                                        isize);
                    *p.offset(::std::mem::size_of::<db_pgno_t>() as
                                  libc::c_ulong as
                                  isize).offset(0 as libc::c_int as isize) =
                        _tmp_7[3 as libc::c_int as usize];
                    *p.offset(::std::mem::size_of::<db_pgno_t>() as
                                  libc::c_ulong as
                                  isize).offset(1 as libc::c_int as isize) =
                        _tmp_7[2 as libc::c_int as usize];
                    *p.offset(::std::mem::size_of::<db_pgno_t>() as
                                  libc::c_ulong as
                                  isize).offset(2 as libc::c_int as isize) =
                        _tmp_7[1 as libc::c_int as usize];
                    *p.offset(::std::mem::size_of::<db_pgno_t>() as
                                  libc::c_ulong as
                                  isize).offset(3 as libc::c_int as isize) =
                        _tmp_7[0 as libc::c_int as usize]
                }
                if flags as libc::c_int & 0x1 as libc::c_int != 0 {
                    p = p.offset(ksize as isize);
                    let mut _tmp_8: [libc::c_char; 4] = [0; 4];
                    _tmp_8[0 as libc::c_int as usize] =
                        *p.offset(0 as libc::c_int as isize);
                    _tmp_8[1 as libc::c_int as usize] =
                        *p.offset(1 as libc::c_int as isize);
                    _tmp_8[2 as libc::c_int as usize] =
                        *p.offset(2 as libc::c_int as isize);
                    _tmp_8[3 as libc::c_int as usize] =
                        *p.offset(3 as libc::c_int as isize);
                    *p.offset(0 as libc::c_int as isize) =
                        _tmp_8[3 as libc::c_int as usize];
                    *p.offset(1 as libc::c_int as isize) =
                        _tmp_8[2 as libc::c_int as usize];
                    *p.offset(2 as libc::c_int as isize) =
                        _tmp_8[1 as libc::c_int as usize];
                    *p.offset(3 as libc::c_int as isize) =
                        _tmp_8[0 as libc::c_int as usize];
                    p =
                        p.offset(::std::mem::size_of::<db_pgno_t>() as
                                     libc::c_ulong as isize);
                    let mut _tmp_9: [libc::c_char; 4] = [0; 4];
                    _tmp_9[0 as libc::c_int as usize] =
                        *p.offset(0 as libc::c_int as isize);
                    _tmp_9[1 as libc::c_int as usize] =
                        *p.offset(1 as libc::c_int as isize);
                    _tmp_9[2 as libc::c_int as usize] =
                        *p.offset(2 as libc::c_int as isize);
                    _tmp_9[3 as libc::c_int as usize] =
                        *p.offset(3 as libc::c_int as isize);
                    *p.offset(0 as libc::c_int as isize) =
                        _tmp_9[3 as libc::c_int as usize];
                    *p.offset(1 as libc::c_int as isize) =
                        _tmp_9[2 as libc::c_int as usize];
                    *p.offset(2 as libc::c_int as isize) =
                        _tmp_9[1 as libc::c_int as usize];
                    *p.offset(3 as libc::c_int as isize) =
                        _tmp_9[0 as libc::c_int as usize]
                }
            }
            let mut _tmp_10: u_int16_t =
                *(*h).linp.as_mut_ptr().offset(i as isize);
            *(&mut *(*h).linp.as_mut_ptr().offset(i as isize) as *mut indx_t
                  as *mut libc::c_char).offset(0 as libc::c_int as isize) =
                *(&mut _tmp_10 as *mut u_int16_t as
                      *mut libc::c_char).offset(1 as libc::c_int as isize);
            *(&mut *(*h).linp.as_mut_ptr().offset(i as isize) as *mut indx_t
                  as *mut libc::c_char).offset(1 as libc::c_int as isize) =
                *(&mut _tmp_10 as *mut u_int16_t as
                      *mut libc::c_char).offset(0 as libc::c_int as isize);
            i = i.wrapping_add(1)
        }
    }
    let mut _tmp_11: u_int32_t = (*h).pgno;
    *(&mut (*h).pgno as *mut db_pgno_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_11 as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*h).pgno as *mut db_pgno_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_11 as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*h).pgno as *mut db_pgno_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp_11 as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*h).pgno as *mut db_pgno_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp_11 as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_12: u_int32_t = (*h).prevpg;
    *(&mut (*h).prevpg as *mut db_pgno_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_12 as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*h).prevpg as *mut db_pgno_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_12 as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*h).prevpg as *mut db_pgno_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp_12 as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*h).prevpg as *mut db_pgno_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp_12 as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_13: u_int32_t = (*h).nextpg;
    *(&mut (*h).nextpg as *mut db_pgno_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_13 as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*h).nextpg as *mut db_pgno_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_13 as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*h).nextpg as *mut db_pgno_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp_13 as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*h).nextpg as *mut db_pgno_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp_13 as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_14: u_int32_t = (*h).flags;
    *(&mut (*h).flags as *mut u_int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_14 as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*h).flags as *mut u_int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_14 as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*h).flags as *mut u_int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp_14 as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*h).flags as *mut u_int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp_14 as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_15: u_int16_t = (*h).lower;
    *(&mut (*h).lower as *mut indx_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_15 as *mut u_int16_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*h).lower as *mut indx_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_15 as *mut u_int16_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_16: u_int16_t = (*h).upper;
    *(&mut (*h).upper as *mut indx_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_16 as *mut u_int16_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*h).upper as *mut indx_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_16 as *mut u_int16_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
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
 * MSWAP -- Actually swap the bytes on the meta page.
 *
 * Parameters:
 *	p:	page to convert
 */
#[c2rust::src_loc = "205:1"]
unsafe extern "C" fn mswap(mut pg: *mut PAGE) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = pg as *mut libc::c_char;
    let mut _tmp: [libc::c_char; 4] = [0; 4];
    _tmp[0 as libc::c_int as usize] = *p.offset(0 as libc::c_int as isize);
    _tmp[1 as libc::c_int as usize] = *p.offset(1 as libc::c_int as isize);
    _tmp[2 as libc::c_int as usize] = *p.offset(2 as libc::c_int as isize);
    _tmp[3 as libc::c_int as usize] = *p.offset(3 as libc::c_int as isize);
    *p.offset(0 as libc::c_int as isize) = _tmp[3 as libc::c_int as usize];
    *p.offset(1 as libc::c_int as isize) = _tmp[2 as libc::c_int as usize];
    *p.offset(2 as libc::c_int as isize) = _tmp[1 as libc::c_int as usize];
    *p.offset(3 as libc::c_int as isize) = _tmp[0 as libc::c_int as usize];
    /* magic */
    p =
        p.offset(::std::mem::size_of::<u_int32_t>() as libc::c_ulong as
                     isize);
    let mut _tmp_0: [libc::c_char; 4] = [0; 4];
    _tmp_0[0 as libc::c_int as usize] = *p.offset(0 as libc::c_int as isize);
    _tmp_0[1 as libc::c_int as usize] = *p.offset(1 as libc::c_int as isize);
    _tmp_0[2 as libc::c_int as usize] = *p.offset(2 as libc::c_int as isize);
    _tmp_0[3 as libc::c_int as usize] = *p.offset(3 as libc::c_int as isize);
    *p.offset(0 as libc::c_int as isize) = _tmp_0[3 as libc::c_int as usize];
    *p.offset(1 as libc::c_int as isize) = _tmp_0[2 as libc::c_int as usize];
    *p.offset(2 as libc::c_int as isize) = _tmp_0[1 as libc::c_int as usize];
    *p.offset(3 as libc::c_int as isize) = _tmp_0[0 as libc::c_int as usize];
    /* version */
    p =
        p.offset(::std::mem::size_of::<u_int32_t>() as libc::c_ulong as
                     isize);
    let mut _tmp_1: [libc::c_char; 4] = [0; 4];
    _tmp_1[0 as libc::c_int as usize] = *p.offset(0 as libc::c_int as isize);
    _tmp_1[1 as libc::c_int as usize] = *p.offset(1 as libc::c_int as isize);
    _tmp_1[2 as libc::c_int as usize] = *p.offset(2 as libc::c_int as isize);
    _tmp_1[3 as libc::c_int as usize] = *p.offset(3 as libc::c_int as isize);
    *p.offset(0 as libc::c_int as isize) = _tmp_1[3 as libc::c_int as usize];
    *p.offset(1 as libc::c_int as isize) = _tmp_1[2 as libc::c_int as usize];
    *p.offset(2 as libc::c_int as isize) = _tmp_1[1 as libc::c_int as usize];
    *p.offset(3 as libc::c_int as isize) = _tmp_1[0 as libc::c_int as usize];
    /* psize */
    p =
        p.offset(::std::mem::size_of::<u_int32_t>() as libc::c_ulong as
                     isize);
    let mut _tmp_2: [libc::c_char; 4] = [0; 4];
    _tmp_2[0 as libc::c_int as usize] = *p.offset(0 as libc::c_int as isize);
    _tmp_2[1 as libc::c_int as usize] = *p.offset(1 as libc::c_int as isize);
    _tmp_2[2 as libc::c_int as usize] = *p.offset(2 as libc::c_int as isize);
    _tmp_2[3 as libc::c_int as usize] = *p.offset(3 as libc::c_int as isize);
    *p.offset(0 as libc::c_int as isize) = _tmp_2[3 as libc::c_int as usize];
    *p.offset(1 as libc::c_int as isize) = _tmp_2[2 as libc::c_int as usize];
    *p.offset(2 as libc::c_int as isize) = _tmp_2[1 as libc::c_int as usize];
    *p.offset(3 as libc::c_int as isize) = _tmp_2[0 as libc::c_int as usize];
    /* free */
    p =
        p.offset(::std::mem::size_of::<u_int32_t>() as libc::c_ulong as
                     isize);
    let mut _tmp_3: [libc::c_char; 4] = [0; 4];
    _tmp_3[0 as libc::c_int as usize] = *p.offset(0 as libc::c_int as isize);
    _tmp_3[1 as libc::c_int as usize] = *p.offset(1 as libc::c_int as isize);
    _tmp_3[2 as libc::c_int as usize] = *p.offset(2 as libc::c_int as isize);
    _tmp_3[3 as libc::c_int as usize] = *p.offset(3 as libc::c_int as isize);
    *p.offset(0 as libc::c_int as isize) = _tmp_3[3 as libc::c_int as usize];
    *p.offset(1 as libc::c_int as isize) = _tmp_3[2 as libc::c_int as usize];
    *p.offset(2 as libc::c_int as isize) = _tmp_3[1 as libc::c_int as usize];
    *p.offset(3 as libc::c_int as isize) = _tmp_3[0 as libc::c_int as usize];
    /* nrecs */
    p =
        p.offset(::std::mem::size_of::<u_int32_t>() as libc::c_ulong as
                     isize);
    let mut _tmp_4: [libc::c_char; 4] = [0; 4];
    _tmp_4[0 as libc::c_int as usize] = *p.offset(0 as libc::c_int as isize);
    _tmp_4[1 as libc::c_int as usize] = *p.offset(1 as libc::c_int as isize);
    _tmp_4[2 as libc::c_int as usize] = *p.offset(2 as libc::c_int as isize);
    _tmp_4[3 as libc::c_int as usize] = *p.offset(3 as libc::c_int as isize);
    *p.offset(0 as libc::c_int as isize) = _tmp_4[3 as libc::c_int as usize];
    *p.offset(1 as libc::c_int as isize) = _tmp_4[2 as libc::c_int as usize];
    *p.offset(2 as libc::c_int as isize) = _tmp_4[1 as libc::c_int as usize];
    *p.offset(3 as libc::c_int as isize) = _tmp_4[0 as libc::c_int as usize];
    /* flags */
    p =
        p.offset(::std::mem::size_of::<u_int32_t>() as libc::c_ulong as
                     isize);
}
