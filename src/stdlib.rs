extern "C" {
    #[no_mangle]
    pub fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    #[no_mangle]
    pub fn __ctype_b_loc() -> *mut *const u16;
    pub type dirent;

    #[no_mangle]
    pub fn glob(
        __pattern: *const i8,
        __flags: i32,
        __errfunc: Option<unsafe extern "C" fn(_: *const i8, _: i32) -> i32>,
        __pglob: *mut crate::stdlib::glob_t,
    ) -> i32;

    #[no_mangle]
    pub fn globfree(__pglob: *mut crate::stdlib::glob_t);
    #[no_mangle]
    pub fn dgettext(__domainname: *const i8, __msgid: *const i8) -> *mut i8;
    #[no_mangle]
    pub fn getpwuid_r(
        __uid: crate::stdlib::__uid_t,
        __resultbuf: *mut crate::stdlib::passwd,
        __buffer: *mut i8,
        __buflen: crate::stddef_h::size_t,
        __result: *mut *mut crate::stdlib::passwd,
    ) -> i32;

    #[no_mangle]
    pub fn getpwnam_r(
        __name: *const i8,
        __resultbuf: *mut crate::stdlib::passwd,
        __buffer: *mut i8,
        __buflen: crate::stddef_h::size_t,
        __result: *mut *mut crate::stdlib::passwd,
    ) -> i32;
    #[no_mangle]
    pub fn asprintf(__ptr: *mut *mut i8, __fmt: *const i8, _: ...) -> i32;

    #[no_mangle]
    pub fn vasprintf(__ptr: *mut *mut i8, __f: *const i8, __arg: ::std::ffi::VaList) -> i32;

    #[no_mangle]
    pub static mut stderr: *mut crate::stdlib::FILE;

    #[no_mangle]
    pub fn fprintf(_: *mut crate::stdlib::FILE, _: *const i8, _: ...) -> i32;

    #[no_mangle]
    pub fn fclose(__stream: *mut crate::stdlib::FILE) -> i32;

    #[no_mangle]
    pub fn fgets(__s: *mut i8, __n: i32, __stream: *mut crate::stdlib::FILE) -> *mut i8;

    #[no_mangle]
    pub fn fopen(_: *const i8, _: *const i8) -> *mut crate::stdlib::FILE;

    #[no_mangle]
    pub fn snprintf(_: *mut i8, _: usize, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    pub fn realloc(_: *mut libc::c_void, _: usize) -> *mut libc::c_void;

    #[no_mangle]
    pub fn abort() -> !;

    #[no_mangle]
    pub fn atoi(__nptr: *const i8) -> i32;

    #[no_mangle]
    pub fn secure_getenv(__name: *const i8) -> *mut i8;

    #[no_mangle]
    pub fn free(__ptr: *mut libc::c_void);

    #[no_mangle]
    pub fn calloc(_: usize, _: usize) -> *mut libc::c_void;

    #[no_mangle]
    pub fn malloc(_: usize) -> *mut libc::c_void;

    #[no_mangle]
    pub fn atol(__nptr: *const i8) -> isize;
    #[no_mangle]
    pub fn strlen(_: *const i8) -> usize;

    #[no_mangle]
    pub fn strcmp(_: *const i8, _: *const i8) -> i32;

    #[no_mangle]
    pub fn strerror(_: i32) -> *mut i8;

    #[no_mangle]
    pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: usize) -> i32;

    #[no_mangle]
    pub fn strncpy(_: *mut i8, _: *const i8, _: usize) -> *mut i8;

    #[no_mangle]
    pub fn memchr(_: *const libc::c_void, _: i32, _: usize) -> *mut libc::c_void;

    #[no_mangle]
    pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: usize) -> *mut libc::c_void;

    #[no_mangle]
    pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: usize) -> *mut libc::c_void;

    #[no_mangle]
    pub fn memset(_: *mut libc::c_void, _: i32, _: usize) -> *mut libc::c_void;

    #[no_mangle]
    pub fn explicit_bzero(__s: *mut libc::c_void, __n: crate::stddef_h::size_t);

    #[no_mangle]
    pub fn strdup(_: *const i8) -> *mut i8;
    pub type _IO_wide_data;

    pub type _IO_codecvt;

    pub type _IO_marker;
    #[no_mangle]
    pub fn lstat(__file: *const i8, __buf: *mut crate::stdlib::stat) -> i32;

    #[no_mangle]
    pub fn stat(__file: *const i8, __buf: *mut crate::stdlib::stat) -> i32;
    #[no_mangle]
    pub fn time(__timer: *mut crate::stdlib::time_t) -> crate::stdlib::time_t;
}
// =============== BEGIN FILE_h ================
pub type FILE = crate::stdlib::_IO_FILE;
// ================ END FILE_h ================
// =============== BEGIN ctype_h ================
pub type C2RustUnnamed_7 = u32;
pub const _ISspace: crate::stdlib::C2RustUnnamed_7 = 8192;
pub const _ISdigit: crate::stdlib::C2RustUnnamed_7 = 2048;
pub const _ISalnum: crate::stdlib::C2RustUnnamed_7 = 8;
pub const _ISpunct: crate::stdlib::C2RustUnnamed_7 = 4;
pub const _IScntrl: crate::stdlib::C2RustUnnamed_7 = 2;
pub const _ISblank: crate::stdlib::C2RustUnnamed_7 = 1;
pub const _ISgraph: crate::stdlib::C2RustUnnamed_7 = 32768;
pub const _ISprint: crate::stdlib::C2RustUnnamed_7 = 16384;
pub const _ISxdigit: crate::stdlib::C2RustUnnamed_7 = 4096;
pub const _ISalpha: crate::stdlib::C2RustUnnamed_7 = 1024;
pub const _ISlower: crate::stdlib::C2RustUnnamed_7 = 512;
pub const _ISupper: crate::stdlib::C2RustUnnamed_7 = 256;
// ================ END ctype_h ================
// =============== BEGIN glob_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct glob_t {
    pub gl_pathc: crate::stdlib::__size_t,
    pub gl_pathv: *mut *mut i8,
    pub gl_offs: crate::stdlib::__size_t,
    pub gl_flags: i32,
    pub gl_closedir: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub gl_readdir:
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> *mut crate::stdlib::dirent>,
    pub gl_opendir: Option<unsafe extern "C" fn(_: *const i8) -> *mut libc::c_void>,
    pub gl_lstat: Option<unsafe extern "C" fn(_: *const i8, _: *mut crate::stdlib::stat) -> i32>,
    pub gl_stat: Option<unsafe extern "C" fn(_: *const i8, _: *mut crate::stdlib::stat) -> i32>,
}
pub type __size_t = usize;
// ================ END glob_h ================
// =============== BEGIN pthreadtypes_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: crate::stdlib::__pthread_mutex_s,
    pub __size: [i8; 40],
    pub __align: isize,
}
pub type pthread_once_t = i32;
// ================ END pthreadtypes_h ================
// =============== BEGIN pwd_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct passwd {
    pub pw_name: *mut i8,
    pub pw_passwd: *mut i8,
    pub pw_uid: crate::stdlib::__uid_t,
    pub pw_gid: crate::stdlib::__gid_t,
    pub pw_gecos: *mut i8,
    pub pw_dir: *mut i8,
    pub pw_shell: *mut i8,
}
// ================ END pwd_h ================
// =============== BEGIN stat_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stat {
    pub st_dev: crate::stdlib::__dev_t,
    pub st_ino: crate::stdlib::__ino_t,
    pub st_nlink: crate::stdlib::__nlink_t,
    pub st_mode: crate::stdlib::__mode_t,
    pub st_uid: crate::stdlib::__uid_t,
    pub st_gid: crate::stdlib::__gid_t,
    pub __pad0: i32,
    pub st_rdev: crate::stdlib::__dev_t,
    pub st_size: crate::stdlib::__off_t,
    pub st_blksize: crate::stdlib::__blksize_t,
    pub st_blocks: crate::stdlib::__blkcnt_t,
    pub st_atim: crate::stdlib::timespec,
    pub st_mtim: crate::stdlib::timespec,
    pub st_ctim: crate::stdlib::timespec,
    pub __glibc_reserved: [crate::stdlib::__syscall_slong_t; 3],
}
// ================ END stat_h ================
// =============== BEGIN stdint_intn_h ================
pub type int32_t = crate::stdlib::__int32_t;
pub type int16_t = crate::stdlib::__int16_t;
pub type int64_t = crate::stdlib::__int64_t;
// ================ END stdint_intn_h ================
// =============== BEGIN stdint_uintn_h ================
pub type uint32_t = crate::stdlib::__uint32_t;
pub type uint64_t = crate::stdlib::__uint64_t;
pub type uint8_t = crate::stdlib::__uint8_t;
pub type uint16_t = crate::stdlib::__uint16_t;
// ================ END stdint_uintn_h ================
// =============== BEGIN struct_FILE_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut crate::stdlib::_IO_marker,
    pub _chain: *mut crate::stdlib::_IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: crate::stdlib::__off_t,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: crate::stdlib::__off64_t,
    pub _codecvt: *mut crate::stdlib::_IO_codecvt,
    pub _wide_data: *mut crate::stdlib::_IO_wide_data,
    pub _freeres_list: *mut crate::stdlib::_IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: crate::stddef_h::size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
// ================ END struct_FILE_h ================
// =============== BEGIN struct_timespec_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec {
    pub tv_sec: crate::stdlib::__time_t,
    pub tv_nsec: crate::stdlib::__syscall_slong_t,
}
// ================ END struct_timespec_h ================
// =============== BEGIN sys_types_h ================
pub type uid_t = crate::stdlib::__uid_t;
pub type ssize_t = crate::stdlib::__ssize_t;
// ================ END sys_types_h ================
// =============== BEGIN thread_shared_types_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_internal_list {
    pub __prev: *mut crate::stdlib::__pthread_internal_list,
    pub __next: *mut crate::stdlib::__pthread_internal_list,
}
pub type __pthread_list_t = crate::stdlib::__pthread_internal_list;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_mutex_s {
    pub __lock: i32,
    pub __count: u32,
    pub __owner: i32,
    pub __nusers: u32,
    pub __kind: i32,
    pub __spins: i16,
    pub __elision: i16,
    pub __list: crate::stdlib::__pthread_list_t,
}
// ================ END thread_shared_types_h ================
// =============== BEGIN time_t_h ================
pub type time_t = crate::stdlib::__time_t;
// ================ END time_t_h ================
// =============== BEGIN types_h ================
pub type __int32_t = i32;
pub type __off_t = isize;
pub type __off64_t = isize;
pub type __uint32_t = u32;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __dev_t = usize;
pub type __ino_t = usize;
pub type __mode_t = u32;
pub type __nlink_t = usize;
pub type __blksize_t = isize;
pub type __blkcnt_t = isize;
pub type __syscall_slong_t = isize;
pub type __uint64_t = usize;
pub type __time_t = isize;
pub type __ssize_t = isize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int64_t = isize;
