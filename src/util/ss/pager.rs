use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:9"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:9"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "154:1"]
    pub type __pid_t = libc::c_int;
    #[c2rust::src_loc = "156:1"]
    pub type __clock_t = libc::c_long;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/sys/types.h:9"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
}
#[c2rust::header_src = "/usr/include/bits/types/__sigset_t.h:9"]
pub mod __sigset_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "5:9"]
    pub struct __sigset_t {
        pub __val: [libc::c_ulong; 16],
    }
}
#[c2rust::header_src = "/usr/include/bits/types/sigset_t.h:9"]
pub mod sigset_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type sigset_t = __sigset_t;
    use super::__sigset_t_h::__sigset_t;
}
#[c2rust::header_src = "/usr/include/bits/types/__sigval_t.h:15"]
pub mod __sigval_t_h {
    #[c2rust::src_loc = "30:1"]
    pub type __sigval_t = sigval;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:7"]
    pub union sigval {
        pub sival_int: libc::c_int,
        pub sival_ptr: *mut libc::c_void,
    }
}
#[c2rust::header_src = "/usr/include/bits/sigaction.h:15"]
pub mod sigaction_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:8"]
    pub struct sigaction {
        pub __sigaction_handler: C2RustUnnamed,
        pub sa_mask: __sigset_t,
        pub sa_flags: libc::c_int,
        pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:5"]
    pub union C2RustUnnamed {
        pub sa_handler: __sighandler_t,
        pub sa_sigaction: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *mut siginfo_t,
                                                      _: *mut libc::c_void)
                                     -> ()>,
    }
    use super::__sigset_t_h::__sigset_t;
    use super::signal_h::__sighandler_t;
    use super::siginfo_t_h::siginfo_t;
}
#[c2rust::header_src = "/usr/include/bits/types/siginfo_t.h:15"]
pub mod siginfo_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:9"]
    pub struct siginfo_t {
        pub si_signo: libc::c_int,
        pub si_errno: libc::c_int,
        pub si_code: libc::c_int,
        pub __pad0: libc::c_int,
        pub _sifields: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "51:5"]
    pub union C2RustUnnamed_0 {
        pub _pad: [libc::c_int; 28],
        pub _kill: C2RustUnnamed_9,
        pub _timer: C2RustUnnamed_8,
        pub _rt: C2RustUnnamed_7,
        pub _sigchld: C2RustUnnamed_6,
        pub _sigfault: C2RustUnnamed_3,
        pub _sigpoll: C2RustUnnamed_2,
        pub _sigsys: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "116:2"]
    pub struct C2RustUnnamed_1 {
        pub _call_addr: *mut libc::c_void,
        pub _syscall: libc::c_int,
        pub _arch: libc::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "108:2"]
    pub struct C2RustUnnamed_2 {
        pub si_band: libc::c_long,
        pub si_fd: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "89:2"]
    pub struct C2RustUnnamed_3 {
        pub si_addr: *mut libc::c_void,
        pub si_addr_lsb: libc::c_short,
        pub _bounds: C2RustUnnamed_4,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "94:6"]
    pub union C2RustUnnamed_4 {
        pub _addr_bnd: C2RustUnnamed_5,
        pub _pkey: __uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "97:3"]
    pub struct C2RustUnnamed_5 {
        pub _lower: *mut libc::c_void,
        pub _upper: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "79:2"]
    pub struct C2RustUnnamed_6 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
        pub si_status: libc::c_int,
        pub si_utime: __clock_t,
        pub si_stime: __clock_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "71:2"]
    pub struct C2RustUnnamed_7 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
        pub si_sigval: __sigval_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:2"]
    pub struct C2RustUnnamed_8 {
        pub si_tid: libc::c_int,
        pub si_overrun: libc::c_int,
        pub si_sigval: __sigval_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "56:2"]
    pub struct C2RustUnnamed_9 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
    }
    use super::types_h::{__uint32_t, __pid_t, __uid_t, __clock_t};
    use super::__sigval_t_h::__sigval_t;
}
#[c2rust::header_src = "/usr/include/signal.h:15"]
pub mod signal_h {
    #[c2rust::src_loc = "72:1"]
    pub type __sighandler_t
        =
        Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
    use super::sigset_t_h::sigset_t;
    use super::sigaction_h::sigaction;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "202:1"]
        pub fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "196:1"]
        pub fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "229:1"]
        pub fn sigprocmask(__how: libc::c_int, __set: *const sigset_t,
                           __oset: *mut sigset_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "240:1"]
        pub fn sigaction(__sig: libc::c_int, __act: *const sigaction,
                         __oact: *mut sigaction) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:9"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
        #[no_mangle]
        #[c2rust::src_loc = "634:1"]
        pub fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:9"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "175:1"]
        pub fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:9"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:9"]
pub mod unistd_h {
    use super::stddef_h::size_t;
    use super::sys_types_h::ssize_t;
    use super::types_h::__pid_t;
    extern "C" {
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
        #[c2rust::src_loc = "417:1"]
        pub fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "534:1"]
        pub fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "584:1"]
        pub fn execlp(__file: *const libc::c_char, __arg: *const libc::c_char,
                      _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "756:1"]
        pub fn fork() -> __pid_t;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint32_t, __uid_t, __pid_t, __clock_t, __ssize_t};
pub use self::sys_types_h::ssize_t;
pub use self::__sigset_t_h::__sigset_t;
pub use self::sigset_t_h::sigset_t;
pub use self::__sigval_t_h::{__sigval_t, sigval};
pub use self::sigaction_h::{sigaction, C2RustUnnamed};
pub use self::siginfo_t_h::{siginfo_t, C2RustUnnamed_0, C2RustUnnamed_1,
                            C2RustUnnamed_2, C2RustUnnamed_3, C2RustUnnamed_4,
                            C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7,
                            C2RustUnnamed_8, C2RustUnnamed_9};
pub use self::signal_h::{__sighandler_t, sigaddset, sigemptyset, sigprocmask,
                         sigaction};
use self::stdlib_h::{exit, getenv};
use self::fcntl_h::fcntl;
use self::errno_h::__errno_location;
use self::unistd_h::{close, read, write, pipe, dup2, execlp, fork};
extern "C" {
    #[no_mangle]
    #[c2rust::src_loc = "18:14"]
    pub static mut _ss_pager_name: *mut libc::c_char;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* util/ss/pager.c - create a "more" running out of a file descriptor */
/*
 * Copyright 1987, 1988 by MIT Student Information Processing Board
 *
 * For copyright information, see copyright.h.
 */
#[c2rust::src_loc = "17:13"]
static mut MORE: [libc::c_char; 5] = [109, 111, 114, 101, 0];
#[no_mangle]
#[c2rust::src_loc = "31:1"]
pub unsafe extern "C" fn ss_pager_create() -> libc::c_int {
    let mut filedes: [libc::c_int; 2] = [0; 2];
    if pipe(filedes.as_mut_ptr()) != 0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    match fork() {
        -1 => { return -(1 as libc::c_int) }
        0 => {
            /*
         * Child; dup read half to 0, close all but 0, 1, and 2
         */
            if dup2(filedes[0 as libc::c_int as usize], 0 as libc::c_int) ==
                   -(1 as libc::c_int) {
                exit(1 as libc::c_int);
            }
            ss_page_stdin();
        }
        _ => { }
    }
    /*
         * Parent:  close "read" side of pipe, return
         * "write" side.
         */
    close(filedes[0 as libc::c_int as usize]);
    fcntl(filedes[1 as libc::c_int as usize], 2 as libc::c_int,
          1 as libc::c_int);
    return filedes[1 as libc::c_int as usize];
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1987, 1988 by MIT Student Information Processing Board
 *
 * For copyright information, see copyright.h.
 */
/* LOCAL_ALLOC stuff */
/* abbrev name */
/* new tokens to insert */
/*    char *path; */
/* init values */
/* this subsystem */
/* current request info */
/* arg list */
/* primary name */
/* info directory for 'help' */
/* to be extracted by subroutines */
/* (void *) NULL */
/* for ss_listen processing */
/* to get out */
/* exit subsystem */
/*
 * this needs a *lot* of work....
 *
 * run in same process
 * handle SIGINT sensibly
 * allow finer control -- put-page-break-here
 */
/* don't fork */
#[no_mangle]
#[c2rust::src_loc = "69:1"]
pub unsafe extern "C" fn ss_page_stdin() {
    let mut i: libc::c_int = 0;
    let mut sa: sigaction =
        sigaction{__sigaction_handler: C2RustUnnamed{sa_handler: None,},
                  sa_mask: __sigset_t{__val: [0; 16],},
                  sa_flags: 0,
                  sa_restorer: None,};
    let mut mask: sigset_t = __sigset_t{__val: [0; 16],};
    i = 3 as libc::c_int;
    while i < 32 as libc::c_int { close(i); i += 1 }
    sa.__sigaction_handler.sa_handler = None;
    sigemptyset(&mut sa.sa_mask);
    sa.sa_flags = 0 as libc::c_int;
    sigaction(2 as libc::c_int, &mut sa, 0 as *mut sigaction);
    sigemptyset(&mut mask);
    sigaddset(&mut mask, 2 as libc::c_int);
    sigprocmask(1 as libc::c_int, &mut mask, 0 as *mut sigset_t);
    if _ss_pager_name.is_null() {
        _ss_pager_name =
            getenv(b"PAGER\x00" as *const u8 as *const libc::c_char);
        if _ss_pager_name.is_null() { _ss_pager_name = MORE.as_mut_ptr() }
    }
    execlp(_ss_pager_name, _ss_pager_name,
           0 as *mut libc::c_void as *mut libc::c_char);
    /* minimal recovery if pager program isn't found */
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut n: libc::c_int = 0;
    loop  {
        n =
            read(0 as libc::c_int, buf.as_mut_ptr() as *mut libc::c_void,
                 80 as libc::c_int as size_t) as libc::c_int;
        if !(n > 0 as libc::c_int) { break ; }
        write(1 as libc::c_int, buf.as_mut_ptr() as *const libc::c_void,
              n as libc::c_uint as size_t);
    }
    exit(*__errno_location());
}
