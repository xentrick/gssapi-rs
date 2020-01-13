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
    #[c2rust::src_loc = "143:1"]
    pub type db_pgno_t = u_int32_t;
    #[c2rust::src_loc = "145:1"]
    pub type indx_t = u_int16_t;
    #[c2rust::src_loc = "147:1"]
    pub type recno_t = u_int32_t;
    use super::sys_types_h::{u_int32_t, u_int16_t};
    use super::db_h::DB;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn __kdb2_dbpanic(dbp: *mut DB);
    }
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
        #[no_mangle]
        #[c2rust::src_loc = "109:1"]
        pub fn kdb2_mpool_get(_: *mut MPOOL, _: db_pgno_t, _: u_int)
         -> *mut libc::c_void;
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
    #[c2rust::src_loc = "151:16"]
    pub struct _rinternal {
        pub nrecs: recno_t,
        pub pgno: db_pgno_t,
    }
    #[c2rust::src_loc = "151:1"]
    pub type RINTERNAL = _rinternal;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "205:16"]
    pub struct _rleaf {
        pub dsize: u_int32_t,
        pub flags: u_char,
        pub bytes: [libc::c_char; 1],
    }
    #[c2rust::src_loc = "205:1"]
    pub type RLEAF = _rleaf;
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
    /* The in-memory btree/recno data structure. */
    #[c2rust::src_loc = "304:1"]
    pub type BTREE = _btree;
    use super::db_int_h::{db_pgno_t, indx_t, recno_t};
    use super::sys_types_h::{u_int32_t, u_char, u_int8_t, caddr_t};
    use super::db_h::{DBT, DB};
    use super::mpool_h::MPOOL;
    use super::stddef_h::size_t;
    use super::FILE_h::FILE;
}
#[c2rust::header_src = "/usr/include/stdlib.h:45"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
    }
}
#[c2rust::header_src = "/usr/include/string.h:46"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/btree/extern.h:49"]
pub mod extern_h {
    use super::btree_h::{BTREE, PAGE};
    use super::db_int_h::db_pgno_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn __kdb2_bt_new(_: *mut BTREE, _: *mut db_pgno_t) -> *mut PAGE;
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
pub use self::db_int_h::{db_pgno_t, indx_t, recno_t, __kdb2_dbpanic};
pub use self::mpool_h::{_bkt, C2RustUnnamed, C2RustUnnamed_0, MPOOL, _hqh,
                        _lqh, kdb2_mpool_put, kdb2_mpool_get};
pub use self::btree_h::{_page, PAGE, _binternal, BINTERNAL, _rinternal,
                        RINTERNAL, _bleaf, BLEAF, _rleaf, RLEAF, _epgno,
                        EPGNO, _epg, EPG, _cursor, CURSOR, _btree,
                        C2RustUnnamed_1, FORWARD, BACK, NOT, BTREE};
use self::stdlib_h::{malloc, free, abort};
use self::string_h::{memcpy, memmove, memset};
use self::extern_h::__kdb2_bt_new;
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
/*
 * __BT_SPLIT -- Split the tree.
 *
 * Parameters:
 *	t:	tree
 *	sp:	page to split
 *	key:	key to insert
 *	data:	data to insert
 *	flags:	BIGKEY/BIGDATA flags
 *	ilen:	insert length
 *	skip:	index to leave open
 *
 * Returns:
 *	RET_ERROR, RET_SUCCESS
 */
#[no_mangle]
#[c2rust::src_loc = "81:1"]
pub unsafe extern "C" fn __kdb2_bt_split(mut t: *mut BTREE, mut sp: *mut PAGE,
                                         mut key: *const DBT,
                                         mut data: *const DBT,
                                         mut flags: libc::c_int,
                                         mut ilen: size_t,
                                         mut argskip: u_int32_t)
 -> libc::c_int {
    let mut current_block: u64;
    let mut bi: *mut BINTERNAL = 0 as *mut BINTERNAL;
    let mut bl: *mut BLEAF = 0 as *mut BLEAF;
    let mut tbl: *mut BLEAF = 0 as *mut BLEAF;
    let mut a: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    let mut b: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    let mut parent: *mut EPGNO = 0 as *mut EPGNO;
    let mut h: *mut PAGE = 0 as *mut PAGE;
    let mut l: *mut PAGE = 0 as *mut PAGE;
    let mut r: *mut PAGE = 0 as *mut PAGE;
    let mut lchild: *mut PAGE = 0 as *mut PAGE;
    let mut rchild: *mut PAGE = 0 as *mut PAGE;
    let mut nxtindex: indx_t = 0;
    let mut skip: u_int16_t = 0;
    let mut n: u_int32_t = 0;
    let mut nbytes: u_int32_t = 0;
    let mut nksize: u_int32_t = 0 as libc::c_int as u_int32_t;
    let mut parentsplit: libc::c_int = 0;
    let mut dest: *mut libc::c_char = 0 as *mut libc::c_char;
    /*
	 * Split the page into two pages, l and r.  The split routines return
	 * a pointer to the page into which the key should be inserted and with
	 * skip set to the offset which should be used.  Additionally, l and r
	 * are pinned.
	 */
    skip = argskip as u_int16_t;
    h =
        if (*sp).pgno == 1 as libc::c_int as libc::c_uint {
            bt_root(t, sp, &mut l, &mut r, &mut skip, ilen)
        } else { bt_page(t, sp, &mut l, &mut r, &mut skip, ilen) };
    if h.is_null() { return -(1 as libc::c_int) }
    /*
	 * Insert the new key/data pair into the leaf page.  (Key inserts
	 * always cause a leaf page to split first.)
	 */
    (*h).upper =
        ((*h).upper as libc::c_ulong).wrapping_sub(ilen) as indx_t as indx_t;
    *(*h).linp.as_mut_ptr().offset(skip as isize) = (*h).upper;
    dest =
        (h as *mut libc::c_char).offset((*h).upper as libc::c_int as isize);
    if (*t).flags & 0x80 as libc::c_int as libc::c_uint != 0 {
        *(dest as *mut libc::c_void as *mut u_int32_t) =
            (*data).size as u_int32_t;
        dest =
            dest.offset(::std::mem::size_of::<u_int32_t>() as libc::c_ulong as
                            isize);
        *(dest as *mut u_char) = flags as u_char;
        dest =
            dest.offset(::std::mem::size_of::<u_char>() as libc::c_ulong as
                            isize);
        memmove(dest as *mut libc::c_void, (*data).data, (*data).size);
    } else {
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
        *(dest as *mut u_char) = flags as u_char;
        dest =
            dest.offset(::std::mem::size_of::<u_char>() as libc::c_ulong as
                            isize);
        memmove(dest as *mut libc::c_void, (*key).data, (*key).size);
        dest = dest.offset((*key).size as isize);
        memmove(dest as *mut libc::c_void, (*data).data, (*data).size);
    }
    /* If the root page was split, make it look right. */
    if (*sp).pgno == 1 as libc::c_int as libc::c_uint &&
           (if (*t).flags & 0x80 as libc::c_int as libc::c_uint != 0 {
                bt_rroot(t, sp, l, r)
            } else { bt_broot(t, sp, l, r) }) == -(1 as libc::c_int) {
        current_block = 12387293126886848581;
    } else { current_block = 4068382217303356765; }
    loop  {
        match current_block {
            4068382217303356765 =>
            /*
	 * Now we walk the parent page stack -- a LIFO stack of the pages that
	 * were traversed when we searched for the page that split.  Each stack
	 * entry is a page number and a page index offset.  The offset is for
	 * the page traversed on the search.  We've just split a page, so we
	 * have to insert a new key into the parent page.
	 *
	 * If the insert into the parent page causes it to split, may have to
	 * continue splitting all the way up the tree.  We stop if the root
	 * splits or the page inserted into didn't have to split to hold the
	 * new key.  Some algorithms replace the key for the old page as well
	 * as the new page.  We don't, as there's no reason to believe that the
	 * first key on the old page is any better than the key we have, and,
	 * in the case of a key being placed at index 0 causing the split, the
	 * key is unavailable.
	 *
	 * There are a maximum of 5 pages pinned at any time.  We keep the left
	 * and right pages pinned while working on the parent.   The 5 are the
	 * two children, left parent and right parent (when the parent splits)
	 * and the root page or the overflow key page when calling bt_preserve.
	 * This code must make sure that all pins are released other than the
	 * root page or overflow page which is unlocked elsewhere.
	 */
            {
                parent =
                    if (*t).bt_sp == (*t).bt_stack.as_mut_ptr() {
                        0 as *mut EPGNO
                    } else { (*t).bt_sp = (*t).bt_sp.offset(-1); (*t).bt_sp };
                if parent.is_null() { break ; }
                lchild = l;
                rchild = r;
                /* Get the parent page. */
                h =
                    kdb2_mpool_get((*t).bt_mp, (*parent).pgno,
                                   0 as libc::c_int as u_int) as *mut PAGE;
                if h.is_null() {
                    current_block = 12387293126886848581;
                    continue ;
                }
                /*
		 * The new key goes ONE AFTER the index, because the split
		 * was to the right.
		 */
                skip =
                    ((*parent).index as libc::c_int + 1 as libc::c_int) as
                        u_int16_t;
                /*
		 * Calculate the space needed on the parent page.
		 *
		 * Prefix trees: space hack when inserting into BINTERNAL
		 * pages.  Retain only what's needed to distinguish between
		 * the new entry and the LAST entry on the page to its left.
		 * If the keys compare equal, retain the entire key.  Note,
		 * we don't touch overflow keys, and the entire key must be
		 * retained for the next-to-left most key on the leftmost
		 * page of each level, or the search will fail.  Applicable
		 * ONLY to internal pages that have leaf pages as children.
		 * Further reduction of the key between pairs of internal
		 * pages loses too much information.
		 */
                match (*rchild).flags & 0x1f as libc::c_int as libc::c_uint {
                    1 => {
                        bi =
                            (rchild as
                                 *mut libc::c_char).offset(*(*rchild).linp.as_mut_ptr().offset(0
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   isize)
                                                               as libc::c_int
                                                               as isize) as
                                *mut libc::c_void as *mut BINTERNAL;
                        nbytes =
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
                                       libc::c_ulong).wrapping_sub(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulong))
                                as u_int32_t
                    }
                    2 => {
                        bl =
                            (rchild as
                                 *mut libc::c_char).offset(*(*rchild).linp.as_mut_ptr().offset(0
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   isize)
                                                               as libc::c_int
                                                               as isize) as
                                *mut libc::c_void as *mut BLEAF;
                        nbytes =
                            ((::std::mem::size_of::<u_int32_t>() as
                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                  as
                                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<u_char>()
                                                                                                  as
                                                                                                  libc::c_ulong).wrapping_add((*bl).ksize
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
                                       libc::c_ulong).wrapping_sub(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulong))
                                as u_int32_t;
                        if (*t).bt_pfx.is_some() &&
                               (*bl).flags as libc::c_int & 0x2 as libc::c_int
                                   == 0 &&
                               ((*h).prevpg !=
                                    0 as libc::c_int as libc::c_uint ||
                                    skip as libc::c_int > 1 as libc::c_int) {
                            tbl =
                                (lchild as
                                     *mut libc::c_char).offset(*(*lchild).linp.as_mut_ptr().offset(((*lchild).lower
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
                                                                   as
                                                                   libc::c_int
                                                                   as isize)
                                    as *mut libc::c_void as *mut BLEAF;
                            a.size = (*tbl).ksize as size_t;
                            a.data =
                                (*tbl).bytes.as_mut_ptr() as
                                    *mut libc::c_void;
                            b.size = (*bl).ksize as size_t;
                            b.data =
                                (*bl).bytes.as_mut_ptr() as *mut libc::c_void;
                            nksize =
                                (*t).bt_pfx.expect("non-null function pointer")(&mut a,
                                                                                &mut b)
                                    as u_int32_t;
                            n =
                                ((::std::mem::size_of::<u_int32_t>() as
                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                      as
                                                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<u_char>()
                                                                                                      as
                                                                                                      libc::c_ulong).wrapping_add(nksize
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
                                           libc::c_ulong).wrapping_sub(1 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ulong))
                                    as u_int32_t;
                            if n < nbytes {
                                nbytes = n
                            } else { nksize = 0 as libc::c_int as u_int32_t }
                        } else { nksize = 0 as libc::c_int as u_int32_t }
                    }
                    8 | 16 => {
                        nbytes =
                            ((::std::mem::size_of::<recno_t>() as
                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
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
                                       libc::c_ulong).wrapping_sub(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulong))
                                as u_int32_t
                    }
                    _ => { abort(); }
                }
                /* Split the parent page if necessary or shift the indices. */
                if (((*h).upper as
                         u_int32_t).wrapping_sub((*h).lower as u_int32_t) as
                        libc::c_ulong) <
                       (nbytes as
                            libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                            as libc::c_ulong)
                   {
                    sp = h;
                    h =
                        if (*h).pgno == 1 as libc::c_int as libc::c_uint {
                            bt_root(t, h, &mut l, &mut r, &mut skip,
                                    nbytes as size_t)
                        } else {
                            bt_page(t, h, &mut l, &mut r, &mut skip,
                                    nbytes as size_t)
                        };
                    if h.is_null() {
                        current_block = 7932438647009970261;
                    } else {
                        parentsplit = 1 as libc::c_int;
                        current_block = 16203797167131938757;
                    }
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
                    if (skip as libc::c_int) < nxtindex as libc::c_int {
                        memmove((*h).linp.as_mut_ptr().offset(skip as
                                                                  libc::c_int
                                                                  as
                                                                  isize).offset(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize)
                                    as *mut libc::c_void,
                                (*h).linp.as_mut_ptr().offset(skip as
                                                                  libc::c_int
                                                                  as isize) as
                                    *const libc::c_void,
                                ((nxtindex as libc::c_int -
                                      skip as libc::c_int) as
                                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<indx_t>()
                                                                     as
                                                                     libc::c_ulong));
                    }
                    (*h).lower =
                        ((*h).lower as
                             libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                             as libc::c_ulong)
                            as indx_t as indx_t;
                    parentsplit = 0 as libc::c_int;
                    current_block = 16203797167131938757;
                }
                match current_block {
                    16203797167131938757 =>
                    /* Insert the key into the parent page. */
                    {
                        match (*rchild).flags &
                                  0x1f as libc::c_int as libc::c_uint {
                            1 => {
                                (*h).upper =
                                    ((*h).upper as
                                         libc::c_uint).wrapping_sub(nbytes) as
                                        indx_t as indx_t;
                                *(*h).linp.as_mut_ptr().offset(skip as isize)
                                    = (*h).upper;
                                dest =
                                    (h as
                                         *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(skip
                                                                                                      as
                                                                                                      isize)
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       isize);
                                memmove(dest as *mut libc::c_void,
                                        bi as *const libc::c_void,
                                        nbytes as libc::c_ulong);
                                (*(dest as *mut libc::c_void as
                                       *mut BINTERNAL)).pgno = (*rchild).pgno;
                                current_block = 12369290732426379360;
                            }
                            2 => {
                                (*h).upper =
                                    ((*h).upper as
                                         libc::c_uint).wrapping_sub(nbytes) as
                                        indx_t as indx_t;
                                *(*h).linp.as_mut_ptr().offset(skip as isize)
                                    = (*h).upper;
                                dest =
                                    (h as
                                         *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(skip
                                                                                                      as
                                                                                                      isize)
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       isize);
                                *(dest as *mut libc::c_void as *mut u_int32_t)
                                    =
                                    if nksize != 0 {
                                        nksize
                                    } else { (*bl).ksize };
                                dest =
                                    dest.offset(::std::mem::size_of::<u_int32_t>()
                                                    as libc::c_ulong as
                                                    isize);
                                *(dest as *mut libc::c_void as *mut db_pgno_t)
                                    = (*rchild).pgno;
                                dest =
                                    dest.offset(::std::mem::size_of::<db_pgno_t>()
                                                    as libc::c_ulong as
                                                    isize);
                                *(dest as *mut u_char) =
                                    ((*bl).flags as libc::c_int &
                                         0x2 as libc::c_int) as u_char;
                                dest =
                                    dest.offset(::std::mem::size_of::<u_char>()
                                                    as libc::c_ulong as
                                                    isize);
                                memmove(dest as *mut libc::c_void,
                                        (*bl).bytes.as_mut_ptr() as
                                            *const libc::c_void,
                                        if nksize != 0 {
                                            nksize
                                        } else { (*bl).ksize } as
                                            libc::c_ulong);
                                if (*bl).flags as libc::c_int &
                                       0x2 as libc::c_int != 0 {
                                    let mut pgno: db_pgno_t = 0;
                                    memcpy(&mut pgno as *mut db_pgno_t as
                                               *mut libc::c_void,
                                           (*bl).bytes.as_mut_ptr() as
                                               *const libc::c_void,
                                           ::std::mem::size_of::<db_pgno_t>()
                                               as libc::c_ulong);
                                    if bt_preserve(t, pgno) ==
                                           -(1 as libc::c_int) {
                                        current_block = 7932438647009970261;
                                    } else {
                                        current_block = 12369290732426379360;
                                    }
                                } else {
                                    current_block = 12369290732426379360;
                                }
                            }
                            8 => {
                                /*
			 * Update the left page count.  If split
			 * added at index 0, fix the correct page.
			 */
                                if skip as libc::c_int > 0 as libc::c_int {
                                    dest =
                                        (h as
                                             *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset((skip
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           -
                                                                                                           1
                                                                                                               as
                                                                                                               libc::c_int)
                                                                                                          as
                                                                                                          isize)
                                                                           as
                                                                           libc::c_int
                                                                           as
                                                                           isize)
                                } else {
                                    dest =
                                        (l as
                                             *mut libc::c_char).offset(*(*l).linp.as_mut_ptr().offset(((*l).lower
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
                                                                           as
                                                                           libc::c_int
                                                                           as
                                                                           isize)
                                }
                                (*(dest as *mut libc::c_void as
                                       *mut RINTERNAL)).nrecs =
                                    rec_total(lchild);
                                (*(dest as *mut libc::c_void as
                                       *mut RINTERNAL)).pgno = (*lchild).pgno;
                                /* Update the right page count. */
                                (*h).upper =
                                    ((*h).upper as
                                         libc::c_uint).wrapping_sub(nbytes) as
                                        indx_t as indx_t;
                                *(*h).linp.as_mut_ptr().offset(skip as isize)
                                    = (*h).upper;
                                dest =
                                    (h as
                                         *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(skip
                                                                                                      as
                                                                                                      isize)
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       isize);
                                (*(dest as *mut libc::c_void as
                                       *mut RINTERNAL)).nrecs =
                                    rec_total(rchild);
                                (*(dest as *mut libc::c_void as
                                       *mut RINTERNAL)).pgno = (*rchild).pgno;
                                current_block = 12369290732426379360;
                            }
                            16 => {
                                /*
			 * Update the left page count.  If split
			 * added at index 0, fix the correct page.
			 */
                                if skip as libc::c_int > 0 as libc::c_int {
                                    dest =
                                        (h as
                                             *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset((skip
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           -
                                                                                                           1
                                                                                                               as
                                                                                                               libc::c_int)
                                                                                                          as
                                                                                                          isize)
                                                                           as
                                                                           libc::c_int
                                                                           as
                                                                           isize)
                                } else {
                                    dest =
                                        (l as
                                             *mut libc::c_char).offset(*(*l).linp.as_mut_ptr().offset(((*l).lower
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
                                                                           as
                                                                           libc::c_int
                                                                           as
                                                                           isize)
                                }
                                (*(dest as *mut libc::c_void as
                                       *mut RINTERNAL)).nrecs =
                                    ((*lchild).lower as
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
                                        as recno_t;
                                (*(dest as *mut libc::c_void as
                                       *mut RINTERNAL)).pgno = (*lchild).pgno;
                                /* Update the right page count. */
                                (*h).upper =
                                    ((*h).upper as
                                         libc::c_uint).wrapping_sub(nbytes) as
                                        indx_t as indx_t;
                                *(*h).linp.as_mut_ptr().offset(skip as isize)
                                    = (*h).upper;
                                dest =
                                    (h as
                                         *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(skip
                                                                                                      as
                                                                                                      isize)
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       isize);
                                (*(dest as *mut libc::c_void as
                                       *mut RINTERNAL)).nrecs =
                                    ((*rchild).lower as
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
                                        as recno_t;
                                (*(dest as *mut libc::c_void as
                                       *mut RINTERNAL)).pgno = (*rchild).pgno;
                                current_block = 12369290732426379360;
                            }
                            _ => { abort(); }
                        }
                        match current_block {
                            7932438647009970261 => { }
                            _ =>
                            /* Unpin the held pages. */
                            {
                                if parentsplit == 0 {
                                    kdb2_mpool_put((*t).bt_mp,
                                                   h as *mut libc::c_void,
                                                   0x1 as libc::c_int as
                                                       u_int);
                                    break ;
                                } else if !((*sp).pgno ==
                                                1 as libc::c_int as
                                                    libc::c_uint &&
                                                (if (*t).flags &
                                                        0x80 as libc::c_int as
                                                            libc::c_uint != 0
                                                    {
                                                     bt_rroot(t, sp, l, r)
                                                 } else {
                                                     bt_broot(t, sp, l, r)
                                                 }) == -(1 as libc::c_int)) {
                                    kdb2_mpool_put((*t).bt_mp,
                                                   lchild as
                                                       *mut libc::c_void,
                                                   0x1 as libc::c_int as
                                                       u_int);
                                    kdb2_mpool_put((*t).bt_mp,
                                                   rchild as
                                                       *mut libc::c_void,
                                                   0x1 as libc::c_int as
                                                       u_int);
                                    current_block = 4068382217303356765;
                                    continue ;
                                }
                            }
                        }
                    }
                    _ => { }
                }
                /* If the root page was split, make it look right. */
                /*
	 * If something fails in the above loop we were already walking back
	 * up the tree and the tree is now inconsistent.  Nothing much we can
	 * do about it but release any memory we're holding.
	 */
                kdb2_mpool_put((*t).bt_mp, lchild as *mut libc::c_void,
                               0x1 as libc::c_int as u_int);
                kdb2_mpool_put((*t).bt_mp, rchild as *mut libc::c_void,
                               0x1 as libc::c_int as u_int);
                current_block = 12387293126886848581;
            }
            _ => {
                kdb2_mpool_put((*t).bt_mp, l as *mut libc::c_void,
                               0 as libc::c_int as u_int);
                kdb2_mpool_put((*t).bt_mp, r as *mut libc::c_void,
                               0 as libc::c_int as u_int);
                __kdb2_dbpanic((*t).bt_dbp);
                return -(1 as libc::c_int)
            }
        }
    }
    /* Unpin the held pages. */
    kdb2_mpool_put((*t).bt_mp, l as *mut libc::c_void,
                   0x1 as libc::c_int as u_int);
    kdb2_mpool_put((*t).bt_mp, r as *mut libc::c_void,
                   0x1 as libc::c_int as u_int);
    /* Clear any pages left on the stack. */
    return 0 as libc::c_int;
}
/*
 * BT_PAGE -- Split a non-root page of a btree.
 *
 * Parameters:
 *	t:	tree
 *	h:	root page
 *	lp:	pointer to left page pointer
 *	rp:	pointer to right page pointer
 *	skip:	pointer to index to leave open
 *	ilen:	insert length
 *
 * Returns:
 *	Pointer to page in which to insert or NULL on error.
 */
#[c2rust::src_loc = "347:1"]
unsafe extern "C" fn bt_page(mut t: *mut BTREE, mut h: *mut PAGE,
                             mut lp: *mut *mut PAGE, mut rp: *mut *mut PAGE,
                             mut skip: *mut indx_t, mut ilen: size_t)
 -> *mut PAGE {
    let mut l: *mut PAGE = 0 as *mut PAGE;
    let mut r: *mut PAGE = 0 as *mut PAGE;
    let mut tp: *mut PAGE = 0 as *mut PAGE;
    let mut npg: db_pgno_t = 0;
    /* Put the new right page for the split into place. */
    r = __kdb2_bt_new(t, &mut npg);
    if r.is_null() { return 0 as *mut PAGE }
    (*r).pgno = npg;
    (*r).lower =
        (::std::mem::size_of::<db_pgno_t>() as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>() as
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
    (*r).upper = (*t).bt_psize as indx_t;
    (*r).nextpg = (*h).nextpg;
    (*r).prevpg = (*h).pgno;
    (*r).flags = (*h).flags & 0x1f as libc::c_int as libc::c_uint;
    /*
	 * If we're splitting the last page on a level because we're appending
	 * a key to it (skip is NEXTINDEX()), it's likely that the data is
	 * sorted.  Adding an empty page on the side of the level is less work
	 * and can push the fill factor much higher than normal.  If we're
	 * wrong it's no big deal, we'll just do the split the right way next
	 * time.  It may look like it's equally easy to do a similar hack for
	 * reverse sorted data, that is, split the tree left, but it's not.
	 * Don't even try.
	 */
    if (*h).nextpg == 0 as libc::c_int as libc::c_uint &&
           *skip as libc::c_ulong ==
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
        (*h).nextpg = (*r).pgno;
        (*r).lower =
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
                                                                                                                                                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                 as
                                                                                                                                                                                                                 libc::c_ulong)
                as indx_t;
        *skip = 0 as libc::c_int as indx_t;
        *lp = h;
        *rp = r;
        return r
    }
    /* Put the new left page for the split into place. */
    l = malloc((*t).bt_psize as libc::c_ulong) as *mut PAGE;
    if l.is_null() {
        kdb2_mpool_put((*t).bt_mp, r as *mut libc::c_void,
                       0 as libc::c_int as u_int);
        return 0 as *mut PAGE
    }
    /* def PURIFY */
    memset(l as *mut libc::c_void, 0xff as libc::c_int,
           (*t).bt_psize as libc::c_ulong);
    (*l).pgno = (*h).pgno;
    (*l).nextpg = (*r).pgno;
    (*l).prevpg = (*h).prevpg;
    (*l).lower =
        (::std::mem::size_of::<db_pgno_t>() as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>() as
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
    (*l).upper = (*t).bt_psize as indx_t;
    (*l).flags = (*h).flags & 0x1f as libc::c_int as libc::c_uint;
    /* Fix up the previous pointer of the page after the split page. */
    if (*h).nextpg != 0 as libc::c_int as libc::c_uint {
        tp =
            kdb2_mpool_get((*t).bt_mp, (*h).nextpg, 0 as libc::c_int as u_int)
                as *mut PAGE;
        if tp.is_null() {
            free(l as *mut libc::c_void);
            /* XXX mpool_free(t->bt_mp, r->pgno); */
            return 0 as *mut PAGE
        }
        (*tp).prevpg = (*r).pgno;
        kdb2_mpool_put((*t).bt_mp, tp as *mut libc::c_void,
                       0x1 as libc::c_int as u_int);
    }
    /*
	 * Split right.  The key/data pairs aren't sorted in the btree page so
	 * it's simpler to copy the data from the split page onto two new pages
	 * instead of copying half the data to the right page and compacting
	 * the left page in place.  Since the left page can't change, we have
	 * to swap the original and the allocated left page after the split.
	 */
    tp = bt_psplit(t, h, l, r, skip, ilen);
    /* Move the new left page onto the old left page. */
    memmove(h as *mut libc::c_void, l as *const libc::c_void,
            (*t).bt_psize as libc::c_ulong);
    if tp == l { tp = h }
    free(l as *mut libc::c_void);
    *lp = h;
    *rp = r;
    return tp;
}
/*
 * BT_ROOT -- Split the root page of a btree.
 *
 * Parameters:
 *	t:	tree
 *	h:	root page
 *	lp:	pointer to left page pointer
 *	rp:	pointer to right page pointer
 *	skip:	pointer to index to leave open
 *	ilen:	insert length
 *
 * Returns:
 *	Pointer to page in which to insert or NULL on error.
 */
#[c2rust::src_loc = "452:1"]
unsafe extern "C" fn bt_root(mut t: *mut BTREE, mut h: *mut PAGE,
                             mut lp: *mut *mut PAGE, mut rp: *mut *mut PAGE,
                             mut skip: *mut indx_t, mut ilen: size_t)
 -> *mut PAGE {
    let mut l: *mut PAGE = 0 as *mut PAGE;
    let mut r: *mut PAGE = 0 as *mut PAGE;
    let mut tp: *mut PAGE = 0 as *mut PAGE;
    let mut lnpg: db_pgno_t = 0;
    let mut rnpg: db_pgno_t = 0;
    /* Put the new left and right pages for the split into place. */
    l = __kdb2_bt_new(t, &mut lnpg);
    if l.is_null() || { r = __kdb2_bt_new(t, &mut rnpg); r.is_null() } {
        return 0 as *mut PAGE
    }
    (*l).pgno = lnpg;
    (*r).pgno = rnpg;
    (*l).nextpg = (*r).pgno;
    (*r).prevpg = (*l).pgno;
    (*r).nextpg = 0 as libc::c_int as db_pgno_t;
    (*l).prevpg = (*r).nextpg;
    (*r).lower =
        (::std::mem::size_of::<db_pgno_t>() as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>() as
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
    (*l).lower = (*r).lower;
    (*r).upper = (*t).bt_psize as indx_t;
    (*l).upper = (*r).upper;
    (*r).flags = (*h).flags & 0x1f as libc::c_int as libc::c_uint;
    (*l).flags = (*r).flags;
    /* Split the root page. */
    tp = bt_psplit(t, h, l, r, skip, ilen);
    *lp = l;
    *rp = r;
    return tp;
}
/*
 * BT_RROOT -- Fix up the recno root page after it has been split.
 *
 * Parameters:
 *	t:	tree
 *	h:	root page
 *	l:	left page
 *	r:	right page
 *
 * Returns:
 *	RET_ERROR, RET_SUCCESS
 */
#[c2rust::src_loc = "499:1"]
unsafe extern "C" fn bt_rroot(mut t: *mut BTREE, mut h: *mut PAGE,
                              mut l: *mut PAGE, mut r: *mut PAGE)
 -> libc::c_int {
    let mut dest: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Insert the left and right keys, set the header information. */
    (*h).upper =
        ((*t).bt_psize as
             libc::c_ulong).wrapping_sub((::std::mem::size_of::<recno_t>() as
                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                              as
                                                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                                              as
                                                                                                              libc::c_ulong).wrapping_sub(1
                                                                                                                                              as
                                                                                                                                              libc::c_int
                                                                                                                                              as
                                                                                                                                              libc::c_ulong)
                                             &
                                             !(::std::mem::size_of::<db_pgno_t>()
                                                   as
                                                   libc::c_ulong).wrapping_sub(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_ulong))
            as indx_t;
    *(*h).linp.as_mut_ptr().offset(0 as libc::c_int as isize) = (*h).upper;
    dest =
        (h as *mut libc::c_char).offset((*h).upper as libc::c_int as isize);
    *(dest as *mut libc::c_void as *mut recno_t) =
        if (*l).flags & 0x10 as libc::c_int as libc::c_uint != 0 {
            ((*l).lower as
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
        } else { rec_total(l) as libc::c_ulong } as recno_t;
    dest =
        dest.offset(::std::mem::size_of::<recno_t>() as libc::c_ulong as
                        isize);
    *(dest as *mut libc::c_void as *mut db_pgno_t) = (*l).pgno;
    (*h).upper =
        ((*h).upper as
             libc::c_ulong).wrapping_sub((::std::mem::size_of::<recno_t>() as
                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                              as
                                                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                                                              as
                                                                                                              libc::c_ulong).wrapping_sub(1
                                                                                                                                              as
                                                                                                                                              libc::c_int
                                                                                                                                              as
                                                                                                                                              libc::c_ulong)
                                             &
                                             !(::std::mem::size_of::<db_pgno_t>()
                                                   as
                                                   libc::c_ulong).wrapping_sub(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_ulong))
            as indx_t as indx_t;
    *(*h).linp.as_mut_ptr().offset(1 as libc::c_int as isize) = (*h).upper;
    dest =
        (h as *mut libc::c_char).offset((*h).upper as libc::c_int as isize);
    *(dest as *mut libc::c_void as *mut recno_t) =
        if (*r).flags & 0x10 as libc::c_int as libc::c_uint != 0 {
            ((*r).lower as
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
        } else { rec_total(r) as libc::c_ulong } as recno_t;
    dest =
        dest.offset(::std::mem::size_of::<recno_t>() as libc::c_ulong as
                        isize);
    *(dest as *mut libc::c_void as *mut db_pgno_t) = (*r).pgno;
    (*h).lower =
        (::std::mem::size_of::<db_pgno_t>() as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>() as
                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                             as
                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                                                             as
                                                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                             as
                                                                                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                             as
                                                                                                                                                                             libc::c_ulong).wrapping_add((2
                                                                                                                                                                                                              as
                                                                                                                                                                                                              libc::c_int
                                                                                                                                                                                                              as
                                                                                                                                                                                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                                              as
                                                                                                                                                                                                                                              libc::c_ulong))
            as indx_t;
    /* Unpin the root page, set to recno internal page. */
    (*h).flags &= !(0x1f as libc::c_int) as libc::c_uint;
    (*h).flags |= 0x8 as libc::c_int as libc::c_uint;
    kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                   0x1 as libc::c_int as u_int);
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
 * BT_BROOT -- Fix up the btree root page after it has been split.
 *
 * Parameters:
 *	t:	tree
 *	h:	root page
 *	l:	left page
 *	r:	right page
 *
 * Returns:
 *	RET_ERROR, RET_SUCCESS
 */
#[c2rust::src_loc = "539:1"]
unsafe extern "C" fn bt_broot(mut t: *mut BTREE, mut h: *mut PAGE,
                              mut l: *mut PAGE, mut r: *mut PAGE)
 -> libc::c_int {
    let mut bi: *mut BINTERNAL = 0 as *mut BINTERNAL;
    let mut bl: *mut BLEAF = 0 as *mut BLEAF;
    let mut nbytes: u_int32_t = 0;
    let mut dest: *mut libc::c_char = 0 as *mut libc::c_char;
    /*
	 * If the root page was a leaf page, change it into an internal page.
	 * We copy the key we split on (but not the key's data, in the case of
	 * a leaf page) to the new root page.
	 *
	 * The btree comparison code guarantees that the left-most key on any
	 * level of the tree is never used, so it doesn't need to be filled in.
	 */
    nbytes =
        ((::std::mem::size_of::<u_int32_t>() as
              libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                              as
                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<u_char>()
                                                                              as
                                                                              libc::c_ulong).wrapping_add(0
                                                                                                              as
                                                                                                              libc::c_int
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
    (*h).upper = (*t).bt_psize.wrapping_sub(nbytes) as indx_t;
    *(*h).linp.as_mut_ptr().offset(0 as libc::c_int as isize) = (*h).upper;
    dest =
        (h as *mut libc::c_char).offset((*h).upper as libc::c_int as isize);
    *(dest as *mut libc::c_void as *mut u_int32_t) =
        0 as libc::c_int as u_int32_t;
    dest =
        dest.offset(::std::mem::size_of::<u_int32_t>() as libc::c_ulong as
                        isize);
    *(dest as *mut libc::c_void as *mut db_pgno_t) = (*l).pgno;
    dest =
        dest.offset(::std::mem::size_of::<db_pgno_t>() as libc::c_ulong as
                        isize);
    *(dest as *mut u_char) = 0 as libc::c_int as u_char;
    dest =
        dest.offset(::std::mem::size_of::<u_char>() as libc::c_ulong as
                        isize);
    match (*h).flags & 0x1f as libc::c_int as libc::c_uint {
        2 => {
            bl =
                (r as
                     *mut libc::c_char).offset(*(*r).linp.as_mut_ptr().offset(0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)
                                                   as libc::c_int as isize) as
                    *mut libc::c_void as *mut BLEAF;
            nbytes =
                ((::std::mem::size_of::<u_int32_t>() as
                      libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                      as
                                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<u_char>()
                                                                                      as
                                                                                      libc::c_ulong).wrapping_add((*bl).ksize
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
            (*h).upper =
                ((*h).upper as libc::c_uint).wrapping_sub(nbytes) as indx_t as
                    indx_t;
            *(*h).linp.as_mut_ptr().offset(1 as libc::c_int as isize) =
                (*h).upper;
            dest =
                (h as
                     *mut libc::c_char).offset((*h).upper as libc::c_int as
                                                   isize);
            *(dest as *mut libc::c_void as *mut u_int32_t) = (*bl).ksize;
            dest =
                dest.offset(::std::mem::size_of::<u_int32_t>() as
                                libc::c_ulong as isize);
            *(dest as *mut libc::c_void as *mut db_pgno_t) = (*r).pgno;
            dest =
                dest.offset(::std::mem::size_of::<db_pgno_t>() as
                                libc::c_ulong as isize);
            *(dest as *mut u_char) = 0 as libc::c_int as u_char;
            dest =
                dest.offset(::std::mem::size_of::<u_char>() as libc::c_ulong
                                as isize);
            memmove(dest as *mut libc::c_void,
                    (*bl).bytes.as_mut_ptr() as *const libc::c_void,
                    (*bl).ksize as libc::c_ulong);
            /*
		 * If the key is on an overflow page, mark the overflow chain
		 * so it isn't deleted when the leaf copy of the key is deleted.
		 */
            if (*bl).flags as libc::c_int & 0x2 as libc::c_int != 0 {
                let mut pgno: db_pgno_t = 0;
                memcpy(&mut pgno as *mut db_pgno_t as *mut libc::c_void,
                       (*bl).bytes.as_mut_ptr() as *const libc::c_void,
                       ::std::mem::size_of::<db_pgno_t>() as libc::c_ulong);
                if bt_preserve(t, pgno) == -(1 as libc::c_int) {
                    return -(1 as libc::c_int)
                }
            }
        }
        1 => {
            bi =
                (r as
                     *mut libc::c_char).offset(*(*r).linp.as_mut_ptr().offset(0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)
                                                   as libc::c_int as isize) as
                    *mut libc::c_void as *mut BINTERNAL;
            nbytes =
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
            (*h).upper =
                ((*h).upper as libc::c_uint).wrapping_sub(nbytes) as indx_t as
                    indx_t;
            *(*h).linp.as_mut_ptr().offset(1 as libc::c_int as isize) =
                (*h).upper;
            dest =
                (h as
                     *mut libc::c_char).offset((*h).upper as libc::c_int as
                                                   isize);
            memmove(dest as *mut libc::c_void, bi as *const libc::c_void,
                    nbytes as libc::c_ulong);
            (*(dest as *mut libc::c_void as *mut BINTERNAL)).pgno = (*r).pgno
        }
        _ => { abort(); }
    }
    /* There are two keys on the page. */
    (*h).lower =
        (::std::mem::size_of::<db_pgno_t>() as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>() as
                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
                                                                             as
                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                                                             as
                                                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                             as
                                                                                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                                                             as
                                                                                                                                                                             libc::c_ulong).wrapping_add((2
                                                                                                                                                                                                              as
                                                                                                                                                                                                              libc::c_int
                                                                                                                                                                                                              as
                                                                                                                                                                                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                                              as
                                                                                                                                                                                                                                              libc::c_ulong))
            as indx_t;
    /* Unpin the root page, set to btree internal page. */
    (*h).flags &= !(0x1f as libc::c_int) as libc::c_uint;
    (*h).flags |= 0x1 as libc::c_int as libc::c_uint;
    kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                   0x1 as libc::c_int as u_int);
    return 0 as libc::c_int;
}
/*
 * BT_PSPLIT -- Do the real work of splitting the page.
 *
 * Parameters:
 *	t:	tree
 *	h:	page to be split
 *	l:	page to put lower half of data
 *	r:	page to put upper half of data
 *	pskip:	pointer to index to leave open
 *	ilen:	insert length
 *
 * Returns:
 *	Pointer to page in which to insert.
 */
#[c2rust::src_loc = "619:1"]
unsafe extern "C" fn bt_psplit(mut t: *mut BTREE, mut h: *mut PAGE,
                               mut l: *mut PAGE, mut r: *mut PAGE,
                               mut pskip: *mut indx_t, mut ilen: size_t)
 -> *mut PAGE {
    let mut bi: *mut BINTERNAL = 0 as *mut BINTERNAL;
    let mut bl: *mut BLEAF = 0 as *mut BLEAF;
    let mut c: *mut CURSOR = 0 as *mut CURSOR;
    let mut rl: *mut RLEAF = 0 as *mut RLEAF;
    let mut rval: *mut PAGE = 0 as *mut PAGE;
    let mut src: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut full: indx_t = 0;
    let mut half: indx_t = 0;
    let mut nxt: indx_t = 0;
    let mut off: indx_t = 0;
    let mut skip: indx_t = 0;
    let mut top: indx_t = 0;
    let mut used: indx_t = 0;
    let mut nbytes: u_int32_t = 0;
    let mut bigkeycnt: libc::c_int = 0;
    let mut isbigkey: libc::c_int = 0;
    /*
	 * Split the data to the left and right pages.  Leave the skip index
	 * open.  Additionally, make some effort not to split on an overflow
	 * key.  This makes internal page processing faster and can save
	 * space as overflow keys used by internal pages are never deleted.
	 */
    bigkeycnt = 0 as libc::c_int; /* silence gcc "uninitialized" warning */
    skip = *pskip;
    full =
        ((*t).bt_psize as
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
                                                                                                                                                                                                              libc::c_ulong))
            as indx_t;
    half = (full as libc::c_int / 2 as libc::c_int) as indx_t;
    used = 0 as libc::c_int as indx_t;
    src = 0 as *mut libc::c_void;
    off = 0 as libc::c_int as indx_t;
    nxt = off;
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
    while (nxt as libc::c_int) < top as libc::c_int {
        if skip as libc::c_int == off as libc::c_int {
            nbytes = ilen as u_int32_t;
            isbigkey = 0 as libc::c_int
            /* XXX: not really known. */
        } else {
            match (*h).flags & 0x1f as libc::c_int as libc::c_uint {
                1 => {
                    bi =
                        (h as
                             *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(nxt
                                                                                          as
                                                                                          isize)
                                                           as libc::c_int as
                                                           isize) as
                            *mut libc::c_void as *mut BINTERNAL;
                    src = bi as *mut libc::c_void;
                    nbytes =
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
                                   libc::c_ulong).wrapping_sub(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong))
                            as u_int32_t;
                    isbigkey = (*bi).flags as libc::c_int & 0x2 as libc::c_int
                }
                2 => {
                    bl =
                        (h as
                             *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(nxt
                                                                                          as
                                                                                          isize)
                                                           as libc::c_int as
                                                           isize) as
                            *mut libc::c_void as *mut BLEAF;
                    src = bl as *mut libc::c_void;
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
                                   libc::c_ulong).wrapping_sub(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong))
                            as u_int32_t;
                    isbigkey = (*bl).flags as libc::c_int & 0x2 as libc::c_int
                }
                8 => {
                    src =
                        (h as *mut libc::c_void as
                             *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(nxt
                                                                                          as
                                                                                          isize)
                                                           as libc::c_int as
                                                           isize) as
                            *mut libc::c_void as *mut RINTERNAL as
                            *mut libc::c_void;
                    nbytes =
                        ((::std::mem::size_of::<recno_t>() as
                              libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
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
                                   libc::c_ulong).wrapping_sub(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong))
                            as u_int32_t;
                    isbigkey = 0 as libc::c_int
                }
                16 => {
                    rl =
                        (h as
                             *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(nxt
                                                                                          as
                                                                                          isize)
                                                           as libc::c_int as
                                                           isize) as
                            *mut libc::c_void as *mut RLEAF;
                    src = rl as *mut libc::c_void;
                    nbytes =
                        ((::std::mem::size_of::<u_int32_t>() as
                              libc::c_ulong).wrapping_add(::std::mem::size_of::<u_char>()
                                                              as
                                                              libc::c_ulong).wrapping_add((*rl).dsize
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
                                   libc::c_ulong).wrapping_sub(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong))
                            as u_int32_t;
                    isbigkey = 0 as libc::c_int
                }
                _ => { abort(); }
            }
        }
        /*
		 * If the key/data pairs are substantial fractions of the max
		 * possible size for the page, it's possible to get situations
		 * where we decide to try and copy too much onto the left page.
		 * Make sure that doesn't happen.
		 */
        if skip as libc::c_int <= off as libc::c_int &&
               ((used as libc::c_uint).wrapping_add(nbytes) as
                    libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                    as libc::c_ulong) >=
                   full as libc::c_ulong ||
               nxt as libc::c_int == top as libc::c_int - 1 as libc::c_int {
            off = off.wrapping_sub(1);
            break ;
        } else {
            /* Copy the key/data pair, if not the skipped index. */
            if skip as libc::c_int != off as libc::c_int {
                nxt = nxt.wrapping_add(1);
                (*l).upper =
                    ((*l).upper as libc::c_uint).wrapping_sub(nbytes) as
                        indx_t as indx_t;
                *(*l).linp.as_mut_ptr().offset(off as isize) = (*l).upper;
                memmove((l as
                             *mut libc::c_char).offset((*l).upper as
                                                           libc::c_int as
                                                           isize) as
                            *mut libc::c_void, src, nbytes as libc::c_ulong);
            }
            used =
                (used as
                     libc::c_ulong).wrapping_add((nbytes as
                                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                      as
                                                                                      libc::c_ulong))
                    as indx_t as indx_t;
            if used as libc::c_int >= half as libc::c_int {
                if isbigkey == 0 || bigkeycnt == 3 as libc::c_int { break ; }
                bigkeycnt += 1
            }
            off = off.wrapping_add(1)
        }
    }
    /*
	 * Off is the last offset that's valid for the left page.
	 * Nxt is the first offset to be placed on the right page.
	 */
    (*l).lower =
        ((*l).lower as
             libc::c_ulong).wrapping_add(((off as libc::c_int +
                                               1 as libc::c_int) as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<indx_t>()
                                                                              as
                                                                              libc::c_ulong))
            as indx_t as indx_t;
    /*
	 * If splitting the page that the cursor was on, the cursor has to be
	 * adjusted to point to the same record as before the split.  If the
	 * cursor is at or past the skipped slot, the cursor is incremented by
	 * one.  If the cursor is on the right page, it is decremented by the
	 * number of records split to the left page.
	 */
    c = &mut (*t).bt_cursor;
    if (*c).flags as libc::c_int & 0x8 as libc::c_int != 0 &&
           (*c).pg.pgno == (*h).pgno {
        if (*c).pg.index as libc::c_int >= skip as libc::c_int {
            (*c).pg.index = (*c).pg.index.wrapping_add(1)
        }
        if ((*c).pg.index as libc::c_int) < nxt as libc::c_int {
            /* Left page. */
            (*c).pg.pgno = (*l).pgno
        } else {
            /* Right page. */
            (*c).pg.pgno = (*r).pgno;
            (*c).pg.index =
                ((*c).pg.index as libc::c_int - nxt as libc::c_int) as indx_t
        }
    }
    /*
	 * If the skipped index was on the left page, just return that page.
	 * Otherwise, adjust the skip index to reflect the new position on
	 * the right page.
	 */
    if skip as libc::c_int <= off as libc::c_int {
        skip = -(1 as libc::c_int) as indx_t;
        rval = l
    } else {
        rval = r;
        *pskip = (*pskip as libc::c_int - nxt as libc::c_int) as indx_t
    }
    off = 0 as libc::c_int as indx_t;
    while (nxt as libc::c_int) < top as libc::c_int {
        if skip as libc::c_int == nxt as libc::c_int {
            off = off.wrapping_add(1);
            skip = -(1 as libc::c_int) as indx_t
        }
        match (*h).flags & 0x1f as libc::c_int as libc::c_uint {
            1 => {
                bi =
                    (h as
                         *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(nxt
                                                                                      as
                                                                                      isize)
                                                       as libc::c_int as
                                                       isize) as
                        *mut libc::c_void as *mut BINTERNAL;
                src = bi as *mut libc::c_void;
                nbytes =
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
                                                               libc::c_ulong))
                        as u_int32_t
            }
            2 => {
                bl =
                    (h as
                         *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(nxt
                                                                                      as
                                                                                      isize)
                                                       as libc::c_int as
                                                       isize) as
                        *mut libc::c_void as *mut BLEAF;
                src = bl as *mut libc::c_void;
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
                                                               libc::c_ulong))
                        as u_int32_t
            }
            8 => {
                src =
                    (h as *mut libc::c_void as
                         *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(nxt
                                                                                      as
                                                                                      isize)
                                                       as libc::c_int as
                                                       isize) as
                        *mut libc::c_void as *mut RINTERNAL as
                        *mut libc::c_void;
                nbytes =
                    ((::std::mem::size_of::<recno_t>() as
                          libc::c_ulong).wrapping_add(::std::mem::size_of::<db_pgno_t>()
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
                                                               libc::c_ulong))
                        as u_int32_t
            }
            16 => {
                rl =
                    (h as
                         *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(nxt
                                                                                      as
                                                                                      isize)
                                                       as libc::c_int as
                                                       isize) as
                        *mut libc::c_void as *mut RLEAF;
                src = rl as *mut libc::c_void;
                nbytes =
                    ((::std::mem::size_of::<u_int32_t>() as
                          libc::c_ulong).wrapping_add(::std::mem::size_of::<u_char>()
                                                          as
                                                          libc::c_ulong).wrapping_add((*rl).dsize
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
                                                               libc::c_ulong))
                        as u_int32_t
            }
            _ => { abort(); }
        }
        nxt = nxt.wrapping_add(1);
        (*r).upper =
            ((*r).upper as libc::c_uint).wrapping_sub(nbytes) as indx_t as
                indx_t;
        *(*r).linp.as_mut_ptr().offset(off as isize) = (*r).upper;
        memmove((r as
                     *mut libc::c_char).offset((*r).upper as libc::c_int as
                                                   isize) as
                    *mut libc::c_void, src, nbytes as libc::c_ulong);
        off = off.wrapping_add(1)
    }
    (*r).lower =
        ((*r).lower as
             libc::c_ulong).wrapping_add((off as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<indx_t>()
                                                                              as
                                                                              libc::c_ulong))
            as indx_t as indx_t;
    /* If the key is being appended to the page, adjust the index. */
    if skip as libc::c_int == top as libc::c_int {
        (*r).lower =
            ((*r).lower as
                 libc::c_ulong).wrapping_add(::std::mem::size_of::<indx_t>()
                                                 as libc::c_ulong) as indx_t
                as indx_t
    }
    return rval;
}
/*
 * BT_PRESERVE -- Mark a chain of pages as used by an internal node.
 *
 * Chains of indirect blocks pointed to by leaf nodes get reclaimed when the
 * record that references them gets deleted.  Chains pointed to by internal
 * pages never get deleted.  This routine marks a chain as pointed to by an
 * internal page.
 *
 * Parameters:
 *	t:	tree
 *	pg:	page number of first page in the chain.
 *
 * Returns:
 *	RET_SUCCESS, RET_ERROR.
 */
#[c2rust::src_loc = "798:1"]
unsafe extern "C" fn bt_preserve(mut t: *mut BTREE, mut pg: db_pgno_t)
 -> libc::c_int {
    let mut h: *mut PAGE = 0 as *mut PAGE;
    h =
        kdb2_mpool_get((*t).bt_mp, pg, 0 as libc::c_int as u_int) as
            *mut PAGE;
    if h.is_null() { return -(1 as libc::c_int) }
    (*h).flags |= 0x20 as libc::c_int as libc::c_uint;
    kdb2_mpool_put((*t).bt_mp, h as *mut libc::c_void,
                   0x1 as libc::c_int as u_int);
    return 0 as libc::c_int;
}
/*
 * REC_TOTAL -- Return the number of recno entries below a page.
 *
 * Parameters:
 *	h:	page
 *
 * Returns:
 *	The number of recno entries below a page.
 *
 * XXX
 * These values could be set by the bt_psplit routine.  The problem is that the
 * entry has to be popped off of the stack etc. or the values have to be passed
 * all the way back to bt_split/bt_rroot and it's not very clean.
 */
#[c2rust::src_loc = "826:1"]
unsafe extern "C" fn rec_total(mut h: *mut PAGE) -> recno_t {
    let mut recs: recno_t = 0;
    let mut nxt: indx_t = 0;
    let mut top: indx_t = 0;
    recs = 0 as libc::c_int as recno_t;
    nxt = 0 as libc::c_int as indx_t;
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
    while (nxt as libc::c_int) < top as libc::c_int {
        recs =
            (recs as
                 libc::c_uint).wrapping_add((*((h as *mut libc::c_void as
                                                    *mut libc::c_char).offset(*(*h).linp.as_mut_ptr().offset(nxt
                                                                                                                 as
                                                                                                                 isize)
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)
                                                   as *mut libc::c_void as
                                                   *mut RINTERNAL)).nrecs) as
                recno_t as recno_t;
        nxt = nxt.wrapping_add(1)
    }
    return recs;
}
