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
    #[c2rust::src_loc = "145:1"]
    pub type __dev_t = libc::c_ulong;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "147:1"]
    pub type __gid_t = libc::c_uint;
    #[c2rust::src_loc = "148:1"]
    pub type __ino_t = libc::c_ulong;
    #[c2rust::src_loc = "150:1"]
    pub type __mode_t = libc::c_uint;
    #[c2rust::src_loc = "151:1"]
    pub type __nlink_t = libc::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "174:1"]
    pub type __blksize_t = libc::c_long;
    #[c2rust::src_loc = "179:1"]
    pub type __blkcnt_t = libc::c_long;
    #[c2rust::src_loc = "196:1"]
    pub type __syscall_slong_t = libc::c_long;
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
    #[c2rust::src_loc = "85:1"]
    pub type off_t = __off_t;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    #[c2rust::src_loc = "158:1"]
    pub type u_int8_t = __uint8_t;
    #[c2rust::src_loc = "159:1"]
    pub type u_int16_t = __uint16_t;
    #[c2rust::src_loc = "160:1"]
    pub type u_int32_t = __uint32_t;
    use super::types_h::{__u_char, __u_int, __u_long, __off_t, __caddr_t,
                         __uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:41"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:41"]
pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "9:8"]
    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }
    use super::types_h::{__time_t, __syscall_slong_t};
}
#[c2rust::header_src = "/usr/include/bits/stat.h:45"]
pub mod stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
    pub struct stat {
        pub st_dev: __dev_t,
        pub st_ino: __ino_t,
        pub st_nlink: __nlink_t,
        pub st_mode: __mode_t,
        pub st_uid: __uid_t,
        pub st_gid: __gid_t,
        pub __pad0: libc::c_int,
        pub st_rdev: __dev_t,
        pub st_size: __off_t,
        pub st_blksize: __blksize_t,
        pub st_blocks: __blkcnt_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        pub __glibc_reserved: [__syscall_slong_t; 3],
    }
    use super::types_h::{__dev_t, __ino_t, __nlink_t, __mode_t, __uid_t,
                         __gid_t, __off_t, __blksize_t, __blkcnt_t,
                         __syscall_slong_t};
    use super::struct_timespec_h::timespec;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:51"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:51"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/db.h:54"]
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
    use super::sys_types_h::{u_int, u_long, u_char};
    /* !_DB_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/include/db-int.h:54"]
pub mod db_int_h {
    #[c2rust::src_loc = "143:1"]
    pub type db_pgno_t = u_int32_t;
    #[c2rust::src_loc = "145:1"]
    pub type indx_t = u_int16_t;
    #[c2rust::src_loc = "147:1"]
    pub type recno_t = u_int32_t;
    use super::sys_types_h::{u_int32_t, u_int16_t};
    use super::db_h::{BTREEINFO, DB};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "210:1"]
        pub fn __kdb2_bt_open(_: *const libc::c_char, _: libc::c_int,
                              _: libc::c_int, _: *const BTREEINFO,
                              _: libc::c_int) -> *mut DB;
    }
    /* _DB_INT_H_ */
    /* Needed for Win32 compiles */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/btree/btree.h:55"]
pub mod btree_h {
    #[c2rust::src_loc = "67:1"]
    pub type PAGE = _page;
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
    /* The in-memory btree/recno data structure. */
    #[c2rust::src_loc = "304:1"]
    pub type BTREE = _btree;
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
        pub bt_order: C2RustUnnamed,
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
    #[c2rust::src_loc = "241:1"]
    pub type EPGNO = _epgno;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "241:16"]
    pub struct _epgno {
        pub pgno: db_pgno_t,
        pub index: indx_t,
    }
    #[c2rust::src_loc = "334:2"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "334:20"]
    pub const FORWARD: C2RustUnnamed = 2;
    #[c2rust::src_loc = "334:14"]
    pub const BACK: C2RustUnnamed = 1;
    #[c2rust::src_loc = "334:9"]
    pub const NOT: C2RustUnnamed = 0;
    /* memory pool cookie */
    /* pointer to enclosing DB */
    /* current (pinned) page */
    /* page pinned across calls */
    /* cursor */
    /* stack of parent pages */
    /* current stack pointer */
    /* returned key */
    /* returned data */
    /* tree file descriptor */
    /* next free page */
    /* page size */
    /* cut-off for key/data overflow */
    /* byte order */
    /* sorted order */
    /* last insert */
    /* B: key comparison function */
    /* B: prefix comparison function */
    /* R: recno input function */
    /* R: record FILE pointer */
    /* R: record file descriptor */
    /* R: current point in mapped space */
    /* R: start of mapped space */
    /* R: end of mapped space */
    /* R: size of mapped region. */
    /* R: number of records */
    /* R: fixed record length */
    /* R: delimiting byte/pad character */
    /*
 * NB:
 * B_NODUPS and R_RECNO are stored on disk, and may not be changed.
 */
    /* in-memory tree */
    /* need to write metadata */
    /* tree modified */
    /* if byte order requires swapping */
    /* read-only tree */
    /* no duplicate keys permitted */
    /* record oriented tree */
    /* opened a file pointer */
    /* end of input file reached. */
    /* fixed length records */
    /* memory mapped file. */
    /* in-memory file */
    /* modified file */
    /* read-only file */
    /* DB_LOCK specified. */
    /* DB_SHMEM specified. */
    /* DB_TXN specified. */
    /*-
 * Copyright (c) 1991, 1993, 1994
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
 *
 *	@(#)btree.h	8.11 (Berkeley) 8/17/94
 */
    /* Macros to set/clear/test flags. */
    /* Minimum keys per page */
    /* Minimum cached pages */
    /* Minimum page size */
    /*
 * Page 0 of a btree file contains a copy of the meta-data.  This page is also
 * used as an out-of-band page, i.e. page pointers that point to nowhere point
 * to page 0.  Page 1 is the root of the btree.
 */
    /* Invalid tree page number. */
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
    /* First and next index. */
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
    /* key size */
    /* page number stored on */
    /* overflow data */
    /* overflow key */
    /* data */
    /* Get the page's BINTERNAL structure at index indx. */
    /* Get the number of bytes in the entry. */
    /* Copy a BINTERNAL entry to the page. */
    /*
 * For the recno internal pages, the item is a page number with the number of
 * keys found on that page and below.
 */
    /* number of records */
    /* page number stored below */
    /* Get the page's RINTERNAL structure at index indx. */
    /* Get the number of bytes in the entry. */
    /* Copy a RINTERAL entry to the page. */
    /* For the btree leaf pages, the item is a key and data pair. */
    /* size of key */
    /* size of data */
    /* P_BIGDATA, P_BIGKEY */
    /* data */
    /* Get the page's BLEAF structure at index indx. */
    /* Get the number of bytes in the entry. */
    /* Get the number of bytes in the user's key/data pair. */
    /* Copy a BLEAF entry to the page. */
    /* For the recno leaf pages, the item is a data entry. */
    /* size of data */
    /* P_BIGDATA */
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
    /* the page number */
    /* the index on the page */
    /* the (pinned) page */
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
    #[c2rust::src_loc = "275:1"]
    pub type CURSOR = _cursor;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "275:16"]
    pub struct _cursor {
        pub pg: EPGNO,
        pub key: DBT,
        pub rcursor: recno_t,
        pub flags: u_int8_t,
    }
    #[c2rust::src_loc = "246:1"]
    pub type EPG = _epg;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "246:16"]
    pub struct _epg {
        pub page: *mut PAGE,
        pub index: indx_t,
    }
    use super::db_int_h::{db_pgno_t, indx_t, recno_t};
    use super::sys_types_h::{u_int32_t, caddr_t, u_char, u_int8_t};
    use super::mpool_h::MPOOL;
    use super::db_h::{DB, DBT};
    use super::stddef_h::size_t;
    use super::FILE_h::FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/mpool/mpool.h:55"]
pub mod mpool_h {
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
    #[c2rust::src_loc = "49:16"]
    pub struct _bkt {
        pub hq: C2RustUnnamed_1,
        pub q: C2RustUnnamed_0,
        pub page: *mut libc::c_void,
        pub pgno: db_pgno_t,
        pub flags: u_int8_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "51:2"]
    pub struct C2RustUnnamed_0 {
        pub tqe_next: *mut _bkt,
        pub tqe_prev: *mut *mut _bkt,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:2"]
    pub struct C2RustUnnamed_1 {
        pub tqe_next: *mut _bkt,
        pub tqe_prev: *mut *mut _bkt,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:13"]
    pub struct _lqh {
        pub tqh_first: *mut _bkt,
        pub tqh_last: *mut *mut _bkt,
    }
    use super::db_int_h::db_pgno_t;
    use super::sys_types_h::{u_long, u_int8_t, u_int};
    extern "C" {
        /* B: Saved tree reference. */
        /* B: Saved key, or key.data == NULL. */
        /* R: recno cursor (1-based) */
        /*  B: Cursor needs to be reacquired. */
        /*  B: Unreturned cursor after key. */
        /*  B: Unreturned cursor before key. */
        /* RB: Cursor initialized. */
        /* Ignore if the page is pinned. */
        /* Allocate a new page with a
					   specific page number. */
        /* Allocate a new page with the next
					  page number. */
        #[no_mangle]
        #[c2rust::src_loc = "111:1"]
        pub fn kdb2_mpool_put(_: *mut MPOOL, _: *mut libc::c_void, _: u_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "109:1"]
        pub fn kdb2_mpool_get(_: *mut MPOOL, _: db_pgno_t, _: u_int)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/sys/stat.h:45"]
pub mod sys_stat_h {
    use super::stat_h::stat;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "210:1"]
        pub fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:47"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:48"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "175:1"]
        pub fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "195:1"]
        pub fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:51"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "279:1"]
        pub fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char)
         -> *mut FILE;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:52"]
pub mod unistd_h {
    use super::types_h::__off_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "334:1"]
        pub fn lseek(__fd: libc::c_int, __offset: __off_t,
                     __whence: libc::c_int) -> __off_t;
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/recno/extern.h:55"]
pub mod extern_h {
    use super::db_h::{DB, DBT};
    use super::sys_types_h::u_int;
    use super::btree_h::BTREE;
    use super::db_int_h::recno_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "56:1"]
        pub fn __kdb2_rec_close(_: *mut DB) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "57:1"]
        pub fn __kdb2_rec_delete(_: *const DB, _: *const DBT, _: u_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "62:1"]
        pub fn __kdb2_rec_fpipe(_: *mut BTREE, _: recno_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "63:1"]
        pub fn __kdb2_rec_get(_: *const DB, _: *const DBT, _: *mut DBT,
                              _: u_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "65:1"]
        pub fn __kdb2_rec_put(dbp: *const DB, _: *mut DBT, _: *const DBT,
                              _: u_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "68:1"]
        pub fn __kdb2_rec_seq(_: *const DB, _: *mut DBT, _: *mut DBT,
                              _: u_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "69:1"]
        pub fn __kdb2_rec_sync(_: *const DB, _: u_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "72:1"]
        pub fn __kdb2_rec_vpipe(_: *mut BTREE, _: recno_t) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/btree/extern.h:55"]
pub mod btree_extern_h {
    use super::db_h::DB;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn __kdb2_bt_close(_: *mut DB) -> libc::c_int;
    }
}
pub use self::types_h::{__u_char, __u_int, __u_long, __uint8_t, __uint16_t,
                        __uint32_t, __dev_t, __uid_t, __gid_t, __ino_t,
                        __mode_t, __nlink_t, __off_t, __off64_t, __time_t,
                        __blksize_t, __blkcnt_t, __syscall_slong_t,
                        __caddr_t};
pub use self::sys_types_h::{u_char, u_int, u_long, off_t, caddr_t, u_int8_t,
                            u_int16_t, u_int32_t};
pub use self::stddef_h::size_t;
pub use self::struct_timespec_h::timespec;
pub use self::stat_h::stat;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::db_h::{DBT, DBTYPE, DB_RECNO, DB_HASH, DB_BTREE, __db, DB,
                     BTREEINFO, RECNOINFO};
pub use self::db_int_h::{db_pgno_t, indx_t, recno_t, __kdb2_bt_open};
pub use self::btree_h::{PAGE, _page, BTREE, _btree, EPGNO, _epgno,
                        C2RustUnnamed, FORWARD, BACK, NOT, CURSOR, _cursor,
                        EPG, _epg};
pub use self::mpool_h::{MPOOL, _hqh, _bkt, C2RustUnnamed_0, C2RustUnnamed_1,
                        _lqh, kdb2_mpool_put, kdb2_mpool_get};
use self::sys_stat_h::fstat;
use self::errno_h::__errno_location;
use self::fcntl_h::{fcntl, open};
use self::stdio_h::fdopen;
use self::unistd_h::{lseek, close};
use self::extern_h::{__kdb2_rec_close, __kdb2_rec_delete, __kdb2_rec_fpipe,
                     __kdb2_rec_get, __kdb2_rec_put, __kdb2_rec_seq,
                     __kdb2_rec_sync, __kdb2_rec_vpipe};
use self::btree_extern_h::__kdb2_bt_close;
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
/* >= # of bytes in a page */
/* >= # of records in a tree */
/*
 * Little endian <==> big endian 32-bit swap macros.
 *	M_32_SWAP	swap a memory location
 *	P_32_SWAP	swap a referenced memory location
 *	P_32_COPY	swap from one location to another
 */
/*
 * Little endian <==> big endian 16-bit swap macros.
 *	M_16_SWAP	swap a memory location
 *	P_16_SWAP	swap a referenced memory location
 *	P_16_COPY	swap from one location to another
 */
/* open functions for each database type, used in dbopen() */
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
#[no_mangle]
#[c2rust::src_loc = "57:1"]
pub unsafe extern "C" fn __kdb2_rec_open(mut fname: *const libc::c_char,
                                         mut flags: libc::c_int,
                                         mut mode: libc::c_int,
                                         mut openinfo: *const RECNOINFO,
                                         mut dflags: libc::c_int) -> *mut DB {
    let mut current_block: u64;
    let mut t: *mut BTREE = 0 as *mut BTREE;
    let mut btopeninfo: BTREEINFO =
        BTREEINFO{flags: 0,
                  cachesize: 0,
                  maxkeypage: 0,
                  minkeypage: 0,
                  psize: 0,
                  compare: None,
                  prefix: None,
                  lorder: 0,};
    let mut dbp: *mut DB = 0 as *mut DB;
    let mut h: *mut PAGE = 0 as *mut PAGE;
    let mut sb: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    let mut rfd: libc::c_int = -(1 as libc::c_int);
    let mut sverrno: libc::c_int = 0;
    /* Open the user's file -- if this fails, we're done. */
    if !fname.is_null() &&
           {
               rfd = open(fname, flags | 0 as libc::c_int, mode);
               (rfd) < 0 as libc::c_int
           } {
        return 0 as *mut DB
    }
    if !fname.is_null() &&
           fcntl(rfd, 2 as libc::c_int, 1 as libc::c_int) ==
               -(1 as libc::c_int) {
        close(rfd);
        return 0 as *mut DB
    }
    /* Create a btree in memory (backed by disk). */
    dbp = 0 as *mut DB;
    if !openinfo.is_null() {
        if (*openinfo).flags &
               !(0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int)
                   as libc::c_ulong != 0 {
            current_block = 13794778498410580684;
        } else {
            btopeninfo.flags = 0 as libc::c_int as u_long;
            btopeninfo.cachesize = (*openinfo).cachesize;
            btopeninfo.maxkeypage = 0 as libc::c_int;
            btopeninfo.minkeypage = 0 as libc::c_int;
            btopeninfo.psize = (*openinfo).psize;
            btopeninfo.compare = None;
            btopeninfo.prefix = None;
            btopeninfo.lorder = (*openinfo).lorder;
            dbp =
                __kdb2_bt_open((*openinfo).bfname,
                               0o2 as libc::c_int | 0 as libc::c_int,
                               0o400 as libc::c_int | 0o200 as libc::c_int,
                               &mut btopeninfo, dflags);
            current_block = 2838571290723028321;
        }
    } else {
        dbp =
            __kdb2_bt_open(0 as *const libc::c_char,
                           0o2 as libc::c_int | 0 as libc::c_int,
                           0o400 as libc::c_int | 0o200 as libc::c_int,
                           0 as *const BTREEINFO, dflags);
        current_block = 2838571290723028321;
    }
    match current_block {
        2838571290723028321 => {
            if dbp.is_null() {
                current_block = 18136059781278469891;
            } else {
                /*
	 * Some fields in the tree structure are recno specific.  Fill them
	 * in and make the btree structure look like a recno structure.  We
	 * don't change the bt_ovflsize value, it's close enough and slightly
	 * bigger.
	 */
                t = (*dbp).internal as *mut BTREE;
                if !openinfo.is_null() {
                    if (*openinfo).flags & 0x1 as libc::c_int as libc::c_ulong
                           != 0 {
                        (*t).flags |= 0x200 as libc::c_int as libc::c_uint;
                        (*t).bt_reclen = (*openinfo).reclen;
                        if (*t).bt_reclen == 0 as libc::c_int as libc::c_ulong
                           {
                            current_block = 13794778498410580684;
                        } else { current_block = 14818589718467733107; }
                    } else { current_block = 14818589718467733107; }
                    match current_block {
                        13794778498410580684 => { }
                        _ => {
                            (*t).bt_bval = (*openinfo).bval;
                            current_block = 4761528863920922185;
                        }
                    }
                } else {
                    (*t).bt_bval = '\n' as i32 as u_char;
                    current_block = 4761528863920922185;
                }
                match current_block {
                    13794778498410580684 => { }
                    _ => {
                        (*t).flags |= 0x80 as libc::c_int as libc::c_uint;
                        if fname.is_null() {
                            (*t).flags |=
                                (0x100 as libc::c_int | 0x800 as libc::c_int)
                                    as libc::c_uint
                        } else { (*t).bt_rfd = rfd }
                        if !fname.is_null() {
                            /*
		 * In 4.4BSD, stat(2) returns true for ISSOCK on pipes.
		 * Unfortunately, that's not portable, so we use lseek
		 * and check the errno values.
		 */
                            *__errno_location() = 0 as libc::c_int;
                            if lseek(rfd, 0 as libc::c_int as off_t,
                                     1 as libc::c_int) ==
                                   -(1 as libc::c_int) as libc::c_long &&
                                   *__errno_location() == 29 as libc::c_int {
                                match flags & 0o3 as libc::c_int {
                                    0 => {
                                        (*t).flags |=
                                            0x2000 as libc::c_int as
                                                libc::c_uint;
                                        current_block = 12781735722417726073;
                                    }
                                    _ => {
                                        current_block = 13794778498410580684;
                                    }
                                }
                            } else {
                                match flags & 0o3 as libc::c_int {
                                    0 => {
                                        current_block = 14284512915221609433;
                                        match current_block {
                                            14284512915221609433 => {
                                                (*t).flags |=
                                                    0x2000 as libc::c_int as
                                                        libc::c_uint
                                            }
                                            _ => { }
                                        }
                                        if fstat(rfd, &mut sb) != 0 {
                                            current_block =
                                                18136059781278469891;
                                        } else if sb.st_size ==
                                                      0 as libc::c_int as
                                                          libc::c_long {
                                            (*t).flags |=
                                                0x100 as libc::c_int as
                                                    libc::c_uint;
                                            current_block =
                                                2516253395664191498;
                                        } else {
                                            current_block =
                                                12781735722417726073;
                                        }
                                    }
                                    2 => {
                                        current_block = 6174974146017752131;
                                        match current_block {
                                            14284512915221609433 => {
                                                (*t).flags |=
                                                    0x2000 as libc::c_int as
                                                        libc::c_uint
                                            }
                                            _ => { }
                                        }
                                        if fstat(rfd, &mut sb) != 0 {
                                            current_block =
                                                18136059781278469891;
                                        } else if sb.st_size ==
                                                      0 as libc::c_int as
                                                          libc::c_long {
                                            (*t).flags |=
                                                0x100 as libc::c_int as
                                                    libc::c_uint;
                                            current_block =
                                                2516253395664191498;
                                        } else {
                                            current_block =
                                                12781735722417726073;
                                        }
                                    }
                                    _ => {
                                        current_block = 13794778498410580684;
                                    }
                                }
                            }
                            match current_block {
                                2516253395664191498 => { }
                                13794778498410580684 => { }
                                18136059781278469891 => { }
                                _ => {
                                    (*t).bt_rfp =
                                        fdopen(rfd,
                                               b"rb\x00" as *const u8 as
                                                   *const libc::c_char);
                                    if (*t).bt_rfp.is_null() {
                                        current_block = 18136059781278469891;
                                    } else {
                                        (*t).flags |=
                                            0x40 as libc::c_int as
                                                libc::c_uint;
                                        (*t).bt_irec =
                                            if (*t).flags &
                                                   0x200 as libc::c_int as
                                                       libc::c_uint != 0 {
                                                Some(__kdb2_rec_fpipe as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut BTREE,
                                                                              _:
                                                                                  recno_t)
                                                             -> libc::c_int)
                                            } else {
                                                Some(__kdb2_rec_vpipe as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut BTREE,
                                                                              _:
                                                                                  recno_t)
                                                             -> libc::c_int)
                                            };
                                        current_block = 2516253395664191498;
                                    }
                                }
                            }
                        } else { current_block = 2516253395664191498; }
                        match current_block {
                            13794778498410580684 => { }
                            18136059781278469891 => { }
                            _ => {
                                /*
			 * Kluge -- we'd like to test to see if the file is too
			 * big to mmap.  Since, we don't know what size or type
			 * off_t's or size_t's are, what the largest unsigned
			 * integral type is, or what random insanity the local
			 * C compiler will perpetrate, doing the comparison in
			 * a portable way is flatly impossible.  Hope that mmap
			 * fails if the file is too large.
			 */
                                /* Use the recno routines. */
                                (*dbp).close =
                                    Some(__kdb2_rec_close as
                                             unsafe extern "C" fn(_: *mut DB)
                                                 -> libc::c_int);
                                (*dbp).del =
                                    Some(__kdb2_rec_delete as
                                             unsafe extern "C" fn(_:
                                                                      *const DB,
                                                                  _:
                                                                      *const DBT,
                                                                  _: u_int)
                                                 -> libc::c_int);
                                (*dbp).fd =
                                    Some(__kdb2_rec_fd as
                                             unsafe extern "C" fn(_:
                                                                      *const DB)
                                                 -> libc::c_int);
                                (*dbp).get =
                                    Some(__kdb2_rec_get as
                                             unsafe extern "C" fn(_:
                                                                      *const DB,
                                                                  _:
                                                                      *const DBT,
                                                                  _: *mut DBT,
                                                                  _: u_int)
                                                 -> libc::c_int);
                                (*dbp).put =
                                    Some(__kdb2_rec_put as
                                             unsafe extern "C" fn(_:
                                                                      *const DB,
                                                                  _: *mut DBT,
                                                                  _:
                                                                      *const DBT,
                                                                  _: u_int)
                                                 -> libc::c_int);
                                (*dbp).seq =
                                    Some(__kdb2_rec_seq as
                                             unsafe extern "C" fn(_:
                                                                      *const DB,
                                                                  _: *mut DBT,
                                                                  _: *mut DBT,
                                                                  _: u_int)
                                                 -> libc::c_int);
                                (*dbp).sync =
                                    Some(__kdb2_rec_sync as
                                             unsafe extern "C" fn(_:
                                                                      *const DB,
                                                                  _: u_int)
                                                 -> libc::c_int);
                                /* If the root page was created, reset the flags. */
                                h =
                                    kdb2_mpool_get((*t).bt_mp,
                                                   1 as libc::c_int as
                                                       db_pgno_t,
                                                   0 as libc::c_int as u_int)
                                        as *mut PAGE;
                                if h.is_null() {
                                    current_block = 18136059781278469891;
                                } else {
                                    if (*h).flags &
                                           0x1f as libc::c_int as libc::c_uint
                                           ==
                                           0x2 as libc::c_int as libc::c_uint
                                       {
                                        (*h).flags &=
                                            !(0x1f as libc::c_int) as
                                                libc::c_uint;
                                        (*h).flags |=
                                            0x10 as libc::c_int as
                                                libc::c_uint;
                                        kdb2_mpool_put((*t).bt_mp,
                                                       h as *mut libc::c_void,
                                                       0x1 as libc::c_int as
                                                           u_int);
                                    } else {
                                        kdb2_mpool_put((*t).bt_mp,
                                                       h as *mut libc::c_void,
                                                       0 as libc::c_int as
                                                           u_int);
                                    }
                                    if !openinfo.is_null() &&
                                           (*openinfo).flags &
                                               0x4 as libc::c_int as
                                                   libc::c_ulong != 0 &&
                                           (*t).flags &
                                               (0x100 as libc::c_int |
                                                    0x800 as libc::c_int) as
                                                   libc::c_uint == 0 &&
                                           (*t).bt_irec.expect("non-null function pointer")(t,
                                                                                            0xffffffff
                                                                                                as
                                                                                                libc::c_uint)
                                               == -(1 as libc::c_int) {
                                        current_block = 18136059781278469891;
                                    } else { return dbp }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    match current_block {
        13794778498410580684 => { *__errno_location() = 22 as libc::c_int }
        _ => { }
    }
    sverrno = *__errno_location();
    if !dbp.is_null() { __kdb2_bt_close(dbp); }
    if !fname.is_null() { close(rfd); }
    *__errno_location() = sverrno;
    return 0 as *mut DB;
}
#[no_mangle]
#[c2rust::src_loc = "228:1"]
pub unsafe extern "C" fn __kdb2_rec_fd(mut dbp: *const DB) -> libc::c_int {
    let mut t: *mut BTREE = 0 as *mut BTREE;
    t = (*dbp).internal as *mut BTREE;
    /* Toss any page pinned across calls. */
    if !(*t).bt_pinned.is_null() {
        kdb2_mpool_put((*t).bt_mp, (*t).bt_pinned as *mut libc::c_void,
                       0 as libc::c_int as u_int);
        (*t).bt_pinned = 0 as *mut PAGE
    }
    /* In-memory database can't have a file descriptor. */
    if (*t).flags & 0x800 as libc::c_int as libc::c_uint != 0 {
        *__errno_location() = 2 as libc::c_int;
        return -(1 as libc::c_int)
    }
    return (*t).bt_rfd;
}
