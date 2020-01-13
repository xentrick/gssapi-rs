use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:38"]
pub mod types_h {
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "34:1"]
    pub type __u_long = libc::c_ulong;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
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
#[c2rust::header_src = "/usr/include/sys/types.h:38"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "36:1"]
    pub type u_long = __u_long;
    #[c2rust::src_loc = "85:1"]
    pub type off_t = __off_t;
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    #[c2rust::src_loc = "158:1"]
    pub type u_int8_t = __uint8_t;
    #[c2rust::src_loc = "160:1"]
    pub type u_int32_t = __uint32_t;
    use super::types_h::{__u_int, __u_long, __off_t, __ssize_t, __uint8_t,
                         __uint32_t};
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:38"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:38"]
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
#[c2rust::header_src = "/usr/include/bits/stat.h:39"]
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
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:42"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:42"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/include/db-int.h:47"]
pub mod db_int_h {
    /* >= # of pages in a file */
    #[c2rust::src_loc = "143:1"]
    pub type db_pgno_t = u_int32_t;
    use super::sys_types_h::u_int32_t;
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
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/libdb2/mpool/mpool.h:48"]
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
    /* The BKT structures are the elements of the queues. */
    #[c2rust::src_loc = "49:1"]
    pub type BKT = _bkt;
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
#[c2rust::header_src = "/usr/include/sys/stat.h:39"]
pub mod sys_stat_h {
    use super::stat_h::stat;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "210:1"]
        pub fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:41"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:42"]
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
#[c2rust::header_src = "/usr/include/stdlib.h:43"]
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
#[c2rust::header_src = "/usr/include/string.h:44"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:45"]
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
        #[c2rust::src_loc = "360:1"]
        pub fn read(__fd: libc::c_int, __buf: *mut libc::c_void,
                    __nbytes: size_t) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "366:1"]
        pub fn write(__fd: libc::c_int, __buf: *const libc::c_void,
                     __n: size_t) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "954:1"]
        pub fn fsync(__fd: libc::c_int) -> libc::c_int;
    }
}
pub use self::types_h::{__u_int, __u_long, __uint8_t, __uint32_t, __dev_t,
                        __uid_t, __gid_t, __ino_t, __mode_t, __nlink_t,
                        __off_t, __off64_t, __time_t, __blksize_t, __blkcnt_t,
                        __ssize_t, __syscall_slong_t};
pub use self::sys_types_h::{u_int, u_long, off_t, ssize_t, u_int8_t,
                            u_int32_t};
pub use self::stddef_h::size_t;
pub use self::struct_timespec_h::timespec;
pub use self::stat_h::stat;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::db_int_h::db_pgno_t;
pub use self::mpool_h::{_bkt, C2RustUnnamed, C2RustUnnamed_0, BKT, MPOOL,
                        _hqh, _lqh};
use self::sys_stat_h::fstat;
use self::errno_h::__errno_location;
use self::stdio_h::{stderr, fprintf};
use self::stdlib_h::{malloc, calloc, free, abort};
use self::string_h::memset;
use self::unistd_h::{lseek, read, write, fsync};
/* hash queue */
/* lru queue */
/* page */
/* page number */
/* page needs to be written */
/* page is pinned into memory */
/* page address is valid */
/* flags */
/* Allocate a new page with a
					   specific page number. */
/* Allocate a new page with the next
					  page number. */
/*
 * mpool_open --
 *	Initialize a memory pool.
 */
#[no_mangle]
#[c2rust::src_loc = "58:1"]
pub unsafe extern "C" fn kdb2_mpool_open(mut key: *mut libc::c_void,
                                         mut fd: libc::c_int,
                                         mut pagesize: db_pgno_t,
                                         mut maxcache: db_pgno_t)
 -> *mut MPOOL {
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
    let mut mp: *mut MPOOL = 0 as *mut MPOOL;
    let mut entry: libc::c_int = 0;
    /*
	 * Get information about the file.
	 *
	 * XXX
	 * We don't currently handle pipes, although we should.
	 */
    if fstat(fd, &mut sb) != 0 { return 0 as *mut MPOOL }
    if !(sb.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
             0o100000 as libc::c_int as libc::c_uint) {
        *__errno_location() = 29 as libc::c_int;
        return 0 as *mut MPOOL
    }
    /* Allocate and initialize the MPOOL cookie. */
    mp =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<MPOOL>() as libc::c_ulong) as *mut MPOOL;
    if mp.is_null() { return 0 as *mut MPOOL }
    (*mp).lqh.tqh_first = 0 as *mut _bkt;
    (*mp).lqh.tqh_last = &mut (*mp).lqh.tqh_first;
    entry = 0 as libc::c_int;
    while entry < 128 as libc::c_int {
        (*mp).hqh[entry as usize].tqh_first = 0 as *mut _bkt;
        (*mp).hqh[entry as usize].tqh_last =
            &mut (*(*mp).hqh.as_mut_ptr().offset(entry as isize)).tqh_first;
        entry += 1
    }
    (*mp).maxcache = maxcache;
    (*mp).npages = (sb.st_size / pagesize as libc::c_long) as db_pgno_t;
    (*mp).pagesize = pagesize as u_long;
    (*mp).fd = fd;
    return mp;
}
/*
 * mpool_filter --
 *	Initialize input/output filters.
 */
#[no_mangle]
#[c2rust::src_loc = "98:1"]
pub unsafe extern "C" fn kdb2_mpool_filter(mut mp: *mut MPOOL,
                                           mut pgin:
                                               Option<unsafe extern "C" fn(_:
                                                                               *mut libc::c_void,
                                                                           _:
                                                                               db_pgno_t,
                                                                           _:
                                                                               *mut libc::c_void)
                                                          -> ()>,
                                           mut pgout:
                                               Option<unsafe extern "C" fn(_:
                                                                               *mut libc::c_void,
                                                                           _:
                                                                               db_pgno_t,
                                                                           _:
                                                                               *mut libc::c_void)
                                                          -> ()>,
                                           mut pgcookie: *mut libc::c_void) {
    (*mp).pgin = pgin;
    (*mp).pgout = pgout;
    (*mp).pgcookie = pgcookie;
}
/*
 * mpool_new --
 *	Get a new page of memory.
 */
#[no_mangle]
#[c2rust::src_loc = "114:1"]
pub unsafe extern "C" fn kdb2_mpool_new(mut mp: *mut MPOOL,
                                        mut pgnoaddr: *mut db_pgno_t,
                                        mut flags: u_int)
 -> *mut libc::c_void {
    let mut head: *mut _hqh = 0 as *mut _hqh;
    let mut bp: *mut BKT = 0 as *mut BKT;
    if (*mp).npages == 0xffffffff as libc::c_uint {
        fprintf(stderr,
                b"mpool_new: page allocation overflow.\n\x00" as *const u8 as
                    *const libc::c_char);
        abort();
    }
    /*
	 * Get a BKT from the cache.  Assign a new page number, attach
	 * it to the head of the hash chain, the tail of the lru chain,
	 * and return.
	 */
    bp = mpool_bkt(mp);
    if bp.is_null() { return 0 as *mut libc::c_void }
    if flags == 0x1 as libc::c_int as libc::c_uint {
        (*mp).npages = (*mp).npages.wrapping_add(1);
        (*bp).pgno = *pgnoaddr
    } else {
        let fresh0 = (*mp).npages;
        (*mp).npages = (*mp).npages.wrapping_add(1);
        *pgnoaddr = fresh0;
        (*bp).pgno = *pgnoaddr
    }
    (*bp).flags = (0x2 as libc::c_int | 0x4 as libc::c_int) as u_int8_t;
    head =
        &mut *(*mp).hqh.as_mut_ptr().offset((*bp).pgno.wrapping_sub(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint).wrapping_rem(128
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint)
                                                as isize) as *mut _hqh;
    (*bp).hq.tqe_next = (*head).tqh_first;
    if !(*bp).hq.tqe_next.is_null() {
        (*(*bp).hq.tqe_next).hq.tqe_prev = &mut (*bp).hq.tqe_next
    } else { (*head).tqh_last = &mut (*bp).hq.tqe_next }
    (*head).tqh_first = bp;
    (*bp).hq.tqe_prev = &mut (*head).tqh_first;
    (*bp).q.tqe_next = 0 as *mut _bkt;
    (*bp).q.tqe_prev = (*mp).lqh.tqh_last;
    *(*mp).lqh.tqh_last = bp;
    (*mp).lqh.tqh_last = &mut (*bp).q.tqe_next;
    return (*bp).page;
}
#[no_mangle]
#[c2rust::src_loc = "151:1"]
pub unsafe extern "C" fn kdb2_mpool_delete(mut mp: *mut MPOOL,
                                           mut page: *mut libc::c_void)
 -> libc::c_int {
    let mut head: *mut _hqh = 0 as *mut _hqh;
    let mut bp: *mut BKT = 0 as *mut BKT;
    bp =
        (page as
             *mut libc::c_char).offset(-(::std::mem::size_of::<BKT>() as
                                             libc::c_ulong as isize)) as
            *mut libc::c_void as *mut BKT;
    /* Remove from the hash and lru queues. */
    head =
        &mut *(*mp).hqh.as_mut_ptr().offset((*bp).pgno.wrapping_sub(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint).wrapping_rem(128
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint)
                                                as isize) as *mut _hqh;
    if !(*bp).hq.tqe_next.is_null() {
        (*(*bp).hq.tqe_next).hq.tqe_prev = (*bp).hq.tqe_prev
    } else { (*head).tqh_last = (*bp).hq.tqe_prev }
    *(*bp).hq.tqe_prev = (*bp).hq.tqe_next;
    if !(*bp).q.tqe_next.is_null() {
        (*(*bp).q.tqe_next).q.tqe_prev = (*bp).q.tqe_prev
    } else { (*mp).lqh.tqh_last = (*bp).q.tqe_prev }
    *(*bp).q.tqe_prev = (*bp).q.tqe_next;
    free(bp as *mut libc::c_void);
    return 0 as libc::c_int;
}
/*
 * mpool_get
 *	Get a page.
 */
#[no_mangle]
#[c2rust::src_loc = "182:1"]
pub unsafe extern "C" fn kdb2_mpool_get(mut mp: *mut MPOOL,
                                        mut pgno: db_pgno_t, mut flags: u_int)
 -> *mut libc::c_void 
 /* XXX not used? */
 {
    let mut head: *mut _hqh = 0 as *mut _hqh;
    let mut bp: *mut BKT = 0 as *mut BKT;
    let mut off: off_t = 0;
    let mut nr: libc::c_int = 0;
    /* Check for a page that is cached. */
    bp = mpool_look(mp, pgno);
    if !bp.is_null() {
        /*
		 * Move the page to the head of the hash chain and the tail
		 * of the lru chain.
		 */
        head =
            &mut *(*mp).hqh.as_mut_ptr().offset((*bp).pgno.wrapping_sub(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_rem(128
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_uint)
                                                    as isize) as *mut _hqh;
        if !(*bp).hq.tqe_next.is_null() {
            (*(*bp).hq.tqe_next).hq.tqe_prev = (*bp).hq.tqe_prev
        } else { (*head).tqh_last = (*bp).hq.tqe_prev }
        *(*bp).hq.tqe_prev = (*bp).hq.tqe_next;
        (*bp).hq.tqe_next = (*head).tqh_first;
        if !(*bp).hq.tqe_next.is_null() {
            (*(*bp).hq.tqe_next).hq.tqe_prev = &mut (*bp).hq.tqe_next
        } else { (*head).tqh_last = &mut (*bp).hq.tqe_next }
        (*head).tqh_first = bp;
        (*bp).hq.tqe_prev = &mut (*head).tqh_first;
        if !(*bp).q.tqe_next.is_null() {
            (*(*bp).q.tqe_next).q.tqe_prev = (*bp).q.tqe_prev
        } else { (*mp).lqh.tqh_last = (*bp).q.tqe_prev }
        *(*bp).q.tqe_prev = (*bp).q.tqe_next;
        (*bp).q.tqe_next = 0 as *mut _bkt;
        (*bp).q.tqe_prev = (*mp).lqh.tqh_last;
        *(*mp).lqh.tqh_last = bp;
        (*mp).lqh.tqh_last = &mut (*bp).q.tqe_next;
        /* Return a pinned page. */
        if flags & 0x1 as libc::c_int as libc::c_uint == 0 {
            (*bp).flags =
                ((*bp).flags as libc::c_int | 0x2 as libc::c_int) as u_int8_t
        }
        return (*bp).page
    }
    /* Get a page from the cache. */
    bp = mpool_bkt(mp);
    if bp.is_null() { return 0 as *mut libc::c_void }
    /* Read in the contents. */
    off = (*mp).pagesize.wrapping_mul(pgno as libc::c_ulong) as off_t;
    if (off as libc::c_ulong).wrapping_div((*mp).pagesize) !=
           pgno as libc::c_ulong {
        /* Run past the end of the file, or at least the part we
	       can address without large-file support?  */
        *__errno_location() = 7 as libc::c_int;
        return 0 as *mut libc::c_void
    }
    if lseek((*mp).fd, off, 0 as libc::c_int) != off {
        return 0 as *mut libc::c_void
    }
    nr = read((*mp).fd, (*bp).page, (*mp).pagesize) as libc::c_int;
    if nr as libc::c_long != (*mp).pagesize as ssize_t {
        if nr > 0 as libc::c_int {
            /* A partial read is definitely bad. */
            *__errno_location() = 22 as libc::c_int;
            return 0 as *mut libc::c_void
        } else {
            /*
			 * A zero-length reads, means you need to create a
			 * new page.
			 */
            memset((*bp).page, 0 as libc::c_int, (*mp).pagesize);
        }
    }
    /* Set the page number, pin the page. */
    (*bp).pgno = pgno;
    if flags & 0x1 as libc::c_int as libc::c_uint == 0 {
        (*bp).flags = 0x2 as libc::c_int as u_int8_t
    }
    (*bp).flags =
        ((*bp).flags as libc::c_int | 0x4 as libc::c_int) as u_int8_t;
    /*
	 * Add the page to the head of the hash chain and the tail
	 * of the lru chain.
	 */
    head =
        &mut *(*mp).hqh.as_mut_ptr().offset((*bp).pgno.wrapping_sub(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint).wrapping_rem(128
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint)
                                                as isize) as *mut _hqh;
    (*bp).hq.tqe_next = (*head).tqh_first;
    if !(*bp).hq.tqe_next.is_null() {
        (*(*bp).hq.tqe_next).hq.tqe_prev = &mut (*bp).hq.tqe_next
    } else { (*head).tqh_last = &mut (*bp).hq.tqe_next }
    (*head).tqh_first = bp;
    (*bp).hq.tqe_prev = &mut (*head).tqh_first;
    (*bp).q.tqe_next = 0 as *mut _bkt;
    (*bp).q.tqe_prev = (*mp).lqh.tqh_last;
    *(*mp).lqh.tqh_last = bp;
    (*mp).lqh.tqh_last = &mut (*bp).q.tqe_next;
    /* Run through the user's filter. */
    if (*mp).pgin.is_some() {
        (*mp).pgin.expect("non-null function pointer")((*mp).pgcookie,
                                                       (*bp).pgno,
                                                       (*bp).page);
    }
    return (*bp).page;
}
/*
 * mpool_put
 *	Return a page.
 */
#[no_mangle]
#[c2rust::src_loc = "280:1"]
pub unsafe extern "C" fn kdb2_mpool_put(mut mp: *mut MPOOL,
                                        mut page: *mut libc::c_void,
                                        mut flags: u_int) -> libc::c_int {
    let mut bp: *mut BKT = 0 as *mut BKT;
    bp =
        (page as
             *mut libc::c_char).offset(-(::std::mem::size_of::<BKT>() as
                                             libc::c_ulong as isize)) as
            *mut libc::c_void as *mut BKT;
    (*bp).flags =
        ((*bp).flags as libc::c_int & !(0x2 as libc::c_int)) as u_int8_t;
    if flags & 0x1 as libc::c_int as libc::c_uint != 0 {
        (*bp).flags =
            ((*bp).flags as libc::c_uint |
                 flags & 0x1 as libc::c_int as libc::c_uint) as u_int8_t
    }
    return 0 as libc::c_int;
}
/*
 * mpool_close
 *	Close the buffer pool.
 */
#[no_mangle]
#[c2rust::src_loc = "309:1"]
pub unsafe extern "C" fn kdb2_mpool_close(mut mp: *mut MPOOL) -> libc::c_int {
    let mut bp: *mut BKT = 0 as *mut BKT;
    loop 
         /* Free up any space allocated to the lru pages. */
         {
        bp = (*mp).lqh.tqh_first;
        if bp.is_null() { break ; }
        if !(*(*mp).lqh.tqh_first).q.tqe_next.is_null() {
            (*(*(*mp).lqh.tqh_first).q.tqe_next).q.tqe_prev =
                (*(*mp).lqh.tqh_first).q.tqe_prev
        } else { (*mp).lqh.tqh_last = (*(*mp).lqh.tqh_first).q.tqe_prev }
        *(*(*mp).lqh.tqh_first).q.tqe_prev =
            (*(*mp).lqh.tqh_first).q.tqe_next;
        free(bp as *mut libc::c_void);
    }
    /* Free the MPOOL cookie. */
    free(mp as *mut libc::c_void);
    return 0 as libc::c_int;
}
/*
 * mpool_sync
 *	Sync the pool to disk.
 */
#[no_mangle]
#[c2rust::src_loc = "330:1"]
pub unsafe extern "C" fn kdb2_mpool_sync(mut mp: *mut MPOOL) -> libc::c_int {
    let mut bp: *mut BKT = 0 as *mut BKT;
    /* Walk the lru chain, flushing any dirty pages to disk. */
    bp = (*mp).lqh.tqh_first;
    while !bp.is_null() {
        if (*bp).flags as libc::c_int & 0x1 as libc::c_int != 0 &&
               mpool_write(mp, bp) == -(1 as libc::c_int) {
            return -(1 as libc::c_int)
        }
        bp = (*bp).q.tqe_next
    }
    /* Sync the file descriptor. */
    return if fsync((*mp).fd) != 0 {
               -(1 as libc::c_int)
           } else { 0 as libc::c_int };
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
 */
/* LIBC_SCCS and not lint */
/*
 * mpool_bkt
 *	Get a page from the cache (or create one).
 */
#[c2rust::src_loc = "350:1"]
unsafe extern "C" fn mpool_bkt(mut mp: *mut MPOOL) -> *mut BKT {
    let mut head: *mut _hqh = 0 as *mut _hqh;
    let mut bp: *mut BKT = 0 as *mut BKT;
    /* If under the max cached, always create a new page. */
    if !((*mp).curcache < (*mp).maxcache) {
        /*
	 * If the cache is max'd out, walk the lru list for a buffer we
	 * can flush.  If we find one, write it (if necessary) and take it
	 * off any lists.  If we don't find anything we grow the cache anyway.
	 * The cache never shrinks.
	 */
        bp = (*mp).lqh.tqh_first;
        while !bp.is_null() {
            if (*bp).flags as libc::c_int & 0x2 as libc::c_int == 0 {
                /* Flush if dirty. */
                if (*bp).flags as libc::c_int & 0x1 as libc::c_int != 0 &&
                       mpool_write(mp, bp) == -(1 as libc::c_int) {
                    return 0 as *mut BKT
                }
                /* Remove from the hash and lru queues. */
                head =
                    &mut *(*mp).hqh.as_mut_ptr().offset((*bp).pgno.wrapping_sub(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint).wrapping_rem(128
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_uint)
                                                            as isize) as
                        *mut _hqh;
                if !(*bp).hq.tqe_next.is_null() {
                    (*(*bp).hq.tqe_next).hq.tqe_prev = (*bp).hq.tqe_prev
                } else { (*head).tqh_last = (*bp).hq.tqe_prev }
                *(*bp).hq.tqe_prev = (*bp).hq.tqe_next;
                if !(*bp).q.tqe_next.is_null() {
                    (*(*bp).q.tqe_next).q.tqe_prev = (*bp).q.tqe_prev
                } else { (*mp).lqh.tqh_last = (*bp).q.tqe_prev }
                *(*bp).q.tqe_prev = (*bp).q.tqe_next;
                (*bp).flags = 0 as libc::c_int as u_int8_t;
                return bp
            }
            bp = (*bp).q.tqe_next
        }
    }
    bp =
        malloc((::std::mem::size_of::<BKT>() as
                    libc::c_ulong).wrapping_add((*mp).pagesize)) as *mut BKT;
    if bp.is_null() { return 0 as *mut BKT }
    memset(bp as *mut libc::c_void, 0xff as libc::c_int,
           (::std::mem::size_of::<BKT>() as
                libc::c_ulong).wrapping_add((*mp).pagesize));
    (*bp).page =
        (bp as
             *mut libc::c_char).offset(::std::mem::size_of::<BKT>() as
                                           libc::c_ulong as isize) as
            *mut libc::c_void;
    (*bp).flags = 0 as libc::c_int as u_int8_t;
    (*mp).curcache = (*mp).curcache.wrapping_add(1);
    return bp;
}
/*
 * mpool_write
 *	Write a page to disk.
 */
#[c2rust::src_loc = "409:1"]
unsafe extern "C" fn mpool_write(mut mp: *mut MPOOL, mut bp: *mut BKT)
 -> libc::c_int {
    let mut off: off_t = 0;
    /* Run through the user's filter. */
    if (*mp).pgout.is_some() {
        (*mp).pgout.expect("non-null function pointer")((*mp).pgcookie,
                                                        (*bp).pgno,
                                                        (*bp).page);
    }
    off = (*mp).pagesize.wrapping_mul((*bp).pgno as libc::c_ulong) as off_t;
    if (off as libc::c_ulong).wrapping_div((*mp).pagesize) !=
           (*bp).pgno as libc::c_ulong {
        /* Run past the end of the file, or at least the part we
	       can address without large-file support?  */
        *__errno_location() = 7 as libc::c_int;
        return -(1 as libc::c_int)
    }
    if lseek((*mp).fd, off, 0 as libc::c_int) != off {
        return -(1 as libc::c_int)
    }
    if write((*mp).fd, (*bp).page, (*mp).pagesize) !=
           (*mp).pagesize as ssize_t {
        return -(1 as libc::c_int)
    }
    /*
	 * Re-run through the input filter since this page may soon be
	 * accessed via the cache, and whatever the user's output filter
	 * did may screw things up if we don't let the input filter
	 * restore the in-core copy.
	 */
    if (*mp).pgin.is_some() {
        (*mp).pgin.expect("non-null function pointer")((*mp).pgcookie,
                                                       (*bp).pgno,
                                                       (*bp).page);
    }
    (*bp).flags =
        ((*bp).flags as libc::c_int & !(0x1 as libc::c_int)) as u_int8_t;
    return 0 as libc::c_int;
}
/*
 * mpool_look
 *	Lookup a page in the cache.
 */
#[c2rust::src_loc = "453:1"]
unsafe extern "C" fn mpool_look(mut mp: *mut MPOOL, mut pgno: db_pgno_t)
 -> *mut BKT {
    let mut head: *mut _hqh = 0 as *mut _hqh;
    let mut bp: *mut BKT = 0 as *mut BKT;
    head =
        &mut *(*mp).hqh.as_mut_ptr().offset(pgno.wrapping_sub(1 as libc::c_int
                                                                  as
                                                                  libc::c_uint).wrapping_rem(128
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint)
                                                as isize) as *mut _hqh;
    bp = (*head).tqh_first;
    while !bp.is_null() {
        if (*bp).pgno == pgno &&
               (*bp).flags as libc::c_int & 0x4 as libc::c_int != 0 {
            return bp
        }
        bp = (*bp).hq.tqe_next
    }
    return 0 as *mut BKT;
}
#[no_mangle]
#[c2rust::src_loc = "522:1"]
pub unsafe extern "C" fn kdb2_mpool_stat(mut mp: *mut MPOOL) { }
