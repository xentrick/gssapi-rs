use ::libc;
use ::c2rust_asm_casts;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:48"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:48"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "162:1"]
    pub type __suseconds_t = libc::c_long;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:48"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:48"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/stdio.h:48"]
pub mod stdio_h {
    #[c2rust::src_loc = "77:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "218:1"]
        pub fn fflush(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "626:1"]
        pub fn fputs(__s: *const libc::c_char, __stream: *mut FILE)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "775:1"]
        pub fn perror(__s: *const libc::c_char);
    }
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timeval.h:53"]
pub mod struct_timeval_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8:8"]
    pub struct timeval {
        pub tv_sec: __time_t,
        pub tv_usec: __suseconds_t,
    }
    use super::types_h::{__time_t, __suseconds_t};
}
#[c2rust::header_src = "/usr/include/sys/select.h:53"]
pub mod select_h {
    #[c2rust::src_loc = "49:1"]
    pub type __fd_mask = libc::c_long;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "59:9"]
    pub struct fd_set {
        pub fds_bits: [__fd_mask; 16],
    }
    use super::struct_timeval_h::timeval;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "101:1"]
        pub fn select(__nfds: libc::c_int, __readfds: *mut fd_set,
                      __writefds: *mut fd_set, __exceptfds: *mut fd_set,
                      __timeout: *mut timeval) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:54"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:71"]
pub mod gssapi_h {
    /*
 * The following type must be defined as the smallest natural unsigned integer
 * supported by the platform that has at least 32 bits of precision.
 */
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    /* OM_STRING */
    /*
 * We can't use X/Open definitions, so roll our own.
 */
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "106:16"]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_desc = gss_buffer_desc_struct;
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        /* qop_state */
        #[no_mangle]
        #[c2rust::src_loc = "530:1"]
        pub fn gss_display_status(_: *mut OM_uint32, _: OM_uint32,
                                  _: libc::c_int, _: gss_OID,
                                  _: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
        /* input_name */
        #[no_mangle]
        #[c2rust::src_loc = "574:1"]
        pub fn gss_release_buffer(_: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:74"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "574:5"]
    pub struct C2RustUnnamed {
        pub i: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "620:12"]
    pub struct C2RustUnnamed_0 {
        pub i: uint32_t,
    }
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
    /* Do it in macro form so we get the file/line of the invocation if
   the assertion fails.  */
    /* forward declaration for use in initializer */
    /* so ';' following macro use won't get error */
    /* This should be called in finalization only, so we shouldn't have
   multiple active threads mucking around in our library at this
   point.  So ignore the once_t object and just look at the flag.

   XXX Could we have problems with memory coherence between processors
   if we don't invoke mutex/once routines?  Probably not, the
   application code should already be coordinating things such that
   the library code is not in use by this point, and memory
   synchronization will be needed there.  */
    /*
 * In this case, we just don't care about finalization.  The code will still
 * define the function, but we won't do anything with it.
 */
    /* Read and write integer values as (unaligned) octet strings in
   specific byte orders.  Add per-platform optimizations as
   needed.  */
    /* Check for BIG/LITTLE_ENDIAN macros.  If exactly one is defined, use
   it.  If both are defined, then BYTE_ORDER should be defined and
   match one of them.  Try those symbols, then try again with an
   underscore prefix.  */
    /* Optimize for GCC on platforms with known byte orders.

   GCC's packed structures can be written to with any alignment; the
   compiler will use byte operations, unaligned-word operations, or
   normal memory ops as appropriate for the architecture.

   This assumes the availability of uint##_t types, which should work
   on most of our platforms except Windows, where we're not using
   GCC.  */
    /* To do: Define SWAP16, SWAP32, SWAP64 macros to byte-swap values
   with the indicated numbers of bits.

   Linux: byteswap.h, bswap_16 etc.
   Solaris 10: none
   macOS: machine/endian.h or byte_order.h, NXSwap{Short,Int,LongLong}
   NetBSD: sys/bswap.h, bswap16 etc.  */
    /* Note that on Windows at least this file can be included from C++
   source, so casts *from* void* are required.  */
    #[inline]
    #[c2rust::src_loc = "567:1"]
    pub unsafe extern "C" fn store_32_be(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed)).i = __bswap_32(val);
    }
    #[inline]
    #[c2rust::src_loc = "613:1"]
    pub unsafe extern "C" fn load_32_be(mut cvp: *const libc::c_void)
     -> libc::c_uint {
        let mut p: *const libc::c_uchar = cvp as *const libc::c_uchar;
        return __bswap_32((*(p as *const C2RustUnnamed_0)).i);
    }
    use super::stdint_uintn_h::uint32_t;
    use super::byteswap_h::__bswap_32;
    /* K5_PLATFORM_H */
}
#[c2rust::header_src = "/usr/include/bits/byteswap.h:53"]
pub mod byteswap_h {
    #[inline]
    #[c2rust::src_loc = "48:1"]
    pub unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
        return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int |
                   (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int |
                   (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int |
                   (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
    }
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/usr/include/sys/socket.h:54"]
pub mod socket_h {
    use super::stddef_h::size_t;
    use super::stdio_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "138:1"]
        pub fn send(__fd: libc::c_int, __buf: *const libc::c_void,
                    __n: size_t, __flags: libc::c_int) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "145:1"]
        pub fn recv(__fd: libc::c_int, __buf: *mut libc::c_void, __n: size_t,
                    __flags: libc::c_int) -> ssize_t;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:57"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:74"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
    }
}
use c2rust_asm_casts::AsmCastTrait;
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint32_t, __off_t, __off64_t, __time_t,
                        __suseconds_t, __ssize_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stdio_h::{ssize_t, stderr, fflush, fprintf, fputs, perror};
pub use self::struct_timeval_h::timeval;
pub use self::select_h::{__fd_mask, fd_set, select};
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssapi_h::{gss_uint32, OM_uint32, gss_OID_desc_struct, gss_OID,
                         gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_display_status,
                         gss_release_buffer};
pub use self::k5_platform_h::{C2RustUnnamed, C2RustUnnamed_0, store_32_be,
                              load_32_be};
pub use self::byteswap_h::__bswap_32;
use self::socket_h::{send, recv};
use self::errno_h::__errno_location;
use self::stdlib_h::{free, malloc, abort};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1994 by OpenVision Technologies, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
/*
 * Copyright (C) 2003, 2004 by the Massachusetts Institute of Technology.
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
/* need struct timeval */
/* for store_32_be */
#[no_mangle]
#[c2rust::src_loc = "82:7"]
pub static mut display_file: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
#[c2rust::src_loc = "84:17"]
pub static mut empty_token_buf: gss_buffer_desc =
    {
        let mut init =
            gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                   value:
                                       b"\x00" as *const u8 as
                                           *const libc::c_char as
                                           *mut libc::c_void,};
        init
    };
#[no_mangle]
#[c2rust::src_loc = "85:14"]
pub static mut empty_token: gss_buffer_t =
    unsafe {
        &empty_token_buf as *const gss_buffer_desc as *mut gss_buffer_desc
    };
#[c2rust::src_loc = "89:1"]
unsafe extern "C" fn write_all(mut fildes: libc::c_int,
                               mut buf: *mut libc::c_char,
                               mut nbyte: libc::c_uint) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    ptr = buf;
    while nbyte != 0 {
        ret =
            send(fildes, ptr as *const libc::c_void, nbyte as size_t,
                 0 as libc::c_int) as libc::c_int;
        if ret < 0 as libc::c_int {
            if !(*__errno_location() == 4 as libc::c_int) { return ret }
        } else if ret == 0 as libc::c_int {
            return ptr.wrapping_offset_from(buf) as libc::c_long as
                       libc::c_int
        }
        ptr = ptr.offset(ret as isize);
        nbyte = nbyte.wrapping_sub(ret as libc::c_uint)
    }
    return ptr.wrapping_offset_from(buf) as libc::c_long as libc::c_int;
}
#[c2rust::src_loc = "109:1"]
unsafe extern "C" fn read_all(mut fildes: libc::c_int,
                              mut buf: *mut libc::c_char,
                              mut nbyte: libc::c_uint) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rfds: fd_set = fd_set{fds_bits: [0; 16],};
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh0 = &mut __d0;
    let fresh1;
    let fresh2 = &mut __d1;
    let fresh3;
    let fresh4 =
        (::std::mem::size_of::<fd_set>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<__fd_mask>() as
                                             libc::c_ulong);
    let fresh5 =
        &mut *rfds.fds_bits.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut __fd_mask;
    asm!("cld; rep; stosq" : "={cx}" (fresh1), "={di}" (fresh3) : "{ax}"
         (0 as libc::c_int), "0"
         (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh4)), "1"
         (c2rust_asm_casts::AsmCast::cast_in(fresh2, fresh5)) : "memory" :
         "volatile");
    c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh4, fresh1);
    c2rust_asm_casts::AsmCast::cast_out(fresh2, fresh5, fresh3);
    rfds.fds_bits[(fildes /
                       (8 as libc::c_int *
                            ::std::mem::size_of::<__fd_mask>() as
                                libc::c_ulong as libc::c_int)) as usize] |=
        ((1 as libc::c_ulong) <<
             fildes %
                 (8 as libc::c_int *
                      ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as
                          libc::c_int)) as __fd_mask;
    tv.tv_sec = 10 as libc::c_int as __time_t;
    tv.tv_usec = 0 as libc::c_int as __suseconds_t;
    ptr = buf;
    while nbyte != 0 {
        if select(1024 as libc::c_int, &mut rfds, 0 as *mut fd_set,
                  0 as *mut fd_set, &mut tv) <= 0 as libc::c_int ||
               !(rfds.fds_bits[(fildes /
                                    (8 as libc::c_int *
                                         ::std::mem::size_of::<__fd_mask>() as
                                             libc::c_ulong as libc::c_int)) as
                                   usize] &
                     ((1 as libc::c_ulong) <<
                          fildes %
                              (8 as libc::c_int *
                                   ::std::mem::size_of::<__fd_mask>() as
                                       libc::c_ulong as libc::c_int)) as
                         __fd_mask != 0 as libc::c_int as libc::c_long) {
            return ptr.wrapping_offset_from(buf) as libc::c_long as
                       libc::c_int
        }
        ret =
            recv(fildes, ptr as *mut libc::c_void, nbyte as size_t,
                 0 as libc::c_int) as libc::c_int;
        if ret < 0 as libc::c_int {
            if !(*__errno_location() == 4 as libc::c_int) { return ret }
        } else if ret == 0 as libc::c_int {
            return ptr.wrapping_offset_from(buf) as libc::c_long as
                       libc::c_int
        }
        ptr = ptr.offset(ret as isize);
        nbyte = nbyte.wrapping_sub(ret as libc::c_uint)
    }
    return ptr.wrapping_offset_from(buf) as libc::c_long as libc::c_int;
}
/*
 * Copyright 1994 by OpenVision Technologies, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
/*
 * Function: send_token
 *
 * Purpose: Writes a token to a file descriptor.
 *
 * Arguments:
 *
 *      s               (r) an open file descriptor
 *      flags           (r) the flags to write
 *      tok             (r) the token to write
 *
 * Returns: 0 on success, -1 on failure
 *
 * Effects:
 *
 * If the flags are non-null, send_token writes the token flags (a
 * single byte, even though they're passed in in an integer). Next,
 * the token length (as a network long) and then the token data are
 * written to the file descriptor s.  It returns 0 on success, and -1
 * if an error occurs or if it could not write all the data.
 */
#[no_mangle]
#[c2rust::src_loc = "160:1"]
pub unsafe extern "C" fn send_token(mut s: libc::c_int,
                                    mut flags: libc::c_int,
                                    mut tok: gss_buffer_t) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut char_flags: libc::c_uchar = flags as libc::c_uchar;
    let mut lenbuf: [libc::c_uchar; 4] = [0; 4];
    if char_flags != 0 {
        ret =
            write_all(s,
                      &mut char_flags as *mut libc::c_uchar as
                          *mut libc::c_char,
                      1 as libc::c_int as libc::c_uint);
        if ret != 1 as libc::c_int {
            perror(b"sending token flags\x00" as *const u8 as
                       *const libc::c_char);
            return -(1 as libc::c_int)
        }
    }
    if (*tok).length > 0xffffffff as libc::c_ulong { abort(); }
    store_32_be((*tok).length as libc::c_uint,
                lenbuf.as_mut_ptr() as *mut libc::c_void);
    ret =
        write_all(s, lenbuf.as_mut_ptr() as *mut libc::c_char,
                  4 as libc::c_int as libc::c_uint);
    if ret < 0 as libc::c_int {
        perror(b"sending token length\x00" as *const u8 as
                   *const libc::c_char);
        return -(1 as libc::c_int)
    } else {
        if ret != 4 as libc::c_int {
            if !display_file.is_null() {
                fprintf(display_file,
                        b"sending token length: %d of %d bytes written\n\x00"
                            as *const u8 as *const libc::c_char, ret,
                        4 as libc::c_int);
            }
            return -(1 as libc::c_int)
        }
    }
    ret =
        write_all(s, (*tok).value as *mut libc::c_char,
                  (*tok).length as libc::c_uint);
    if ret < 0 as libc::c_int {
        perror(b"sending token data\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    } else {
        if ret as size_t != (*tok).length {
            if !display_file.is_null() {
                fprintf(display_file,
                        b"sending token data: %d of %d bytes written\n\x00" as
                            *const u8 as *const libc::c_char, ret,
                        (*tok).length as libc::c_int);
            }
            return -(1 as libc::c_int)
        }
    }
    return 0 as libc::c_int;
}
/*
 * Function: recv_token
 *
 * Purpose: Reads a token from a file descriptor.
 *
 * Arguments:
 *
 *      s               (r) an open file descriptor
 *      flags           (w) the read flags
 *      tok             (w) the read token
 *
 * Returns: 0 on success, -1 on failure
 *
 * Effects:
 *
 * recv_token reads the token flags (a single byte, even though
 * they're stored into an integer, then reads the token length (as a
 * network long), allocates memory to hold the data, and then reads
 * the token data from the file descriptor s.  It blocks to read the
 * length and data, if necessary.  On a successful return, the token
 * should be freed with gss_release_buffer.  It returns 0 on success,
 * and -1 if an error occurs or if it could not read all the data.
 */
#[no_mangle]
#[c2rust::src_loc = "228:1"]
pub unsafe extern "C" fn recv_token(mut s: libc::c_int,
                                    mut flags: *mut libc::c_int,
                                    mut tok: gss_buffer_t) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut char_flags: libc::c_uchar = 0;
    let mut lenbuf: [libc::c_uchar; 4] = [0; 4];
    ret =
        read_all(s,
                 &mut char_flags as *mut libc::c_uchar as *mut libc::c_char,
                 1 as libc::c_int as libc::c_uint);
    if ret < 0 as libc::c_int {
        perror(b"reading token flags\x00" as *const u8 as
                   *const libc::c_char);
        return -(1 as libc::c_int)
    } else {
        if ret == 0 {
            if !display_file.is_null() {
                fputs(b"reading token flags: 0 bytes read\n\x00" as *const u8
                          as *const libc::c_char, display_file);
            }
            return -(1 as libc::c_int)
        } else { *flags = char_flags as libc::c_int }
    }
    if char_flags as libc::c_int == 0 as libc::c_int {
        lenbuf[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
        ret =
            read_all(s,
                     &mut *lenbuf.as_mut_ptr().offset(1 as libc::c_int as
                                                          isize) as
                         *mut libc::c_uchar as *mut libc::c_char,
                     3 as libc::c_int as libc::c_uint);
        if ret < 0 as libc::c_int {
            perror(b"reading token length\x00" as *const u8 as
                       *const libc::c_char);
            return -(1 as libc::c_int)
        } else {
            if ret != 3 as libc::c_int {
                if !display_file.is_null() {
                    fprintf(display_file,
                            b"reading token length: %d of %d bytes read\n\x00"
                                as *const u8 as *const libc::c_char, ret,
                            3 as libc::c_int);
                }
                return -(1 as libc::c_int)
            }
        }
    } else {
        ret =
            read_all(s, lenbuf.as_mut_ptr() as *mut libc::c_char,
                     4 as libc::c_int as libc::c_uint);
        if ret < 0 as libc::c_int {
            perror(b"reading token length\x00" as *const u8 as
                       *const libc::c_char);
            return -(1 as libc::c_int)
        } else {
            if ret != 4 as libc::c_int {
                if !display_file.is_null() {
                    fprintf(display_file,
                            b"reading token length: %d of %d bytes read\n\x00"
                                as *const u8 as *const libc::c_char, ret,
                            4 as libc::c_int);
                }
                return -(1 as libc::c_int)
            }
        }
    }
    (*tok).length =
        load_32_be(lenbuf.as_mut_ptr() as *const libc::c_void) as size_t;
    (*tok).value =
        malloc(if (*tok).length != 0 {
                   (*tok).length
               } else { 1 as libc::c_int as libc::c_ulong });
    if (*tok).length != 0 && (*tok).value.is_null() {
        if !display_file.is_null() {
            fprintf(display_file,
                    b"Out of memory allocating token data\n\x00" as *const u8
                        as *const libc::c_char);
        }
        return -(1 as libc::c_int)
    }
    ret =
        read_all(s, (*tok).value as *mut libc::c_char,
                 (*tok).length as libc::c_uint);
    if ret < 0 as libc::c_int {
        perror(b"reading token data\x00" as *const u8 as *const libc::c_char);
        free((*tok).value);
        return -(1 as libc::c_int)
    } else {
        if ret as size_t != (*tok).length {
            fprintf(stderr,
                    b"sending token data: %d of %d bytes written\n\x00" as
                        *const u8 as *const libc::c_char, ret,
                    (*tok).length as libc::c_int);
            free((*tok).value);
            return -(1 as libc::c_int)
        }
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "297:1"]
unsafe extern "C" fn display_status_1(mut m: *mut libc::c_char,
                                      mut code: OM_uint32,
                                      mut type_0: libc::c_int) {
    let mut min_stat: OM_uint32 = 0;
    let mut msg: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut msg_ctx: OM_uint32 = 0;
    msg_ctx = 0 as libc::c_int as OM_uint32;
    loop  {
        gss_display_status(&mut min_stat, code, type_0, 0 as gss_OID,
                           &mut msg_ctx, &mut msg);
        if !display_file.is_null() {
            fprintf(display_file,
                    b"GSS-API error %s: %s\n\x00" as *const u8 as
                        *const libc::c_char, m,
                    msg.value as *mut libc::c_char);
        }
        gss_release_buffer(&mut min_stat, &mut msg);
        if msg_ctx == 0 { break ; }
    };
}
/*
 * Function: display_status
 *
 * Purpose: displays GSS-API messages
 *
 * Arguments:
 *
 *      msg             a string to be displayed with the message
 *      maj_stat        the GSS-API major status code
 *      min_stat        the GSS-API minor status code
 *
 * Effects:
 *
 * The GSS-API messages associated with maj_stat and min_stat are
 * displayed on stderr, each preceeded by "GSS-API error <msg>: " and
 * followed by a newline.
 */
#[no_mangle]
#[c2rust::src_loc = "336:1"]
pub unsafe extern "C" fn display_status(mut msg: *mut libc::c_char,
                                        mut maj_stat: OM_uint32,
                                        mut min_stat: OM_uint32) {
    display_status_1(msg, maj_stat, 1 as libc::c_int);
    display_status_1(msg, min_stat, 2 as libc::c_int);
}
/*
 * Function: display_ctx_flags
 *
 * Purpose: displays the flags returned by context initation in
 *          a human-readable form
 *
 * Arguments:
 *
 *      int             ret_flags
 *
 * Effects:
 *
 * Strings corresponding to the context flags are printed on
 * stdout, preceded by "context flag: " and followed by a newline
 */
#[no_mangle]
#[c2rust::src_loc = "359:1"]
pub unsafe extern "C" fn display_ctx_flags(mut flags: OM_uint32) {
    if flags & 1 as libc::c_int as libc::c_uint != 0 {
        fprintf(display_file,
                b"context flag: GSS_C_DELEG_FLAG\n\x00" as *const u8 as
                    *const libc::c_char);
    }
    if flags & 2 as libc::c_int as libc::c_uint != 0 {
        fprintf(display_file,
                b"context flag: GSS_C_MUTUAL_FLAG\n\x00" as *const u8 as
                    *const libc::c_char);
    }
    if flags & 4 as libc::c_int as libc::c_uint != 0 {
        fprintf(display_file,
                b"context flag: GSS_C_REPLAY_FLAG\n\x00" as *const u8 as
                    *const libc::c_char);
    }
    if flags & 8 as libc::c_int as libc::c_uint != 0 {
        fprintf(display_file,
                b"context flag: GSS_C_SEQUENCE_FLAG\n\x00" as *const u8 as
                    *const libc::c_char);
    }
    if flags & 16 as libc::c_int as libc::c_uint != 0 {
        fprintf(display_file,
                b"context flag: GSS_C_CONF_FLAG \n\x00" as *const u8 as
                    *const libc::c_char);
    }
    if flags & 32 as libc::c_int as libc::c_uint != 0 {
        fprintf(display_file,
                b"context flag: GSS_C_INTEG_FLAG \n\x00" as *const u8 as
                    *const libc::c_char);
    };
}
#[no_mangle]
#[c2rust::src_loc = "376:1"]
pub unsafe extern "C" fn print_token(mut tok: gss_buffer_t) {
    let mut i: size_t = 0;
    let mut p: *mut libc::c_uchar = (*tok).value as *mut libc::c_uchar;
    if display_file.is_null() { return }
    i = 0 as libc::c_int as size_t;
    while i < (*tok).length {
        fprintf(display_file,
                b"%02x \x00" as *const u8 as *const libc::c_char,
                *p as libc::c_int);
        if i.wrapping_rem(16 as libc::c_int as libc::c_ulong) ==
               15 as libc::c_int as libc::c_ulong {
            fprintf(display_file,
                    b"\n\x00" as *const u8 as *const libc::c_char);
        }
        i = i.wrapping_add(1);
        p = p.offset(1)
    }
    fprintf(display_file, b"\n\x00" as *const u8 as *const libc::c_char);
    fflush(display_file);
}
/* _WIN32 */
