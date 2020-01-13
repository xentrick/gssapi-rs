use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:65"]
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
#[c2rust::header_src = "/usr/include/sys/types.h:65"]
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
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:65"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:69"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:69"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/db.h:73"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/include/db-int.h:73"]
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
    /* >= # of records in a tree */
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
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/mpool/mpool.h:74"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/btree/btree.h:74"]
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
    #[c2rust::src_loc = "120:1"]
    pub type BINTERNAL = _binternal;
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
    /* B: Saved tree reference. */
    /* B: Saved key, or key.data == NULL. */
    /* R: recno cursor (1-based) */
    /*  B: Cursor needs to be reacquired. */
    /*  B: Unreturned cursor after key. */
    /*  B: Unreturned cursor before key. */
    /* RB: Cursor initialized. */
    /*
 * The metadata of the tree.  The nrecs field is used only by the RECNO code.
 * This is because the btree doesn't really need it and it requires that every
 * put or delete call modify the metadata.
 */
    /* magic number */
    /* version */
    /* page size */
    /* page number of first free page */
    /* R: number of records */
    /* bt_flags & SAVEMETA */
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
#[c2rust::header_src = "/usr/include/errno.h:67"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:70"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/btree/extern.h:74"]
pub mod extern_h {
    use super::btree_h::{BTREE, EPG};
    use super::db_h::DBT;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "68:1"]
        pub fn __kdb2_bt_cmp(_: *mut BTREE, _: *const DBT, _: *mut EPG)
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
pub use self::btree_h::{_page, PAGE, _binternal, BINTERNAL, _epgno, EPGNO,
                        _epg, EPG, _cursor, CURSOR, _btree, C2RustUnnamed_1,
                        FORWARD, BACK, NOT, BTREE};
use self::errno_h::__errno_location;
use self::stdlib_h::free;
use self::extern_h::{__kdb2_bt_cmp, __kdb2_bt_ret, __kdb2_bt_search};
/*
 * Sequential scan support.
 *
 * The tree can be scanned sequentially, starting from either end of the
 * tree or from any specific key.  A scan request before any scanning is
 * done is initialized as starting from the least node.
 */
/*
 * __bt_seq --
 *	Btree sequential scan interface.
 *
 * Parameters:
 *	dbp:	pointer to access method
 *	key:	key for positioning and return value
 *	data:	data return value
 *	flags:	R_CURSOR, R_FIRST, R_LAST, R_NEXT, R_PREV.
 *
 * Returns:
 *	RET_ERROR, RET_SUCCESS or RET_SPECIAL if there's no next key.
 */
#[no_mangle]
#[c2rust::src_loc = "104:1"]
pub unsafe extern "C" fn __kdb2_bt_seq(mut dbp: *const DB, mut key: *mut DBT,
                                       mut data: *mut DBT, mut flags: u_int)
 -> libc::c_int {
    let mut t: *mut BTREE = 0 as *mut BTREE;
    let mut e: EPG = EPG{page: 0 as *mut PAGE, index: 0,};
    let mut status: libc::c_int = 0;
    t = (*dbp).internal as *mut BTREE;
    /* Toss any page pinned across calls. */
    if !(*t).bt_pinned.is_null() {
        kdb2_mpool_put((*t).bt_mp, (*t).bt_pinned as *mut libc::c_void,
                       0 as libc::c_int as u_int);
        (*t).bt_pinned = 0 as *mut PAGE
    }
    let mut current_block_8: u64;
    /*
	 * If scan unitialized as yet, or starting at a specific record, set
	 * the scan to a specific key.  Both __bt_seqset and __bt_seqadv pin
	 * the page the cursor references if they're successful.
	 */
    match flags {
        7 | 9 | 128 | 129 => {
            if (*t).bt_cursor.flags as libc::c_int & 0x8 as libc::c_int != 0 {
                status = __bt_seqadv(t, &mut e, flags as libc::c_int);
                current_block_8 = 17407779659766490442;
            } else { current_block_8 = 6784655606881109955; }
        }
        3 | 6 | 1 => { current_block_8 = 6784655606881109955; }
        _ => {
            *__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int)
        }
    }
    match current_block_8 {
        6784655606881109955 =>
        /* FALLTHROUGH */
        {
            status = __bt_seqset(t, &mut e, key, flags as libc::c_int)
        }
        _ => { }
    }
    if status == 0 as libc::c_int {
        __kdb2_bt_setcur(t, (*e.page).pgno, e.index as u_int);
        status =
            __kdb2_bt_ret(t, &mut e, key, &mut (*t).bt_rkey, data,
                          &mut (*t).bt_rdata, 0 as libc::c_int);
        /*
		 * If the user is doing concurrent access, we copied the
		 * key/data, toss the page.
		 */
        if (*t).flags & 0x4000 as libc::c_int as libc::c_uint != 0 {
            kdb2_mpool_put((*t).bt_mp, e.page as *mut libc::c_void,
                           0 as libc::c_int as u_int);
        } else { (*t).bt_pinned = e.page }
    }
    return status;
}
/*
 * __bt_seqset --
 *	Set the sequential scan to a specific key.
 *
 * Parameters:
 *	t:	tree
 *	ep:	storage for returned key
 *	key:	key for initial scan position
 *	flags:	R_CURSOR, R_FIRST, R_LAST, R_NEXT, R_PREV
 *
 * Side effects:
 *	Pins the page the cursor references.
 *
 * Returns:
 *	RET_ERROR, RET_SUCCESS or RET_SPECIAL if there's no next key.
 */
#[c2rust::src_loc = "181:1"]
unsafe extern "C" fn __bt_seqset(mut t: *mut BTREE, mut ep: *mut EPG,
                                 mut key: *mut DBT, mut flags: libc::c_int)
 -> libc::c_int {
    let mut h: *mut PAGE = 0 as *mut PAGE;
    let mut pg: db_pgno_t = 0;
    let mut exact: libc::c_int = 0;
    /*
	 * Find the first, last or specific key in the tree and point the
	 * cursor at it.  The cursor may not be moved until a new key has
	 * been found.
	 */
    match flags {
        1 => {
            /* Keyed scan. */
            /*
		 * Find the first instance of the key or the smallest key
		 * which is greater than or equal to the specified key.
		 */
            if (*key).data.is_null() ||
                   (*key).size == 0 as libc::c_int as libc::c_ulong {
                *__errno_location() = 22 as libc::c_int;
                return -(1 as libc::c_int)
            }
            return __bt_first(t, key, ep, &mut exact)
        }
        3 | 7 | 128 => {
            /* First record. */
            (*t).bt_sp = (*t).bt_stack.as_mut_ptr();
            /* Walk down the left-hand side of the tree. */
            pg = 1 as libc::c_int as db_pgno_t;
            loop  {
                h =
                    kdb2_mpool_get((*t).bt_mp, pg, 0 as libc::c_int as u_int)
                        as *mut PAGE;
                if h.is_null() { return -(1 as libc::c_int) }
                /* Check for an empty tree. */
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
                    kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                                   0 as libc::c_int as u_int);
                    return 1 as libc::c_int
                }
                if (*h).flags &
                       (0x2 as libc::c_int | 0x10 as libc::c_int) as
                           libc::c_uint != 0 {
                    break ;
                }
                pg =
                    (*((h as
                            *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(0
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         isize)
                                                          as libc::c_int as
                                                          isize) as
                           *mut libc::c_void as *mut BINTERNAL)).pgno;
                (*(*t).bt_sp).pgno = (*h).pgno;
                (*(*t).bt_sp).index = 0 as libc::c_int as indx_t;
                (*t).bt_sp = (*t).bt_sp.offset(1);
                kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                               0 as libc::c_int as u_int);
            }
            (*ep).page = h;
            (*ep).index = 0 as libc::c_int as indx_t
        }
        6 | 9 | 129 => {
            /* Last record. */
            (*t).bt_sp = (*t).bt_stack.as_mut_ptr();
            /* Walk down the right-hand side of the tree. */
            pg = 1 as libc::c_int as db_pgno_t;
            loop  {
                h =
                    kdb2_mpool_get((*t).bt_mp, pg, 0 as libc::c_int as u_int)
                        as *mut PAGE;
                if h.is_null() { return -(1 as libc::c_int) }
                /* Check for an empty tree. */
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
                    kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                                   0 as libc::c_int as u_int);
                    return 1 as libc::c_int
                }
                if (*h).flags &
                       (0x2 as libc::c_int | 0x10 as libc::c_int) as
                           libc::c_uint != 0 {
                    break ;
                }
                pg =
                    (*((h as
                            *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(((*h).lower
                                                                                          as
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
                                                                                         as
                                                                                         isize)
                                                          as libc::c_int as
                                                          isize) as
                           *mut libc::c_void as *mut BINTERNAL)).pgno;
                (*(*t).bt_sp).pgno = (*h).pgno;
                (*(*t).bt_sp).index =
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
                (*t).bt_sp = (*t).bt_sp.offset(1);
                kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                               0 as libc::c_int as u_int);
            }
            (*ep).page = h;
            (*ep).index =
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
                    as indx_t
        }
        _ => { }
    }
    return 0 as libc::c_int;
}
/*
 * __bt_seqadvance --
 *	Advance the sequential scan.
 *
 * Parameters:
 *	t:	tree
 *	flags:	R_NEXT, R_PREV
 *
 * Side effects:
 *	Pins the page the new key/data record is on.
 *
 * Returns:
 *	RET_ERROR, RET_SUCCESS or RET_SPECIAL if there's no next key.
 */
#[c2rust::src_loc = "275:1"]
unsafe extern "C" fn __bt_seqadv(mut t: *mut BTREE, mut ep: *mut EPG,
                                 mut flags: libc::c_int) -> libc::c_int {
    let mut c: *mut CURSOR = 0 as *mut CURSOR;
    let mut h: *mut PAGE = 0 as *mut PAGE;
    let mut idx: indx_t = 0 as libc::c_int as indx_t;
    let mut pg: db_pgno_t = 0;
    let mut exact: libc::c_int = 0;
    let mut rval: libc::c_int = 0;
    /*
	 * There are a couple of states that we can be in.  The cursor has
	 * been initialized by the time we get here, but that's all we know.
	 */
    c = &mut (*t).bt_cursor;
    /*
	 * The cursor was deleted and there weren't any duplicate records,
	 * so the cursor's key was saved.  Find out where that key would
	 * be in the current tree.  If the returned key is an exact match,
	 * it means that a key/data pair was inserted into the tree after
	 * the delete.  We could reasonably return the key, but the problem
	 * is that this is the access pattern we'll see if the user is
	 * doing seq(..., R_NEXT)/put(..., 0) pairs, i.e. the put deletes
	 * the cursor record and then replaces it, so the cursor was saved,
	 * and we'll simply return the same "new" record until the user
	 * notices and doesn't do a put() of it.  Since the key is an exact
	 * match, we could as easily put the new record before the cursor,
	 * and we've made no guarantee to return it.  So, move forward or
	 * back a record if it's an exact match.
	 *
	 * XXX
	 * In the current implementation, put's to the cursor are done with
	 * delete/add pairs.  This has two consequences.  First, it means
	 * that seq(..., R_NEXT)/put(..., R_CURSOR) pairs are going to exhibit
	 * the same behavior as above.  Second, you can return the same key
	 * twice if you have duplicate records.  The scenario is that the
	 * cursor record is deleted, moving the cursor forward or backward
	 * to a duplicate.  The add then inserts the new record at a location
	 * ahead of the cursor because duplicates aren't sorted in any way,
	 * and the new record is later returned.  This has to be fixed at some
	 * point.
	 */
    if (*c).flags as libc::c_int & 0x1 as libc::c_int != 0 {
        rval = __bt_first(t, &mut (*c).key, ep, &mut exact);
        if rval == -(1 as libc::c_int) { return -(1 as libc::c_int) }
        if exact == 0 { return rval }
        /*
		 * XXX
		 * Kluge -- get, release, get the page.
		 */
        (*c).pg.pgno = (*(*ep).page).pgno;
        (*c).pg.index = (*ep).index;
        kdb2_mpool_put((*t).bt_mp, (*ep).page as *mut libc::c_void,
                       0 as libc::c_int as u_int);
    }
    /* Get the page referenced by the cursor. */
    h =
        kdb2_mpool_get((*t).bt_mp, (*c).pg.pgno, 0 as libc::c_int as u_int) as
            *mut PAGE;
    if h.is_null() { return -(1 as libc::c_int) }
    let mut current_block_46: u64;
    /*
 	 * Find the next/previous record in the tree and point the cursor at
	 * it.  The cursor may not be moved until a new key has been found.
	 */
    match flags {
        7 => {
            /* Next record. */
            current_block_46 = 10648711420305664561;
        }
        128 => { current_block_46 = 10648711420305664561; }
        9 => {
            /* Previous record. */
            current_block_46 = 14618798359016307036;
        }
        129 => { current_block_46 = 14618798359016307036; }
        _ => { current_block_46 = 12497913735442871383; }
    }
    match current_block_46 {
        14618798359016307036 =>
        /*
		 * The cursor was deleted in duplicate records, and moved
		 * backward to a record that has yet to be returned.  Clear
		 * that flag, and return the record.
		 */
        {
            if (*c).flags as libc::c_int & 0x4 as libc::c_int != 0 {
                current_block_46 = 2268133538364636286;
            } else {
                idx = (*c).pg.index;
                if idx as libc::c_int == 0 as libc::c_int {
                    if flags == 129 as libc::c_int {
                        (*ep).page = h;
                        (*ep).index = idx;
                        return bt_rseq_prev(t, ep)
                    }
                    pg = (*h).prevpg;
                    kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                                   0 as libc::c_int as u_int);
                    if pg == 0 as libc::c_int as libc::c_uint {
                        return 1 as libc::c_int
                    }
                    h =
                        kdb2_mpool_get((*t).bt_mp, pg,
                                       0 as libc::c_int as u_int) as
                            *mut PAGE;
                    if h.is_null() { return -(1 as libc::c_int) }
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
                            as indx_t
                } else { idx = idx.wrapping_sub(1) }
                current_block_46 = 12497913735442871383;
            }
        }
        10648711420305664561 =>
        /*
		 * The cursor was deleted in duplicate records, and moved
		 * forward to a record that has yet to be returned.  Clear
		 * that flag, and return the record.
		 */
        {
            if (*c).flags as libc::c_int & 0x2 as libc::c_int != 0 {
                current_block_46 = 2268133538364636286;
            } else {
                idx = (*c).pg.index;
                idx = idx.wrapping_add(1);
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
                                                                                                                                                                                                                                                              libc::c_ulong)
                   {
                    if flags == 128 as libc::c_int {
                        (*ep).page = h;
                        (*ep).index = idx;
                        return bt_rseq_next(t, ep)
                    }
                    pg = (*h).nextpg;
                    kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                                   0 as libc::c_int as u_int);
                    if pg == 0 as libc::c_int as libc::c_uint {
                        return 1 as libc::c_int
                    }
                    h =
                        kdb2_mpool_get((*t).bt_mp, pg,
                                       0 as libc::c_int as u_int) as
                            *mut PAGE;
                    if h.is_null() { return -(1 as libc::c_int) }
                    idx = 0 as libc::c_int as indx_t
                }
                current_block_46 = 12497913735442871383;
            }
        }
        _ => { }
    }
    match current_block_46 {
        12497913735442871383 => { }
        _ => {
            (*c).flags =
                ((*c).flags as libc::c_int &
                     !(0x2 as libc::c_int | 0x4 as libc::c_int)) as u_int8_t;
            (*ep).page = h;
            (*ep).index = (*c).pg.index;
            return 0 as libc::c_int
        }
    }
    (*ep).page = h;
    (*ep).index = idx;
    return 0 as libc::c_int;
}
/*
 * Get the first item on the next page, but by going up and down the tree.
 */
#[c2rust::src_loc = "408:1"]
unsafe extern "C" fn bt_rseq_next(mut t: *mut BTREE, mut ep: *mut EPG)
 -> libc::c_int {
    let mut h: *mut PAGE = 0 as *mut PAGE;
    let mut idx: indx_t = 0;
    let mut up: *mut EPGNO = 0 as *mut EPGNO;
    let mut pg: db_pgno_t = 0;
    h = (*ep).page;
    idx = (*ep).index;
    loop  {
        /* Move up the tree. */
        up =
            if (*t).bt_sp == (*t).bt_stack.as_mut_ptr() {
                0 as *mut EPGNO
            } else { (*t).bt_sp = (*t).bt_sp.offset(-1); (*t).bt_sp };
        kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                       0 as libc::c_int as u_int);
        /* Did we hit the right edge of the root? */
        if up.is_null() { return 1 as libc::c_int }
        h =
            kdb2_mpool_get((*t).bt_mp, (*up).pgno, 0 as libc::c_int as u_int)
                as *mut PAGE;
        if h.is_null() { return -(1 as libc::c_int) }
        idx = (*up).index;
        idx = idx.wrapping_add(1);
        if !(idx as libc::c_ulong ==
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
                                                                                                                                                                                                                                                        libc::c_ulong))
           {
            break ;
        }
    }
    while (*h).flags &
              (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint == 0
          {
        /* Move back down the tree. */
        (*(*t).bt_sp).pgno = (*h).pgno;
        (*(*t).bt_sp).index = idx;
        (*t).bt_sp = (*t).bt_sp.offset(1);
        pg =
            (*((h as
                    *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(idx
                                                                                 as
                                                                                 isize)
                                                  as libc::c_int as isize) as
                   *mut libc::c_void as *mut BINTERNAL)).pgno;
        kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                       0 as libc::c_int as u_int);
        h =
            kdb2_mpool_get((*t).bt_mp, pg, 0 as libc::c_int as u_int) as
                *mut PAGE;
        if h.is_null() { return -(1 as libc::c_int) }
        idx = 0 as libc::c_int as indx_t
    }
    (*ep).page = h;
    (*ep).index = idx;
    return 0 as libc::c_int;
}
/*
 * Get the last item on the previous page, but by going up and down the tree.
 */
#[c2rust::src_loc = "447:1"]
unsafe extern "C" fn bt_rseq_prev(mut t: *mut BTREE, mut ep: *mut EPG)
 -> libc::c_int {
    let mut h: *mut PAGE = 0 as *mut PAGE;
    let mut idx: indx_t = 0;
    let mut up: *mut EPGNO = 0 as *mut EPGNO;
    let mut pg: db_pgno_t = 0;
    h = (*ep).page;
    idx = (*ep).index;
    loop  {
        /* Move up the tree. */
        up =
            if (*t).bt_sp == (*t).bt_stack.as_mut_ptr() {
                0 as *mut EPGNO
            } else { (*t).bt_sp = (*t).bt_sp.offset(-1); (*t).bt_sp };
        kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                       0 as libc::c_int as u_int);
        /* Did we hit the left edge of the root? */
        if up.is_null() { return 1 as libc::c_int }
        h =
            kdb2_mpool_get((*t).bt_mp, (*up).pgno, 0 as libc::c_int as u_int)
                as *mut PAGE;
        if h.is_null() { return -(1 as libc::c_int) }
        idx = (*up).index;
        if !(idx as libc::c_int == 0 as libc::c_int) { break ; }
    }
    idx = idx.wrapping_sub(1);
    while (*h).flags &
              (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint == 0
          {
        /* Move back down the tree. */
        (*(*t).bt_sp).pgno = (*h).pgno;
        (*(*t).bt_sp).index = idx;
        (*t).bt_sp = (*t).bt_sp.offset(1);
        pg =
            (*((h as
                    *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(idx
                                                                                 as
                                                                                 isize)
                                                  as libc::c_int as isize) as
                   *mut libc::c_void as *mut BINTERNAL)).pgno;
        kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                       0 as libc::c_int as u_int);
        h =
            kdb2_mpool_get((*t).bt_mp, pg, 0 as libc::c_int as u_int) as
                *mut PAGE;
        if h.is_null() { return -(1 as libc::c_int) }
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
                as indx_t
    }
    (*ep).page = h;
    (*ep).index = idx;
    return 0 as libc::c_int;
}
/*
 * Copyright (C) 2002, 2016 by the Massachusetts Institute of Technology.
 * All rights reserved.
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
 * __bt_first --
 *	Find the first entry.
 *
 * Parameters:
 *	t:	the tree
 *    key:	the key
 *  erval:	return EPG
 * exactp:	pointer to exact match flag
 *
 * Returns:
 *	The first entry in the tree greater than or equal to key,
 *	or RET_SPECIAL if no such key exists.
 */
#[c2rust::src_loc = "497:1"]
unsafe extern "C" fn __bt_first(mut t: *mut BTREE, mut key: *const DBT,
                                mut erval: *mut EPG,
                                mut exactp: *mut libc::c_int) -> libc::c_int {
    let mut h: *mut PAGE = 0 as *mut PAGE;
    let mut hprev: *mut PAGE = 0 as *mut PAGE;
    let mut ep: *mut EPG = 0 as *mut EPG;
    let mut save: EPG = EPG{page: 0 as *mut PAGE, index: 0,};
    let mut pg: db_pgno_t = 0;
    /*
	 * Find any matching record; __bt_search pins the page.
	 *
	 * If it's an exact match and duplicates are possible, walk backwards
	 * in the tree until we find the first one.  Otherwise, make sure it's
	 * a valid key (__bt_search may return an index just past the end of a
	 * page) and return it.
	 */
    ep = __kdb2_bt_search(t, key, exactp);
    if ep.is_null() { return 1 as libc::c_int }
    if *exactp != 0 {
        if (*t).flags & 0x20 as libc::c_int as libc::c_uint != 0 {
            *erval = *ep;
            return 0 as libc::c_int
        }
        /*
		 * Walk backwards, as long as the entry matches and there are
		 * keys left in the tree.  Save a copy of each match in case
		 * we go too far.
		 */
        save = *ep;
        h = (*ep).page;
        loop  {
            if (*save.page).pgno != (*(*ep).page).pgno {
                kdb2_mpool_put((*t).bt_mp, save.page as *mut libc::c_void,
                               0 as libc::c_int as u_int);
                save = *ep
            } else { save.index = (*ep).index }
            /*
			 * Don't unpin the page the last (or original) match
			 * was on, but make sure it's unpinned if an error
			 * occurs.
			 */
            if (*ep).index as libc::c_int == 0 as libc::c_int {
                if (*h).prevpg == 0 as libc::c_int as libc::c_uint { break ; }
                if (*h).pgno != (*save.page).pgno {
                    kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                                   0 as libc::c_int as u_int);
                }
                hprev =
                    kdb2_mpool_get((*t).bt_mp, (*h).prevpg,
                                   0 as libc::c_int as u_int) as *mut PAGE;
                if hprev.is_null() {
                    if (*h).pgno == (*save.page).pgno {
                        kdb2_mpool_put((*t).bt_mp,
                                       save.page as *mut libc::c_void,
                                       0 as libc::c_int as u_int);
                    }
                    return -(1 as libc::c_int)
                }
                h = hprev;
                (*ep).page = h;
                (*ep).index =
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
                        as indx_t
            }
            (*ep).index = (*ep).index.wrapping_sub(1);
            if !(__kdb2_bt_cmp(t, key, ep) == 0 as libc::c_int) { break ; }
        }
        /*
		 * Reach here with the last page that was looked at pinned,
		 * which may or may not be the same as the last (or original)
		 * match page.  If it's not useful, release it.
		 */
        if (*h).pgno != (*save.page).pgno {
            kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                           0 as libc::c_int as u_int);
        }
        *erval = save;
        return 0 as libc::c_int
    }
    /* If at the end of a page, find the next entry. */
    if (*ep).index as libc::c_ulong ==
           ((*(*ep).page).lower as
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
        h = (*ep).page;
        pg = (*h).nextpg;
        kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                       0 as libc::c_int as u_int);
        if pg == 0 as libc::c_int as libc::c_uint { return 1 as libc::c_int }
        h =
            kdb2_mpool_get((*t).bt_mp, pg, 0 as libc::c_int as u_int) as
                *mut PAGE;
        if h.is_null() { return -(1 as libc::c_int) }
        (*ep).index = 0 as libc::c_int as indx_t;
        (*ep).page = h
    }
    *erval = *ep;
    return 0 as libc::c_int;
}
/*-
 * Copyright (c) 1991, 1993, 1994
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
 *	@(#)extern.h	8.11 (Berkeley) 1/9/95
 */
/*
 * __bt_setcur --
 *	Set the cursor to an entry in the tree.
 *
 * Parameters:
 *	t:	the tree
 *   pgno:	page number
 *  index:	page index
 */
#[no_mangle]
#[c2rust::src_loc = "598:1"]
pub unsafe extern "C" fn __kdb2_bt_setcur(mut t: *mut BTREE,
                                          mut pgno: db_pgno_t,
                                          mut idx: u_int) {
    /* Lose any already deleted key. */
    if !(*t).bt_cursor.key.data.is_null() {
        free((*t).bt_cursor.key.data);
        (*t).bt_cursor.key.size = 0 as libc::c_int as size_t;
        (*t).bt_cursor.key.data = 0 as *mut libc::c_void
    }
    (*t).bt_cursor.flags =
        ((*t).bt_cursor.flags as libc::c_int &
             !(0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int))
            as u_int8_t;
    /* Update the cursor. */
    (*t).bt_cursor.pg.pgno = pgno;
    (*t).bt_cursor.pg.index = idx as indx_t;
    (*t).bt_cursor.flags =
        ((*t).bt_cursor.flags as libc::c_int | 0x8 as libc::c_int) as
            u_int8_t;
}
