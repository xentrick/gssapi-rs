use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:41"]
pub mod types_h {
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
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
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/sys/types.h:41"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    use super::types_h::__u_int;
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:41"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:41"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:41"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:41"]
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
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:41"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:41"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:41"]
pub mod k5_thread_h {
    /* Values:
   2 - function has not been run
   3 - function has been run
   4 - function is being run -- deadlock detected */
    #[c2rust::src_loc = "188:1"]
    pub type k5_os_nothread_once_t = libc::c_uchar;
    /* Weak reference support, etc.

   Linux: Stub mutex routines exist, but pthread_once does not.

   Solaris <10: In libc there's a pthread_once that doesn't seem to do
   anything.  Bleah.  But pthread_mutexattr_setrobust_np is defined
   only in libpthread.  However, some version of GNU libc (Red Hat's
   Fedora Core 5, reportedly) seems to have that function, but no
   declaration, so we'd have to declare it in order to test for its
   address.  We now have tests to see if pthread_once actually works,
   so stick with that for now.

   Solaris 10: The real thread support now lives in libc, and
   libpthread is just a filter object.  So we might as well use the
   real functions unconditionally.  Since we haven't got a test for
   this property yet, we use NO_WEAK_PTHREADS defined in aclocal.m4
   depending on the OS type.

   IRIX 6.5 stub pthread support in libc is really annoying.  The
   pthread_mutex_lock function returns ENOSYS for a program not linked
   against -lpthread.  No link-time failure, no weak symbols, etc.
   The C library doesn't provide pthread_once; we can use weak
   reference support for that.

   If weak references are not available, then for now, we assume that
   the pthread support routines will always be available -- either the
   real thing, or functional stubs that merely prohibit creating
   threads.

   If we find a platform with non-functional stubs and no weak
   references, we may have to resort to some hack like dlsym on the
   symbol tables of the current process.  */
    /* Can't rely on useful stubs -- see above regarding Solaris.  */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "256:9"]
    pub struct k5_once_t {
        pub o: pthread_once_t,
        pub n: k5_os_nothread_once_t,
    }
    #[c2rust::src_loc = "283:1"]
    pub type k5_os_mutex = pthread_mutex_t;
    /* is pthreads always available? */
    #[c2rust::src_loc = "354:1"]
    pub type k5_mutex_t = k5_os_mutex;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:41"]
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
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "644:5"]
    pub struct C2RustUnnamed {
        pub i: uint16_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "657:5"]
    pub struct C2RustUnnamed_0 {
        pub i: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "691:12"]
    pub struct C2RustUnnamed_1 {
        pub i: uint16_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "703:12"]
    pub struct C2RustUnnamed_2 {
        pub i: uint32_t,
    }
    #[inline]
    #[c2rust::src_loc = "639:1"]
    pub unsafe extern "C" fn store_16_le(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed)).i = val as uint16_t;
    }
    #[inline]
    #[c2rust::src_loc = "652:1"]
    pub unsafe extern "C" fn store_32_le(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed_0)).i = val;
    }
    #[inline]
    #[c2rust::src_loc = "686:1"]
    pub unsafe extern "C" fn load_16_le(mut cvp: *const libc::c_void)
     -> libc::c_ushort {
        let mut p: *const libc::c_uchar = cvp as *const libc::c_uchar;
        return (*(p as *const C2RustUnnamed_1)).i;
    }
    #[inline]
    #[c2rust::src_loc = "698:1"]
    pub unsafe extern "C" fn load_32_le(mut cvp: *const libc::c_void)
     -> libc::c_uint {
        let mut p: *const libc::c_uchar = cvp as *const libc::c_uchar;
        return (*(p as *const C2RustUnnamed_2)).i;
    }
    use super::k5_thread_h::k5_once_t;
    use super::stdint_uintn_h::{uint16_t, uint32_t};
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:41"]
pub mod krb5_h {
    /* typedef struct _profile_t *profile_t; */
    /*
 * begin wordsize.h
 */
    /*
 * Word-size related definition.
 */
    #[c2rust::src_loc = "136:1"]
    pub type krb5_octet = uint8_t;
    #[c2rust::src_loc = "137:1"]
    pub type krb5_int16 = int16_t;
    #[c2rust::src_loc = "138:1"]
    pub type krb5_ui_2 = uint16_t;
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "175:1"]
    pub type krb5_msgtype = libc::c_uint;
    #[c2rust::src_loc = "176:1"]
    pub type krb5_kvno = libc::c_uint;
    #[c2rust::src_loc = "178:1"]
    pub type krb5_addrtype = krb5_int32;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "181:1"]
    pub type krb5_authdatatype = krb5_int32;
    #[c2rust::src_loc = "185:1"]
    pub type krb5_preauthtype = krb5_int32;
    /* This may change, later on */
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
    /* *
 * Represents a timestamp in seconds since the POSIX epoch.  This legacy type
 * is used frequently in the ABI, but cannot represent timestamps after 2038 as
 * a positive number.  Code which uses this type should cast values of it to
 * uint32_t so that negative values are treated as timestamps between 2038 and
 * 2106 on platforms with 64-bit time_t.
 */
    #[c2rust::src_loc = "195:1"]
    pub type krb5_timestamp = krb5_int32;
    #[c2rust::src_loc = "197:1"]
    pub type krb5_deltat = krb5_int32;
    /* *
 * Used to convey an operation status.  The value 0 indicates success; any
 * other values are com_err codes.  Use krb5_get_error_message() to obtain a
 * string describing the error.
 */
    #[c2rust::src_loc = "204:1"]
    pub type krb5_error_code = krb5_int32;
    #[c2rust::src_loc = "206:1"]
    pub type krb5_magic = krb5_error_code;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "208:16"]
    pub struct _krb5_data {
        pub magic: krb5_magic,
        pub length: libc::c_uint,
        pub data: *mut libc::c_char,
    }
    #[c2rust::src_loc = "208:1"]
    pub type krb5_data = _krb5_data;
    /* Originally used to recognize AFS and default salts.  No longer used. */
    #[c2rust::src_loc = "225:1"]
    pub type krb5_pointer = *mut libc::c_void;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "228:16"]
    pub struct krb5_principal_data {
        pub magic: krb5_magic,
        pub realm: krb5_data,
        pub data: *mut krb5_data,
        pub length: krb5_int32,
        pub type_0: krb5_int32,
    }
    #[c2rust::src_loc = "236:1"]
    pub type krb5_principal = *mut krb5_principal_data;
    /*
 * Per V5 spec on definition of principal types
 */
    /* *<  Name type not known */
    /* *< Just the name of the principal
                                      as in DCE, or for users */
    /* *< Service and other unique instance (krbtgt) */
    /* *< Service with host name as instance
                                      (telnet, rcommands) */
    /* *< Service with host as remaining components */
    /* *< Unique ID */
    /* *< PKINIT */
    /* *< Name in form of SMTP email name */
    /* *< Windows 2000 UPN */
    /* *< Well-known (special) principal */
    /* *< First component of
                                                NT_WELLKNOWN principals */
    /* *< Windows 2000 UPN and SID */
    /* *< NT 4 style name */
    /* *< NT 4 style name and SID */
    /* * Constant version of krb5_principal_data */
    #[c2rust::src_loc = "261:1"]
    pub type krb5_const_principal = *const krb5_principal_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "323:16"]
    pub struct _krb5_address {
        pub magic: krb5_magic,
        pub addrtype: krb5_addrtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* *< Anonymous realm */
    /* *< Anonymous principal name */
    /*
 * end "base-defs.h"
 */
    /*
 * begin "hostaddr.h"
 */
    /* * Structure for address */
    #[c2rust::src_loc = "323:1"]
    pub type krb5_address = _krb5_address;
    /* *
 * Sign a PAC, possibly with a specified realm.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [in]  authtime        Expected timestamp
 * @param [in]  principal       Principal name (or NULL)
 * @param [in]  server_key      Key for server checksum
 * @param [in]  privsvr_key     Key for KDC checksum
 * @param [in]  with_realm      If true, include the realm of @a principal
 * @param [out] data            Signed PAC encoding
 *
 * This function is similar to krb5_pac_sign(), but adds a parameter
 * @a with_realm.  If @a with_realm is true, the PAC_CLIENT_INFO field of the
 * signed PAC will include the realm of @a principal as well as the name.  This
 * flag is necessary to generate PACs for cross-realm S4U2Self referrals.
 *
 * @version New in 1.17
 */
    /*
 * Read client information from a PAC.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [out] authtime_out    Authentication timestamp (NULL if not needed)
 * @param [out] princname_out   Client account name
 *
 * Read the PAC_CLIENT_INFO buffer in @a pac.  Place the client account name as
 * a string in @a princname_out.  If @a authtime_out is not NULL, place the
 * initial authentication timestamp in @a authtime_out.
 *
 * @retval 0 on success, ENOENT if no PAC_CLIENT_INFO buffer is present in @a
 * pac, ERANGE if the buffer contains invalid lengths.
 *
 * @version New in 1.18
 */
    /* *
 * Allow the appplication to override the profile's allow_weak_crypto setting.
 *
 * @param [in] context          Library context
 * @param [in] enable           Boolean flag
 *
 * This function allows an application to override the allow_weak_crypto
 * setting.  It is primarily for use by aklog.
 *
 * @retval 0  (always)
 */
    /* *
 * A wrapper for passing information to a @c krb5_trace_callback.
 *
 * Currently, it only contains the formatted message as determined
 * the the format string and arguments of the tracing macro, but it
 * may be extended to contain more fields in the future.
 */
    /* *
 * Specify a callback function for trace events.
 *
 * @param [in] context          Library context
 * @param [in] fn               Callback function
 * @param [in] cb_data          Callback data
 *
 * Specify a callback for trace events occurring in krb5 operations performed
 * within @a context.  @a fn will be invoked with @a context as the first
 * argument, @a cb_data as the last argument, and a pointer to a
 * krb5_trace_info as the second argument.  If the trace callback is reset via
 * this function or @a context is destroyed, @a fn will be invoked with a NULL
 * second argument so it can clean up @a cb_data.  Supply a NULL value for @a
 * fn to disable trace callbacks within @a context.
 *
 * @note This function overrides the information passed through the
 * @a KRB5_TRACE environment variable.
 *
 * @version New in 1.9
 *
 * @return Returns KRB5_TRACE_NOSUPP if tracing is not supported in the library
 * (unless @a fn is NULL).
 */
    /* *
 * Specify a file name for directing trace events.
 *
 * @param [in] context          Library context
 * @param [in] filename         File name
 *
 * Open @a filename for appending (creating it, if necessary) and set up a
 * callback to write trace events to it.
 *
 * @note This function overrides the information passed through the
 * @a KRB5_TRACE environment variable.
 *
 * @version New in 1.9
 *
 * @retval KRB5_TRACE_NOSUPP Tracing is not supported in the library.
 */
    /* *
 * Hook function for inspecting or modifying messages sent to KDCs.
 *
 * @param [in]  context         Library context
 * @param [in]  data            Callback data
 * @param [in]  realm           The realm the message will be sent to
 * @param [in]  message         The original message to be sent to the KDC
 * @param [out] new_message_out Optional replacement message to be sent
 * @param [out] reply_out       Optional synthetic reply
 *
 * If the hook function returns an error code, the KDC communication will be
 * aborted and the error code will be returned to the library operation which
 * initiated the communication.
 *
 * If the hook function sets @a reply_out, @a message will not be sent to the
 * KDC, and the given reply will used instead.
 *
 * If the hook function sets @a new_message_out, the given message will be sent
 * to the KDC in place of @a message.
 *
 * If the hook function returns successfully without setting either output,
 * @a message will be sent to the KDC normally.
 *
 * The hook function should use krb5_copy_data() to construct the value for
 * @a new_message_out or @a reply_out, to ensure that it can be freed correctly
 * by the library.
 *
 * @version New in 1.15
 *
 * @retval 0 Success
 * @return A Kerberos error code
 */
    /* *
 * Hook function for inspecting or overriding KDC replies.
 *
 * @param [in]  context         Library context
 * @param [in]  data            Callback data
 * @param [in]  code            Status of KDC communication
 * @param [in]  realm           The realm the reply was received from
 * @param [in]  message         The message sent to the realm's KDC
 * @param [in]  reply           The reply received from the KDC
 * @param [out] new_reply_out   Optional replacement reply
 *
 * If @a code is zero, @a reply contains the reply received from the KDC.  The
 * hook function may return an error code to simulate an error, may synthesize
 * a different reply by setting @a new_reply_out, or may simply return
 * successfully to do nothing.
 *
 * If @a code is non-zero, KDC communication failed and @a reply should be
 * ignored.  The hook function may return @a code or a different error code, or
 * may synthesize a reply by setting @a new_reply_out and return successfully.
 *
 * The hook function should use krb5_copy_data() to construct the value for
 * @a new_reply_out, to ensure that it can be freed correctly by the library.
 *
 * @version New in 1.15
 *
 * @retval 0 Success
 * @return A Kerberos error code
 */
    #[c2rust::src_loc = "8510:1"]
    pub type krb5_post_recv_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: krb5_error_code, _: *const krb5_data,
                                    _: *const krb5_data, _: *const krb5_data,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    /* per Kerberos v5 protocol spec */
    /* not yet in the spec... */
    /* macros to determine if a type is a local type */
    /*
 * end "hostaddr.h"
 */
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
    #[c2rust::src_loc = "8475:1"]
    pub type krb5_pre_send_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: *const krb5_data, _: *const krb5_data,
                                    _: *mut *mut krb5_data,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "8391:1"]
    pub type krb5_trace_callback
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *const krb5_trace_info,
                                    _: *mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "8387:1"]
    pub type krb5_trace_info = _krb5_trace_info;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8387:16"]
    pub struct _krb5_trace_info {
        pub message: *const libc::c_char,
    }
    /*
 * Prompter enhancements
 */
/* * Prompt for password */
    /* * Prompt for new password (during password change) */
    /* * Prompt for new password again */
    /* * Prompt for preauthentication data (such as an OTP value) */
    #[c2rust::src_loc = "7865:1"]
    pub type krb5_prompt_type = krb5_int32;
    /*
 * begin "encryption.h"
 */
    /* * Exposed contents of a key. */
    #[c2rust::src_loc = "363:1"]
    pub type krb5_keyblock = _krb5_keyblock;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "363:16"]
    pub struct _krb5_keyblock {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* * Structure for auth data */
    #[c2rust::src_loc = "1946:1"]
    pub type krb5_authdata = _krb5_authdata;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1946:16"]
    pub struct _krb5_authdata {
        pub magic: krb5_magic,
        pub ad_type: krb5_authdatatype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* *< ADTYPE */
    /* *< Length of data  */
    /* *< Data */
    /* * C representation of KDC-REQ protocol message, including KDC-REQ-BODY */
    #[c2rust::src_loc = "2054:1"]
    pub type krb5_kdc_req = _krb5_kdc_req;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2054:16"]
    pub struct _krb5_kdc_req {
        pub magic: krb5_magic,
        pub msg_type: krb5_msgtype,
        pub padata: *mut *mut krb5_pa_data,
        pub kdc_options: krb5_flags,
        pub client: krb5_principal,
        pub server: krb5_principal,
        pub from: krb5_timestamp,
        pub till: krb5_timestamp,
        pub rtime: krb5_timestamp,
        pub nonce: krb5_int32,
        pub nktypes: libc::c_int,
        pub ktype: *mut krb5_enctype,
        pub addresses: *mut *mut krb5_address,
        pub authorization_data: krb5_enc_data,
        pub unenc_authdata: *mut *mut krb5_authdata,
        pub second_ticket: *mut *mut krb5_ticket,
    }
    /* *< KRB5_AS_REQ or KRB5_TGS_REQ */
    /* *< Preauthentication data */
    /* real body */
    /* *< Requested options */
    /* *< Client principal and realm */
    /* *< Server principal and realm */
    /* *< Requested start time */
    /* *< Requested end time */
    /* *< Requested renewable end time */
    /* *< Nonce to match request and response */
    /* *< Number of enctypes */
    /* *< Requested enctypes */
    /* *< Requested addresses (optional) */
    /* *< Encrypted authz data (optional) */
    /* *< Unencrypted authz data */
    /* *< Second ticket array (optional) */
    /* *
 * Ticket structure.
 *
 * The C representation of the ticket message, with a pointer to the
 * C representation of the encrypted part.
 */
    #[c2rust::src_loc = "1979:1"]
    pub type krb5_ticket = _krb5_ticket;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1979:16"]
    pub struct _krb5_ticket {
        pub magic: krb5_magic,
        pub server: krb5_principal,
        pub enc_part: krb5_enc_data,
        pub enc_part2: *mut krb5_enc_tkt_part,
    }
    /* cleartext portion */
    /* *< server name/realm */
    /* *< encryption type, kvno, encrypted encoding */
    /* *< ptr to decrypted version, if available */
    /* * Encrypted part of ticket. */
    #[c2rust::src_loc = "1961:1"]
    pub type krb5_enc_tkt_part = _krb5_enc_tkt_part;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1961:16"]
    pub struct _krb5_enc_tkt_part {
        pub magic: krb5_magic,
        pub flags: krb5_flags,
        pub session: *mut krb5_keyblock,
        pub client: krb5_principal,
        pub transited: krb5_transited,
        pub times: krb5_ticket_times,
        pub caddrs: *mut *mut krb5_address,
        pub authorization_data: *mut *mut krb5_authdata,
    }
    /* to-be-encrypted portion */
    /* *< flags */
    /* *< session key: includes enctype */
    /* *< client name/realm */
    /* *< list of transited realms */
    /* *< auth, start, end, renew_till */
    /* *< array of ptrs to addresses */
    /* *< auth data */
    /* KRB5_OLD_CRYPTO */
    /*
 * end "encryption.h"
 */
    /*
 * begin "fieldbits.h"
 */
    /* kdc_options for kdc_request */
/* options is 32 bits; each host is responsible to put the 4 bytes
   representing these bits into net order before transmission */
/* #define      KDC_OPT_RESERVED        0x80000000 */
    /* #define      KDC_OPT_UNUSED          0x01000000 */
    /* #define      KDC_OPT_UNUSED          0x00400000 */
/* #define      KDC_OPT_RESERVED        0x00200000 */
/* #define      KDC_OPT_RESERVED        0x00100000 */
/* #define      KDC_OPT_RESERVED        0x00080000 */
/* #define      KDC_OPT_RESERVED        0x00040000 */
    /* #define      KDC_OPT_RESERVED        0x00004000 */
/* #define      KDC_OPT_RESERVED        0x00002000 */
/* #define      KDC_OPT_RESERVED        0x00001000 */
/* #define      KDC_OPT_RESERVED        0x00000800 */
/* #define      KDC_OPT_RESERVED        0x00000400 */
/* #define      KDC_OPT_RESERVED        0x00000200 */
/* #define      KDC_OPT_RESERVED        0x00000100 */
/* #define      KDC_OPT_RESERVED        0x00000080 */
/* #define      KDC_OPT_RESERVED        0x00000040 */
    /* #define      KDC_OPT_UNUSED          0x00000004 */
    /*
 * Mask of ticket flags in the TGT which should be converted into KDC
 * options when using the TGT to get derivitive tickets.
 *
 *  New mask = KDC_OPT_FORWARDABLE | KDC_OPT_PROXIABLE |
 *             KDC_OPT_ALLOW_POSTDATE | KDC_OPT_RENEWABLE
 */
    /* definitions for ap_options fields */
    /* * @defgroup AP_OPTS AP_OPTS
 *
 * ap_options are 32 bits; each host is responsible to put the 4 bytes
 * representing these bits into net order before transmission
 * @{
 */
    /* *< Use session key */
    /* *< Perform a mutual
                                                 authentication exchange */
    /* *< Generate a subsession key
                                                 from the current session key
                                                 obtained from the
                                                 credentials */
    /* #define      AP_OPTS_RESERVED        0x10000000 */
/* #define      AP_OPTS_RESERVED        0x08000000 */
/* #define      AP_OPTS_RESERVED        0x04000000 */
/* #define      AP_OPTS_RESERVED        0x02000000 */
/* #define      AP_OPTS_RESERVED        0x01000000 */
/* #define      AP_OPTS_RESERVED        0x00800000 */
/* #define      AP_OPTS_RESERVED        0x00400000 */
/* #define      AP_OPTS_RESERVED        0x00200000 */
/* #define      AP_OPTS_RESERVED        0x00100000 */
/* #define      AP_OPTS_RESERVED        0x00080000 */
/* #define      AP_OPTS_RESERVED        0x00040000 */
/* #define      AP_OPTS_RESERVED        0x00020000 */
/* #define      AP_OPTS_RESERVED        0x00010000 */
/* #define      AP_OPTS_RESERVED        0x00008000 */
/* #define      AP_OPTS_RESERVED        0x00004000 */
/* #define      AP_OPTS_RESERVED        0x00002000 */
/* #define      AP_OPTS_RESERVED        0x00001000 */
/* #define      AP_OPTS_RESERVED        0x00000800 */
/* #define      AP_OPTS_RESERVED        0x00000400 */
/* #define      AP_OPTS_RESERVED        0x00000200 */
/* #define      AP_OPTS_RESERVED        0x00000100 */
/* #define      AP_OPTS_RESERVED        0x00000080 */
/* #define      AP_OPTS_RESERVED        0x00000040 */
/* #define      AP_OPTS_RESERVED        0x00000020 */
/* #define      AP_OPTS_RESERVED        0x00000010 */
/* #define      AP_OPTS_RESERVED        0x00000008 */
/* #define      AP_OPTS_RESERVED        0x00000004 */
    /* * @} */
    /* end of AP_OPTS group */
    /* definitions for ad_type fields. */
    /* Ticket flags */
/* flags are 32 bits; each host is responsible to put the 4 bytes
   representing these bits into net order before transmission */
/* #define      TKT_FLG_RESERVED        0x80000000 */
    /* #define      TKT_FLG_RESERVED        0x00004000 */
/* #define      TKT_FLG_RESERVED        0x00002000 */
/* #define      TKT_FLG_RESERVED        0x00001000 */
/* #define      TKT_FLG_RESERVED        0x00000800 */
/* #define      TKT_FLG_RESERVED        0x00000400 */
/* #define      TKT_FLG_RESERVED        0x00000200 */
/* #define      TKT_FLG_RESERVED        0x00000100 */
/* #define      TKT_FLG_RESERVED        0x00000080 */
/* #define      TKT_FLG_RESERVED        0x00000040 */
/* #define      TKT_FLG_RESERVED        0x00000020 */
/* #define      TKT_FLG_RESERVED        0x00000010 */
/* #define      TKT_FLG_RESERVED        0x00000008 */
/* #define      TKT_FLG_RESERVED        0x00000004 */
/* #define      TKT_FLG_RESERVED        0x00000002 */
/* #define      TKT_FLG_RESERVED        0x00000001 */
    /* definitions for lr_type fields. */
    /* definitions for msec direction bit for KRB_SAFE, KRB_PRIV */
    /*
 * end "fieldbits.h"
 */
    /*
 * begin "proto.h"
 */
    /* * Protocol version number */
    /* Message types */
    /* *< Initial authentication request */
    /* *< Response to AS request */
    /* *< Ticket granting server request */
    /* *< Response to TGS request */
    /* *< Auth req to application server */
    /* *< Response to mutual AP request */
    /* *< Safe application message */
    /* *< Private application message */
    /* *< Cred forwarding message */
    /* *< Error response */
    /* LastReq types */
    /* PADATA types */
    /* *< RFC 4120 */
    /* *< RFC 4120 */
    /* Not used */
    /* *< timestamp encrypted in key. RFC 4120 */
    /* *< SecurId passcode. RFC 4120 */
    /* *< Sesame project. RFC 4120 */
    /* *< OSF DCE. RFC 4120 */
    /* *< Cybersafe. RFC 4120 */
    /* *< Cygnus. RFC 4120, 3961 */
    /* *< Etype info for preauth. RFC 4120 */
    /* *< SAM/OTP */
    /* *< SAM/OTP */
    /* *< PKINIT */
    /* *< PKINIT */
    /* *< PKINIT. RFC 4556 */
    /* *< PKINIT. RFC 4556 */
    /* *< RFC 4120 */
    /* *< RFC 4120 */
    /* *< Windows 2000 referrals. RFC 6820 */
    /* *< SAM/OTP. RFC 4120 */
    /* *< Embedded in typed data. RFC 4120 */
    /* *< draft referral system */
    /* *< draft challenge system, updated */
    /* *< draft challenge system, updated */
    /* MS-KILE */
    /* *< include Windows PAC */
    /* *< username protocol transition request */
    /* *< certificate protocol transition request */
    /* *< AS checksum */
    /* *< RFC 6113 */
    /* *< RFC 6113 */
    /* *< RFC 6113 */
    /* *< RFC 6113 */
    /* *< RFC 6560 section 4.1 */
    /* *< RFC 6560 section 4.2 */
    /* *< RFC 6560 section 4.3 */
    /* *< RFC 6112 */
    /* *< RFC 6806 */
    /* *< RFC 8070 */
    /* *< MS-KILE and MS-SFU */
    /* *< currently must be zero */
    /* * Transited encoding types */
    /* * alternate authentication types */
    /* authorization data types. See RFC 4120 section 5.2.6 */
    /* * @defgroup KRB5_AUTHDATA KRB5_AUTHDATA
 * @{
 */
    /* *< RFC 4537 */
    /* *< formerly 142 in krb5 1.8 */
    /* * @} */
    /* end of KRB5_AUTHDATA group */
    /* password change constants */
    /* *< Success */
    /* *< Malformed request */
    /* *< Server error */
    /* *< Authentication error */
    /* *< Password change rejected */
    /* These are Microsoft's extensions in RFC 3244, and it looks like
   they'll become standardized, possibly with other additions.  */
    /* *< Not authorized */
    /* *< Unknown RPC version */
    /* * The presented credentials were not obtained using a password directly */
    /*
 * end "proto.h"
 */
    /* Time set */
/* * Ticket start time, end time, and renewal duration. */
    #[c2rust::src_loc = "1936:1"]
    pub type krb5_ticket_times = _krb5_ticket_times;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1936:16"]
    pub struct _krb5_ticket_times {
        pub authtime: krb5_timestamp,
        pub starttime: krb5_timestamp,
        pub endtime: krb5_timestamp,
        pub renew_till: krb5_timestamp,
    }
    /* *< Time at which KDC issued the initial ticket that corresponds to this ticket */
    /* XXX ? should ktime in KDC_REP == authtime
       in ticket? otherwise client can't get this */
    /* *< optional in ticket, if not present, use @a authtime */
    /* *< Ticket expiration time */
    /* *< Latest time at which renewal of ticket can be valid */
    /* * Structure for transited encoding */
    #[c2rust::src_loc = "1954:1"]
    pub type krb5_transited = _krb5_transited;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1954:16"]
    pub struct _krb5_transited {
        pub magic: krb5_magic,
        pub tr_type: krb5_octet,
        pub tr_contents: krb5_data,
    }
    #[c2rust::src_loc = "398:1"]
    pub type krb5_enc_data = _krb5_enc_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "398:16"]
    pub struct _krb5_enc_data {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub kvno: krb5_kvno,
        pub ciphertext: krb5_data,
    }
    /* *< Transited encoding type */
    /* *< Contents */
    /* * Pre-authentication data */
    #[c2rust::src_loc = "2038:1"]
    pub type krb5_pa_data = _krb5_pa_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2038:16"]
    pub struct _krb5_pa_data {
        pub magic: krb5_magic,
        pub pa_type: krb5_preauthtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        /* *< Preauthentication data type */
        /* *< Length of data */
        /* *< Data */
        /* This file is generated, please don't edit it directly.  */
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* General definitions for Kerberos version 5. */
/*
 * Copyright 1989, 1990, 1995, 2001, 2003, 2007, 2011 by the Massachusetts
 * Institute of Technology.  All Rights Reserved.
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
 * Copyright (C) 1998 by the FundsXpress, INC.
 *
 * All rights reserved.
 *
 * Export of this software from the United States of America may require
 * a specific license from the United States Government.  It is the
 * responsibility of any person or organization contemplating export to
 * obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of FundsXpress. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  FundsXpress makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 *
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND WITHOUT ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED
 * WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
 */
        /* * @defgroup KRB5_H krb5 library API
 * @{
 */
        /* By default, do not expose deprecated interfaces. */
        /* !KRB5_CONFIG__ */
        /* for *_MAX */
        /* from profile.h */
        #[c2rust::src_loc = "125:8"]
        pub type _profile_t;
        /* *
 * Convert a string (such a password) to a key.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [in]  string          String to be converted
 * @param [in]  salt            Salt value
 * @param [out] key             Generated key
 *
 * This function converts @a string to a @a key of encryption type @a enctype,
 * using the specified @a salt.  The newly created @a key must be released by
 * calling krb5_free_keyblock_contents() when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "839:1"]
        pub fn krb5_c_string_to_key(context: krb5_context,
                                    enctype: krb5_enctype,
                                    string: *const krb5_data,
                                    salt: *const krb5_data,
                                    key: *mut krb5_keyblock)
         -> krb5_error_code;
        /* *
 * Convert a string principal name to a krb5_principal structure.
 *
 * @param [in]  context         Library context
 * @param [in]  name            String representation of a principal name
 * @param [out] principal_out   New principal
 *
 * Convert a string representation of a principal name to a krb5_principal
 * structure.
 *
 * A string representation of a Kerberos name consists of one or more principal
 * name components, separated by slashes, optionally followed by the \@
 * character and a realm name.  If the realm name is not specified, the local
 * realm is used.
 *
 * To use the slash and \@ symbols as part of a component (quoted) instead of
 * using them as a component separator or as a realm prefix), put a backslash
 * (\) character in front of the symbol.  Similarly, newline, tab, backspace,
 * and NULL characters can be included in a component by using @c n, @c t, @c b
 * or @c 0, respectively.
 *
 * @note The realm in a Kerberos @a name cannot contain slash, colon,
 * or NULL characters.
 *
 * Use krb5_free_principal() to free @a principal_out when it is no longer
 * needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3427:1"]
        pub fn krb5_parse_name(context: krb5_context,
                               name: *const libc::c_char,
                               principal_out: *mut krb5_principal)
         -> krb5_error_code;
        /* *
 * Convert a krb5_principal structure to a string representation.
 *
 * @param [in]  context         Library context
 * @param [in]  principal       Principal
 * @param [out] name            String representation of principal name
 *
 * The resulting string representation uses the format and quoting conventions
 * described for krb5_parse_name().
 *
 * Use krb5_free_unparsed_name() to free @a name when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3489:1"]
        pub fn krb5_unparse_name(context: krb5_context,
                                 principal: krb5_const_principal,
                                 name: *mut *mut libc::c_char)
         -> krb5_error_code;
        /* *
 * Copy a krb5_data object.
 *
 * @param [in]  context           Library context
 * @param [in]  indata            Data object to be copied
 * @param [out] outdata           Copy of @a indata
 *
 * This function creates a new krb5_data object with the contents of @a indata.
 * Use krb5_free_data() to free @a outdata when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3797:1"]
        pub fn krb5_copy_data(context: krb5_context, indata: *const krb5_data,
                              outdata: *mut *mut krb5_data)
         -> krb5_error_code;
        /* *
 * Copy a principal.
 *
 * @param [in]  context         Library context
 * @param [in]  inprinc         Principal to be copied
 * @param [out] outprinc        Copy of @a inprinc
 *
 * This function creates a new principal structure with the contents of @a
 * inprinc.  Use krb5_free_principal() to free @a outprinc when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3813:1"]
        pub fn krb5_copy_principal(context: krb5_context,
                                   inprinc: krb5_const_principal,
                                   outprinc: *mut krb5_principal)
         -> krb5_error_code;
        /* *
 * Convert a principal name into the default salt for that principal.
 *
 * @param [in]  context         Library context
 * @param [in]  pr              Principal name
 * @param [out] ret             Default salt for @a pr to be filled in
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4313:1"]
        pub fn krb5_principal2salt(context: krb5_context,
                                   pr: krb5_const_principal,
                                   ret: *mut krb5_data) -> krb5_error_code;
        /* krb5_free.c */
/* *
 * Free the storage assigned to a principal.
 *
 * @param [in] context          Library context
 * @param [in] val              Principal to be freed
 */
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
        /* *
 * Free the contents of a krb5_keyblock structure.
 *
 * @param [in] context          Library context
 * @param [in] key              Keyblock to be freed
 *
 * This function frees the contents of @a key, but not the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4721:1"]
        pub fn krb5_free_keyblock_contents(context: krb5_context,
                                           key: *mut krb5_keyblock);
        /* *
 * Retrieve the current time with context specific time offset adjustment.
 *
 * @param [in]  context         Library context
 * @param [out] timeret         Timestamp to fill in
 *
 * This function retrieves the system time of day with the context specific
 * time offset adjustment.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4837:1"]
        pub fn krb5_timeofday(context: krb5_context,
                              timeret: *mut krb5_timestamp)
         -> krb5_error_code;
        /* *
 * Retrieve the default realm.
 *
 * @param [in]  context         Library context
 * @param [out] lrealm          Default realm name
 *
 * Retrieves the default realm to be used if no user-specified realm is
 * available.
 *
 * Use krb5_free_default_realm() to free @a lrealm when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4887:1"]
        pub fn krb5_get_default_realm(context: krb5_context,
                                      lrealm: *mut *mut libc::c_char)
         -> krb5_error_code;
        /* *
 * Free a default realm string returned by krb5_get_default_realm().
 *
 * @param [in] context          Library context
 * @param [in] lrealm           Realm to be freed
 */
        #[no_mangle]
        #[c2rust::src_loc = "4912:1"]
        pub fn krb5_free_default_realm(context: krb5_context,
                                       lrealm: *mut libc::c_char);
        /*
 * end "func-proto.h"
 */
        /*
 * begin stuff from libos.h
 */
        /* *
 * @brief Read a password from keyboard input.
 *
 * @param [in]     context      Library context
 * @param [in]     prompt       First user prompt when reading password
 * @param [in]     prompt2      Second user prompt (NULL to prompt only once)
 * @param [out]    return_pwd   Returned password
 * @param [in,out] size_return  On input, maximum size of password; on output,
 *                              size of password read
 *
 * This function reads a password from keyboard input and stores it in @a
 * return_pwd.  @a size_return should be set by the caller to the amount of
 * storage space available in @a return_pwd; on successful return, it will be
 * set to the length of the password read.
 *
 * @a prompt is printed to the terminal, followed by ": ", and then a password
 * is read from the keyboard.
 *
 * If @a prompt2 is NULL, the password is read only once.  Otherwise, @a
 * prompt2 is printed to the terminal and a second password is read.  If the
 * two passwords entered are not identical, KRB5_LIBOS_BADPWDMATCH is returned.
 *
 * Echoing is turned off when the password is read.
 *
 * @retval
 *  0   Success
 * @return
 * Error in reading or verifying the password
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "6096:1"]
        pub fn krb5_read_password(context: krb5_context,
                                  prompt: *const libc::c_char,
                                  prompt2: *const libc::c_char,
                                  return_pwd: *mut libc::c_char,
                                  size_return: *mut libc::c_uint)
         -> krb5_error_code;
        /* Error reporting */
/* *
 * Set an extended error message for an error code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Error string for the error code
 * @param [in] ...              printf(3) style parameters
 */
        #[no_mangle]
        #[c2rust::src_loc = "7892:1"]
        pub fn krb5_set_error_message(ctx: krb5_context,
                                      code: krb5_error_code,
                                      fmt: *const libc::c_char, _: ...);
        /* *
 * Add a prefix to the message for an error code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] ...              printf(3) style parameters
 *
 * Format a message and prepend it to the current message for @a code.  The
 * prefix will be separated from the old message with a colon and space.
 */
        #[no_mangle]
        #[c2rust::src_loc = "7926:1"]
        pub fn krb5_prepend_error_message(ctx: krb5_context,
                                          code: krb5_error_code,
                                          fmt: *const libc::c_char, _: ...);
        /* *
 * Clear the extended error message in a context.
 *
 * @param [in] ctx              Library context
 *
 * This function unsets the extended error message in a context, to ensure that
 * it is not mistakenly applied to another occurrence of the same error code.
 */
        #[no_mangle]
        #[c2rust::src_loc = "8043:1"]
        pub fn krb5_clear_error_message(ctx: krb5_context);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:41"]
pub mod k5_int_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 1989,1990,1991,1992,1993,1994,1995,2000,2001,
 * 2003,2006,2007,2008,2009 by the Massachusetts Institute of Technology,
 * Cambridge, MA, USA.  All Rights Reserved.
 *
 * This software is being provided to you, the LICENSEE, by the
 * Massachusetts Institute of Technology (M.I.T.) under the following
 * license.  By obtaining, using and/or copying this software, you agree
 * that you have read, understood, and will comply with these terms and
 * conditions:
 *
 * Export of this software from the United States of America may
 * require a specific license from the United States Government.
 * It is the responsibility of any person or organization contemplating
 * export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify and distribute
 * this software and its documentation for any purpose and without fee or
 * royalty is hereby granted, provided that you agree to comply with the
 * following copyright notice and statements, including the disclaimer, and
 * that the same appear on ALL copies of the software and documentation,
 * including modifications that you make for internal use or for
 * distribution:
 *
 * THIS SOFTWARE IS PROVIDED "AS IS", AND M.I.T. MAKES NO REPRESENTATIONS
 * OR WARRANTIES, EXPRESS OR IMPLIED.  By way of example, but not
 * limitation, M.I.T. MAKES NO REPRESENTATIONS OR WARRANTIES OF
 * MERCHANTABILITY OR FITNESS FOR ANY PARTICULAR PURPOSE OR THAT THE USE OF
 * THE LICENSED SOFTWARE OR DOCUMENTATION WILL NOT INFRINGE ANY THIRD PARTY
 * PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
 *
 * The name of the Massachusetts Institute of Technology or M.I.T. may NOT
 * be used in advertising or publicity pertaining to distribution of the
 * software.  Title to copyright in this software and any associated
 * documentation shall at all times remain with M.I.T., and USER agrees to
 * preserve same.
 *
 * Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 */
/*
 * Copyright (C) 1998 by the FundsXpress, INC.
 *
 * All rights reserved.
 *
 * Export of this software from the United States of America may require
 * a specific license from the United States Government.  It is the
 * responsibility of any person or organization contemplating export to
 * obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of FundsXpress. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  FundsXpress makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 *
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND WITHOUT ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED
 * WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
 */
    /*
 * This prototype for k5-int.h (Krb5 internals include file)
 * includes the user-visible definitions from krb5.h and then
 * includes other definitions that are not user-visible but are
 * required for compiling Kerberos internal routines.
 *
 * John Gilmore, Cygnus Support, Sat Jan 21 22:45:52 PST 1995
 */
    /* KRB5_GENERAL__ */
    /*
 * Begin "k5-config.h"
 */
    /*
 * Machine-type definitions: PC Clone 386 running Microloss Windows
 */
    /* From autoconf.h */
    /* HAVE_SYS_TYPES_H */
    /* HAVE_SYS_TYPES_H */
    /* KRB5_SYSTYPES__ */
    /* one day */
    /* one week */
    /* Thu Jan  1 00:00:00 2038 UTC */
    /*
 * Windows requires a different api interface to each function. Here
 * just define it as NULL.
 */
    /* #define KRB5_OLD_CRYPTO is done in krb5.h */
    /* KRB5_CONFIG__ */
    /*
 * End "k5-config.h"
 */
    /*
 * After loading the configuration definitions, load the Kerberos definitions.
 */
    /* Get mutex support; currently used only for the replay cache.  */
    /* Get error info support.  */
    /* Get string buffer support. */
    /* Define tracing macros. */
    /* Profile variables.  Constants are named KRB5_CONF_STRING, where STRING
 * matches the variable name.  Keep these alphabetized. */
    /* Cache configuration variables */
    /* Error codes used in KRB_ERROR protocol messages.
   Return values of library routines are based on a different error table
   (which allows non-ambiguous error codes between subsystems) */
    /* KDC errors */
    /* No error */
    /* Client's entry in DB expired */
    /* Server's entry in DB expired */
    /* Requested pvno not supported */
    /* C's key encrypted in old master */
    /* S's key encrypted in old master */
    /* Client not found in Kerberos DB */
    /* Server not found in Kerberos DB */
    /* Multiple entries in Kerberos DB */
    /* The C or S has a null key */
    /* Tkt ineligible for postdating */
    /* Requested starttime > endtime */
    /* KDC policy rejects request */
    /* KDC can't do requested opt. */
    /* No support for encryption type */
    /* No support for checksum type */
    /* No support for padata type */
    /* No support for transited type */
    /* C's creds have been revoked */
    /* S's creds have been revoked */
    /* TGT has been revoked */
    /* C not yet valid */
    /* S not yet valid */
    /* Password has expired */
    /* Preauthentication failed */
    /* Additional preauthentication */
                                           /* required */
    /* Requested server and */
                                           /* ticket don't match*/
    /* Server principal valid for */
                                           /*   user2user only */
    /* KDC policy rejected transited */
                                           /*   path */
    /* A service is not
                                            * available that is
                                            * required to process the
                                            * request */
    /* Application errors */
    /* Decrypt integrity check failed */
    /* Ticket expired */
    /* Ticket not yet valid */
    /* Request is a replay */
    /* The ticket isn't for us */
    /* Ticket/authenticator don't match */
    /* Clock skew too great */
    /* Incorrect net address */
    /* Protocol version mismatch */
    /* Invalid message type */
    /* Message stream modified */
    /* Message out of order */
    /* Key version is not available */
    /* Service key not available */
    /* Mutual authentication failed */
    /* Incorrect message direction */
    /* Alternative authentication */
                                        /* method required */
    /* Incorrect sequence numnber */
                                        /* in message */
    /* Inappropriate type of */
                                        /* checksum in message */
    /* Policy rejects transited path */
    /* Response too big for UDP, */
                                        /*   retry with TCP */
    /* other errors */
    /* Generic error (description */
                                        /* in e-text) */
    /* Field is too long for impl. */
    /* PKINIT server-reported errors */
    /* client cert not trusted */
    /* client signature verify failed */
    /* invalid Diffie-Hellman parameters */
    /* client cert not verifiable to */
                                                   /* trusted root cert */
    /* client cert had invalid signature */
    /* client cert was revoked */
    /* client cert revoked, reason unknown */
    /* mismatch between client cert and */
                                                   /* principal name */
    /* bad extended key use */
    /* bad digest algorithm in client cert */
    /* missing paChecksum in PA-PK-AS-REQ */
    /* bad digest algorithm in SignedData */
    /* The IAKERB proxy could
                                                      not find a KDC */
    /* The KDC did not respond
                                                      to the IAKERB proxy */
    /* RFC 6113 */
    /* RFC 6113 */
    /* err table base max offset for protocol err codes */
    /*
 * A null-terminated array of this structure is returned by the KDC as
 * the data part of the ETYPE_INFO preauth type.  It informs the
 * client which encryption types are supported.
 * The  same data structure is used by both etype-info and etype-info2
 * but s2kparams must be null when encoding etype-info.
 */
    /*
 *  This is essentially -1 without sign extension which can screw up
 *  comparisons on 64 bit machines. If the length is this value, then
 *  the salt data is not present. This is to distinguish between not
 *  being set and being of 0 length.
 */
    /* RFC 4537 */
    /* sam_type values -- informational only */
    /*  Enigma Logic */
    /*  Digital Pathways */
    /*  S/key where  KDC has key 0 */
    /*  Traditional S/Key */
    /*  Security Dynamics */
    /*  CRYPTOCard */
    /* XXX need to figure out who has which numbers assigned */
    /*  ActivCard decimal mode */
    /*  ActivCard hex mode */
    /*  Digital Pathways hex mode */
    /* experimental */
    /* testing */
    /* special */
    /* Array of checksums */
    /* information */
    /* KRB5_SAM_* values */
    /* informational */
    /* KRB5_SAM_* values */
    /* copied */
    /* krb5_enc_sam_response_enc */
    /*
 * Keep the pkinit definitions in a separate file so that the plugin
 * only has to include k5-int-pkinit.h rather than k5-int.h
 */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* Plain text of an encrypted PA-FX-COOKIE value produced by the KDC. */
    /* In PAC options, indicates Resource-Based Constrained Delegation support. */
    /* struct stat, stat() */
    /* MAXPATHLEN */
    /* prototypes for file-related
                                           syscalls; flags for open &
                                           friends */
    /* libos.spec */
    /* Internal structure of an opaque key identifier */
    /*
     * Cache of data private to the cipher implementation, which we
     * don't want to have to recompute for every operation.  This may
     * include key schedules, iteration counts, etc.
     *
     * The cipher implementation is responsible for setting this up
     * whenever needed, and the enc_provider key_cleanup method must
     * then be provided to dispose of it.
     */
    /* Write the SHA-256 hash of in (containing n elements) to out. */
    /* Convenience function: zap and free ptr if it is non-NULL. */
    /* Convenience function: zap and free zero-terminated str if it is non-NULL. */
    /* Convenience function: zap and free krb5_data pointer if it is non-NULL. */
    /*
 * End "los-proto.h"
 */
    /*
 * Flags for the os_flags field
 *
 * KRB5_OS_TOFFSET_VALID means that the time offset fields are valid.
 * The intention is that this facility to correct the system clocks so
 * that they reflect the "real" time, for systems where for some
 * reason we can't set the system clock.  Instead we calculate the
 * offset between the system time and real time, and store the offset
 * in the os context so that we can correct the system clock as necessary.
 *
 * KRB5_OS_TOFFSET_TIME means that the time offset fields should be
 * returned as the time by the krb5 time routines.  This should only
 * be used for testing purposes (obviously!)
 */
    /* lock mode flags */
    /*
 * Begin "preauth.h"
 *
 * (Originally written by Glen Machin at Sandia Labs.)
 */
/*
 * Sandia National Laboratories also makes no representations about the
 * suitability of the modifications, or additions to this software for
 * any purpose.  It is provided "as is" without express or implied warranty.
 */
    /* check logon hour restrictions */
    /* sign with usage 27 instead of 26 */
    /* padata from req_body is used*/
    /* Bits 0-15 are critical in FAST options (RFC 6113 section 7.3). */
    /*
 * AD-CAMMAC's other-verifiers field is a sequence of Verifier, which is an
 * extensible choice with only one selection, Verifier-MAC.  For the time being
 * we will represent this field directly as an array of krb5_verifier_mac.
 * That will have to change if other selections are added.
 */
    /* Does not return a copy; original padata sequence responsible for freeing*/
    /* Allocate a pa-data object with uninitialized contents of size len.  If len
 * is 0, set the contents field to NULL. */
    /* Free a single pa-data object. */
    /* Without copying, add single element *pa to *list, reallocating as necessary.
 * If *list is NULL, allocate a new list.  Set *pa to NULL on success. */
    /* Without copying, add a pa-data element of type pa_type to *list with the
 * contents in data.  Set *data to empty_data() on success. */
    /* Add an empty pa-data element of type pa_type to *list. */
    /* KRB5_PREAUTH__ */
    /*
 * End "preauth.h"
 */
    /* #include "krb5/wordsize.h" -- comes in through base-defs.h. */
    /* ** Plugin framework ***/
    /*
 * This framework can be used to create pluggable interfaces.  Not all existing
 * pluggable interface use this framework, but new ones should.  A new
 * pluggable interface entails:
 *
 * - An interface ID definition in the list of #defines below.
 *
 * - A name in the interface_names array in lib/krb5/krb/plugins.c.
 *
 * - An installed public header file in include/krb5.  The public header should
 *   include <krb5/plugin.h> and should declare a vtable structure for each
 *   supported major version of the interface.
 *
 * - A consumer API implementation, located within the code unit which makes
 *   use of the pluggable interface.  The consumer API should consist of:
 *
 *   . An interface-specific handle type which contains a vtable structure for
 *     the module (or a union of several such structures, if there are multiple
 *     supported major versions) and, optionally, resource data bound to the
 *     handle.
 *
 *   . An interface-specific loader function which creates a handle or list of
 *     handles.  A list of handles would be created if the interface is a
 *     one-to-many interface where the consumer wants to consult all available
 *     modules; a single handle would be created for an interface where the
 *     consumer wants to consult a specific module.  The loader function should
 *     use k5_plugin_load or k5_plugin_load_all to produce one or a list of
 *     vtable initializer functions, and should use those functions to fill in
 *     the vtable structure for the module (if necessary, trying each supported
 *     major version starting from the most recent).  The loader function can
 *     also bind resource data into the handle based on caller arguments, if
 *     appropriate.
 *
 *   . For each plugin method, a wrapper function which accepts a krb5_context,
 *     a plugin handle, and the method arguments.  Wrapper functions should
 *     invoke the method function contained in the handle's vtable.
 *
 * - Possibly, built-in implementations of the interface, also located within
 *   the code unit which makes use of the interface.  Built-in implementations
 *   must be registered with k5_plugin_register before the first call to
 *   k5_plugin_load or k5_plugin_load_all.
 *
 * A pluggable interface should have one or more currently supported major
 * versions, starting at 1.  Each major version should have a current minor
 * version, also starting at 1.  If new methods are added to a vtable, the
 * minor version should be incremented and the vtable stucture should document
 * where each minor vtable version ends.  If method signatures for a vtable are
 * changed, the major version should be incremented.
 *
 * Plugin module implementations (either built-in or dynamically loaded) should
 * define a function named <interfacename>_<modulename>_initvt, matching the
 * signature of krb5_plugin_initvt_fn as declared in include/krb5/plugin.h.
 * The initvt function should check the given maj_ver argument against its own
 * supported major versions, cast the vtable pointer to the appropriate
 * interface-specific vtable type, and fill in the vtable methods, stopping as
 * appropriate for the given min_ver.  Memory for the vtable structure is
 * allocated by the caller, not by the module.
 *
 * Dynamic plugin modules are registered with the framework through the
 * [plugins] section of the profile, as described in the admin documentation
 * and krb5.conf man page.
 */
    /* Holds krb5_context information about each pluggable interface. */
    /* A list of plugin interface IDs.  Make sure to increment
 * PLUGIN_NUM_INTERFACES when a new interface is added, and add an entry to the
 * interface_names table in lib/krb5/krb/plugin.c. */
    /* Retrieve the plugin module of type interface_id and name modname,
 * storing the result into module. */
    /* Retrieve all plugin modules of type interface_id, storing the result
 * into modules.  Free the result with k5_plugin_free_handles. */
    /* Release a module list allocated by k5_plugin_load_all. */
    /* Register a plugin module of type interface_id and name modname. */
    /*
 * Register a plugin module which is part of the krb5 tree but is built as a
 * dynamic plugin.  Look for the module in modsubdir relative to the
 * context->base_plugin_dir.
 */
    /* Destroy the module state within context; used by krb5_free_context. */
    /* private, in kdb5.h */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1208:8"]
    pub struct _krb5_context {
        pub magic: krb5_magic,
        pub in_tkt_etypes: *mut krb5_enctype,
        pub tgs_etypes: *mut krb5_enctype,
        pub os_context: _krb5_os_context,
        pub default_realm: *mut libc::c_char,
        pub profile: profile_t,
        pub dal_handle: *mut kdb5_dal_handle,
        pub clockskew: krb5_deltat,
        pub kdc_default_options: krb5_flags,
        pub library_options: krb5_flags,
        pub profile_secure: krb5_boolean,
        pub fcc_default_format: libc::c_int,
        pub prompt_types: *mut krb5_prompt_type,
        pub udp_pref_limit: libc::c_int,
        pub use_conf_ktypes: krb5_boolean,
        pub libkrb5_plugins: plugin_dir_handle,
        pub preauth_context: krb5_preauth_context,
        pub ccselect_handles: *mut *mut ccselect_module_handle,
        pub localauth_handles: *mut *mut localauth_module_handle,
        pub hostrealm_handles: *mut *mut hostrealm_module_handle,
        pub tls: *mut k5_tls_vtable_st,
        pub err: errinfo,
        pub err_fmt: *mut libc::c_char,
        pub kdblog_context: *mut _kdb_log_context,
        pub allow_weak_crypto: krb5_boolean,
        pub ignore_acceptor_hostname: krb5_boolean,
        pub enforce_ok_as_delegate: krb5_boolean,
        pub dns_canonicalize_hostname: dns_canonhost,
        pub trace_callback: krb5_trace_callback,
        pub trace_callback_data: *mut libc::c_void,
        pub kdc_send_hook: krb5_pre_send_fn,
        pub kdc_send_hook_data: *mut libc::c_void,
        pub kdc_recv_hook: krb5_post_recv_fn,
        pub kdc_recv_hook_data: *mut libc::c_void,
        pub plugins: [plugin_interface; 13],
        pub plugin_base_dir: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1137:8"]
    pub struct plugin_interface {
        pub modules: *mut *mut plugin_mapping,
        pub configured: krb5_boolean,
    }
    #[c2rust::src_loc = "1194:1"]
    pub type dns_canonhost = libc::c_uint;
    #[c2rust::src_loc = "1197:5"]
    pub const CANONHOST_FALLBACK: dns_canonhost = 2;
    #[c2rust::src_loc = "1196:5"]
    pub const CANONHOST_TRUE: dns_canonhost = 1;
    #[c2rust::src_loc = "1195:5"]
    pub const CANONHOST_FALSE: dns_canonhost = 0;
    #[c2rust::src_loc = "1203:1"]
    pub type krb5_preauth_context = *mut krb5_preauth_context_st;
    #[c2rust::src_loc = "1201:1"]
    pub type kdb5_dal_handle = _kdb5_dal_handle;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "702:16"]
    pub struct _krb5_os_context {
        pub magic: krb5_magic,
        pub time_offset: krb5_int32,
        pub usec_offset: krb5_int32,
        pub os_flags: krb5_int32,
        pub default_ccname: *mut libc::c_char,
    }
    #[inline]
    #[c2rust::src_loc = "2361:1"]
    pub unsafe extern "C" fn ts_after(mut a: krb5_timestamp,
                                      mut b: krb5_timestamp) -> krb5_boolean {
        return (a as uint32_t > b as uint32_t) as libc::c_int as krb5_boolean;
    }
    #[inline]
    #[c2rust::src_loc = "2315:1"]
    pub unsafe extern "C" fn k5memdup(mut in_0: *const libc::c_void,
                                      mut len: size_t,
                                      mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void = k5alloc(len, code);
        if !ptr.is_null() && len > 0 as libc::c_int as libc::c_ulong {
            memcpy(ptr, in_0, len);
        }
        return ptr;
    }
    #[inline]
    #[c2rust::src_loc = "2308:1"]
    pub unsafe extern "C" fn k5alloc(mut size: size_t,
                                     mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        return k5calloc(1 as libc::c_int as size_t, size, code);
    }
    #[inline]
    #[c2rust::src_loc = "2296:1"]
    pub unsafe extern "C" fn k5calloc(mut nmemb: size_t, mut size: size_t,
                                      mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
        ptr =
            calloc(if nmemb != 0 {
                       nmemb
                   } else { 1 as libc::c_int as libc::c_ulong },
                   if size != 0 {
                       size
                   } else { 1 as libc::c_int as libc::c_ulong });
        *code =
            if ptr.is_null() { 12 as libc::c_int } else { 0 as libc::c_int };
        return ptr;
    }
    #[inline]
    #[c2rust::src_loc = "2251:1"]
    pub unsafe extern "C" fn make_data(mut data: *mut libc::c_void,
                                       mut len: libc::c_uint) -> krb5_data {
        let mut d: krb5_data =
            krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
        d.magic = -(1760647422 as libc::c_long) as krb5_magic;
        d.data = data as *mut libc::c_char;
        d.length = len;
        return d;
    }
    #[inline]
    #[c2rust::src_loc = "656:1"]
    pub unsafe extern "C" fn zapfree(mut ptr: *mut libc::c_void,
                                     mut len: size_t) {
        if !ptr.is_null() { explicit_bzero(ptr, len); free(ptr); };
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_timestamp, krb5_error_code, krb5_data,
                        krb5_context, krb5_principal_data,
                        krb5_const_principal};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::kdb_log_h::_kdb_log_context;
    use super::kdb5_h::_kdb5_dal_handle;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    use super::string_h::{memcpy, explicit_bzero};
    use super::stdlib_h::{calloc, free};
    extern "C" {
        #[c2rust::src_loc = "1134:8"]
        pub type plugin_mapping;
        #[c2rust::src_loc = "1207:8"]
        pub type k5_tls_vtable_st;
        #[c2rust::src_loc = "1206:8"]
        pub type hostrealm_module_handle;
        #[c2rust::src_loc = "1205:8"]
        pub type localauth_module_handle;
        #[c2rust::src_loc = "1204:8"]
        pub type ccselect_module_handle;
        #[c2rust::src_loc = "1203:16"]
        pub type krb5_preauth_context_st;
        #[no_mangle]
        #[c2rust::src_loc = "2129:1"]
        pub fn krb5_principal2salt_norealm(_: krb5_context,
                                           _: krb5_const_principal,
                                           _: *mut krb5_data)
         -> krb5_error_code;
    }
    /* _KRB5_INT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kdb_log.h:43"]
pub mod kdb_log_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "102:16"]
    pub struct _kdb_log_context {
        pub iproprole: iprop_role,
        pub ulog: *mut kdb_hlog_t,
        pub ulogentries: uint32_t,
        pub ulogfd: libc::c_int,
    }
    #[c2rust::src_loc = "81:1"]
    pub type kdb_hlog_t = kdb_hlog;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "81:16"]
    pub struct kdb_hlog {
        pub kdb_hmagic: uint32_t,
        pub db_version_num: uint16_t,
        pub kdb_num: uint32_t,
        pub kdb_first_time: kdbe_time_t,
        pub kdb_last_time: kdbe_time_t,
        pub kdb_first_sno: kdb_sno_t,
        pub kdb_last_sno: kdb_sno_t,
        pub kdb_state: uint16_t,
        pub kdb_block: uint16_t,
    }
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2004 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
    /* #pragma ident        "@(#)kdb_log.h  1.3     04/02/23 SMI" */
    /*
 * DB macros
 */
    /*
 * Current DB version #
 */
    /*
 * DB log states
 */
    /*
 * DB log constants
 */
    /*
 * Default ulog file attributes
 */
    /* in seconds */
    /*
 * Max size of update entry + update header
 * We make this large since resizing can be costly.
 */
    /* Default size of principal record */
    /* 256 MB log file */
    /*
 * Prototype declarations
 */
    /* Log header magic # */
    /* Kerberos database version no. */
    /* # of updates in log */
    /* Timestamp of first update */
    /* Timestamp of last update */
    /* First serial # in the update log */
    /* Last serial # in the update log */
    /* State of update log */
    /* Block size of each element */
    /* Update entry magic # */
    /* Serial # of entry */
    /* Timestamp of update */
    /* Is the entry committed or not */
    /* Size of update entry */
    /* Address of kdb_incr_update_t */
    #[c2rust::src_loc = "102:1"]
    pub type kdb_log_context = _kdb_log_context;
    use super::iprop_hdr_h::iprop_role;
    use super::stdint_uintn_h::{uint32_t, uint16_t};
    use super::iprop_h::{kdbe_time_t, kdb_sno_t, kdb_incr_update_t};
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_error_code};
    use super::kdb_h::krb5_db_entry;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "73:1"]
        pub fn ulog_free_entries(updates: *mut kdb_incr_update_t,
                                 no_of_updates: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn ulog_add_update(context: krb5_context,
                               upd: *mut kdb_incr_update_t)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "69:1"]
        pub fn ulog_conv_2logentry(context: krb5_context,
                                   entry: *mut krb5_db_entry,
                                   update: *mut kdb_incr_update_t)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "63:1"]
        pub fn ulog_init_header(context: krb5_context) -> krb5_error_code;
    }
    /* !_KDB_LOG_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop.h:43"]
pub mod iprop_h {
    #[c2rust::src_loc = "22:1"]
    pub type kdb_sno_t = uint32_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:8"]
    pub struct kdbe_time_t {
        pub seconds: uint32_t,
        pub useconds: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "143:8"]
    pub struct kdb_incr_update_t {
        pub kdb_princ_name: utf8str_t,
        pub kdb_entry_sno: kdb_sno_t,
        pub kdb_time: kdbe_time_t,
        pub kdb_update: kdbe_t,
        pub kdb_deleted: libc::c_int,
        pub kdb_commit: libc::c_int,
        pub kdb_kdcs_seen_by: C2RustUnnamed_4,
        pub kdb_futures: C2RustUnnamed_3,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "154:2"]
    pub struct C2RustUnnamed_3 {
        pub kdb_futures_len: u_int,
        pub kdb_futures_val: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "150:2"]
    pub struct C2RustUnnamed_4 {
        pub kdb_kdcs_seen_by_len: u_int,
        pub kdb_kdcs_seen_by_val: *mut utf8str_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "17:9"]
    pub struct utf8str_t {
        pub utf8str_t_len: u_int,
        pub utf8str_t_val: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "138:9"]
    pub struct kdbe_t {
        pub kdbe_t_len: u_int,
        pub kdbe_t_val: *mut kdbe_val_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "98:8"]
    pub struct kdbe_val_t {
        pub av_type: kdbe_attr_type_t,
        pub kdbe_val_t_u: C2RustUnnamed_5,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "100:2"]
    pub union C2RustUnnamed_5 {
        pub av_attrflags: uint32_t,
        pub av_max_life: uint32_t,
        pub av_max_renew_life: uint32_t,
        pub av_exp: uint32_t,
        pub av_pw_exp: uint32_t,
        pub av_last_success: uint32_t,
        pub av_last_failed: uint32_t,
        pub av_fail_auth_count: uint32_t,
        pub av_princ: kdbe_princ_t,
        pub av_keydata: C2RustUnnamed_13,
        pub av_tldata: C2RustUnnamed_11,
        pub av_len: int16_t,
        pub av_pw_last_change: uint32_t,
        pub av_mod_princ: kdbe_princ_t,
        pub av_mod_time: uint32_t,
        pub av_mod_where: utf8str_t,
        pub av_pw_policy: utf8str_t,
        pub av_pw_policy_switch: libc::c_int,
        pub av_pw_hist_kvno: uint32_t,
        pub av_pw_hist: C2RustUnnamed_7,
        pub av_extension: C2RustUnnamed_6,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "130:3"]
    pub struct C2RustUnnamed_6 {
        pub av_extension_len: u_int,
        pub av_extension_val: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "126:3"]
    pub struct C2RustUnnamed_7 {
        pub av_pw_hist_len: u_int,
        pub av_pw_hist_val: *mut kdbe_pw_hist_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "69:9"]
    pub struct kdbe_pw_hist_t {
        pub kdbe_pw_hist_t_len: u_int,
        pub kdbe_pw_hist_t_val: *mut kdbe_key_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "30:8"]
    pub struct kdbe_key_t {
        pub k_ver: int32_t,
        pub k_kvno: int32_t,
        pub k_enctype: C2RustUnnamed_9,
        pub k_contents: C2RustUnnamed_8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:2"]
    pub struct C2RustUnnamed_8 {
        pub k_contents_len: u_int,
        pub k_contents_val: *mut utf8str_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:2"]
    pub struct C2RustUnnamed_9 {
        pub k_enctype_len: u_int,
        pub k_enctype_val: *mut int32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:8"]
    pub struct kdbe_princ_t {
        pub k_realm: utf8str_t,
        pub k_components: C2RustUnnamed_10,
        pub k_nametype: int32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:2"]
    pub struct C2RustUnnamed_10 {
        pub k_components_len: u_int,
        pub k_components_val: *mut kdbe_data_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "44:8"]
    pub struct kdbe_data_t {
        pub k_magic: int32_t,
        pub k_data: utf8str_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "114:3"]
    pub struct C2RustUnnamed_11 {
        pub av_tldata_len: u_int,
        pub av_tldata_val: *mut kdbe_tl_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "60:8"]
    pub struct kdbe_tl_t {
        pub tl_type: int16_t,
        pub tl_data: C2RustUnnamed_12,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:2"]
    pub struct C2RustUnnamed_12 {
        pub tl_data_len: u_int,
        pub tl_data_val: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "110:3"]
    pub struct C2RustUnnamed_13 {
        pub av_keydata_len: u_int,
        pub av_keydata_val: *mut kdbe_key_t,
    }
    #[c2rust::src_loc = "74:1"]
    pub type kdbe_attr_type_t = libc::c_uint;
    #[c2rust::src_loc = "94:2"]
    pub const AT_PW_HIST: kdbe_attr_type_t = 19;
    #[c2rust::src_loc = "93:2"]
    pub const AT_PW_HIST_KVNO: kdbe_attr_type_t = 18;
    #[c2rust::src_loc = "92:2"]
    pub const AT_PW_POLICY_SWITCH: kdbe_attr_type_t = 17;
    #[c2rust::src_loc = "91:2"]
    pub const AT_PW_POLICY: kdbe_attr_type_t = 16;
    #[c2rust::src_loc = "90:2"]
    pub const AT_PW_LAST_CHANGE: kdbe_attr_type_t = 15;
    #[c2rust::src_loc = "89:2"]
    pub const AT_MOD_WHERE: kdbe_attr_type_t = 14;
    #[c2rust::src_loc = "88:2"]
    pub const AT_MOD_TIME: kdbe_attr_type_t = 13;
    #[c2rust::src_loc = "87:2"]
    pub const AT_MOD_PRINC: kdbe_attr_type_t = 12;
    #[c2rust::src_loc = "86:2"]
    pub const AT_LEN: kdbe_attr_type_t = 11;
    #[c2rust::src_loc = "85:2"]
    pub const AT_TL_DATA: kdbe_attr_type_t = 10;
    #[c2rust::src_loc = "84:2"]
    pub const AT_KEYDATA: kdbe_attr_type_t = 9;
    #[c2rust::src_loc = "83:2"]
    pub const AT_PRINC: kdbe_attr_type_t = 8;
    #[c2rust::src_loc = "82:2"]
    pub const AT_FAIL_AUTH_COUNT: kdbe_attr_type_t = 7;
    #[c2rust::src_loc = "81:2"]
    pub const AT_LAST_FAILED: kdbe_attr_type_t = 6;
    #[c2rust::src_loc = "80:2"]
    pub const AT_LAST_SUCCESS: kdbe_attr_type_t = 5;
    #[c2rust::src_loc = "79:2"]
    pub const AT_PW_EXP: kdbe_attr_type_t = 4;
    #[c2rust::src_loc = "78:2"]
    pub const AT_EXP: kdbe_attr_type_t = 3;
    #[c2rust::src_loc = "77:2"]
    pub const AT_MAX_RENEW_LIFE: kdbe_attr_type_t = 2;
    #[c2rust::src_loc = "76:2"]
    pub const AT_MAX_LIFE: kdbe_attr_type_t = 1;
    #[c2rust::src_loc = "75:2"]
    pub const AT_ATTRFLAGS: kdbe_attr_type_t = 0;
    use super::stdint_uintn_h::uint32_t;
    use super::sys_types_h::u_int;
    use super::stdint_intn_h::{int16_t, int32_t};
    /* !_IPROP_H_RPCGEN */
    /* K&R C */
    /* K&R C */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop_hdr.h:43"]
pub mod iprop_hdr_h {
    #[c2rust::src_loc = "32:1"]
    pub type iprop_role = libc::c_uint;
    #[c2rust::src_loc = "35:5"]
    pub const IPROP_REPLICA: iprop_role = 2;
    #[c2rust::src_loc = "34:5"]
    pub const IPROP_MASTER: iprop_role = 1;
    #[c2rust::src_loc = "33:5"]
    pub const IPROP_NULL: iprop_role = 0;
    /* !_IPROP_HDR_H */
    /*
 * Full resync dump versioning
 */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:41"]
pub mod k5_err_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-err.h */
/*
 * Copyright 2006, 2007 Massachusetts Institute of Technology.
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
 *
 * Error-message handling
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:8"]
    pub struct errinfo {
        pub code: libc::c_long,
        pub msg: *mut libc::c_char,
    }
    /* K5_ERR_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:41"]
pub mod k5_plugin_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2006 Massachusetts Institute of Technology.
 * All Rights Reserved.
 *
 * This software is being provided to you, the LICENSEE, by the
 * Massachusetts Institute of Technology (M.I.T.) under the following
 * license.  By obtaining, using and/or copying this software, you agree
 * that you have read, understood, and will comply with these terms and
 * conditions:
 *
 * Export of this software from the United States of America may
 * require a specific license from the United States Government.
 * It is the responsibility of any person or organization contemplating
 * export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify and distribute
 * this software and its documentation for any purpose and without fee or
 * royalty is hereby granted, provided that you agree to comply with the
 * following copyright notice and statements, including the disclaimer, and
 * that the same appear on ALL copies of the software and documentation,
 * including modifications that you make for internal use or for
 * distribution:
 *
 * THIS SOFTWARE IS PROVIDED "AS IS", AND M.I.T. MAKES NO REPRESENTATIONS
 * OR WARRANTIES, EXPRESS OR IMPLIED.  By way of example, but not
 * limitation, M.I.T. MAKES NO REPRESENTATIONS OR WARRANTIES OF
 * MERCHANTABILITY OR FITNESS FOR ANY PARTICULAR PURPOSE OR THAT THE USE OF
 * THE LICENSED SOFTWARE OR DOCUMENTATION WILL NOT INFRINGE ANY THIRD PARTY
 * PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
 *
 * The name of the Massachusetts Institute of Technology or M.I.T. may NOT
 * be used in advertising or publicity pertaining to distribution of the
 * software.  Title to copyright in this software and any associated
 * documentation shall at all times remain with M.I.T., and USER agrees to
 * preserve same.
 *
 * Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 */
    /* Just those definitions which are needed by util/support/plugins.c,
   which gets compiled before util/et is built, which happens before
   we can construct krb5.h, which is included by k5-int.h.

   So, no krb5 types.  */
    /*
 * Plugins normally export fixed symbol names, but when statically
 * linking plugins, we need a different symbol name for each plugin.
 * The first argument to PLUGIN_SYMBOL_NAME acts as the
 * differentiator, and is only used for static plugin linking.
 *
 * Although this macro (and thus this header file) are used in plugins
 * whose code lies inside the krb5 tree, plugins maintained separately
 * from the krb5 tree do not need it; they can just use the fixed
 * symbol name unconditionally.
 */
    /* opaque */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:8"]
    pub struct plugin_dir_handle {
        pub files: *mut *mut plugin_file_handle,
    }
    use super::k5_err_h::errinfo;
    extern "C" {
        #[c2rust::src_loc = "80:8"]
        pub type plugin_file_handle;
        #[no_mangle]
        #[c2rust::src_loc = "103:1"]
        pub fn krb5int_open_plugin_dirs(_: *const *const libc::c_char,
                                        _: *const *const libc::c_char,
                                        _: *mut plugin_dir_handle,
                                        _: *mut errinfo) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "106:1"]
        pub fn krb5int_close_plugin_dirs(_: *mut plugin_dir_handle);
        #[no_mangle]
        #[c2rust::src_loc = "109:1"]
        pub fn krb5int_get_plugin_dir_data(_: *mut plugin_dir_handle,
                                           _: *const libc::c_char,
                                           _: *mut *mut *mut libc::c_void,
                                           _: *mut errinfo) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "112:1"]
        pub fn krb5int_free_plugin_dir_data(_: *mut *mut libc::c_void);
    }
    /* K5_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/kdb/kdb5.h:42"]
pub mod kdb5_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:8"]
    pub struct _kdb5_dal_handle {
        pub db_context: *mut libc::c_void,
        pub lib_handle: db_library,
        pub master_keylist: *mut krb5_keylist_node,
        pub master_princ: krb5_principal,
    }
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
    #[c2rust::src_loc = "19:1"]
    pub type db_library = *mut _db_library;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "19:16"]
    pub struct _db_library {
        pub name: [libc::c_char; 128],
        pub reference_cnt: libc::c_int,
        pub dl_dir_handle: plugin_dir_handle,
        pub vftabl: kdb_vftabl,
        pub next: *mut _db_library,
        pub prev: *mut _db_library,
    }
    use super::kdb_h::{krb5_keylist_node, kdb_vftabl};
    use super::krb5_h::krb5_principal;
    use super::k5_plugin_h::plugin_dir_handle;
    /* end of _KRB5_KDB5_H_ */
    /* typedef kdb5_dal_handle is in k5-int.h now */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:42"]
pub mod kdb_h {
    #[c2rust::src_loc = "294:1"]
    pub type krb5_keylist_node = _krb5_keylist_node;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "294:16"]
    pub struct _krb5_keylist_node {
        pub keyblock: krb5_keyblock,
        pub kvno: krb5_kvno,
        pub next: *mut _krb5_keylist_node,
    }
    /*
 * A krb5_context can hold one database object.  Modules should use
 * krb5_db_set_context and krb5_db_get_context to store state associated with
 * the database object.
 *
 * Some module functions are mandatory for KDC operation; others are optional
 * or apply only to administrative operations.  If a function is optional, a
 * module can leave the function pointer as NULL.  Alternatively, modules can
 * return KRB5_PLUGIN_OP_NOTSUPP when asked to perform an inapplicable action.
 *
 * Some module functions have default implementations which will call back into
 * the vtable interface.  Leave these functions as NULL to use the default
 * implementations.
 *
 * The documentation in these comments describes the DAL as it is currently
 * implemented and used, not as it should be.  So if anything seems off, that
 * probably means the current state of things is off.
 *
 * Modules must allocate memory for principal entries, policy entries, and
 * other structures using an allocator compatible with malloc() as seen by
 * libkdb5 and libkrb5.  Modules may link against libkdb5 and call
 * krb5_db_alloc() to be certain that the same malloc implementation is used.
 */
    #[c2rust::src_loc = "923:1"]
    pub type kdb_vftabl = _kdb_vftabl;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "923:16"]
    pub struct _kdb_vftabl {
        pub maj_ver: libc::c_short,
        pub min_ver: libc::c_short,
        pub init_library: Option<unsafe extern "C" fn() -> krb5_error_code>,
        pub fini_library: Option<unsafe extern "C" fn() -> krb5_error_code>,
        pub init_module: Option<unsafe extern "C" fn(_: krb5_context,
                                                     _: *mut libc::c_char,
                                                     _:
                                                         *mut *mut libc::c_char,
                                                     _: libc::c_int)
                                    -> krb5_error_code>,
        pub fini_module: Option<unsafe extern "C" fn(_: krb5_context)
                                    -> krb5_error_code>,
        pub create: Option<unsafe extern "C" fn(_: krb5_context,
                                                _: *mut libc::c_char,
                                                _: *mut *mut libc::c_char)
                               -> krb5_error_code>,
        pub destroy: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: *mut libc::c_char,
                                                 _: *mut *mut libc::c_char)
                                -> krb5_error_code>,
        pub get_age: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: *mut libc::c_char,
                                                 _: *mut time_t)
                                -> krb5_error_code>,
        pub lock: Option<unsafe extern "C" fn(_: krb5_context, _: libc::c_int)
                             -> krb5_error_code>,
        pub unlock: Option<unsafe extern "C" fn(_: krb5_context)
                               -> krb5_error_code>,
        pub get_principal: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _:
                                                           krb5_const_principal,
                                                       _: libc::c_uint,
                                                       _:
                                                           *mut *mut krb5_db_entry)
                                      -> krb5_error_code>,
        pub put_principal: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: *mut krb5_db_entry,
                                                       _:
                                                           *mut *mut libc::c_char)
                                      -> krb5_error_code>,
        pub delete_principal: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              krb5_const_principal)
                                         -> krb5_error_code>,
        pub rename_principal: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              krb5_const_principal,
                                                          _:
                                                              krb5_const_principal)
                                         -> krb5_error_code>,
        pub iterate: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: *mut libc::c_char,
                                                 _:
                                                     Option<unsafe extern "C" fn(_:
                                                                                     krb5_pointer,
                                                                                 _:
                                                                                     *mut krb5_db_entry)
                                                                ->
                                                                    libc::c_int>,
                                                 _: krb5_pointer,
                                                 _: krb5_flags)
                                -> krb5_error_code>,
        pub create_policy: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: osa_policy_ent_t)
                                      -> krb5_error_code>,
        pub get_policy: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: *mut libc::c_char,
                                                    _: *mut osa_policy_ent_t)
                                   -> krb5_error_code>,
        pub put_policy: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: osa_policy_ent_t)
                                   -> krb5_error_code>,
        pub iter_policy: Option<unsafe extern "C" fn(_: krb5_context,
                                                     _: *mut libc::c_char,
                                                     _:
                                                         osa_adb_iter_policy_func,
                                                     _: *mut libc::c_void)
                                    -> krb5_error_code>,
        pub delete_policy: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: *mut libc::c_char)
                                      -> krb5_error_code>,
        pub fetch_master_key: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _: krb5_principal,
                                                          _:
                                                              *mut krb5_keyblock,
                                                          _: *mut krb5_kvno,
                                                          _:
                                                              *mut libc::c_char)
                                         -> krb5_error_code>,
        pub fetch_master_key_list: Option<unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   krb5_principal,
                                                               _:
                                                                   *const krb5_keyblock,
                                                               _:
                                                                   *mut *mut krb5_keylist_node)
                                              -> krb5_error_code>,
        pub store_master_key_list: Option<unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *mut libc::c_char,
                                                               _:
                                                                   krb5_principal,
                                                               _:
                                                                   *mut krb5_keylist_node,
                                                               _:
                                                                   *mut libc::c_char)
                                              -> krb5_error_code>,
        pub dbe_search_enctype: Option<unsafe extern "C" fn(_: krb5_context,
                                                            _:
                                                                *mut krb5_db_entry,
                                                            _:
                                                                *mut krb5_int32,
                                                            _: krb5_int32,
                                                            _: krb5_int32,
                                                            _: krb5_int32,
                                                            _:
                                                                *mut *mut krb5_key_data)
                                           -> krb5_error_code>,
        pub change_pwd: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: *mut krb5_keyblock,
                                                    _:
                                                        *mut krb5_key_salt_tuple,
                                                    _: libc::c_int,
                                                    _: *mut libc::c_char,
                                                    _: libc::c_int,
                                                    _: krb5_boolean,
                                                    _: *mut krb5_db_entry)
                                   -> krb5_error_code>,
        pub promote_db: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: *mut libc::c_char,
                                                    _: *mut *mut libc::c_char)
                                   -> krb5_error_code>,
        pub decrypt_key_data: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              *const krb5_keyblock,
                                                          _:
                                                              *const krb5_key_data,
                                                          _:
                                                              *mut krb5_keyblock,
                                                          _:
                                                              *mut krb5_keysalt)
                                         -> krb5_error_code>,
        pub encrypt_key_data: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              *const krb5_keyblock,
                                                          _:
                                                              *const krb5_keyblock,
                                                          _:
                                                              *const krb5_keysalt,
                                                          _: libc::c_int,
                                                          _:
                                                              *mut krb5_key_data)
                                         -> krb5_error_code>,
        pub sign_authdata: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: libc::c_uint,
                                                       _:
                                                           krb5_const_principal,
                                                       _:
                                                           krb5_const_principal,
                                                       _: *mut krb5_db_entry,
                                                       _: *mut krb5_db_entry,
                                                       _: *mut krb5_db_entry,
                                                       _: *mut krb5_db_entry,
                                                       _: *mut krb5_keyblock,
                                                       _: *mut krb5_keyblock,
                                                       _: *mut krb5_keyblock,
                                                       _: *mut krb5_keyblock,
                                                       _: *mut krb5_keyblock,
                                                       _: krb5_timestamp,
                                                       _:
                                                           *mut *mut krb5_authdata,
                                                       _: *mut libc::c_void,
                                                       _:
                                                           *mut *mut *mut krb5_data,
                                                       _:
                                                           *mut *mut *mut krb5_authdata)
                                      -> krb5_error_code>,
        pub check_transited_realms: Option<unsafe extern "C" fn(_:
                                                                    krb5_context,
                                                                _:
                                                                    *const krb5_data,
                                                                _:
                                                                    *const krb5_data,
                                                                _:
                                                                    *const krb5_data)
                                               -> krb5_error_code>,
        pub check_policy_as: Option<unsafe extern "C" fn(_: krb5_context,
                                                         _: *mut krb5_kdc_req,
                                                         _:
                                                             *mut krb5_db_entry,
                                                         _:
                                                             *mut krb5_db_entry,
                                                         _: krb5_timestamp,
                                                         _:
                                                             *mut *const libc::c_char,
                                                         _:
                                                             *mut *mut *mut krb5_pa_data)
                                        -> krb5_error_code>,
        pub check_policy_tgs: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              *mut krb5_kdc_req,
                                                          _:
                                                              *mut krb5_db_entry,
                                                          _: *mut krb5_ticket,
                                                          _:
                                                              *mut *const libc::c_char,
                                                          _:
                                                              *mut *mut *mut krb5_pa_data)
                                         -> krb5_error_code>,
        pub audit_as_req: Option<unsafe extern "C" fn(_: krb5_context,
                                                      _: *mut krb5_kdc_req,
                                                      _: *const krb5_address,
                                                      _: *const krb5_address,
                                                      _: *mut krb5_db_entry,
                                                      _: *mut krb5_db_entry,
                                                      _: krb5_timestamp,
                                                      _: krb5_error_code)
                                     -> ()>,
        pub refresh_config: Option<unsafe extern "C" fn(_: krb5_context)
                                       -> ()>,
        pub check_allowed_to_delegate: Option<unsafe extern "C" fn(_:
                                                                       krb5_context,
                                                                   _:
                                                                       krb5_const_principal,
                                                                   _:
                                                                       *const krb5_db_entry,
                                                                   _:
                                                                       krb5_const_principal)
                                                  -> krb5_error_code>,
        pub free_principal_e_data: Option<unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *mut krb5_octet)
                                              -> ()>,
        pub get_s4u_x509_principal: Option<unsafe extern "C" fn(_:
                                                                    krb5_context,
                                                                _:
                                                                    *const krb5_data,
                                                                _:
                                                                    krb5_const_principal,
                                                                _:
                                                                    libc::c_uint,
                                                                _:
                                                                    *mut *mut krb5_db_entry)
                                               -> krb5_error_code>,
        pub allowed_to_delegate_from: Option<unsafe extern "C" fn(_:
                                                                      krb5_context,
                                                                  _:
                                                                      krb5_const_principal,
                                                                  _:
                                                                      krb5_const_principal,
                                                                  _:
                                                                      *mut libc::c_void,
                                                                  _:
                                                                      *const krb5_db_entry)
                                                 -> krb5_error_code>,
        pub get_authdata_info: Option<unsafe extern "C" fn(_: krb5_context,
                                                           _: libc::c_uint,
                                                           _:
                                                               *mut *mut krb5_authdata,
                                                           _:
                                                               krb5_const_principal,
                                                           _:
                                                               krb5_const_principal,
                                                           _:
                                                               *mut krb5_keyblock,
                                                           _:
                                                               *mut krb5_keyblock,
                                                           _:
                                                               *mut krb5_db_entry,
                                                           _: krb5_timestamp,
                                                           _:
                                                               *mut *mut libc::c_void,
                                                           _:
                                                               *mut krb5_principal)
                                          -> krb5_error_code>,
        pub free_authdata_info: Option<unsafe extern "C" fn(_: krb5_context,
                                                            _:
                                                                *mut libc::c_void)
                                           -> ()>,
    }
    /*
     * Mandatory: Invoked after the module library is loaded, when the first DB
     * using the module is opened, across all contexts.
     */
    /*
     * Mandatory: Invoked before the module library is unloaded, after the last
     * DB using the module is closed, across all contexts.
     */
    /*
     * Mandatory: Initialize a database object.  Profile settings should be
     * read from conf_section inside KDB_MODULE_SECTION.  db_args communicates
     * command-line arguments for module-specific flags.  mode will be one of
     * KRB5_KDB_OPEN_{RW,RO} or'd with one of
     * KRB5_KDB_SRV_TYPE_{KDC,ADMIN,PASSWD,OTHER}.
     */
    /*
     * Mandatory: Finalize the database object contained in a context.  Free
     * any state contained in the db_context pointer and null it out.
     */
    /*
     * Optional: Initialize a database object while creating the underlying
     * database.  conf_section and db_args have the same meaning as in
     * init_module.  This function may return an error if the database already
     * exists.  Used by kdb5_util create.
     *
     * If db_args contains the value "temporary", the module should create an
     * exclusively locked side copy of the database suitable for loading in a
     * propagation from master to replica.  This side copy will later be
     * promoted with promote_db, allowing complete updates of the DB with no
     * loss in read availability.  If the module cannot comply with this
     * architecture, it should return an error.
     */
    /*
     * Optional: Destroy a database.  conf_section and db_args have the same
     * meaning as in init_module.  Used by kdb5_util destroy.  In current
     * usage, the database is destroyed while open, so the module should handle
     * that.
     */
    /*
     * Deprecated: No longer used as of krb5 1.10; can be removed in the next
     * DAL revision.  Modules should leave as NULL.
     */
    /*
     * Optional: Lock the database, with semantics depending on the mode
     * argument:
     *
     * KRB5_DB_LOCKMODE_SHARED: Lock may coexist with other shared locks.
     * KRB5_DB_LOCKMODE_EXCLUSIVE: Lock may not coexist with other locks.
     * KRB5_DB_LOCKMODE_PERMANENT: Exclusive lock surviving process exit.
     *
     * Used by the "kadmin lock" command, incremental propagation, and
     * kdb5_util dump.  Incremental propagation support requires shared locks
     * to operate.  kdb5_util dump will continue unlocked if the module returns
     * KRB5_PLUGIN_OP_NOTSUPP.
     */
    /* Optional: Release a lock created with db_lock. */
    /*
     * Mandatory: Set *entry to an allocated entry for the principal
     * search_for.  If the principal is not found, return KRB5_KDB_NOENTRY.
     *
     * The meaning of flags are as follows:
     *
     * KRB5_KDB_FLAG_CANONICALIZE: Set by the KDC when looking up entries for
     *     an AS or TGS request with canonicalization requested.  Determines
     *     whether the module should return out-of-realm referrals.
     *
     * KRB5_KDB_FLAG_INCLUDE_PAC: Set by the KDC during an AS request when the
     *     client requested PAC information during padata, and during most TGS
     *     requests.  Indicates that the module should include PAC information
     *     when its sign_authdata method is invoked.
     *
     * KRB5_KDB_FLAG_CLIENT_REFERRALS_ONLY: Set by the KDC when looking up the
     *     client entry in an AS request.  Affects how the module should return
     *     out-of-realm referrals.
     *
     * KRB5_KDB_FLAG_MAP_PRINCIPALS: Set by the KDC when looking up the client
     *     entry during TGS requests, except for S4U TGS requests and requests
     *     where the server entry has the KRB5_KDB_NO_AUTH_DATA_REQUIRED
     *     attribute.  Indicates that the module should map foreign principals
     *     to local principals if it supports doing so.
     *
     * KRB5_KDB_FLAG_PROTOCOL_TRANSITION: Set by the KDC when looking up the
     *     client entry during an S4U2Self TGS request.  This affects the PAC
     *     information which should be included when authorization data is
     *     generated; see the Microsoft S4U specification for details.
     *
     * KRB5_KDB_FLAG_CONSTRAINED_DELEGATION: Set by the KDC when looking up the
     *     client entry during an S4U2Proxy TGS request.  Also affects PAC
     *     generation.
     *
     * KRB5_KDB_FLAG_CROSS_REALM: Set by the KDC after looking up a server
     *     entry during a TGS request, if the header ticket was issued by a
     *     different realm.
     *
     * KRB5_KDB_FLAG_ISSUING_REFERRAL: Set by the KDC after looking up a server
     *     entry during a TGS request, if the requested server principal is not
     *     part of the realm being served, and a referral or alternate TGT will
     *     be issued instead.
     *
     * A module may return an in-realm alias by setting (*entry)->princ to the
     * canonical name.  The KDC will decide based on the request whether to use
     * the requested name or the canonical name in the issued ticket.
     *
     * A module can return a referral to another realm if
     * KRB5_KDB_FLAG_CANONICALIZE is set, or if
     * KRB5_KDB_FLAG_CLIENT_REFERRALS_ONLY is set and search_for->type is
     * KRB5_NT_ENTERPRISE_PRINCIPAL.  If KRB5_KDB_FLAG_CLIENT_REFERRALS_ONLY is
     * set, the module should return a referral by simply filling in an
     * out-of-realm name in (*entry)->princ and setting all other fields to
     * NULL.  Otherwise, the module should return the entry for the cross-realm
     * TGS of the referred-to realm.  For TGS referals, the module can also
     * include tl-data of type KRB5_TL_SERVER_REFERRAL containing ASN.1-encoded
     * Windows referral data as documented in
     * draft-ietf-krb-wg-kerberos-referrals-11 appendix A; this will be
     * returned to the client as encrypted padata.
     */
    /*
     * Optional: Create or modify a principal entry.  db_args communicates
     * command-line arguments for module-specific flags.
     *
     * The mask field of an entry indicates the changed fields.  Mask values
     * are defined in kadmin's admin.h header.  If KADM5_PRINCIPAL is set in
     * the mask, the entry is new; otherwise it already exists.  All fields of
     * an entry are expected to contain correct values, regardless of whether
     * they are specified in the mask, so it is acceptable for a module to
     * ignore the mask and update the entire entry.
     */
    /*
     * Optional: Delete the entry for the principal search_for.  If the
     * principal did not exist, return KRB5_KDB_NOENTRY.
     */
    /*
     * Optional with default: Rename a principal.  If the source principal does
     * not exist, return KRB5_KDB_NOENTRY.  If the target exists, return an
     * error.
     *
     * NOTE: If the module chooses to implement a custom function for renaming
     * a principal instead of using the default, then rename operations will
     * fail if iprop logging is enabled.
     */
    /*
     * Optional: For each principal entry in the database, invoke func with the
     * argments func_arg and the entry data.  If match_entry is specified, the
     * module may narrow the iteration to principal names matching that regular
     * expression; a module may alternatively ignore match_entry.
     */
    /*
     * Optional: Create a password policy entry.  Return an error if the policy
     * already exists.
     */
    /*
     * Optional: Set *policy to the policy entry of the specified name.  If the
     * entry does not exist, return KRB5_KDB_NOENTRY.
     */
    /*
     * Optional: Modify an existing password policy entry to match the values
     * in policy.  Return an error if the policy does not already exist.
     */
    /*
     * Optional: For each password policy entry in the database, invoke func
     * with the argments data and the entry data.  If match_entry is specified,
     * the module may narrow the iteration to policy names matching that
     * regular expression; a module may alternatively ignore match_entry.
     */
    /*
     * Optional: Delete the password policy entry with the name policy.  Return
     * an error if the entry does not exist.
     */
    /*
     * Optional with default: Retrieve a master keyblock from the stash file
     * db_args, filling in *key and *kvno.  mname is the name of the master
     * principal for the realm.
     *
     * The default implementation reads the master keyblock from a keytab or
     * old-format stash file.
     */
    /*
     * Optional with default: Given a keyblock for some version of the
     * database's master key, fetch the decrypted master key values from the
     * database and store the list into *mkeys_list.  The caller will free
     * *mkeys_list using a libkdb5 function which uses the standard free()
     * function, so the module must not use a custom allocator.
     *
     * The caller may not know the version number of the master key it has, in
     * which case it will pass IGNORE_VNO.
     *
     * The default implementation ignores kvno and tries the key against the
     * current master key data and all KRB5_TL_MKEY_AUX values, which contain
     * copies of the master keys encrypted with old master keys.
     */
    /*
     * Optional with default: Save a list of master keyblocks, obtained from
     * fetch_master_key_list, into the stash file db_arg.  The caller will set
     * master_pwd to NULL, so the module should just ignore it.  mname is the
     * name of the master principal for the realm.
     *
     * The default implementation saves the list of master keys in a
     * keytab-format file.
     */
    /*
     * Optional with default: Starting at position *start, scan the key data of
     * a database entry for a key matching the enctype ktype, the salt type
     * stype, and the version kvno.  Store the resulting key into *kdatap and
     * set *start to the position after the key found.  If ktype is negative,
     * match any enctype.  If stype is negative, match any salt type.  If kvno
     * is zero or negative, find the most recent key version satisfying the
     * other constraints.
     */
    /*
     * Optional with default: Change the key data for db_entry to include keys
     * derived from the password passwd in each of the specified key-salt
     * types, at version new_kvno.  Discard the old key data if keepold is not
     * set.
     *
     * The default implementation uses the keyblock master_key to encrypt each
     * new key, via the function encrypt_key_data.
     */
    /*
     * Optional: Promote a temporary database to be the live one.  context must
     * be initialized with an exclusively locked database created with the
     * "temporary" db_arg.  On success, the database object contained in
     * context will be finalized.
     *
     * This method is used by kdb5_util load to replace the live database with
     * minimal loss of read availability.
     */
    /*
     * Optional with default: Decrypt the key in key_data with master keyblock
     * mkey, placing the result into dbkey.  Copy the salt from key_data, if
     * any, into keysalt.  Either dbkey or keysalt may be left unmodified on
     * successful return if key_data does not contain key or salt information.
     *
     * The default implementation expects the encrypted key (in krb5_c_encrypt
     * format) to be stored in key_data_contents[0], with length given by
     * key_data_length[0].  If key_data_ver is 2, it expects the salt to be
     * stored, unencrypted, in key_data_contents[1], with length given by
     * key_data_length[1].
     */
    /*
     * Optional with default: Encrypt dbkey with master keyblock mkey, placing
     * the result into key_data along with keysalt.
     *
     * The default implementation stores the encrypted key (in krb5_c_encrypt
     * format) in key_data_contents[0] and the length in key_data_length[0].
     * If keysalt is specified, it sets key_data_ver to 2, and stores the salt
     * in key_data_contents[1] and its length in key_data_length[1].  If
     * keysalt is not specified, key_data_ver is set to 1.
     */
    /*
     * Optional: Generate signed authorization data, such as a Windows PAC, for
     * the ticket to be returned to the client.  Place the signed authorization
     * data, if any, in *signed_auth_data.  This function will be invoked for
     * an AS request if the client included padata requesting a PAC.  This
     * function will be invoked for a TGS request if there is authorization
     * data in the TGT, if the client is from another realm, or if the TGS
     * request is an S4U2Self or S4U2Proxy request.  This function will not be
     * invoked during TGS requests if the server principal has the
     * no_auth_data_required attribute set.  Input parameters are:
     *
     *   flags: The flags used to look up the client principal.
     *
     *   client_princ: For S4U2Self and S4U2Proxy TGS requests, the client
     *     principal requested by the service; for regular TGS requests, the
     *     possibly-canonicalized client principal.
     *
     *   server_princ: The server principal in the request.
     *
     *   client: The DB entry of the client if it is in the local realm, NULL
     *     if not.  For S4U2Self and S4U2Proxy TGS requests, this is the DB
     *     entry for the client principal requested by the service.
     *
     *   server: The DB entry of the service principal, or of a cross-realm
     *     krbtgt principal in case of referral.
     *
     *   header_server: For S4U2Proxy requests, the DB entry of the second
     *     ticket server.  For other TGS requests, the DB entry of the header
     *     ticket server.  For AS requests, NULL.
     *
     *   local_tgt: the DB entry of the local krbtgt principal.
     *
     *   client_key: The reply key for the KDC request, before any FAST armor
     *     is applied.  For AS requests, this may be the client's long-term key
     *     or a key chosen by a preauth mechanism.  For TGS requests, this may
     *     be the subkey found in the AP-REQ or the session key of the TGT.
     *
     *   server_key: The server key used to encrypt the returned ticket.
     *
     *   header_key: For S4U2Proxy requests, the key used to decrypt the second
     *     ticket.  For TGS requests, the key used to decrypt the header
     *     ticket.  For AS requests, NULL.
     *
     *   local_tgt_key: The decrypted first key of local_tgt.
     *
     *   session_key: The session key of the ticket being granted to the
     *     requestor.
     *
     *   authtime: The timestamp of the original client authentication time.
     *     For AS requests, this is the current time.  For TGS requests, this
     *     is the authtime of the subject ticket (TGT or S4U2Proxy evidence
     *     ticket).
     *
     *   tgt_auth_data: For TGS requests, the authorization data present in the
     *     subject ticket.  For AS requests, NULL.
     *
     *   ad_info: For TGS requests, the parsed authorization data if obtained
     *     by get_authdata_info method from the authorization data present in
     *     the subject ticket.  Otherwise NULL.
     *
     *   auth_indicators: Points to NULL or a null-terminated list of krb5_data
     *     pointers, each containing an authentication indicator (RFC 8129).
     *     The method may modify this list, or free it and replace
     *     *auth_indicators with NULL, to change which auth indicators will be
     *     included in the ticket.
     */
    /*
     * Optional: Perform a policy check on a cross-realm ticket's transited
     * field.  Return 0 if the check authoritatively succeeds,
     * KRB5_PLUGIN_NO_HANDLE to use the core transited-checking mechanisms, or
     * another error (other than KRB5_PLUGIN_OP_NOTSUPP) if the check fails.
     */
    /*
     * Optional: Perform a policy check on an AS request, in addition to the
     * standard policy checks.  Return 0 if the AS request is allowed.  If the
     * AS request is not allowed:
     *   - Place a short string literal into *status.
     *   - If desired, place data into e_data.  Any data placed here will be
     *     freed by the caller using the standard free function.
     *   - Return an appropriate error (such as KRB5KDC_ERR_POLICY).
     */
    /*
     * Optional: Perform a policy check on a TGS request, in addition to the
     * standard policy checks.  Return 0 if the TGS request is allowed.  If the
     * TGS request is not allowed:
     *   - Place a short string literal into *status.
     *   - If desired, place data into e_data.  Any data placed here will be
     *     freed by the caller using the standard free function.
     *   - Return an appropriate error (such as KRB5KDC_ERR_POLICY).
     * The input parameter ticket contains the TGT used in the TGS request.
     */
    /*
     * Optional: This method informs the module of a successful or unsuccessful
     * AS request.
     */
    /* Note: there is currently no method for auditing TGS requests. */
    /*
     * Optional: This method informs the module of a request to reload
     * configuration or other state (that is, the KDC received a SIGHUP).
     */
    /*
     * Optional: Perform a policy check on server being allowed to obtain
     * tickets from client to proxy.  (Note that proxy is the target of the
     * delegation, not the delegating service; the term "proxy" is from the
     * viewpoint of the delegating service asking another service to perform
     * some of its work in the authentication context of the client.  This
     * terminology comes from the Microsoft S4U protocol documentation.)
     * Return 0 if policy allows it, or an appropriate error (such as
     * KRB5KDC_ERR_POLICY) if not.  If this method is not implemented, all
     * S4U2Proxy delegation requests will be rejected.
     */
    /*
     * Optional: Free the e_data pointer of a database entry.  If this method
     * is not implemented, the e_data pointer in principal entries will be
     * freed with free() as seen by libkdb5.
     */
    /*
     * Optional: get a principal entry for S4U2Self based on X509 certificate.
     *
     * If flags include KRB5_KDB_FLAG_CLIENT_REFERRALS_ONLY, princ->realm
     * indicates the request realm, but the data components should be ignored.
     * The module can return an out-of-realm client referral as it would for
     * get_principal().
     *
     * If flags does not include KRB5_KDB_FLAG_CLIENT_REFERRALS_ONLY, princ is
     * from PA-S4U-X509-USER.  If it contains data components (and not just a
     * realm), the module should verify that it is the same as the lookup
     * result for client_cert.  The module should not return a referral.
     */
    /*
     * Optional: Perform a policy check on server being allowed to obtain
     * tickets from client to proxy.  This method is similar to
     * check_allowed_to_delegate, but it operates on the target server DB entry
     * (called "proxy" here as in Microsoft's protocol documentation) rather
     * than the intermediate server entry.  server_ad_info represents the
     * authdata of the intermediate server, as returned by the
     * get_authdata_info method on the header ticket.  Return 0 if policy
     * allows the delegation, or an appropriate error (such as
     * KRB5KDC_ERR_POLICY) if not.
     *
     * This method is called for S4U2Proxy requests and implements the
     * resource-based constrained delegation variant, which can support
     * cross-realm delegation.  If this method is not implemented or if it
     * returns a policy error, the KDC will fall back to
     * check_allowed_to_delegate if the intermediate and target servers are in
     * the same realm and the evidence ticket is forwardable.
     */
    /*
     * Optional: Perform verification and policy checks on authorization data,
     * such as a Windows PAC, based on the request client lookup flags.  Return
     * 0 if all checks have passed.  Optionally return a representation of the
     * authdata in *ad_info_out, to be consumed by allowed_to_delegate_from and
     * sign_authdata.  If client_out is not NULL, set *client_out to the client
     * name in the PAC; this indicates the requested client principal for a
     * cross-realm S4U2Proxy request.
     *
     * This method is called for TGS requests on the authorization data from
     * the header ticket.  For S4U2Proxy requests it is also called on the
     * authorization data from the evidence ticket.  If the
     * KRB5_KDB_FLAG_PROTOCOL_TRANSITION bit is set in flags, the authdata is
     * from the header ticket of an S4U2Self referral request, and the supplied
     * client_princ is the requested client.
     */
    /* End of minor version 0 for major version 8. */
    /*
 * A principal database entry.  Extensions to this structure currently use the
 * tl_data list.  The e_data and e_length fields are not used by any calling
 * code except kdb5_util dump and load, which marshal and unmarshal the array
 * in the dump record.  KDB modules may use these fields internally as long as
 * they set e_length appropriately (non-zero if the data should be marshalled
 * across dump and load, zero if not) and handle null e_data values in
 * caller-constructed principal entries.
 */
    #[c2rust::src_loc = "191:1"]
    pub type krb5_db_entry = _krb5_db_entry_new;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "191:16"]
    pub struct _krb5_db_entry_new {
        pub magic: krb5_magic,
        pub len: krb5_ui_2,
        pub mask: krb5_ui_4,
        pub attributes: krb5_flags,
        pub max_life: krb5_deltat,
        pub max_renewable_life: krb5_deltat,
        pub expiration: krb5_timestamp,
        pub pw_expiration: krb5_timestamp,
        pub last_success: krb5_timestamp,
        pub last_failed: krb5_timestamp,
        pub fail_auth_count: krb5_kvno,
        pub n_tl_data: krb5_int16,
        pub n_key_data: krb5_int16,
        pub e_length: krb5_ui_2,
        pub e_data: *mut krb5_octet,
        pub princ: krb5_principal,
        pub tl_data: *mut krb5_tl_data,
        pub key_data: *mut krb5_key_data,
    }
    /* NOT saved */
    /* members currently changed/set */
    /* When the client expires */
    /* When its passwd expires */
    /* Last successful passwd */
    /* Last failed passwd attempt */
    /* # of failed passwd attempt */
    /* Length of extra data */
    /* Extra data to be saved */
    /* Length, data */
    /* Linked list */
    /* key_data must be sorted by kvno in descending order. */
    /* Array */
    /*
 * If this ever changes up the version number and make the arrays be as
 * big as necessary.
 *
 * Currently the first type is the enctype and the second is the salt type.
 */
    #[c2rust::src_loc = "167:1"]
    pub type krb5_key_data = _krb5_key_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "167:16"]
    pub struct _krb5_key_data {
        pub key_data_ver: krb5_int16,
        pub key_data_kvno: krb5_ui_2,
        pub key_data_type: [krb5_int16; 2],
        pub key_data_length: [krb5_ui_2; 2],
        pub key_data_contents: [*mut krb5_octet; 2],
    }
    /* Version */
    /* Key Version */
    /* Array of types */
    /* Array of lengths */
    /* Array of pointers */
    /* #define KRB5_KDB_SALTTYPE_AFS3          5 */
    /* Attributes */
    /* S4U2Self OK */
    /* Creation flags */
    /* Entry get flags */
/* Name canonicalization requested */
    /* Include authorization data generated by backend */
    /* Is AS-REQ (client referrals only) */
    /* Map cross-realm principals */
    /* Protocol transition */
    /* Constrained delegation */
    /* User-to-user */
    /* Cross-realm */
    /* Issuing referral */
    /* KDB iteration flags */
    /* String attribute names recognized by krb5 */
    /*
 * Note --- these structures cannot be modified without changing the
 * database version number in libkdb.a, but should be expandable by
 * adding new tl_data types.
 */
    #[c2rust::src_loc = "147:1"]
    pub type krb5_tl_data = _krb5_tl_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "147:16"]
    pub struct _krb5_tl_data {
        pub tl_data_next: *mut _krb5_tl_data,
        pub tl_data_type: krb5_int16,
        pub tl_data_length: krb5_ui_2,
        pub tl_data_contents: *mut krb5_octet,
    }
    /* NOT saved */
    /* # of array elements */
    #[c2rust::src_loc = "177:1"]
    pub type krb5_keysalt = _krb5_keysalt;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "177:16"]
    pub struct _krb5_keysalt {
        pub type_0: krb5_int16,
        pub data: krb5_data,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "237:1"]
    pub type osa_adb_iter_policy_func
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: osa_policy_ent_t)
                   -> ()>;
    #[c2rust::src_loc = "215:1"]
    pub type osa_policy_ent_t = *mut _osa_policy_ent_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "215:16"]
    pub struct _osa_policy_ent_t {
        pub version: libc::c_int,
        pub name: *mut libc::c_char,
        pub pw_min_life: krb5_ui_4,
        pub pw_max_life: krb5_ui_4,
        pub pw_min_length: krb5_ui_4,
        pub pw_min_classes: krb5_ui_4,
        pub pw_history_num: krb5_ui_4,
        pub policy_refcnt: krb5_ui_4,
        pub pw_max_fail: krb5_ui_4,
        pub pw_failcnt_interval: krb5_ui_4,
        pub pw_lockout_duration: krb5_ui_4,
        pub attributes: krb5_ui_4,
        pub max_life: krb5_ui_4,
        pub max_renewable_life: krb5_ui_4,
        pub allowed_keysalts: *mut libc::c_char,
        pub n_tl_data: krb5_int16,
        pub tl_data: *mut krb5_tl_data,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "156:16"]
    pub struct krb5_string_attr_st {
        pub key: *mut libc::c_char,
        pub value: *mut libc::c_char,
    }
    /* Length, data */
    /* no longer used */
    /* Only valid if version > 1 */
    /* pwdMaxFailure */
    /* pwdFailureCountInterval */
    /* pwdLockoutDuration */
    /* Only valid if version > 2 */
    /* String attributes (currently stored inside tl-data) map C string keys to
 * values.  They can be set via kadmin and consumed by KDC plugins. */
    #[c2rust::src_loc = "156:1"]
    pub type krb5_string_attr = krb5_string_attr_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "282:16"]
    pub struct _krb5_actkvno_node {
        pub next: *mut _krb5_actkvno_node,
        pub act_kvno: krb5_kvno,
        pub act_time: krb5_timestamp,
    }
    #[c2rust::src_loc = "282:1"]
    pub type krb5_actkvno_node = _krb5_actkvno_node;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "288:16"]
    pub struct _krb5_mkey_aux_node {
        pub next: *mut _krb5_mkey_aux_node,
        pub mkey_kvno: krb5_kvno,
        pub latest_mkey: krb5_key_data,
    }
    #[c2rust::src_loc = "288:1"]
    pub type krb5_mkey_aux_node = _krb5_mkey_aux_node;
    use super::krb5_h::{krb5_keyblock, krb5_kvno, krb5_error_code,
                        krb5_context, krb5_const_principal, krb5_pointer,
                        krb5_flags, krb5_principal, krb5_int32, krb5_boolean,
                        krb5_timestamp, krb5_authdata, krb5_data,
                        krb5_kdc_req, krb5_pa_data, krb5_ticket, krb5_address,
                        krb5_octet, krb5_magic, krb5_ui_2, krb5_ui_4,
                        krb5_deltat, krb5_int16, krb5_enctype,
                        krb5_principal_data};
    use super::time_t_h::time_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
        /* kvno of mkey protecting the latest_mkey */
        /* most recent mkey */
        /* default functions. Should not be directly called */
/*
 *   Default functions prototype
 */
        #[no_mangle]
        #[c2rust::src_loc = "755:1"]
        pub fn krb5_dbe_def_search_enctype(kcontext: krb5_context,
                                           dbentp: *mut krb5_db_entry,
                                           start: *mut krb5_int32,
                                           ktype: krb5_int32,
                                           stype: krb5_int32,
                                           kvno: krb5_int32,
                                           kdatap: *mut *mut krb5_key_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "764:1"]
        pub fn krb5_def_store_mkey_list(context: krb5_context,
                                        keyfile: *mut libc::c_char,
                                        mname: krb5_principal,
                                        keylist: *mut krb5_keylist_node,
                                        master_pwd: *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "771:1"]
        pub fn krb5_db_def_fetch_mkey(context: krb5_context,
                                      mname: krb5_principal,
                                      key: *mut krb5_keyblock,
                                      kvno: *mut krb5_kvno,
                                      db_args: *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "778:1"]
        pub fn krb5_def_fetch_mkey_list(context: krb5_context,
                                        mprinc: krb5_principal,
                                        mkey: *const krb5_keyblock,
                                        mkeys_list:
                                            *mut *mut krb5_keylist_node)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "784:1"]
        pub fn krb5_dbe_def_cpw(context: krb5_context,
                                master_key: *mut krb5_keyblock,
                                ks_tuple: *mut krb5_key_salt_tuple,
                                ks_tuple_count: libc::c_int,
                                passwd: *mut libc::c_char,
                                new_kvno: libc::c_int, keepold: krb5_boolean,
                                db_entry: *mut krb5_db_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "794:1"]
        pub fn krb5_dbe_def_decrypt_key_data(context: krb5_context,
                                             mkey: *const krb5_keyblock,
                                             key_data: *const krb5_key_data,
                                             dbkey: *mut krb5_keyblock,
                                             keysalt: *mut krb5_keysalt)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "801:1"]
        pub fn krb5_dbe_def_encrypt_key_data(context: krb5_context,
                                             mkey: *const krb5_keyblock,
                                             dbkey: *const krb5_keyblock,
                                             keysalt: *const krb5_keysalt,
                                             keyver: libc::c_int,
                                             key_data: *mut krb5_key_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "809:1"]
        pub fn krb5_db_def_rename_principal(kcontext: krb5_context,
                                            source: krb5_const_principal,
                                            target: krb5_const_principal)
         -> krb5_error_code;
    }
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:41"]
pub mod profile_h {
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn profile_get_values(profile: profile_t,
                                  names: *const *const libc::c_char,
                                  ret_values: *mut *mut *mut libc::c_char)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn profile_free_list(list: *mut *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "80:1"]
        pub fn profile_get_string(profile: profile_t,
                                  name: *const libc::c_char,
                                  subname: *const libc::c_char,
                                  subsubname: *const libc::c_char,
                                  def_val: *const libc::c_char,
                                  ret_string: *mut *mut libc::c_char)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "110:1"]
        pub fn profile_release_string(str: *mut libc::c_char);
    }
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-buf.h:41"]
pub mod k5_buf_h {
    #[c2rust::src_loc = "48:1"]
    pub type k5buftype = libc::c_uint;
    #[c2rust::src_loc = "48:59"]
    pub const K5BUF_DYNAMIC_ZAP: k5buftype = 3;
    #[c2rust::src_loc = "48:44"]
    pub const K5BUF_DYNAMIC: k5buftype = 2;
    #[c2rust::src_loc = "48:31"]
    pub const K5BUF_FIXED: k5buftype = 1;
    #[c2rust::src_loc = "48:18"]
    pub const K5BUF_ERROR: k5buftype = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:8"]
    pub struct k5buf {
        pub buftype: k5buftype,
        pub data: *mut libc::c_void,
        pub space: size_t,
        pub len: size_t,
    }
    use super::stddef_h::size_t;
    extern "C" {
        /* Initialize a k5buf using an internally allocated dynamic buffer. */
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn k5_buf_init_dynamic(buf: *mut k5buf);
        /* Add a counted series of bytes to BUF. */
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn k5_buf_add_len(buf: *mut k5buf, data: *const libc::c_void,
                              len: size_t);
        /* Return ENOMEM if buf is in an error state, 0 otherwise. */
        #[no_mangle]
        #[c2rust::src_loc = "100:1"]
        pub fn k5_buf_status(buf: *mut k5buf) -> libc::c_int;
        /*
 * Free the storage used in the dynamic buffer BUF.  The caller may choose to
 * take responsibility for freeing the data pointer instead of using this
 * function.  If BUF is a fixed buffer, an assertion failure will result.
 * Freeing a buffer in the error state, a buffer initialized with EMPTY_K5BUF,
 * or a zeroed k5buf structure is a no-op.
 */
        #[no_mangle]
        #[c2rust::src_loc = "109:1"]
        pub fn k5_buf_free(buf: *mut k5buf);
    }
    /* K5_BUF_H */
}
#[c2rust::header_src = "/usr/include/stdio.h:41"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:41"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:41"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/string.h:41"]
pub mod string_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "435:1"]
        pub fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
        #[no_mangle]
        #[c2rust::src_loc = "396:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "90:14"]
        pub fn memchr(_: *const libc::c_void, _: libc::c_int,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:41"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/kdb/adb_err.h:48"]
pub mod adb_err_h {
    extern "C" {
        /* for compatibility with older versions... */
        #[no_mangle]
        #[c2rust::src_loc = "28:1"]
        pub fn initialize_adb_error_table();
    }
    /*@modifies internalState@*/
}
pub use self::types_h::{__u_int, __uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t, __off_t, __off64_t, __time_t};
pub use self::sys_types_h::u_int;
pub use self::time_t_h::time_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::{pthread_once_t, pthread_mutex_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::k5_thread_h::{k5_os_nothread_once_t, k5_once_t, k5_os_mutex,
                            k5_mutex_t, k5_mutex_finish_init, k5_mutex_lock,
                            k5_mutex_unlock, k5_once, k5_os_mutex_lock,
                            k5_os_mutex_unlock};
pub use self::k5_platform_h::{k5_init_t, C2RustUnnamed, C2RustUnnamed_0,
                              C2RustUnnamed_1, C2RustUnnamed_2, store_16_le,
                              store_32_le, load_16_le, load_32_le,
                              krb5int_strlcpy};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_msgtype, krb5_kvno,
                       krb5_addrtype, krb5_enctype, krb5_authdatatype,
                       krb5_preauthtype, krb5_flags, krb5_timestamp,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_pointer, krb5_principal_data,
                       krb5_principal, krb5_const_principal, _krb5_address,
                       krb5_address, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, krb5_keyblock,
                       _krb5_keyblock, krb5_authdata, _krb5_authdata,
                       krb5_kdc_req, _krb5_kdc_req, krb5_ticket, _krb5_ticket,
                       krb5_enc_tkt_part, _krb5_enc_tkt_part,
                       krb5_ticket_times, _krb5_ticket_times, krb5_transited,
                       _krb5_transited, krb5_enc_data, _krb5_enc_data,
                       krb5_pa_data, _krb5_pa_data, _profile_t,
                       krb5_c_string_to_key, krb5_parse_name,
                       krb5_unparse_name, krb5_copy_data, krb5_copy_principal,
                       krb5_principal2salt, krb5_free_principal,
                       krb5_free_keyblock_contents, krb5_timeofday,
                       krb5_get_default_realm, krb5_free_default_realm,
                       krb5_read_password, krb5_set_error_message,
                       krb5_prepend_error_message, krb5_clear_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, ts_after, k5memdup, k5alloc,
                         k5calloc, make_data, zapfree, plugin_mapping,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st,
                         krb5_principal2salt_norealm};
pub use self::kdb_log_h::{_kdb_log_context, kdb_hlog_t, kdb_hlog,
                          kdb_log_context, ulog_free_entries, ulog_add_update,
                          ulog_conv_2logentry, ulog_init_header};
pub use self::iprop_h::{kdb_sno_t, kdbe_time_t, kdb_incr_update_t,
                        C2RustUnnamed_3, C2RustUnnamed_4, utf8str_t, kdbe_t,
                        kdbe_val_t, C2RustUnnamed_5, C2RustUnnamed_6,
                        C2RustUnnamed_7, kdbe_pw_hist_t, kdbe_key_t,
                        C2RustUnnamed_8, C2RustUnnamed_9, kdbe_princ_t,
                        C2RustUnnamed_10, kdbe_data_t, C2RustUnnamed_11,
                        kdbe_tl_t, C2RustUnnamed_12, C2RustUnnamed_13,
                        kdbe_attr_type_t, AT_PW_HIST, AT_PW_HIST_KVNO,
                        AT_PW_POLICY_SWITCH, AT_PW_POLICY, AT_PW_LAST_CHANGE,
                        AT_MOD_WHERE, AT_MOD_TIME, AT_MOD_PRINC, AT_LEN,
                        AT_TL_DATA, AT_KEYDATA, AT_PRINC, AT_FAIL_AUTH_COUNT,
                        AT_LAST_FAILED, AT_LAST_SUCCESS, AT_PW_EXP, AT_EXP,
                        AT_MAX_RENEW_LIFE, AT_MAX_LIFE, AT_ATTRFLAGS};
pub use self::iprop_hdr_h::{iprop_role, IPROP_REPLICA, IPROP_MASTER,
                            IPROP_NULL};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle,
                            krb5int_open_plugin_dirs,
                            krb5int_close_plugin_dirs,
                            krb5int_get_plugin_dir_data,
                            krb5int_free_plugin_dir_data};
pub use self::kdb5_h::{_kdb5_dal_handle, db_library, _db_library};
pub use self::kdb_h::{krb5_keylist_node, _krb5_keylist_node, kdb_vftabl,
                      _kdb_vftabl, krb5_db_entry, _krb5_db_entry_new,
                      krb5_key_data, _krb5_key_data, krb5_tl_data,
                      _krb5_tl_data, krb5_keysalt, _krb5_keysalt,
                      krb5_key_salt_tuple, __krb5_key_salt_tuple,
                      osa_adb_iter_policy_func, osa_policy_ent_t,
                      _osa_policy_ent_t, krb5_string_attr_st,
                      krb5_string_attr, _krb5_actkvno_node, krb5_actkvno_node,
                      _krb5_mkey_aux_node, krb5_mkey_aux_node,
                      krb5_dbe_def_search_enctype, krb5_def_store_mkey_list,
                      krb5_db_def_fetch_mkey, krb5_def_fetch_mkey_list,
                      krb5_dbe_def_cpw, krb5_dbe_def_decrypt_key_data,
                      krb5_dbe_def_encrypt_key_data,
                      krb5_db_def_rename_principal};
pub use self::profile_h::{profile_t, profile_get_values, profile_free_list,
                          profile_get_string, profile_release_string};
pub use self::k5_buf_h::{k5buftype, K5BUF_DYNAMIC_ZAP, K5BUF_DYNAMIC,
                         K5BUF_FIXED, K5BUF_ERROR, k5buf, k5_buf_init_dynamic,
                         k5_buf_add_len, k5_buf_status, k5_buf_free};
use self::stdio_h::{asprintf, fprintf, stderr};
use self::libintl_h::dgettext;
use self::stdlib_h::{free, realloc, calloc, malloc};
use self::string_h::{explicit_bzero, strerror, strlen, strdup, strcmp, memchr,
                     memset, memcpy};
use self::assert_h::__assert_fail;
use self::adb_err_h::initialize_adb_error_table;
/*
 * Use a proxy function for iterate so that we can sort the keys before sending
 * them to the callback.
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "1054:8"]
pub struct callback_proxy_args {
    pub func: Option<unsafe extern "C" fn(_: krb5_pointer,
                                          _: *mut krb5_db_entry)
                         -> libc::c_int>,
    pub func_arg: krb5_pointer,
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2006, 2009, 2010, 2016 by the Massachusetts Institute of
 * Technology.
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
 * Copyright 2009 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
/*
 * This code was based on code donated to MIT by Novell for
 * distribution under the MIT license.
 */
/*
 * Include files
 */
/* Currently DB2 policy related errors are exported from DAL.  But
   other databases should set_err function to return string.  */
/*
 * internal static variable
 */
#[c2rust::src_loc = "54:19"]
static mut db_lock: k5_mutex_t =
    pthread_mutex_t{__data:
                        {
                            let mut init =
                                __pthread_mutex_s{__lock: 0 as libc::c_int,
                                                  __count:
                                                      0 as libc::c_int as
                                                          libc::c_uint,
                                                  __owner: 0 as libc::c_int,
                                                  __nusers:
                                                      0 as libc::c_int as
                                                          libc::c_uint,
                                                  __kind: 0 as libc::c_int,
                                                  __spins:
                                                      0 as libc::c_int as
                                                          libc::c_short,
                                                  __elision:
                                                      0 as libc::c_int as
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
                                                                                              *mut __pthread_internal_list,};
                                                          init
                                                      },};
                            init
                        },};
#[c2rust::src_loc = "56:19"]
static mut lib_list: db_library = 0 as *const _db_library as *mut _db_library;
/*
 * Helper Functions
 */
#[c2rust::src_loc = "62:1"]
unsafe extern "C" fn kdb_init_lock_list__aux() {
    kdb_init_lock_list__once.did_run = 1 as libc::c_int;
    kdb_init_lock_list__once.error = kdb_init_lock_list();
}
#[c2rust::src_loc = "62:1"]
static mut kdb_init_lock_list__once: k5_init_t =
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
                              Some(kdb_init_lock_list__aux as
                                       unsafe extern "C" fn() -> ()),};
            init
        }
    };
#[c2rust::src_loc = "65:1"]
unsafe extern "C" fn free_mkey_list(mut context: krb5_context,
                                    mut mkey_list: *mut krb5_keylist_node) {
    let mut cur: *mut krb5_keylist_node = 0 as *mut krb5_keylist_node;
    let mut next: *mut krb5_keylist_node = 0 as *mut krb5_keylist_node;
    cur = mkey_list;
    while !cur.is_null() {
        next = (*cur).next;
        krb5_free_keyblock_contents(context, &mut (*cur).keyblock);
        free(cur as *mut libc::c_void);
        cur = next
    };
}
#[c2rust::src_loc = "77:1"]
unsafe extern "C" fn kdb_init_lock_list() -> libc::c_int {
    return k5_mutex_finish_init(&mut db_lock);
}
#[c2rust::src_loc = "83:1"]
unsafe extern "C" fn kdb_lock_list() -> libc::c_int {
    let mut err: libc::c_int = 0;
    err =
        ({
             let mut k5int_i: *mut k5_init_t = &mut kdb_init_lock_list__once;
             let mut k5int_err: libc::c_int =
                 k5_once(&mut (*k5int_i).once, (*k5int_i).fn_0);
             if k5int_err != 0 {
                 k5int_err
             } else {
                 if (*k5int_i).did_run != 0 as libc::c_int {
                 } else {
                     __assert_fail(b"k5int_i->did_run != 0\x00" as *const u8
                                       as *const libc::c_char,
                                   b"kdb5.c\x00" as *const u8 as
                                       *const libc::c_char,
                                   87 as libc::c_int as libc::c_uint,
                                   (*::std::mem::transmute::<&[u8; 20],
                                                             &[libc::c_char; 20]>(b"int kdb_lock_list()\x00")).as_ptr());
                 }
                 (*k5int_i).error
             }
         });
    if err != 0 { return err }
    k5_mutex_lock(&mut db_lock);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "101:1"]
unsafe extern "C" fn kdb_unlock_list() { k5_mutex_unlock(&mut db_lock); }
/* Return true if the ulog is mapped in the master role. */
#[inline]
#[c2rust::src_loc = "108:1"]
unsafe extern "C" fn logging(mut context: krb5_context) -> krb5_boolean {
    let mut log_ctx: *mut kdb_log_context = (*context).kdblog_context;
    return (!log_ctx.is_null() &&
                (*log_ctx).iproprole as libc::c_uint ==
                    IPROP_MASTER as libc::c_int as libc::c_uint &&
                !(*log_ctx).ulog.is_null()) as libc::c_int as krb5_boolean;
}
#[no_mangle]
#[c2rust::src_loc = "117:1"]
pub unsafe extern "C" fn krb5_dbe_free_key_data_contents(mut context:
                                                             krb5_context,
                                                         mut key:
                                                             *mut krb5_key_data) {
    let mut i: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    if !key.is_null() {
        idx =
            if (*key).key_data_ver as libc::c_int == 1 as libc::c_int {
                1 as libc::c_int
            } else { 2 as libc::c_int };
        i = 0 as libc::c_int;
        while i < idx {
            if !(*key).key_data_contents[i as usize].is_null() {
                explicit_bzero((*key).key_data_contents[i as usize] as
                                   *mut libc::c_void,
                               (*key).key_data_length[i as usize] as size_t);
                free((*key).key_data_contents[i as usize] as
                         *mut libc::c_void);
            }
            i += 1
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "134:1"]
pub unsafe extern "C" fn krb5_dbe_free_key_list(mut context: krb5_context,
                                                mut val:
                                                    *mut krb5_keylist_node) {
    let mut temp: *mut krb5_keylist_node = val;
    let mut prev: *mut krb5_keylist_node = 0 as *mut krb5_keylist_node;
    while !temp.is_null() {
        prev = temp;
        temp = (*temp).next;
        krb5_free_keyblock_contents(context, &mut (*prev).keyblock);
        free(prev as *mut libc::c_void);
    };
}
#[no_mangle]
#[c2rust::src_loc = "147:1"]
pub unsafe extern "C" fn krb5_dbe_free_actkvno_list(mut context: krb5_context,
                                                    mut val:
                                                        *mut krb5_actkvno_node) {
    let mut temp: *mut krb5_actkvno_node = val;
    let mut prev: *mut krb5_actkvno_node = 0 as *mut krb5_actkvno_node;
    while !temp.is_null() {
        prev = temp;
        temp = (*temp).next;
        free(prev as *mut libc::c_void);
    };
}
#[no_mangle]
#[c2rust::src_loc = "159:1"]
pub unsafe extern "C" fn krb5_dbe_free_mkey_aux_list(mut context:
                                                         krb5_context,
                                                     mut val:
                                                         *mut krb5_mkey_aux_node) {
    let mut temp: *mut krb5_mkey_aux_node = val;
    let mut prev: *mut krb5_mkey_aux_node = 0 as *mut krb5_mkey_aux_node;
    while !temp.is_null() {
        prev = temp;
        temp = (*temp).next;
        krb5_dbe_free_key_data_contents(context, &mut (*prev).latest_mkey);
        free(prev as *mut libc::c_void);
    };
}
#[no_mangle]
#[c2rust::src_loc = "172:1"]
pub unsafe extern "C" fn krb5_dbe_free_tl_data(mut context: krb5_context,
                                               mut tl_data:
                                                   *mut krb5_tl_data) {
    if !tl_data.is_null() {
        if !(*tl_data).tl_data_contents.is_null() {
            free((*tl_data).tl_data_contents as *mut libc::c_void);
        }
        free(tl_data as *mut libc::c_void);
    };
}
#[no_mangle]
#[c2rust::src_loc = "182:1"]
pub unsafe extern "C" fn krb5_dbe_free_strings(mut context: krb5_context,
                                               mut strings:
                                                   *mut krb5_string_attr,
                                               mut count: libc::c_int) {
    let mut i: libc::c_int = 0;
    if strings.is_null() { return }
    i = 0 as libc::c_int;
    while i < count {
        free((*strings.offset(i as isize)).key as *mut libc::c_void);
        free((*strings.offset(i as isize)).value as *mut libc::c_void);
        i += 1
    }
    free(strings as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "197:1"]
pub unsafe extern "C" fn krb5_dbe_free_string(mut context: krb5_context,
                                              mut string: *mut libc::c_char) {
    free(string as *mut libc::c_void);
}
/* Set *section to the appropriate section to use for a database module's
 * profile queries.  The caller must free the result. */
#[c2rust::src_loc = "205:1"]
unsafe extern "C" fn get_conf_section(mut context: krb5_context,
                                      mut section: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut defrealm: *mut libc::c_char = 0 as *mut libc::c_char;
    *section = 0 as *mut libc::c_char;
    status = krb5_get_default_realm(context, &mut defrealm);
    if status != 0 {
        krb5_set_error_message(context,
                               -(1780008413 as libc::c_long) as
                                   krb5_error_code,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"No default realm set; cannot initialize KDB\x00"
                                            as *const u8 as
                                            *const libc::c_char));
        return -(1780008413 as libc::c_long) as krb5_error_code
    }
    status =
        profile_get_string((*context).profile,
                           b"realms\x00" as *const u8 as *const libc::c_char,
                           defrealm,
                           b"database_module\x00" as *const u8 as
                               *const libc::c_char, defrealm, &mut value) as
            krb5_error_code;
    krb5_free_default_realm(context, defrealm);
    if status != 0 { return status }
    result = strdup(value);
    profile_release_string(value);
    if result.is_null() { return 12 as libc::c_int }
    *section = result;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "239:1"]
unsafe extern "C" fn kdb_get_library_name(mut kcontext: krb5_context,
                                          mut libname_out:
                                              *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lib: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut defrealm: *mut libc::c_char = 0 as *mut libc::c_char;
    *libname_out = 0 as *mut libc::c_char;
    status = krb5_get_default_realm(kcontext, &mut defrealm);
    if !(status != 0) {
        status =
            profile_get_string((*kcontext).profile,
                               b"realms\x00" as *const u8 as
                                   *const libc::c_char, defrealm,
                               b"database_module\x00" as *const u8 as
                                   *const libc::c_char, defrealm, &mut value)
                as krb5_error_code;
        if !(status != 0) {
            /* we got the module section. Get the library name from the module */
            status =
                profile_get_string((*kcontext).profile,
                                   b"dbmodules\x00" as *const u8 as
                                       *const libc::c_char, value,
                                   b"db_library\x00" as *const u8 as
                                       *const libc::c_char,
                                   b"db2\x00" as *const u8 as
                                       *const libc::c_char, &mut lib) as
                    krb5_error_code;
            if !(status != 0) {
                *libname_out = strdup(lib);
                if (*libname_out).is_null() { status = 12 as libc::c_int }
            }
        }
    }
    krb5_free_default_realm(kcontext, defrealm);
    profile_release_string(value);
    profile_release_string(lib);
    return status;
}
#[c2rust::src_loc = "285:1"]
unsafe extern "C" fn copy_vtable(mut in_0: *const kdb_vftabl,
                                 mut out: *mut kdb_vftabl) {
    /* Copy fields for minor version 0. */
    (*out).maj_ver = (*in_0).maj_ver;
    (*out).min_ver = (*in_0).min_ver;
    (*out).init_library = (*in_0).init_library;
    (*out).fini_library = (*in_0).fini_library;
    (*out).init_module = (*in_0).init_module;
    (*out).fini_module = (*in_0).fini_module;
    (*out).create = (*in_0).create;
    (*out).destroy = (*in_0).destroy;
    (*out).get_age = (*in_0).get_age;
    (*out).lock = (*in_0).lock;
    (*out).unlock = (*in_0).unlock;
    (*out).get_principal = (*in_0).get_principal;
    (*out).put_principal = (*in_0).put_principal;
    (*out).delete_principal = (*in_0).delete_principal;
    (*out).rename_principal = (*in_0).rename_principal;
    (*out).iterate = (*in_0).iterate;
    (*out).create_policy = (*in_0).create_policy;
    (*out).get_policy = (*in_0).get_policy;
    (*out).put_policy = (*in_0).put_policy;
    (*out).iter_policy = (*in_0).iter_policy;
    (*out).delete_policy = (*in_0).delete_policy;
    (*out).fetch_master_key = (*in_0).fetch_master_key;
    (*out).fetch_master_key_list = (*in_0).fetch_master_key_list;
    (*out).store_master_key_list = (*in_0).store_master_key_list;
    (*out).dbe_search_enctype = (*in_0).dbe_search_enctype;
    (*out).change_pwd = (*in_0).change_pwd;
    (*out).promote_db = (*in_0).promote_db;
    (*out).decrypt_key_data = (*in_0).decrypt_key_data;
    (*out).encrypt_key_data = (*in_0).encrypt_key_data;
    (*out).sign_authdata = (*in_0).sign_authdata;
    (*out).check_transited_realms = (*in_0).check_transited_realms;
    (*out).check_policy_as = (*in_0).check_policy_as;
    (*out).check_policy_tgs = (*in_0).check_policy_tgs;
    (*out).audit_as_req = (*in_0).audit_as_req;
    (*out).refresh_config = (*in_0).refresh_config;
    (*out).check_allowed_to_delegate = (*in_0).check_allowed_to_delegate;
    (*out).free_principal_e_data = (*in_0).free_principal_e_data;
    (*out).get_s4u_x509_principal = (*in_0).get_s4u_x509_principal;
    (*out).allowed_to_delegate_from = (*in_0).allowed_to_delegate_from;
    (*out).get_authdata_info = (*in_0).get_authdata_info;
    (*out).free_authdata_info = (*in_0).free_authdata_info;
    /* Set defaults for optional fields. */
    if (*out).fetch_master_key.is_none() {
        (*out).fetch_master_key =
            Some(krb5_db_def_fetch_mkey as
                     unsafe extern "C" fn(_: krb5_context, _: krb5_principal,
                                          _: *mut krb5_keyblock,
                                          _: *mut krb5_kvno,
                                          _: *mut libc::c_char)
                         -> krb5_error_code)
    }
    if (*out).fetch_master_key_list.is_none() {
        (*out).fetch_master_key_list =
            Some(krb5_def_fetch_mkey_list as
                     unsafe extern "C" fn(_: krb5_context, _: krb5_principal,
                                          _: *const krb5_keyblock,
                                          _: *mut *mut krb5_keylist_node)
                         -> krb5_error_code)
    }
    if (*out).store_master_key_list.is_none() {
        (*out).store_master_key_list =
            Some(krb5_def_store_mkey_list as
                     unsafe extern "C" fn(_: krb5_context,
                                          _: *mut libc::c_char,
                                          _: krb5_principal,
                                          _: *mut krb5_keylist_node,
                                          _: *mut libc::c_char)
                         -> krb5_error_code)
    }
    if (*out).dbe_search_enctype.is_none() {
        (*out).dbe_search_enctype =
            Some(krb5_dbe_def_search_enctype as
                     unsafe extern "C" fn(_: krb5_context,
                                          _: *mut krb5_db_entry,
                                          _: *mut krb5_int32, _: krb5_int32,
                                          _: krb5_int32, _: krb5_int32,
                                          _: *mut *mut krb5_key_data)
                         -> krb5_error_code)
    }
    if (*out).change_pwd.is_none() {
        (*out).change_pwd =
            Some(krb5_dbe_def_cpw as
                     unsafe extern "C" fn(_: krb5_context,
                                          _: *mut krb5_keyblock,
                                          _: *mut krb5_key_salt_tuple,
                                          _: libc::c_int,
                                          _: *mut libc::c_char,
                                          _: libc::c_int, _: krb5_boolean,
                                          _: *mut krb5_db_entry)
                         -> krb5_error_code)
    }
    if (*out).decrypt_key_data.is_none() {
        (*out).decrypt_key_data =
            Some(krb5_dbe_def_decrypt_key_data as
                     unsafe extern "C" fn(_: krb5_context,
                                          _: *const krb5_keyblock,
                                          _: *const krb5_key_data,
                                          _: *mut krb5_keyblock,
                                          _: *mut krb5_keysalt)
                         -> krb5_error_code)
    }
    if (*out).encrypt_key_data.is_none() {
        (*out).encrypt_key_data =
            Some(krb5_dbe_def_encrypt_key_data as
                     unsafe extern "C" fn(_: krb5_context,
                                          _: *const krb5_keyblock,
                                          _: *const krb5_keyblock,
                                          _: *const krb5_keysalt,
                                          _: libc::c_int,
                                          _: *mut krb5_key_data)
                         -> krb5_error_code)
    }
    if (*out).rename_principal.is_none() {
        (*out).rename_principal =
            Some(krb5_db_def_rename_principal as
                     unsafe extern "C" fn(_: krb5_context,
                                          _: krb5_const_principal,
                                          _: krb5_const_principal)
                         -> krb5_error_code)
    };
}
/* KDB5_STATIC_LINK*/
#[c2rust::src_loc = "397:14"]
static mut db_dl_location: [*mut libc::c_char; 2] =
    [b"/usr/local/lib/krb5/plugins/kdb\x00" as *const u8 as
         *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char];
#[c2rust::src_loc = "400:1"]
unsafe extern "C" fn kdb_load_library(mut kcontext: krb5_context,
                                      mut lib_name: *mut libc::c_char,
                                      mut lib: *mut db_library)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut ndx: libc::c_int = 0;
    let mut vftabl_addrs: *mut *mut libc::c_void =
        0 as *mut *mut libc::c_void;
    /* N.B.: If this is "const" but not "static", the Solaris 10
       native compiler has trouble building the library because of
       absolute relocations needed in read-only section ".rodata".
       When it's static, it goes into ".picdata", which is
       read-write.  */
    static mut dbpath_names: [*const libc::c_char; 3] =
        [b"dbmodules\x00" as *const u8 as *const libc::c_char,
         b"db_module_dir\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char];
    let mut filebases: [*const libc::c_char; 2] =
        [0 as *const libc::c_char; 2];
    let mut profpath: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut path: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    filebases[0 as libc::c_int as usize] = lib_name;
    filebases[1 as libc::c_int as usize] = 0 as *const libc::c_char;
    *lib =
        calloc(1 as libc::c_int as size_t,
               ::std::mem::size_of::<_db_library>() as libc::c_ulong) as
            db_library;
    if (*lib).is_null() { return 12 as libc::c_int }
    krb5int_strlcpy((**lib).name.as_mut_ptr(), lib_name,
                    ::std::mem::size_of::<[libc::c_char; 128]>() as
                        libc::c_ulong);
    /* Fetch the list of directories specified in the config
       file(s) first.  */
    status =
        profile_get_values((*kcontext).profile, dbpath_names.as_ptr(),
                           &mut profpath) as krb5_error_code;
    if !(status != 0 as libc::c_int &&
             status as libc::c_long != -(1429577725 as libc::c_long)) {
        ndx = 0 as libc::c_int;
        if !profpath.is_null() {
            while !(*profpath.offset(ndx as isize)).is_null() { ndx += 1 }
        }
        path =
            calloc((ndx as
                        libc::c_ulong).wrapping_add((::std::mem::size_of::<[*mut libc::c_char; 2]>()
                                                         as
                                                         libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut libc::c_char>()
                                                                                         as
                                                                                         libc::c_ulong)),
                   ::std::mem::size_of::<*mut libc::c_char>() as
                       libc::c_ulong) as *mut *mut libc::c_char;
        if path.is_null() {
            status = 12 as libc::c_int
        } else {
            if ndx != 0 {
                memcpy(path as *mut libc::c_void,
                       profpath as *const libc::c_void,
                       (ndx as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                            as
                                                            libc::c_ulong));
            }
            memcpy(path.offset(ndx as isize) as *mut libc::c_void,
                   db_dl_location.as_mut_ptr() as *const libc::c_void,
                   (::std::mem::size_of::<[*mut libc::c_char; 2]>() as
                        libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut libc::c_char>()
                                                        as
                                                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                                                        as
                                                                                        libc::c_ulong));
            status = 0 as libc::c_int;
            status =
                krb5int_open_plugin_dirs(path as *mut *const libc::c_char,
                                         filebases.as_mut_ptr(),
                                         &mut (**lib).dl_dir_handle,
                                         &mut (*kcontext).err) as
                    krb5_error_code;
            if status != 0 {
                status = -(1780008416 as libc::c_long) as krb5_error_code;
                krb5_prepend_error_message(kcontext, status,
                                           dgettext(b"mit-krb5\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    b"Unable to find requested database type\x00"
                                                        as *const u8 as
                                                        *const libc::c_char));
            } else {
                status =
                    krb5int_get_plugin_dir_data(&mut (**lib).dl_dir_handle,
                                                b"kdb_function_table\x00" as
                                                    *const u8 as
                                                    *const libc::c_char,
                                                &mut vftabl_addrs,
                                                &mut (*kcontext).err) as
                        krb5_error_code;
                if status != 0 {
                    status = -(1780008414 as libc::c_long) as krb5_error_code;
                    krb5_prepend_error_message(kcontext, status,
                                               dgettext(b"mit-krb5\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        b"plugin symbol \'kdb_function_table\' lookup failed\x00"
                                                            as *const u8 as
                                                            *const libc::c_char));
                } else if (*vftabl_addrs.offset(0 as libc::c_int as
                                                    isize)).is_null() {
                    /* No plugins! */
                    status = -(1780008416 as libc::c_long) as krb5_error_code;
                    krb5_set_error_message(kcontext, status,
                                           dgettext(b"mit-krb5\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    b"Unable to load requested database module \'%s\': plugin symbol \'kdb_function_table\' not found\x00"
                                                        as *const u8 as
                                                        *const libc::c_char),
                                           lib_name);
                } else if (*(*vftabl_addrs.offset(0 as libc::c_int as isize)
                                 as *mut kdb_vftabl)).maj_ver as libc::c_int
                              != 8 as libc::c_int {
                    status = -(1780008405 as libc::c_long) as krb5_error_code
                } else {
                    copy_vtable(*vftabl_addrs.offset(0 as libc::c_int as
                                                         isize) as
                                    *const kdb_vftabl, &mut (**lib).vftabl);
                    status =
                        (**lib).vftabl.init_library.expect("non-null function pointer")();
                    (status) != 0;
                }
            }
        }
    }
    krb5int_free_plugin_dir_data(vftabl_addrs);
    /* Both of these DTRT with NULL.  */
    profile_free_list(profpath);
    free(path as *mut libc::c_void);
    if status != 0 && !(*lib).is_null() {
        if !(**lib).dl_dir_handle.files.is_null() {
            krb5int_close_plugin_dirs(&mut (**lib).dl_dir_handle);
        }
        free(*lib as *mut libc::c_void);
        *lib = 0 as db_library
    }
    return status;
}
/* end of _KDB5_STATIC_LINK */
#[c2rust::src_loc = "500:1"]
unsafe extern "C" fn kdb_find_library(mut kcontext: krb5_context,
                                      mut lib_name: *mut libc::c_char,
                                      mut lib: *mut db_library)
 -> krb5_error_code {
    let mut current_block: u64;
    /* lock here so that no two threads try to do the same at the same time */
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut locked: libc::c_int = 0 as libc::c_int;
    let mut curr_elt: db_library = 0 as *mut _db_library;
    let mut prev_elt: db_library = 0 as db_library;
    static mut kdb_db2_pol_err_loaded: libc::c_int = 0 as libc::c_int;
    if strcmp(b"db2\x00" as *const u8 as *const libc::c_char, lib_name) == 0
           && kdb_db2_pol_err_loaded == 0 as libc::c_int {
        initialize_adb_error_table();
        kdb_db2_pol_err_loaded = 1 as libc::c_int
    }
    status = kdb_lock_list();
    if !(status != 0 as libc::c_int) {
        locked = 1 as libc::c_int;
        curr_elt = lib_list;
        loop  {
            if curr_elt.is_null() {
                current_block = 10048703153582371463;
                break ;
            }
            if strcmp(lib_name, (*curr_elt).name.as_mut_ptr()) ==
                   0 as libc::c_int {
                *lib = curr_elt;
                current_block = 1295455896264598655;
                break ;
            } else { prev_elt = curr_elt; curr_elt = (*curr_elt).next }
        }
        match current_block {
            1295455896264598655 => { }
            _ => {
                /* module not found. create and add to list */
                status = kdb_load_library(kcontext, lib_name, lib);
                if !(status != 0) {
                    if !prev_elt.is_null() {
                        /* prev_elt points to the last element in the list */
                        (*prev_elt).next = *lib;
                        (**lib).prev = prev_elt
                    } else { lib_list = *lib }
                }
            }
        }
    }
    if !(*lib).is_null() { (**lib).reference_cnt += 1 }
    if locked != 0 { kdb_unlock_list(); }
    return status;
}
#[c2rust::src_loc = "551:1"]
unsafe extern "C" fn kdb_free_library(mut lib: db_library)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut locked: libc::c_int = 0 as libc::c_int;
    status = kdb_lock_list();
    if !(status != 0 as libc::c_int) {
        locked = 1 as libc::c_int;
        (*lib).reference_cnt -= 1;
        if (*lib).reference_cnt == 0 as libc::c_int {
            status =
                (*lib).vftabl.fini_library.expect("non-null function pointer")();
            if !(status != 0) {
                /* close the library */
                if !(*lib).dl_dir_handle.files.is_null() {
                    krb5int_close_plugin_dirs(&mut (*lib).dl_dir_handle); /* first element in the list */
                }
                if (*lib).prev.is_null() {
                    lib_list = (*lib).next
                } else { (*(*lib).prev).next = (*lib).next }
                if !(*lib).next.is_null() {
                    (*(*lib).next).prev = (*lib).prev
                }
                free(lib as *mut libc::c_void);
            }
        }
    }
    if locked != 0 { kdb_unlock_list(); }
    return status;
}
/*
 * These macros specify the encoding of data within the database.
 *
 * Data encoding is little-endian.
 */
/* _KRB5_INT_H */
/* 0x0300 was KRB5_KDB_SRV_TYPE_PASSWD but it is no longer used. */
/* libkdb.spec */
#[no_mangle]
#[c2rust::src_loc = "589:1"]
pub unsafe extern "C" fn krb5_db_setup_lib_handle(mut kcontext: krb5_context)
 -> krb5_error_code {
    let mut library: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut lib: db_library = 0 as db_library;
    let mut dal_handle: *mut kdb5_dal_handle = 0 as *mut kdb5_dal_handle;
    dal_handle =
        calloc(1 as libc::c_int as size_t,
               ::std::mem::size_of::<kdb5_dal_handle>() as libc::c_ulong) as
            *mut kdb5_dal_handle;
    if dal_handle.is_null() {
        status = 12 as libc::c_int
    } else {
        status = kdb_get_library_name(kcontext, &mut library);
        if library.is_null() {
            krb5_prepend_error_message(kcontext, status,
                                       dgettext(b"mit-krb5\x00" as *const u8
                                                    as *const libc::c_char,
                                                b"Cannot initialize database library\x00"
                                                    as *const u8 as
                                                    *const libc::c_char));
        } else {
            status = kdb_find_library(kcontext, library, &mut lib);
            if !(status != 0) {
                (*dal_handle).lib_handle = lib;
                (*kcontext).dal_handle = dal_handle
            }
        }
    }
    free(library as *mut libc::c_void);
    if status != 0 {
        free(dal_handle as *mut libc::c_void);
        if !lib.is_null() { kdb_free_library(lib); }
    }
    return status;
}
#[c2rust::src_loc = "629:1"]
unsafe extern "C" fn kdb_free_lib_handle(mut kcontext: krb5_context)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    status = kdb_free_library((*(*kcontext).dal_handle).lib_handle);
    if status != 0 { return status }
    free_mkey_list(kcontext, (*(*kcontext).dal_handle).master_keylist);
    krb5_free_principal(kcontext, (*(*kcontext).dal_handle).master_princ);
    free((*kcontext).dal_handle as *mut libc::c_void);
    (*kcontext).dal_handle = 0 as *mut kdb5_dal_handle;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "645:1"]
unsafe extern "C" fn get_vftabl(mut kcontext: krb5_context,
                                mut vftabl_ptr: *mut *mut kdb_vftabl)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0;
    *vftabl_ptr = 0 as *mut kdb_vftabl;
    if (*kcontext).dal_handle.is_null() {
        status = krb5_db_setup_lib_handle(kcontext);
        if status != 0 { return status }
    }
    *vftabl_ptr = &mut (*(*(*kcontext).dal_handle).lib_handle).vftabl;
    return 0 as libc::c_int;
}
/*
 *      External functions... DAL API
 */
#[no_mangle]
#[c2rust::src_loc = "663:1"]
pub unsafe extern "C" fn krb5_db_open(mut kcontext: krb5_context,
                                      mut db_args: *mut *mut libc::c_char,
                                      mut mode: libc::c_int)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0;
    let mut section: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    status = get_conf_section(kcontext, &mut section);
    if status != 0 { return status }
    status =
        (*v).init_module.expect("non-null function pointer")(kcontext,
                                                             section, db_args,
                                                             mode);
    free(section as *mut libc::c_void);
    return status;
}
#[no_mangle]
#[c2rust::src_loc = "681:1"]
pub unsafe extern "C" fn krb5_db_inited(mut kcontext: krb5_context)
 -> krb5_error_code {
    return !(!kcontext.is_null() && !(*kcontext).dal_handle.is_null() &&
                 !(*(*kcontext).dal_handle).db_context.is_null()) as
               libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "688:1"]
pub unsafe extern "C" fn krb5_db_create(mut kcontext: krb5_context,
                                        mut db_args: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0;
    let mut section: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    if (*v).create.is_none() {
        return -(1765328134 as libc::c_long) as krb5_error_code
    }
    status = get_conf_section(kcontext, &mut section);
    if status != 0 { return status }
    status =
        (*v).create.expect("non-null function pointer")(kcontext, section,
                                                        db_args);
    free(section as *mut libc::c_void);
    return status;
}
#[no_mangle]
#[c2rust::src_loc = "708:1"]
pub unsafe extern "C" fn krb5_db_fini(mut kcontext: krb5_context)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    /* Do nothing if module was never loaded. */
    if (*kcontext).dal_handle.is_null() { return 0 as libc::c_int }
    v = &mut (*(*(*kcontext).dal_handle).lib_handle).vftabl;
    status = (*v).fini_module.expect("non-null function pointer")(kcontext);
    if status != 0 { return status }
    return kdb_free_lib_handle(kcontext);
}
#[no_mangle]
#[c2rust::src_loc = "727:1"]
pub unsafe extern "C" fn krb5_db_destroy(mut kcontext: krb5_context,
                                         mut db_args: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0;
    let mut section: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    if (*v).destroy.is_none() {
        return -(1765328134 as libc::c_long) as krb5_error_code
    }
    status = get_conf_section(kcontext, &mut section);
    if status != 0 { return status }
    status =
        (*v).destroy.expect("non-null function pointer")(kcontext, section,
                                                         db_args);
    free(section as *mut libc::c_void);
    return status;
}
#[no_mangle]
#[c2rust::src_loc = "747:1"]
pub unsafe extern "C" fn krb5_db_get_age(mut kcontext: krb5_context,
                                         mut db_name: *mut libc::c_char,
                                         mut t: *mut time_t)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    if (*v).get_age.is_none() {
        return -(1765328134 as libc::c_long) as krb5_error_code
    }
    return (*v).get_age.expect("non-null function pointer")(kcontext, db_name,
                                                            t);
}
#[no_mangle]
#[c2rust::src_loc = "761:1"]
pub unsafe extern "C" fn krb5_db_lock(mut kcontext: krb5_context,
                                      mut lock_mode: libc::c_int)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    if (*v).lock.is_none() {
        return -(1765328134 as libc::c_long) as krb5_error_code
    }
    return (*v).lock.expect("non-null function pointer")(kcontext, lock_mode);
}
#[no_mangle]
#[c2rust::src_loc = "775:1"]
pub unsafe extern "C" fn krb5_db_unlock(mut kcontext: krb5_context)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    if (*v).unlock.is_none() {
        return -(1765328134 as libc::c_long) as krb5_error_code
    }
    return (*v).unlock.expect("non-null function pointer")(kcontext);
}
#[no_mangle]
#[c2rust::src_loc = "789:1"]
pub unsafe extern "C" fn krb5_db_get_principal(mut kcontext: krb5_context,
                                               mut search_for:
                                                   krb5_const_principal,
                                               mut flags: libc::c_uint,
                                               mut entry:
                                                   *mut *mut krb5_db_entry)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    *entry = 0 as *mut krb5_db_entry;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    if (*v).get_principal.is_none() {
        return -(1765328134 as libc::c_long) as krb5_error_code
    }
    status =
        (*v).get_principal.expect("non-null function pointer")(kcontext,
                                                               search_for,
                                                               flags, entry);
    if status != 0 { return status }
    /* Sort the keys in the db entry as some parts of krb5 expect it to be. */
    if !(**entry).key_data.is_null() {
        krb5_dbe_sort_key_data((**entry).key_data,
                               (**entry).n_key_data as size_t);
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "813:1"]
unsafe extern "C" fn free_tl_data(mut list: *mut krb5_tl_data) {
    let mut next: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    while !list.is_null() {
        next = (*list).tl_data_next;
        free((*list).tl_data_contents as *mut libc::c_void);
        free(list as *mut libc::c_void);
        list = next
    };
}
#[no_mangle]
#[c2rust::src_loc = "825:1"]
pub unsafe extern "C" fn krb5_db_free_principal(mut kcontext: krb5_context,
                                                mut entry:
                                                    *mut krb5_db_entry) {
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    let mut i: libc::c_int = 0;
    if entry.is_null() { return }
    if !(*entry).e_data.is_null() {
        if get_vftabl(kcontext, &mut v) == 0 as libc::c_int &&
               (*v).free_principal_e_data.is_some() {
            (*v).free_principal_e_data.expect("non-null function pointer")(kcontext,
                                                                           (*entry).e_data);
        } else { free((*entry).e_data as *mut libc::c_void); }
    }
    krb5_free_principal(kcontext, (*entry).princ);
    free_tl_data((*entry).tl_data);
    i = 0 as libc::c_int;
    while i < (*entry).n_key_data as libc::c_int {
        krb5_dbe_free_key_data_contents(kcontext,
                                        &mut *(*entry).key_data.offset(i as
                                                                           isize));
        i += 1
    }
    free((*entry).key_data as *mut libc::c_void);
    free(entry as *mut libc::c_void);
}
#[c2rust::src_loc = "847:1"]
unsafe extern "C" fn free_db_args(mut db_args: *mut *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    if !db_args.is_null() {
        i = 0 as libc::c_int;
        while !(*db_args.offset(i as isize)).is_null() {
            free(*db_args.offset(i as isize) as *mut libc::c_void);
            i += 1
        }
        free(db_args as *mut libc::c_void);
    };
}
#[c2rust::src_loc = "858:1"]
unsafe extern "C" fn extract_db_args_from_tl_data(mut kcontext: krb5_context,
                                                  mut start:
                                                      *mut *mut krb5_tl_data,
                                                  mut count: *mut krb5_int16,
                                                  mut db_argsp:
                                                      *mut *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut db_args: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut db_args_size: libc::c_int = 0 as libc::c_int;
    let mut prev: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    let mut curr: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    let mut next: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    let mut status: krb5_error_code = 0;
    /* Giving db_args as part of tl data causes db2 to store the
       tl_data as such.  To prevent this, tl_data is collated and
       passed as a separate argument.  Currently supports only one
       principal, but passing it as a separate argument makes it
       difficult for kadmin remote to pass arguments to server.  */
    prev = 0 as *mut krb5_tl_data;
    curr = *start;
    loop  {
        if curr.is_null() { current_block = 7056779235015430508; break ; }
        if (*curr).tl_data_type as libc::c_int == 0x7fff as libc::c_int {
            let mut t: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            /* Since this is expected to be NULL terminated string and
               this could come from any client, do a check before
               passing it to db.  */
            if *((*curr).tl_data_contents as
                     *mut libc::c_char).offset(((*curr).tl_data_length as
                                                    libc::c_int -
                                                    1 as libc::c_int) as
                                                   isize) as libc::c_int !=
                   '\u{0}' as i32 {
                /* Not null terminated. Dangerous input.  */
                status = 22 as libc::c_int; /* 1 for NULL */
                current_block = 3144764210734233507;
                break ;
            } else {
                db_args_size += 1;
                t =
                    realloc(db_args as *mut libc::c_void,
                            (::std::mem::size_of::<*mut libc::c_char>() as
                                 libc::c_ulong).wrapping_mul((db_args_size +
                                                                  1 as
                                                                      libc::c_int)
                                                                 as
                                                                 libc::c_ulong))
                        as *mut *mut libc::c_char;
                if t.is_null() {
                    status = 12 as libc::c_int;
                    current_block = 3144764210734233507;
                    break ;
                } else {
                    db_args = t;
                    let ref mut fresh0 =
                        *db_args.offset((db_args_size - 1 as libc::c_int) as
                                            isize);
                    *fresh0 = (*curr).tl_data_contents as *mut libc::c_char;
                    let ref mut fresh1 =
                        *db_args.offset(db_args_size as isize);
                    *fresh1 = 0 as *mut libc::c_char;
                    next = (*curr).tl_data_next;
                    if prev.is_null() {
                        /* current node is the first in the linked list. remove it */
                        *start = (*curr).tl_data_next
                    } else { (*prev).tl_data_next = (*curr).tl_data_next }
                    *count -= 1;
                    free(curr as *mut libc::c_void);
                    /* previous does not change */
                    curr = next
                }
            }
        } else { prev = curr; curr = (*curr).tl_data_next }
    }
    match current_block {
        7056779235015430508 => { status = 0 as libc::c_int }
        _ => { }
    }
    if status != 0 as libc::c_int {
        free_db_args(db_args);
        db_args = 0 as *mut *mut libc::c_char
    }
    *db_argsp = db_args;
    return status;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/kdb/kdb5int.h - Private header for kdb5 library */
/*
 * Copyright (C) 2008 by the Massachusetts Institute of Technology.
 * All rights reserved.
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
#[no_mangle]
#[c2rust::src_loc = "924:1"]
pub unsafe extern "C" fn krb5int_put_principal_no_log(mut kcontext:
                                                          krb5_context,
                                                      mut entry:
                                                          *mut krb5_db_entry)
 -> krb5_error_code {
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    let mut status: krb5_error_code = 0;
    let mut db_args: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    if (*v).put_principal.is_none() {
        return -(1765328134 as libc::c_long) as krb5_error_code
    }
    status =
        extract_db_args_from_tl_data(kcontext, &mut (*entry).tl_data,
                                     &mut (*entry).n_tl_data, &mut db_args);
    if status != 0 { return status }
    status =
        (*v).put_principal.expect("non-null function pointer")(kcontext,
                                                               entry,
                                                               db_args);
    free_db_args(db_args);
    return status;
}
#[no_mangle]
#[c2rust::src_loc = "946:1"]
pub unsafe extern "C" fn krb5_db_put_principal(mut kcontext: krb5_context,
                                               mut entry: *mut krb5_db_entry)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut upd: *mut kdb_incr_update_t = 0 as *mut kdb_incr_update_t;
    let mut princ_name: *mut libc::c_char = 0 as *mut libc::c_char;
    if logging(kcontext) != 0 {
        upd =
            k5alloc(::std::mem::size_of::<kdb_incr_update_t>() as
                        libc::c_ulong, &mut status) as *mut kdb_incr_update_t;
        if upd.is_null() {
            current_block = 2071543600698304364;
        } else {
            status = ulog_conv_2logentry(kcontext, entry, upd);
            if status != 0 {
                current_block = 2071543600698304364;
            } else {
                status =
                    krb5_unparse_name(kcontext,
                                      (*entry).princ as krb5_const_principal,
                                      &mut princ_name);
                if status != 0 as libc::c_int {
                    current_block = 2071543600698304364;
                } else {
                    (*upd).kdb_princ_name.utf8str_t_val = princ_name;
                    (*upd).kdb_princ_name.utf8str_t_len =
                        strlen(princ_name) as u_int;
                    current_block = 11812396948646013369;
                }
            }
        }
    } else { current_block = 11812396948646013369; }
    match current_block {
        11812396948646013369 => {
            status = krb5int_put_principal_no_log(kcontext, entry);
            if !(status != 0) {
                if logging(kcontext) != 0 {
                    status = ulog_add_update(kcontext, upd)
                }
            }
        }
        _ => { }
    }
    ulog_free_entries(upd, 1 as libc::c_int);
    return status;
}
#[no_mangle]
#[c2rust::src_loc = "980:1"]
pub unsafe extern "C" fn krb5int_delete_principal_no_log(mut kcontext:
                                                             krb5_context,
                                                         mut search_for:
                                                             krb5_principal)
 -> krb5_error_code {
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    let mut status: krb5_error_code = 0;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    if (*v).delete_principal.is_none() {
        return -(1765328134 as libc::c_long) as krb5_error_code
    }
    return (*v).delete_principal.expect("non-null function pointer")(kcontext,
                                                                     search_for
                                                                         as
                                                                         krb5_const_principal);
}
#[no_mangle]
#[c2rust::src_loc = "995:1"]
pub unsafe extern "C" fn krb5_db_delete_principal(mut kcontext: krb5_context,
                                                  mut search_for:
                                                      krb5_principal)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut upd: kdb_incr_update_t =
        kdb_incr_update_t{kdb_princ_name:
                              utf8str_t{utf8str_t_len: 0,
                                        utf8str_t_val:
                                            0 as *mut libc::c_char,},
                          kdb_entry_sno: 0,
                          kdb_time: kdbe_time_t{seconds: 0, useconds: 0,},
                          kdb_update:
                              kdbe_t{kdbe_t_len: 0,
                                     kdbe_t_val: 0 as *mut kdbe_val_t,},
                          kdb_deleted: 0,
                          kdb_commit: 0,
                          kdb_kdcs_seen_by:
                              C2RustUnnamed_4{kdb_kdcs_seen_by_len: 0,
                                              kdb_kdcs_seen_by_val:
                                                  0 as *mut utf8str_t,},
                          kdb_futures:
                              C2RustUnnamed_3{kdb_futures_len: 0,
                                              kdb_futures_val:
                                                  0 as *mut libc::c_char,},};
    let mut princ_name: *mut libc::c_char = 0 as *mut libc::c_char;
    status = krb5int_delete_principal_no_log(kcontext, search_for);
    if status != 0 || logging(kcontext) == 0 { return status }
    status =
        krb5_unparse_name(kcontext, search_for as krb5_const_principal,
                          &mut princ_name);
    if status != 0 { return status }
    memset(&mut upd as *mut kdb_incr_update_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<kdb_incr_update_t>() as libc::c_ulong);
    upd.kdb_princ_name.utf8str_t_val = princ_name;
    upd.kdb_princ_name.utf8str_t_len = strlen(princ_name) as u_int;
    upd.kdb_deleted = 1 as libc::c_int;
    status = ulog_add_update(kcontext, &mut upd);
    free(princ_name as *mut libc::c_void);
    return status;
}
#[no_mangle]
#[c2rust::src_loc = "1020:1"]
pub unsafe extern "C" fn krb5_db_rename_principal(mut kcontext: krb5_context,
                                                  mut source: krb5_principal,
                                                  mut target: krb5_principal)
 -> krb5_error_code {
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    let mut status: krb5_error_code = 0;
    let mut entry: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    /*
     * If the default rename function isn't used and logging is enabled, iprop
     * would fail since it doesn't formally support renaming.  In that case
     * return KRB5_PLUGIN_OP_NOTSUPP.
     */
    if (*v).rename_principal !=
           Some(krb5_db_def_rename_principal as
                    unsafe extern "C" fn(_: krb5_context,
                                         _: krb5_const_principal,
                                         _: krb5_const_principal)
                        -> krb5_error_code) && logging(kcontext) != 0 {
        return -(1765328134 as libc::c_long) as krb5_error_code
    }
    status =
        krb5_db_get_principal(kcontext, target as krb5_const_principal,
                              0 as libc::c_int as libc::c_uint, &mut entry);
    if status == 0 as libc::c_int {
        krb5_db_free_principal(kcontext, entry);
        return -(1780008447 as libc::c_long) as krb5_error_code
    }
    return (*v).rename_principal.expect("non-null function pointer")(kcontext,
                                                                     source as
                                                                         krb5_const_principal,
                                                                     target as
                                                                         krb5_const_principal);
}
#[c2rust::src_loc = "1059:1"]
unsafe extern "C" fn sort_entry_callback_proxy(mut func_arg: krb5_pointer,
                                               mut entry: *mut krb5_db_entry)
 -> libc::c_int {
    let mut args: *mut callback_proxy_args =
        func_arg as *mut callback_proxy_args;
    /* Sort the keys in the db entry as some parts of krb5 expect it to be. */
    if !entry.is_null() && !(*entry).key_data.is_null() {
        krb5_dbe_sort_key_data((*entry).key_data,
                               (*entry).n_key_data as size_t);
    }
    return (*args).func.expect("non-null function pointer")((*args).func_arg,
                                                            entry);
}
/*
 * Iterate over principals in the KDB.  If the callback may write to the DB,
 * the caller must get an exclusive lock with krb5_db_lock before iterating,
 * and release it with krb5_db_unlock after iterating.
 */
#[no_mangle]
#[c2rust::src_loc = "1070:1"]
pub unsafe extern "C" fn krb5_db_iterate(mut kcontext: krb5_context,
                                         mut match_entry: *mut libc::c_char,
                                         mut func:
                                             Option<unsafe extern "C" fn(_:
                                                                             krb5_pointer,
                                                                         _:
                                                                             *mut krb5_db_entry)
                                                        -> libc::c_int>,
                                         mut func_arg: krb5_pointer,
                                         mut iterflags: krb5_flags)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    let mut proxy_args: callback_proxy_args =
        callback_proxy_args{func: None, func_arg: 0 as *mut libc::c_void,};
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    if (*v).iterate.is_none() {
        return -(1765328134 as libc::c_long) as krb5_error_code
    }
    /* Use the proxy function to sort key data before passing entries to
     * callback. */
    proxy_args.func = func;
    proxy_args.func_arg = func_arg;
    return (*v).iterate.expect("non-null function pointer")(kcontext,
                                                            match_entry,
                                                            Some(sort_entry_callback_proxy
                                                                     as
                                                                     unsafe extern "C" fn(_:
                                                                                              krb5_pointer,
                                                                                          _:
                                                                                              *mut krb5_db_entry)
                                                                         ->
                                                                             libc::c_int),
                                                            &mut proxy_args as
                                                                *mut callback_proxy_args
                                                                as
                                                                krb5_pointer,
                                                            iterflags);
}
/* Return a read only pointer alias to mkey list.  Do not free this! */
#[no_mangle]
#[c2rust::src_loc = "1094:1"]
pub unsafe extern "C" fn krb5_db_mkey_list_alias(mut kcontext: krb5_context)
 -> *mut krb5_keylist_node {
    return (*(*kcontext).dal_handle).master_keylist;
}
#[no_mangle]
#[c2rust::src_loc = "1100:1"]
pub unsafe extern "C" fn krb5_db_fetch_mkey_list(mut context: krb5_context,
                                                 mut mname: krb5_principal,
                                                 mut mkey:
                                                     *const krb5_keyblock)
 -> krb5_error_code {
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut local_keylist: *mut krb5_keylist_node =
        0 as *mut krb5_keylist_node;
    status = get_vftabl(context, &mut v);
    if status != 0 { return status }
    if (*(*context).dal_handle).master_princ.is_null() {
        status =
            krb5_copy_principal(context, mname as krb5_const_principal,
                                &mut (*(*context).dal_handle).master_princ);
        if status != 0 { return status }
    }
    status =
        (*v).fetch_master_key_list.expect("non-null function pointer")(context,
                                                                       mname,
                                                                       mkey,
                                                                       &mut local_keylist);
    if status == 0 as libc::c_int {
        free_mkey_list(context, (*(*context).dal_handle).master_keylist);
        (*(*context).dal_handle).master_keylist = local_keylist
    }
    return status;
}
#[no_mangle]
#[c2rust::src_loc = "1127:1"]
pub unsafe extern "C" fn krb5_db_store_master_key(mut kcontext: krb5_context,
                                                  mut keyfile:
                                                      *mut libc::c_char,
                                                  mut mname: krb5_principal,
                                                  mut kvno: krb5_kvno,
                                                  mut key: *mut krb5_keyblock,
                                                  mut master_pwd:
                                                      *mut libc::c_char)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    let mut list: krb5_keylist_node =
        krb5_keylist_node{keyblock:
                              krb5_keyblock{magic: 0,
                                            enctype: 0,
                                            length: 0,
                                            contents: 0 as *mut krb5_octet,},
                          kvno: 0,
                          next: 0 as *mut _krb5_keylist_node,};
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    if (*v).store_master_key_list.is_none() {
        return -(1780008415 as libc::c_long) as krb5_error_code
    }
    list.kvno = kvno;
    list.keyblock = *key;
    list.next = 0 as *mut _krb5_keylist_node;
    return (*v).store_master_key_list.expect("non-null function pointer")(kcontext,
                                                                          keyfile,
                                                                          mname,
                                                                          &mut list,
                                                                          master_pwd);
}
#[no_mangle]
#[c2rust::src_loc = "1151:1"]
pub unsafe extern "C" fn krb5_db_store_master_key_list(mut kcontext:
                                                           krb5_context,
                                                       mut keyfile:
                                                           *mut libc::c_char,
                                                       mut mname:
                                                           krb5_principal,
                                                       mut master_pwd:
                                                           *mut libc::c_char)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    if (*v).store_master_key_list.is_none() {
        return -(1780008415 as libc::c_long) as krb5_error_code
    }
    if (*(*kcontext).dal_handle).master_keylist.is_null() {
        return -(1780008435 as libc::c_long) as krb5_error_code
    }
    return (*v).store_master_key_list.expect("non-null function pointer")(kcontext,
                                                                          keyfile,
                                                                          mname,
                                                                          (*(*kcontext).dal_handle).master_keylist,
                                                                          master_pwd);
}
#[no_mangle]
#[c2rust::src_loc = "1173:9"]
pub static mut krb5_mkey_pwd_prompt1: *mut libc::c_char =
    b"Enter KDC database master key\x00" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
#[no_mangle]
#[c2rust::src_loc = "1174:9"]
pub static mut krb5_mkey_pwd_prompt2: *mut libc::c_char =
    b"Re-enter KDC database master key to verify\x00" as *const u8 as
        *const libc::c_char as *mut libc::c_char;
#[no_mangle]
#[c2rust::src_loc = "1176:1"]
pub unsafe extern "C" fn krb5_db_fetch_mkey(mut context: krb5_context,
                                            mut mname: krb5_principal,
                                            mut etype: krb5_enctype,
                                            mut fromkeyboard: krb5_boolean,
                                            mut twice: krb5_boolean,
                                            mut db_args: *mut libc::c_char,
                                            mut kvno: *mut krb5_kvno,
                                            mut salt: *mut krb5_data,
                                            mut key: *mut krb5_keyblock)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut password: [libc::c_char; 8192] = [0; 8192];
    let mut pwd: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut size: libc::c_uint =
        ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as
            libc::c_uint;
    let mut tmp_key: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    memset(&mut tmp_key as *mut krb5_keyblock as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong);
    if fromkeyboard != 0 {
        let mut scratch: krb5_data =
            krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
        retval =
            krb5_read_password(context, krb5_mkey_pwd_prompt1,
                               if twice != 0 {
                                   krb5_mkey_pwd_prompt2
                               } else { 0 as *mut libc::c_char },
                               password.as_mut_ptr(), &mut size);
        if !(retval != 0) {
            pwd.data = password.as_mut_ptr();
            pwd.length = size;
            if salt.is_null() {
                retval =
                    krb5_principal2salt(context,
                                        mname as krb5_const_principal,
                                        &mut scratch);
                if retval != 0 {
                    current_block = 10991094515395304355;
                } else { current_block = 5399440093318478209; }
            } else { current_block = 5399440093318478209; }
            match current_block {
                10991094515395304355 => { }
                _ => {
                    retval =
                        krb5_c_string_to_key(context, etype, &mut pwd,
                                             if !salt.is_null() {
                                                 salt
                                             } else { &mut scratch }, key);
                    /* erase it */
                    /*
         * If a kvno pointer was passed in and it dereferences the IGNORE_VNO
         * value then it should be assigned the value of the kvno associated
         * with the current mkey princ key if that princ entry is available
         * otherwise assign 1 which is the default kvno value for the mkey
         * princ.
         */
                    if !kvno.is_null() &&
                           *kvno == 0 as libc::c_int as libc::c_uint {
                        let mut rc: krb5_error_code = 0;
                        let mut master_entry: *mut krb5_db_entry =
                            0 as *mut krb5_db_entry;
                        rc =
                            krb5_db_get_principal(context,
                                                  mname as
                                                      krb5_const_principal,
                                                  0 as libc::c_int as
                                                      libc::c_uint,
                                                  &mut master_entry);
                        if rc == 0 as libc::c_int &&
                               (*master_entry).n_key_data as libc::c_int >
                                   0 as libc::c_int {
                            *kvno =
                                (*(*master_entry).key_data).key_data_kvno as
                                    krb5_kvno
                        } else { *kvno = 1 as libc::c_int as krb5_kvno }
                        if rc == 0 as libc::c_int {
                            krb5_db_free_principal(context, master_entry);
                        }
                    }
                    if salt.is_null() {
                        free(scratch.data as *mut libc::c_void);
                    }
                    explicit_bzero(password.as_mut_ptr() as *mut libc::c_void,
                                   ::std::mem::size_of::<[libc::c_char; 8192]>()
                                       as libc::c_ulong);
                }
            }
        }
    } else {
        let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
        if (*context).dal_handle.is_null() {
            retval = krb5_db_setup_lib_handle(context);
            if retval != 0 {
                current_block = 10991094515395304355;
            } else { current_block = 14359455889292382949; }
        } else { current_block = 14359455889292382949; }
        match current_block {
            10991094515395304355 => { }
            _ => {
                /* get the enctype from the stash */
                tmp_key.enctype = 0x1ff as libc::c_int;
                v = &mut (*(*(*context).dal_handle).lib_handle).vftabl;
                retval =
                    (*v).fetch_master_key.expect("non-null function pointer")(context,
                                                                              mname,
                                                                              &mut tmp_key,
                                                                              kvno,
                                                                              db_args);
                if !(retval != 0) {
                    (*key).contents =
                        k5memdup(tmp_key.contents as *const libc::c_void,
                                 tmp_key.length as size_t, &mut retval) as
                            *mut krb5_octet;
                    if !(*key).contents.is_null() {
                        (*key).magic = tmp_key.magic;
                        (*key).enctype = tmp_key.enctype;
                        (*key).length = tmp_key.length
                    }
                }
            }
        }
    }
    zapfree(tmp_key.contents as *mut libc::c_void, tmp_key.length as size_t);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "1265:1"]
pub unsafe extern "C" fn krb5_dbe_fetch_act_key_list(mut context:
                                                         krb5_context,
                                                     mut princ:
                                                         krb5_principal,
                                                     mut act_key_list:
                                                         *mut *mut krb5_actkvno_node)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut entry: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    if act_key_list.is_null() { return 22 as libc::c_int }
    retval =
        krb5_db_get_principal(context, princ as krb5_const_principal,
                              0 as libc::c_int as libc::c_uint, &mut entry);
    if retval as libc::c_long == -(1780008443 as libc::c_long) {
        return -(1780008432 as libc::c_long) as krb5_error_code
    } else { if retval != 0 { return retval } }
    retval = krb5_dbe_lookup_actkvno(context, entry, act_key_list);
    krb5_db_free_principal(context, entry);
    return retval;
}
/* Find the most recent entry in list (which must not be empty) for the given
 * timestamp, and return its kvno. */
#[c2rust::src_loc = "1288:1"]
unsafe extern "C" fn find_actkvno(mut list: *mut krb5_actkvno_node,
                                  mut now: krb5_timestamp) -> krb5_kvno {
    /*
     * The list is sorted in ascending order of time.  Return the kvno of the
     * predecessor of the first entry whose time is in the future.  If
     * (contrary to the safety checks in kdb5_util use_mkey) all of the entries
     * are in the future, we will return the first node; if all are in the
     * past, we will return the last node.
     */
    while !(*list).next.is_null() &&
              ts_after((*(*list).next).act_time, now) == 0 {
        list = (*list).next
    }
    return (*list).act_kvno;
}
/* Search the master keylist for the master key with the specified kvno.
 * Return the keyblock of the matching entry or NULL if it does not exist. */
#[c2rust::src_loc = "1305:1"]
unsafe extern "C" fn find_master_key(mut context: krb5_context,
                                     mut kvno: krb5_kvno)
 -> *mut krb5_keyblock {
    let mut n: *mut krb5_keylist_node = 0 as *mut krb5_keylist_node;
    n = (*(*context).dal_handle).master_keylist;
    while !n.is_null() {
        if (*n).kvno == kvno { return &mut (*n).keyblock }
        n = (*n).next
    }
    return 0 as *mut krb5_keyblock;
}
/*
 * Locates the "active" mkey used when encrypting a princ's keys.  Note, the
 * caller must NOT free the output act_mkey.
 */
#[no_mangle]
#[c2rust::src_loc = "1322:1"]
pub unsafe extern "C" fn krb5_dbe_find_act_mkey(mut context: krb5_context,
                                                mut act_mkey_list:
                                                    *mut krb5_actkvno_node,
                                                mut act_kvno: *mut krb5_kvno,
                                                mut act_mkey:
                                                    *mut *mut krb5_keyblock)
 -> krb5_error_code {
    let mut kvno: krb5_kvno = 0;
    let mut retval: krb5_error_code = 0;
    let mut mkey: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut cur_mkey: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut now: krb5_timestamp = 0;
    if act_mkey_list.is_null() {
        *act_kvno = 0 as libc::c_int as krb5_kvno;
        *act_mkey = 0 as *mut krb5_keyblock;
        return 0 as libc::c_int
    }
    if (*(*context).dal_handle).master_keylist.is_null() {
        return -(1780008435 as libc::c_long) as krb5_error_code
    }
    /* Find the currently active master key version. */
    retval = krb5_timeofday(context, &mut now);
    if retval != 0 { return retval }
    kvno = find_actkvno(act_mkey_list, now);
    /* Find the corresponding master key. */
    mkey = find_master_key(context, kvno);
    if mkey.is_null() {
        /* Reload the master key list and try again. */
        cur_mkey = &mut (*(*(*context).dal_handle).master_keylist).keyblock;
        if krb5_db_fetch_mkey_list(context,
                                   (*(*context).dal_handle).master_princ,
                                   cur_mkey) == 0 as libc::c_int {
            mkey = find_master_key(context, kvno)
        }
    }
    if mkey.is_null() {
        return -(1780008417 as libc::c_long) as krb5_error_code
    }
    *act_mkey = mkey;
    if !act_kvno.is_null() { *act_kvno = kvno }
    return 0 as libc::c_int;
}
/*
 * Locates the mkey used to protect a princ's keys.  Note, the caller must not
 * free the output key.
 */
#[no_mangle]
#[c2rust::src_loc = "1367:1"]
pub unsafe extern "C" fn krb5_dbe_find_mkey(mut context: krb5_context,
                                            mut entry: *mut krb5_db_entry,
                                            mut mkey: *mut *mut krb5_keyblock)
 -> krb5_error_code {
    let mut mkvno: krb5_kvno = 0;
    let mut retval: krb5_error_code = 0;
    let mut cur_keyblock: *mut krb5_keylist_node =
        (*(*context).dal_handle).master_keylist;
    if cur_keyblock.is_null() {
        return -(1780008435 as libc::c_long) as krb5_error_code
    }
    retval = krb5_dbe_get_mkvno(context, entry, &mut mkvno);
    if retval != 0 { return retval }
    while !cur_keyblock.is_null() && (*cur_keyblock).kvno != mkvno {
        cur_keyblock = (*cur_keyblock).next
    }
    if !cur_keyblock.is_null() {
        *mkey = &mut (*cur_keyblock).keyblock;
        return 0 as libc::c_int
    } else { return -(1780008417 as libc::c_long) as krb5_error_code };
}
/*
 * These are wrappers around realloc() and free().  Applications and KDB
 * modules can use them when manipulating principal and policy entries to
 * ensure that they allocate and free memory in a manner compatible with the
 * library.  Using libkrb5 or libkbd5 functions to construct values (such as
 * krb5_copy_principal() to construct the princ field of a krb5_db_entry) is
 * also safe.  On Unix platforms, just using malloc() and free() is safe as
 * long as the application or module does not use a malloc replacement.
 */
#[no_mangle]
#[c2rust::src_loc = "1393:1"]
pub unsafe extern "C" fn krb5_db_alloc(mut kcontext: krb5_context,
                                       mut ptr: *mut libc::c_void,
                                       mut size: size_t)
 -> *mut libc::c_void {
    return realloc(ptr, size);
}
#[no_mangle]
#[c2rust::src_loc = "1399:1"]
pub unsafe extern "C" fn krb5_db_free(mut kcontext: krb5_context,
                                      mut ptr: *mut libc::c_void) {
    free(ptr);
}
/* has to be modified */
#[no_mangle]
#[c2rust::src_loc = "1407:1"]
pub unsafe extern "C" fn krb5_dbe_find_enctype(mut kcontext: krb5_context,
                                               mut dbentp: *mut krb5_db_entry,
                                               mut ktype: krb5_int32,
                                               mut stype: krb5_int32,
                                               mut kvno: krb5_int32,
                                               mut kdatap:
                                                   *mut *mut krb5_key_data)
 -> krb5_error_code {
    let mut start: krb5_int32 = 0 as libc::c_int; /* XXX external? */
    return krb5_dbe_search_enctype(kcontext, dbentp, &mut start, ktype, stype,
                                   kvno, kdatap);
}
#[no_mangle]
#[c2rust::src_loc = "1417:1"]
pub unsafe extern "C" fn krb5_dbe_search_enctype(mut kcontext: krb5_context,
                                                 mut dbentp:
                                                     *mut krb5_db_entry,
                                                 mut start: *mut krb5_int32,
                                                 mut ktype: krb5_int32,
                                                 mut stype: krb5_int32,
                                                 mut kvno: krb5_int32,
                                                 mut kdatap:
                                                     *mut *mut krb5_key_data)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    return (*v).dbe_search_enctype.expect("non-null function pointer")(kcontext,
                                                                       dbentp,
                                                                       start,
                                                                       ktype,
                                                                       stype,
                                                                       kvno,
                                                                       kdatap);
}
#[no_mangle]
#[c2rust::src_loc = "1434:1"]
pub unsafe extern "C" fn krb5_db_setup_mkey_name(mut context: krb5_context,
                                                 mut keyname:
                                                     *const libc::c_char,
                                                 mut realm:
                                                     *const libc::c_char,
                                                 mut fullname:
                                                     *mut *mut libc::c_char,
                                                 mut principal:
                                                     *mut krb5_principal)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
    if keyname.is_null() {
        keyname = b"K/M\x00" as *const u8 as *const libc::c_char
    }
    if asprintf(&mut fname as *mut *mut libc::c_char,
                b"%s%s%s\x00" as *const u8 as *const libc::c_char, keyname,
                b"@\x00" as *const u8 as *const libc::c_char, realm) <
           0 as libc::c_int {
        return 12 as libc::c_int
    }
    retval = krb5_parse_name(context, fname, principal);
    if retval != 0 { return retval }
    if !fullname.is_null() {
        *fullname = fname
    } else { free(fname as *mut libc::c_void); }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1457:1"]
pub unsafe extern "C" fn krb5_dbe_lookup_last_pwd_change(mut context:
                                                             krb5_context,
                                                         mut entry:
                                                             *mut krb5_db_entry,
                                                         mut stamp:
                                                             *mut krb5_timestamp)
 -> krb5_error_code {
    let mut tl_data: krb5_tl_data =
        krb5_tl_data{tl_data_next: 0 as *mut _krb5_tl_data,
                     tl_data_type: 0,
                     tl_data_length: 0,
                     tl_data_contents: 0 as *mut krb5_octet,};
    let mut code: krb5_error_code = 0;
    let mut tmp: krb5_int32 = 0;
    tl_data.tl_data_type = 0x1 as libc::c_int as krb5_int16;
    code = krb5_dbe_lookup_tl_data(context, entry, &mut tl_data);
    if code != 0 { return code }
    if tl_data.tl_data_length as libc::c_int != 4 as libc::c_int {
        *stamp = 0 as libc::c_int;
        return 0 as libc::c_int
    }
    *(&mut tmp as *mut krb5_int32) =
        load_32_le(tl_data.tl_data_contents as *const libc::c_void) as
            krb5_int32;
    *stamp = tmp;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1482:1"]
pub unsafe extern "C" fn krb5_dbe_lookup_last_admin_unlock(mut context:
                                                               krb5_context,
                                                           mut entry:
                                                               *mut krb5_db_entry,
                                                           mut stamp:
                                                               *mut krb5_timestamp)
 -> krb5_error_code {
    let mut tl_data: krb5_tl_data =
        krb5_tl_data{tl_data_next: 0 as *mut _krb5_tl_data,
                     tl_data_type: 0,
                     tl_data_length: 0,
                     tl_data_contents: 0 as *mut krb5_octet,};
    let mut code: krb5_error_code = 0;
    let mut tmp: krb5_int32 = 0;
    tl_data.tl_data_type = 0x700 as libc::c_int as krb5_int16;
    code = krb5_dbe_lookup_tl_data(context, entry, &mut tl_data);
    if code != 0 { return code }
    if tl_data.tl_data_length as libc::c_int != 4 as libc::c_int {
        *stamp = 0 as libc::c_int;
        return 0 as libc::c_int
    }
    *(&mut tmp as *mut krb5_int32) =
        load_32_le(tl_data.tl_data_contents as *const libc::c_void) as
            krb5_int32;
    *stamp = tmp;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1507:1"]
pub unsafe extern "C" fn krb5_dbe_lookup_tl_data(mut context: krb5_context,
                                                 mut entry:
                                                     *mut krb5_db_entry,
                                                 mut ret_tl_data:
                                                     *mut krb5_tl_data)
 -> krb5_error_code {
    let mut tl_data: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    tl_data = (*entry).tl_data;
    while !tl_data.is_null() {
        if (*tl_data).tl_data_type as libc::c_int ==
               (*ret_tl_data).tl_data_type as libc::c_int {
            *ret_tl_data = *tl_data;
            return 0 as libc::c_int
        }
        tl_data = (*tl_data).tl_data_next
    }
    /*
     * If the requested record isn't found, return zero bytes.  If it
     * ever means something to have a zero-length tl_data, this code
     * and its callers will have to be changed.
     */
    (*ret_tl_data).tl_data_length = 0 as libc::c_int as krb5_ui_2;
    (*ret_tl_data).tl_data_contents = 0 as *mut krb5_octet;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1531:1"]
pub unsafe extern "C" fn krb5_dbe_create_key_data(mut context: krb5_context,
                                                  mut entry:
                                                      *mut krb5_db_entry)
 -> krb5_error_code {
    let mut newptr: *mut krb5_key_data = 0 as *mut krb5_key_data;
    newptr =
        realloc((*entry).key_data as *mut libc::c_void,
                (((*entry).n_key_data as libc::c_int + 1 as libc::c_int) as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_key_data>()
                                                     as libc::c_ulong)) as
            *mut krb5_key_data;
    if newptr.is_null() { return 12 as libc::c_int }
    (*entry).key_data = newptr;
    memset((*entry).key_data.offset((*entry).n_key_data as libc::c_int as
                                        isize) as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_key_data>() as libc::c_ulong);
    (*entry).n_key_data += 1;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1548:1"]
pub unsafe extern "C" fn krb5_dbe_update_mod_princ_data(mut context:
                                                            krb5_context,
                                                        mut entry:
                                                            *mut krb5_db_entry,
                                                        mut mod_date:
                                                            krb5_timestamp,
                                                        mut mod_princ:
                                                            krb5_const_principal)
 -> krb5_error_code {
    let mut tl_data: krb5_tl_data =
        krb5_tl_data{tl_data_next: 0 as *mut _krb5_tl_data,
                     tl_data_type: 0,
                     tl_data_length: 0,
                     tl_data_contents: 0 as *mut krb5_octet,};
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut nextloc: *mut krb5_octet = 0 as *mut krb5_octet;
    let mut unparse_mod_princ: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut unparse_mod_princ_size: libc::c_uint = 0;
    retval = krb5_unparse_name(context, mod_princ, &mut unparse_mod_princ);
    if retval != 0 { return retval }
    unparse_mod_princ_size =
        strlen(unparse_mod_princ).wrapping_add(1 as libc::c_int as
                                                   libc::c_ulong) as
            libc::c_uint;
    nextloc =
        malloc(unparse_mod_princ_size.wrapping_add(4 as libc::c_int as
                                                       libc::c_uint) as
                   libc::c_ulong) as *mut krb5_octet;
    if nextloc.is_null() {
        free(unparse_mod_princ as *mut libc::c_void);
        return 12 as libc::c_int
    }
    tl_data.tl_data_type = 0x2 as libc::c_int as krb5_int16;
    tl_data.tl_data_length =
        unparse_mod_princ_size.wrapping_add(4 as libc::c_int as libc::c_uint)
            as krb5_ui_2;
    tl_data.tl_data_contents = nextloc;
    /* Mod Date */
    store_32_le(mod_date as libc::c_uint, nextloc as *mut libc::c_void);
    /* Mod Princ */
    memcpy(nextloc.offset(4 as libc::c_int as isize) as *mut libc::c_void,
           unparse_mod_princ as *const libc::c_void,
           unparse_mod_princ_size as libc::c_ulong);
    retval = krb5_dbe_update_tl_data(context, entry, &mut tl_data);
    free(unparse_mod_princ as *mut libc::c_void);
    free(nextloc as *mut libc::c_void);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "1589:1"]
pub unsafe extern "C" fn krb5_dbe_lookup_mod_princ_data(mut context:
                                                            krb5_context,
                                                        mut entry:
                                                            *mut krb5_db_entry,
                                                        mut mod_time:
                                                            *mut krb5_timestamp,
                                                        mut mod_princ:
                                                            *mut krb5_principal)
 -> krb5_error_code {
    let mut tl_data: krb5_tl_data =
        krb5_tl_data{tl_data_next: 0 as *mut _krb5_tl_data,
                     tl_data_type: 0,
                     tl_data_length: 0,
                     tl_data_contents: 0 as *mut krb5_octet,};
    let mut code: krb5_error_code = 0;
    *mod_princ = 0 as krb5_principal;
    *mod_time = 0 as libc::c_int;
    tl_data.tl_data_type = 0x2 as libc::c_int as krb5_int16;
    code = krb5_dbe_lookup_tl_data(context, entry, &mut tl_data);
    if code != 0 { return code }
    if (tl_data.tl_data_length as libc::c_int) < 5 as libc::c_int ||
           *tl_data.tl_data_contents.offset((tl_data.tl_data_length as
                                                 libc::c_int -
                                                 1 as libc::c_int) as isize)
               as libc::c_int != '\u{0}' as i32 {
        return -(1780008439 as libc::c_long) as krb5_error_code
    }
    /* Mod Date */
    *(mod_time as *mut krb5_int32) =
        load_32_le(tl_data.tl_data_contents as *const libc::c_void) as
            krb5_int32;
    /* Mod Princ */
    code =
        krb5_parse_name(context,
                        tl_data.tl_data_contents.offset(4 as libc::c_int as
                                                            isize) as
                            *const libc::c_char, mod_princ);
    if code != 0 { return code }
    return 0 as libc::c_int;
}
/* Set *mkvno to mkvno in entry tl_data, or 0 if not present. */
#[no_mangle]
#[c2rust::src_loc = "1621:1"]
pub unsafe extern "C" fn krb5_dbe_lookup_mkvno(mut context: krb5_context,
                                               mut entry: *mut krb5_db_entry,
                                               mut mkvno: *mut krb5_kvno)
 -> krb5_error_code {
    let mut tl_data: krb5_tl_data =
        krb5_tl_data{tl_data_next: 0 as *mut _krb5_tl_data,
                     tl_data_type: 0,
                     tl_data_length: 0,
                     tl_data_contents:
                         0 as
                             *mut krb5_octet,}; /* Indicates KRB5_TL_MKVNO data not present */
    let mut code: krb5_error_code = 0;
    let mut tmp: krb5_int16 = 0;
    tl_data.tl_data_type = 0x8 as libc::c_int as krb5_int16;
    code = krb5_dbe_lookup_tl_data(context, entry, &mut tl_data);
    if code != 0 { return code }
    if tl_data.tl_data_length as libc::c_int == 0 as libc::c_int {
        *mkvno = 0 as libc::c_int as krb5_kvno;
        return 0 as libc::c_int
    } else {
        if tl_data.tl_data_length as libc::c_int != 2 as libc::c_int {
            return -(1780008439 as libc::c_long) as krb5_error_code
        }
    }
    *(&mut tmp as *mut krb5_int16) =
        load_16_le(tl_data.tl_data_contents as *const libc::c_void) as
            krb5_int16;
    *mkvno = tmp as krb5_kvno;
    return 0 as libc::c_int;
}
/* Set *mkvno to mkvno in entry tl_data, or minimum value from mkey_list. */
#[no_mangle]
#[c2rust::src_loc = "1646:1"]
pub unsafe extern "C" fn krb5_dbe_get_mkvno(mut context: krb5_context,
                                            mut entry: *mut krb5_db_entry,
                                            mut mkvno: *mut krb5_kvno)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut kvno: krb5_kvno = 0;
    let mut mkey_list: *mut krb5_keylist_node =
        (*(*context).dal_handle).master_keylist;
    if mkey_list.is_null() {
        return -(1780008435 as libc::c_long) as krb5_error_code
    }
    /* Output the value from entry tl_data if present. */
    code = krb5_dbe_lookup_mkvno(context, entry, &mut kvno);
    if code != 0 as libc::c_int { return code }
    if kvno != 0 as libc::c_int as libc::c_uint {
        *mkvno = kvno;
        return 0 as libc::c_int
    }
    /* Determine the minimum kvno in mkey_list and output that. */
    kvno =
        -(1 as libc::c_int) as
            krb5_kvno; /* this is the encoded size of an int16 */
    while !mkey_list.is_null() {
        if (*mkey_list).kvno < kvno {
            kvno = (*mkey_list).kvno
        } /* current location pointer */
        mkey_list = (*mkey_list).next
    }
    *mkvno = kvno;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1677:1"]
pub unsafe extern "C" fn krb5_dbe_update_mkvno(mut context: krb5_context,
                                               mut entry: *mut krb5_db_entry,
                                               mut mkvno: krb5_kvno)
 -> krb5_error_code {
    let mut tl_data: krb5_tl_data =
        krb5_tl_data{tl_data_next: 0 as *mut _krb5_tl_data,
                     tl_data_type: 0,
                     tl_data_length: 0,
                     tl_data_contents: 0 as *mut krb5_octet,};
    let mut buf: [krb5_octet; 2] = [0; 2];
    let mut tmp_kvno: krb5_int16 = mkvno as krb5_int16;
    tl_data.tl_data_type = 0x8 as libc::c_int as krb5_int16;
    tl_data.tl_data_length =
        ::std::mem::size_of::<[krb5_octet; 2]>() as libc::c_ulong as
            krb5_ui_2;
    store_16_le(tmp_kvno as libc::c_uint,
                buf.as_mut_ptr() as *mut libc::c_void);
    tl_data.tl_data_contents = buf.as_mut_ptr();
    return krb5_dbe_update_tl_data(context, entry, &mut tl_data);
}
#[no_mangle]
#[c2rust::src_loc = "1693:1"]
pub unsafe extern "C" fn krb5_dbe_lookup_mkey_aux(mut context: krb5_context,
                                                  mut entry:
                                                      *mut krb5_db_entry,
                                                  mut mkey_aux_data_list:
                                                      *mut *mut krb5_mkey_aux_node)
 -> krb5_error_code {
    let mut tl_data: krb5_tl_data =
        krb5_tl_data{tl_data_next: 0 as *mut _krb5_tl_data,
                     tl_data_type: 0,
                     tl_data_length: 0,
                     tl_data_contents: 0 as *mut krb5_octet,};
    let mut version: krb5_int16 = 0;
    let mut mkey_kvno: krb5_int16 = 0;
    let mut head_data: *mut krb5_mkey_aux_node = 0 as *mut krb5_mkey_aux_node;
    let mut new_data: *mut krb5_mkey_aux_node = 0 as *mut krb5_mkey_aux_node;
    let mut prev_data: *mut krb5_mkey_aux_node = 0 as *mut krb5_mkey_aux_node;
    let mut curloc: *mut krb5_octet = 0 as *mut krb5_octet;
    let mut code: krb5_error_code = 0;
    tl_data.tl_data_type = 0xa as libc::c_int as krb5_int16;
    code = krb5_dbe_lookup_tl_data(context, entry, &mut tl_data);
    if code != 0 { return code }
    if tl_data.tl_data_contents.is_null() {
        *mkey_aux_data_list = 0 as *mut krb5_mkey_aux_node;
        return 0 as libc::c_int
    } else {
        /* get version to determine how to parse the data */
        *(&mut version as *mut krb5_int16) =
            load_16_le(tl_data.tl_data_contents as *const libc::c_void) as
                krb5_int16;
        if version as libc::c_int == 1 as libc::c_int {
            /* variable size, must be at least 10 bytes */
            if (tl_data.tl_data_length as libc::c_int) < 10 as libc::c_int {
                return -(1780008439 as libc::c_long) as krb5_error_code
            }
            /* curloc points to first tuple entry in the tl_data_contents */
            curloc =
                tl_data.tl_data_contents.offset(::std::mem::size_of::<krb5_int16>()
                                                    as libc::c_ulong as
                                                    isize);
            while curloc <
                      tl_data.tl_data_contents.offset(tl_data.tl_data_length
                                                          as libc::c_int as
                                                          isize) {
                new_data =
                    malloc(::std::mem::size_of::<krb5_mkey_aux_node>() as
                               libc::c_ulong) as *mut krb5_mkey_aux_node;
                if new_data.is_null() {
                    krb5_dbe_free_mkey_aux_list(context, head_data);
                    return 12 as libc::c_int
                }
                memset(new_data as *mut libc::c_void, 0 as libc::c_int,
                       ::std::mem::size_of::<krb5_mkey_aux_node>() as
                           libc::c_ulong);
                *(&mut mkey_kvno as *mut krb5_int16) =
                    load_16_le(curloc as *const libc::c_void) as krb5_int16;
                (*new_data).mkey_kvno = mkey_kvno as krb5_kvno;
                curloc =
                    curloc.offset(::std::mem::size_of::<krb5_ui_2>() as
                                      libc::c_ulong as isize);
                *(&mut (*new_data).latest_mkey.key_data_kvno as *mut krb5_ui_2
                      as *mut krb5_int16) =
                    load_16_le(curloc as *const libc::c_void) as krb5_int16;
                curloc =
                    curloc.offset(::std::mem::size_of::<krb5_ui_2>() as
                                      libc::c_ulong as isize);
                *(&mut *(*new_data).latest_mkey.key_data_type.as_mut_ptr().offset(0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize)
                      as *mut krb5_int16) =
                    load_16_le(curloc as *const libc::c_void) as krb5_int16;
                curloc =
                    curloc.offset(::std::mem::size_of::<krb5_ui_2>() as
                                      libc::c_ulong as isize);
                *(&mut *(*new_data).latest_mkey.key_data_length.as_mut_ptr().offset(0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                      as *mut krb5_ui_2 as *mut krb5_int16) =
                    load_16_le(curloc as *const libc::c_void) as krb5_int16;
                curloc =
                    curloc.offset(::std::mem::size_of::<krb5_ui_2>() as
                                      libc::c_ulong as isize);
                (*new_data).latest_mkey.key_data_contents[0 as libc::c_int as
                                                              usize] =
                    malloc((*new_data).latest_mkey.key_data_length[0 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                               as libc::c_ulong) as *mut krb5_octet;
                if (*new_data).latest_mkey.key_data_contents[0 as libc::c_int
                                                                 as
                                                                 usize].is_null()
                   {
                    krb5_dbe_free_mkey_aux_list(context, head_data);
                    free(new_data as *mut libc::c_void);
                    return 12 as libc::c_int
                }
                memcpy((*new_data).latest_mkey.key_data_contents[0 as
                                                                     libc::c_int
                                                                     as usize]
                           as *mut libc::c_void,
                       curloc as *const libc::c_void,
                       (*new_data).latest_mkey.key_data_length[0 as
                                                                   libc::c_int
                                                                   as usize]
                           as libc::c_ulong);
                curloc =
                    curloc.offset((*new_data).latest_mkey.key_data_length[0 as
                                                                              libc::c_int
                                                                              as
                                                                              usize]
                                      as libc::c_int as isize);
                /* always using key data ver 1 for mkeys */
                (*new_data).latest_mkey.key_data_ver =
                    1 as libc::c_int as krb5_int16;
                (*new_data).next = 0 as *mut _krb5_mkey_aux_node;
                if !prev_data.is_null() {
                    (*prev_data).next = new_data
                } else { head_data = new_data }
                prev_data = new_data
            }
        } else {
            krb5_set_error_message(context,
                                   -(1780008422 as libc::c_long) as
                                       krb5_error_code,
                                   dgettext(b"mit-krb5\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"Illegal version number for KRB5_TL_MKEY_AUX %d\n\x00"
                                                as *const u8 as
                                                *const libc::c_char),
                                   version as libc::c_int);
            return -(1780008422 as libc::c_long) as krb5_error_code
        }
    }
    *mkey_aux_data_list = head_data;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1775:1"]
pub unsafe extern "C" fn krb5_dbe_update_mkey_aux(mut context: krb5_context,
                                                  mut entry:
                                                      *mut krb5_db_entry,
                                                  mut mkey_aux_data_list:
                                                      *mut krb5_mkey_aux_node)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0;
    let mut tl_data: krb5_tl_data =
        krb5_tl_data{tl_data_next: 0 as *mut _krb5_tl_data,
                     tl_data_type: 0,
                     tl_data_length: 0,
                     tl_data_contents: 0 as *mut krb5_octet,};
    let mut version: krb5_int16 = 0;
    let mut tmp_kvno: krb5_int16 = 0;
    let mut nextloc: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut aux_data_entry: *mut krb5_mkey_aux_node =
        0 as *mut krb5_mkey_aux_node;
    if mkey_aux_data_list.is_null() {
        /* delete the KRB5_TL_MKEY_AUX from the entry */
        krb5_dbe_delete_tl_data(context, entry,
                                0xa as libc::c_int as krb5_int16);
        return 0 as libc::c_int
    }
    memset(&mut tl_data as *mut krb5_tl_data as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_tl_data>() as libc::c_ulong);
    tl_data.tl_data_type = 0xa as libc::c_int as krb5_int16;
    /*
     * determine out how much space to allocate.  Note key_data_ver not stored
     * as this is hard coded to one and is accounted for in
     * krb5_dbe_lookup_mkey_aux.
     */
    tl_data.tl_data_length =
        ::std::mem::size_of::<krb5_int16>() as libc::c_ulong as
            krb5_ui_2; /* version */
    aux_data_entry = mkey_aux_data_list;
    while !aux_data_entry.is_null() {
        tl_data.tl_data_length =
            (tl_data.tl_data_length as
                 libc::c_ulong).wrapping_add((::std::mem::size_of::<krb5_ui_2>()
                                                  as
                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<krb5_ui_2>()
                                                                                  as
                                                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<krb5_ui_2>()
                                                                                                                  as
                                                                                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<krb5_ui_2>()
                                                                                                                                                  as
                                                                                                                                                  libc::c_ulong).wrapping_add((*aux_data_entry).latest_mkey.key_data_length[0
                                                                                                                                                                                                                                as
                                                                                                                                                                                                                                libc::c_int
                                                                                                                                                                                                                                as
                                                                                                                                                                                                                                usize]
                                                                                                                                                                                  as
                                                                                                                                                                                  libc::c_ulong))
                as krb5_ui_2 as krb5_ui_2;
        aux_data_entry = (*aux_data_entry).next
    }
    tl_data.tl_data_contents =
        malloc(tl_data.tl_data_length as libc::c_ulong) as *mut krb5_octet;
    if tl_data.tl_data_contents.is_null() { return 12 as libc::c_int }
    nextloc = tl_data.tl_data_contents;
    version = 1 as libc::c_int as krb5_int16;
    store_16_le(version as libc::c_uint, nextloc as *mut libc::c_void);
    nextloc =
        nextloc.offset(::std::mem::size_of::<krb5_ui_2>() as libc::c_ulong as
                           isize);
    aux_data_entry = mkey_aux_data_list;
    while !aux_data_entry.is_null() {
        tmp_kvno = (*aux_data_entry).mkey_kvno as krb5_int16;
        store_16_le(tmp_kvno as libc::c_uint, nextloc as *mut libc::c_void);
        nextloc =
            nextloc.offset(::std::mem::size_of::<krb5_ui_2>() as libc::c_ulong
                               as isize);
        store_16_le((*aux_data_entry).latest_mkey.key_data_kvno as
                        libc::c_uint, nextloc as *mut libc::c_void);
        nextloc =
            nextloc.offset(::std::mem::size_of::<krb5_ui_2>() as libc::c_ulong
                               as isize);
        store_16_le((*aux_data_entry).latest_mkey.key_data_type[0 as
                                                                    libc::c_int
                                                                    as usize]
                        as libc::c_uint, nextloc as *mut libc::c_void);
        nextloc =
            nextloc.offset(::std::mem::size_of::<krb5_ui_2>() as libc::c_ulong
                               as isize);
        store_16_le((*aux_data_entry).latest_mkey.key_data_length[0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]
                        as libc::c_uint, nextloc as *mut libc::c_void);
        nextloc =
            nextloc.offset(::std::mem::size_of::<krb5_ui_2>() as libc::c_ulong
                               as isize);
        if (*aux_data_entry).latest_mkey.key_data_length[0 as libc::c_int as
                                                             usize] as
               libc::c_int > 0 as libc::c_int {
            memcpy(nextloc as *mut libc::c_void,
                   (*aux_data_entry).latest_mkey.key_data_contents[0 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                       as *const libc::c_void,
                   (*aux_data_entry).latest_mkey.key_data_length[0 as
                                                                     libc::c_int
                                                                     as usize]
                       as libc::c_ulong);
            nextloc =
                nextloc.offset((*aux_data_entry).latest_mkey.key_data_length[0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 usize]
                                   as libc::c_int as isize)
        }
        aux_data_entry = (*aux_data_entry).next
    }
    status = krb5_dbe_update_tl_data(context, entry, &mut tl_data);
    free(tl_data.tl_data_contents as *mut libc::c_void);
    return status;
}
/* return pointer to start of act_kvno data */
/* return pointer to start of act_time data */
#[no_mangle]
#[c2rust::src_loc = "1860:1"]
pub unsafe extern "C" fn krb5_dbe_lookup_actkvno(mut context: krb5_context,
                                                 mut entry:
                                                     *mut krb5_db_entry,
                                                 mut actkvno_list:
                                                     *mut *mut krb5_actkvno_node)
 -> krb5_error_code {
    let mut tl_data: krb5_tl_data =
        krb5_tl_data{tl_data_next: 0 as *mut _krb5_tl_data,
                     tl_data_type: 0,
                     tl_data_length: 0,
                     tl_data_contents: 0 as *mut krb5_octet,};
    let mut code: krb5_error_code = 0;
    let mut version: krb5_int16 = 0;
    let mut tmp_kvno: krb5_int16 = 0;
    let mut head_data: *mut krb5_actkvno_node = 0 as *mut krb5_actkvno_node;
    let mut new_data: *mut krb5_actkvno_node = 0 as *mut krb5_actkvno_node;
    let mut prev_data: *mut krb5_actkvno_node = 0 as *mut krb5_actkvno_node;
    let mut num_actkvno: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut next_tuple: *mut krb5_octet = 0 as *mut krb5_octet;
    let mut earliest_kvno: krb5_kvno = 0;
    memset(&mut tl_data as *mut krb5_tl_data as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_tl_data>() as libc::c_ulong);
    tl_data.tl_data_type = 0x9 as libc::c_int as krb5_int16;
    code = krb5_dbe_lookup_tl_data(context, entry, &mut tl_data);
    if code != 0 { return code }
    if tl_data.tl_data_contents.is_null() {
        /*
         * If there is no KRB5_TL_ACTKVNO data (likely because the KDB was
         * created prior to 1.7), synthesize the list which should have been
         * created at KDB initialization, making the earliest master key
         * active.
         */
        /* Get the earliest master key version. */
        if (*entry).n_key_data as libc::c_int == 0 as libc::c_int {
            return -(1780008432 as libc::c_long) as krb5_error_code
        } /* earliest time possible */
        earliest_kvno =
            (*(*entry).key_data.offset(((*entry).n_key_data as libc::c_int -
                                            1 as libc::c_int) as
                                           isize)).key_data_kvno as krb5_kvno;
        head_data =
            malloc(::std::mem::size_of::<krb5_actkvno_node>() as
                       libc::c_ulong) as *mut krb5_actkvno_node;
        if head_data.is_null() { return 12 as libc::c_int }
        memset(head_data as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<krb5_actkvno_node>() as libc::c_ulong);
        (*head_data).act_time = 0 as libc::c_int;
        (*head_data).act_kvno = earliest_kvno
    } else {
        /* get version to determine how to parse the data */
        *(&mut version as *mut krb5_int16) =
            load_16_le(tl_data.tl_data_contents as *const libc::c_void) as
                krb5_int16;
        if version as libc::c_int == 1 as libc::c_int {
            /* variable size, must be at least 8 bytes */
            if (tl_data.tl_data_length as libc::c_int) < 8 as libc::c_int {
                return -(1780008439 as libc::c_long) as krb5_error_code
            }
            /*
             * Find number of tuple entries, remembering to account for version
             * field.
             */
            num_actkvno =
                (tl_data.tl_data_length as
                     libc::c_ulong).wrapping_sub(::std::mem::size_of::<krb5_int16>()
                                                     as
                                                     libc::c_ulong).wrapping_div((::std::mem::size_of::<krb5_int16>()
                                                                                      as
                                                                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<krb5_int32>()
                                                                                                                      as
                                                                                                                      libc::c_ulong))
                    as libc::c_uint;
            prev_data = 0 as *mut krb5_actkvno_node;
            /* next_tuple points to first tuple entry in the tl_data_contents */
            next_tuple =
                tl_data.tl_data_contents.offset(::std::mem::size_of::<krb5_int16>()
                                                    as libc::c_ulong as
                                                    isize);
            i = 0 as libc::c_int as libc::c_uint;
            while i < num_actkvno {
                new_data =
                    malloc(::std::mem::size_of::<krb5_actkvno_node>() as
                               libc::c_ulong) as *mut krb5_actkvno_node;
                if new_data.is_null() {
                    krb5_dbe_free_actkvno_list(context, head_data);
                    return 12 as libc::c_int
                }
                memset(new_data as *mut libc::c_void, 0 as libc::c_int,
                       ::std::mem::size_of::<krb5_actkvno_node>() as
                           libc::c_ulong);
                /* using tmp_kvno to avoid type mismatch */
                *(&mut tmp_kvno as *mut krb5_int16) =
                    load_16_le(next_tuple as *const libc::c_void) as
                        krb5_int16;
                (*new_data).act_kvno = tmp_kvno as krb5_kvno;
                *(&mut (*new_data).act_time as *mut krb5_timestamp as
                      *mut krb5_int32) =
                    load_32_le(next_tuple.offset(::std::mem::size_of::<krb5_int16>()
                                                     as libc::c_ulong as
                                                     isize) as
                                   *const libc::c_void) as krb5_int32;
                if !prev_data.is_null() {
                    (*prev_data).next = new_data
                } else { head_data = new_data }
                prev_data = new_data;
                next_tuple =
                    next_tuple.offset((::std::mem::size_of::<krb5_int16>() as
                                           libc::c_ulong).wrapping_add(::std::mem::size_of::<krb5_int32>()
                                                                           as
                                                                           libc::c_ulong)
                                          as isize);
                i = i.wrapping_add(1)
            }
        } else {
            krb5_set_error_message(context,
                                   -(1780008422 as libc::c_long) as
                                       krb5_error_code,
                                   dgettext(b"mit-krb5\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"Illegal version number for KRB5_TL_ACTKVNO %d\n\x00"
                                                as *const u8 as
                                                *const libc::c_char),
                                   version as libc::c_int);
            return -(1780008422 as libc::c_long) as krb5_error_code
        }
    }
    *actkvno_list = head_data;
    return 0 as libc::c_int;
}
/*
 * Add KRB5_TL_ACTKVNO TL data entries to krb5_db_entry *entry
 */
#[no_mangle]
#[c2rust::src_loc = "1950:1"]
pub unsafe extern "C" fn krb5_dbe_update_actkvno(mut context: krb5_context,
                                                 mut entry:
                                                     *mut krb5_db_entry,
                                                 mut actkvno_list:
                                                     *const krb5_actkvno_node)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut version: krb5_int16 = 0;
    let mut tmp_kvno: krb5_int16 = 0;
    let mut new_tl_data: krb5_tl_data =
        krb5_tl_data{tl_data_next: 0 as *mut _krb5_tl_data,
                     tl_data_type: 0,
                     tl_data_length: 0,
                     tl_data_contents: 0 as *mut krb5_octet,};
    let mut nextloc: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut cur_actkvno: *const krb5_actkvno_node =
        0 as *const krb5_actkvno_node;
    let mut tmpptr: *mut krb5_octet = 0 as *mut krb5_octet;
    if actkvno_list.is_null() { return 22 as libc::c_int }
    memset(&mut new_tl_data as *mut krb5_tl_data as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_tl_data>() as libc::c_ulong);
    /* allocate initial KRB5_TL_ACTKVNO tl_data entry */
    new_tl_data.tl_data_length =
        ::std::mem::size_of::<krb5_int16>() as libc::c_ulong as krb5_ui_2;
    new_tl_data.tl_data_contents =
        malloc(new_tl_data.tl_data_length as libc::c_ulong) as
            *mut krb5_octet;
    if new_tl_data.tl_data_contents.is_null() { return 12 as libc::c_int }
    /* add the current version # for the data format used for KRB5_TL_ACTKVNO */
    version = 1 as libc::c_int as krb5_int16;
    store_16_le(version as libc::c_uint,
                new_tl_data.tl_data_contents as *mut libc::c_uchar as
                    *mut libc::c_void);
    cur_actkvno = actkvno_list;
    while !cur_actkvno.is_null() {
        new_tl_data.tl_data_length =
            (new_tl_data.tl_data_length as
                 libc::c_ulong).wrapping_add((::std::mem::size_of::<krb5_int16>()
                                                  as
                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<krb5_int32>()
                                                                                  as
                                                                                  libc::c_ulong))
                as krb5_ui_2 as krb5_ui_2;
        tmpptr =
            realloc(new_tl_data.tl_data_contents as *mut libc::c_void,
                    new_tl_data.tl_data_length as libc::c_ulong) as
                *mut krb5_octet;
        if tmpptr.is_null() {
            free(new_tl_data.tl_data_contents as *mut libc::c_void);
            return 12 as libc::c_int
        } else { new_tl_data.tl_data_contents = tmpptr }
        /*
         * Using realloc so tl_data_contents is required to correctly calculate
         * next location to store new tuple.
         */
        nextloc =
            new_tl_data.tl_data_contents.offset(new_tl_data.tl_data_length as
                                                    libc::c_int as
                                                    isize).offset(-((::std::mem::size_of::<krb5_int16>()
                                                                         as
                                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<krb5_int32>()
                                                                                                         as
                                                                                                         libc::c_ulong)
                                                                        as
                                                                        isize));
        /* using tmp_kvno to avoid type mismatch issues */
        tmp_kvno = (*cur_actkvno).act_kvno as krb5_int16;
        store_16_le(tmp_kvno as libc::c_uint, nextloc as *mut libc::c_void);
        nextloc =
            nextloc.offset(::std::mem::size_of::<krb5_ui_2>() as libc::c_ulong
                               as isize);
        store_32_le((*cur_actkvno).act_time as krb5_ui_4,
                    nextloc as *mut libc::c_void);
        cur_actkvno = (*cur_actkvno).next
    }
    new_tl_data.tl_data_type = 0x9 as libc::c_int as krb5_int16;
    retval = krb5_dbe_update_tl_data(context, entry, &mut new_tl_data);
    free(new_tl_data.tl_data_contents as *mut libc::c_void);
    return retval;
}
/* KRB5_TL_ACTKVNO_VER == 1 */
#[no_mangle]
#[c2rust::src_loc = "2007:1"]
pub unsafe extern "C" fn krb5_dbe_update_last_pwd_change(mut context:
                                                             krb5_context,
                                                         mut entry:
                                                             *mut krb5_db_entry,
                                                         mut stamp:
                                                             krb5_timestamp)
 -> krb5_error_code {
    let mut tl_data: krb5_tl_data =
        krb5_tl_data{tl_data_next: 0 as *mut _krb5_tl_data,
                     tl_data_type: 0,
                     tl_data_length: 0,
                     tl_data_contents:
                         0 as
                             *mut krb5_octet,}; /* this is the encoded size of an int32 */
    let mut buf: [krb5_octet; 4] =
        [0; 4]; /* this is the encoded size of an int32 */
    tl_data.tl_data_type = 0x1 as libc::c_int as krb5_int16;
    tl_data.tl_data_length =
        ::std::mem::size_of::<[krb5_octet; 4]>() as libc::c_ulong as
            krb5_ui_2;
    store_32_le(stamp as libc::c_uint, buf.as_mut_ptr() as *mut libc::c_void);
    tl_data.tl_data_contents = buf.as_mut_ptr();
    return krb5_dbe_update_tl_data(context, entry, &mut tl_data);
}
#[no_mangle]
#[c2rust::src_loc = "2022:1"]
pub unsafe extern "C" fn krb5_dbe_update_last_admin_unlock(mut context:
                                                               krb5_context,
                                                           mut entry:
                                                               *mut krb5_db_entry,
                                                           mut stamp:
                                                               krb5_timestamp)
 -> krb5_error_code {
    let mut tl_data: krb5_tl_data =
        krb5_tl_data{tl_data_next: 0 as *mut _krb5_tl_data,
                     tl_data_type: 0,
                     tl_data_length: 0,
                     tl_data_contents: 0 as *mut krb5_octet,};
    let mut buf: [krb5_octet; 4] = [0; 4];
    tl_data.tl_data_type = 0x700 as libc::c_int as krb5_int16;
    tl_data.tl_data_length =
        ::std::mem::size_of::<[krb5_octet; 4]>() as libc::c_ulong as
            krb5_ui_2;
    store_32_le(stamp as libc::c_uint, buf.as_mut_ptr() as *mut libc::c_void);
    tl_data.tl_data_contents = buf.as_mut_ptr();
    return krb5_dbe_update_tl_data(context, entry, &mut tl_data);
}
/*
 * Prepare to iterate over the string attributes of entry.  The returned
 * pointers are aliases into entry's tl_data (or into an empty string literal)
 * and remain valid until the entry's tl_data is changed.
 */
#[c2rust::src_loc = "2042:1"]
unsafe extern "C" fn begin_attrs(mut context: krb5_context,
                                 mut entry: *mut krb5_db_entry,
                                 mut pos_out: *mut *const libc::c_char,
                                 mut end_out: *mut *const libc::c_char)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut tl_data: krb5_tl_data =
        krb5_tl_data{tl_data_next: 0 as *mut _krb5_tl_data,
                     tl_data_type: 0,
                     tl_data_length: 0,
                     tl_data_contents: 0 as *mut krb5_octet,};
    *end_out = 0 as *const libc::c_char;
    *pos_out = *end_out;
    tl_data.tl_data_type = 0xb as libc::c_int as krb5_int16;
    code = krb5_dbe_lookup_tl_data(context, entry, &mut tl_data);
    if code != 0 { return code }
    /* Copy the current mapping to buf, updating key with value if found. */
    *pos_out = tl_data.tl_data_contents as *const libc::c_char;
    *end_out =
        (*pos_out).offset(tl_data.tl_data_length as libc::c_int as isize);
    return 0 as libc::c_int;
}
/* Find the next key and value pair in *pos and update *pos. */
#[c2rust::src_loc = "2062:1"]
unsafe extern "C" fn next_attr(mut pos: *mut *const libc::c_char,
                               mut end: *const libc::c_char,
                               mut key_out: *mut *const libc::c_char,
                               mut val_out: *mut *const libc::c_char)
 -> krb5_boolean {
    let mut key: *const libc::c_char = 0 as *const libc::c_char;
    let mut key_end: *const libc::c_char = 0 as *const libc::c_char;
    let mut val: *const libc::c_char = 0 as *const libc::c_char;
    let mut val_end: *const libc::c_char = 0 as *const libc::c_char;
    *val_out = 0 as *const libc::c_char;
    *key_out = *val_out;
    if *pos == end { return 0 as libc::c_int as krb5_boolean }
    key = *pos;
    key_end =
        memchr(key as *const libc::c_void, '\u{0}' as i32,
               end.wrapping_offset_from(key) as libc::c_long as libc::c_ulong)
            as *const libc::c_char;
    if key_end.is_null() {
        /* Malformed representation; give up. */
        return 0 as libc::c_int as krb5_boolean
    }
    val = key_end.offset(1 as libc::c_int as isize);
    val_end =
        memchr(val as *const libc::c_void, '\u{0}' as i32,
               end.wrapping_offset_from(val) as libc::c_long as libc::c_ulong)
            as *const libc::c_char;
    if val_end.is_null() {
        /* Malformed representation; give up. */
        return 0 as libc::c_int as krb5_boolean
    }
    *key_out = key;
    *val_out = val;
    *pos = val_end.offset(1 as libc::c_int as isize);
    return 1 as libc::c_int as krb5_boolean;
}
/* Retrieve the set of string attributes in entry, in no particular order.
 * Free *strings_out with krb5_dbe_free_strings when done. */
#[no_mangle]
#[c2rust::src_loc = "2086:1"]
pub unsafe extern "C" fn krb5_dbe_get_strings(mut context: krb5_context,
                                              mut entry: *mut krb5_db_entry,
                                              mut strings_out:
                                                  *mut *mut krb5_string_attr,
                                              mut count_out: *mut libc::c_int)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut code: krb5_error_code = 0;
    let mut pos: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut mapkey: *const libc::c_char = 0 as *const libc::c_char;
    let mut mapval: *const libc::c_char = 0 as *const libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut strings: *mut krb5_string_attr = 0 as *mut krb5_string_attr;
    let mut newstrings: *mut krb5_string_attr = 0 as *mut krb5_string_attr;
    let mut count: libc::c_int = 0 as libc::c_int;
    *strings_out = 0 as *mut krb5_string_attr;
    *count_out = 0 as libc::c_int;
    code = begin_attrs(context, entry, &mut pos, &mut end);
    if code != 0 { return code }
    loop  {
        if !(next_attr(&mut pos, end, &mut mapkey, &mut mapval) != 0) {
            current_block = 7149356873433890176;
            break ;
        }
        /* Add a copy of mapkey and mapvalue to strings. */
        newstrings =
            realloc(strings as *mut libc::c_void,
                    ((count + 1 as libc::c_int) as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_string_attr>()
                                                         as libc::c_ulong)) as
                *mut krb5_string_attr;
        if newstrings.is_null() {
            current_block = 13847485488344700075;
            break ;
        }
        strings = newstrings;
        key = strdup(mapkey);
        val = strdup(mapval);
        if key.is_null() || val.is_null() {
            current_block = 13847485488344700075;
            break ;
        }
        let ref mut fresh2 = (*strings.offset(count as isize)).key;
        *fresh2 = key;
        let ref mut fresh3 = (*strings.offset(count as isize)).value;
        *fresh3 = val;
        count += 1
    }
    match current_block {
        13847485488344700075 => {
            free(key as *mut libc::c_void);
            free(val as *mut libc::c_void);
            krb5_dbe_free_strings(context, strings, count);
            return 12 as libc::c_int
        }
        _ => {
            *strings_out = strings;
            *count_out = count;
            return 0 as libc::c_int
        }
    };
}
/* Retrieve a single string attribute from entry, or NULL if there is no
 * attribute for key.  Free *value_out with krb5_dbe_free_string when done. */
#[no_mangle]
#[c2rust::src_loc = "2128:1"]
pub unsafe extern "C" fn krb5_dbe_get_string(mut context: krb5_context,
                                             mut entry: *mut krb5_db_entry,
                                             mut key: *const libc::c_char,
                                             mut value_out:
                                                 *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut pos: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut mapkey: *const libc::c_char = 0 as *const libc::c_char;
    let mut mapval: *const libc::c_char = 0 as *const libc::c_char;
    *value_out = 0 as *mut libc::c_char;
    code = begin_attrs(context, entry, &mut pos, &mut end);
    if code != 0 { return code }
    while next_attr(&mut pos, end, &mut mapkey, &mut mapval) != 0 {
        if strcmp(mapkey, key) == 0 as libc::c_int {
            *value_out = strdup(mapval);
            return if (*value_out).is_null() {
                       12 as libc::c_int
                   } else { 0 as libc::c_int }
        }
    }
    return 0 as libc::c_int;
}
/* Change or add a string attribute in entry, or delete it if value is NULL. */
#[no_mangle]
#[c2rust::src_loc = "2149:1"]
pub unsafe extern "C" fn krb5_dbe_set_string(mut context: krb5_context,
                                             mut entry: *mut krb5_db_entry,
                                             mut key: *const libc::c_char,
                                             mut value: *const libc::c_char)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut pos: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut mapkey: *const libc::c_char = 0 as *const libc::c_char;
    let mut mapval: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: k5buf =
        {
            let mut init =
                k5buf{buftype: K5BUF_ERROR,
                      data: 0 as *mut libc::c_void,
                      space: 0,
                      len: 0,};
            init
        };
    let mut found: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut tl_data: krb5_tl_data =
        krb5_tl_data{tl_data_next: 0 as *mut _krb5_tl_data,
                     tl_data_type: 0,
                     tl_data_length: 0,
                     tl_data_contents: 0 as *mut krb5_octet,};
    /* Copy the current mapping to buf, updating key with value if found. */
    code = begin_attrs(context, entry, &mut pos, &mut end);
    if code != 0 { return code }
    k5_buf_init_dynamic(&mut buf);
    while next_attr(&mut pos, end, &mut mapkey, &mut mapval) != 0 {
        if strcmp(mapkey, key) == 0 as libc::c_int {
            if !value.is_null() {
                k5_buf_add_len(&mut buf, mapkey as *const libc::c_void,
                               strlen(mapkey).wrapping_add(1 as libc::c_int as
                                                               libc::c_ulong));
                k5_buf_add_len(&mut buf, value as *const libc::c_void,
                               strlen(value).wrapping_add(1 as libc::c_int as
                                                              libc::c_ulong));
            }
            found = 1 as libc::c_int as krb5_boolean
        } else {
            k5_buf_add_len(&mut buf, mapkey as *const libc::c_void,
                           strlen(mapkey).wrapping_add(1 as libc::c_int as
                                                           libc::c_ulong));
            k5_buf_add_len(&mut buf, mapval as *const libc::c_void,
                           strlen(mapval).wrapping_add(1 as libc::c_int as
                                                           libc::c_ulong));
        }
    }
    /* If key wasn't found in the map, add a new entry for it. */
    if found == 0 && !value.is_null() {
        k5_buf_add_len(&mut buf, key as *const libc::c_void,
                       strlen(key).wrapping_add(1 as libc::c_int as
                                                    libc::c_ulong));
        k5_buf_add_len(&mut buf, value as *const libc::c_void,
                       strlen(value).wrapping_add(1 as libc::c_int as
                                                      libc::c_ulong));
    }
    if k5_buf_status(&mut buf) != 0 as libc::c_int {
        return 12 as libc::c_int
    }
    if buf.len > 65535 as libc::c_int as libc::c_ulong {
        code = -(1780008403 as libc::c_long) as krb5_error_code
    } else {
        tl_data.tl_data_type = 0xb as libc::c_int as krb5_int16;
        tl_data.tl_data_contents = buf.data as *mut krb5_octet;
        tl_data.tl_data_length = buf.len as krb5_ui_2;
        code = krb5_dbe_update_tl_data(context, entry, &mut tl_data)
    }
    k5_buf_free(&mut buf);
    return code;
}
#[no_mangle]
#[c2rust::src_loc = "2200:1"]
pub unsafe extern "C" fn krb5_dbe_delete_tl_data(mut context: krb5_context,
                                                 mut entry:
                                                     *mut krb5_db_entry,
                                                 mut tl_data_type: krb5_int16)
 -> krb5_error_code {
    let mut tl_data: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    let mut prev_tl_data: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    let mut free_tl_data_0: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    /*
     * Find existing entries of the specified type and remove them from the
     * entry's tl_data list.
     */
    tl_data = (*entry).tl_data;
    prev_tl_data = tl_data;
    while !tl_data.is_null() {
        if (*tl_data).tl_data_type as libc::c_int ==
               tl_data_type as libc::c_int {
            if tl_data == (*entry).tl_data {
                /* remove from head */
                (*entry).tl_data = (*tl_data).tl_data_next;
                prev_tl_data = (*entry).tl_data
            } else if (*tl_data).tl_data_next.is_null() {
                /* remove from tail */
                (*prev_tl_data).tl_data_next = 0 as *mut _krb5_tl_data
            } else {
                /* remove in between */
                (*prev_tl_data).tl_data_next = (*tl_data).tl_data_next
            }
            free_tl_data_0 = tl_data;
            tl_data = (*tl_data).tl_data_next;
            krb5_dbe_free_tl_data(context, free_tl_data_0);
            (*entry).n_tl_data -= 1
        } else { prev_tl_data = tl_data; tl_data = (*tl_data).tl_data_next }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "2237:1"]
pub unsafe extern "C" fn krb5_db_update_tl_data(mut context: krb5_context,
                                                mut n_tl_datap:
                                                    *mut krb5_int16,
                                                mut tl_datap:
                                                    *mut *mut krb5_tl_data,
                                                mut new_tl_data:
                                                    *mut krb5_tl_data)
 -> krb5_error_code {
    let mut tl_data: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    let mut tmp: *mut krb5_octet = 0 as *mut krb5_octet;
    /*
     * Copy the new data first, so we can fail cleanly if malloc()
     * fails.
     */
    tmp =
        malloc((*new_tl_data).tl_data_length as libc::c_ulong) as
            *mut krb5_octet;
    if tmp.is_null() { return 12 as libc::c_int }
    /*
     * Find an existing entry of the specified type and point at
     * it, or NULL if not found.
     */
    if (*new_tl_data).tl_data_type as libc::c_int != 0x7fff as libc::c_int {
        /* db_args can be multiple */
        tl_data = *tl_datap;
        while !tl_data.is_null() {
            if (*tl_data).tl_data_type as libc::c_int ==
                   (*new_tl_data).tl_data_type as libc::c_int {
                break ;
            }
            tl_data = (*tl_data).tl_data_next
        }
    }
    /* If necessary, chain a new record in the beginning and point at it.  */
    if tl_data.is_null() {
        tl_data =
            calloc(1 as libc::c_int as libc::c_ulong,
                   ::std::mem::size_of::<krb5_tl_data>() as libc::c_ulong) as
                *mut krb5_tl_data;
        if tl_data.is_null() {
            free(tmp as *mut libc::c_void);
            return 12 as libc::c_int
        }
        (*tl_data).tl_data_next = *tl_datap;
        *tl_datap = tl_data;
        *n_tl_datap += 1
    }
    /* fill in the record */
    free((*tl_data).tl_data_contents as *mut libc::c_void);
    (*tl_data).tl_data_type = (*new_tl_data).tl_data_type;
    (*tl_data).tl_data_length = (*new_tl_data).tl_data_length;
    (*tl_data).tl_data_contents = tmp;
    memcpy(tmp as *mut libc::c_void,
           (*new_tl_data).tl_data_contents as *const libc::c_void,
           (*tl_data).tl_data_length as libc::c_ulong);
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "2289:1"]
pub unsafe extern "C" fn krb5_dbe_update_tl_data(mut context: krb5_context,
                                                 mut entry:
                                                     *mut krb5_db_entry,
                                                 mut new_tl_data:
                                                     *mut krb5_tl_data)
 -> krb5_error_code {
    return krb5_db_update_tl_data(context, &mut (*entry).n_tl_data,
                                  &mut (*entry).tl_data, new_tl_data);
}
/* Compute the salt for a key data entry given the corresponding principal. */
#[no_mangle]
#[c2rust::src_loc = "2297:1"]
pub unsafe extern "C" fn krb5_dbe_compute_salt(mut context: krb5_context,
                                               mut key: *const krb5_key_data,
                                               mut princ:
                                                   krb5_const_principal,
                                               mut salttype_out:
                                                   *mut krb5_int16,
                                               mut salt_out:
                                                   *mut *mut krb5_data)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut stype: krb5_int16 = 0;
    let mut salt: *mut krb5_data = 0 as *mut krb5_data;
    let mut sdata: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    stype =
        if ((*key).key_data_ver as libc::c_int) < 2 as libc::c_int {
            0 as libc::c_int
        } else {
            (*key).key_data_type[1 as libc::c_int as usize] as libc::c_int
        } as krb5_int16;
    *salttype_out = stype;
    *salt_out = 0 as *mut krb5_data;
    /* Place computed salt into sdata, or directly into salt_out and return. */
    match stype as libc::c_int {
        0 => {
            retval = krb5_principal2salt(context, princ, &mut sdata);
            if retval != 0 { return retval }
        }
        2 => {
            retval = krb5_principal2salt_norealm(context, princ, &mut sdata);
            if retval != 0 { return retval }
        }
        3 => { return krb5_copy_data(context, &(*princ).realm, salt_out) }
        4 => {
            sdata =
                make_data((*key).key_data_contents[1 as libc::c_int as usize]
                              as *mut libc::c_void,
                          (*key).key_data_length[1 as libc::c_int as usize] as
                              libc::c_uint);
            return krb5_copy_data(context, &mut sdata, salt_out)
        }
        _ => { return -(1780008421 as libc::c_long) as krb5_error_code }
    }
    /* Make a container for sdata. */
    salt =
        malloc(::std::mem::size_of::<krb5_data>() as libc::c_ulong) as
            *mut krb5_data;
    if salt.is_null() {
        free(sdata.data as *mut libc::c_void);
        return 12 as libc::c_int
    }
    *salt = sdata;
    *salt_out = salt;
    return 0 as libc::c_int;
}
/*
 * Modify the key data of entry to explicitly store salt values using the
 * KRB5_KDB_SALTTYPE_SPECIAL salt type.
 */
#[no_mangle]
#[c2rust::src_loc = "2343:1"]
pub unsafe extern "C" fn krb5_dbe_specialize_salt(mut context: krb5_context,
                                                  mut entry:
                                                      *mut krb5_db_entry)
 -> krb5_error_code {
    let mut stype: krb5_int16 = 0;
    let mut i: krb5_int16 = 0;
    let mut salt: *mut krb5_data = 0 as *mut krb5_data;
    let mut ret: krb5_error_code = 0;
    if context.is_null() || entry.is_null() { return 22 as libc::c_int }
    /*
     * Store salt values explicitly so that they don't depend on the principal
     * name.
     */
    i = 0 as libc::c_int as krb5_int16;
    while (i as libc::c_int) < (*entry).n_key_data as libc::c_int {
        ret =
            krb5_dbe_compute_salt(context,
                                  &mut *(*entry).key_data.offset(i as isize),
                                  (*entry).princ as krb5_const_principal,
                                  &mut stype, &mut salt);
        if ret != 0 { return ret }
        /* Steal the data pointer from salt and free the container. */
        if (*(*entry).key_data.offset(i as isize)).key_data_ver as libc::c_int
               >= 2 as libc::c_int {
            free((*(*entry).key_data.offset(i as
                                                isize)).key_data_contents[1 as
                                                                              libc::c_int
                                                                              as
                                                                              usize]
                     as *mut libc::c_void);
        }
        (*(*entry).key_data.offset(i as
                                       isize)).key_data_type[1 as libc::c_int
                                                                 as usize] =
            4 as libc::c_int as krb5_int16;
        let ref mut fresh4 =
            (*(*entry).key_data.offset(i as
                                           isize)).key_data_contents[1 as
                                                                         libc::c_int
                                                                         as
                                                                         usize];
        *fresh4 = (*salt).data as *mut uint8_t;
        (*(*entry).key_data.offset(i as
                                       isize)).key_data_length[1 as
                                                                   libc::c_int
                                                                   as usize] =
            (*salt).length as krb5_ui_2;
        (*(*entry).key_data.offset(i as isize)).key_data_ver =
            2 as libc::c_int as krb5_int16;
        free(salt as *mut libc::c_void);
        i += 1
    }
    return 0 as libc::c_int;
}
/* change password functions */
#[no_mangle]
#[c2rust::src_loc = "2377:1"]
pub unsafe extern "C" fn krb5_dbe_cpw(mut kcontext: krb5_context,
                                      mut master_key: *mut krb5_keyblock,
                                      mut ks_tuple: *mut krb5_key_salt_tuple,
                                      mut ks_tuple_count: libc::c_int,
                                      mut passwd: *mut libc::c_char,
                                      mut new_kvno: libc::c_int,
                                      mut keepold: krb5_boolean,
                                      mut db_entry: *mut krb5_db_entry)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    return (*v).change_pwd.expect("non-null function pointer")(kcontext,
                                                               master_key,
                                                               ks_tuple,
                                                               ks_tuple_count,
                                                               passwd,
                                                               new_kvno,
                                                               keepold,
                                                               db_entry);
}
/* policy management functions */
#[no_mangle]
#[c2rust::src_loc = "2393:1"]
pub unsafe extern "C" fn krb5_db_create_policy(mut kcontext: krb5_context,
                                               mut policy: osa_policy_ent_t)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    if (*v).create_policy.is_none() {
        return -(1765328134 as libc::c_long) as krb5_error_code
    }
    status =
        (*v).create_policy.expect("non-null function pointer")(kcontext,
                                                               policy);
    /* iprop does not support policy mods; force full resync. */
    if status == 0 && logging(kcontext) != 0 {
        status = ulog_init_header(kcontext)
    }
    return status;
}
#[no_mangle]
#[c2rust::src_loc = "2412:1"]
pub unsafe extern "C" fn krb5_db_get_policy(mut kcontext: krb5_context,
                                            mut name: *mut libc::c_char,
                                            mut policy: *mut osa_policy_ent_t)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    if (*v).get_policy.is_none() {
        return -(1765328134 as libc::c_long) as krb5_error_code
    }
    return (*v).get_policy.expect("non-null function pointer")(kcontext, name,
                                                               policy);
}
#[no_mangle]
#[c2rust::src_loc = "2426:1"]
pub unsafe extern "C" fn krb5_db_put_policy(mut kcontext: krb5_context,
                                            mut policy: osa_policy_ent_t)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    if (*v).put_policy.is_none() {
        return -(1765328134 as libc::c_long) as krb5_error_code
    }
    status =
        (*v).put_policy.expect("non-null function pointer")(kcontext, policy);
    /* iprop does not support policy mods; force full resync. */
    if status == 0 && logging(kcontext) != 0 {
        status = ulog_init_header(kcontext)
    }
    return status;
}
#[no_mangle]
#[c2rust::src_loc = "2445:1"]
pub unsafe extern "C" fn krb5_db_iter_policy(mut kcontext: krb5_context,
                                             mut match_entry:
                                                 *mut libc::c_char,
                                             mut func:
                                                 osa_adb_iter_policy_func,
                                             mut data: *mut libc::c_void)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    if (*v).iter_policy.is_none() { return 0 as libc::c_int }
    return (*v).iter_policy.expect("non-null function pointer")(kcontext,
                                                                match_entry,
                                                                func, data);
}
#[no_mangle]
#[c2rust::src_loc = "2460:1"]
pub unsafe extern "C" fn krb5_db_delete_policy(mut kcontext: krb5_context,
                                               mut policy: *mut libc::c_char)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    if (*v).delete_policy.is_none() {
        return -(1765328134 as libc::c_long) as krb5_error_code
    }
    status =
        (*v).delete_policy.expect("non-null function pointer")(kcontext,
                                                               policy);
    /* iprop does not support policy mods; force full resync. */
    if status == 0 && logging(kcontext) != 0 {
        status = ulog_init_header(kcontext)
    }
    return status;
}
#[no_mangle]
#[c2rust::src_loc = "2479:1"]
pub unsafe extern "C" fn krb5_db_free_policy(mut kcontext: krb5_context,
                                             mut policy: osa_policy_ent_t) {
    if policy.is_null() { return }
    free((*policy).name as *mut libc::c_void);
    free((*policy).allowed_keysalts as *mut libc::c_void);
    free_tl_data((*policy).tl_data);
    free(policy as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "2490:1"]
pub unsafe extern "C" fn krb5_db_promote(mut kcontext: krb5_context,
                                         mut db_args: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0;
    let mut section: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    if (*v).promote_db.is_none() {
        return -(1765328134 as libc::c_long) as krb5_error_code
    }
    status = get_conf_section(kcontext, &mut section);
    if status != 0 { return status }
    status =
        (*v).promote_db.expect("non-null function pointer")(kcontext, section,
                                                            db_args);
    free(section as *mut libc::c_void);
    return status;
}
#[c2rust::src_loc = "2510:1"]
unsafe extern "C" fn decrypt_iterator(mut kcontext: krb5_context,
                                      mut key_data: *const krb5_key_data,
                                      mut dbkey: *mut krb5_keyblock,
                                      mut keysalt: *mut krb5_keysalt)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    let mut n: *mut krb5_keylist_node =
        (*(*kcontext).dal_handle).master_keylist;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    while !n.is_null() {
        krb5_clear_error_message(kcontext);
        status =
            (*v).decrypt_key_data.expect("non-null function pointer")(kcontext,
                                                                      &mut (*n).keyblock,
                                                                      key_data,
                                                                      dbkey,
                                                                      keysalt);
        if status == 0 as libc::c_int { return 0 as libc::c_int }
        n = (*n).next
    }
    return status;
}
/* *
 * Decrypts the key given in @@a key_data. If @a mkey is specified, that
 * master key is used. If @a mkey is NULL, then all master keys are tried.
 */
#[no_mangle]
#[c2rust::src_loc = "2531:1"]
pub unsafe extern "C" fn krb5_dbe_decrypt_key_data(mut kcontext: krb5_context,
                                                   mut mkey:
                                                       *const krb5_keyblock,
                                                   mut key_data:
                                                       *const krb5_key_data,
                                                   mut dbkey:
                                                       *mut krb5_keyblock,
                                                   mut keysalt:
                                                       *mut krb5_keysalt)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    let mut cur_mkey: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    if !mkey.is_null() || (*(*kcontext).dal_handle).master_keylist.is_null() {
        return (*v).decrypt_key_data.expect("non-null function pointer")(kcontext,
                                                                         mkey,
                                                                         key_data,
                                                                         dbkey,
                                                                         keysalt)
    }
    status = decrypt_iterator(kcontext, key_data, dbkey, keysalt);
    if status == 0 as libc::c_int { return 0 as libc::c_int }
    if !(*(*kcontext).dal_handle).master_keylist.is_null() {
        /* Try reloading master keys. */
        cur_mkey = &mut (*(*(*kcontext).dal_handle).master_keylist).keyblock;
        if krb5_db_fetch_mkey_list(kcontext,
                                   (*(*kcontext).dal_handle).master_princ,
                                   cur_mkey) == 0 as libc::c_int {
            return decrypt_iterator(kcontext, key_data, dbkey, keysalt)
        }
    }
    return status;
}
#[no_mangle]
#[c2rust::src_loc = "2559:1"]
pub unsafe extern "C" fn krb5_dbe_encrypt_key_data(mut kcontext: krb5_context,
                                                   mut mkey:
                                                       *const krb5_keyblock,
                                                   mut dbkey:
                                                       *const krb5_keyblock,
                                                   mut keysalt:
                                                       *const krb5_keysalt,
                                                   mut keyver: libc::c_int,
                                                   mut key_data:
                                                       *mut krb5_key_data)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    return (*v).encrypt_key_data.expect("non-null function pointer")(kcontext,
                                                                     mkey,
                                                                     dbkey,
                                                                     keysalt,
                                                                     keyver,
                                                                     key_data);
}
#[no_mangle]
#[c2rust::src_loc = "2575:1"]
pub unsafe extern "C" fn krb5_db_get_context(mut context: krb5_context,
                                             mut db_context:
                                                 *mut *mut libc::c_void)
 -> krb5_error_code {
    *db_context = (*(*context).dal_handle).db_context;
    if (*db_context).is_null() {
        return -(1780008435 as libc::c_long) as krb5_error_code
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "2584:1"]
pub unsafe extern "C" fn krb5_db_set_context(mut context: krb5_context,
                                             mut db_context:
                                                 *mut libc::c_void)
 -> krb5_error_code {
    (*(*context).dal_handle).db_context = db_context;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "2592:1"]
pub unsafe extern "C" fn krb5_db_sign_authdata(mut kcontext: krb5_context,
                                               mut flags: libc::c_uint,
                                               mut client_princ:
                                                   krb5_const_principal,
                                               mut server_princ:
                                                   krb5_const_principal,
                                               mut client: *mut krb5_db_entry,
                                               mut server: *mut krb5_db_entry,
                                               mut header_server:
                                                   *mut krb5_db_entry,
                                               mut local_tgt:
                                                   *mut krb5_db_entry,
                                               mut client_key:
                                                   *mut krb5_keyblock,
                                               mut server_key:
                                                   *mut krb5_keyblock,
                                               mut header_key:
                                                   *mut krb5_keyblock,
                                               mut local_tgt_key:
                                                   *mut krb5_keyblock,
                                               mut session_key:
                                                   *mut krb5_keyblock,
                                               mut authtime: krb5_timestamp,
                                               mut tgt_auth_data:
                                                   *mut *mut krb5_authdata,
                                               mut ad_info: *mut libc::c_void,
                                               mut auth_indicators:
                                                   *mut *mut *mut krb5_data,
                                               mut signed_auth_data:
                                                   *mut *mut *mut krb5_authdata)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    *signed_auth_data = 0 as *mut *mut krb5_authdata;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    if (*v).sign_authdata.is_none() {
        return -(1765328134 as libc::c_long) as krb5_error_code
    }
    return (*v).sign_authdata.expect("non-null function pointer")(kcontext,
                                                                  flags,
                                                                  client_princ,
                                                                  server_princ,
                                                                  client,
                                                                  server,
                                                                  header_server,
                                                                  local_tgt,
                                                                  client_key,
                                                                  server_key,
                                                                  header_key,
                                                                  local_tgt_key,
                                                                  session_key,
                                                                  authtime,
                                                                  tgt_auth_data,
                                                                  ad_info,
                                                                  auth_indicators,
                                                                  signed_auth_data);
}
#[no_mangle]
#[c2rust::src_loc = "2620:1"]
pub unsafe extern "C" fn krb5_db_check_transited_realms(mut kcontext:
                                                            krb5_context,
                                                        mut tr_contents:
                                                            *const krb5_data,
                                                        mut client_realm:
                                                            *const krb5_data,
                                                        mut server_realm:
                                                            *const krb5_data)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 { return status }
    if (*v).check_transited_realms.is_none() {
        return -(1765328134 as libc::c_long) as krb5_error_code
    }
    return (*v).check_transited_realms.expect("non-null function pointer")(kcontext,
                                                                           tr_contents,
                                                                           client_realm,
                                                                           server_realm);
}
#[no_mangle]
#[c2rust::src_loc = "2638:1"]
pub unsafe extern "C" fn krb5_db_check_policy_as(mut kcontext: krb5_context,
                                                 mut request:
                                                     *mut krb5_kdc_req,
                                                 mut client:
                                                     *mut krb5_db_entry,
                                                 mut server:
                                                     *mut krb5_db_entry,
                                                 mut kdc_time: krb5_timestamp,
                                                 mut status:
                                                     *mut *const libc::c_char,
                                                 mut e_data:
                                                     *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    *status = 0 as *const libc::c_char;
    *e_data = 0 as *mut *mut krb5_pa_data;
    ret = get_vftabl(kcontext, &mut v);
    if ret != 0 { return ret }
    if (*v).check_policy_as.is_none() {
        return -(1765328134 as libc::c_long) as krb5_error_code
    }
    return (*v).check_policy_as.expect("non-null function pointer")(kcontext,
                                                                    request,
                                                                    client,
                                                                    server,
                                                                    kdc_time,
                                                                    status,
                                                                    e_data);
}
#[no_mangle]
#[c2rust::src_loc = "2658:1"]
pub unsafe extern "C" fn krb5_db_check_policy_tgs(mut kcontext: krb5_context,
                                                  mut request:
                                                      *mut krb5_kdc_req,
                                                  mut server:
                                                      *mut krb5_db_entry,
                                                  mut ticket:
                                                      *mut krb5_ticket,
                                                  mut status:
                                                      *mut *const libc::c_char,
                                                  mut e_data:
                                                      *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    *status = 0 as *const libc::c_char;
    *e_data = 0 as *mut *mut krb5_pa_data;
    ret = get_vftabl(kcontext, &mut v);
    if ret != 0 { return ret }
    if (*v).check_policy_tgs.is_none() {
        return -(1765328134 as libc::c_long) as krb5_error_code
    }
    return (*v).check_policy_tgs.expect("non-null function pointer")(kcontext,
                                                                     request,
                                                                     server,
                                                                     ticket,
                                                                     status,
                                                                     e_data);
}
#[no_mangle]
#[c2rust::src_loc = "2677:1"]
pub unsafe extern "C" fn krb5_db_audit_as_req(mut kcontext: krb5_context,
                                              mut request: *mut krb5_kdc_req,
                                              mut local_addr:
                                                  *const krb5_address,
                                              mut remote_addr:
                                                  *const krb5_address,
                                              mut client: *mut krb5_db_entry,
                                              mut server: *mut krb5_db_entry,
                                              mut authtime: krb5_timestamp,
                                              mut error_code:
                                                  krb5_error_code) {
    let mut status: krb5_error_code = 0;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 || (*v).audit_as_req.is_none() { return }
    (*v).audit_as_req.expect("non-null function pointer")(kcontext, request,
                                                          local_addr,
                                                          remote_addr, client,
                                                          server, authtime,
                                                          error_code);
}
#[no_mangle]
#[c2rust::src_loc = "2694:1"]
pub unsafe extern "C" fn krb5_db_refresh_config(mut kcontext: krb5_context) {
    let mut status: krb5_error_code = 0;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    status = get_vftabl(kcontext, &mut v);
    if status != 0 || (*v).refresh_config.is_none() { return }
    (*v).refresh_config.expect("non-null function pointer")(kcontext);
}
#[no_mangle]
#[c2rust::src_loc = "2706:1"]
pub unsafe extern "C" fn krb5_db_check_allowed_to_delegate(mut kcontext:
                                                               krb5_context,
                                                           mut client:
                                                               krb5_const_principal,
                                                           mut server:
                                                               *const krb5_db_entry,
                                                           mut proxy:
                                                               krb5_const_principal)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    ret = get_vftabl(kcontext, &mut v);
    if ret != 0 { return ret }
    if (*v).check_allowed_to_delegate.is_none() {
        return -(1765328134 as libc::c_long) as krb5_error_code
    }
    return (*v).check_allowed_to_delegate.expect("non-null function pointer")(kcontext,
                                                                              client,
                                                                              server,
                                                                              proxy);
}
#[no_mangle]
#[c2rust::src_loc = "2723:1"]
pub unsafe extern "C" fn krb5_db_get_s4u_x509_principal(mut kcontext:
                                                            krb5_context,
                                                        mut client_cert:
                                                            *const krb5_data,
                                                        mut in_princ:
                                                            krb5_const_principal,
                                                        mut flags:
                                                            libc::c_uint,
                                                        mut entry:
                                                            *mut *mut krb5_db_entry)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    ret = get_vftabl(kcontext, &mut v);
    if ret != 0 { return ret }
    if (*v).get_s4u_x509_principal.is_none() {
        return -(1765328134 as libc::c_long) as krb5_error_code
    }
    ret =
        (*v).get_s4u_x509_principal.expect("non-null function pointer")(kcontext,
                                                                        client_cert,
                                                                        in_princ,
                                                                        flags,
                                                                        entry);
    if ret != 0 { return ret }
    /* Sort the keys in the db entry, same as get_principal(). */
    if !(**entry).key_data.is_null() {
        krb5_dbe_sort_key_data((**entry).key_data,
                               (**entry).n_key_data as size_t);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "2749:1"]
pub unsafe extern "C" fn krb5_db_allowed_to_delegate_from(mut kcontext:
                                                              krb5_context,
                                                          mut client:
                                                              krb5_const_principal,
                                                          mut server:
                                                              krb5_const_principal,
                                                          mut server_ad_info:
                                                              *mut libc::c_void,
                                                          mut proxy:
                                                              *const krb5_db_entry)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    ret = get_vftabl(kcontext, &mut v);
    if ret != 0 { return ret }
    if (*v).allowed_to_delegate_from.is_none() {
        return -(1765328134 as libc::c_long) as krb5_error_code
    }
    return (*v).allowed_to_delegate_from.expect("non-null function pointer")(kcontext,
                                                                             client,
                                                                             server,
                                                                             server_ad_info,
                                                                             proxy);
}
#[no_mangle]
#[c2rust::src_loc = "2768:1"]
pub unsafe extern "C" fn krb5_db_get_authdata_info(mut kcontext: krb5_context,
                                                   mut flags: libc::c_uint,
                                                   mut in_authdata:
                                                       *mut *mut krb5_authdata,
                                                   mut client_princ:
                                                       krb5_const_principal,
                                                   mut server_princ:
                                                       krb5_const_principal,
                                                   mut server_key:
                                                       *mut krb5_keyblock,
                                                   mut krbtgt_key:
                                                       *mut krb5_keyblock,
                                                   mut krbtgt:
                                                       *mut krb5_db_entry,
                                                   mut authtime:
                                                       krb5_timestamp,
                                                   mut ad_info_out:
                                                       *mut *mut libc::c_void,
                                                   mut client_out:
                                                       *mut krb5_principal)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    *ad_info_out = 0 as *mut libc::c_void;
    if !client_out.is_null() { *client_out = 0 as krb5_principal }
    ret = get_vftabl(kcontext, &mut v);
    if ret != 0 { return ret }
    if (*v).get_authdata_info.is_none() {
        return -(1765328134 as libc::c_long) as krb5_error_code
    }
    return (*v).get_authdata_info.expect("non-null function pointer")(kcontext,
                                                                      flags,
                                                                      in_authdata,
                                                                      client_princ,
                                                                      server_princ,
                                                                      server_key,
                                                                      krbtgt_key,
                                                                      krbtgt,
                                                                      authtime,
                                                                      ad_info_out,
                                                                      client_out);
}
#[no_mangle]
#[c2rust::src_loc = "2794:1"]
pub unsafe extern "C" fn krb5_db_free_authdata_info(mut kcontext:
                                                        krb5_context,
                                                    mut ad_info:
                                                        *mut libc::c_void) {
    let mut ret: krb5_error_code = 0;
    let mut v: *mut kdb_vftabl = 0 as *mut kdb_vftabl;
    if ad_info.is_null() { return }
    ret = get_vftabl(kcontext, &mut v);
    if ret != 0 { return }
    if (*v).free_authdata_info.is_none() { return }
    (*v).free_authdata_info.expect("non-null function pointer")(kcontext,
                                                                ad_info);
}
/* *
 * Sort an array of @a krb5_key_data keys in descending order by their kvno.
 * Key data order within a kvno is preserved.
 *
 * @param key_data
 *     The @a krb5_key_data array to sort.  This is sorted in place so the
 *     array will be modified.
 * @param key_data_length
 *     The length of @a key_data.
 */
#[no_mangle]
#[c2rust::src_loc = "2810:1"]
pub unsafe extern "C" fn krb5_dbe_sort_key_data(mut key_data:
                                                    *mut krb5_key_data,
                                                mut key_data_length: size_t) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut tmp: krb5_key_data =
        krb5_key_data{key_data_ver: 0,
                      key_data_kvno: 0,
                      key_data_type: [0; 2],
                      key_data_length: [0; 2],
                      key_data_contents: [0 as *mut krb5_octet; 2],};
    /* Use insertion sort as a stable sort. */
    i = 1 as libc::c_int as size_t;
    while i < key_data_length {
        j = i;
        while j > 0 as libc::c_int as libc::c_ulong &&
                  ((*key_data.offset(j.wrapping_sub(1 as libc::c_int as
                                                        libc::c_ulong) as
                                         isize)).key_data_kvno as libc::c_int)
                      <
                      (*key_data.offset(j as isize)).key_data_kvno as
                          libc::c_int {
            tmp = *key_data.offset(j as isize);
            *key_data.offset(j as isize) =
                *key_data.offset(j.wrapping_sub(1 as libc::c_int as
                                                    libc::c_ulong) as isize);
            *key_data.offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                 as isize) = tmp;
            j = j.wrapping_sub(1)
        }
        i = i.wrapping_add(1)
    };
}
