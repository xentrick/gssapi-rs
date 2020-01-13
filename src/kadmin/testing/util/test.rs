use ::libc;
#[c2rust::header_src = "/usr/include/tcl.h:4"]
pub mod tcl_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "493:16"]
    pub struct Tcl_Interp {
        pub resultDontUse: *mut libc::c_char,
        pub freeProcDontUse: Option<unsafe extern "C" fn(_: *mut libc::c_char)
                                        -> ()>,
        pub errorLineDontUse: libc::c_int,
    }
    #[c2rust::src_loc = "714:1"]
    pub type Tcl_AppInitProc
        =
        unsafe extern "C" fn(_: *mut Tcl_Interp) -> libc::c_int;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "2429:1"]
        pub fn Tcl_MainEx(argc: libc::c_int, argv: *mut *mut libc::c_char,
                          appInitProc: Option<Tcl_AppInitProc>,
                          interp: *mut Tcl_Interp);
    }
}
#[c2rust::header_src = "/usr/include/tclDecls.h:4"]
pub mod tclDecls_h {
    use super::tcl_h::Tcl_Interp;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "314:1"]
        pub fn Tcl_CreateInterp() -> *mut Tcl_Interp;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kadmin/testing/util/tcl_kadm5.h:8"]
pub mod tcl_kadm5_h {
    use super::tcl_h::Tcl_Interp;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
        #[no_mangle]
        #[c2rust::src_loc = "3:1"]
        pub fn Tcl_kadm5_init(interp: *mut Tcl_Interp);
    }
}
pub use self::tcl_h::{Tcl_Interp, Tcl_AppInitProc, Tcl_MainEx};
use self::tclDecls_h::Tcl_CreateInterp;
use self::tcl_kadm5_h::Tcl_kadm5_init;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
#[c2rust::src_loc = "13:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int 
 /* Values of command-line arguments. */
 {
    Tcl_MainEx(argc, argv, Some(Tcl_AppInit as Tcl_AppInitProc),
               Tcl_CreateInterp());
    return 0 as libc::c_int;
    /* Needed only to prevent compiler warning. */
}
#[no_mangle]
#[c2rust::src_loc = "33:1"]
pub unsafe extern "C" fn Tcl_AppInit(mut interp: *mut Tcl_Interp)
 -> libc::c_int {
    Tcl_kadm5_init(interp);
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
