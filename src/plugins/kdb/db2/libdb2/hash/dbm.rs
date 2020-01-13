use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:41"]
pub mod types_h {
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "34:1"]
    pub type __u_long = libc::c_ulong;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/sys/types.h:41"]
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
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:41"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:41"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/db.h:41"]
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
    /* Structure used to pass parameters to the hashing routines. */
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
    use super::stddef_h::size_t;
    use super::sys_types_h::{u_int, u_int32_t};
    /* !_DB_H_ */
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/include/db-int.h:41"]
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
    use super::db_h::{HASHINFO, DB};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "211:1"]
        pub fn __kdb2_hash_open(_: *const libc::c_char, _: libc::c_int,
                                _: libc::c_int, _: *const HASHINFO,
                                _: libc::c_int) -> *mut DB;
    }
    /* _DB_INT_H_ */
    /* Needed for Win32 compiles */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/include/db-ndbm.h:49"]
pub mod db_ndbm_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "57:9"]
    pub struct datum {
        pub dptr: *mut libc::c_char,
        pub dsize: libc::c_int,
    }
    #[c2rust::src_loc = "62:1"]
    pub type DBM = DB;
    use super::db_h::DB;
    /* !_NDBM_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/hash/hash.h:51"]
pub mod hash_h {
    #[c2rust::src_loc = "86:1"]
    pub type HTAB = htab;
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
    #[c2rust::src_loc = "48:1"]
    pub type CURSOR = cursor_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "48:16"]
    pub struct cursor_t {
        pub queue: C2RustUnnamed_1,
        pub get: Option<unsafe extern "C" fn(_: *const DB, _: *mut cursor_t,
                                             _: *mut DBT, _: *mut DBT,
                                             _: u_int32_t) -> libc::c_int>,
        pub kdb2_delete: Option<unsafe extern "C" fn(_: *const DB,
                                                     _: *mut cursor_t,
                                                     _: u_int32_t)
                                    -> libc::c_int>,
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
    #[c2rust::src_loc = "66:1"]
    pub type HASHHDR = hashhdr;
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
    /* address of overflow page bitmaps */
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
 *
 *	@(#)hash.h	8.4 (Berkeley) 11/2/95
 */
    /* Operations */
    /* cursor structure */
    /* Hash Table Information */
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "87:13"]
    pub struct _cursor_queue {
        pub tqh_first: *mut cursor_t,
        pub tqh_last: *mut *mut cursor_t,
    }
    use super::sys_types_h::{u_int32_t, u_int8_t, u_int16_t};
    use super::stddef_h::size_t;
    use super::stdint_intn_h::int32_t;
    use super::mpool_h::MPOOL;
    use super::db_h::{DB, DBT};
    use super::db_int_h::{db_pgno_t, indx_t};
    /* for num_items */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/mpool/mpool.h:51"]
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
    #[c2rust::src_loc = "62:13"]
    pub struct _lqh {
        pub tqh_first: *mut _bkt,
        pub tqh_last: *mut *mut _bkt,
    }
    use super::db_int_h::db_pgno_t;
    use super::sys_types_h::{u_long, u_int8_t};
}
#[c2rust::header_src = "/usr/include/stdio.h:41"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:47"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "124:14"]
        pub fn strncpy(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "132:14"]
        pub fn strncat(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
pub use self::types_h::{__u_int, __u_long, __uint8_t, __uint16_t, __int32_t,
                        __uint32_t, __off_t, __off64_t};
pub use self::sys_types_h::{u_int, u_long, u_int8_t, u_int16_t, u_int32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::db_h::{DBT, DBTYPE, DB_RECNO, DB_HASH, DB_BTREE, __db, DB,
                     HASHINFO};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::db_int_h::{db_pgno_t, indx_t, __kdb2_hash_open};
pub use self::db_ndbm_h::{datum, DBM};
pub use self::hash_h::{HTAB, htab, CURSOR, cursor_t, C2RustUnnamed_1, HASHHDR,
                       hashhdr, _cursor_queue};
pub use self::mpool_h::{MPOOL, _hqh, _bkt, C2RustUnnamed, C2RustUnnamed_0,
                        _lqh};
use self::stdio_h::{stderr, fprintf};
use self::string_h::{strncpy, strncat, strlen};
/*-
 * Copyright (c) 1990, 1993
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
/* If the two size fields of datum and DBMT are not equal, then
 * casting between structures will result in stack garbage being
 * transferred. Has been observed for DEC Alpha OSF, but will handle
 *  the general case.
 */
/*
 *
 * This package provides dbm and ndbm compatible interfaces to DB.
 * First are the DBM routines, which call the NDBM routines, and
 * the NDBM routines, which call the DB routines.
 */
#[c2rust::src_loc = "67:13"]
static mut __cur_db: *mut DBM = 0 as *const DBM as *mut DBM;
#[no_mangle]
#[c2rust::src_loc = "71:1"]
pub unsafe extern "C" fn kdb2_dbminit(mut file: *mut libc::c_char)
 -> libc::c_int {
    if !__cur_db.is_null() { kdb2_dbm_close(__cur_db); }
    __cur_db =
        kdb2_dbm_open(file, 0o2 as libc::c_int | 0 as libc::c_int,
                      0 as libc::c_int);
    if !__cur_db.is_null() { return 0 as libc::c_int }
    __cur_db =
        kdb2_dbm_open(file, 0 as libc::c_int | 0 as libc::c_int,
                      0 as libc::c_int);
    if !__cur_db.is_null() { return 0 as libc::c_int }
    return -(1 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "84:1"]
pub unsafe extern "C" fn kdb2_fetch(mut key: datum) -> datum {
    let mut item: datum = datum{dptr: 0 as *mut libc::c_char, dsize: 0,};
    if __cur_db.is_null() {
        no_open_db();
        item.dptr = 0 as *mut libc::c_char;
        item.dsize = 0 as libc::c_int;
        return item
    }
    return kdb2_dbm_fetch(__cur_db, key);
}
#[no_mangle]
#[c2rust::src_loc = "99:1"]
pub unsafe extern "C" fn kdb2_firstkey() -> datum {
    let mut item: datum = datum{dptr: 0 as *mut libc::c_char, dsize: 0,};
    if __cur_db.is_null() {
        no_open_db();
        item.dptr = 0 as *mut libc::c_char;
        item.dsize = 0 as libc::c_int;
        return item
    }
    return kdb2_dbm_firstkey(__cur_db);
}
#[no_mangle]
#[c2rust::src_loc = "113:1"]
pub unsafe extern "C" fn kdb2_nextkey(mut key: datum) -> datum {
    let mut item: datum = datum{dptr: 0 as *mut libc::c_char, dsize: 0,};
    if __cur_db.is_null() {
        no_open_db();
        item.dptr = 0 as *mut libc::c_char;
        item.dsize = 0 as libc::c_int;
        return item
    }
    return kdb2_dbm_nextkey(__cur_db);
}
#[no_mangle]
#[c2rust::src_loc = "128:1"]
pub unsafe extern "C" fn kdb2_delete(mut key: datum) -> libc::c_int {
    if __cur_db.is_null() { no_open_db(); return -(1 as libc::c_int) }
    return kdb2_dbm_delete(__cur_db, key);
}
#[no_mangle]
#[c2rust::src_loc = "139:1"]
pub unsafe extern "C" fn kdb2_store(mut key: datum, mut dat: datum)
 -> libc::c_int {
    if __cur_db.is_null() { no_open_db(); return -(1 as libc::c_int) }
    return kdb2_dbm_store(__cur_db, key, dat, 1 as libc::c_int);
}
#[c2rust::src_loc = "150:1"]
unsafe extern "C" fn no_open_db() {
    fprintf(stderr,
            b"dbm: no open database.\n\x00" as *const u8 as
                *const libc::c_char);
}
/*
 * Returns:
 * 	*DBM on success
 *	 NULL on failure
 */
#[no_mangle]
#[c2rust::src_loc = "161:1"]
pub unsafe extern "C" fn kdb2_dbm_open(mut file: *const libc::c_char,
                                       mut flags: libc::c_int,
                                       mut mode: libc::c_int) -> *mut DBM {
    let mut info: HASHINFO =
        HASHINFO{bsize: 0,
                 ffactor: 0,
                 nelem: 0,
                 cachesize: 0,
                 hash: None,
                 lorder: 0,};
    let mut path: [libc::c_char; 4096] = [0; 4096];
    info.bsize = 4096 as libc::c_int as u_int;
    info.ffactor = 40 as libc::c_int as u_int;
    info.nelem = 1 as libc::c_int as u_int;
    info.cachesize = 0 as libc::c_int as u_int;
    info.hash = None;
    info.lorder = 0 as libc::c_int;
    strncpy(path.as_mut_ptr(), file,
            (::std::mem::size_of::<[libc::c_char; 4096]>() as
                 libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                 libc::c_ulong));
    path[(::std::mem::size_of::<[libc::c_char; 4096]>() as
              libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
             as usize] = '\u{0}' as i32 as libc::c_char;
    strncat(path.as_mut_ptr(), b".db\x00" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 4096]>() as
                 libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                 libc::c_ulong).wrapping_sub(strlen(path.as_mut_ptr())));
    return __kdb2_hash_open(path.as_mut_ptr(), flags, mode, &mut info,
                            0 as libc::c_int) as *mut DBM;
}
/*
 * Returns:
 *	Nothing.
 */
#[no_mangle]
#[c2rust::src_loc = "185:1"]
pub unsafe extern "C" fn kdb2_dbm_close(mut db: *mut DBM) {
    (*db).close.expect("non-null function pointer")(db);
}
/*
 * Returns:
 *	DATUM on success
 *	NULL on failure
 */
#[no_mangle]
#[c2rust::src_loc = "197:1"]
pub unsafe extern "C" fn kdb2_dbm_fetch(mut db: *mut DBM, mut key: datum)
 -> datum {
    let mut retval: datum = datum{dptr: 0 as *mut libc::c_char, dsize: 0,};
    let mut status: libc::c_int = 0;
    let mut k: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    let mut r: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    k.data = key.dptr as *mut libc::c_void;
    k.size = key.dsize as size_t;
    status =
        (*db).get.expect("non-null function pointer")(db, &mut k, &mut r,
                                                      0 as libc::c_int as
                                                          u_int);
    retval.dptr = r.data as *mut libc::c_char;
    retval.dsize = r.size as libc::c_int;
    if status != 0 {
        retval.dptr = 0 as *mut libc::c_char;
        retval.dsize = 0 as libc::c_int
    }
    return retval;
}
/*
 * Returns:
 *	DATUM on success
 *	NULL on failure
 */
#[no_mangle]
#[c2rust::src_loc = "228:1"]
pub unsafe extern "C" fn kdb2_dbm_firstkey(mut db: *mut DBM) -> datum {
    let mut status: libc::c_int = 0;
    let mut retkey: datum = datum{dptr: 0 as *mut libc::c_char, dsize: 0,};
    let mut k: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    let mut r: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    status =
        (*db).seq.expect("non-null function pointer")(db, &mut k, &mut r,
                                                      3 as libc::c_int as
                                                          u_int);
    retkey.dptr = k.data as *mut libc::c_char;
    retkey.dsize = k.size as libc::c_int;
    if status != 0 { retkey.dptr = 0 as *mut libc::c_char }
    return retkey;
}
/*
 * Returns:
 *	DATUM on success
 *	NULL on failure
 */
#[no_mangle]
#[c2rust::src_loc = "256:1"]
pub unsafe extern "C" fn kdb2_dbm_nextkey(mut db: *mut DBM) -> datum {
    let mut status: libc::c_int = 0;
    let mut retkey: datum = datum{dptr: 0 as *mut libc::c_char, dsize: 0,};
    let mut k: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    let mut r: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    status =
        (*db).seq.expect("non-null function pointer")(db, &mut k, &mut r,
                                                      7 as libc::c_int as
                                                          u_int);
    retkey.dptr = k.data as *mut libc::c_char;
    retkey.dsize = k.size as libc::c_int;
    if status != 0 { retkey.dptr = 0 as *mut libc::c_char }
    return retkey;
}
/*
 * Returns:
 *	 0 on success
 *	<0 failure
 */
#[no_mangle]
#[c2rust::src_loc = "284:1"]
pub unsafe extern "C" fn kdb2_dbm_delete(mut db: *mut DBM, mut key: datum)
 -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut k: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    k.data = key.dptr as *mut libc::c_void;
    k.size = key.dsize as size_t;
    status =
        (*db).del.expect("non-null function pointer")(db, &mut k,
                                                      0 as libc::c_int as
                                                          u_int);
    if status != 0 {
        return -(1 as libc::c_int)
    } else { return 0 as libc::c_int };
}
/*
 * Returns:
 *	 0 on success
 *	<0 failure
 *	 1 if DBM_INSERT and entry exists
 */
#[no_mangle]
#[c2rust::src_loc = "312:1"]
pub unsafe extern "C" fn kdb2_dbm_store(mut db: *mut DBM, mut key: datum,
                                        mut content: datum,
                                        mut flags: libc::c_int)
 -> libc::c_int {
    let mut k: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    let mut c: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    k.data = key.dptr as *mut libc::c_void;
    k.size = key.dsize as size_t;
    c.data = content.dptr as *mut libc::c_void;
    c.size = content.dsize as size_t;
    return (*db).put.expect("non-null function pointer")(db, &mut k, &mut c,
                                                         if flags ==
                                                                0 as
                                                                    libc::c_int
                                                            {
                                                             8 as libc::c_int
                                                         } else {
                                                             0 as libc::c_int
                                                         } as u_int);
}
#[no_mangle]
#[c2rust::src_loc = "333:1"]
pub unsafe extern "C" fn kdb2_dbm_error(mut db: *mut DBM) -> libc::c_int {
    let mut hp: *mut HTAB = 0 as *mut HTAB;
    hp = (*db).internal as *mut HTAB;
    return (*hp).local_errno;
}
#[no_mangle]
#[c2rust::src_loc = "343:1"]
pub unsafe extern "C" fn kdb2_dbm_clearerr(mut db: *mut DBM) -> libc::c_int {
    let mut hp: *mut HTAB = 0 as *mut HTAB;
    hp = (*db).internal as *mut HTAB;
    (*hp).local_errno = 0 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "354:1"]
pub unsafe extern "C" fn kdb2_dbm_dirfno(mut db: *mut DBM) -> libc::c_int {
    return (*((*db).internal as *mut HTAB)).fp;
}
