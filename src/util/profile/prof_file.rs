use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:6"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:6"]
pub mod types_h {
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = libc::c_ulong;
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
    #[c2rust::src_loc = "196:1"]
    pub type __syscall_slong_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:6"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::__uint64_t;
}
#[c2rust::header_src = "/usr/include/sys/types.h:6"]
pub mod sys_types_h {
    #[c2rust::src_loc = "79:1"]
    pub type uid_t = __uid_t;
    use super::types_h::__uid_t;
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:6"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:6"]
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
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:6"]
pub mod thread_shared_types_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:16"]
    pub struct __pthread_internal_list {
        pub __prev: *mut __pthread_internal_list,
        pub __next: *mut __pthread_internal_list,
    }
    #[c2rust::src_loc = "82:1"]
    pub type __pthread_list_t = __pthread_internal_list;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:8"]
    pub struct __pthread_mutex_s {
        pub __lock: libc::c_int,
        pub __count: libc::c_uint,
        pub __owner: libc::c_int,
        pub __nusers: libc::c_uint,
        pub __kind: libc::c_int,
        pub __spins: libc::c_short,
        pub __elision: libc::c_short,
        pub __list: __pthread_list_t,
    }
}
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:6"]
pub mod pthreadtypes_h {
    #[c2rust::src_loc = "53:1"]
    pub type pthread_once_t = libc::c_int;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "67:9"]
    pub union pthread_mutex_t {
        pub __data: __pthread_mutex_s,
        pub __size: [libc::c_char; 40],
        pub __align: libc::c_long,
    }
    use super::thread_shared_types_h::__pthread_mutex_s;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:6"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:6"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/stat.h:6"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:6"]
pub mod k5_thread_h {
    #[c2rust::src_loc = "188:1"]
    pub type k5_os_nothread_once_t = libc::c_uchar;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "256:9"]
    pub struct k5_once_t {
        pub o: pthread_once_t,
        pub n: k5_os_nothread_once_t,
    }
    #[c2rust::src_loc = "283:1"]
    pub type k5_os_mutex = pthread_mutex_t;
    #[c2rust::src_loc = "354:1"]
    pub type k5_mutex_t = k5_os_mutex;
    #[inline]
    #[c2rust::src_loc = "356:1"]
    pub unsafe extern "C" fn k5_mutex_init(mut m: *mut k5_mutex_t)
     -> libc::c_int {
        return k5_os_mutex_init(m);
    }
    #[inline]
    #[c2rust::src_loc = "360:1"]
    pub unsafe extern "C" fn k5_mutex_finish_init(mut m: *mut k5_mutex_t)
     -> libc::c_int {
        return 0 as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "367:1"]
    pub unsafe extern "C" fn k5_mutex_lock(mut m: *mut k5_mutex_t) {
        let mut r: libc::c_int = k5_os_mutex_lock(m);
        if r != 0 as libc::c_int {
            fprintf(stderr,
                    b"k5_mutex_lock: Received error %d (%s)\n\x00" as
                        *const u8 as *const libc::c_char, r, strerror(r));
        }
        if r == 0 as libc::c_int {
        } else {
            __assert_fail(b"r == 0\x00" as *const u8 as *const libc::c_char,
                          b"../../include/k5-thread.h\x00" as *const u8 as
                              *const libc::c_char,
                          376 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 33],
                                                    &[libc::c_char; 33]>(b"void k5_mutex_lock(k5_mutex_t *)\x00")).as_ptr());
        };
    }
    #[inline]
    #[c2rust::src_loc = "379:1"]
    pub unsafe extern "C" fn k5_mutex_unlock(mut m: *mut k5_mutex_t) {
        let mut r: libc::c_int = k5_os_mutex_unlock(m);
        if r != 0 as libc::c_int {
            fprintf(stderr,
                    b"k5_mutex_unlock: Received error %d (%s)\n\x00" as
                        *const u8 as *const libc::c_char, r, strerror(r));
        }
        if r == 0 as libc::c_int {
        } else {
            __assert_fail(b"r == 0\x00" as *const u8 as *const libc::c_char,
                          b"../../include/k5-thread.h\x00" as *const u8 as
                              *const libc::c_char,
                          388 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 35],
                                                    &[libc::c_char; 35]>(b"void k5_mutex_unlock(k5_mutex_t *)\x00")).as_ptr());
        };
    }
    use super::pthreadtypes_h::{pthread_once_t, pthread_mutex_t};
    use super::stdio_h::{fprintf, stderr};
    use super::string_h::strerror;
    use super::assert_h::__assert_fail;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "262:1"]
        pub fn k5_once(once: *mut k5_once_t,
                       fn_0: Option<unsafe extern "C" fn() -> ()>)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "290:1"]
        pub fn k5_os_mutex_init(m: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "291:1"]
        pub fn k5_os_mutex_destroy(m: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "292:1"]
        pub fn k5_os_mutex_lock(m: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "293:1"]
        pub fn k5_os_mutex_unlock(m: *mut k5_os_mutex) -> libc::c_int;
    }
    /* multiple inclusion? */
    /* In time, many of the definitions above should move into the support
   library, and this file should be greatly simplified.  For type
   definitions, that'll take some work, since other data structures
   incorporate mutexes directly, and our mutex type is dependent on
   configuration options and system attributes.  For most functions,
   though, it should be relatively easy.

   For now, plugins should use the exported functions, and not the
   above macros, and use krb5int_mutex_alloc for allocations.  */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:6"]
pub mod k5_platform_h {
    /* -*- mode: c; indent-tabs-mode: nil -*- */
/* include/k5-platform.h */
/*
 * Copyright 2003, 2004, 2005, 2007, 2008, 2009 Massachusetts Institute of Technology.
 * All Rights Reserved.
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
    /*
 * Some platform-dependent definitions to sync up the C support level.
 * Some to a C99-ish level, some related utility code.
 *
 * Currently:
 * + [u]int{8,16,32}_t types
 * + 64-bit types and load/store code
 * + SIZE_MAX
 * + shared library init/fini hooks
 * + consistent getpwnam/getpwuid interfaces
 * + va_copy fudged if not provided
 * + strlcpy/strlcat
 * + fnmatch
 * + [v]asprintf
 * + strerror_r
 * + mkstemp
 * + zap (support function and macro)
 * + constant time memory comparison
 * + path manipulation
 * + _, N_, dgettext, bindtextdomain (for localization)
 * + getopt_long
 * + secure_getenv
 * + fetching filenames from a directory
 */
    /* This attribute prevents unused function warnings in gcc and clang. */
    /* Initialization and finalization function support for libraries.

   At top level, before the functions are defined or even declared:
   MAKE_INIT_FUNCTION(init_fn);
   MAKE_FINI_FUNCTION(fini_fn);
   Then:
   int init_fn(void) { ... }
   void fini_fn(void) { if (INITIALIZER_RAN(init_fn)) ... }
   In code, in the same file:
   err = CALL_INIT_FUNCTION(init_fn);

   To trigger or verify the initializer invocation from another file,
   a helper function must be created.

   This model handles both the load-time execution (Windows) and
   delayed execution (pthread_once) approaches, and should be able to
   guarantee in both cases that the init function is run once, in one
   thread, before other stuff in the library is done; furthermore, the
   finalization code should only run if the initialization code did.
   (Maybe I could've made the "if INITIALIZER_RAN" test implicit, via
   another function hidden in macros, but this is hairy enough
   already.)

   The init_fn and fini_fn names should be chosen such that any
   exported names staring with those names, and optionally followed by
   additional characters, fits in with any namespace constraints on
   the library in question.


   There's also PROGRAM_EXITING() currently always defined as zero.
   If there's some trivial way to find out if the fini function is
   being called because the program that the library is linked into is
   exiting, we can just skip all the work because the resources are
   about to be freed up anyways.  Generally this is likely to be the
   same as distinguishing whether the library was loaded dynamically
   while the program was running, or loaded as part of program
   startup.  On most platforms, I don't think we can distinguish these
   cases easily, and it's probably not worth expending any significant
   effort.  (Note in particular that atexit() won't do, because if the
   library is explicitly loaded and unloaded, it would have to be able
   to deregister the atexit callback function.  Also, the system limit
   on atexit callbacks may be small.)


   Implementation outline:

   Windows: MAKE_FINI_FUNCTION creates a symbol with a magic name that
   is sought at library build time, and code is added to invoke the
   function when the library is unloaded.  MAKE_INIT_FUNCTION does
   likewise, but the function is invoked when the library is loaded,
   and an extra variable is declared to hold an error code and a "yes
   the initializer ran" flag.  CALL_INIT_FUNCTION blows up if the flag
   isn't set, otherwise returns the error code.

   UNIX: MAKE_INIT_FUNCTION creates and initializes a variable with a
   name derived from the function name, containing a k5_once_t
   (pthread_once_t or int), an error code, and a pointer to the
   function.  The function itself is declared static, but the
   associated variable has external linkage.  CALL_INIT_FUNCTION
   ensures thath the function is called exactly once (pthread_once or
   just check the flag) and returns the stored error code (or the
   pthread_once error).

   (That's the basic idea.  With some debugging assert() calls and
   such, it's a bit more complicated.  And we also need to handle
   doing the pthread test at run time on systems where that works, so
   we use the k5_once_t stuff instead.)

   UNIX, with compiler support: MAKE_FINI_FUNCTION declares the
   function as a destructor, and the run time linker support or
   whatever will cause it to be invoked when the library is unloaded,
   the program ends, etc.

   UNIX, with linker support: MAKE_FINI_FUNCTION creates a symbol with
   a magic name that is sought at library build time, and linker
   options are used to mark it as a finalization function for the
   library.  The symbol must be exported.

   UNIX, no library finalization support: The finalization function
   never runs, and we leak memory.  Tough.

   DELAY_INITIALIZER will be defined by the configure script if we
   want to use k5_once instead of load-time initialization.  That'll
   be the preferred method on most systems except Windows, where we
   have to initialize some mutexes.




   For maximum flexibility in defining the macros, the function name
   parameter should be a simple name, not even a macro defined as
   another name.  The function should have a unique name, and should
   conform to whatever namespace is used by the library in question.
   (We do have export lists, but (1) they're not used for all
   platforms, and (2) they're not used for static libraries.)

   If the macro expansion needs the function to have been declared, it
   must include a declaration.  If it is not necessary for the symbol
   name to be exported from the object file, the macro should declare
   it as "static".  Hence the signature must exactly match "void
   foo(void)".  (ANSI C allows a static declaration followed by a
   non-static one; the result is internal linkage.)  The macro
   expansion has to come before the function, because gcc apparently
   won't act on "__attribute__((constructor))" if it comes after the
   function definition.

   This is going to be compiler- and environment-specific, and may
   require some support at library build time, and/or "asm"
   statements.  But through macro expansion and auxiliary functions,
   we should be able to handle most things except #pragma.

   It's okay for this code to require that the library be built
   with the same compiler and compiler options throughout, but
   we shouldn't require that the library and application use the
   same compiler.

   For static libraries, we don't really care about cleanup too much,
   since it's all memory handling and mutex allocation which will all
   be cleaned up when the program exits.  Thus, it's okay if gcc-built
   static libraries don't play nicely with cc-built executables when
   it comes to static constructors, just as long as it doesn't cause
   linking to fail.

   For dynamic libraries on UNIX, we'll use pthread_once-type support
   to do delayed initialization, so if finalization can't be made to
   work, we'll only have memory leaks in a load/use/unload cycle.  If
   anyone (like, say, the OS vendor) complains about this, they can
   tell us how to get a shared library finalization function invoked
   automatically.

   Currently there's --disable-delayed-initialization for preventing
   the initialization from being delayed on UNIX, but that's mainly
   just for testing the linker options for initialization, and will
   probably be removed at some point.  */
    /* Helper macros.  */
    /* XXX Should test USE_LINKER_INIT_OPTION early, and if it's set,
   always provide a function by the expected name, even if we're
   delaying initialization.  */
    /* Run the initialization code during program execution, at the latest
   possible moment.  This means multiple threads may be active.  */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "237:9"]
    pub struct k5_init_t {
        pub once: k5_once_t,
        pub error: libc::c_int,
        pub did_run: libc::c_int,
        pub fn_0: Option<unsafe extern "C" fn() -> ()>,
    }
    use super::k5_thread_h::k5_once_t;
    use super::stddef_h::size_t;
    extern "C" {
        /* Make the interfaces to getpwnam and getpwuid consistent.
   Model the wrappers on the POSIX thread-safe versions, but
   use the unsafe system versions if the safe ones don't exist
   or we can't figure out their interfaces.  */
        /* int k5_getpwnam_r(const char *, blah blah) */
        /* POSIX */
        /* no getpwnam_r, or can't figure out #args or return type */
        /* int k5_getpwuid_r(uid_t, blah blah) */
        /* POSIX */
        /* no getpwuid_r, or can't figure out #args or return type */
        /* Ensure, if possible, that the indicated file descriptor won't be
   kept open if we exec another process (e.g., launching a ccapi
   server).  If we don't know how to do it... well, just go about our
   business.  Probably most callers won't check the return status
   anyways.  */
        /* Macros make the Sun compiler happier, and all variants of this do a
   single evaluation of the argument, and fcntl and fileno should
   produce reasonable error messages on type mismatches, on any system
   with F_SETFD.  */
        /* Since the original ANSI C spec left it undefined whether or
   how you could copy around a va_list, C 99 added va_copy.
   For old implementations, let's do our best to fake it.

   XXX Doesn't yet handle implementations with __va_copy (early draft)
   or GCC's __builtin_va_copy.  */
        /* Do nothing.  */
        /* Provide strlcpy/strlcat interfaces. */
        #[no_mangle]
        #[c2rust::src_loc = "887:1"]
        pub fn krb5int_strlcpy(dst: *mut libc::c_char,
                               src: *const libc::c_char, siz: size_t)
         -> size_t;
    }
    /* K5_PLATFORM_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:6"]
pub mod com_err_h {
    /*
 * Copyright 1988, Student Information Processing Board of the
 * Massachusetts Institute of Technology.
 *
 * Copyright 1995 by Cygnus Support.
 *
 * For copyright and distribution info, see the documentation supplied
 * with this package.
 */
    /* Header file for common error description library. */
    #[c2rust::src_loc = "26:1"]
    pub type errcode_t = libc::c_long;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "30:8"]
    pub struct error_table {
        pub msgs: *const *const libc::c_char,
        pub base: libc::c_long,
        pub n_msgs: libc::c_uint,
    }
    extern "C" {
        /*@modifies internalState@*/
        #[no_mangle]
        #[c2rust::src_loc = "57:1"]
        pub fn add_error_table(_: *const error_table) -> errcode_t;
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/profile/prof_int.h:6"]
pub mod prof_int_h {
    #[c2rust::src_loc = "71:1"]
    pub type prf_file_t = *mut _prf_file_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "65:8"]
    pub struct _prf_file_t {
        pub magic: prf_magic_t,
        pub data: *mut _prf_data_t,
        pub next: *mut _prf_file_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "30:8"]
    pub struct _prf_data_t {
        pub magic: prf_magic_t,
        pub lock: k5_mutex_t,
        pub root: *mut profile_node,
        pub last_stat: time_t,
        pub timestamp: time_t,
        pub frac_ts: libc::c_ulong,
        pub flags: libc::c_int,
        pub upd_serial: libc::c_int,
        pub fslen: size_t,
        pub pad: C2RustUnnamed,
        pub refcount: libc::c_int,
        pub next: *mut _prf_data_t,
        pub filespec: [libc::c_char; 15],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:2"]
    pub union C2RustUnnamed {
        pub d: libc::c_double,
        pub p: *mut libc::c_void,
        pub ll: uint64_t,
        pub m: k5_mutex_t,
    }
    #[c2rust::src_loc = "19:1"]
    pub type prf_magic_t = libc::c_long;
    #[c2rust::src_loc = "62:1"]
    pub type prf_data_t = *mut _prf_data_t;
    use super::k5_thread_h::k5_mutex_t;
    use super::time_t_h::time_t;
    use super::stddef_h::size_t;
    use super::stdint_uintn_h::uint64_t;
    use super::FILE_h::FILE;
    use super::com_err_h::errcode_t;
    extern "C" {
        #[c2rust::src_loc = "33:9"]
        pub type profile_node;
        #[no_mangle]
        #[c2rust::src_loc = "120:1"]
        pub fn profile_parse_file(f: *mut FILE, root: *mut *mut profile_node,
                                  ret_modspec: *mut *mut libc::c_char)
         -> errcode_t;
        #[no_mangle]
        #[c2rust::src_loc = "123:1"]
        pub fn profile_process_directory(dirname: *const libc::c_char,
                                         root: *mut *mut profile_node)
         -> errcode_t;
        #[no_mangle]
        #[c2rust::src_loc = "126:1"]
        pub fn profile_write_tree_file(root: *mut profile_node,
                                       dstfile: *mut FILE) -> errcode_t;
        #[no_mangle]
        #[c2rust::src_loc = "129:1"]
        pub fn profile_write_tree_to_buffer(root: *mut profile_node,
                                            buf: *mut *mut libc::c_char)
         -> errcode_t;
        #[no_mangle]
        #[c2rust::src_loc = "135:1"]
        pub fn profile_free_node(relation: *mut profile_node);
    }
    /* prof_set.c -- included from profile.h */
    /* Others included from profile.h */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/profile/profile.h:6"]
pub mod profile_h {
    #[c2rust::src_loc = "40:1"]
    pub type profile_filespec_t = *mut libc::c_char;
    #[c2rust::src_loc = "42:1"]
    pub type const_profile_filespec_t = *const libc::c_char;
    use super::com_err_h::error_table;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "343:33"]
        pub static et_prof_error_table: error_table;
    }
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/pwd.h:23"]
pub mod pwd_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct passwd {
        pub pw_name: *mut libc::c_char,
        pub pw_passwd: *mut libc::c_char,
        pub pw_uid: __uid_t,
        pub pw_gid: __gid_t,
        pub pw_gecos: *mut libc::c_char,
        pub pw_dir: *mut libc::c_char,
        pub pw_shell: *mut libc::c_char,
    }
    use super::types_h::{__uid_t, __gid_t};
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "145:1"]
        pub fn getpwuid_r(__uid: __uid_t, __resultbuf: *mut passwd,
                          __buffer: *mut libc::c_char, __buflen: size_t,
                          __result: *mut *mut passwd) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:6"]
pub mod unistd_h {
    use super::types_h::__uid_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "287:1"]
        pub fn access(__name: *const libc::c_char, __type: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "675:1"]
        pub fn getuid() -> __uid_t;
        #[no_mangle]
        #[c2rust::src_loc = "789:1"]
        pub fn link(__from: *const libc::c_char, __to: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "825:1"]
        pub fn unlink(__name: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "970:1"]
        pub fn sync();
    }
}
#[c2rust::header_src = "/usr/include/assert.h:6"]
pub mod assert_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn __assert_fail(__assertion: *const libc::c_char,
                             __file: *const libc::c_char,
                             __line: libc::c_uint,
                             __function: *const libc::c_char) -> !;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:6"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "639:1"]
        pub fn secure_getenv(__name: *const libc::c_char)
         -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:6"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "148:1"]
        pub fn rename(__old: *const libc::c_char, __new: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "246:14"]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "786:1"]
        pub fn fileno(__stream: *mut FILE) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:6"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "175:1"]
        pub fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:6"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:6"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "396:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/time.h:6"]
pub mod time_h {
    use super::time_t_h::time_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "75:1"]
        pub fn time(__timer: *mut time_t) -> time_t;
    }
}
#[c2rust::header_src = "/usr/include/sys/stat.h:19"]
pub mod sys_stat_h {
    use super::stat_h::stat;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "205:1"]
        pub fn stat(__file: *const libc::c_char, __buf: *mut stat)
         -> libc::c_int;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint64_t, __dev_t, __uid_t, __gid_t, __ino_t,
                        __mode_t, __nlink_t, __off_t, __off64_t, __time_t,
                        __blksize_t, __blkcnt_t, __syscall_slong_t};
pub use self::stdint_uintn_h::uint64_t;
pub use self::sys_types_h::uid_t;
pub use self::time_t_h::time_t;
pub use self::struct_timespec_h::timespec;
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::{pthread_once_t, pthread_mutex_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stat_h::stat;
pub use self::k5_thread_h::{k5_os_nothread_once_t, k5_once_t, k5_os_mutex,
                            k5_mutex_t, k5_mutex_init, k5_mutex_finish_init,
                            k5_mutex_lock, k5_mutex_unlock, k5_once,
                            k5_os_mutex_init, k5_os_mutex_destroy,
                            k5_os_mutex_lock, k5_os_mutex_unlock};
pub use self::k5_platform_h::{k5_init_t, krb5int_strlcpy};
pub use self::com_err_h::{errcode_t, error_table, add_error_table};
pub use self::prof_int_h::{prf_file_t, _prf_file_t, _prf_data_t,
                           C2RustUnnamed, prf_magic_t, prf_data_t,
                           profile_node, profile_parse_file,
                           profile_process_directory, profile_write_tree_file,
                           profile_write_tree_to_buffer, profile_free_node};
pub use self::profile_h::{profile_filespec_t, const_profile_filespec_t,
                          et_prof_error_table};
pub use self::pwd_h::{passwd, getpwuid_r};
use self::unistd_h::{access, getuid, link, unlink, sync};
use self::assert_h::__assert_fail;
use self::stdlib_h::{malloc, free, secure_getenv};
use self::stdio_h::{stderr, rename, fclose, fopen, fprintf, asprintf, fileno};
use self::fcntl_h::fcntl;
use self::errno_h::__errno_location;
use self::string_h::{strcmp, memset, strdup, strlen, strerror};
use self::time_h::time;
use self::sys_stat_h::stat;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * prof_file.c ---- routines that manipulate an individual profile file.
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "37:8"]
pub struct global_shared_profile_data {
    pub trees: prf_data_t,
    pub mutex: k5_mutex_t,
}
#[c2rust::src_loc = "46:42"]
static mut krb5int_profile_shared_data: global_shared_profile_data =
    {
        let mut init =
            global_shared_profile_data{trees:
                                           0 as *const _prf_data_t as
                                               prf_data_t,
                                       mutex:
                                           pthread_mutex_t{__data:
                                                               {
                                                                   let mut init =
                                                                       __pthread_mutex_s{__lock:
                                                                                             0
                                                                                                 as
                                                                                                 libc::c_int,
                                                                                         __count:
                                                                                             0
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint,
                                                                                         __owner:
                                                                                             0
                                                                                                 as
                                                                                                 libc::c_int,
                                                                                         __nusers:
                                                                                             0
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint,
                                                                                         __kind:
                                                                                             0
                                                                                                 as
                                                                                                 libc::c_int,
                                                                                         __spins:
                                                                                             0
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_short,
                                                                                         __elision:
                                                                                             0
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_short,
                                                                                         __list:
                                                                                             {
                                                                                                 let mut init =
                                                                                                     __pthread_internal_list{__prev:
                                                                                                                                 0
                                                                                                                                     as
                                                                                                                                     *const __pthread_internal_list
                                                                                                                                     as
                                                                                                                                     *mut __pthread_internal_list,
                                                                                                                             __next:
                                                                                                                                 0
                                                                                                                                     as
                                                                                                                                     *const __pthread_internal_list
                                                                                                                                     as
                                                                                                                                     *mut __pthread_internal_list,}; /* Make sure to stat when updating. */
                                                                                                 init
                                                                                             },};
                                                                   init
                                                               },},};
        init
    };
#[c2rust::src_loc = "51:1"]
unsafe extern "C" fn profile_library_initializer__aux() {
    profile_library_initializer__once.did_run = 1 as libc::c_int;
    profile_library_initializer__once.error = profile_library_initializer();
}
#[c2rust::src_loc = "51:1"]
static mut profile_library_initializer__once: k5_init_t =
    unsafe {
        {
            let mut init =
                k5_init_t{once:
                              {
                                  let mut init =
                                      k5_once_t{o: 0 as libc::c_int,
                                                n:
                                                    2 as libc::c_int as
                                                        k5_os_nothread_once_t,};
                                  init
                              },
                          error: 0 as libc::c_int,
                          did_run: 0 as libc::c_int,
                          fn_0:
                              Some(profile_library_initializer__aux as
                                       unsafe extern "C" fn() -> ()),};
            init
        }
    };
#[c2rust::src_loc = "54:1"]
unsafe extern "C" fn profile_library_initializer() -> libc::c_int {
    add_error_table(&et_prof_error_table);
    return k5_mutex_finish_init(&mut krb5int_profile_shared_data.mutex);
}
#[c2rust::src_loc = "81:1"]
unsafe extern "C" fn rw_access(mut filespec: const_profile_filespec_t)
 -> libc::c_int {
    if access(filespec, 2 as libc::c_int) == 0 as libc::c_int {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
#[c2rust::src_loc = "105:1"]
unsafe extern "C" fn r_access(mut filespec: const_profile_filespec_t)
 -> libc::c_int {
    if access(filespec, 4 as libc::c_int) == 0 as libc::c_int {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
#[no_mangle]
#[c2rust::src_loc = "129:1"]
pub unsafe extern "C" fn profile_file_is_writable(mut profile: prf_file_t)
 -> libc::c_int {
    if !profile.is_null() && !(*profile).data.is_null() {
        return rw_access((*(*profile).data).filespec.as_ptr())
    } else { return 0 as libc::c_int };
}
#[no_mangle]
#[c2rust::src_loc = "138:1"]
pub unsafe extern "C" fn profile_make_prf_data(mut filename:
                                                   *const libc::c_char)
 -> prf_data_t {
    let mut d: prf_data_t = 0 as *mut _prf_data_t;
    let mut len: size_t = 0;
    let mut flen: size_t = 0;
    let mut slen: size_t = 0;
    let mut fcopy: *mut libc::c_char = 0 as *mut libc::c_char;
    flen = strlen(filename);
    slen = 152 as libc::c_ulong;
    len =
        slen.wrapping_add(flen).wrapping_add(1 as libc::c_int as
                                                 libc::c_ulong);
    if len < ::std::mem::size_of::<_prf_data_t>() as libc::c_ulong {
        len = ::std::mem::size_of::<_prf_data_t>() as libc::c_ulong
    }
    d = malloc(len) as prf_data_t;
    if d.is_null() { return 0 as prf_data_t }
    memset(d as *mut libc::c_void, 0 as libc::c_int, len);
    fcopy = (d as *mut libc::c_char).offset(slen as isize);
    if fcopy == (*d).filespec.as_ptr() as *mut libc::c_char {
    } else {
        __assert_fail(b"fcopy == d->filespec\x00" as *const u8 as
                          *const libc::c_char,
                      b"prof_file.c\x00" as *const u8 as *const libc::c_char,
                      155 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"prf_data_t profile_make_prf_data(const char *)\x00")).as_ptr());
    }
    krb5int_strlcpy(fcopy, filename,
                    flen.wrapping_add(1 as libc::c_int as libc::c_ulong));
    (*d).refcount = 1 as libc::c_int;
    (*d).magic = -(1429577698 as libc::c_long);
    (*d).root = 0 as *mut profile_node;
    (*d).next = 0 as *mut _prf_data_t;
    (*d).fslen = flen;
    return d;
}
#[no_mangle]
#[c2rust::src_loc = "165:1"]
pub unsafe extern "C" fn profile_open_file(mut filespec:
                                               const_profile_filespec_t,
                                           mut ret_prof: *mut prf_file_t,
                                           mut ret_modspec:
                                               *mut *mut libc::c_char)
 -> errcode_t {
    let mut prf: prf_file_t = 0 as *mut _prf_file_t;
    let mut retval: errcode_t = 0;
    let mut home_env: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut data: prf_data_t = 0 as *mut _prf_data_t;
    let mut expanded_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    retval =
        ({
             let mut k5int_i: *mut k5_init_t =
                 &mut profile_library_initializer__once;
             let mut k5int_err: libc::c_int =
                 k5_once(&mut (*k5int_i).once, (*k5int_i).fn_0);
             if k5int_err != 0 {
                 k5int_err
             } else {
                 if (*k5int_i).did_run != 0 as libc::c_int {
                 } else {
                     __assert_fail(b"k5int_i->did_run != 0\x00" as *const u8
                                       as *const libc::c_char,
                                   b"prof_file.c\x00" as *const u8 as
                                       *const libc::c_char,
                                   174 as libc::c_int as libc::c_uint,
                                   (*::std::mem::transmute::<&[u8; 77],
                                                             &[libc::c_char; 77]>(b"errcode_t profile_open_file(const_profile_filespec_t, prf_file_t *, char **)\x00")).as_ptr());
                 }
                 (*k5int_i).error
             }
         }) as errcode_t;
    if retval != 0 { return retval }
    prf =
        malloc(::std::mem::size_of::<_prf_file_t>() as libc::c_ulong) as
            prf_file_t;
    if prf.is_null() { return 12 as libc::c_int as errcode_t }
    memset(prf as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<_prf_file_t>() as libc::c_ulong);
    (*prf).magic = -(1429577703 as libc::c_long);
    if *filespec.offset(0 as libc::c_int as isize) as libc::c_int ==
           '~' as i32 &&
           *filespec.offset(1 as libc::c_int as isize) as libc::c_int ==
               '/' as i32 {
        home_env =
            secure_getenv(b"HOME\x00" as *const u8 as *const libc::c_char);
        if home_env.is_null() {
            let mut uid: uid_t = 0;
            let mut pw: *mut passwd = 0 as *mut passwd;
            let mut pwx: passwd =
                passwd{pw_name: 0 as *mut libc::c_char,
                       pw_passwd: 0 as *mut libc::c_char,
                       pw_uid: 0,
                       pw_gid: 0,
                       pw_gecos: 0 as *mut libc::c_char,
                       pw_dir: 0 as *mut libc::c_char,
                       pw_shell: 0 as *mut libc::c_char,};
            let mut pwbuf: [libc::c_char; 8192] = [0; 8192];
            uid = getuid();
            if (if getpwuid_r(uid, &mut pwx, pwbuf.as_mut_ptr(),
                              ::std::mem::size_of::<[libc::c_char; 8192]>() as
                                  libc::c_ulong, &mut pw) == 0 as libc::c_int
                   {
                    (if pw.is_null() {
                         -(1 as libc::c_int)
                     } else { 0 as libc::c_int })
                } else { -(1 as libc::c_int) }) == 0 && !pw.is_null() &&
                   *(*pw).pw_dir.offset(0 as libc::c_int as isize) as
                       libc::c_int != 0 as libc::c_int {
                home_env = (*pw).pw_dir
            }
        }
    }
    if !home_env.is_null() {
        if asprintf(&mut expanded_filename as *mut *mut libc::c_char,
                    b"%s%s\x00" as *const u8 as *const libc::c_char, home_env,
                    filespec.offset(1 as libc::c_int as isize)) <
               0 as libc::c_int {
            expanded_filename = 0 as *mut libc::c_char
        }
    } else { expanded_filename = strdup(filespec) }
    if expanded_filename.is_null() {
        free(prf as *mut libc::c_void);
        return 12 as libc::c_int as errcode_t
    }
    k5_mutex_lock(&mut krb5int_profile_shared_data.mutex);
    data = krb5int_profile_shared_data.trees;
    while !data.is_null() {
        if strcmp((*data).filespec.as_ptr(), expanded_filename) == 0 &&
               r_access((*data).filespec.as_ptr()) != 0 {
            break ;
        }
        data = (*data).next
    }
    if !data.is_null() {
        (*data).refcount += 1;
        (*data).last_stat = 0 as libc::c_int as time_t;
        k5_mutex_unlock(&mut krb5int_profile_shared_data.mutex);
        retval = profile_update_file_data(data, 0 as *mut *mut libc::c_char);
        free(expanded_filename as *mut libc::c_void);
        if retval != 0 {
            profile_dereference_data(data);
            free(prf as *mut libc::c_void);
            return retval
        }
        (*prf).data = data;
        *ret_prof = prf;
        return 0 as libc::c_int as errcode_t
    }
    k5_mutex_unlock(&mut krb5int_profile_shared_data.mutex);
    data = profile_make_prf_data(expanded_filename);
    if data.is_null() {
        free(prf as *mut libc::c_void);
        free(expanded_filename as *mut libc::c_void);
        return 12 as libc::c_int as errcode_t
    }
    free(expanded_filename as *mut libc::c_void);
    (*prf).data = data;
    retval = k5_mutex_init(&mut (*data).lock) as errcode_t;
    if retval != 0 {
        free(data as *mut libc::c_void);
        free(prf as *mut libc::c_void);
        return retval
    }
    retval = profile_update_file_data((*prf).data, ret_modspec);
    if retval != 0 { profile_close_file(prf); return retval }
    k5_mutex_lock(&mut krb5int_profile_shared_data.mutex);
    (*data).flags |= 0x4 as libc::c_int;
    (*data).next = krb5int_profile_shared_data.trees;
    krb5int_profile_shared_data.trees = data;
    k5_mutex_unlock(&mut krb5int_profile_shared_data.mutex);
    *ret_prof = prf;
    return 0 as libc::c_int as errcode_t;
}
#[no_mangle]
#[c2rust::src_loc = "265:1"]
pub unsafe extern "C" fn profile_update_file_data_locked(mut data: prf_data_t,
                                                         mut ret_modspec:
                                                             *mut *mut libc::c_char)
 -> errcode_t {
    let mut retval: errcode_t = 0;
    let mut st: stat =
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
    let mut frac: libc::c_ulong = 0;
    let mut now: time_t = 0;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut isdir: libc::c_int = 0 as libc::c_int;
    if (*data).flags & 0x1 as libc::c_int != 0 && !(*data).root.is_null() {
        return 0 as libc::c_int as errcode_t
    }
    now = time(0 as *mut time_t);
    if now == (*data).last_stat && !(*data).root.is_null() {
        return 0 as libc::c_int as errcode_t
    }
    if stat((*data).filespec.as_ptr(), &mut st) != 0 {
        return *__errno_location() as errcode_t
    }
    (*data).last_stat = now;
    frac = st.st_mtim.tv_nsec as libc::c_ulong;
    if st.st_mtim.tv_sec == (*data).timestamp && frac == (*data).frac_ts &&
           !(*data).root.is_null() {
        return 0 as libc::c_int as errcode_t
    }
    if !(*data).root.is_null() {
        profile_free_node((*data).root);
        (*data).root = 0 as *mut profile_node
    }
    /* Only try to reload regular files, not devices such as pipes. */
    if st.st_mode & 0o170000 as libc::c_int as libc::c_uint !=
           0o100000 as libc::c_int as libc::c_uint {
        (*data).flags |= 0x1 as libc::c_int
    }
    isdir =
        (st.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
             0o40000 as libc::c_int as libc::c_uint) as libc::c_int;
    if isdir == 0 {
        *__errno_location() = 0 as libc::c_int;
        f =
            fopen((*data).filespec.as_ptr(),
                  b"r\x00" as *const u8 as *const libc::c_char);
        if f.is_null() {
            return if *__errno_location() != 0 as libc::c_int {
                       *__errno_location()
                   } else { 2 as libc::c_int } as errcode_t
        }
        fcntl(fileno(f), 2 as libc::c_int, 1 as libc::c_int);
    }
    (*data).upd_serial += 1;
    (*data).flags &= !(0x2 as libc::c_int);
    if isdir != 0 {
        retval =
            profile_process_directory((*data).filespec.as_ptr(),
                                      &mut (*data).root)
    } else {
        retval = profile_parse_file(f, &mut (*data).root, ret_modspec);
        fclose(f);
    }
    if retval != 0 { return retval }
    if !(*data).root.is_null() {
    } else {
        __assert_fail(b"data->root != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"prof_file.c\x00" as *const u8 as *const libc::c_char,
                      344 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 63],
                                                &[libc::c_char; 63]>(b"errcode_t profile_update_file_data_locked(prf_data_t, char **)\x00")).as_ptr());
    }
    (*data).timestamp = st.st_mtim.tv_sec;
    (*data).frac_ts = frac;
    return 0 as libc::c_int as errcode_t;
}
#[no_mangle]
#[c2rust::src_loc = "352:1"]
pub unsafe extern "C" fn profile_update_file_data(mut data: prf_data_t,
                                                  mut ret_modspec:
                                                      *mut *mut libc::c_char)
 -> errcode_t {
    let mut retval: errcode_t = 0;
    k5_mutex_lock(&mut (*data).lock);
    retval = profile_update_file_data_locked(data, ret_modspec);
    k5_mutex_unlock(&mut (*data).lock);
    return retval;
}
#[c2rust::src_loc = "362:1"]
unsafe extern "C" fn make_hard_link(mut oldpath: *const libc::c_char,
                                    mut newpath: *const libc::c_char)
 -> libc::c_int {
    return link(oldpath, newpath);
}
#[c2rust::src_loc = "372:1"]
unsafe extern "C" fn write_data_to_file(mut data: prf_data_t,
                                        mut outfile: *const libc::c_char,
                                        mut can_create: libc::c_int)
 -> errcode_t {
    let mut current_block: u64;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut new_file: profile_filespec_t = 0 as *mut libc::c_char;
    let mut old_file: profile_filespec_t = 0 as *mut libc::c_char;
    let mut retval: errcode_t = 0 as libc::c_int as errcode_t;
    retval = 12 as libc::c_int as errcode_t;
    old_file = 0 as profile_filespec_t;
    new_file = old_file;
    if asprintf(&mut new_file as *mut profile_filespec_t,
                b"%s.$$$\x00" as *const u8 as *const libc::c_char, outfile) <
           0 as libc::c_int {
        new_file = 0 as profile_filespec_t
    } else if asprintf(&mut old_file as *mut profile_filespec_t,
                       b"%s.bak\x00" as *const u8 as *const libc::c_char,
                       outfile) < 0 as libc::c_int {
        old_file = 0 as profile_filespec_t
    } else {
        *__errno_location() = 0 as libc::c_int;
        f =
            fopen(new_file as *const libc::c_char,
                  b"w\x00" as *const u8 as *const libc::c_char);
        if f.is_null() {
            retval = *__errno_location() as errcode_t;
            if retval == 0 as libc::c_int as libc::c_long {
                retval = -(1429577702 as libc::c_long)
            }
        } else {
            fcntl(fileno(f), 2 as libc::c_int, 1 as libc::c_int);
            profile_write_tree_file((*data).root, f);
            if fclose(f) != 0 as libc::c_int {
                retval = *__errno_location() as errcode_t
            } else {
                unlink(old_file as *const libc::c_char);
                if make_hard_link(outfile, old_file as *const libc::c_char) ==
                       0 as libc::c_int {
                    /* Okay, got the hard link.  Yay.  Now we've got our
           backup version, so just put the new version in
           place.  */
                    if rename(new_file as *const libc::c_char, outfile) != 0 {
                        /* Weird, the rename didn't work.  But the old version
               should still be in place, so no special cleanup is
               needed.  */
                        retval = *__errno_location() as errcode_t;
                        current_block = 160786919731145636;
                    } else { current_block = 2604890879466389055; }
                } else if *__errno_location() == 2 as libc::c_int &&
                              can_create != 0 {
                    if rename(new_file as *const libc::c_char, outfile) != 0 {
                        retval = *__errno_location() as errcode_t;
                        current_block = 160786919731145636;
                    } else { current_block = 2604890879466389055; }
                } else {
                    /* Couldn't make the hard link, so there's going to be a
           small window where data->filespec does not refer to
           either version.  */
                    sync(); /* back out... */
                    if rename(outfile, old_file as *const libc::c_char) != 0 {
                        retval = *__errno_location() as errcode_t;
                        current_block = 160786919731145636;
                    } else if rename(new_file as *const libc::c_char, outfile)
                                  != 0 {
                        retval = *__errno_location() as errcode_t;
                        rename(old_file as *const libc::c_char, outfile);
                        current_block = 160786919731145636;
                    } else { current_block = 2604890879466389055; }
                }
                match current_block {
                    160786919731145636 => { }
                    _ => { retval = 0 as libc::c_int as errcode_t }
                }
            }
        }
    }
    if !new_file.is_null() { free(new_file as *mut libc::c_void); }
    if !old_file.is_null() { free(old_file as *mut libc::c_void); }
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "454:1"]
pub unsafe extern "C" fn profile_flush_file_data_to_buffer(mut data:
                                                               prf_data_t,
                                                           mut bufp:
                                                               *mut *mut libc::c_char)
 -> errcode_t {
    let mut retval: errcode_t = 0;
    k5_mutex_lock(&mut (*data).lock);
    retval = profile_write_tree_to_buffer((*data).root, bufp);
    k5_mutex_unlock(&mut (*data).lock);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "464:1"]
pub unsafe extern "C" fn profile_flush_file_data(mut data: prf_data_t)
 -> errcode_t {
    let mut retval: errcode_t = 0 as libc::c_int as errcode_t;
    if data.is_null() || (*data).magic != -(1429577698 as libc::c_long) {
        return -(1429577698 as libc::c_long)
    }
    k5_mutex_lock(&mut (*data).lock);
    if (*data).flags & 0x2 as libc::c_int == 0 as libc::c_int {
        k5_mutex_unlock(&mut (*data).lock);
        return 0 as libc::c_int as errcode_t
    }
    retval =
        write_data_to_file(data, (*data).filespec.as_ptr(), 0 as libc::c_int);
    (*data).flags &= !(0x2 as libc::c_int);
    k5_mutex_unlock(&mut (*data).lock);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "484:1"]
pub unsafe extern "C" fn profile_flush_file_data_to_file(mut data: prf_data_t,
                                                         mut outfile:
                                                             *const libc::c_char)
 -> errcode_t {
    let mut retval: errcode_t = 0 as libc::c_int as errcode_t;
    if data.is_null() || (*data).magic != -(1429577698 as libc::c_long) {
        return -(1429577698 as libc::c_long)
    }
    k5_mutex_lock(&mut (*data).lock);
    retval = write_data_to_file(data, outfile, 1 as libc::c_int);
    k5_mutex_unlock(&mut (*data).lock);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "499:1"]
pub unsafe extern "C" fn profile_dereference_data(mut data: prf_data_t) {
    k5_mutex_lock(&mut krb5int_profile_shared_data.mutex);
    profile_dereference_data_locked(data);
    k5_mutex_unlock(&mut krb5int_profile_shared_data.mutex);
}
#[no_mangle]
#[c2rust::src_loc = "505:1"]
pub unsafe extern "C" fn profile_dereference_data_locked(mut data:
                                                             prf_data_t) {
    (*data).refcount -= 1;
    if (*data).refcount == 0 as libc::c_int { profile_free_file_data(data); };
}
#[no_mangle]
#[c2rust::src_loc = "512:1"]
pub unsafe extern "C" fn profile_lock_global() {
    k5_mutex_lock(&mut krb5int_profile_shared_data.mutex);
}
#[no_mangle]
#[c2rust::src_loc = "516:1"]
pub unsafe extern "C" fn profile_unlock_global() {
    k5_mutex_unlock(&mut krb5int_profile_shared_data.mutex);
}
#[no_mangle]
#[c2rust::src_loc = "521:1"]
pub unsafe extern "C" fn profile_free_file(mut prf: prf_file_t) {
    profile_dereference_data((*prf).data);
    free(prf as *mut libc::c_void);
}
/* Call with mutex locked!  */
#[c2rust::src_loc = "528:1"]
unsafe extern "C" fn profile_free_file_data(mut data: prf_data_t) {
    if (*data).flags & 0x4 as libc::c_int != 0 {
        /* Remove from linked list.  */
        if krb5int_profile_shared_data.trees == data {
            krb5int_profile_shared_data.trees = (*data).next
        } else {
            let mut prev: prf_data_t = 0 as *mut _prf_data_t;
            let mut next: prf_data_t = 0 as *mut _prf_data_t;
            prev = krb5int_profile_shared_data.trees;
            next = (*prev).next;
            while !next.is_null() {
                if next == data {
                    (*prev).next = (*next).next;
                    break ;
                } else { prev = next; next = (*next).next }
            }
        }
    }
    if !(*data).root.is_null() { profile_free_node((*data).root); }
    (*data).magic = 0 as libc::c_int as prf_magic_t;
    k5_os_mutex_destroy(&mut (*data).lock);
    free(data as *mut libc::c_void);
}
/*
 * prof-int.h
 */
/*
 * This is the structure which stores the profile information for a
 * particular configuration file.
 *
 * Locking strategy:
 * - filespec, fslen are fixed after creation
 * - refcount and next should only be tweaked with the global lock held
 * - other fields can be tweaked after grabbing the in-struct lock
 */
/* time tree was last updated from file */
/* fractional part of timestamp, if any */
/* r/w, dirty */
/* incremented when data changes */
/* Some separation between fields controlled by different
	   mutexes.  Theoretically, both could be accessed at the same
	   time from different threads on different CPUs with separate
	   caches.  Don't let the threads clobber each other's
	   changes.  One mutex controlling the whole thing would be
	   better, but sufficient separation might suffice.

	   This is icky.  I just hope it's adequate.

	   For next major release, fix this.  */
/* prf_file_t references */
/* Was: "profile_filespec_t filespec".  Now: flexible char
	   array ... except, we need to work in C89, so an array
	   length must be specified.  */
/*
 * The profile flags
 */
/*
 * This structure defines the high-level, user visible profile_t
 * object, which is used as a handle by users who need to query some
 * configuration file(s)
 */
/* If non-null, use vtable operations instead of native ones. */
/*
 * Used by the profile iterator in prof_get.c
 */
/*
 * Check if a filespec is last in a list (NULL on UNIX, invalid FSSpec on MacOS
 */
/* profile_parse.c */
/* prof_tree.c */
/* prof_file.c */
#[no_mangle]
#[c2rust::src_loc = "555:1"]
pub unsafe extern "C" fn profile_close_file(mut prf: prf_file_t)
 -> errcode_t {
    let mut retval: errcode_t = 0;
    retval =
        if !prf.is_null() && (*prf).magic == -(1429577703 as libc::c_long) {
            profile_flush_file_data((*prf).data)
        } else { -(1429577703 as libc::c_long) };
    if retval != 0 { return retval }
    profile_free_file(prf);
    return 0 as libc::c_int as errcode_t;
}
