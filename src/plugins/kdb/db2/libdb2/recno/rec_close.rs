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
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
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
    #[c2rust::src_loc = "85:1"]
    pub type off_t = __off_t;
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    #[c2rust::src_loc = "158:1"]
    pub type u_int8_t = __uint8_t;
    #[c2rust::src_loc = "159:1"]
    pub type u_int16_t = __uint16_t;
    #[c2rust::src_loc = "160:1"]
    pub type u_int32_t = __uint32_t;
    use super::types_h::{__u_char, __u_int, __u_long, __off_t, __ssize_t,
                         __caddr_t, __uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:38"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_iovec.h:39"]
pub mod struct_iovec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "26:8"]
    pub struct iovec {
        pub iov_base: *mut libc::c_void,
        pub iov_len: size_t,
    }
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:46"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:46"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/db.h:49"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/include/db-int.h:49"]
pub mod db_int_h {
    #[c2rust::src_loc = "143:1"]
    pub type db_pgno_t = u_int32_t;
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
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/mpool/mpool.h:50"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/btree/btree.h:50"]
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
    #[c2rust::src_loc = "241:1"]
    pub type EPGNO = _epgno;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "246:16"]
    pub struct _epg {
        pub page: *mut PAGE,
        pub index: indx_t,
    }
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
    #[c2rust::src_loc = "304:1"]
    pub type BTREE = _btree;
    use super::db_int_h::{db_pgno_t, indx_t, recno_t};
    use super::sys_types_h::{u_int32_t, u_int8_t, caddr_t, u_char};
    use super::db_h::{DBT, DB};
    use super::mpool_h::MPOOL;
    use super::stddef_h::size_t;
    use super::FILE_h::FILE;
}
#[c2rust::header_src = "/usr/include/sys/uio.h:39"]
pub mod uio_h {
    use super::struct_iovec_h::iovec;
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "52:1"]
        pub fn writev(__fd: libc::c_int, __iovec: *const iovec,
                      __count: libc::c_int) -> ssize_t;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:46"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:47"]
pub mod unistd_h {
    use super::types_h::__off_t;
    use super::stddef_h::size_t;
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "334:1"]
        pub fn lseek(__fd: libc::c_int, __offset: __off_t,
                     __whence: libc::c_int) -> __off_t;
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "366:1"]
        pub fn write(__fd: libc::c_int, __buf: *const libc::c_void,
                     __n: size_t) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "1014:1"]
        pub fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/btree/extern.h:50"]
pub mod extern_h {
    use super::db_h::DB;
    use super::sys_types_h::u_int;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn __kdb2_bt_close(_: *mut DB) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "88:1"]
        pub fn __kdb2_bt_sync(_: *const DB, _: u_int) -> libc::c_int;
    }
}
pub use self::types_h::{__u_char, __u_int, __u_long, __uint8_t, __uint16_t,
                        __uint32_t, __off_t, __off64_t, __ssize_t, __caddr_t};
pub use self::sys_types_h::{u_char, u_int, u_long, off_t, ssize_t, caddr_t,
                            u_int8_t, u_int16_t, u_int32_t};
pub use self::stddef_h::size_t;
pub use self::struct_iovec_h::iovec;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::db_h::{DBT, DBTYPE, DB_RECNO, DB_HASH, DB_BTREE, __db, DB};
pub use self::db_int_h::{db_pgno_t, indx_t, recno_t};
pub use self::mpool_h::{_bkt, C2RustUnnamed, C2RustUnnamed_0, MPOOL, _hqh,
                        _lqh, kdb2_mpool_put};
pub use self::btree_h::{_page, PAGE, _epgno, EPGNO, _epg, EPG, _cursor,
                        CURSOR, _btree, C2RustUnnamed_1, FORWARD, BACK, NOT,
                        BTREE};
use self::uio_h::writev;
use self::stdio_h::fclose;
use self::unistd_h::{lseek, close, write, ftruncate};
use self::extern_h::{__kdb2_bt_close, __kdb2_bt_sync};
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
 * __REC_CLOSE -- Close a recno tree.
 *
 * Parameters:
 *	dbp:	pointer to access method
 *
 * Returns:
 *	RET_ERROR, RET_SUCCESS
 */
#[no_mangle]
#[c2rust::src_loc = "61:1"]
pub unsafe extern "C" fn __kdb2_rec_close(mut dbp: *mut DB) -> libc::c_int {
    let mut t: *mut BTREE = 0 as *mut BTREE;
    let mut status: libc::c_int = 0;
    t = (*dbp).internal as *mut BTREE;
    /* Toss any page pinned across calls. */
    if !(*t).bt_pinned.is_null() {
        kdb2_mpool_put((*t).bt_mp, (*t).bt_pinned as *mut libc::c_void,
                       0 as libc::c_int as u_int);
        (*t).bt_pinned = 0 as *mut PAGE
    }
    if __kdb2_rec_sync(dbp, 0 as libc::c_int as u_int) == -(1 as libc::c_int)
       {
        return -(1 as libc::c_int)
    }
    /* Committed to closing. */
    status = 0 as libc::c_int;
    if (*t).flags & 0x800 as libc::c_int as libc::c_uint == 0 {
        if (*t).flags & 0x40 as libc::c_int as libc::c_uint != 0 {
            if fclose((*t).bt_rfp) != 0 { status = -(1 as libc::c_int) }
        } else if close((*t).bt_rfd) != 0 { status = -(1 as libc::c_int) }
    }
    if __kdb2_bt_close(dbp) == -(1 as libc::c_int) {
        status = -(1 as libc::c_int)
    }
    return status;
}
/*
 * __REC_SYNC -- sync the recno tree to disk.
 *
 * Parameters:
 *	dbp:	pointer to access method
 *
 * Returns:
 *	RET_SUCCESS, RET_ERROR.
 */
#[no_mangle]
#[c2rust::src_loc = "110:1"]
pub unsafe extern "C" fn __kdb2_rec_sync(mut dbp: *const DB, mut flags: u_int)
 -> libc::c_int {
    let mut iov: [iovec; 2] =
        [iovec{iov_base: 0 as *mut libc::c_void, iov_len: 0,}; 2];
    let mut t: *mut BTREE = 0 as *mut BTREE;
    let mut data: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    let mut key: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    let mut off: off_t = 0;
    let mut scursor: recno_t = 0;
    let mut trec: recno_t = 0;
    let mut status: libc::c_int = 0;
    t = (*dbp).internal as *mut BTREE;
    /* Toss any page pinned across calls. */
    if !(*t).bt_pinned.is_null() {
        kdb2_mpool_put((*t).bt_mp, (*t).bt_pinned as *mut libc::c_void,
                       0 as libc::c_int as u_int);
        (*t).bt_pinned = 0 as *mut PAGE
    }
    if flags == 11 as libc::c_int as libc::c_uint {
        return __kdb2_bt_sync(dbp, 0 as libc::c_int as u_int)
    }
    if (*t).flags &
           (0x2000 as libc::c_int | 0x800 as libc::c_int) as libc::c_uint != 0
           || (*t).flags & 0x1000 as libc::c_int as libc::c_uint == 0 {
        return 0 as libc::c_int
    }
    /* Read any remaining records into the tree. */
    if (*t).flags & 0x100 as libc::c_int as libc::c_uint == 0 &&
           (*t).bt_irec.expect("non-null function pointer")(t,
                                                            0xffffffff as
                                                                libc::c_uint)
               == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    }
    /* Rewind the file descriptor. */
    if lseek((*t).bt_rfd, 0 as libc::c_int as off_t, 0 as libc::c_int) !=
           0 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int)
    }
    /* Save the cursor. */
    scursor = (*t).bt_cursor.rcursor;
    key.size = ::std::mem::size_of::<recno_t>() as libc::c_ulong;
    key.data = &mut trec as *mut recno_t as *mut libc::c_void;
    if (*t).flags & 0x200 as libc::c_int as libc::c_uint != 0 {
        /*
		 * We assume that fixed length records are all fixed length.
		 * Any that aren't are either EINVAL'd or corrected by the
		 * record put code.
		 */
        status =
            (*dbp).seq.expect("non-null function pointer")(dbp, &mut key,
                                                           &mut data,
                                                           3 as libc::c_int as
                                                               u_int);
        while status == 0 as libc::c_int {
            if write((*t).bt_rfd, data.data, data.size) !=
                   data.size as ssize_t {
                return -(1 as libc::c_int)
            }
            status =
                (*dbp).seq.expect("non-null function pointer")(dbp, &mut key,
                                                               &mut data,
                                                               7 as
                                                                   libc::c_int
                                                                   as u_int)
        }
    } else {
        iov[1 as libc::c_int as usize].iov_base =
            &mut (*t).bt_bval as *mut u_char as *mut libc::c_void;
        iov[1 as libc::c_int as usize].iov_len = 1 as libc::c_int as size_t;
        status =
            (*dbp).seq.expect("non-null function pointer")(dbp, &mut key,
                                                           &mut data,
                                                           3 as libc::c_int as
                                                               u_int);
        while status == 0 as libc::c_int {
            iov[0 as libc::c_int as usize].iov_base = data.data;
            iov[0 as libc::c_int as usize].iov_len = data.size;
            if writev((*t).bt_rfd, iov.as_mut_ptr(), 2 as libc::c_int) !=
                   data.size as ssize_t + 1 as libc::c_int as libc::c_long {
                return -(1 as libc::c_int)
            }
            status =
                (*dbp).seq.expect("non-null function pointer")(dbp, &mut key,
                                                               &mut data,
                                                               7 as
                                                                   libc::c_int
                                                                   as u_int)
        }
    }
    /* Restore the cursor. */
    (*t).bt_cursor.rcursor = scursor;
    if status == -(1 as libc::c_int) { return -(1 as libc::c_int) }
    off = lseek((*t).bt_rfd, 0 as libc::c_int as off_t, 1 as libc::c_int);
    if off == -(1 as libc::c_int) as libc::c_long {
        return -(1 as libc::c_int)
    }
    if ftruncate((*t).bt_rfd, off) != 0 { return -(1 as libc::c_int) }
    (*t).flags &= !(0x1000 as libc::c_int) as libc::c_uint;
    return 0 as libc::c_int;
}
