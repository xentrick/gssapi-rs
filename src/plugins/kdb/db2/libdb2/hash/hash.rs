use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:41"]
pub mod types_h {
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "34:1"]
    pub type __u_long = libc::c_ulong;
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = libc::c_schar;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
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
}
#[c2rust::header_src = "/usr/include/sys/types.h:41"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "36:1"]
    pub type u_long = __u_long;
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    #[c2rust::src_loc = "158:1"]
    pub type u_int8_t = __uint8_t;
    #[c2rust::src_loc = "159:1"]
    pub type u_int16_t = __uint16_t;
    #[c2rust::src_loc = "160:1"]
    pub type u_int32_t = __uint32_t;
    use super::types_h::{__u_int, __u_long, __ssize_t, __uint8_t, __uint16_t,
                         __uint32_t};
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:41"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:41"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int8_t, __int32_t};
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
#[c2rust::header_src = "/usr/include/bits/stat.h:42"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/include/db-int.h:54"]
pub mod db_int_h {
    #[c2rust::src_loc = "143:1"]
    pub type db_pgno_t = u_int32_t;
    #[c2rust::src_loc = "145:1"]
    pub type indx_t = u_int16_t;
    use super::sys_types_h::{u_int32_t, u_int16_t};
    /* _DB_INT_H_ */
    /* Needed for Win32 compiles */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/hash/hash.h:55"]
pub mod hash_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "48:16"]
    pub struct cursor_t {
        pub queue: C2RustUnnamed,
        pub get: Option<unsafe extern "C" fn(_: *const DB, _: *mut cursor_t,
                                             _: *mut DBT, _: *mut DBT,
                                             _: u_int32_t) -> libc::c_int>,
        pub delete: Option<unsafe extern "C" fn(_: *const DB,
                                                _: *mut cursor_t,
                                                _: u_int32_t) -> libc::c_int>,
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
    pub struct C2RustUnnamed {
        pub tqe_next: *mut cursor_t,
        pub tqe_prev: *mut *mut cursor_t,
    }
    #[c2rust::src_loc = "48:1"]
    pub type CURSOR = cursor_t;
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
    #[c2rust::src_loc = "66:1"]
    pub type HASHHDR = hashhdr;
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
    /* address of overflow page bitmaps */
    /* Shorthands for accessing structure */
    #[c2rust::src_loc = "171:1"]
    pub type ITEM_INFO = item_info;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "171:16"]
    pub struct item_info {
        pub pgno: db_pgno_t,
        pub bucket: db_pgno_t,
        pub ndx: indx_t,
        pub pgndx: indx_t,
        pub status: u_int8_t,
        pub seek_size: u_int32_t,
        pub seek_found_page: db_pgno_t,
        pub key_off: indx_t,
        pub data_off: indx_t,
        pub caused_expand: u_int8_t,
    }
    #[c2rust::src_loc = "43:9"]
    pub type ACTION = libc::c_uint;
    #[c2rust::src_loc = "44:60"]
    pub const HASH_NEXT: ACTION = 5;
    #[c2rust::src_loc = "44:48"]
    pub const HASH_FIRST: ACTION = 4;
    #[c2rust::src_loc = "44:35"]
    pub const HASH_DELETE: ACTION = 3;
    #[c2rust::src_loc = "44:22"]
    pub const HASH_PUTNEW: ACTION = 2;
    #[c2rust::src_loc = "44:12"]
    pub const HASH_PUT: ACTION = 1;
    #[c2rust::src_loc = "44:2"]
    pub const HASH_GET: ACTION = 0;
    use super::db_h::{DB, DBT};
    use super::sys_types_h::{u_int32_t, u_int16_t, u_int8_t};
    use super::db_int_h::{db_pgno_t, indx_t};
    use super::stddef_h::size_t;
    use super::stdint_intn_h::int32_t;
    use super::mpool_h::MPOOL;
    /* for num_items */
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
    use super::sys_types_h::{u_long, u_int8_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "105:1"]
        pub fn kdb2_mpool_open(_: *mut libc::c_void, _: libc::c_int,
                               _: db_pgno_t, _: db_pgno_t) -> *mut MPOOL;
        #[no_mangle]
        #[c2rust::src_loc = "112:1"]
        pub fn kdb2_mpool_sync(_: *mut MPOOL) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "113:1"]
        pub fn kdb2_mpool_close(_: *mut MPOOL) -> libc::c_int;
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
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/hash/page.h:56"]
pub mod page_h {
    #[c2rust::src_loc = "150:1"]
    pub type PAGE16 = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/sys/stat.h:42"]
pub mod sys_stat_h {
    use super::stat_h::stat;
    use super::types_h::__mode_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "205:1"]
        pub fn stat(__file: *const libc::c_char, __buf: *mut stat)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "280:1"]
        pub fn chmod(__file: *const libc::c_char, __mode: __mode_t)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:44"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:45"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "195:1"]
        pub fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "175:1"]
        pub fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:46"]
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
#[c2rust::header_src = "/usr/include/stdlib.h:47"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
    }
}
#[c2rust::header_src = "/usr/include/string.h:48"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:49"]
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
        #[c2rust::src_loc = "360:1"]
        pub fn read(__fd: libc::c_int, __buf: *mut libc::c_void,
                    __nbytes: size_t) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "366:1"]
        pub fn write(__fd: libc::c_int, __buf: *const libc::c_void,
                     __n: size_t) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "825:1"]
        pub fn unlink(__name: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/hash/extern.h:57"]
pub mod extern_h {
    use super::hash_h::{HTAB, ITEM_INFO, CURSOR};
    use super::db_h::DBT;
    use super::sys_types_h::{u_int32_t, u_int8_t};
    use super::stdint_intn_h::{int32_t, int8_t};
    use super::page_h::PAGE16;
    use super::db_int_h::db_pgno_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "71:1"]
        pub fn __kdb2_addel(_: *mut HTAB, _: *mut ITEM_INFO, _: *const DBT,
                            _: *const DBT, _: u_int32_t, _: u_int8_t)
         -> int32_t;
        #[no_mangle]
        #[c2rust::src_loc = "76:1"]
        pub fn __kdb2_big_keydata(_: *mut HTAB, _: *mut PAGE16, _: *mut DBT,
                                  _: *mut DBT, _: int32_t) -> int32_t;
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn __kdb2_big_return(_: *mut HTAB, _: *mut ITEM_INFO, _: *mut DBT,
                                 _: int32_t) -> int32_t;
        #[no_mangle]
        #[c2rust::src_loc = "81:1"]
        pub fn __kdb2_delpair(_: *mut HTAB, _: *mut CURSOR, _: *mut ITEM_INFO)
         -> int32_t;
        #[no_mangle]
        #[c2rust::src_loc = "83:1"]
        pub fn __kdb2_find_bigpair(_: *mut HTAB, _: *mut CURSOR,
                                   _: *mut int8_t, _: int32_t) -> int32_t;
        #[no_mangle]
        #[c2rust::src_loc = "88:1"]
        pub fn __kdb2_get_item_done(_: *mut HTAB, _: *mut CURSOR)
         -> u_int32_t;
        #[no_mangle]
        #[c2rust::src_loc = "89:1"]
        pub fn __kdb2_get_item_first(_: *mut HTAB, _: *mut CURSOR,
                                     _: *mut DBT, _: *mut DBT,
                                     _: *mut ITEM_INFO) -> u_int32_t;
        #[no_mangle]
        #[c2rust::src_loc = "90:1"]
        pub fn __kdb2_get_item_next(_: *mut HTAB, _: *mut CURSOR, _: *mut DBT,
                                    _: *mut DBT, _: *mut ITEM_INFO)
         -> u_int32_t;
        #[no_mangle]
        #[c2rust::src_loc = "91:1"]
        pub fn __kdb2_get_item_reset(_: *mut HTAB, _: *mut CURSOR)
         -> u_int32_t;
        #[no_mangle]
        #[c2rust::src_loc = "93:1"]
        pub fn __kdb2_ibitmap(_: *mut HTAB, _: int32_t, _: int32_t,
                              _: int32_t) -> int32_t;
        #[no_mangle]
        #[c2rust::src_loc = "94:1"]
        pub fn __kdb2_log2(_: u_int32_t) -> u_int32_t;
        #[no_mangle]
        #[c2rust::src_loc = "95:1"]
        pub fn __kdb2_new_page(_: *mut HTAB, _: u_int32_t, _: int32_t)
         -> int32_t;
        #[no_mangle]
        #[c2rust::src_loc = "96:1"]
        pub fn __kdb2_pgin_routine(_: *mut libc::c_void, _: db_pgno_t,
                                   _: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "97:1"]
        pub fn __kdb2_pgout_routine(_: *mut libc::c_void, _: db_pgno_t,
                                    _: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "99:1"]
        pub fn __kdb2_put_page(_: *mut HTAB, _: *mut PAGE16, _: int32_t,
                               _: int32_t) -> int32_t;
        #[no_mangle]
        #[c2rust::src_loc = "101:1"]
        pub fn __kdb2_split_page(_: *mut HTAB, _: u_int32_t, _: u_int32_t)
         -> int32_t;
        /* Default hash routine. */
        #[no_mangle]
        #[c2rust::src_loc = "104:20"]
        pub static mut __default_hash:
                   Option<unsafe extern "C" fn(_: *const libc::c_void,
                                               _: size_t) -> u_int32_t>;
    }
}
pub use self::types_h::{__u_int, __u_long, __int8_t, __uint8_t, __uint16_t,
                        __int32_t, __uint32_t, __dev_t, __uid_t, __gid_t,
                        __ino_t, __mode_t, __nlink_t, __off_t, __off64_t,
                        __time_t, __blksize_t, __blkcnt_t, __ssize_t,
                        __syscall_slong_t};
pub use self::sys_types_h::{u_int, u_long, ssize_t, u_int8_t, u_int16_t,
                            u_int32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int8_t, int32_t};
pub use self::struct_timespec_h::timespec;
pub use self::stat_h::stat;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::db_h::{DBT, DBTYPE, DB_RECNO, DB_HASH, DB_BTREE, __db, DB,
                     HASHINFO};
pub use self::db_int_h::{db_pgno_t, indx_t};
pub use self::hash_h::{cursor_t, C2RustUnnamed, CURSOR, HTAB, htab, HASHHDR,
                       hashhdr, _cursor_queue, ITEM_INFO, item_info, ACTION,
                       HASH_NEXT, HASH_FIRST, HASH_DELETE, HASH_PUTNEW,
                       HASH_PUT, HASH_GET};
pub use self::mpool_h::{MPOOL, _hqh, _bkt, C2RustUnnamed_0, C2RustUnnamed_1,
                        _lqh, kdb2_mpool_open, kdb2_mpool_sync,
                        kdb2_mpool_close, kdb2_mpool_filter};
pub use self::page_h::PAGE16;
use self::sys_stat_h::{stat, chmod};
use self::errno_h::__errno_location;
use self::fcntl_h::{open, fcntl};
use self::stdio_h::{stderr, fprintf};
use self::stdlib_h::{malloc, calloc, free, abort};
use self::string_h::{memset, memcmp, strlen};
use self::unistd_h::{lseek, close, read, write, unlink};
use self::extern_h::{__kdb2_addel, __kdb2_big_keydata, __kdb2_big_return,
                     __kdb2_delpair, __kdb2_find_bigpair,
                     __kdb2_get_item_done, __kdb2_get_item_first,
                     __kdb2_get_item_next, __kdb2_get_item_reset,
                     __kdb2_ibitmap, __kdb2_log2, __kdb2_new_page,
                     __kdb2_pgin_routine, __kdb2_pgout_routine,
                     __kdb2_put_page, __kdb2_split_page, __default_hash};
/* ************************* INTERFACE ROUTINES ***************************/
/* OPEN/CLOSE */
#[no_mangle]
#[c2rust::src_loc = "96:1"]
pub unsafe extern "C" fn __kdb2_hash_open(mut file: *const libc::c_char,
                                          mut flags: libc::c_int,
                                          mut mode: libc::c_int,
                                          mut info: *const HASHINFO,
                                          mut dflags: libc::c_int) -> *mut DB 
 /* Special directives for create */
 {
    let mut current_block: u64;
    let mut statbuf: stat =
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
    let mut dbp: *mut DB = 0 as *mut DB;
    let mut mpool_key: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    let mut hashp: *mut HTAB = 0 as *mut HTAB;
    let mut bpages: int32_t = 0;
    let mut csize: int32_t = 0;
    let mut new_table: int32_t = 0;
    let mut save_errno: int32_t = 0;
    if file.is_null() || flags & 0o3 as libc::c_int == 0o1 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut DB
    }
    hashp =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<HTAB>() as libc::c_ulong) as *mut HTAB;
    if hashp.is_null() { return 0 as *mut DB }
    (*hashp).fp = -(1 as libc::c_int);
    /*
	 * Even if user wants write only, we need to be able to read
	 * the actual file, so we need to open it read/write. But, the
	 * field in the hashp structure needs to be accurate so that
	 * we can check accesses.
	 */
    (*hashp).flags = flags; /* In case someone looks at errno. */
    (*hashp).save_file = (*hashp).flags & 0o2 as libc::c_int;
    new_table = 0 as libc::c_int;
    if file.is_null() || flags & 0o1000 as libc::c_int != 0 ||
           stat(file, &mut statbuf) != 0 &&
               *__errno_location() == 2 as libc::c_int {
        if *__errno_location() == 2 as libc::c_int {
            *__errno_location() = 0 as libc::c_int
        }
        new_table = 1 as libc::c_int
    }
    if !file.is_null() {
        (*hashp).fp = open(file, flags | 0 as libc::c_int, mode);
        if (*hashp).fp == -(1 as libc::c_int) {
            save_errno = *__errno_location();
            current_block = 17443757415349084853;
        } else {
            fcntl((*hashp).fp, 2 as libc::c_int, 1 as libc::c_int);
            current_block = 5601891728916014340;
        }
    } else { current_block = 5601891728916014340; }
    match current_block {
        5601891728916014340 =>
        /* Process arguments to set up hash table header. */
        {
            if new_table != 0 {
                hashp = init_hash(hashp, file, info);
                if hashp.is_null() {
                    save_errno = *__errno_location();
                    current_block = 16212105214275517957;
                } else { current_block = 6717214610478484138; }
            } else {
                /* Table already exists */
                if !info.is_null() && (*info).hash.is_some() {
                    (*hashp).hash = (*info).hash
                } else { (*hashp).hash = __default_hash }
                /* copy metadata from page into header */
                if hget_header(hashp,
                               (if !info.is_null() && (*info).bsize != 0 {
                                    (*info).bsize
                                } else {
                                    ((1 as libc::c_int) << 12 as libc::c_int)
                                        as libc::c_uint
                                })) as libc::c_ulong !=
                       ::std::mem::size_of::<HASHHDR>() as libc::c_ulong {
                    save_errno = 22 as libc::c_int;
                    current_block = 16212105214275517957;
                } else if (*hashp).hdr.magic != 0x61561 as libc::c_int {
                    save_errno = 22 as libc::c_int;
                    current_block = 16212105214275517957;
                } else if (*hashp).hdr.version != 3 as libc::c_int &&
                              (*hashp).hdr.version != 1 as libc::c_int {
                    save_errno = 22 as libc::c_int;
                    current_block = 16212105214275517957;
                } else if (*hashp).hash.expect("non-null function pointer")(b"%$sniglet^&\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char
                                                                                as
                                                                                *const libc::c_void,
                                                                            ::std::mem::size_of::<[libc::c_char; 12]>()
                                                                                as
                                                                                libc::c_ulong)
                              != (*hashp).hdr.h_charkey {
                    save_errno = 22 as libc::c_int;
                    current_block = 16212105214275517957;
                } else {
                    /* Verify file type, versions and hash function */
                    /*
		 * Figure out how many segments we need.  Max_Bucket is the
		 * maximum bucket number, so the number of buckets is
		 * max_bucket + 1.
		 */
                    /* Read in bitmaps */
                    bpages =
                        ((*hashp).hdr.spares[(*hashp).hdr.ovfl_point as
                                                 usize].wrapping_add((*hashp).hdr.bsize
                                                                         <<
                                                                         3 as
                                                                             libc::c_int).wrapping_sub(1
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_uint)
                             >> (*hashp).hdr.bshift + 3 as libc::c_int) as
                            int32_t;
                    (*hashp).nmaps = bpages;
                    memset(&mut *(*hashp).mapp.as_mut_ptr().offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                               as *mut *mut u_int32_t as *mut libc::c_void,
                           0 as libc::c_int,
                           (bpages as
                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut u_int32_t>()
                                                                as
                                                                libc::c_ulong));
                    current_block = 6717214610478484138;
                }
            }
            match current_block {
                6717214610478484138 => {
                    /* start up mpool */
                    mpool_key.data =
                        file as *mut u_int8_t as *mut libc::c_void;
                    mpool_key.size = strlen(file);
                    if !info.is_null() && (*info).cachesize != 0 {
                        csize =
                            (*info).cachesize.wrapping_div((*hashp).hdr.bsize)
                                as int32_t
                    } else {
                        csize =
                            (65536 as libc::c_int as
                                 libc::c_uint).wrapping_div((*hashp).hdr.bsize)
                                as int32_t
                    }
                    (*hashp).mp =
                        kdb2_mpool_open(&mut mpool_key as *mut DBT as
                                            *mut libc::c_void, (*hashp).fp,
                                        (*hashp).hdr.bsize,
                                        csize as db_pgno_t);
                    if (*hashp).mp.is_null() {
                        save_errno = *__errno_location()
                    } else {
                        kdb2_mpool_filter((*hashp).mp,
                                          Some(__kdb2_pgin_routine as
                                                   unsafe extern "C" fn(_:
                                                                            *mut libc::c_void,
                                                                        _:
                                                                            db_pgno_t,
                                                                        _:
                                                                            *mut libc::c_void)
                                                       -> ()),
                                          Some(__kdb2_pgout_routine as
                                                   unsafe extern "C" fn(_:
                                                                            *mut libc::c_void,
                                                                        _:
                                                                            db_pgno_t,
                                                                        _:
                                                                            *mut libc::c_void)
                                                       -> ()),
                                          hashp as *mut libc::c_void);
                        /*
	 * For a new table, set up the bitmaps.
	 */
                        if !(new_table != 0 &&
                                 init_htab(hashp,
                                           (if !info.is_null() &&
                                                   (*info).nelem != 0 {
                                                (*info).nelem
                                            } else {
                                                1 as libc::c_int as
                                                    libc::c_uint
                                            }) as int32_t) != 0) {
                            /* initialize the cursor queue */
                            (*hashp).curs_queue.tqh_first =
                                0 as *mut cursor_t;
                            (*hashp).curs_queue.tqh_last =
                                &mut (*hashp).curs_queue.tqh_first;
                            (*hashp).seq_cursor = 0 as *mut CURSOR;
                            /* get a chunk of memory for our split buffer */
                            (*hashp).split_buf =
                                malloc((*hashp).hdr.bsize as libc::c_ulong) as
                                    *mut PAGE16;
                            if !(*hashp).split_buf.is_null() {
                                (*hashp).new_file = new_table;
                                dbp =
                                    malloc(::std::mem::size_of::<DB>() as
                                               libc::c_ulong) as *mut DB;
                                if !dbp.is_null() {
                                    (*dbp).internal =
                                        hashp as *mut libc::c_void;
                                    (*dbp).close =
                                        Some(hash_close as
                                                 unsafe extern "C" fn(_:
                                                                          *mut DB)
                                                     -> int32_t);
                                    (*dbp).del =
                                        Some(hash_delete as
                                                 unsafe extern "C" fn(_:
                                                                          *const DB,
                                                                      _:
                                                                          *const DBT,
                                                                      _:
                                                                          u_int32_t)
                                                     -> int32_t);
                                    (*dbp).fd =
                                        Some(hash_fd as
                                                 unsafe extern "C" fn(_:
                                                                          *const DB)
                                                     -> int32_t);
                                    (*dbp).get =
                                        Some(hash_get as
                                                 unsafe extern "C" fn(_:
                                                                          *const DB,
                                                                      _:
                                                                          *const DBT,
                                                                      _:
                                                                          *mut DBT,
                                                                      _:
                                                                          u_int32_t)
                                                     -> int32_t);
                                    (*dbp).put =
                                        Some(hash_put as
                                                 unsafe extern "C" fn(_:
                                                                          *const DB,
                                                                      _:
                                                                          *mut DBT,
                                                                      _:
                                                                          *const DBT,
                                                                      _:
                                                                          u_int32_t)
                                                     -> int32_t);
                                    (*dbp).seq =
                                        Some(hash_seq as
                                                 unsafe extern "C" fn(_:
                                                                          *const DB,
                                                                      _:
                                                                          *mut DBT,
                                                                      _:
                                                                          *mut DBT,
                                                                      _:
                                                                          u_int32_t)
                                                     -> int32_t);
                                    (*dbp).sync =
                                        Some(hash_sync as
                                                 unsafe extern "C" fn(_:
                                                                          *const DB,
                                                                      _:
                                                                          u_int32_t)
                                                     -> int32_t);
                                    (*dbp).type_0 = DB_HASH;
                                    return dbp
                                }
                            }
                        }
                        save_errno = *__errno_location();
                        hdestroy(hashp);
                        *__errno_location() = save_errno;
                        return 0 as *mut DB
                    }
                }
                _ => { }
            }
            if !hashp.is_null() { close((*hashp).fp); }
        }
        _ => { }
    }
    free(hashp as *mut libc::c_void);
    *__errno_location() = save_errno;
    return 0 as *mut DB;
}
#[c2rust::src_loc = "262:1"]
unsafe extern "C" fn hash_close(mut dbp: *mut DB) -> int32_t {
    let mut hashp: *mut HTAB = 0 as *mut HTAB;
    let mut retval: int32_t = 0;
    if dbp.is_null() { return -(1 as libc::c_int) }
    hashp = (*dbp).internal as *mut HTAB;
    retval = hdestroy(hashp);
    free(dbp as *mut libc::c_void);
    return retval;
}
#[c2rust::src_loc = "278:1"]
unsafe extern "C" fn hash_fd(mut dbp: *const DB) -> int32_t {
    let mut hashp: *mut HTAB = 0 as *mut HTAB;
    if dbp.is_null() { return -(1 as libc::c_int) }
    hashp = (*dbp).internal as *mut HTAB;
    if (*hashp).fp == -(1 as libc::c_int) {
        *__errno_location() = 2 as libc::c_int;
        return -(1 as libc::c_int)
    }
    return (*hashp).fp;
}
/* ************************* LOCAL CREATION ROUTINES **********************/
#[c2rust::src_loc = "296:1"]
unsafe extern "C" fn init_hash(mut hashp: *mut HTAB,
                               mut file: *const libc::c_char,
                               mut info: *const HASHINFO) -> *mut HTAB {
    let mut statbuf: stat =
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
    (*hashp).hdr.nkeys = 0 as libc::c_int;
    (*hashp).hdr.lorder = 1234 as libc::c_int;
    (*hashp).hdr.bsize =
        ((1 as libc::c_int) << 12 as libc::c_int) as u_int32_t;
    (*hashp).hdr.bshift = 12 as libc::c_int;
    (*hashp).hdr.ffactor = 65536 as libc::c_int as u_int32_t;
    (*hashp).hash = __default_hash;
    memset((*hashp).hdr.spares.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[u_int32_t; 32]>() as libc::c_ulong);
    memset((*hashp).hdr.bitmaps.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[u_int16_t; 32]>() as libc::c_ulong);
    /* Fix bucket size to be optimal for file system */
    if !file.is_null() {
        if stat(file, &mut statbuf) != 0 { return 0 as *mut HTAB }
        (*hashp).hdr.bsize = statbuf.st_blksize as u_int32_t;
        if (*hashp).hdr.bsize > 65536 as libc::c_int as libc::c_uint {
            (*hashp).hdr.bsize = 65536 as libc::c_int as u_int32_t
        }
        (*hashp).hdr.bshift = __kdb2_log2((*hashp).hdr.bsize) as int32_t
    }
    if !info.is_null() {
        if (*info).bsize != 0 {
            /* Round pagesize up to power of 2 */
            (*hashp).hdr.bshift = __kdb2_log2((*info).bsize) as int32_t;
            (*hashp).hdr.bsize =
                ((1 as libc::c_int) << (*hashp).hdr.bshift) as u_int32_t;
            if (*hashp).hdr.bsize > 65536 as libc::c_int as libc::c_uint {
                *__errno_location() = 22 as libc::c_int;
                return 0 as *mut HTAB
            }
        }
        if (*info).ffactor != 0 { (*hashp).hdr.ffactor = (*info).ffactor }
        if (*info).hash.is_some() { (*hashp).hash = (*info).hash }
        if (*info).lorder != 0 {
            if (*info).lorder != 4321 as libc::c_int &&
                   (*info).lorder != 1234 as libc::c_int {
                *__errno_location() = 22 as libc::c_int;
                return 0 as *mut HTAB
            }
            (*hashp).hdr.lorder = (*info).lorder
        }
    }
    return hashp;
}
/*
 * Returns 0 on No Error
 */
#[c2rust::src_loc = "351:1"]
unsafe extern "C" fn init_htab(mut hashp: *mut HTAB, mut nelem: int32_t)
 -> int32_t {
    let mut l2: int32_t = 0;
    let mut nbuckets: int32_t = 0;
    /*
	 * Divide number of elements by the fill factor and determine a
	 * desired number of buckets.  Allocate space for the next greater
	 * power of two number of buckets.
	 */
    nelem =
        ((nelem - 1 as libc::c_int) as
             libc::c_uint).wrapping_div((*hashp).hdr.ffactor).wrapping_add(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint)
            as int32_t;
    l2 =
        __kdb2_log2(if nelem > 2 as libc::c_int {
                        nelem
                    } else { 2 as libc::c_int } as u_int32_t) as int32_t;
    nbuckets = (1 as libc::c_int) << l2;
    (*hashp).hdr.spares[l2 as usize] = (l2 + 1 as libc::c_int) as u_int32_t;
    (*hashp).hdr.spares[(l2 + 1 as libc::c_int) as usize] =
        (l2 + 1 as libc::c_int) as u_int32_t;
    (*hashp).hdr.ovfl_point = l2;
    (*hashp).hdr.last_freed = 2 as libc::c_int as u_int32_t;
    (*hashp).hdr.low_mask = (nbuckets - 1 as libc::c_int) as u_int32_t;
    (*hashp).hdr.max_bucket = (*hashp).hdr.low_mask;
    (*hashp).hdr.high_mask =
        ((nbuckets << 1 as libc::c_int) - 1 as libc::c_int) as u_int32_t;
    /*
	 * The number of header pages is the size of the header divided by
	 * the amount of freespace on header pages (the page size - the
	 * size of 1 integer where the length of the header info on that
	 * page is stored) plus another page if it didn't divide evenly.
	 */
    (*hashp).hdr.hdrpages =
        (::std::mem::size_of::<HASHHDR>() as
             libc::c_ulong).wrapping_div((*hashp).hdr.bsize.wrapping_sub(4 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                             as
                                             libc::c_ulong).wrapping_add((if (::std::mem::size_of::<HASHHDR>()
                                                                                  as
                                                                                  libc::c_ulong).wrapping_rem((*hashp).hdr.bsize.wrapping_sub(4
                                                                                                                                                  as
                                                                                                                                                  libc::c_int
                                                                                                                                                  as
                                                                                                                                                  libc::c_uint)
                                                                                                                  as
                                                                                                                  libc::c_ulong)
                                                                                 ==
                                                                                 0
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong
                                                                             {
                                                                              0
                                                                                  as
                                                                                  libc::c_int
                                                                          } else {
                                                                              1
                                                                                  as
                                                                                  libc::c_int
                                                                          })
                                                                             as
                                                                             libc::c_ulong)
            as u_int32_t;
    /* Create pages for these buckets */
	/*
	for (i = 0; i <= hashp->hdr.max_bucket; i++) {
		if (__new_page(hashp, (u_int32_t)i, A_BUCKET) != 0)
			return (-1);
	}
	*/
    /* First bitmap page is at: splitpoint l2 page offset 1 */
    if __kdb2_ibitmap(hashp,
                      ((l2 as u_int32_t) <<
                           11 as
                               libc::c_int).wrapping_add(1 as libc::c_int as
                                                             libc::c_uint) as
                          int32_t, l2 + 1 as libc::c_int, 0 as libc::c_int) !=
           0 {
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
/*
 * Functions to get/put hash header.  We access the file directly.
 */
#[c2rust::src_loc = "405:1"]
unsafe extern "C" fn hget_header(mut hashp: *mut HTAB,
                                 mut page_size: u_int32_t) -> u_int32_t {
    let mut num_copied: u_int32_t = 0;
    let mut hdr_dest: *mut u_int8_t = 0 as *mut u_int8_t;
    num_copied = 0 as libc::c_int as u_int32_t;
    hdr_dest = &mut (*hashp).hdr as *mut HASHHDR as *mut u_int8_t;
    /*
	 * XXX
	 * This should not be printing to stderr on a "normal" error case.
	 */
    lseek((*hashp).fp, 0 as libc::c_int as __off_t, 0 as libc::c_int);
    num_copied =
        read((*hashp).fp, hdr_dest as *mut libc::c_void,
             ::std::mem::size_of::<HASHHDR>() as libc::c_ulong) as u_int32_t;
    if num_copied as libc::c_ulong !=
           ::std::mem::size_of::<HASHHDR>() as libc::c_ulong {
        fprintf(stderr,
                b"hash: could not retrieve header\x00" as *const u8 as
                    *const libc::c_char);
        return 0 as libc::c_int as u_int32_t
    }
    swap_header(hashp);
    return num_copied;
}
#[c2rust::src_loc = "433:1"]
unsafe extern "C" fn hput_header(mut hashp: *mut HTAB) {
    let mut whdrp: *mut HASHHDR = 0 as *mut HASHHDR;
    let mut whdr: HASHHDR =
        HASHHDR{magic: 0,
                version: 0,
                lorder: 0,
                bsize: 0,
                bshift: 0,
                ovfl_point: 0,
                last_freed: 0,
                max_bucket: 0,
                high_mask: 0,
                low_mask: 0,
                ffactor: 0,
                nkeys: 0,
                hdrpages: 0,
                h_charkey: 0,
                spares: [0; 32],
                bitmaps: [0; 32],};
    let mut num_copied: u_int32_t = 0;
    num_copied = 0 as libc::c_int as u_int32_t;
    whdrp = &mut (*hashp).hdr;
    whdrp = &mut whdr;
    swap_header_copy(&mut (*hashp).hdr, whdrp);
    lseek((*hashp).fp, 0 as libc::c_int as __off_t, 0 as libc::c_int);
    num_copied =
        write((*hashp).fp, whdrp as *const libc::c_void,
              ::std::mem::size_of::<HASHHDR>() as libc::c_ulong) as u_int32_t;
    if num_copied as libc::c_ulong !=
           ::std::mem::size_of::<HASHHDR>() as libc::c_ulong {
        fprintf(stderr,
                b"hash: could not write hash header\x00" as *const u8 as
                    *const libc::c_char);
    };
}
/* ********************* DESTROY/CLOSE ROUTINES ************************/
/*
 * Flushes any changes to the file if necessary and destroys the hashp
 * structure, freeing all allocated space.
 */
#[c2rust::src_loc = "464:1"]
unsafe extern "C" fn hdestroy(mut hashp: *mut HTAB) -> int32_t {
    let mut save_errno: int32_t = 0;
    save_errno = 0 as libc::c_int;
    if flush_meta(hashp) != 0 && save_errno == 0 {
        save_errno = *__errno_location()
    }
    /* Free the split page */
    if !(*hashp).split_buf.is_null() {
        free((*hashp).split_buf as *mut libc::c_void);
    }
    /* Free the big key and big data returns */
    if !(*hashp).bigkey_buf.is_null() {
        free((*hashp).bigkey_buf as *mut libc::c_void);
    }
    if !(*hashp).bigdata_buf.is_null() {
        free((*hashp).bigdata_buf as *mut libc::c_void);
    }
    /* XXX This should really iterate over the cursor queue, but
	   it's not clear how to do that, and the only cursor a hash
	   table ever creates is the one used by hash_seq().  Passing
	   NULL as the first arg is also a kludge, but I know that
	   it's never used, so I do it.  The intent is to plug the
	   memory leak.  Correctness can come later. */
    if !(*hashp).seq_cursor.is_null() {
        (*(*hashp).seq_cursor).delete.expect("non-null function pointer")(0 as
                                                                              *const DB,
                                                                          (*hashp).seq_cursor,
                                                                          0 as
                                                                              libc::c_int
                                                                              as
                                                                              u_int32_t);
    }
    /* shut down mpool */
    kdb2_mpool_sync((*hashp).mp);
    kdb2_mpool_close((*hashp).mp);
    if (*hashp).fp != -(1 as libc::c_int) { close((*hashp).fp); }
    /*
	 * *** This may cause problems if hashp->fname is set in any case
	 * other than the case that we are generating a temporary file name.
	 * Note that the new version of mpool should support temporary
	 * files within mpool itself.
	 */
    if !(*hashp).fname.is_null() && (*hashp).save_file == 0 {
        /* we need to chmod the file to allow it to be deleted... */
        chmod((*hashp).fname, 0o700 as libc::c_int as __mode_t);
        unlink((*hashp).fname);
    }
    free(hashp as *mut libc::c_void);
    if save_errno != 0 {
        *__errno_location() = save_errno;
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
/*
 * Write modified pages to disk
 *
 * Returns:
 *	 0 == OK
 *	-1 ERROR
 */
#[c2rust::src_loc = "551:1"]
unsafe extern "C" fn hash_sync(mut dbp: *const DB, mut flags: u_int32_t)
 -> int32_t {
    let mut hashp: *mut HTAB = 0 as *mut HTAB;
    hashp = (*dbp).internal as *mut HTAB;
    /*
	 * XXX
	 * Check success/failure conditions.
	 */
    return (flush_meta(hashp) != 0 || kdb2_mpool_sync((*hashp).mp) != 0) as
               libc::c_int;
}
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
 */
/* LIBC_SCCS and not lint */
/*
 * Returns:
 *	 0 == OK
 *	-1 indicates that errno should be set
 */
#[c2rust::src_loc = "572:1"]
unsafe extern "C" fn flush_meta(mut hashp: *mut HTAB) -> int32_t {
    let mut i: int32_t = 0;
    if (*hashp).save_file == 0 { return 0 as libc::c_int }
    (*hashp).hdr.magic = 0x61561 as libc::c_int;
    (*hashp).hdr.version = 3 as libc::c_int;
    (*hashp).hdr.h_charkey =
        (*hashp).hash.expect("non-null function pointer")(b"%$sniglet^&\x00"
                                                              as *const u8 as
                                                              *const libc::c_char
                                                              as
                                                              *const libc::c_void,
                                                          ::std::mem::size_of::<[libc::c_char; 12]>()
                                                              as
                                                              libc::c_ulong);
    /* write out metadata */
    hput_header(hashp);
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if !(*hashp).mapp[i as usize].is_null() {
            if __kdb2_put_page(hashp,
                               (*hashp).mapp[i as usize] as *mut PAGE16,
                               2 as libc::c_int, 1 as libc::c_int) != 0 {
                return -(1 as libc::c_int)
            }
            (*hashp).mapp[i as usize] = 0 as *mut u_int32_t
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/* ******************************SEARCH ROUTINES *****************************/
/*
 * All the access routines return
 *
 * Returns:
 *	 0 on SUCCESS
 *	 1 to indicate an external ERROR (i.e. key not found, etc)
 *	-1 to indicate an internal ERROR (i.e. out of memory, etc)
 */
/* *** make sure this is true! */
#[c2rust::src_loc = "609:1"]
unsafe extern "C" fn hash_get(mut dbp: *const DB, mut key: *const DBT,
                              mut data: *mut DBT, mut flag: u_int32_t)
 -> int32_t {
    let mut hashp: *mut HTAB = 0 as *mut HTAB;
    hashp = (*dbp).internal as *mut HTAB;
    if flag != 0 {
        let ref mut fresh0 = *__errno_location();
        *fresh0 = 22 as libc::c_int;
        (*hashp).local_errno = *fresh0;
        return -(1 as libc::c_int)
    }
    return hash_access(hashp, HASH_GET, key, data);
}
#[c2rust::src_loc = "626:1"]
unsafe extern "C" fn hash_put(mut dbp: *const DB, mut key: *mut DBT,
                              mut data: *const DBT, mut flag: u_int32_t)
 -> int32_t {
    let mut hashp: *mut HTAB = 0 as *mut HTAB;
    hashp = (*dbp).internal as *mut HTAB;
    if flag != 0 && flag != 8 as libc::c_int as libc::c_uint {
        let ref mut fresh1 = *__errno_location();
        *fresh1 = 22 as libc::c_int;
        (*hashp).local_errno = *fresh1;
        return -(1 as libc::c_int)
    }
    if (*hashp).flags & 0o3 as libc::c_int == 0 as libc::c_int {
        let ref mut fresh2 = *__errno_location();
        *fresh2 = 1 as libc::c_int;
        (*hashp).local_errno = *fresh2;
        return -(1 as libc::c_int)
    }
    return hash_access(hashp,
                       if flag == 8 as libc::c_int as libc::c_uint {
                           HASH_PUTNEW as libc::c_int
                       } else { HASH_PUT as libc::c_int } as ACTION, key,
                       data as *mut DBT);
}
#[c2rust::src_loc = "648:1"]
unsafe extern "C" fn hash_delete(mut dbp: *const DB, mut key: *const DBT,
                                 mut flag: u_int32_t) -> int32_t 
 /* Ignored */
 {
    let mut hashp: *mut HTAB = 0 as *mut HTAB;
    hashp = (*dbp).internal as *mut HTAB;
    if flag != 0 {
        let ref mut fresh3 = *__errno_location();
        *fresh3 = 22 as libc::c_int;
        (*hashp).local_errno = *fresh3;
        return -(1 as libc::c_int)
    }
    if (*hashp).flags & 0o3 as libc::c_int == 0 as libc::c_int {
        let ref mut fresh4 = *__errno_location();
        *fresh4 = 1 as libc::c_int;
        (*hashp).local_errno = *fresh4;
        return -(1 as libc::c_int)
    }
    return hash_access(hashp, HASH_DELETE, key, 0 as *mut DBT);
}
/*
 * Assume that hashp has been set in wrapper routine.
 */
#[c2rust::src_loc = "672:1"]
unsafe extern "C" fn hash_access(mut hashp: *mut HTAB, mut action: ACTION,
                                 mut key: *const DBT, mut val: *mut DBT)
 -> int32_t {
    let mut current_block: u64;
    let mut page_key: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    let mut page_val: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    let mut cursor: CURSOR =
        CURSOR{queue:
                   C2RustUnnamed{tqe_next: 0 as *mut cursor_t,
                                 tqe_prev: 0 as *mut *mut cursor_t,},
               get: None,
               delete: None,
               bucket: 0,
               pgno: 0,
               ndx: 0,
               pgndx: 0,
               pagep: 0 as *mut u_int16_t,
               internal: 0 as *mut libc::c_void,};
    let mut item_info: ITEM_INFO =
        ITEM_INFO{pgno: 0,
                  bucket: 0,
                  ndx: 0,
                  pgndx: 0,
                  status: 0,
                  seek_size: 0,
                  seek_found_page: 0,
                  key_off: 0,
                  data_off: 0,
                  caused_expand: 0,};
    let mut bucket: u_int32_t = 0;
    let mut num_items: u_int32_t = 0;
    num_items = 0 as libc::c_int as u_int32_t;
    /*
	 * Set up item_info so that we're looking for space to add an item
	 * as we cycle through the pages looking for the key.
	 */
    if action as libc::c_uint == HASH_PUT as libc::c_int as libc::c_uint ||
           action as libc::c_uint ==
               HASH_PUTNEW as libc::c_int as libc::c_uint {
        if if (*key).size.wrapping_add((*val).size) as libc::c_double >
                  (*hashp).hdr.bsize as libc::c_double * 0.75f64 {
               1 as libc::c_int
           } else { 0 as libc::c_int } != 0 {
            item_info.seek_size =
                ((::std::mem::size_of::<indx_t>() as libc::c_ulong) <<
                     1 as libc::c_int) as u_int32_t
        } else {
            item_info.seek_size =
                (*key).size.wrapping_add((*val).size) as u_int32_t
        }
    } else { item_info.seek_size = 0 as libc::c_int as u_int32_t }
    item_info.seek_found_page = 0 as libc::c_int as db_pgno_t;
    bucket =
        __kdb2_call_hash(hashp, (*key).data as *mut int8_t,
                         (*key).size as int32_t);
    cursor.pagep = 0 as *mut u_int16_t;
    __kdb2_get_item_reset(hashp, &mut cursor);
    cursor.bucket = bucket;
    loop  {
        __kdb2_get_item_next(hashp, &mut cursor, &mut page_key, &mut page_val,
                             &mut item_info);
        if item_info.status as libc::c_int == 0 as libc::c_int {
            return 1 as libc::c_int
        }
        if item_info.status as libc::c_int == 2 as libc::c_int {
            current_block = 5634871135123216486;
            break ;
        }
        num_items = num_items.wrapping_add(1);
        if item_info.key_off as libc::c_int == 0 as libc::c_int {
            /*
			 * !!!
			 * 0 is a valid index.
			 */
            if __kdb2_find_bigpair(hashp, &mut cursor,
                                   (*key).data as *mut int8_t,
                                   (*key).size as int32_t) > 0 as libc::c_int
               {
                current_block = 11864735870557481929;
                break ;
            }
        } else if (*key).size == page_key.size &&
                      memcmp((*key).data, page_key.data, (*key).size) == 0 {
            current_block = 11864735870557481929;
            break ;
        }
    }
    match current_block {
        11864735870557481929 => {
            __kdb2_get_item_done(hashp, &mut cursor);
            match action as libc::c_uint {
                2 => {
                    /* mpool_put(hashp->mp, pagep, 0); */
                    return 1 as libc::c_int
                }
                0 => {
                    if item_info.key_off as libc::c_int == 0 as libc::c_int {
                        if __kdb2_big_return(hashp, &mut item_info, val,
                                             0 as libc::c_int) != 0 {
                            return -(1 as libc::c_int)
                        }
                    } else {
                        (*val).data = page_val.data;
                        (*val).size = page_val.size
                    }
                }
                1 => {
                    if __kdb2_delpair(hashp, &mut cursor, &mut item_info) != 0
                           ||
                           __kdb2_addel(hashp, &mut item_info, key, val,
                                        0xffffffff as libc::c_uint,
                                        0 as libc::c_int as u_int8_t) != 0 {
                        return -(1 as libc::c_int)
                    }
                    __kdb2_get_item_done(hashp, &mut cursor);
                    if item_info.caused_expand != 0 {
                        __kdb2_expand_table(hashp);
                    }
                }
                3 => {
                    if __kdb2_delpair(hashp, &mut cursor, &mut item_info) != 0
                       {
                        return -(1 as libc::c_int)
                    }
                }
                _ => { abort(); }
            }
            return 0 as libc::c_int
        }
        _ => {
            __kdb2_get_item_done(hashp, &mut cursor);
            /*
	 * At this point, item_info will list either the last page in
	 * the chain, or the last page in the chain plus a pgno for where
	 * to find the first page in the chain with space for the
	 * item we wish to add.
	 */
            /* Not found */
            match action as libc::c_uint {
                1 | 2 => {
                    if __kdb2_addel(hashp, &mut item_info, key, val,
                                    num_items, 0 as libc::c_int as u_int8_t)
                           != 0 {
                        return -(1 as libc::c_int)
                    }
                }
                0 | 3 | _ => { return 1 as libc::c_int }
            }
            if item_info.caused_expand != 0 { __kdb2_expand_table(hashp); }
            return 0 as libc::c_int
        }
    };
}
/* ****************** CURSORS ********************************** */
#[no_mangle]
#[c2rust::src_loc = "793:1"]
pub unsafe extern "C" fn __kdb2_cursor_creat(mut dbp: *const DB)
 -> *mut CURSOR {
    let mut new_curs: *mut CURSOR = 0 as *mut CURSOR;
    let mut hashp: *mut HTAB = 0 as *mut HTAB;
    new_curs =
        malloc(::std::mem::size_of::<cursor_t>() as libc::c_ulong) as
            *mut CURSOR;
    if new_curs.is_null() { return 0 as *mut CURSOR }
    (*new_curs).internal =
        malloc(::std::mem::size_of::<item_info>() as libc::c_ulong) as
            *mut item_info as *mut libc::c_void;
    if (*new_curs).internal.is_null() {
        free(new_curs as *mut libc::c_void);
        return 0 as *mut CURSOR
    }
    (*new_curs).get =
        Some(cursor_get as
                 unsafe extern "C" fn(_: *const DB, _: *mut CURSOR,
                                      _: *mut DBT, _: *mut DBT, _: u_int32_t)
                     -> int32_t);
    (*new_curs).delete =
        Some(cursor_delete as
                 unsafe extern "C" fn(_: *const DB, _: *mut CURSOR,
                                      _: u_int32_t) -> int32_t);
    (*new_curs).bucket = 0 as libc::c_int as db_pgno_t;
    (*new_curs).pgno = 0xffffffff as libc::c_uint;
    (*new_curs).ndx = 0 as libc::c_int as indx_t;
    (*new_curs).pgndx = 0 as libc::c_int as indx_t;
    (*new_curs).pagep = 0 as *mut u_int16_t;
    /* place onto queue of cursors */
    hashp = (*dbp).internal as *mut HTAB;
    (*new_curs).queue.tqe_next = 0 as *mut cursor_t;
    (*new_curs).queue.tqe_prev = (*hashp).curs_queue.tqh_last;
    *(*hashp).curs_queue.tqh_last = new_curs;
    (*hashp).curs_queue.tqh_last = &mut (*new_curs).queue.tqe_next;
    return new_curs;
}
#[c2rust::src_loc = "825:1"]
unsafe extern "C" fn cursor_get(mut dbp: *const DB, mut cursorp: *mut CURSOR,
                                mut key: *mut DBT, mut val: *mut DBT,
                                mut flags: u_int32_t) -> int32_t {
    let mut hashp: *mut HTAB = 0 as *mut HTAB;
    let mut item_info: ITEM_INFO =
        ITEM_INFO{pgno: 0,
                  bucket: 0,
                  ndx: 0,
                  pgndx: 0,
                  status: 0,
                  seek_size: 0,
                  seek_found_page: 0,
                  key_off: 0,
                  data_off: 0,
                  caused_expand: 0,};
    hashp = (*dbp).internal as *mut HTAB;
    if flags != 0 && flags != 3 as libc::c_int as libc::c_uint &&
           flags != 7 as libc::c_int as libc::c_uint {
        let ref mut fresh5 = *__errno_location();
        *fresh5 = 22 as libc::c_int;
        (*hashp).local_errno = *fresh5;
        return -(1 as libc::c_int)
    }
    item_info.seek_size = 0 as libc::c_int as u_int32_t;
    if flags == 3 as libc::c_int as libc::c_uint {
        __kdb2_get_item_first(hashp, cursorp, key, val, &mut item_info);
    } else { __kdb2_get_item_next(hashp, cursorp, key, val, &mut item_info); }
    loop 
         /*
	 * This needs to be changed around.  As is, get_item_next advances
	 * the pointers on the page but this function actually advances
	 * bucket pointers.  This works, since the only other place we
	 * use get_item_next is in hash_access which only deals with one
	 * bucket at a time.  However, there is the problem that certain other
	 * functions (such as find_bigpair and delpair) depend on the
	 * pgndx member of the cursor.  Right now, they are using pngdx - 1
	 * since indices refer to the __next__ item that is to be fetched
	 * from the page.  This is ugly, as you may have noticed, whoever
	 * you are.  The best solution would be to depend on item_infos to
	 * deal with _current_ information, and have the cursors only
	 * deal with _next_ information.  In that scheme, get_item_next
	 * would also advance buckets.  Version 3...
	 */
         /*
	 * Must always enter this loop to do error handling and
	 * check for big key/data pair.
	 */
         {
        if item_info.status as libc::c_int == 1 as libc::c_int {
            if item_info.key_off as libc::c_int == 0 as libc::c_int &&
                   __kdb2_big_keydata(hashp, (*cursorp).pagep, key, val,
                                      item_info.pgndx as int32_t) != 0 {
                return 1 as libc::c_int
            }
            break ;
        } else {
            if item_info.status as libc::c_int != 2 as libc::c_int {
                return 1 as libc::c_int
            }
            __kdb2_put_page(hashp, (*cursorp).pagep, 4 as libc::c_int,
                            0 as libc::c_int);
            (*cursorp).pgndx = 0 as libc::c_int as indx_t;
            (*cursorp).ndx = (*cursorp).pgndx;
            (*cursorp).bucket = (*cursorp).bucket.wrapping_add(1);
            (*cursorp).pgno = 0xffffffff as libc::c_uint;
            (*cursorp).pagep = 0 as *mut u_int16_t;
            if (*cursorp).bucket > (*hashp).hdr.max_bucket {
                return 1 as libc::c_int
            }
            __kdb2_get_item_next(hashp, cursorp, key, val, &mut item_info);
        }
    }
    __kdb2_get_item_done(hashp, cursorp);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "898:1"]
unsafe extern "C" fn cursor_delete(mut dbp: *const DB,
                                   mut cursor: *mut CURSOR,
                                   mut flags: u_int32_t) -> int32_t {
    /* XXX this is empirically determined, so it might not be completely
	   correct, but it seems to work.  At the very least it fixes
	   a memory leak */
    free((*cursor).internal);
    free(cursor as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "914:1"]
unsafe extern "C" fn hash_seq(mut dbp: *const DB, mut key: *mut DBT,
                              mut val: *mut DBT, mut flag: u_int32_t)
 -> int32_t {
    let mut hashp: *mut HTAB = 0 as *mut HTAB;
    /*
	 * Seq just uses the default cursor to go sequecing through the
	 * database.  Note that the default cursor is the first in the list.
	 */
    hashp = (*dbp).internal as *mut HTAB;
    if (*hashp).seq_cursor.is_null() {
        (*hashp).seq_cursor = __kdb2_cursor_creat(dbp)
    }
    return (*(*hashp).seq_cursor).get.expect("non-null function pointer")(dbp,
                                                                          (*hashp).seq_cursor,
                                                                          key,
                                                                          val,
                                                                          flag);
}
/* ******************************** UTILITIES ************************/
/*
 * Returns:
 *	 0 ==> OK
 *	-1 ==> Error
 */
#[no_mangle]
#[c2rust::src_loc = "941:1"]
pub unsafe extern "C" fn __kdb2_expand_table(mut hashp: *mut HTAB)
 -> int32_t {
    let mut old_bucket: u_int32_t = 0;
    let mut new_bucket: u_int32_t = 0;
    let mut spare_ndx: int32_t = 0;
    (*hashp).hdr.max_bucket = (*hashp).hdr.max_bucket.wrapping_add(1);
    new_bucket = (*hashp).hdr.max_bucket;
    old_bucket = (*hashp).hdr.max_bucket & (*hashp).hdr.low_mask;
    /* Get a page for this new bucket */
    if __kdb2_new_page(hashp, new_bucket, 0 as libc::c_int) !=
           0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    /*
	 * If the split point is increasing (hdr.max_bucket's log base 2
	 * increases), we need to copy the current contents of the spare
	 * split bucket to the next bucket.
	 */
    spare_ndx =
        __kdb2_log2((*hashp).hdr.max_bucket.wrapping_add(1 as libc::c_int as
                                                             libc::c_uint)) as
            int32_t;
    if spare_ndx > (*hashp).hdr.ovfl_point {
        (*hashp).hdr.spares[spare_ndx as usize] =
            (*hashp).hdr.spares[(*hashp).hdr.ovfl_point as usize];
        (*hashp).hdr.ovfl_point = spare_ndx
    }
    if new_bucket > (*hashp).hdr.high_mask {
        /* Starting a new doubling */
        (*hashp).hdr.low_mask = (*hashp).hdr.high_mask;
        (*hashp).hdr.high_mask = new_bucket | (*hashp).hdr.low_mask
    }
    if new_bucket.wrapping_add((*hashp).hdr.hdrpages).wrapping_add((if new_bucket
                                                                           !=
                                                                           0 {
                                                                        (*hashp).hdr.spares[__kdb2_log2(new_bucket.wrapping_add(1
                                                                                                                                    as
                                                                                                                                    libc::c_int
                                                                                                                                    as
                                                                                                                                    libc::c_uint)).wrapping_sub(1
                                                                                                                                                                    as
                                                                                                                                                                    libc::c_int
                                                                                                                                                                    as
                                                                                                                                                                    libc::c_uint)
                                                                                                as
                                                                                                usize]
                                                                    } else {
                                                                        0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint
                                                                    })) as
           libc::c_long >
           9223372036854775807 as libc::c_long /
               (*hashp).hdr.bsize as libc::c_long {
        fprintf(stderr,
                b"hash: Cannot allocate new bucket.  Pages exhausted.\n\x00"
                    as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    /* Relocate records to the new bucket */
    return __kdb2_split_page(hashp, old_bucket, new_bucket);
}
#[no_mangle]
#[c2rust::src_loc = "981:1"]
pub unsafe extern "C" fn __kdb2_call_hash(mut hashp: *mut HTAB,
                                          mut k: *mut int8_t,
                                          mut len: int32_t) -> u_int32_t {
    let mut n: u_int32_t = 0;
    let mut bucket: u_int32_t = 0;
    n =
        (*hashp).hash.expect("non-null function pointer")(k as
                                                              *const libc::c_void,
                                                          len as size_t);
    bucket = n & (*hashp).hdr.high_mask;
    if bucket > (*hashp).hdr.max_bucket {
        bucket = bucket & (*hashp).hdr.low_mask
    }
    return bucket;
}
/*
 * Hashp->hdr needs to be byteswapped.
 */
#[c2rust::src_loc = "1000:1"]
unsafe extern "C" fn swap_header_copy(mut srcp: *mut HASHHDR,
                                      mut destp: *mut HASHHDR) {
    let mut i: int32_t = 0;
    *(&mut (*destp).magic as *mut int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut (*srcp).magic as *mut int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*destp).magic as *mut int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut (*srcp).magic as *mut int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*destp).magic as *mut int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut (*srcp).magic as *mut int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*destp).magic as *mut int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut (*srcp).magic as *mut int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    *(&mut (*destp).version as *mut int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut (*srcp).version as *mut int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*destp).version as *mut int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut (*srcp).version as *mut int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*destp).version as *mut int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut (*srcp).version as *mut int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*destp).version as *mut int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut (*srcp).version as *mut int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    *(&mut (*destp).lorder as *mut int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut (*srcp).lorder as *mut int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*destp).lorder as *mut int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut (*srcp).lorder as *mut int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*destp).lorder as *mut int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut (*srcp).lorder as *mut int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*destp).lorder as *mut int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut (*srcp).lorder as *mut int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    *(&mut (*destp).bsize as *mut u_int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut (*srcp).bsize as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*destp).bsize as *mut u_int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut (*srcp).bsize as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*destp).bsize as *mut u_int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut (*srcp).bsize as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*destp).bsize as *mut u_int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut (*srcp).bsize as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    *(&mut (*destp).bshift as *mut int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut (*srcp).bshift as *mut int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*destp).bshift as *mut int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut (*srcp).bshift as *mut int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*destp).bshift as *mut int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut (*srcp).bshift as *mut int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*destp).bshift as *mut int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut (*srcp).bshift as *mut int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    *(&mut (*destp).ovfl_point as *mut int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut (*srcp).ovfl_point as *mut int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*destp).ovfl_point as *mut int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut (*srcp).ovfl_point as *mut int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*destp).ovfl_point as *mut int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut (*srcp).ovfl_point as *mut int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*destp).ovfl_point as *mut int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut (*srcp).ovfl_point as *mut int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    *(&mut (*destp).last_freed as *mut u_int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut (*srcp).last_freed as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*destp).last_freed as *mut u_int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut (*srcp).last_freed as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*destp).last_freed as *mut u_int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut (*srcp).last_freed as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*destp).last_freed as *mut u_int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut (*srcp).last_freed as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    *(&mut (*destp).max_bucket as *mut u_int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut (*srcp).max_bucket as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*destp).max_bucket as *mut u_int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut (*srcp).max_bucket as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*destp).max_bucket as *mut u_int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut (*srcp).max_bucket as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*destp).max_bucket as *mut u_int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut (*srcp).max_bucket as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    *(&mut (*destp).high_mask as *mut u_int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut (*srcp).high_mask as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*destp).high_mask as *mut u_int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut (*srcp).high_mask as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*destp).high_mask as *mut u_int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut (*srcp).high_mask as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*destp).high_mask as *mut u_int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut (*srcp).high_mask as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    *(&mut (*destp).low_mask as *mut u_int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut (*srcp).low_mask as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*destp).low_mask as *mut u_int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut (*srcp).low_mask as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*destp).low_mask as *mut u_int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut (*srcp).low_mask as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*destp).low_mask as *mut u_int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut (*srcp).low_mask as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    *(&mut (*destp).ffactor as *mut u_int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut (*srcp).ffactor as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*destp).ffactor as *mut u_int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut (*srcp).ffactor as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*destp).ffactor as *mut u_int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut (*srcp).ffactor as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*destp).ffactor as *mut u_int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut (*srcp).ffactor as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    *(&mut (*destp).nkeys as *mut int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut (*srcp).nkeys as *mut int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*destp).nkeys as *mut int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut (*srcp).nkeys as *mut int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*destp).nkeys as *mut int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut (*srcp).nkeys as *mut int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*destp).nkeys as *mut int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut (*srcp).nkeys as *mut int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    *(&mut (*destp).hdrpages as *mut u_int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut (*srcp).hdrpages as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*destp).hdrpages as *mut u_int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut (*srcp).hdrpages as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*destp).hdrpages as *mut u_int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut (*srcp).hdrpages as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*destp).hdrpages as *mut u_int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut (*srcp).hdrpages as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    *(&mut (*destp).h_charkey as *mut u_int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut (*srcp).h_charkey as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*destp).h_charkey as *mut u_int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut (*srcp).h_charkey as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*destp).h_charkey as *mut u_int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut (*srcp).h_charkey as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*destp).h_charkey as *mut u_int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut (*srcp).h_charkey as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        *(&mut *(*destp).spares.as_mut_ptr().offset(i as isize) as
              *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize) =
            *(&mut *(*srcp).spares.as_mut_ptr().offset(i as isize) as
                  *mut u_int32_t as
                  *mut libc::c_char).offset(3 as libc::c_int as isize);
        *(&mut *(*destp).spares.as_mut_ptr().offset(i as isize) as
              *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize) =
            *(&mut *(*srcp).spares.as_mut_ptr().offset(i as isize) as
                  *mut u_int32_t as
                  *mut libc::c_char).offset(2 as libc::c_int as isize);
        *(&mut *(*destp).spares.as_mut_ptr().offset(i as isize) as
              *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize) =
            *(&mut *(*srcp).spares.as_mut_ptr().offset(i as isize) as
                  *mut u_int32_t as
                  *mut libc::c_char).offset(1 as libc::c_int as isize);
        *(&mut *(*destp).spares.as_mut_ptr().offset(i as isize) as
              *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize) =
            *(&mut *(*srcp).spares.as_mut_ptr().offset(i as isize) as
                  *mut u_int32_t as
                  *mut libc::c_char).offset(0 as libc::c_int as isize);
        *(&mut *(*destp).bitmaps.as_mut_ptr().offset(i as isize) as
              *mut u_int16_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize) =
            *(&mut *(*srcp).bitmaps.as_mut_ptr().offset(i as isize) as
                  *mut u_int16_t as
                  *mut libc::c_char).offset(1 as libc::c_int as isize);
        *(&mut *(*destp).bitmaps.as_mut_ptr().offset(i as isize) as
              *mut u_int16_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize) =
            *(&mut *(*srcp).bitmaps.as_mut_ptr().offset(i as isize) as
                  *mut u_int16_t as
                  *mut libc::c_char).offset(0 as libc::c_int as isize);
        i += 1
    };
}
#[c2rust::src_loc = "1026:1"]
unsafe extern "C" fn swap_header(mut hashp: *mut HTAB) {
    let mut hdrp: *mut HASHHDR = 0 as *mut HASHHDR;
    let mut i: int32_t = 0;
    hdrp = &mut (*hashp).hdr;
    let mut _tmp: u_int32_t = (*hdrp).magic as u_int32_t;
    *(&mut (*hdrp).magic as *mut int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*hdrp).magic as *mut int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*hdrp).magic as *mut int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*hdrp).magic as *mut int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_0: u_int32_t = (*hdrp).version as u_int32_t;
    *(&mut (*hdrp).version as *mut int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_0 as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*hdrp).version as *mut int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_0 as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*hdrp).version as *mut int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp_0 as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*hdrp).version as *mut int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp_0 as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_1: u_int32_t = (*hdrp).lorder as u_int32_t;
    *(&mut (*hdrp).lorder as *mut int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_1 as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*hdrp).lorder as *mut int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_1 as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*hdrp).lorder as *mut int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp_1 as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*hdrp).lorder as *mut int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp_1 as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_2: u_int32_t = (*hdrp).bsize;
    *(&mut (*hdrp).bsize as *mut u_int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_2 as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*hdrp).bsize as *mut u_int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_2 as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*hdrp).bsize as *mut u_int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp_2 as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*hdrp).bsize as *mut u_int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp_2 as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_3: u_int32_t = (*hdrp).bshift as u_int32_t;
    *(&mut (*hdrp).bshift as *mut int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_3 as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*hdrp).bshift as *mut int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_3 as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*hdrp).bshift as *mut int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp_3 as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*hdrp).bshift as *mut int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp_3 as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_4: u_int32_t = (*hdrp).ovfl_point as u_int32_t;
    *(&mut (*hdrp).ovfl_point as *mut int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_4 as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*hdrp).ovfl_point as *mut int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_4 as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*hdrp).ovfl_point as *mut int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp_4 as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*hdrp).ovfl_point as *mut int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp_4 as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_5: u_int32_t = (*hdrp).last_freed;
    *(&mut (*hdrp).last_freed as *mut u_int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_5 as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*hdrp).last_freed as *mut u_int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_5 as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*hdrp).last_freed as *mut u_int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp_5 as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*hdrp).last_freed as *mut u_int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp_5 as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_6: u_int32_t = (*hdrp).max_bucket;
    *(&mut (*hdrp).max_bucket as *mut u_int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_6 as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*hdrp).max_bucket as *mut u_int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_6 as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*hdrp).max_bucket as *mut u_int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp_6 as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*hdrp).max_bucket as *mut u_int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp_6 as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_7: u_int32_t = (*hdrp).high_mask;
    *(&mut (*hdrp).high_mask as *mut u_int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_7 as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*hdrp).high_mask as *mut u_int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_7 as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*hdrp).high_mask as *mut u_int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp_7 as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*hdrp).high_mask as *mut u_int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp_7 as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_8: u_int32_t = (*hdrp).low_mask;
    *(&mut (*hdrp).low_mask as *mut u_int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_8 as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*hdrp).low_mask as *mut u_int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_8 as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*hdrp).low_mask as *mut u_int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp_8 as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*hdrp).low_mask as *mut u_int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp_8 as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_9: u_int32_t = (*hdrp).ffactor;
    *(&mut (*hdrp).ffactor as *mut u_int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_9 as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*hdrp).ffactor as *mut u_int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_9 as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*hdrp).ffactor as *mut u_int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp_9 as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*hdrp).ffactor as *mut u_int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp_9 as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_10: u_int32_t = (*hdrp).nkeys as u_int32_t;
    *(&mut (*hdrp).nkeys as *mut int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_10 as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*hdrp).nkeys as *mut int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_10 as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*hdrp).nkeys as *mut int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp_10 as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*hdrp).nkeys as *mut int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp_10 as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_11: u_int32_t = (*hdrp).hdrpages;
    *(&mut (*hdrp).hdrpages as *mut u_int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_11 as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*hdrp).hdrpages as *mut u_int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_11 as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*hdrp).hdrpages as *mut u_int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp_11 as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*hdrp).hdrpages as *mut u_int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp_11 as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    let mut _tmp_12: u_int32_t = (*hdrp).h_charkey;
    *(&mut (*hdrp).h_charkey as *mut u_int32_t as
          *mut libc::c_char).offset(0 as libc::c_int as isize) =
        *(&mut _tmp_12 as *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize);
    *(&mut (*hdrp).h_charkey as *mut u_int32_t as
          *mut libc::c_char).offset(1 as libc::c_int as isize) =
        *(&mut _tmp_12 as *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize);
    *(&mut (*hdrp).h_charkey as *mut u_int32_t as
          *mut libc::c_char).offset(2 as libc::c_int as isize) =
        *(&mut _tmp_12 as *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize);
    *(&mut (*hdrp).h_charkey as *mut u_int32_t as
          *mut libc::c_char).offset(3 as libc::c_int as isize) =
        *(&mut _tmp_12 as *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        let mut _tmp_13: u_int32_t = (*hdrp).spares[i as usize];
        *(&mut *(*hdrp).spares.as_mut_ptr().offset(i as isize) as
              *mut u_int32_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize) =
            *(&mut _tmp_13 as *mut u_int32_t as
                  *mut libc::c_char).offset(3 as libc::c_int as isize);
        *(&mut *(*hdrp).spares.as_mut_ptr().offset(i as isize) as
              *mut u_int32_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize) =
            *(&mut _tmp_13 as *mut u_int32_t as
                  *mut libc::c_char).offset(2 as libc::c_int as isize);
        *(&mut *(*hdrp).spares.as_mut_ptr().offset(i as isize) as
              *mut u_int32_t as
              *mut libc::c_char).offset(2 as libc::c_int as isize) =
            *(&mut _tmp_13 as *mut u_int32_t as
                  *mut libc::c_char).offset(1 as libc::c_int as isize);
        *(&mut *(*hdrp).spares.as_mut_ptr().offset(i as isize) as
              *mut u_int32_t as
              *mut libc::c_char).offset(3 as libc::c_int as isize) =
            *(&mut _tmp_13 as *mut u_int32_t as
                  *mut libc::c_char).offset(0 as libc::c_int as isize);
        let mut _tmp_14: u_int16_t = (*hdrp).bitmaps[i as usize];
        *(&mut *(*hdrp).bitmaps.as_mut_ptr().offset(i as isize) as
              *mut u_int16_t as
              *mut libc::c_char).offset(0 as libc::c_int as isize) =
            *(&mut _tmp_14 as *mut u_int16_t as
                  *mut libc::c_char).offset(1 as libc::c_int as isize);
        *(&mut *(*hdrp).bitmaps.as_mut_ptr().offset(i as isize) as
              *mut u_int16_t as
              *mut libc::c_char).offset(1 as libc::c_int as isize) =
            *(&mut _tmp_14 as *mut u_int16_t as
                  *mut libc::c_char).offset(0 as libc::c_int as isize);
        i += 1
    };
}
/* DB_BYTE_ORDER == DB_LITTLE_ENDIAN */
