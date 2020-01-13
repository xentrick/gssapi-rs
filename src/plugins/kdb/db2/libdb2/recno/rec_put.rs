use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:38"]
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
#[c2rust::header_src = "/usr/include/sys/types.h:38"]
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
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:38"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:41"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:41"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/db.h:45"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/include/db-int.h:45"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/recno/recno.h:46"]
pub mod recno_h {
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
 *
 *	@(#)recno.h	8.1 (Berkeley) 6/4/93
 */
    #[c2rust::src_loc = "36:1"]
    pub type SRCHOP = libc::c_uint;
    #[c2rust::src_loc = "36:33"]
    pub const SEARCH: SRCHOP = 2;
    #[c2rust::src_loc = "36:24"]
    pub const SINSERT: SRCHOP = 1;
    #[c2rust::src_loc = "36:15"]
    pub const SDELETE: SRCHOP = 0;
    /* Rec_search operation. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/mpool/mpool.h:46"]
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
        #[c2rust::src_loc = "111:1"]
        pub fn kdb2_mpool_put(_: *mut MPOOL, _: *mut libc::c_void, _: u_int)
         -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/btree/btree.h:46"]
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
    #[c2rust::src_loc = "67:1"]
    pub type PAGE = _page;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "241:16"]
    pub struct _epgno {
        pub pgno: db_pgno_t,
        pub index: indx_t,
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "275:16"]
    pub struct _cursor {
        pub pg: EPGNO,
        pub key: DBT,
        pub rcursor: recno_t,
        pub flags: u_int8_t,
    }
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
    /* B: Saved tree reference. */
    /* B: Saved key, or key.data == NULL. */
    /* R: recno cursor (1-based) */
    /*  B: Cursor needs to be reacquired. */
    /*  B: Unreturned cursor after key. */
    /*  B: Unreturned cursor before key. */
    /* RB: Cursor initialized. */
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
#[c2rust::header_src = "/usr/include/errno.h:40"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:42"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/string.h:43"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/recno/extern.h:46"]
pub mod extern_h {
    use super::btree_h::{BTREE, EPG, PAGE};
    use super::db_int_h::recno_t;
    use super::db_h::DBT;
    use super::recno_h::{SRCHOP, SDELETE};
    use super::sys_types_h::u_int32_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "66:1"]
        pub fn __kdb2_rec_ret(_: *mut BTREE, _: *mut EPG, _: recno_t,
                              _: *mut DBT, _: *mut DBT) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn __kdb2_rec_search(_: *mut BTREE, _: recno_t, _: SRCHOP)
         -> *mut EPG;
        #[no_mangle]
        #[c2rust::src_loc = "58:1"]
        pub fn __kdb2_rec_dleaf(_: *mut BTREE, _: *mut PAGE, _: u_int32_t)
         -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/btree/extern.h:46"]
pub mod btree_extern_h {
    use super::btree_h::{BTREE, PAGE};
    use super::db_h::DBT;
    use super::stddef_h::size_t;
    use super::sys_types_h::u_int32_t;
    use super::db_int_h::db_pgno_t;
    extern "C" {
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
pub use self::recno_h::{SRCHOP, SEARCH, SINSERT, SDELETE};
pub use self::mpool_h::{_bkt, C2RustUnnamed, C2RustUnnamed_0, MPOOL, _hqh,
                        _lqh, kdb2_mpool_put};
pub use self::btree_h::{_page, PAGE, _epgno, EPGNO, _epg, EPG, _cursor,
                        CURSOR, _btree, C2RustUnnamed_1, FORWARD, BACK, NOT,
                        BTREE};
use self::errno_h::__errno_location;
use self::stdlib_h::{malloc, realloc, free};
use self::string_h::{memcpy, memmove, memset};
use self::extern_h::{__kdb2_rec_ret, __kdb2_rec_search, __kdb2_rec_dleaf};
use self::btree_extern_h::{__kdb2_bt_split, __kdb2_ovfl_put};
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
 */
/* LIBC_SCCS and not lint */
/*
 * __REC_PUT -- Add a recno item to the tree.
 *
 * Parameters:
 *	dbp:	pointer to access method
 *	key:	key
 *	data:	data
 *	flag:	R_CURSOR, R_IAFTER, R_IBEFORE, R_NOOVERWRITE
 *
 * Returns:
 *	RET_ERROR, RET_SUCCESS and RET_SPECIAL if the key is
 *	already in the tree and R_NOOVERWRITE specified.
 */
#[no_mangle]
#[c2rust::src_loc = "61:1"]
pub unsafe extern "C" fn __kdb2_rec_put(mut dbp: *const DB, mut key: *mut DBT,
                                        mut data: *const DBT,
                                        mut flags: u_int) -> libc::c_int {
    let mut current_block: u64;
    let mut t: *mut BTREE = 0 as *mut BTREE;
    let mut fdata: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    let mut tdata: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    let mut nrec: recno_t = 0;
    let mut status: libc::c_int = 0;
    t = (*dbp).internal as *mut BTREE;
    /* Toss any page pinned across calls. */
    if !(*t).bt_pinned.is_null() {
        kdb2_mpool_put((*t).bt_mp, (*t).bt_pinned as *mut libc::c_void,
                       0 as libc::c_int as u_int);
        (*t).bt_pinned = 0 as *mut PAGE
    }
    /*
	 * If using fixed-length records, and the record is long, return
	 * EINVAL.  If it's short, pad it out.  Use the record data return
	 * memory, it's only short-term.
	 */
    if (*t).flags & 0x200 as libc::c_int as libc::c_uint != 0 &&
           (*data).size != (*t).bt_reclen {
        if (*data).size > (*t).bt_reclen {
            current_block = 14003073271977894080;
        } else {
            if (*t).bt_rdata.size < (*t).bt_reclen {
                (*t).bt_rdata.data =
                    if (*t).bt_rdata.data.is_null() {
                        malloc((*t).bt_reclen)
                    } else { realloc((*t).bt_rdata.data, (*t).bt_reclen) };
                if (*t).bt_rdata.data.is_null() { return -(1 as libc::c_int) }
                (*t).bt_rdata.size = (*t).bt_reclen
            }
            memmove((*t).bt_rdata.data, (*data).data, (*data).size);
            memset(((*t).bt_rdata.data as
                        *mut libc::c_char).offset((*data).size as isize) as
                       *mut libc::c_void, (*t).bt_bval as libc::c_int,
                   (*t).bt_reclen.wrapping_sub((*data).size));
            fdata.data = (*t).bt_rdata.data;
            fdata.size = (*t).bt_reclen;
            current_block = 26972500619410423;
        }
    } else {
        fdata.data = (*data).data;
        fdata.size = (*data).size;
        current_block = 26972500619410423;
    }
    match current_block {
        26972500619410423 => {
            match flags {
                1 => {
                    current_block = 9738537073278594712;
                    match current_block {
                        15664224570546439045 => {
                            nrec = *((*key).data as *mut recno_t);
                            if nrec == 0 as libc::c_int as libc::c_uint {
                                current_block = 14003073271977894080;
                            } else { current_block = 13131896068329595644; }
                        }
                        9738537073278594712 => {
                            if (*t).bt_cursor.flags as libc::c_int &
                                   0x8 as libc::c_int == 0 {
                                current_block = 14003073271977894080;
                            } else {
                                nrec = (*t).bt_cursor.rcursor;
                                current_block = 13131896068329595644;
                            }
                        }
                        14776158859635069963 => {
                            nrec = *((*key).data as *mut recno_t);
                            if nrec == 0 as libc::c_int as libc::c_uint {
                                current_block = 14003073271977894080;
                            } else { current_block = 13131896068329595644; }
                        }
                        15872131333666336404 => {
                            nrec = *((*key).data as *mut recno_t);
                            if nrec == 0 as libc::c_int as libc::c_uint {
                                nrec = 1 as libc::c_int as recno_t;
                                flags = 5 as libc::c_int as u_int
                            }
                            current_block = 13131896068329595644;
                        }
                        _ => {
                            nrec = *((*key).data as *mut recno_t);
                            if nrec == 0 as libc::c_int as libc::c_uint {
                                current_block = 14003073271977894080;
                            } else {
                                if nrec <= (*t).bt_nrecs {
                                    return 1 as libc::c_int
                                }
                                current_block = 13131896068329595644;
                            }
                        }
                    }
                    match current_block {
                        14003073271977894080 => { }
                        _ => {
                            /*
	 * Make sure that records up to and including the put record are
	 * already in the database.  If skipping records, create empty ones.
	 */
                            if nrec > (*t).bt_nrecs {
                                if (*t).flags &
                                       (0x100 as libc::c_int |
                                            0x800 as libc::c_int) as
                                           libc::c_uint == 0 &&
                                       (*t).bt_irec.expect("non-null function pointer")(t,
                                                                                        nrec)
                                           == -(1 as libc::c_int) {
                                    return -(1 as libc::c_int)
                                }
                                if nrec >
                                       (*t).bt_nrecs.wrapping_add(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                                   {
                                    if (*t).flags &
                                           0x200 as libc::c_int as
                                               libc::c_uint != 0 {
                                        tdata.data = malloc((*t).bt_reclen);
                                        if tdata.data.is_null() {
                                            return -(1 as libc::c_int)
                                        }
                                        tdata.size = (*t).bt_reclen;
                                        memset(tdata.data,
                                               (*t).bt_bval as libc::c_int,
                                               tdata.size);
                                    } else {
                                        tdata.data = 0 as *mut libc::c_void;
                                        tdata.size =
                                            0 as libc::c_int as size_t
                                    }
                                    while nrec >
                                              (*t).bt_nrecs.wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                          {
                                        if __kdb2_rec_iput(t, (*t).bt_nrecs,
                                                           &mut tdata,
                                                           0 as libc::c_int as
                                                               u_int) !=
                                               0 as libc::c_int {
                                            return -(1 as libc::c_int)
                                        }
                                    }
                                    if (*t).flags &
                                           0x200 as libc::c_int as
                                               libc::c_uint != 0 {
                                        free(tdata.data);
                                    }
                                }
                            }
                            status =
                                __kdb2_rec_iput(t,
                                                nrec.wrapping_sub(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint),
                                                &mut fdata, flags);
                            if status != 0 as libc::c_int { return status }
                            if flags == 10 as libc::c_int as libc::c_uint {
                                (*t).bt_cursor.rcursor = nrec
                            }
                            (*t).flags |=
                                0x1000 as libc::c_int as libc::c_uint;
                            return __kdb2_rec_ret(t, 0 as *mut EPG, nrec, key,
                                                  0 as *mut DBT)
                        }
                    }
                }
                10 => {
                    current_block = 14776158859635069963;
                    match current_block {
                        15664224570546439045 => {
                            nrec = *((*key).data as *mut recno_t);
                            if nrec == 0 as libc::c_int as libc::c_uint {
                                current_block = 14003073271977894080;
                            } else { current_block = 13131896068329595644; }
                        }
                        9738537073278594712 => {
                            if (*t).bt_cursor.flags as libc::c_int &
                                   0x8 as libc::c_int == 0 {
                                current_block = 14003073271977894080;
                            } else {
                                nrec = (*t).bt_cursor.rcursor;
                                current_block = 13131896068329595644;
                            }
                        }
                        14776158859635069963 => {
                            nrec = *((*key).data as *mut recno_t);
                            if nrec == 0 as libc::c_int as libc::c_uint {
                                current_block = 14003073271977894080;
                            } else { current_block = 13131896068329595644; }
                        }
                        15872131333666336404 => {
                            nrec = *((*key).data as *mut recno_t);
                            if nrec == 0 as libc::c_int as libc::c_uint {
                                nrec = 1 as libc::c_int as recno_t;
                                flags = 5 as libc::c_int as u_int
                            }
                            current_block = 13131896068329595644;
                        }
                        _ => {
                            nrec = *((*key).data as *mut recno_t);
                            if nrec == 0 as libc::c_int as libc::c_uint {
                                current_block = 14003073271977894080;
                            } else {
                                if nrec <= (*t).bt_nrecs {
                                    return 1 as libc::c_int
                                }
                                current_block = 13131896068329595644;
                            }
                        }
                    }
                    match current_block {
                        14003073271977894080 => { }
                        _ => {
                            if nrec > (*t).bt_nrecs {
                                if (*t).flags &
                                       (0x100 as libc::c_int |
                                            0x800 as libc::c_int) as
                                           libc::c_uint == 0 &&
                                       (*t).bt_irec.expect("non-null function pointer")(t,
                                                                                        nrec)
                                           == -(1 as libc::c_int) {
                                    return -(1 as libc::c_int)
                                }
                                if nrec >
                                       (*t).bt_nrecs.wrapping_add(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                                   {
                                    if (*t).flags &
                                           0x200 as libc::c_int as
                                               libc::c_uint != 0 {
                                        tdata.data = malloc((*t).bt_reclen);
                                        if tdata.data.is_null() {
                                            return -(1 as libc::c_int)
                                        }
                                        tdata.size = (*t).bt_reclen;
                                        memset(tdata.data,
                                               (*t).bt_bval as libc::c_int,
                                               tdata.size);
                                    } else {
                                        tdata.data = 0 as *mut libc::c_void;
                                        tdata.size =
                                            0 as libc::c_int as size_t
                                    }
                                    while nrec >
                                              (*t).bt_nrecs.wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                          {
                                        if __kdb2_rec_iput(t, (*t).bt_nrecs,
                                                           &mut tdata,
                                                           0 as libc::c_int as
                                                               u_int) !=
                                               0 as libc::c_int {
                                            return -(1 as libc::c_int)
                                        }
                                    }
                                    if (*t).flags &
                                           0x200 as libc::c_int as
                                               libc::c_uint != 0 {
                                        free(tdata.data);
                                    }
                                }
                            }
                            status =
                                __kdb2_rec_iput(t,
                                                nrec.wrapping_sub(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint),
                                                &mut fdata, flags);
                            if status != 0 as libc::c_int { return status }
                            if flags == 10 as libc::c_int as libc::c_uint {
                                (*t).bt_cursor.rcursor = nrec
                            }
                            (*t).flags |=
                                0x1000 as libc::c_int as libc::c_uint;
                            return __kdb2_rec_ret(t, 0 as *mut EPG, nrec, key,
                                                  0 as *mut DBT)
                        }
                    }
                }
                4 => {
                    current_block = 15872131333666336404;
                    match current_block {
                        15664224570546439045 => {
                            nrec = *((*key).data as *mut recno_t);
                            if nrec == 0 as libc::c_int as libc::c_uint {
                                current_block = 14003073271977894080;
                            } else { current_block = 13131896068329595644; }
                        }
                        9738537073278594712 => {
                            if (*t).bt_cursor.flags as libc::c_int &
                                   0x8 as libc::c_int == 0 {
                                current_block = 14003073271977894080;
                            } else {
                                nrec = (*t).bt_cursor.rcursor;
                                current_block = 13131896068329595644;
                            }
                        }
                        14776158859635069963 => {
                            nrec = *((*key).data as *mut recno_t);
                            if nrec == 0 as libc::c_int as libc::c_uint {
                                current_block = 14003073271977894080;
                            } else { current_block = 13131896068329595644; }
                        }
                        15872131333666336404 => {
                            nrec = *((*key).data as *mut recno_t);
                            if nrec == 0 as libc::c_int as libc::c_uint {
                                nrec = 1 as libc::c_int as recno_t;
                                flags = 5 as libc::c_int as u_int
                            }
                            current_block = 13131896068329595644;
                        }
                        _ => {
                            nrec = *((*key).data as *mut recno_t);
                            if nrec == 0 as libc::c_int as libc::c_uint {
                                current_block = 14003073271977894080;
                            } else {
                                if nrec <= (*t).bt_nrecs {
                                    return 1 as libc::c_int
                                }
                                current_block = 13131896068329595644;
                            }
                        }
                    }
                    match current_block {
                        14003073271977894080 => { }
                        _ => {
                            if nrec > (*t).bt_nrecs {
                                if (*t).flags &
                                       (0x100 as libc::c_int |
                                            0x800 as libc::c_int) as
                                           libc::c_uint == 0 &&
                                       (*t).bt_irec.expect("non-null function pointer")(t,
                                                                                        nrec)
                                           == -(1 as libc::c_int) {
                                    return -(1 as libc::c_int)
                                }
                                if nrec >
                                       (*t).bt_nrecs.wrapping_add(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                                   {
                                    if (*t).flags &
                                           0x200 as libc::c_int as
                                               libc::c_uint != 0 {
                                        tdata.data = malloc((*t).bt_reclen);
                                        if tdata.data.is_null() {
                                            return -(1 as libc::c_int)
                                        }
                                        tdata.size = (*t).bt_reclen;
                                        memset(tdata.data,
                                               (*t).bt_bval as libc::c_int,
                                               tdata.size);
                                    } else {
                                        tdata.data = 0 as *mut libc::c_void;
                                        tdata.size =
                                            0 as libc::c_int as size_t
                                    }
                                    while nrec >
                                              (*t).bt_nrecs.wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                          {
                                        if __kdb2_rec_iput(t, (*t).bt_nrecs,
                                                           &mut tdata,
                                                           0 as libc::c_int as
                                                               u_int) !=
                                               0 as libc::c_int {
                                            return -(1 as libc::c_int)
                                        }
                                    }
                                    if (*t).flags &
                                           0x200 as libc::c_int as
                                               libc::c_uint != 0 {
                                        free(tdata.data);
                                    }
                                }
                            }
                            status =
                                __kdb2_rec_iput(t,
                                                nrec.wrapping_sub(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint),
                                                &mut fdata, flags);
                            if status != 0 as libc::c_int { return status }
                            if flags == 10 as libc::c_int as libc::c_uint {
                                (*t).bt_cursor.rcursor = nrec
                            }
                            (*t).flags |=
                                0x1000 as libc::c_int as libc::c_uint;
                            return __kdb2_rec_ret(t, 0 as *mut EPG, nrec, key,
                                                  0 as *mut DBT)
                        }
                    }
                }
                0 | 5 => {
                    current_block = 15664224570546439045;
                    match current_block {
                        15664224570546439045 => {
                            nrec = *((*key).data as *mut recno_t);
                            if nrec == 0 as libc::c_int as libc::c_uint {
                                current_block = 14003073271977894080;
                            } else { current_block = 13131896068329595644; }
                        }
                        9738537073278594712 => {
                            if (*t).bt_cursor.flags as libc::c_int &
                                   0x8 as libc::c_int == 0 {
                                current_block = 14003073271977894080;
                            } else {
                                nrec = (*t).bt_cursor.rcursor;
                                current_block = 13131896068329595644;
                            }
                        }
                        14776158859635069963 => {
                            nrec = *((*key).data as *mut recno_t);
                            if nrec == 0 as libc::c_int as libc::c_uint {
                                current_block = 14003073271977894080;
                            } else { current_block = 13131896068329595644; }
                        }
                        15872131333666336404 => {
                            nrec = *((*key).data as *mut recno_t);
                            if nrec == 0 as libc::c_int as libc::c_uint {
                                nrec = 1 as libc::c_int as recno_t;
                                flags = 5 as libc::c_int as u_int
                            }
                            current_block = 13131896068329595644;
                        }
                        _ => {
                            nrec = *((*key).data as *mut recno_t);
                            if nrec == 0 as libc::c_int as libc::c_uint {
                                current_block = 14003073271977894080;
                            } else {
                                if nrec <= (*t).bt_nrecs {
                                    return 1 as libc::c_int
                                }
                                current_block = 13131896068329595644;
                            }
                        }
                    }
                    match current_block {
                        14003073271977894080 => { }
                        _ => {
                            if nrec > (*t).bt_nrecs {
                                if (*t).flags &
                                       (0x100 as libc::c_int |
                                            0x800 as libc::c_int) as
                                           libc::c_uint == 0 &&
                                       (*t).bt_irec.expect("non-null function pointer")(t,
                                                                                        nrec)
                                           == -(1 as libc::c_int) {
                                    return -(1 as libc::c_int)
                                }
                                if nrec >
                                       (*t).bt_nrecs.wrapping_add(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                                   {
                                    if (*t).flags &
                                           0x200 as libc::c_int as
                                               libc::c_uint != 0 {
                                        tdata.data = malloc((*t).bt_reclen);
                                        if tdata.data.is_null() {
                                            return -(1 as libc::c_int)
                                        }
                                        tdata.size = (*t).bt_reclen;
                                        memset(tdata.data,
                                               (*t).bt_bval as libc::c_int,
                                               tdata.size);
                                    } else {
                                        tdata.data = 0 as *mut libc::c_void;
                                        tdata.size =
                                            0 as libc::c_int as size_t
                                    }
                                    while nrec >
                                              (*t).bt_nrecs.wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                          {
                                        if __kdb2_rec_iput(t, (*t).bt_nrecs,
                                                           &mut tdata,
                                                           0 as libc::c_int as
                                                               u_int) !=
                                               0 as libc::c_int {
                                            return -(1 as libc::c_int)
                                        }
                                    }
                                    if (*t).flags &
                                           0x200 as libc::c_int as
                                               libc::c_uint != 0 {
                                        free(tdata.data);
                                    }
                                }
                            }
                            status =
                                __kdb2_rec_iput(t,
                                                nrec.wrapping_sub(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint),
                                                &mut fdata, flags);
                            if status != 0 as libc::c_int { return status }
                            if flags == 10 as libc::c_int as libc::c_uint {
                                (*t).bt_cursor.rcursor = nrec
                            }
                            (*t).flags |=
                                0x1000 as libc::c_int as libc::c_uint;
                            return __kdb2_rec_ret(t, 0 as *mut EPG, nrec, key,
                                                  0 as *mut DBT)
                        }
                    }
                }
                8 => {
                    current_block = 2797399796903382152;
                    match current_block {
                        15664224570546439045 => {
                            nrec = *((*key).data as *mut recno_t);
                            if nrec == 0 as libc::c_int as libc::c_uint {
                                current_block = 14003073271977894080;
                            } else { current_block = 13131896068329595644; }
                        }
                        9738537073278594712 => {
                            if (*t).bt_cursor.flags as libc::c_int &
                                   0x8 as libc::c_int == 0 {
                                current_block = 14003073271977894080;
                            } else {
                                nrec = (*t).bt_cursor.rcursor;
                                current_block = 13131896068329595644;
                            }
                        }
                        14776158859635069963 => {
                            nrec = *((*key).data as *mut recno_t);
                            if nrec == 0 as libc::c_int as libc::c_uint {
                                current_block = 14003073271977894080;
                            } else { current_block = 13131896068329595644; }
                        }
                        15872131333666336404 => {
                            nrec = *((*key).data as *mut recno_t);
                            if nrec == 0 as libc::c_int as libc::c_uint {
                                nrec = 1 as libc::c_int as recno_t;
                                flags = 5 as libc::c_int as u_int
                            }
                            current_block = 13131896068329595644;
                        }
                        _ => {
                            nrec = *((*key).data as *mut recno_t);
                            if nrec == 0 as libc::c_int as libc::c_uint {
                                current_block = 14003073271977894080;
                            } else {
                                if nrec <= (*t).bt_nrecs {
                                    return 1 as libc::c_int
                                }
                                current_block = 13131896068329595644;
                            }
                        }
                    }
                    match current_block {
                        14003073271977894080 => { }
                        _ => {
                            if nrec > (*t).bt_nrecs {
                                if (*t).flags &
                                       (0x100 as libc::c_int |
                                            0x800 as libc::c_int) as
                                           libc::c_uint == 0 &&
                                       (*t).bt_irec.expect("non-null function pointer")(t,
                                                                                        nrec)
                                           == -(1 as libc::c_int) {
                                    return -(1 as libc::c_int)
                                }
                                if nrec >
                                       (*t).bt_nrecs.wrapping_add(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                                   {
                                    if (*t).flags &
                                           0x200 as libc::c_int as
                                               libc::c_uint != 0 {
                                        tdata.data = malloc((*t).bt_reclen);
                                        if tdata.data.is_null() {
                                            return -(1 as libc::c_int)
                                        }
                                        tdata.size = (*t).bt_reclen;
                                        memset(tdata.data,
                                               (*t).bt_bval as libc::c_int,
                                               tdata.size);
                                    } else {
                                        tdata.data = 0 as *mut libc::c_void;
                                        tdata.size =
                                            0 as libc::c_int as size_t
                                    }
                                    while nrec >
                                              (*t).bt_nrecs.wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                          {
                                        if __kdb2_rec_iput(t, (*t).bt_nrecs,
                                                           &mut tdata,
                                                           0 as libc::c_int as
                                                               u_int) !=
                                               0 as libc::c_int {
                                            return -(1 as libc::c_int)
                                        }
                                    }
                                    if (*t).flags &
                                           0x200 as libc::c_int as
                                               libc::c_uint != 0 {
                                        free(tdata.data);
                                    }
                                }
                            }
                            status =
                                __kdb2_rec_iput(t,
                                                nrec.wrapping_sub(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint),
                                                &mut fdata, flags);
                            if status != 0 as libc::c_int { return status }
                            if flags == 10 as libc::c_int as libc::c_uint {
                                (*t).bt_cursor.rcursor = nrec
                            }
                            (*t).flags |=
                                0x1000 as libc::c_int as libc::c_uint;
                            return __kdb2_rec_ret(t, 0 as *mut EPG, nrec, key,
                                                  0 as *mut DBT)
                        }
                    }
                }
                _ => { }
            }
        }
        _ => { }
    }
    *__errno_location() = 22 as libc::c_int;
    return -(1 as libc::c_int);
}
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
 *
 *	@(#)extern.h	8.3 (Berkeley) 6/4/94
 */
/*
 * __REC_IPUT -- Add a recno item to the tree.
 *
 * Parameters:
 *	t:	tree
 *	nrec:	record number
 *	data:	data
 *
 * Returns:
 *	RET_ERROR, RET_SUCCESS
 */
#[no_mangle]
#[c2rust::src_loc = "189:1"]
pub unsafe extern "C" fn __kdb2_rec_iput(mut t: *mut BTREE, mut nrec: recno_t,
                                         mut data: *const DBT,
                                         mut flags: u_int) -> libc::c_int {
    let mut tdata: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    let mut e: *mut EPG = 0 as *mut EPG;
    let mut h: *mut PAGE = 0 as *mut PAGE;
    let mut idx: indx_t = 0;
    let mut nxtindex: indx_t = 0;
    let mut pg: db_pgno_t = 0;
    let mut nbytes: u_int32_t = 0;
    let mut dflags: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut dest: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut db: [libc::c_char; 8] = [0; 8];
    /*
	 * If the data won't fit on a page, store it on indirect pages.
	 *
	 * XXX
	 * If the insert fails later on, these pages aren't recovered.
	 */
    if (*data).size > (*t).bt_ovflsize as libc::c_ulong {
        if __kdb2_ovfl_put(t, data, &mut pg) == -(1 as libc::c_int) {
            return -(1 as libc::c_int)
        }
        tdata.data = db.as_mut_ptr() as *mut libc::c_void;
        tdata.size =
            (::std::mem::size_of::<db_pgno_t>() as
                 libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                 as libc::c_ulong);
        memcpy(db.as_mut_ptr() as *mut libc::c_void,
               &mut pg as *mut db_pgno_t as *const libc::c_void,
               ::std::mem::size_of::<db_pgno_t>() as libc::c_ulong);
        *(db.as_mut_ptr().offset(::std::mem::size_of::<db_pgno_t>() as
                                     libc::c_ulong as isize) as
              *mut libc::c_void as *mut u_int32_t) =
            (*data).size as u_int32_t;
        dflags = 0x1 as libc::c_int;
        data = &mut tdata
    } else { dflags = 0 as libc::c_int }
    /* __rec_search pins the returned page. */
    e =
        __kdb2_rec_search(t, nrec,
                          if nrec > (*t).bt_nrecs ||
                                 flags == 4 as libc::c_int as libc::c_uint ||
                                 flags == 5 as libc::c_int as libc::c_uint {
                              SINSERT as libc::c_int
                          } else { SEARCH as libc::c_int } as SRCHOP);
    if e.is_null() { return -(1 as libc::c_int) }
    h = (*e).page;
    idx = (*e).index;
    /*
	 * Add the specified key/data pair to the tree.  The R_IAFTER and
	 * R_IBEFORE flags insert the key after/before the specified key.
	 *
	 * Pages are split as required.
	 */
    match flags {
        4 => { idx = idx.wrapping_add(1) }
        5 => { }
        _ => {
            if nrec < (*t).bt_nrecs &&
                   __kdb2_rec_dleaf(t, h, idx as u_int32_t) ==
                       -(1 as libc::c_int) {
                kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                               0 as libc::c_int as u_int);
                return -(1 as libc::c_int)
            }
        }
    }
    /*
	 * If not enough room, split the page.  The split code will insert
	 * the key and data and unpin the current page.  If inserting into
	 * the offset array, shift the pointers up.
	 */
    nbytes =
        ((::std::mem::size_of::<u_int32_t>() as
              libc::c_ulong).wrapping_add(::std::mem::size_of::<u_char>() as
                                              libc::c_ulong).wrapping_add((*data).size).wrapping_add(::std::mem::size_of::<db_pgno_t>()
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
            __kdb2_bt_split(t, h, 0 as *const DBT, data, dflags,
                            nbytes as size_t, idx as u_int32_t);
        if status == 0 as libc::c_int {
            (*t).bt_nrecs = (*t).bt_nrecs.wrapping_add(1)
        }
        return status
    }
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
                                                                    as isize)
                    as *mut libc::c_void,
                (*h).linp.as_mut_ptr().offset(idx as libc::c_int as isize) as
                    *const libc::c_void,
                ((nxtindex as libc::c_int - idx as libc::c_int) as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<indx_t>()
                                                     as libc::c_ulong));
    }
    (*h).lower =
        ((*h).lower as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>() as
                                             libc::c_ulong) as indx_t as
            indx_t;
    (*h).upper =
        ((*h).upper as libc::c_uint).wrapping_sub(nbytes) as indx_t as indx_t;
    *(*h).linp.as_mut_ptr().offset(idx as isize) = (*h).upper;
    dest =
        (h as *mut libc::c_char).offset((*h).upper as libc::c_int as isize);
    *(dest as *mut libc::c_void as *mut u_int32_t) =
        (*data).size as u_int32_t;
    dest =
        dest.offset(::std::mem::size_of::<u_int32_t>() as libc::c_ulong as
                        isize);
    *(dest as *mut u_char) = dflags as u_char;
    dest =
        dest.offset(::std::mem::size_of::<u_char>() as libc::c_ulong as
                        isize);
    memmove(dest as *mut libc::c_void, (*data).data, (*data).size);
    (*t).bt_nrecs = (*t).bt_nrecs.wrapping_add(1);
    (*t).flags |= 0x4 as libc::c_int as libc::c_uint;
    kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                   0x1 as libc::c_int as u_int);
    return 0 as libc::c_int;
}
