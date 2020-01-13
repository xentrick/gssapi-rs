use ::libc;
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/ss/ss.h:2"]
pub mod ss_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "22:27"]
    pub struct _ss_request_entry {
        pub command_names: *const *const libc::c_char,
        pub function: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _:
                                                      *const *const libc::c_char,
                                                  _: libc::c_int,
                                                  _: *mut libc::c_void)
                                 -> ()>,
        pub info_string: *const libc::c_char,
        pub flags: libc::c_int,
    }
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1987, 1988 by MIT Student Information Processing Board
 *
 * For copyright information, see mit-sipb-copyright.h.
 */
    #[c2rust::src_loc = "22:1"]
    pub type ss_request_entry = _ss_request_entry;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "29:27"]
    pub struct _ss_request_table {
        pub version: libc::c_int,
        pub requests: *const ss_request_entry,
    }
    #[c2rust::src_loc = "29:1"]
    pub type ss_request_table = _ss_request_table;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "48:1"]
        pub fn ss_help(_: libc::c_int, _: *const *const libc::c_char,
                       _: libc::c_int, _: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "49:1"]
        pub fn ss_list_requests(_: libc::c_int, _: *const *const libc::c_char,
                                _: libc::c_int, _: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "50:1"]
        pub fn ss_quit(_: libc::c_int, _: *const *const libc::c_char,
                       _: libc::c_int, _: *mut libc::c_void);
    }
    /* whatever */
    /* foo */
    /* NULL */
    /* 0 */
    /* _ss_h */
}
pub use self::ss_h::{_ss_request_entry, ss_request_entry, _ss_request_table,
                     ss_request_table, ss_help, ss_list_requests, ss_quit};
extern "C" {
    #[no_mangle]
    #[c2rust::src_loc = "12:1"]
    pub fn ss_self_identify(_: libc::c_int, _: *const *const libc::c_char,
                            _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "23:1"]
    pub fn ss_unimplemented(_: libc::c_int, _: *const *const libc::c_char,
                            _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "58:1"]
    pub fn ss_subsystem_name(_: libc::c_int, _: *const *const libc::c_char,
                             _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "63:1"]
    pub fn ss_subsystem_version(_: libc::c_int, _: *const *const libc::c_char,
                                _: libc::c_int, _: *mut libc::c_void);
}
/* std_rqs.c - automatically generated from std_rqs.ct */
#[c2rust::src_loc = "8:27"]
static mut ssu00001: [*const libc::c_char; 2] =
    [b".\x00" as *const u8 as *const libc::c_char, 0 as *const libc::c_char];
#[c2rust::src_loc = "13:27"]
static mut ssu00002: [*const libc::c_char; 2] =
    [b"help\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "18:27"]
static mut ssu00003: [*const libc::c_char; 3] =
    [b"list_help\x00" as *const u8 as *const libc::c_char,
     b"lh\x00" as *const u8 as *const libc::c_char, 0 as *const libc::c_char];
#[c2rust::src_loc = "24:27"]
static mut ssu00004: [*const libc::c_char; 4] =
    [b"list_requests\x00" as *const u8 as *const libc::c_char,
     b"lr\x00" as *const u8 as *const libc::c_char,
     b"?\x00" as *const u8 as *const libc::c_char, 0 as *const libc::c_char];
#[c2rust::src_loc = "31:27"]
static mut ssu00005: [*const libc::c_char; 3] =
    [b"quit\x00" as *const u8 as *const libc::c_char,
     b"q\x00" as *const u8 as *const libc::c_char, 0 as *const libc::c_char];
#[c2rust::src_loc = "37:27"]
static mut ssu00006: [*const libc::c_char; 3] =
    [b"abbrev\x00" as *const u8 as *const libc::c_char,
     b"ab\x00" as *const u8 as *const libc::c_char, 0 as *const libc::c_char];
#[c2rust::src_loc = "43:27"]
static mut ssu00007: [*const libc::c_char; 3] =
    [b"execute\x00" as *const u8 as *const libc::c_char,
     b"e\x00" as *const u8 as *const libc::c_char, 0 as *const libc::c_char];
#[c2rust::src_loc = "49:27"]
static mut ssu00008: [*const libc::c_char; 2] =
    [b"?\x00" as *const u8 as *const libc::c_char, 0 as *const libc::c_char];
#[c2rust::src_loc = "54:27"]
static mut ssu00009: [*const libc::c_char; 2] =
    [b"subsystem_name\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "59:27"]
static mut ssu00010: [*const libc::c_char; 2] =
    [b"subsystem_version\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "64:25"]
static mut ssu00011: [ss_request_entry; 11] =
    unsafe {
        [{
             let mut init =
                 _ss_request_entry{command_names: ssu00001.as_ptr(),
                                   function:
                                       Some(ss_self_identify as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Identify the subsystem.\x00" as
                                           *const u8 as *const libc::c_char,
                                   flags: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00002.as_ptr(),
                                   function:
                                       Some(ss_help as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Display info on command or topic.\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00003.as_ptr(),
                                   function:
                                       Some(ss_unimplemented as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"List topics for which help is available.\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                   flags: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00004.as_ptr(),
                                   function:
                                       Some(ss_list_requests as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"List available commands.\x00" as
                                           *const u8 as *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00005.as_ptr(),
                                   function:
                                       Some(ss_quit as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Leave the subsystem.\x00" as
                                           *const u8 as *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00006.as_ptr(),
                                   function:
                                       Some(ss_unimplemented as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Enable/disable abbreviation processing of request lines.\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                   flags: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00007.as_ptr(),
                                   function:
                                       Some(ss_unimplemented as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Execute a UNIX command line.\x00" as
                                           *const u8 as *const libc::c_char,
                                   flags: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00008.as_ptr(),
                                   function:
                                       Some(ss_unimplemented as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Produce a list of the most commonly used requests.\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                   flags: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00009.as_ptr(),
                                   function:
                                       Some(ss_subsystem_name as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Return the name of this subsystem.\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                   flags: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00010.as_ptr(),
                                   function:
                                       Some(ss_subsystem_version as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *const *const libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> ()),
                                   info_string:
                                       b"Return the version of this subsystem.\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                   flags: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names:
                                       0 as *const *const libc::c_char,
                                   function: None,
                                   info_string: 0 as *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         }]
    };
#[no_mangle]
#[c2rust::src_loc = "108:18"]
pub static mut ss_std_requests: ss_request_table =
    unsafe {
        {
            let mut init =
                _ss_request_table{version: 2 as libc::c_int,
                                  requests: ssu00011.as_ptr(),};
            init
        }
    };
