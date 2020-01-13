use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:49"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:49"]
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
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
    #[c2rust::src_loc = "196:1"]
    pub type __syscall_slong_t = libc::c_long;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
}
#[c2rust::header_src = "/usr/include/sys/types.h:49"]
pub mod sys_types_h {
    #[c2rust::src_loc = "33:1"]
    pub type u_char = __u_char;
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "36:1"]
    pub type u_long = __u_long;
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
    use super::types_h::{__u_char, __u_int, __u_long, __ssize_t, __caddr_t,
                         __uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/__sigset_t.h:49"]
pub mod __sigset_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "5:9"]
    pub struct __sigset_t {
        pub __val: [libc::c_ulong; 16],
    }
}
#[c2rust::header_src = "/usr/include/bits/types/sigset_t.h:49"]
pub mod sigset_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type sigset_t = __sigset_t;
    use super::__sigset_t_h::__sigset_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:49"]
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
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:49"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:49"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/stat.h:49"]
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
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/db.h:63"]
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
    use super::stddef_h::size_t;
    use super::sys_types_h::{u_int, u_long};
    /* !_DB_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/include/db-int.h:63"]
pub mod db_int_h {
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
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/btree/btree.h:64"]
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
    pub type C2RustUnnamed_1 = libc::c_uint;
    #[c2rust::src_loc = "334:20"]
    pub const FORWARD: C2RustUnnamed_1 = 2;
    #[c2rust::src_loc = "334:14"]
    pub const BACK: C2RustUnnamed_1 = 1;
    #[c2rust::src_loc = "334:9"]
    pub const NOT: C2RustUnnamed_1 = 0;
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
    #[c2rust::src_loc = "292:1"]
    pub type BTMETA = _btmeta;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "292:16"]
    pub struct _btmeta {
        pub magic: u_int32_t,
        pub version: u_int32_t,
        pub psize: u_int32_t,
        pub free: u_int32_t,
        pub nrecs: u_int32_t,
        pub flags: u_int32_t,
    }
    use super::db_int_h::{db_pgno_t, indx_t, recno_t};
    use super::sys_types_h::{u_int32_t, caddr_t, u_char, u_int8_t};
    use super::mpool_h::MPOOL;
    use super::db_h::{DB, DBT};
    use super::stddef_h::size_t;
    use super::FILE_h::FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/mpool/mpool.h:64"]
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
    use super::sys_types_h::{u_long, u_int8_t, u_int};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "110:1"]
        pub fn kdb2_mpool_delete(_: *mut MPOOL, _: *mut libc::c_void)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "109:1"]
        pub fn kdb2_mpool_get(_: *mut MPOOL, _: db_pgno_t, _: u_int)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "106:1"]
        pub fn kdb2_mpool_filter(_: *mut MPOOL,
                                 _:
                                     Option<unsafe extern "C" fn(_:
                                                                     *mut libc::c_void,
                                                                 _: db_pgno_t,
                                                                 _:
                                                                     *mut libc::c_void)
                                                -> ()>,
                                 _:
                                     Option<unsafe extern "C" fn(_:
                                                                     *mut libc::c_void,
                                                                 _: db_pgno_t,
                                                                 _:
                                                                     *mut libc::c_void)
                                                -> ()>, _: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "105:1"]
        pub fn kdb2_mpool_open(_: *mut libc::c_void, _: libc::c_int,
                               _: db_pgno_t, _: db_pgno_t) -> *mut MPOOL;
        #[no_mangle]
        #[c2rust::src_loc = "111:1"]
        pub fn kdb2_mpool_put(_: *mut MPOOL, _: *mut libc::c_void, _: u_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "108:1"]
        pub fn kdb2_mpool_new(_: *mut MPOOL, _: *mut db_pgno_t, _: u_int)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:49"]
pub mod unistd_h {
    use super::stddef_h::size_t;
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "360:1"]
        pub fn read(__fd: libc::c_int, __buf: *mut libc::c_void,
                    __nbytes: size_t) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "825:1"]
        pub fn unlink(__name: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:49"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:49"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "634:1"]
        pub fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "688:1"]
        pub fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:49"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:49"]
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
#[c2rust::header_src = "/usr/include/errno.h:49"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/signal.h:51"]
pub mod signal_h {
    use super::sigset_t_h::sigset_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "199:1"]
        pub fn sigfillset(__set: *mut sigset_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "229:1"]
        pub fn sigprocmask(__how: libc::c_int, __set: *const sigset_t,
                           __oset: *mut sigset_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/sys/stat.h:52"]
pub mod sys_stat_h {
    use super::stat_h::stat;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "210:1"]
        pub fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/btree/extern.h:64"]
pub mod extern_h {
    use super::db_h::{DB, DBT};
    use super::sys_types_h::u_int;
    use super::stddef_h::size_t;
    use super::db_int_h::db_pgno_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn __kdb2_bt_close(_: *mut DB) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "76:1"]
        pub fn __kdb2_bt_get(_: *const DB, _: *const DBT, _: *mut DBT,
                             _: u_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "71:1"]
        pub fn __kdb2_bt_defpfx(_: *const DBT, _: *const DBT) -> size_t;
        #[no_mangle]
        #[c2rust::src_loc = "72:1"]
        pub fn __kdb2_bt_delete(_: *const DB, _: *const DBT, _: u_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "78:1"]
        pub fn __kdb2_bt_pgin(_: *mut libc::c_void, _: db_pgno_t,
                              _: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "70:1"]
        pub fn __kdb2_bt_defcmp(_: *const DBT, _: *const DBT) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn __kdb2_bt_pgout(_: *mut libc::c_void, _: db_pgno_t,
                               _: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "84:1"]
        pub fn __kdb2_bt_seq(_: *const DB, _: *mut DBT, _: *mut DBT, _: u_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "88:1"]
        pub fn __kdb2_bt_sync(_: *const DB, _: u_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "81:1"]
        pub fn __kdb2_bt_put(dbp: *const DB, _: *mut DBT, _: *const DBT,
                             _: u_int) -> libc::c_int;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__u_char, __u_int, __u_long, __uint8_t, __uint16_t,
                        __uint32_t, __dev_t, __uid_t, __gid_t, __ino_t,
                        __mode_t, __nlink_t, __off_t, __off64_t, __time_t,
                        __blksize_t, __blkcnt_t, __ssize_t, __syscall_slong_t,
                        __caddr_t};
pub use self::sys_types_h::{u_char, u_int, u_long, ssize_t, caddr_t, u_int8_t,
                            u_int16_t, u_int32_t};
pub use self::__sigset_t_h::__sigset_t;
pub use self::sigset_t_h::sigset_t;
pub use self::struct_timespec_h::timespec;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stat_h::stat;
pub use self::db_h::{DBT, DBTYPE, DB_RECNO, DB_HASH, DB_BTREE, __db, DB,
                     BTREEINFO};
pub use self::db_int_h::{db_pgno_t, indx_t, recno_t};
pub use self::btree_h::{PAGE, _page, BTREE, _btree, EPGNO, _epgno,
                        C2RustUnnamed_1, FORWARD, BACK, NOT, CURSOR, _cursor,
                        EPG, _epg, BTMETA, _btmeta};
pub use self::mpool_h::{MPOOL, _hqh, _bkt, C2RustUnnamed, C2RustUnnamed_0,
                        _lqh, kdb2_mpool_delete, kdb2_mpool_get,
                        kdb2_mpool_filter, kdb2_mpool_open, kdb2_mpool_put,
                        kdb2_mpool_new};
use self::unistd_h::{close, read, unlink};
use self::string_h::{strlen, memset};
use self::stdlib_h::{malloc, free, getenv, mkstemp};
use self::stdio_h::snprintf;
use self::fcntl_h::{fcntl, open};
use self::errno_h::__errno_location;
use self::signal_h::{sigfillset, sigprocmask};
use self::sys_stat_h::fstat;
use self::extern_h::{__kdb2_bt_close, __kdb2_bt_get, __kdb2_bt_defpfx,
                     __kdb2_bt_delete, __kdb2_bt_pgin, __kdb2_bt_defcmp,
                     __kdb2_bt_pgout, __kdb2_bt_seq, __kdb2_bt_sync,
                     __kdb2_bt_put};
/* magic number */
/* version */
/* page size */
/* page number of first free page */
/* R: number of records */
/* bt_flags & SAVEMETA */
/*
 * __BT_OPEN -- Open a btree.
 *
 * Creates and fills a DB struct, and calls the routine that actually
 * opens the btree.
 *
 * Parameters:
 *	fname:	filename (NULL for in-memory trees)
 *	flags:	open flag bits
 *	mode:	open permission bits
 *	b:	BTREEINFO pointer
 *
 * Returns:
 *	NULL on failure, pointer to DB on success.
 *
 */
#[no_mangle]
#[c2rust::src_loc = "91:1"]
pub unsafe extern "C" fn __kdb2_bt_open(mut fname: *const libc::c_char,
                                        mut flags: libc::c_int,
                                        mut mode: libc::c_int,
                                        mut openinfo: *const BTREEINFO,
                                        mut dflags: libc::c_int) -> *mut DB {
    let mut current_block: u64;
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
    let mut m: BTMETA =
        BTMETA{magic: 0, version: 0, psize: 0, free: 0, nrecs: 0, flags: 0,};
    let mut t: *mut BTREE = 0 as *mut BTREE;
    let mut b: BTREEINFO =
        BTREEINFO{flags: 0,
                  cachesize: 0,
                  maxkeypage: 0,
                  minkeypage: 0,
                  psize: 0,
                  compare: None,
                  prefix: None,
                  lorder: 0,};
    let mut dbp: *mut DB = 0 as *mut DB;
    let mut ncache: db_pgno_t = 0;
    let mut nr: ssize_t = 0;
    let mut machine_lorder: libc::c_int = 0;
    t = 0 as *mut BTREE;
    /*
	 * Intention is to make sure all of the user's selections are okay
	 * here and then use them without checking.  Can't be complete, since
	 * we don't know the right page size, lorder or flags until the backing
	 * file is opened.  Also, the file's page size can cause the cachesize
	 * to change.
	 */
    machine_lorder = byteorder();
    if !openinfo.is_null() {
        b = *openinfo;
        /* Flags: R_DUP. */
        if b.flags & !(0x1 as libc::c_int) as libc::c_ulong != 0 {
            current_block = 3616124888043574020;
        } else if b.psize != 0 &&
                      (b.psize < 512 as libc::c_int as libc::c_uint ||
                           b.psize >
                               (65535 as libc::c_int + 1 as libc::c_int) as
                                   libc::c_uint ||
                           b.psize as libc::c_ulong &
                               (::std::mem::size_of::<indx_t>() as
                                    libc::c_ulong).wrapping_sub(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong)
                               != 0) {
            current_block = 3616124888043574020;
        } else {
            /*
		 * Page size must be indx_t aligned and >= MINPSIZE.  Default
		 * page size is set farther on, based on the underlying file
		 * transfer size.
		 */
            /* Minimum number of keys per page; absolute minimum is 2. */
            if b.minkeypage != 0 {
                if b.minkeypage < 2 as libc::c_int {
                    current_block = 3616124888043574020;
                } else { current_block = 3512920355445576850; }
            } else {
                b.minkeypage = 2 as libc::c_int;
                current_block = 3512920355445576850;
            }
            match current_block {
                3616124888043574020 => { }
                _ => {
                    /* If no comparison, use default comparison and prefix. */
                    if b.compare.is_none() {
                        b.compare =
                            Some(__kdb2_bt_defcmp as
                                     unsafe extern "C" fn(_: *const DBT,
                                                          _: *const DBT)
                                         -> libc::c_int);
                        if b.prefix.is_none() {
                            b.prefix =
                                Some(__kdb2_bt_defpfx as
                                         unsafe extern "C" fn(_: *const DBT,
                                                              _: *const DBT)
                                             -> size_t)
                        }
                    }
                    if b.lorder == 0 as libc::c_int {
                        b.lorder = machine_lorder
                    }
                    current_block = 15925075030174552612;
                }
            }
        }
    } else {
        b.compare =
            Some(__kdb2_bt_defcmp as
                     unsafe extern "C" fn(_: *const DBT, _: *const DBT)
                         -> libc::c_int);
        b.cachesize = 0 as libc::c_int as u_int;
        b.flags = 0 as libc::c_int as u_long;
        b.lorder = machine_lorder;
        b.minkeypage = 2 as libc::c_int;
        b.prefix =
            Some(__kdb2_bt_defpfx as
                     unsafe extern "C" fn(_: *const DBT, _: *const DBT)
                         -> size_t);
        b.psize = 0 as libc::c_int as u_int;
        current_block = 15925075030174552612;
    }
    match current_block {
        15925075030174552612 =>
        /* Check for the ubiquitous PDP-11. */
        {
            if b.lorder != 4321 as libc::c_int &&
                   b.lorder != 1234 as libc::c_int {
                current_block = 3616124888043574020;
            } else {
                /* Allocate and initialize DB and BTREE structures. */
                t =
                    malloc(::std::mem::size_of::<BTREE>() as libc::c_ulong) as
                        *mut BTREE; /* Don't close unopened fd on error. */
                if t.is_null() {
                    current_block = 6123457040714290328;
                } else {
                    memset(t as *mut libc::c_void, 0 as libc::c_int,
                           ::std::mem::size_of::<BTREE>() as libc::c_ulong);
                    (*t).bt_fd = -(1 as libc::c_int);
                    (*t).bt_lorder = b.lorder;
                    (*t).bt_order = NOT;
                    (*t).bt_cmp = b.compare;
                    (*t).bt_pfx = b.prefix;
                    (*t).bt_rfd = -(1 as libc::c_int);
                    dbp =
                        malloc(::std::mem::size_of::<DB>() as libc::c_ulong)
                            as *mut DB;
                    (*t).bt_dbp = dbp;
                    if (*t).bt_dbp.is_null() {
                        current_block = 6123457040714290328;
                    } else {
                        memset((*t).bt_dbp as *mut libc::c_void,
                               0 as libc::c_int,
                               ::std::mem::size_of::<DB>() as libc::c_ulong);
                        if (*t).bt_lorder != machine_lorder {
                            (*t).flags |= 0x8 as libc::c_int as libc::c_uint
                        }
                        (*dbp).type_0 = DB_BTREE;
                        (*dbp).internal = t as *mut libc::c_void;
                        (*dbp).close =
                            Some(__kdb2_bt_close as
                                     unsafe extern "C" fn(_: *mut DB)
                                         -> libc::c_int);
                        (*dbp).del =
                            Some(__kdb2_bt_delete as
                                     unsafe extern "C" fn(_: *const DB,
                                                          _: *const DBT,
                                                          _: u_int)
                                         -> libc::c_int);
                        (*dbp).fd =
                            Some(__kdb2_bt_fd as
                                     unsafe extern "C" fn(_: *const DB)
                                         -> libc::c_int);
                        (*dbp).get =
                            Some(__kdb2_bt_get as
                                     unsafe extern "C" fn(_: *const DB,
                                                          _: *const DBT,
                                                          _: *mut DBT,
                                                          _: u_int)
                                         -> libc::c_int);
                        (*dbp).put =
                            Some(__kdb2_bt_put as
                                     unsafe extern "C" fn(_: *const DB,
                                                          _: *mut DBT,
                                                          _: *const DBT,
                                                          _: u_int)
                                         -> libc::c_int);
                        (*dbp).seq =
                            Some(__kdb2_bt_seq as
                                     unsafe extern "C" fn(_: *const DB,
                                                          _: *mut DBT,
                                                          _: *mut DBT,
                                                          _: u_int)
                                         -> libc::c_int);
                        (*dbp).sync =
                            Some(__kdb2_bt_sync as
                                     unsafe extern "C" fn(_: *const DB,
                                                          _: u_int)
                                         -> libc::c_int);
                        /*
	 * If no file name was supplied, this is an in-memory btree and we
	 * open a backing temporary file.  Otherwise, it's a disk-based tree.
	 */
                        if !fname.is_null() {
                            match flags & 0o3 as libc::c_int {
                                0 => {
                                    current_block = 1615128631406360608;
                                    match current_block {
                                        1615128631406360608 => {
                                            (*t).flags |=
                                                0x10 as libc::c_int as
                                                    libc::c_uint
                                        }
                                        _ => { }
                                    }
                                    (*t).bt_fd =
                                        open(fname, flags | 0 as libc::c_int,
                                             mode);
                                    if (*t).bt_fd < 0 as libc::c_int {
                                        current_block = 6123457040714290328;
                                    } else {
                                        current_block = 6528285054092551010;
                                    }
                                }
                                2 => {
                                    current_block = 12556861819962772176;
                                    match current_block {
                                        1615128631406360608 => {
                                            (*t).flags |=
                                                0x10 as libc::c_int as
                                                    libc::c_uint
                                        }
                                        _ => { }
                                    }
                                    (*t).bt_fd =
                                        open(fname, flags | 0 as libc::c_int,
                                             mode);
                                    if (*t).bt_fd < 0 as libc::c_int {
                                        current_block = 6123457040714290328;
                                    } else {
                                        current_block = 6528285054092551010;
                                    }
                                }
                                1 | _ => {
                                    current_block = 3616124888043574020;
                                }
                            }
                        } else if flags & 0o3 as libc::c_int !=
                                      0o2 as libc::c_int {
                            current_block = 3616124888043574020;
                        } else {
                            (*t).bt_fd = tmp();
                            if (*t).bt_fd == -(1 as libc::c_int) {
                                current_block = 6123457040714290328;
                            } else {
                                (*t).flags |=
                                    0x1 as libc::c_int as libc::c_uint;
                                current_block = 6528285054092551010;
                            }
                        }
                        match current_block {
                            6123457040714290328 => { }
                            3616124888043574020 => { }
                            _ => {
                                if fcntl((*t).bt_fd, 2 as libc::c_int,
                                         1 as libc::c_int) ==
                                       -(1 as libc::c_int) {
                                    current_block = 6123457040714290328;
                                } else if fstat((*t).bt_fd, &mut sb) != 0 {
                                    current_block = 6123457040714290328;
                                } else {
                                    if sb.st_size != 0 {
                                        nr =
                                            read((*t).bt_fd,
                                                 &mut m as *mut BTMETA as
                                                     *mut libc::c_void,
                                                 ::std::mem::size_of::<BTMETA>()
                                                     as libc::c_ulong);
                                        if nr <
                                               0 as libc::c_int as
                                                   libc::c_long {
                                            current_block =
                                                6123457040714290328;
                                        } else {
                                            if nr as libc::c_ulong !=
                                                   ::std::mem::size_of::<BTMETA>()
                                                       as libc::c_ulong {
                                                current_block =
                                                    17475637831468219911;
                                            } else {
                                                /*
		 * Read in the meta-data.  This can change the notion of what
		 * the lorder, page size and flags are, and, when the page size
		 * changes, the cachesize value can change too.  If the user
		 * specified the wrong byte order for an existing database, we
		 * don't bother to return an error, we just clear the NEEDSWAP
		 * bit.
		 */
                                                if m.magic ==
                                                       0x53162 as libc::c_int
                                                           as libc::c_uint {
                                                    (*t).flags &=
                                                        !(0x8 as libc::c_int)
                                                            as libc::c_uint
                                                } else {
                                                    (*t).flags |=
                                                        0x8 as libc::c_int as
                                                            libc::c_uint;
                                                    let mut _tmp: u_int32_t =
                                                        m.magic;
                                                    *(&mut m.magic as
                                                          *mut u_int32_t as
                                                          *mut libc::c_char).offset(0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                        =
                                                        *(&mut _tmp as
                                                              *mut u_int32_t
                                                              as
                                                              *mut libc::c_char).offset(3
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize);
                                                    *(&mut m.magic as
                                                          *mut u_int32_t as
                                                          *mut libc::c_char).offset(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                        =
                                                        *(&mut _tmp as
                                                              *mut u_int32_t
                                                              as
                                                              *mut libc::c_char).offset(2
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize);
                                                    *(&mut m.magic as
                                                          *mut u_int32_t as
                                                          *mut libc::c_char).offset(2
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                        =
                                                        *(&mut _tmp as
                                                              *mut u_int32_t
                                                              as
                                                              *mut libc::c_char).offset(1
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize);
                                                    *(&mut m.magic as
                                                          *mut u_int32_t as
                                                          *mut libc::c_char).offset(3
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                        =
                                                        *(&mut _tmp as
                                                              *mut u_int32_t
                                                              as
                                                              *mut libc::c_char).offset(0
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize);
                                                    let mut _tmp_0:
                                                            u_int32_t =
                                                        m.version;
                                                    *(&mut m.version as
                                                          *mut u_int32_t as
                                                          *mut libc::c_char).offset(0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                        =
                                                        *(&mut _tmp_0 as
                                                              *mut u_int32_t
                                                              as
                                                              *mut libc::c_char).offset(3
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize);
                                                    *(&mut m.version as
                                                          *mut u_int32_t as
                                                          *mut libc::c_char).offset(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                        =
                                                        *(&mut _tmp_0 as
                                                              *mut u_int32_t
                                                              as
                                                              *mut libc::c_char).offset(2
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize);
                                                    *(&mut m.version as
                                                          *mut u_int32_t as
                                                          *mut libc::c_char).offset(2
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                        =
                                                        *(&mut _tmp_0 as
                                                              *mut u_int32_t
                                                              as
                                                              *mut libc::c_char).offset(1
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize);
                                                    *(&mut m.version as
                                                          *mut u_int32_t as
                                                          *mut libc::c_char).offset(3
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                        =
                                                        *(&mut _tmp_0 as
                                                              *mut u_int32_t
                                                              as
                                                              *mut libc::c_char).offset(0
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize);
                                                    let mut _tmp_1:
                                                            u_int32_t =
                                                        m.psize;
                                                    *(&mut m.psize as
                                                          *mut u_int32_t as
                                                          *mut libc::c_char).offset(0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                        =
                                                        *(&mut _tmp_1 as
                                                              *mut u_int32_t
                                                              as
                                                              *mut libc::c_char).offset(3
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize);
                                                    *(&mut m.psize as
                                                          *mut u_int32_t as
                                                          *mut libc::c_char).offset(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                        =
                                                        *(&mut _tmp_1 as
                                                              *mut u_int32_t
                                                              as
                                                              *mut libc::c_char).offset(2
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize);
                                                    *(&mut m.psize as
                                                          *mut u_int32_t as
                                                          *mut libc::c_char).offset(2
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                        =
                                                        *(&mut _tmp_1 as
                                                              *mut u_int32_t
                                                              as
                                                              *mut libc::c_char).offset(1
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize);
                                                    *(&mut m.psize as
                                                          *mut u_int32_t as
                                                          *mut libc::c_char).offset(3
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                        =
                                                        *(&mut _tmp_1 as
                                                              *mut u_int32_t
                                                              as
                                                              *mut libc::c_char).offset(0
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize);
                                                    let mut _tmp_2:
                                                            u_int32_t =
                                                        m.free;
                                                    *(&mut m.free as
                                                          *mut u_int32_t as
                                                          *mut libc::c_char).offset(0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                        =
                                                        *(&mut _tmp_2 as
                                                              *mut u_int32_t
                                                              as
                                                              *mut libc::c_char).offset(3
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize);
                                                    *(&mut m.free as
                                                          *mut u_int32_t as
                                                          *mut libc::c_char).offset(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                        =
                                                        *(&mut _tmp_2 as
                                                              *mut u_int32_t
                                                              as
                                                              *mut libc::c_char).offset(2
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize);
                                                    *(&mut m.free as
                                                          *mut u_int32_t as
                                                          *mut libc::c_char).offset(2
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                        =
                                                        *(&mut _tmp_2 as
                                                              *mut u_int32_t
                                                              as
                                                              *mut libc::c_char).offset(1
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize);
                                                    *(&mut m.free as
                                                          *mut u_int32_t as
                                                          *mut libc::c_char).offset(3
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                        =
                                                        *(&mut _tmp_2 as
                                                              *mut u_int32_t
                                                              as
                                                              *mut libc::c_char).offset(0
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize);
                                                    let mut _tmp_3:
                                                            u_int32_t =
                                                        m.nrecs;
                                                    *(&mut m.nrecs as
                                                          *mut u_int32_t as
                                                          *mut libc::c_char).offset(0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                        =
                                                        *(&mut _tmp_3 as
                                                              *mut u_int32_t
                                                              as
                                                              *mut libc::c_char).offset(3
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize);
                                                    *(&mut m.nrecs as
                                                          *mut u_int32_t as
                                                          *mut libc::c_char).offset(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                        =
                                                        *(&mut _tmp_3 as
                                                              *mut u_int32_t
                                                              as
                                                              *mut libc::c_char).offset(2
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize);
                                                    *(&mut m.nrecs as
                                                          *mut u_int32_t as
                                                          *mut libc::c_char).offset(2
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                        =
                                                        *(&mut _tmp_3 as
                                                              *mut u_int32_t
                                                              as
                                                              *mut libc::c_char).offset(1
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize);
                                                    *(&mut m.nrecs as
                                                          *mut u_int32_t as
                                                          *mut libc::c_char).offset(3
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                        =
                                                        *(&mut _tmp_3 as
                                                              *mut u_int32_t
                                                              as
                                                              *mut libc::c_char).offset(0
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize);
                                                    let mut _tmp_4:
                                                            u_int32_t =
                                                        m.flags;
                                                    *(&mut m.flags as
                                                          *mut u_int32_t as
                                                          *mut libc::c_char).offset(0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                        =
                                                        *(&mut _tmp_4 as
                                                              *mut u_int32_t
                                                              as
                                                              *mut libc::c_char).offset(3
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize);
                                                    *(&mut m.flags as
                                                          *mut u_int32_t as
                                                          *mut libc::c_char).offset(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                        =
                                                        *(&mut _tmp_4 as
                                                              *mut u_int32_t
                                                              as
                                                              *mut libc::c_char).offset(2
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize);
                                                    *(&mut m.flags as
                                                          *mut u_int32_t as
                                                          *mut libc::c_char).offset(2
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                        =
                                                        *(&mut _tmp_4 as
                                                              *mut u_int32_t
                                                              as
                                                              *mut libc::c_char).offset(1
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize);
                                                    *(&mut m.flags as
                                                          *mut u_int32_t as
                                                          *mut libc::c_char).offset(3
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                        =
                                                        *(&mut _tmp_4 as
                                                              *mut u_int32_t
                                                              as
                                                              *mut libc::c_char).offset(0
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize)
                                                }
                                                if m.magic !=
                                                       0x53162 as libc::c_int
                                                           as libc::c_uint ||
                                                       m.version !=
                                                           3 as libc::c_int as
                                                               libc::c_uint {
                                                    current_block =
                                                        17475637831468219911;
                                                } else if m.psize <
                                                              512 as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uint
                                                              ||
                                                              m.psize >
                                                                  (65535 as
                                                                       libc::c_int
                                                                       +
                                                                       1 as
                                                                           libc::c_int)
                                                                      as
                                                                      libc::c_uint
                                                              ||
                                                              m.psize as
                                                                  libc::c_ulong
                                                                  &
                                                                  (::std::mem::size_of::<indx_t>()
                                                                       as
                                                                       libc::c_ulong).wrapping_sub(1
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong)
                                                                  != 0 {
                                                    current_block =
                                                        17475637831468219911;
                                                } else if m.flags &
                                                              !(0x20 as
                                                                    libc::c_int
                                                                    |
                                                                    0x80 as
                                                                        libc::c_int)
                                                                  as
                                                                  libc::c_uint
                                                              != 0 {
                                                    current_block =
                                                        17475637831468219911;
                                                } else {
                                                    b.psize = m.psize;
                                                    (*t).flags |= m.flags;
                                                    (*t).bt_free = m.free;
                                                    (*t).bt_nrecs = m.nrecs;
                                                    current_block =
                                                        14652688882591975137;
                                                }
                                            }
                                            match current_block {
                                                14652688882591975137 => { }
                                                _ => {
                                                    *__errno_location() =
                                                        22 as libc::c_int;
                                                    current_block =
                                                        6123457040714290328;
                                                }
                                            }
                                        }
                                    } else {
                                        /*
		 * Set the page size to the best value for I/O to this file.
		 * Don't overflow the page offset type.
		 */
                                        if b.psize ==
                                               0 as libc::c_int as
                                                   libc::c_uint {
                                            b.psize = sb.st_blksize as u_int;
                                            if b.psize <
                                                   512 as libc::c_int as
                                                       libc::c_uint {
                                                b.psize =
                                                    512 as libc::c_int as
                                                        u_int
                                            }
                                            if b.psize >
                                                   (65535 as libc::c_int +
                                                        1 as libc::c_int) as
                                                       libc::c_uint {
                                                b.psize =
                                                    (65535 as libc::c_int +
                                                         1 as libc::c_int) as
                                                        u_int
                                            }
                                        }
                                        /* Set flag if duplicates permitted. */
                                        if b.flags &
                                               0x1 as libc::c_int as
                                                   libc::c_ulong == 0 {
                                            (*t).flags |=
                                                0x20 as libc::c_int as
                                                    libc::c_uint
                                        }
                                        (*t).bt_free =
                                            0 as libc::c_int as db_pgno_t;
                                        (*t).bt_nrecs =
                                            0 as libc::c_int as recno_t;
                                        (*t).flags |=
                                            0x2 as libc::c_int as
                                                libc::c_uint;
                                        current_block = 14652688882591975137;
                                    }
                                    match current_block {
                                        6123457040714290328 => { }
                                        _ => {
                                            (*t).bt_psize = b.psize;
                                            /* Set the cache size; must be a multiple of the page size. */
                                            if b.cachesize != 0 &&
                                                   b.cachesize &
                                                       b.psize.wrapping_sub(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint)
                                                       != 0 {
                                                b.cachesize =
                                                    (b.cachesize as
                                                         libc::c_uint).wrapping_add((!b.cachesize
                                                                                         &
                                                                                         b.psize.wrapping_sub(1
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_uint)).wrapping_add(1
                                                                                                                                                  as
                                                                                                                                                  libc::c_int
                                                                                                                                                  as
                                                                                                                                                  libc::c_uint))
                                                        as u_int as u_int
                                            }
                                            if b.cachesize <
                                                   b.psize.wrapping_mul(5 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
                                               {
                                                b.cachesize =
                                                    b.psize.wrapping_mul(5 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                            }
                                            /* Calculate number of pages to cache. */
                                            ncache =
                                                b.cachesize.wrapping_add((*t).bt_psize).wrapping_sub(1
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint).wrapping_div((*t).bt_psize);
                                            /*
	 * The btree data structure requires that at least two keys can fit on
	 * a page, but other than that there's no fixed requirement.  The user
	 * specified a minimum number per page, and we translated that into the
	 * number of bytes a key/data pair can use before being placed on an
	 * overflow page.  This calculation includes the page header, the size
	 * of the index referencing the leaf item and the size of the leaf item
	 * structure.  Also, don't let the user specify a minkeypage such that
	 * a key/data pair won't fit even if both key and data are on overflow
	 * pages.
	 */
                                            (*t).bt_ovflsize =
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
                                                                                                                                                                                                                                                      libc::c_ulong)).wrapping_div(b.minkeypage
                                                                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                                                                       libc::c_ulong).wrapping_sub((::std::mem::size_of::<indx_t>()
                                                                                                                                                                                                                                                                                                                        as
                                                                                                                                                                                                                                                                                                                        libc::c_ulong).wrapping_add((::std::mem::size_of::<u_int32_t>()
                                                                                                                                                                                                                                                                                                                                                         as
                                                                                                                                                                                                                                                                                                                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                                                                                                                                                                                                                                                                                                                                         as
                                                                                                                                                                                                                                                                                                                                                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<u_char>()
                                                                                                                                                                                                                                                                                                                                                                                                                         as
                                                                                                                                                                                                                                                                                                                                                                                                                         libc::c_ulong).wrapping_add(0
                                                                                                                                                                                                                                                                                                                                                                                                                                                         as
                                                                                                                                                                                                                                                                                                                                                                                                                                                         libc::c_int
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
                                                                                                                                                                                                                                                                                                                                                        !(::std::mem::size_of::<db_pgno_t>()
                                                                                                                                                                                                                                                                                                                                                              as
                                                                                                                                                                                                                                                                                                                                                              libc::c_ulong).wrapping_sub(1
                                                                                                                                                                                                                                                                                                                                                                                              as
                                                                                                                                                                                                                                                                                                                                                                                              libc::c_int
                                                                                                                                                                                                                                                                                                                                                                                              as
                                                                                                                                                                                                                                                                                                                                                                                              libc::c_ulong)))
                                                    as indx_t;
                                            if ((*t).bt_ovflsize as
                                                    libc::c_ulong) <
                                                   ((::std::mem::size_of::<u_int32_t>()
                                                         as
                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                                         as
                                                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<u_char>()
                                                                                                                         as
                                                                                                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<db_pgno_t>()
                                                                                                                                                          as
                                                                                                                                                          libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                                                                                                                                          as
                                                                                                                                                                                          libc::c_ulong)).wrapping_add((::std::mem::size_of::<db_pgno_t>()
                                                                                                                                                                                                                            as
                                                                                                                                                                                                                            libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                                                                                                                                                                                                            as
                                                                                                                                                                                                                                                            libc::c_ulong)).wrapping_add(::std::mem::size_of::<db_pgno_t>()
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
                                                                                              libc::c_ulong)).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                               as
                                                                                                                               libc::c_ulong)
                                               {
                                                (*t).bt_ovflsize =
                                                    ((::std::mem::size_of::<u_int32_t>()
                                                          as
                                                          libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_add(::std::mem::size_of::<u_char>()
                                                                                                                          as
                                                                                                                          libc::c_ulong).wrapping_add((::std::mem::size_of::<db_pgno_t>()
                                                                                                                                                           as
                                                                                                                                                           libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                                                                                                                                           as
                                                                                                                                                                                           libc::c_ulong)).wrapping_add((::std::mem::size_of::<db_pgno_t>()
                                                                                                                                                                                                                             as
                                                                                                                                                                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<u_int32_t>()
                                                                                                                                                                                                                                                             as
                                                                                                                                                                                                                                                             libc::c_ulong)).wrapping_add(::std::mem::size_of::<db_pgno_t>()
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
                                                                                               libc::c_ulong)).wrapping_add(::std::mem::size_of::<indx_t>()
                                                                                                                                as
                                                                                                                                libc::c_ulong)
                                                        as indx_t
                                            }
                                            /* Initialize the buffer pool. */
                                            (*t).bt_mp =
                                                kdb2_mpool_open(0 as
                                                                    *mut libc::c_void,
                                                                (*t).bt_fd,
                                                                (*t).bt_psize,
                                                                ncache);
                                            if (*t).bt_mp.is_null() {
                                                current_block =
                                                    6123457040714290328;
                                            } else {
                                                if (*t).flags &
                                                       0x1 as libc::c_int as
                                                           libc::c_uint == 0 {
                                                    kdb2_mpool_filter((*t).bt_mp,
                                                                      Some(__kdb2_bt_pgin
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut libc::c_void,
                                                                                                    _:
                                                                                                        db_pgno_t,
                                                                                                    _:
                                                                                                        *mut libc::c_void)
                                                                                   ->
                                                                                       ()),
                                                                      Some(__kdb2_bt_pgout
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut libc::c_void,
                                                                                                    _:
                                                                                                        db_pgno_t,
                                                                                                    _:
                                                                                                        *mut libc::c_void)
                                                                                   ->
                                                                                       ()),
                                                                      t as
                                                                          *mut libc::c_void);
                                                }
                                                /* Create a root page if new tree. */
                                                if nroot(t) ==
                                                       -(1 as libc::c_int) {
                                                    current_block =
                                                        6123457040714290328;
                                                } else {
                                                    /* Global flags. */
                                                    if dflags &
                                                           0x20000000 as
                                                               libc::c_int !=
                                                           0 {
                                                        (*t).flags |=
                                                            0x4000 as
                                                                libc::c_int as
                                                                libc::c_uint
                                                    }
                                                    if dflags &
                                                           0x40000000 as
                                                               libc::c_int !=
                                                           0 {
                                                        (*t).flags |=
                                                            0x8000 as
                                                                libc::c_int as
                                                                libc::c_uint
                                                    }
                                                    if dflags as libc::c_uint
                                                           &
                                                           0x80000000 as
                                                               libc::c_uint !=
                                                           0 {
                                                        (*t).flags |=
                                                            0x10000 as
                                                                libc::c_int as
                                                                libc::c_uint
                                                    }
                                                    return dbp
                                                }
                                            }
                                        }
                                    }
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
        3616124888043574020 => { *__errno_location() = 22 as libc::c_int }
        _ => { }
    }
    if !t.is_null() {
        if !(*t).bt_dbp.is_null() { free((*t).bt_dbp as *mut libc::c_void); }
        if (*t).bt_fd != -(1 as libc::c_int) { close((*t).bt_fd); }
        free(t as *mut libc::c_void);
    }
    return 0 as *mut DB;
}
/*
 * NROOT -- Create the root of a new tree.
 *
 * Parameters:
 *	t:	tree
 *
 * Returns:
 *	RET_ERROR, RET_SUCCESS
 */
#[c2rust::src_loc = "354:1"]
unsafe extern "C" fn nroot(mut t: *mut BTREE) -> libc::c_int {
    let mut meta: *mut PAGE = 0 as *mut PAGE;
    let mut root: *mut PAGE = 0 as *mut PAGE;
    let mut npg: db_pgno_t = 0;
    root =
        kdb2_mpool_get((*t).bt_mp, 1 as libc::c_int as db_pgno_t,
                       0 as libc::c_int as u_int) as *mut PAGE;
    if !root.is_null() {
        if (*root).lower as libc::c_int == 0 as libc::c_int &&
               (*root).pgno == 0 as libc::c_int as libc::c_uint &&
               *(*root).linp.as_mut_ptr().offset(0 as libc::c_int as isize) as
                   libc::c_int == 0 as libc::c_int {
            kdb2_mpool_delete((*t).bt_mp, root as *mut libc::c_void);
            *__errno_location() = 22 as libc::c_int
        } else {
            kdb2_mpool_put((*t).bt_mp, root as *mut libc::c_void,
                           0 as libc::c_int as u_int);
            return 0 as libc::c_int
        }
    }
    if *__errno_location() != 22 as libc::c_int {
        /* It's OK to not exist. */
        return -(1 as libc::c_int)
    }
    *__errno_location() = 0 as libc::c_int;
    meta =
        kdb2_mpool_new((*t).bt_mp, &mut npg, 0x2 as libc::c_int as u_int) as
            *mut PAGE;
    if meta.is_null() { return -(1 as libc::c_int) }
    root =
        kdb2_mpool_new((*t).bt_mp, &mut npg, 0x2 as libc::c_int as u_int) as
            *mut PAGE;
    if root.is_null() { return -(1 as libc::c_int) }
    if npg != 1 as libc::c_int as libc::c_uint { return -(1 as libc::c_int) }
    (*root).pgno = npg;
    (*root).nextpg = 0 as libc::c_int as db_pgno_t;
    (*root).prevpg = (*root).nextpg;
    (*root).lower =
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
    (*root).upper = (*t).bt_psize as indx_t;
    (*root).flags = 0x2 as libc::c_int as u_int32_t;
    memset(meta as *mut libc::c_void, 0 as libc::c_int,
           (*t).bt_psize as libc::c_ulong);
    kdb2_mpool_put((*t).bt_mp, meta as *mut libc::c_void,
                   0x1 as libc::c_int as u_int);
    kdb2_mpool_put((*t).bt_mp, root as *mut libc::c_void,
                   0x1 as libc::c_int as u_int);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "395:1"]
unsafe extern "C" fn tmp() -> libc::c_int {
    let mut set: sigset_t = sigset_t{__val: [0; 16],};
    let mut oset: sigset_t = sigset_t{__val: [0; 16],};
    let mut fd: libc::c_int = 0;
    let mut envtmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path: [libc::c_char; 4096] = [0; 4096];
    static mut fn_0: [libc::c_char; 11] =
        [47, 98, 116, 46, 88, 88, 88, 88, 88, 88, 0];
    envtmp = getenv(b"TMPDIR\x00" as *const u8 as *const libc::c_char);
    /* this used to be done with snprintf(), but since snprintf
	   isn't in most operating systems, and overflow checking in
	   this case is easy, this is what is done */
    if !envtmp.is_null() &&
           strlen(envtmp).wrapping_add(::std::mem::size_of::<[libc::c_char; 11]>()
                                           as
                                           libc::c_ulong).wrapping_add(1 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ulong)
               >
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
       {
        return -(1 as libc::c_int)
    }
    snprintf(path.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
             b"%s%s\x00" as *const u8 as *const libc::c_char,
             if !envtmp.is_null() {
                 envtmp as *const libc::c_char
             } else { b"/tmp\x00" as *const u8 as *const libc::c_char },
             fn_0.as_mut_ptr());
    sigfillset(&mut set);
    sigprocmask(0 as libc::c_int, &mut set, &mut oset);
    fd = mkstemp(path.as_mut_ptr());
    if fd != -(1 as libc::c_int) { unlink(path.as_mut_ptr()); }
    fcntl(fd, 2 as libc::c_int, 1 as libc::c_int);
    sigprocmask(2 as libc::c_int, &mut oset, 0 as *mut sigset_t);
    /* __CYGWIN32__ */
    return fd;
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
 * Implementation of btree access method for 4.4BSD.
 *
 * The design here was originally based on that of the btree access method
 * used in the Postgres database system at UC Berkeley.  This implementation
 * is wholly independent of the Postgres code.
 */
#[c2rust::src_loc = "442:1"]
unsafe extern "C" fn byteorder() -> libc::c_int {
    let mut x: u_int32_t = 0;
    let mut p: *mut u_char = 0 as *mut u_char;
    x = 0x1020304 as libc::c_int as u_int32_t;
    p = &mut x as *mut u_int32_t as *mut u_char;
    match *p as libc::c_int {
        1 => { return 4321 as libc::c_int }
        4 => { return 1234 as libc::c_int }
        _ => { return 0 as libc::c_int }
    };
}
#[no_mangle]
#[c2rust::src_loc = "460:1"]
pub unsafe extern "C" fn __kdb2_bt_fd(mut dbp: *const DB) -> libc::c_int {
    let mut t: *mut BTREE = 0 as *mut BTREE;
    t = (*dbp).internal as *mut BTREE;
    /* Toss any page pinned across calls. */
    if !(*t).bt_pinned.is_null() {
        kdb2_mpool_put((*t).bt_mp, (*t).bt_pinned as *mut libc::c_void,
                       0 as libc::c_int as u_int);
        (*t).bt_pinned = 0 as *mut PAGE
    }
    /* In-memory database can't have a file descriptor. */
    if (*t).flags & 0x1 as libc::c_int as libc::c_uint != 0 {
        *__errno_location() = 2 as libc::c_int;
        return -(1 as libc::c_int)
    }
    return (*t).bt_fd;
}
