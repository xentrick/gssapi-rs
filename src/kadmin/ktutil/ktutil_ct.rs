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
                     ss_request_table, ss_list_requests, ss_quit};
extern "C" {
    #[no_mangle]
    #[c2rust::src_loc = "13:1"]
    pub fn ktutil_clear_list(_: libc::c_int, _: *const *const libc::c_char,
                             _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "19:1"]
    pub fn ktutil_read_v5(_: libc::c_int, _: *const *const libc::c_char,
                          _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "25:1"]
    pub fn ktutil_read_v4(_: libc::c_int, _: *const *const libc::c_char,
                          _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "31:1"]
    pub fn ktutil_write_v5(_: libc::c_int, _: *const *const libc::c_char,
                           _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "37:1"]
    pub fn ktutil_write_v4(_: libc::c_int, _: *const *const libc::c_char,
                           _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "43:1"]
    pub fn ktutil_add_entry(_: libc::c_int, _: *const *const libc::c_char,
                            _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "49:1"]
    pub fn ktutil_delete_entry(_: libc::c_int, _: *const *const libc::c_char,
                               _: libc::c_int, _: *mut libc::c_void);
    #[no_mangle]
    #[c2rust::src_loc = "55:1"]
    pub fn ktutil_list(_: libc::c_int, _: *const *const libc::c_char,
                       _: libc::c_int, _: *mut libc::c_void);
}
/* ktutil_ct.c - automatically generated from ktutil_ct.ct */
#[c2rust::src_loc = "8:27"]
static mut ssu00001: [*const libc::c_char; 3] =
    [b"clear_list\x00" as *const u8 as *const libc::c_char,
     b"clear\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "14:27"]
static mut ssu00002: [*const libc::c_char; 3] =
    [b"read_kt\x00" as *const u8 as *const libc::c_char,
     b"rkt\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "20:27"]
static mut ssu00003: [*const libc::c_char; 3] =
    [b"read_st\x00" as *const u8 as *const libc::c_char,
     b"rst\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "26:27"]
static mut ssu00004: [*const libc::c_char; 3] =
    [b"write_kt\x00" as *const u8 as *const libc::c_char,
     b"wkt\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "32:27"]
static mut ssu00005: [*const libc::c_char; 3] =
    [b"write_st\x00" as *const u8 as *const libc::c_char,
     b"wst\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "38:27"]
static mut ssu00006: [*const libc::c_char; 3] =
    [b"add_entry\x00" as *const u8 as *const libc::c_char,
     b"addent\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "44:27"]
static mut ssu00007: [*const libc::c_char; 3] =
    [b"delete_entry\x00" as *const u8 as *const libc::c_char,
     b"delent\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
#[c2rust::src_loc = "50:27"]
static mut ssu00008: [*const libc::c_char; 3] =
    [b"list\x00" as *const u8 as *const libc::c_char,
     b"l\x00" as *const u8 as *const libc::c_char, 0 as *const libc::c_char];
#[c2rust::src_loc = "56:27"]
static mut ssu00009: [*const libc::c_char; 4] =
    [b"list_requests\x00" as *const u8 as *const libc::c_char,
     b"lr\x00" as *const u8 as *const libc::c_char,
     b"?\x00" as *const u8 as *const libc::c_char, 0 as *const libc::c_char];
#[c2rust::src_loc = "63:27"]
static mut ssu00010: [*const libc::c_char; 4] =
    [b"quit\x00" as *const u8 as *const libc::c_char,
     b"exit\x00" as *const u8 as *const libc::c_char,
     b"q\x00" as *const u8 as *const libc::c_char, 0 as *const libc::c_char];
#[c2rust::src_loc = "70:25"]
static mut ssu00011: [ss_request_entry; 11] =
    unsafe {
        [{
             let mut init =
                 _ss_request_entry{command_names: ssu00001.as_ptr(),
                                   function:
                                       Some(ktutil_clear_list as
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
                                       b"Clear the current keylist.\x00" as
                                           *const u8 as *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00002.as_ptr(),
                                   function:
                                       Some(ktutil_read_v5 as
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
                                       b"Read a krb5 keytab into the current keylist.\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00003.as_ptr(),
                                   function:
                                       Some(ktutil_read_v4 as
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
                                       b"Deprecated and removed.\x00" as
                                           *const u8 as *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00004.as_ptr(),
                                   function:
                                       Some(ktutil_write_v5 as
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
                                       b"Write the current keylist to a krb5 keytab.\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00005.as_ptr(),
                                   function:
                                       Some(ktutil_write_v4 as
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
                                       b"Deprecated and removed.\x00" as
                                           *const u8 as *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00006.as_ptr(),
                                   function:
                                       Some(ktutil_add_entry as
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
                                       b"Add an entry to the current keylist.\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00007.as_ptr(),
                                   function:
                                       Some(ktutil_delete_entry as
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
                                       b"Delete an entry from the current keylist.\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00008.as_ptr(),
                                   function:
                                       Some(ktutil_list as
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
                                       b"List the current keylist.\x00" as
                                           *const u8 as *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00009.as_ptr(),
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
                                       b"List available requests.\x00" as
                                           *const u8 as *const libc::c_char,
                                   flags: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _ss_request_entry{command_names: ssu00010.as_ptr(),
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
                                       b"Exit program.\x00" as *const u8 as
                                           *const libc::c_char,
                                   flags: 0 as libc::c_int,};
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
#[c2rust::src_loc = "114:18"]
pub static mut ktutil_cmds: ss_request_table =
    unsafe {
        {
            let mut init =
                _ss_request_table{version: 2 as libc::c_int,
                                  requests: ssu00011.as_ptr(),};
            init
        }
    };
