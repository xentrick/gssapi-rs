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
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/db.h:48"]
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
    use super::stddef_h::size_t;
    use super::sys_types_h::u_int;
    /* !_DB_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/include/db-int.h:48"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/mpool/mpool.h:49"]
pub mod mpool_h {
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
 *	@(#)mpool.h	8.4 (Berkeley) 11/2/95
 */
    /*
 * The memory pool scheme is a simple one.  Each in-memory page is referenced
 * by a bucket which is threaded in up to two of three ways.  All active pages
 * are threaded on a hash chain (hashed by page number) and an lru chain.
 * Inactive pages are threaded on a free chain.  Each reference to a memory
 * pool is handed an opaque MPOOL cookie which stores all of this information.
 */
    /* The BKT structures are the elements of the queues. */
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
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/btree/btree.h:49"]
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
    #[c2rust::src_loc = "241:16"]
    pub struct _epgno {
        pub pgno: db_pgno_t,
        pub index: indx_t,
    }
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
    /* the page number */
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
    use super::sys_types_h::{u_int32_t, u_int8_t, caddr_t, u_char};
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
#[c2rust::header_src = "/usr/include/stdlib.h:45"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
    }
}
#[c2rust::header_src = "/usr/include/string.h:46"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "46:14"]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/btree/extern.h:49"]
pub mod extern_h {
    use super::btree_h::{BTREE, EPG, PAGE};
    use super::db_h::DBT;
    use super::sys_types_h::{u_int, u_int32_t};
    use super::db_int_h::db_pgno_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "68:1"]
        pub fn __kdb2_bt_cmp(_: *mut BTREE, _: *const DBT, _: *mut EPG)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "73:1"]
        pub fn __kdb2_bt_deleaf(_: *mut BTREE, _: *const DBT, _: *mut PAGE,
                                _: u_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "83:1"]
        pub fn __kdb2_bt_search(_: *mut BTREE, _: *const DBT,
                                _: *mut libc::c_int) -> *mut EPG;
        #[no_mangle]
        #[c2rust::src_loc = "85:1"]
        pub fn __kdb2_bt_setcur(_: *mut BTREE, _: db_pgno_t, _: u_int);
        #[no_mangle]
        #[c2rust::src_loc = "86:1"]
        pub fn __kdb2_bt_split(_: *mut BTREE, _: *mut PAGE, _: *const DBT,
                               _: *const DBT, _: libc::c_int, _: size_t,
                               _: u_int32_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "92:1"]
        pub fn __kdb2_ovfl_put(_: *mut BTREE, _: *const DBT,
                               _: *mut db_pgno_t) -> libc::c_int;
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
pub use self::btree_h::{_page, PAGE, _epgno, EPGNO, _epg, EPG, _cursor,
                        CURSOR, _btree, C2RustUnnamed_1, FORWARD, BACK, NOT,
                        BTREE};
use self::errno_h::__errno_location;
use self::stdlib_h::abort;
use self::string_h::memmove;
use self::extern_h::{__kdb2_bt_cmp, __kdb2_bt_deleaf, __kdb2_bt_search,
                     __kdb2_bt_setcur, __kdb2_bt_split, __kdb2_ovfl_put};
/*
 * __BT_PUT -- Add a btree item to the tree.
 *
 * Parameters:
 *	dbp:	pointer to access method
 *	key:	key
 *	data:	data
 *	flag:	R_NOOVERWRITE
 *
 * Returns:
 *	RET_ERROR, RET_SUCCESS and RET_SPECIAL if the key is already in the
 *	tree and R_NOOVERWRITE specified.
 */
#[no_mangle]
#[c2rust::src_loc = "66:1"]
pub unsafe extern "C" fn __kdb2_bt_put(mut dbp: *const DB, mut key: *mut DBT,
                                       mut data: *const DBT, mut flags: u_int)
 -> libc::c_int {
    let mut current_block: u64;
    let mut t: *mut BTREE = 0 as *mut BTREE;
    let mut tkey: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    let mut tdata: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    let mut e: *mut EPG = 0 as *mut EPG;
    let mut h: *mut PAGE = 0 as *mut PAGE;
    let mut idx: indx_t = 0;
    let mut nxtindex: indx_t = 0;
    let mut pg: db_pgno_t = 0;
    let mut nbytes: u_int32_t = 0;
    let mut dflags: libc::c_int = 0;
    let mut exact: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut dest: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut db: [libc::c_char; 8] = [0; 8];
    let mut kb: [libc::c_char; 8] = [0; 8];
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
    let mut current_block_10: u64;
    match flags {
        0 | 8 => { current_block_10 = 5689001924483802034; }
        1 => {
            /*
		 * If flags is R_CURSOR, put the cursor.  Must already
		 * have started a scan and not have already deleted it.
		 */
            if (*t).bt_cursor.flags as libc::c_int & 0x8 as libc::c_int != 0
                   &&
                   (*t).bt_cursor.flags as libc::c_int &
                       (0x1 as libc::c_int | 0x2 as libc::c_int |
                            0x4 as libc::c_int) == 0 {
                current_block_10 = 5689001924483802034;
            } else { current_block_10 = 13683688309710625016; }
        }
        _ => { current_block_10 = 13683688309710625016; }
    }
    match current_block_10 {
        5689001924483802034 => { }
        _ =>
        /* FALLTHROUGH */
        {
            *__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int)
        }
    }
    /*
	 * If the key/data pair won't fit on a page, store it on overflow
	 * pages.  Only put the key on the overflow page if the pair are
	 * still too big after moving the data to an overflow page.
	 *
	 * XXX
	 * If the insert fails later on, the overflow pages aren't recovered.
	 */
    dflags = 0 as libc::c_int;
    if (*key).size.wrapping_add((*data).size) >
           (*t).bt_ovflsize as libc::c_ulong {
        let mut yuck_this_is_gross_code: u_int32_t = 0;
        let mut current_block_35: u64;
        if (*key).size > (*t).bt_ovflsize as libc::c_ulong {
            yuck_this_is_gross_code = 0;
            current_block_35 = 1900231684298568950;
        } else { current_block_35 = 15897653523371991391; }
        loop  {
            match current_block_35 {
                1900231684298568950 => {
                    if __kdb2_ovfl_put(t, key, &mut pg) == -(1 as libc::c_int)
                       {
                        return -(1 as libc::c_int)
                    }
                    tkey.data = kb.as_mut_ptr() as *mut libc::c_void;
                    tkey.size =
                        (::std::mem::size_of::<db_pgno_t>() as
                             libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                             as
                                                             libc::c_ulong);
                    memmove(kb.as_mut_ptr() as *mut libc::c_void,
                            &mut pg as *mut db_pgno_t as *const libc::c_void,
                            ::std::mem::size_of::<db_pgno_t>() as
                                libc::c_ulong);
                    yuck_this_is_gross_code = (*key).size as u_int32_t;
                    if yuck_this_is_gross_code as libc::c_ulong != (*key).size
                       {
                        abort();
                    }
                    memmove(kb.as_mut_ptr().offset(::std::mem::size_of::<db_pgno_t>()
                                                       as libc::c_ulong as
                                                       isize) as
                                *mut libc::c_void,
                            &mut yuck_this_is_gross_code as *mut u_int32_t as
                                *const libc::c_void,
                            ::std::mem::size_of::<u_int32_t>() as
                                libc::c_ulong);
                    dflags |= 0x2 as libc::c_int;
                    key = &mut tkey;
                    current_block_35 = 15897653523371991391;
                }
                _ => {
                    if (*key).size.wrapping_add((*data).size) >
                           (*t).bt_ovflsize as libc::c_ulong {
                        let mut yuck_this_is_gross_code_0: u_int32_t =
                            (*data).size as u_int32_t;
                        if __kdb2_ovfl_put(t, data, &mut pg) ==
                               -(1 as libc::c_int) {
                            return -(1 as libc::c_int)
                        }
                        tdata.data = db.as_mut_ptr() as *mut libc::c_void;
                        tdata.size =
                            (::std::mem::size_of::<db_pgno_t>() as
                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                 as
                                                                 libc::c_ulong);
                        memmove(db.as_mut_ptr() as *mut libc::c_void,
                                &mut pg as *mut db_pgno_t as
                                    *const libc::c_void,
                                ::std::mem::size_of::<db_pgno_t>() as
                                    libc::c_ulong);
                        if yuck_this_is_gross_code_0 as libc::c_ulong !=
                               (*data).size {
                            abort();
                        }
                        memmove(db.as_mut_ptr().offset(::std::mem::size_of::<db_pgno_t>()
                                                           as libc::c_ulong as
                                                           isize) as
                                    *mut libc::c_void,
                                &mut yuck_this_is_gross_code_0 as
                                    *mut u_int32_t as *const libc::c_void,
                                ::std::mem::size_of::<u_int32_t>() as
                                    libc::c_ulong);
                        dflags |= 0x1 as libc::c_int;
                        data = &mut tdata
                    }
                    if (*key).size.wrapping_add((*data).size) >
                           (*t).bt_ovflsize as libc::c_ulong {
                        current_block_35 = 1900231684298568950;
                    } else { break ; }
                }
            }
        }
    }
    /* Replace the cursor. */
    if flags == 1 as libc::c_int as libc::c_uint {
        h =
            kdb2_mpool_get((*t).bt_mp, (*t).bt_cursor.pg.pgno,
                           0 as libc::c_int as u_int) as *mut PAGE;
        if h.is_null() { return -(1 as libc::c_int) }
        idx = (*t).bt_cursor.pg.index;
        current_block = 4510563271777997454;
    } else {
        /*
	 * Find the key to delete, or, the location at which to insert.
	 * Bt_fast and __bt_search both pin the returned page.
	 */
        if (*t).bt_order as libc::c_uint == NOT as libc::c_int as libc::c_uint
               || { e = bt_fast(t, key, data, &mut exact); e.is_null() } {
            e = __kdb2_bt_search(t, key, &mut exact);
            if e.is_null() { return -(1 as libc::c_int) }
        }
        h = (*e).page;
        idx = (*e).index;
        /*
	 * Add the key/data pair to the tree.  If an identical key is already
	 * in the tree, and R_NOOVERWRITE is set, an error is returned.  If
	 * R_NOOVERWRITE is not set, the key is either added (if duplicates are
	 * permitted) or an error is returned.
	 */
        match flags {
            8 => {
                if exact == 0 {
                    current_block = 15594603006322722090;
                } else {
                    kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                                   0 as libc::c_int as u_int);
                    return 1 as libc::c_int
                }
            }
            _ => {
                if exact == 0 ||
                       (*t).flags & 0x20 as libc::c_int as libc::c_uint == 0 {
                    current_block = 15594603006322722090;
                } else { current_block = 4510563271777997454; }
            }
        }
    }
    match current_block {
        4510563271777997454 =>
        /*
		 * !!!
		 * Note, the delete may empty the page, so we need to put a
		 * new entry into the page immediately.
		 */
        {
            if __kdb2_bt_deleaf(t, key, h, idx as u_int) ==
                   -(1 as libc::c_int) {
                kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                               0 as libc::c_int as u_int);
                return -(1 as libc::c_int)
            }
        }
        _ => { }
    }
    /*
	 * If not enough room, or the user has put a ceiling on the number of
	 * keys permitted in the page, split the page.  The split code will
	 * insert the key and data and unpin the current page.  If inserting
	 * into the offset array, shift the pointers up.
	 */
    nbytes =
        ((::std::mem::size_of::<u_int32_t>() as
              libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                              as
                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<u_char>()
                                                                              as
                                                                              libc::c_ulong).wrapping_add((*key).size).wrapping_add((*data).size).wrapping_add(::std::mem::size_of::<db_pgno_t>()
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
    if (((*h).upper as u_int32_t).wrapping_sub((*h).lower as u_int32_t) as
            libc::c_ulong) <
           (nbytes as
                libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>() as
                                                libc::c_ulong) {
        status =
            __kdb2_bt_split(t, h, key, data, dflags, nbytes as size_t,
                            idx as u_int32_t);
        if status != 0 as libc::c_int { return status }
    } else {
        nxtindex =
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
        if (idx as libc::c_int) < nxtindex as libc::c_int {
            memmove((*h).linp.as_mut_ptr().offset(idx as libc::c_int as
                                                      isize).offset(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                        as *mut libc::c_void,
                    (*h).linp.as_mut_ptr().offset(idx as libc::c_int as isize)
                        as *const libc::c_void,
                    ((nxtindex as libc::c_int - idx as libc::c_int) as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<indx_t>()
                                                         as libc::c_ulong));
        }
        (*h).lower =
            ((*h).lower as
                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                 as libc::c_ulong) as indx_t
                as indx_t;
        (*h).upper =
            ((*h).upper as libc::c_uint).wrapping_sub(nbytes) as indx_t as
                indx_t;
        *(*h).linp.as_mut_ptr().offset(idx as isize) = (*h).upper;
        dest =
            (h as
                 *mut libc::c_char).offset((*h).upper as libc::c_int as
                                               isize);
        *(dest as *mut libc::c_void as *mut u_int32_t) =
            (*key).size as u_int32_t;
        dest =
            dest.offset(::std::mem::size_of::<u_int32_t>() as libc::c_ulong as
                            isize);
        *(dest as *mut libc::c_void as *mut u_int32_t) =
            (*data).size as u_int32_t;
        dest =
            dest.offset(::std::mem::size_of::<u_int32_t>() as libc::c_ulong as
                            isize);
        *(dest as *mut u_char) = dflags as u_char;
        dest =
            dest.offset(::std::mem::size_of::<u_char>() as libc::c_ulong as
                            isize);
        memmove(dest as *mut libc::c_void, (*key).data, (*key).size);
        dest = dest.offset((*key).size as isize);
        memmove(dest as *mut libc::c_void, (*data).data, (*data).size);
        /* If the cursor is on this page, adjust it as necessary. */
        if (*t).bt_cursor.flags as libc::c_int & 0x8 as libc::c_int != 0 &&
               (*t).bt_cursor.flags as libc::c_int & 0x1 as libc::c_int == 0
               && (*t).bt_cursor.pg.pgno == (*h).pgno &&
               (*t).bt_cursor.pg.index as libc::c_int >= idx as libc::c_int {
            (*t).bt_cursor.pg.index = (*t).bt_cursor.pg.index.wrapping_add(1)
        }
        if (*t).bt_order as libc::c_uint == NOT as libc::c_int as libc::c_uint
           {
            if (*h).nextpg == 0 as libc::c_int as libc::c_uint {
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
                   {
                    (*t).bt_order = FORWARD;
                    (*t).bt_last.index = idx;
                    (*t).bt_last.pgno = (*h).pgno
                }
            } else if (*h).prevpg == 0 as libc::c_int as libc::c_uint {
                if idx as libc::c_int == 0 as libc::c_int {
                    (*t).bt_order = BACK;
                    (*t).bt_last.index = 0 as libc::c_int as indx_t;
                    (*t).bt_last.pgno = (*h).pgno
                }
            }
        }
        kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                       0x1 as libc::c_int as u_int);
    }
    if flags == 10 as libc::c_int as libc::c_uint {
        __kdb2_bt_setcur(t, (*(*e).page).pgno, (*e).index as u_int);
    }
    (*t).flags |= 0x4 as libc::c_int as libc::c_uint;
    return 0 as libc::c_int;
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
 * BT_FAST -- Do a quick check for sorted data.
 *
 * Parameters:
 *	t:	tree
 *	key:	key to insert
 *
 * Returns:
 * 	EPG for new record or NULL if not found.
 */
#[c2rust::src_loc = "274:1"]
unsafe extern "C" fn bt_fast(mut t: *mut BTREE, mut key: *const DBT,
                             mut data: *const DBT,
                             mut exactp: *mut libc::c_int) -> *mut EPG {
    let mut current_block: u64;
    let mut h: *mut PAGE = 0 as *mut PAGE;
    let mut nbytes: u_int32_t = 0;
    let mut cmp: libc::c_int = 0;
    h =
        kdb2_mpool_get((*t).bt_mp, (*t).bt_last.pgno,
                       0 as libc::c_int as u_int) as *mut PAGE;
    if h.is_null() { (*t).bt_order = NOT; return 0 as *mut EPG }
    (*t).bt_cur.page = h;
    (*t).bt_cur.index = (*t).bt_last.index;
    /*
	 * If won't fit in this page or have too many keys in this page,
	 * have to search to get split stack.
	 */
    nbytes =
        ((::std::mem::size_of::<u_int32_t>() as
              libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                              as
                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<u_char>()
                                                                              as
                                                                              libc::c_ulong).wrapping_add((*key).size).wrapping_add((*data).size).wrapping_add(::std::mem::size_of::<db_pgno_t>()
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
    if !((((*h).upper as u_int32_t).wrapping_sub((*h).lower as u_int32_t) as
              libc::c_ulong) <
             (nbytes as
                  libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                  as libc::c_ulong)) {
        if (*t).bt_order as libc::c_uint ==
               FORWARD as libc::c_int as libc::c_uint {
            if (*(*t).bt_cur.page).nextpg != 0 as libc::c_int as libc::c_uint
               {
                current_block = 18351798603502974965;
            } else if (*t).bt_cur.index as libc::c_ulong !=
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
                current_block = 18351798603502974965;
            } else {
                cmp = __kdb2_bt_cmp(t, key, &mut (*t).bt_cur);
                if cmp < 0 as libc::c_int {
                    current_block = 18351798603502974965;
                } else {
                    (*t).bt_last.index =
                        if cmp != 0 {
                            (*t).bt_cur.index =
                                (*t).bt_cur.index.wrapping_add(1);
                            (*t).bt_cur.index as libc::c_int
                        } else { (*t).bt_cur.index as libc::c_int } as indx_t;
                    current_block = 12124785117276362961;
                }
            }
        } else if (*(*t).bt_cur.page).prevpg !=
                      0 as libc::c_int as libc::c_uint {
            current_block = 18351798603502974965;
        } else if (*t).bt_cur.index as libc::c_int != 0 as libc::c_int {
            current_block = 18351798603502974965;
        } else {
            cmp = __kdb2_bt_cmp(t, key, &mut (*t).bt_cur);
            if cmp > 0 as libc::c_int {
                current_block = 18351798603502974965;
            } else {
                (*t).bt_last.index = 0 as libc::c_int as indx_t;
                current_block = 12124785117276362961;
            }
        }
        match current_block {
            18351798603502974965 => { }
            _ => {
                *exactp = (cmp == 0 as libc::c_int) as libc::c_int;
                return &mut (*t).bt_cur
            }
        }
    }
    (*t).bt_order = NOT;
    kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                   0 as libc::c_int as u_int);
    return 0 as *mut EPG;
}
