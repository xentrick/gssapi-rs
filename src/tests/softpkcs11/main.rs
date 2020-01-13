use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:35"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stdarg.h:35"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/usr/include/bits/types.h:35"]
pub mod types_h {
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = libc::c_ulong;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "147:1"]
    pub type __gid_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:35"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::__uint64_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:35"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:35"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/openssl/asn1.h:38"]
pub mod asn1_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "146:8"]
    pub struct asn1_string_st {
        pub length: libc::c_int,
        pub type_0: libc::c_int,
        pub data: *mut libc::c_uchar,
        pub flags: libc::c_long,
    }
    use super::ossl_typ_h::ASN1_INTEGER;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "570:1"]
        pub fn i2d_ASN1_INTEGER(a: *mut ASN1_INTEGER,
                                out: *mut *mut libc::c_uchar) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/openssl/ossl_typ.h:37"]
pub mod ossl_typ_h {
    #[c2rust::src_loc = "40:1"]
    pub type ASN1_INTEGER = asn1_string_st;
    #[c2rust::src_loc = "80:1"]
    pub type BIGNUM = bignum_st;
    #[c2rust::src_loc = "93:1"]
    pub type EVP_PKEY = evp_pkey_st;
    #[c2rust::src_loc = "110:1"]
    pub type RSA = rsa_st;
    #[c2rust::src_loc = "111:1"]
    pub type RSA_METHOD = rsa_meth_st;
    #[c2rust::src_loc = "120:1"]
    pub type X509 = x509_st;
    #[c2rust::src_loc = "125:1"]
    pub type X509_NAME = X509_name_st;
    #[c2rust::src_loc = "141:1"]
    pub type OPENSSL_INIT_SETTINGS = ossl_init_settings_st;
    use super::asn1_h::asn1_string_st;
    extern "C" {
        #[c2rust::src_loc = "80:16"]
        pub type bignum_st;
        #[c2rust::src_loc = "93:16"]
        pub type evp_pkey_st;
        #[c2rust::src_loc = "110:16"]
        pub type rsa_st;
        #[c2rust::src_loc = "111:16"]
        pub type rsa_meth_st;
        #[c2rust::src_loc = "120:16"]
        pub type x509_st;
        #[c2rust::src_loc = "125:16"]
        pub type X509_name_st;
        #[c2rust::src_loc = "141:16"]
        pub type ossl_init_settings_st;
    }
}
#[c2rust::header_src = "/usr/include/openssl/pem.h:39"]
pub mod pem_h {
    #[c2rust::src_loc = "231:1"]
    pub type pem_password_cb
        =
        unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int,
                             _: libc::c_int, _: *mut libc::c_void)
            -> libc::c_int;
    use super::FILE_h::FILE;
    use super::ossl_typ_h::{EVP_PKEY, X509};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "318:31"]
        pub fn PEM_read_PrivateKey(fp: *mut FILE, x: *mut *mut EVP_PKEY,
                                   cb: Option<pem_password_cb>,
                                   u: *mut libc::c_void) -> *mut EVP_PKEY;
        #[no_mangle]
        #[c2rust::src_loc = "290:22"]
        pub fn PEM_read_X509(fp: *mut FILE, x: *mut *mut X509,
                             cb: Option<pem_password_cb>,
                             u: *mut libc::c_void) -> *mut X509;
    }
}
#[c2rust::header_src = "/usr/include/ctype.h:43"]
pub mod ctype_h {
    #[c2rust::src_loc = "46:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed = 8;
    #[c2rust::src_loc = "58:3"]
    pub const _ISpunct: C2RustUnnamed = 4;
    #[c2rust::src_loc = "57:3"]
    pub const _IScntrl: C2RustUnnamed = 2;
    #[c2rust::src_loc = "56:3"]
    pub const _ISblank: C2RustUnnamed = 1;
    #[c2rust::src_loc = "55:3"]
    pub const _ISgraph: C2RustUnnamed = 32768;
    #[c2rust::src_loc = "54:3"]
    pub const _ISprint: C2RustUnnamed = 16384;
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed = 8192;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed = 4096;
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed = 2048;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed = 256;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    }
}
#[c2rust::header_src = "/usr/include/pwd.h:44"]
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
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "110:1"]
        pub fn getpwuid(__uid: __uid_t) -> *mut passwd;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/preauth/pkinit/pkcs11.h:46"]
pub mod pkcs11_h {
    /*
   Copyright 2006 g10 Code GmbH
   Copyright 2006 Andreas Jellinghaus

   This file is free software; as a special exception the author gives
   unlimited permission to copy and/or distribute it, with or without
   modifications, as long as this notice is preserved.

   This file is distributed in the hope that it will be useful, but
   WITHOUT ANY WARRANTY, to the extent permitted by law; without even
   the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
   PURPOSE.  */
    /* Please submit changes back to the Scute project at
   https://www.scute.org/ (or send them to marcus@g10code.com), so that
   they can be picked up by other projects from there as well.  */
    /* This file is a modified implementation of the PKCS #11 standard by
   RSA Security Inc.  It is mostly a drop-in replacement, with the
   following change:

   This header file does not require any macro definitions by the user
   (like CK_DEFINE_FUNCTION etc).  In fact, it defines those macros
   for you (if useful, some are missing, let me know if you need
   more).

   There is an additional API available that does comply better to the
   GNU coding standard.  It can be switched on by defining
   CRYPTOKI_GNU before including this header file.  For this, the
   following changes are made to the specification:

   All structure types are changed to a "struct ck_foo" where CK_FOO
   is the type name in PKCS #11.

   All non-structure types are changed to ck_foo_t where CK_FOO is the
   lowercase version of the type name in PKCS #11.  The basic types
   (CK_ULONG et al.) are removed without substitute.

   All members of structures are modified in the following way: Type
   indication prefixes are removed, and underscore characters are
   inserted before words.  Then the result is lowercased.

   Note that function names are still in the original case, as they
   need for ABI compatibility.

   CK_FALSE, CK_TRUE and NULL_PTR are removed without substitute.  Use
   <stdbool.h>.

   If CRYPTOKI_COMPAT is defined before including this header file,
   then none of the API changes above take place, and the API is the
   one defined by the PKCS #11 standard.  */
    /* The version of cryptoki we implement.  The revision is changed with
   each modification of this file.  If you do not use the "official"
   version of this file, please consider deleting the revision macro
   (you may use a macro with a different name to keep track of your
   versions).  */
    /* Compatibility interface is default, unless CRYPTOKI_GNU is
   given.  */
    /* System dependencies.  */
    /* If we are in compatibility mode, switch all exposed names to the
     PKCS #11 variant.  There are corresponding #undefs below.  */
    /* CRYPTOKI_COMPAT */
    #[c2rust::src_loc = "184:1"]
    pub type CK_FLAGS = libc::c_ulong;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "186:8"]
    pub struct _CK_VERSION {
        pub major: libc::c_uchar,
        pub minor: libc::c_uchar,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "193:8"]
    pub struct _CK_INFO {
        pub cryptokiVersion: _CK_VERSION,
        pub manufacturerID: [libc::c_uchar; 32],
        pub flags: CK_FLAGS,
        pub libraryDescription: [libc::c_uchar; 32],
        pub libraryVersion: _CK_VERSION,
    }
    #[c2rust::src_loc = "203:1"]
    pub type CK_NOTIFICATION = libc::c_ulong;
    #[c2rust::src_loc = "208:1"]
    pub type CK_SLOT_ID = libc::c_ulong;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "211:8"]
    pub struct _CK_SLOT_INFO {
        pub slotDescription: [libc::c_uchar; 64],
        pub manufacturerID: [libc::c_uchar; 32],
        pub flags: CK_FLAGS,
        pub hardwareVersion: _CK_VERSION,
        pub firmwareVersion: _CK_VERSION,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "227:8"]
    pub struct _CK_TOKEN_INFO {
        pub label: [libc::c_uchar; 32],
        pub manufacturerID: [libc::c_uchar; 32],
        pub model: [libc::c_uchar; 16],
        pub serialNumber: [libc::c_uchar; 16],
        pub flags: CK_FLAGS,
        pub ulMaxSessionCount: libc::c_ulong,
        pub ulSessionCount: libc::c_ulong,
        pub ulMaxRwSessionCount: libc::c_ulong,
        pub ulRwSessionCount: libc::c_ulong,
        pub ulMaxPinLen: libc::c_ulong,
        pub ulMinPinLen: libc::c_ulong,
        pub ulTotalPublicMemory: libc::c_ulong,
        pub ulFreePublicMemory: libc::c_ulong,
        pub ulTotalPrivateMemory: libc::c_ulong,
        pub ulFreePrivateMemory: libc::c_ulong,
        pub hardwareVersion: _CK_VERSION,
        pub firmwareVersion: _CK_VERSION,
        pub utcTime: [libc::c_uchar; 16],
    }
    #[c2rust::src_loc = "273:1"]
    pub type CK_SESSION_HANDLE = libc::c_ulong;
    #[c2rust::src_loc = "278:1"]
    pub type CK_USER_TYPE = libc::c_ulong;
    #[c2rust::src_loc = "285:1"]
    pub type CK_STATE = libc::c_ulong;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "294:8"]
    pub struct _CK_SESSION_INFO {
        pub slotID: CK_SLOT_ID,
        pub state: CK_STATE,
        pub flags: CK_FLAGS,
        pub ulDeviceError: libc::c_ulong,
    }
    #[c2rust::src_loc = "306:1"]
    pub type CK_OBJECT_HANDLE = libc::c_ulong;
    #[c2rust::src_loc = "309:1"]
    pub type CK_OBJECT_CLASS = libc::c_ulong;
    #[c2rust::src_loc = "330:1"]
    pub type CK_KEY_TYPE = libc::c_ulong;
    #[c2rust::src_loc = "360:1"]
    pub type CK_CERTIFICATE_TYPE = libc::c_ulong;
    #[c2rust::src_loc = "368:1"]
    pub type CK_ATTRIBUTE_TYPE = libc::c_ulong;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "457:8"]
    pub struct _CK_ATTRIBUTE {
        pub type_0: CK_ATTRIBUTE_TYPE,
        pub pValue: *mut libc::c_void,
        pub ulValueLen: libc::c_ulong,
    }
    #[c2rust::src_loc = "473:1"]
    pub type CK_MECHANISM_TYPE = libc::c_ulong;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "676:8"]
    pub struct _CK_MECHANISM {
        pub mechanism: CK_MECHANISM_TYPE,
        pub pParameter: *mut libc::c_void,
        pub ulParameterLen: libc::c_ulong,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "684:8"]
    pub struct _CK_MECHANISM_INFO {
        pub ulMinKeySize: libc::c_ulong,
        pub ulMaxKeySize: libc::c_ulong,
        pub flags: CK_FLAGS,
    }
    /* Flags for C_WaitForSlotEvent.  */
    #[c2rust::src_loc = "711:1"]
    pub type CK_RV = libc::c_ulong;
    #[c2rust::src_loc = "714:1"]
    pub type CK_NOTIFY
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE, _: CK_NOTIFICATION,
                                    _: *mut libc::c_void) -> CK_RV>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1001:8"]
    pub struct _CK_FUNCTION_LIST {
        pub version: _CK_VERSION,
        pub C_Initialize: CK_C_Initialize,
        pub C_Finalize: CK_C_Finalize,
        pub C_GetInfo: CK_C_GetInfo,
        pub C_GetFunctionList: CK_C_GetFunctionList,
        pub C_GetSlotList: CK_C_GetSlotList,
        pub C_GetSlotInfo: CK_C_GetSlotInfo,
        pub C_GetTokenInfo: CK_C_GetTokenInfo,
        pub C_GetMechanismList: CK_C_GetMechanismList,
        pub C_GetMechanismInfo: CK_C_GetMechanismInfo,
        pub C_InitToken: CK_C_InitToken,
        pub C_InitPIN: CK_C_InitPIN,
        pub C_SetPIN: CK_C_SetPIN,
        pub C_OpenSession: CK_C_OpenSession,
        pub C_CloseSession: CK_C_CloseSession,
        pub C_CloseAllSessions: CK_C_CloseAllSessions,
        pub C_GetSessionInfo: CK_C_GetSessionInfo,
        pub C_GetOperationState: CK_C_GetOperationState,
        pub C_SetOperationState: CK_C_SetOperationState,
        pub C_Login: CK_C_Login,
        pub C_Logout: CK_C_Logout,
        pub C_CreateObject: CK_C_CreateObject,
        pub C_CopyObject: CK_C_CopyObject,
        pub C_DestroyObject: CK_C_DestroyObject,
        pub C_GetObjectSize: CK_C_GetObjectSize,
        pub C_GetAttributeValue: CK_C_GetAttributeValue,
        pub C_SetAttributeValue: CK_C_SetAttributeValue,
        pub C_FindObjectsInit: CK_C_FindObjectsInit,
        pub C_FindObjects: CK_C_FindObjects,
        pub C_FindObjectsFinal: CK_C_FindObjectsFinal,
        pub C_EncryptInit: CK_C_EncryptInit,
        pub C_Encrypt: CK_C_Encrypt,
        pub C_EncryptUpdate: CK_C_EncryptUpdate,
        pub C_EncryptFinal: CK_C_EncryptFinal,
        pub C_DecryptInit: CK_C_DecryptInit,
        pub C_Decrypt: CK_C_Decrypt,
        pub C_DecryptUpdate: CK_C_DecryptUpdate,
        pub C_DecryptFinal: CK_C_DecryptFinal,
        pub C_DigestInit: CK_C_DigestInit,
        pub C_Digest: CK_C_Digest,
        pub C_DigestUpdate: CK_C_DigestUpdate,
        pub C_DigestKey: CK_C_DigestKey,
        pub C_DigestFinal: CK_C_DigestFinal,
        pub C_SignInit: CK_C_SignInit,
        pub C_Sign: CK_C_Sign,
        pub C_SignUpdate: CK_C_SignUpdate,
        pub C_SignFinal: CK_C_SignFinal,
        pub C_SignRecoverInit: CK_C_SignRecoverInit,
        pub C_SignRecover: CK_C_SignRecover,
        pub C_VerifyInit: CK_C_VerifyInit,
        pub C_Verify: CK_C_Verify,
        pub C_VerifyUpdate: CK_C_VerifyUpdate,
        pub C_VerifyFinal: CK_C_VerifyFinal,
        pub C_VerifyRecoverInit: CK_C_VerifyRecoverInit,
        pub C_VerifyRecover: CK_C_VerifyRecover,
        pub C_DigestEncryptUpdate: CK_C_DigestEncryptUpdate,
        pub C_DecryptDigestUpdate: CK_C_DecryptDigestUpdate,
        pub C_SignEncryptUpdate: CK_C_SignEncryptUpdate,
        pub C_DecryptVerifyUpdate: CK_C_DecryptVerifyUpdate,
        pub C_GenerateKey: CK_C_GenerateKey,
        pub C_GenerateKeyPair: CK_C_GenerateKeyPair,
        pub C_WrapKey: CK_C_WrapKey,
        pub C_UnwrapKey: CK_C_UnwrapKey,
        pub C_DeriveKey: CK_C_DeriveKey,
        pub C_SeedRandom: CK_C_SeedRandom,
        pub C_GenerateRandom: CK_C_GenerateRandom,
        pub C_GetFunctionStatus: CK_C_GetFunctionStatus,
        pub C_CancelFunction: CK_C_CancelFunction,
        pub C_WaitForSlotEvent: CK_C_WaitForSlotEvent,
    }
    #[c2rust::src_loc = "737:1"]
    pub type CK_C_WaitForSlotEvent
        =
        Option<unsafe extern "C" fn(_: CK_FLAGS, _: *mut CK_SLOT_ID,
                                    _: *mut libc::c_void) -> CK_RV>;
    #[c2rust::src_loc = "998:1"]
    pub type CK_C_CancelFunction
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "997:1"]
    pub type CK_C_GetFunctionStatus
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "992:1"]
    pub type CK_C_GenerateRandom
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "989:1"]
    pub type CK_C_SeedRandom
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "981:1"]
    pub type CK_C_DeriveKey
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_MECHANISM,
                                    _: CK_OBJECT_HANDLE,
                                    _: *mut _CK_ATTRIBUTE, _: libc::c_ulong,
                                    _: *mut CK_OBJECT_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "972:1"]
    pub type CK_C_UnwrapKey
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_MECHANISM,
                                    _: CK_OBJECT_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut _CK_ATTRIBUTE, _: libc::c_ulong,
                                    _: *mut CK_OBJECT_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "965:1"]
    pub type CK_C_WrapKey
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_MECHANISM,
                                    _: CK_OBJECT_HANDLE, _: CK_OBJECT_HANDLE,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "956:1"]
    pub type CK_C_GenerateKeyPair
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_MECHANISM,
                                    _: *mut _CK_ATTRIBUTE, _: libc::c_ulong,
                                    _: *mut _CK_ATTRIBUTE, _: libc::c_ulong,
                                    _: *mut CK_OBJECT_HANDLE,
                                    _: *mut CK_OBJECT_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "950:1"]
    pub type CK_C_GenerateKey
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_MECHANISM,
                                    _: *mut _CK_ATTRIBUTE, _: libc::c_ulong,
                                    _: *mut CK_OBJECT_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "943:1"]
    pub type CK_C_DecryptVerifyUpdate
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "938:1"]
    pub type CK_C_SignEncryptUpdate
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "932:1"]
    pub type CK_C_DecryptDigestUpdate
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "927:1"]
    pub type CK_C_DigestEncryptUpdate
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "920:1"]
    pub type CK_C_VerifyRecover
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "916:1"]
    pub type CK_C_VerifyRecoverInit
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_MECHANISM,
                                    _: CK_OBJECT_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "912:1"]
    pub type CK_C_VerifyFinal
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "909:1"]
    pub type CK_C_VerifyUpdate
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "904:1"]
    pub type CK_C_Verify
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "900:1"]
    pub type CK_C_VerifyInit
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_MECHANISM,
                                    _: CK_OBJECT_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "894:1"]
    pub type CK_C_SignRecover
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "890:1"]
    pub type CK_C_SignRecoverInit
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_MECHANISM,
                                    _: CK_OBJECT_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "886:1"]
    pub type CK_C_SignFinal
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "883:1"]
    pub type CK_C_SignUpdate
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "878:1"]
    pub type CK_C_Sign
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "874:1"]
    pub type CK_C_SignInit
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_MECHANISM,
                                    _: CK_OBJECT_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "869:1"]
    pub type CK_C_DigestFinal
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "867:1"]
    pub type CK_C_DigestKey
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE, _: CK_OBJECT_HANDLE)
                   -> CK_RV>;
    #[c2rust::src_loc = "864:1"]
    pub type CK_C_DigestUpdate
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "859:1"]
    pub type CK_C_Digest
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "856:1"]
    pub type CK_C_DigestInit
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_MECHANISM) -> CK_RV>;
    #[c2rust::src_loc = "851:1"]
    pub type CK_C_DecryptFinal
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "846:1"]
    pub type CK_C_DecryptUpdate
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "841:1"]
    pub type CK_C_Decrypt
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "837:1"]
    pub type CK_C_DecryptInit
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_MECHANISM,
                                    _: CK_OBJECT_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "832:1"]
    pub type CK_C_EncryptFinal
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "827:1"]
    pub type CK_C_EncryptUpdate
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "822:1"]
    pub type CK_C_Encrypt
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "818:1"]
    pub type CK_C_EncryptInit
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_MECHANISM,
                                    _: CK_OBJECT_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "815:1"]
    pub type CK_C_FindObjectsFinal
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "810:1"]
    pub type CK_C_FindObjects
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut CK_OBJECT_HANDLE,
                                    _: libc::c_ulong, _: *mut libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "806:1"]
    pub type CK_C_FindObjectsInit
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_ATTRIBUTE, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "801:1"]
    pub type CK_C_SetAttributeValue
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE, _: CK_OBJECT_HANDLE,
                                    _: *mut _CK_ATTRIBUTE, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "796:1"]
    pub type CK_C_GetAttributeValue
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE, _: CK_OBJECT_HANDLE,
                                    _: *mut _CK_ATTRIBUTE, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "792:1"]
    pub type CK_C_GetObjectSize
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE, _: CK_OBJECT_HANDLE,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "789:1"]
    pub type CK_C_DestroyObject
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE, _: CK_OBJECT_HANDLE)
                   -> CK_RV>;
    #[c2rust::src_loc = "785:1"]
    pub type CK_C_CopyObject
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE, _: CK_OBJECT_HANDLE,
                                    _: *mut _CK_ATTRIBUTE, _: libc::c_ulong,
                                    _: *mut CK_OBJECT_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "781:1"]
    pub type CK_C_CreateObject
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_ATTRIBUTE, _: libc::c_ulong,
                                    _: *mut CK_OBJECT_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "779:1"]
    pub type CK_C_Logout
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "776:1"]
    pub type CK_C_Login
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE, _: CK_USER_TYPE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "770:1"]
    pub type CK_C_SetOperationState
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: CK_OBJECT_HANDLE, _: CK_OBJECT_HANDLE)
                   -> CK_RV>;
    #[c2rust::src_loc = "766:1"]
    pub type CK_C_GetOperationState
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "763:1"]
    pub type CK_C_GetSessionInfo
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut _CK_SESSION_INFO) -> CK_RV>;
    #[c2rust::src_loc = "762:1"]
    pub type CK_C_CloseAllSessions
        =
        Option<unsafe extern "C" fn(_: CK_SLOT_ID) -> CK_RV>;
    #[c2rust::src_loc = "761:1"]
    pub type CK_C_CloseSession
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "757:1"]
    pub type CK_C_OpenSession
        =
        Option<unsafe extern "C" fn(_: CK_SLOT_ID, _: CK_FLAGS,
                                    _: *mut libc::c_void, _: CK_NOTIFY,
                                    _: *mut CK_SESSION_HANDLE) -> CK_RV>;
    #[c2rust::src_loc = "752:1"]
    pub type CK_C_SetPIN
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong,
                                    _: *mut libc::c_uchar, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "749:1"]
    pub type CK_C_InitPIN
        =
        Option<unsafe extern "C" fn(_: CK_SESSION_HANDLE,
                                    _: *mut libc::c_uchar, _: libc::c_ulong)
                   -> CK_RV>;
    #[c2rust::src_loc = "746:1"]
    pub type CK_C_InitToken
        =
        Option<unsafe extern "C" fn(_: CK_SLOT_ID, _: *mut libc::c_uchar,
                                    _: libc::c_ulong, _: *mut libc::c_uchar)
                   -> CK_RV>;
    #[c2rust::src_loc = "743:1"]
    pub type CK_C_GetMechanismInfo
        =
        Option<unsafe extern "C" fn(_: CK_SLOT_ID, _: CK_MECHANISM_TYPE,
                                    _: *mut _CK_MECHANISM_INFO) -> CK_RV>;
    #[c2rust::src_loc = "739:1"]
    pub type CK_C_GetMechanismList
        =
        Option<unsafe extern "C" fn(_: CK_SLOT_ID, _: *mut CK_MECHANISM_TYPE,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "735:1"]
    pub type CK_C_GetTokenInfo
        =
        Option<unsafe extern "C" fn(_: CK_SLOT_ID, _: *mut _CK_TOKEN_INFO)
                   -> CK_RV>;
    #[c2rust::src_loc = "733:1"]
    pub type CK_C_GetSlotInfo
        =
        Option<unsafe extern "C" fn(_: CK_SLOT_ID, _: *mut _CK_SLOT_INFO)
                   -> CK_RV>;
    #[c2rust::src_loc = "730:1"]
    pub type CK_C_GetSlotList
        =
        Option<unsafe extern "C" fn(_: libc::c_uchar, _: *mut CK_SLOT_ID,
                                    _: *mut libc::c_ulong) -> CK_RV>;
    #[c2rust::src_loc = "727:1"]
    pub type CK_C_GetFunctionList
        =
        Option<unsafe extern "C" fn(_: *mut *mut _CK_FUNCTION_LIST) -> CK_RV>;
    #[c2rust::src_loc = "726:1"]
    pub type CK_C_GetInfo
        =
        Option<unsafe extern "C" fn(_: *mut _CK_INFO) -> CK_RV>;
    #[c2rust::src_loc = "725:1"]
    pub type CK_C_Finalize
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> CK_RV>;
    #[c2rust::src_loc = "724:1"]
    pub type CK_C_Initialize
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> CK_RV>;
    #[c2rust::src_loc = "1203:1"]
    pub type CK_VOID_PTR = *mut libc::c_void;
    #[c2rust::src_loc = "1199:1"]
    pub type CK_BYTE_PTR = *mut CK_BYTE;
    /* Compatibility layer.  */
    /* For NULL.  */
    #[c2rust::src_loc = "1193:1"]
    pub type CK_BYTE = libc::c_uchar;
    #[c2rust::src_loc = "1239:1"]
    pub type CK_ATTRIBUTE = _CK_ATTRIBUTE;
    #[c2rust::src_loc = "1248:1"]
    pub type CK_MECHANISM_PTR = *mut _CK_MECHANISM;
    #[c2rust::src_loc = "1245:1"]
    pub type CK_MECHANISM_TYPE_PTR = *mut CK_MECHANISM_TYPE;
    #[c2rust::src_loc = "1201:1"]
    pub type CK_UTF8CHAR_PTR = *mut CK_UTF8CHAR;
    #[c2rust::src_loc = "1195:1"]
    pub type CK_UTF8CHAR = libc::c_uchar;
    #[c2rust::src_loc = "1202:1"]
    pub type CK_ULONG_PTR = *mut CK_ULONG;
    #[c2rust::src_loc = "1197:1"]
    pub type CK_ULONG = libc::c_ulong;
    #[c2rust::src_loc = "1196:1"]
    pub type CK_BBOOL = libc::c_uchar;
    #[c2rust::src_loc = "1258:1"]
    pub type CK_C_INITIALIZE_ARGS_PTR = *mut _CK_C_INITIALIZE_ARGS;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1081:8"]
    pub struct _CK_C_INITIALIZE_ARGS {
        pub CreateMutex: CK_CREATEMUTEX,
        pub DestroyMutex: CK_DESTROYMUTEX,
        pub LockMutex: CK_LOCKMUTEX,
        pub UnlockMutex: CK_UNLOCKMUTEX,
        pub flags: CK_FLAGS,
        pub pReserved: *mut libc::c_void,
    }
    #[c2rust::src_loc = "1078:1"]
    pub type CK_UNLOCKMUTEX
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> CK_RV>;
    #[c2rust::src_loc = "1077:1"]
    pub type CK_LOCKMUTEX
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> CK_RV>;
    #[c2rust::src_loc = "1076:1"]
    pub type CK_DESTROYMUTEX
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> CK_RV>;
    #[c2rust::src_loc = "1075:1"]
    pub type CK_CREATEMUTEX
        =
        Option<unsafe extern "C" fn(_: *mut *mut libc::c_void) -> CK_RV>;
    #[c2rust::src_loc = "1220:1"]
    pub type CK_INFO_PTR = *mut _CK_INFO;
    #[c2rust::src_loc = "1255:1"]
    pub type CK_FUNCTION_LIST_PTR_PTR = *mut *mut _CK_FUNCTION_LIST;
    #[c2rust::src_loc = "1253:1"]
    pub type CK_FUNCTION_LIST = _CK_FUNCTION_LIST;
    #[c2rust::src_loc = "1235:1"]
    pub type CK_OBJECT_HANDLE_PTR = *mut CK_OBJECT_HANDLE;
    #[c2rust::src_loc = "1240:1"]
    pub type CK_ATTRIBUTE_PTR = *mut _CK_ATTRIBUTE;
    #[c2rust::src_loc = "1233:1"]
    pub type CK_SESSION_INFO_PTR = *mut _CK_SESSION_INFO;
    #[c2rust::src_loc = "1230:1"]
    pub type CK_SESSION_HANDLE_PTR = *mut CK_SESSION_HANDLE;
    #[c2rust::src_loc = "1251:1"]
    pub type CK_MECHANISM_INFO_PTR = *mut _CK_MECHANISM_INFO;
    #[c2rust::src_loc = "1228:1"]
    pub type CK_TOKEN_INFO_PTR = *mut _CK_TOKEN_INFO;
    #[c2rust::src_loc = "1225:1"]
    pub type CK_SLOT_INFO_PTR = *mut _CK_SLOT_INFO;
    #[c2rust::src_loc = "1222:1"]
    pub type CK_SLOT_ID_PTR = *mut CK_SLOT_ID;
    /* PKCS11_H */
    /* System dependencies.  */
    /* CRYPTOKI_COMPAT */
    /* Delete the helper macros defined at the top of the file.  */
}
#[c2rust::header_src = "/usr/include/string.h:35"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "272:15"]
        pub fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "345:1"]
        pub fn strtok_r(__s: *mut libc::c_char, __delim: *const libc::c_char,
                        __save_ptr: *mut *mut libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
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
#[c2rust::header_src = "/usr/include/stdlib.h:35"]
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
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
        #[no_mangle]
        #[c2rust::src_loc = "634:1"]
        pub fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:35"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    use super::internal::__va_list_tag;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "218:1"]
        pub fn fflush(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "246:14"]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "341:12"]
        pub fn vfprintf(_: *mut FILE, _: *const libc::c_char,
                        _: ::std::ffi::VaList) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "347:12"]
        pub fn vprintf(_: *const libc::c_char, _: ::std::ffi::VaList)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "358:12"]
        pub fn vsnprintf(_: *mut libc::c_char, _: libc::c_ulong,
                         _: *const libc::c_char, _: ::std::ffi::VaList)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "564:1"]
        pub fn fgets(__s: *mut libc::c_char, __n: libc::c_int,
                     __stream: *mut FILE) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:35"]
pub mod unistd_h {
    use super::types_h::__uid_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "675:1"]
        pub fn getuid() -> __uid_t;
        #[no_mangle]
        #[c2rust::src_loc = "678:1"]
        pub fn geteuid() -> __uid_t;
    }
}
#[c2rust::header_src = "/usr/include/openssl/err.h:37"]
pub mod err_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "236:1"]
        pub fn ERR_error_string(e: libc::c_ulong, buf: *mut libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "223:1"]
        pub fn ERR_get_error() -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/openssl/crypto.h:37"]
pub mod crypto_h {
    use super::stdint_uintn_h::uint64_t;
    use super::ossl_typ_h::OPENSSL_INIT_SETTINGS;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "388:1"]
        pub fn OPENSSL_init_crypto(opts: uint64_t,
                                   settings: *const OPENSSL_INIT_SETTINGS)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/openssl/bn.h:38"]
pub mod bn_h {
    use super::ossl_typ_h::BIGNUM;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "222:1"]
        pub fn BN_bn2bin(a: *const BIGNUM, to: *mut libc::c_uchar)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn BN_num_bits(a: *const BIGNUM) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/openssl/evp.h:38"]
pub mod evp_h {
    use super::ossl_typ_h::{EVP_PKEY, rsa_st};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "993:1"]
        pub fn EVP_PKEY_base_id(pkey: *const EVP_PKEY) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1017:1"]
        pub fn EVP_PKEY_get0_RSA(pkey: *mut EVP_PKEY) -> *mut rsa_st;
        #[no_mangle]
        #[c2rust::src_loc = "1041:1"]
        pub fn EVP_PKEY_free(pkey: *mut EVP_PKEY);
    }
}
#[c2rust::header_src = "/usr/include/openssl/rsa.h:39"]
pub mod rsa_h {
    use super::ossl_typ_h::{RSA, BIGNUM, RSA_METHOD};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "201:1"]
        pub fn RSA_size(rsa: *const RSA) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "209:1"]
        pub fn RSA_get0_key(r: *const RSA, n: *mut *const BIGNUM,
                            e: *mut *const BIGNUM, d: *mut *const BIGNUM);
        #[no_mangle]
        #[c2rust::src_loc = "254:1"]
        pub fn RSA_public_encrypt(flen: libc::c_int,
                                  from: *const libc::c_uchar,
                                  to: *mut libc::c_uchar, rsa: *mut RSA,
                                  padding: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "256:1"]
        pub fn RSA_private_encrypt(flen: libc::c_int,
                                   from: *const libc::c_uchar,
                                   to: *mut libc::c_uchar, rsa: *mut RSA,
                                   padding: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "258:1"]
        pub fn RSA_public_decrypt(flen: libc::c_int,
                                  from: *const libc::c_uchar,
                                  to: *mut libc::c_uchar, rsa: *mut RSA,
                                  padding: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "260:1"]
        pub fn RSA_private_decrypt(flen: libc::c_int,
                                   from: *const libc::c_uchar,
                                   to: *mut libc::c_uchar, rsa: *mut RSA,
                                   padding: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "272:1"]
        pub fn RSA_set_method(rsa: *mut RSA, meth: *const RSA_METHOD)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "275:1"]
        pub fn RSA_PKCS1_OpenSSL() -> *const RSA_METHOD;
        #[no_mangle]
        #[c2rust::src_loc = "331:1"]
        pub fn RSA_blinding_off(rsa: *mut RSA);
    }
}
#[c2rust::header_src = "/usr/include/openssl/x509.h:39"]
pub mod x509_h {
    use super::ossl_typ_h::{X509, X509_NAME, ASN1_INTEGER, EVP_PKEY};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "468:1"]
        pub fn X509_dup(x509: *mut X509) -> *mut X509;
        #[no_mangle]
        #[c2rust::src_loc = "545:1"]
        pub fn i2d_X509_NAME(a: *mut X509_NAME, out: *mut *mut libc::c_uchar)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "551:1"]
        pub fn X509_free(a: *mut X509);
        #[no_mangle]
        #[c2rust::src_loc = "551:1"]
        pub fn i2d_X509(a: *mut X509, out: *mut *mut libc::c_uchar)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "637:1"]
        pub fn X509_get_serialNumber(x: *mut X509) -> *mut ASN1_INTEGER;
        #[no_mangle]
        #[c2rust::src_loc = "640:1"]
        pub fn X509_get_issuer_name(a: *const X509) -> *mut X509_NAME;
        #[no_mangle]
        #[c2rust::src_loc = "642:1"]
        pub fn X509_get_subject_name(a: *const X509) -> *mut X509_NAME;
        #[no_mangle]
        #[c2rust::src_loc = "672:1"]
        pub fn X509_get_pubkey(x: *mut X509) -> *mut EVP_PKEY;
        #[no_mangle]
        #[c2rust::src_loc = "749:1"]
        pub fn X509_check_private_key(x509: *const X509,
                                      pkey: *const EVP_PKEY) -> libc::c_int;
    }
}
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::stddef_h::size_t;
pub use self::stdarg_h::va_list;
pub use self::types_h::{__uint64_t, __uid_t, __gid_t, __off_t, __off64_t};
pub use self::stdint_uintn_h::uint64_t;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::asn1_h::{asn1_string_st, i2d_ASN1_INTEGER};
pub use self::ossl_typ_h::{ASN1_INTEGER, BIGNUM, EVP_PKEY, RSA, RSA_METHOD,
                           X509, X509_NAME, OPENSSL_INIT_SETTINGS, bignum_st,
                           evp_pkey_st, rsa_st, rsa_meth_st, x509_st,
                           X509_name_st, ossl_init_settings_st};
pub use self::pem_h::{pem_password_cb, PEM_read_PrivateKey, PEM_read_X509};
pub use self::ctype_h::{C2RustUnnamed, _ISalnum, _ISpunct, _IScntrl, _ISblank,
                        _ISgraph, _ISprint, _ISspace, _ISxdigit, _ISdigit,
                        _ISalpha, _ISlower, _ISupper, __ctype_b_loc};
pub use self::pwd_h::{passwd, getpwuid};
pub use self::pkcs11_h::{CK_FLAGS, _CK_VERSION, _CK_INFO, CK_NOTIFICATION,
                         CK_SLOT_ID, _CK_SLOT_INFO, _CK_TOKEN_INFO,
                         CK_SESSION_HANDLE, CK_USER_TYPE, CK_STATE,
                         _CK_SESSION_INFO, CK_OBJECT_HANDLE, CK_OBJECT_CLASS,
                         CK_KEY_TYPE, CK_CERTIFICATE_TYPE, CK_ATTRIBUTE_TYPE,
                         _CK_ATTRIBUTE, CK_MECHANISM_TYPE, _CK_MECHANISM,
                         _CK_MECHANISM_INFO, CK_RV, CK_NOTIFY,
                         _CK_FUNCTION_LIST, CK_C_WaitForSlotEvent,
                         CK_C_CancelFunction, CK_C_GetFunctionStatus,
                         CK_C_GenerateRandom, CK_C_SeedRandom, CK_C_DeriveKey,
                         CK_C_UnwrapKey, CK_C_WrapKey, CK_C_GenerateKeyPair,
                         CK_C_GenerateKey, CK_C_DecryptVerifyUpdate,
                         CK_C_SignEncryptUpdate, CK_C_DecryptDigestUpdate,
                         CK_C_DigestEncryptUpdate, CK_C_VerifyRecover,
                         CK_C_VerifyRecoverInit, CK_C_VerifyFinal,
                         CK_C_VerifyUpdate, CK_C_Verify, CK_C_VerifyInit,
                         CK_C_SignRecover, CK_C_SignRecoverInit,
                         CK_C_SignFinal, CK_C_SignUpdate, CK_C_Sign,
                         CK_C_SignInit, CK_C_DigestFinal, CK_C_DigestKey,
                         CK_C_DigestUpdate, CK_C_Digest, CK_C_DigestInit,
                         CK_C_DecryptFinal, CK_C_DecryptUpdate, CK_C_Decrypt,
                         CK_C_DecryptInit, CK_C_EncryptFinal,
                         CK_C_EncryptUpdate, CK_C_Encrypt, CK_C_EncryptInit,
                         CK_C_FindObjectsFinal, CK_C_FindObjects,
                         CK_C_FindObjectsInit, CK_C_SetAttributeValue,
                         CK_C_GetAttributeValue, CK_C_GetObjectSize,
                         CK_C_DestroyObject, CK_C_CopyObject,
                         CK_C_CreateObject, CK_C_Logout, CK_C_Login,
                         CK_C_SetOperationState, CK_C_GetOperationState,
                         CK_C_GetSessionInfo, CK_C_CloseAllSessions,
                         CK_C_CloseSession, CK_C_OpenSession, CK_C_SetPIN,
                         CK_C_InitPIN, CK_C_InitToken, CK_C_GetMechanismInfo,
                         CK_C_GetMechanismList, CK_C_GetTokenInfo,
                         CK_C_GetSlotInfo, CK_C_GetSlotList,
                         CK_C_GetFunctionList, CK_C_GetInfo, CK_C_Finalize,
                         CK_C_Initialize, CK_VOID_PTR, CK_BYTE_PTR, CK_BYTE,
                         CK_ATTRIBUTE, CK_MECHANISM_PTR,
                         CK_MECHANISM_TYPE_PTR, CK_UTF8CHAR_PTR, CK_UTF8CHAR,
                         CK_ULONG_PTR, CK_ULONG, CK_BBOOL,
                         CK_C_INITIALIZE_ARGS_PTR, _CK_C_INITIALIZE_ARGS,
                         CK_UNLOCKMUTEX, CK_LOCKMUTEX, CK_DESTROYMUTEX,
                         CK_CREATEMUTEX, CK_INFO_PTR,
                         CK_FUNCTION_LIST_PTR_PTR, CK_FUNCTION_LIST,
                         CK_OBJECT_HANDLE_PTR, CK_ATTRIBUTE_PTR,
                         CK_SESSION_INFO_PTR, CK_SESSION_HANDLE_PTR,
                         CK_MECHANISM_INFO_PTR, CK_TOKEN_INFO_PTR,
                         CK_SLOT_INFO_PTR, CK_SLOT_ID_PTR};
use self::string_h::{strcmp, strdup, strcspn, strtok_r, strlen, memcmp,
                     memset, memcpy};
use self::stdlib_h::{malloc, calloc, realloc, free, abort, getenv};
use self::stdio_h::{fclose, fflush, fopen, vfprintf, vprintf, vsnprintf,
                    asprintf, fgets};
use self::unistd_h::{getuid, geteuid};
use self::err_h::{ERR_error_string, ERR_get_error};
use self::crypto_h::OPENSSL_init_crypto;
use self::bn_h::{BN_bn2bin, BN_num_bits};
use self::evp_h::{EVP_PKEY_base_id, EVP_PKEY_get0_RSA, EVP_PKEY_free};
use self::rsa_h::{RSA_size, RSA_get0_key, RSA_public_encrypt,
                  RSA_private_encrypt, RSA_public_decrypt,
                  RSA_private_decrypt, RSA_set_method, RSA_PKCS1_OpenSSL,
                  RSA_blinding_off};
use self::x509_h::{X509_dup, i2d_X509_NAME, X509_free, i2d_X509,
                   X509_get_serialNumber, X509_get_issuer_name,
                   X509_get_subject_name, X509_get_pubkey,
                   X509_check_private_key};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "99:8"]
pub struct st_object {
    pub object_handle: CK_OBJECT_HANDLE,
    pub attrs: *mut st_attr,
    pub num_attributes: libc::c_int,
    pub type_0: C2RustUnnamed_2,
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "108:5"]
pub union C2RustUnnamed_0 {
    pub cert: *mut X509,
    pub public_key: *mut EVP_PKEY,
    pub private_key: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "111:9"]
pub struct C2RustUnnamed_1 {
    pub file: *mut libc::c_char,
    pub key: *mut EVP_PKEY,
    pub cert: *mut X509,
}
#[c2rust::src_loc = "103:5"]
pub type C2RustUnnamed_2 = libc::c_uint;
#[c2rust::src_loc = "106:9"]
pub const STO_T_PUBLIC_KEY: C2RustUnnamed_2 = 2;
#[c2rust::src_loc = "105:9"]
pub const STO_T_PRIVATE_KEY: C2RustUnnamed_2 = 1;
#[c2rust::src_loc = "104:9"]
pub const STO_T_CERTIFICATE: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "94:8"]
pub struct st_attr {
    pub attribute: CK_ATTRIBUTE,
    pub secret: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "119:15"]
pub struct soft_token {
    pub application: CK_VOID_PTR,
    pub notify: CK_NOTIFY,
    pub object: C2RustUnnamed_5,
    pub flags: C2RustUnnamed_4,
    pub open_sessions: libc::c_int,
    pub state: [session_state; 10],
    pub logfile: *mut FILE,
    pub next_session_handle: CK_SESSION_HANDLE,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "132:12"]
pub struct session_state {
    pub session_handle: CK_SESSION_HANDLE,
    pub find: C2RustUnnamed_3,
    pub encrypt_object: libc::c_int,
    pub encrypt_mechanism: CK_MECHANISM_PTR,
    pub decrypt_object: libc::c_int,
    pub decrypt_mechanism: CK_MECHANISM_PTR,
    pub sign_object: libc::c_int,
    pub sign_mechanism: CK_MECHANISM_PTR,
    pub verify_object: libc::c_int,
    pub verify_mechanism: CK_MECHANISM_PTR,
    pub digest_object: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "135:9"]
pub struct C2RustUnnamed_3 {
    pub attributes: *mut CK_ATTRIBUTE,
    pub num_attributes: CK_ULONG,
    pub next_object: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "126:5"]
pub struct C2RustUnnamed_4 {
    pub hardware_slot: libc::c_int,
    pub app_error_fatal: libc::c_int,
    pub login_done: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "122:5"]
pub struct C2RustUnnamed_5 {
    pub objs: *mut *mut st_object,
    pub num_objs: libc::c_int,
}
#[c2rust::src_loc = "154:3"]
static mut soft_token: soft_token =
    soft_token{application: 0 as *const libc::c_void as *mut libc::c_void,
               notify: None,
               object:
                   C2RustUnnamed_5{objs:
                                       0 as *const *mut st_object as
                                           *mut *mut st_object,
                                   num_objs: 0,},
               flags:
                   C2RustUnnamed_4{hardware_slot: 0,
                                   app_error_fatal: 0,
                                   login_done: 0,},
               open_sessions: 0,
               state:
                   [session_state{session_handle: 0,
                                  find:
                                      C2RustUnnamed_3{attributes:
                                                          0 as
                                                              *const CK_ATTRIBUTE
                                                              as
                                                              *mut CK_ATTRIBUTE,
                                                      num_attributes: 0,
                                                      next_object: 0,},
                                  encrypt_object: 0,
                                  encrypt_mechanism:
                                      0 as *const _CK_MECHANISM as
                                          *mut _CK_MECHANISM,
                                  decrypt_object: 0,
                                  decrypt_mechanism:
                                      0 as *const _CK_MECHANISM as
                                          *mut _CK_MECHANISM,
                                  sign_object: 0,
                                  sign_mechanism:
                                      0 as *const _CK_MECHANISM as
                                          *mut _CK_MECHANISM,
                                  verify_object: 0,
                                  verify_mechanism:
                                      0 as *const _CK_MECHANISM as
                                          *mut _CK_MECHANISM,
                                  digest_object: 0,}; 10],
               logfile: 0 as *const FILE as *mut FILE,
               next_session_handle: 0,};
#[c2rust::src_loc = "156:1"]
unsafe extern "C" fn application_error(mut fmt: *const libc::c_char,
                                       mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    vprintf(fmt, ap.as_va_list());
    if soft_token.flags.app_error_fatal != 0 { abort(); };
}
#[c2rust::src_loc = "167:1"]
unsafe extern "C" fn st_logf(mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    if soft_token.logfile.is_null() { return }
    ap = args.clone();
    vfprintf(soft_token.logfile, fmt, ap.as_va_list());
    fflush(soft_token.logfile);
}
#[c2rust::src_loc = "179:1"]
unsafe extern "C" fn snprintf_fill(mut str: *mut libc::c_char,
                                   mut size: libc::c_int,
                                   mut fillchar: libc::c_char,
                                   mut fmt: *const libc::c_char,
                                   mut args: ...) {
    let mut len: libc::c_int = 0;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    len = vsnprintf(str, size as libc::c_ulong, fmt, ap.as_va_list());
    if len < 0 as libc::c_int || len > size { return }
    while len < size {
        let fresh0 = len;
        len = len + 1;
        *str.offset(fresh0 as isize) = fillchar
    };
}
/* return CKR_OK */
#[c2rust::src_loc = "206:1"]
unsafe extern "C" fn verify_session_handle(mut hSession: CK_SESSION_HANDLE,
                                           mut state: *mut *mut session_state)
 -> CK_RV {
    let mut i: size_t = 0; /* XXX */
    i = 0 as libc::c_int as size_t; /* XXX */
    while i <
              (::std::mem::size_of::<[session_state; 10]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<session_state>()
                                                   as libc::c_ulong) {
        if soft_token.state[i as usize].session_handle == hSession {
            break ; /* XXX */
        } /* XXX */
        i = i.wrapping_add(1)
    }
    if i ==
           (::std::mem::size_of::<[session_state; 10]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<session_state>()
                                                as libc::c_ulong) {
        application_error(b"use of invalid handle: 0x%08lx\n\x00" as *const u8
                              as *const libc::c_char, hSession);
        return 0xb3 as libc::c_int as CK_RV
    }
    if !state.is_null() {
        *state =
            &mut *soft_token.state.as_mut_ptr().offset(i as isize) as
                *mut session_state
    }
    return 0 as libc::c_int as CK_RV;
}
#[c2rust::src_loc = "226:1"]
unsafe extern "C" fn object_handle_to_object(mut handle: CK_OBJECT_HANDLE,
                                             mut object: *mut *mut st_object)
 -> CK_RV {
    let mut i: libc::c_int =
        (handle & 0xfff as libc::c_int as libc::c_ulong) as libc::c_int;
    *object = 0 as *mut st_object;
    if i >= soft_token.object.num_objs { return 7 as libc::c_int as CK_RV }
    if (*soft_token.object.objs.offset(i as isize)).is_null() {
        return 7 as libc::c_int as CK_RV
    }
    if (**soft_token.object.objs.offset(i as isize)).object_handle != handle {
        return 7 as libc::c_int as CK_RV
    }
    *object = *soft_token.object.objs.offset(i as isize);
    return 0 as libc::c_int as CK_RV;
}
#[c2rust::src_loc = "243:1"]
unsafe extern "C" fn attributes_match(mut obj: *const st_object,
                                      mut attributes: *const CK_ATTRIBUTE,
                                      mut num_attributes: CK_ULONG)
 -> libc::c_int {
    let mut i: CK_ULONG = 0;
    let mut j: libc::c_int = 0;
    st_logf(b"attributes_match: %ld\n\x00" as *const u8 as
                *const libc::c_char,
            (*obj).object_handle & 0xfff as libc::c_int as libc::c_ulong);
    i = 0 as libc::c_int as CK_ULONG;
    while i < num_attributes {
        let mut match_0: libc::c_int = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < (*obj).num_attributes {
            if (*attributes.offset(i as isize)).type_0 ==
                   (*(*obj).attrs.offset(j as isize)).attribute.type_0 &&
                   (*attributes.offset(i as isize)).ulValueLen ==
                       (*(*obj).attrs.offset(j as isize)).attribute.ulValueLen
                   &&
                   memcmp((*attributes.offset(i as isize)).pValue,
                          (*(*obj).attrs.offset(j as isize)).attribute.pValue,
                          (*attributes.offset(i as isize)).ulValueLen) ==
                       0 as libc::c_int {
                match_0 = 1 as libc::c_int;
                break ;
            } else { j += 1 }
        }
        if match_0 == 0 as libc::c_int {
            st_logf(b"type %lu attribute have no match\n\x00" as *const u8 as
                        *const libc::c_char,
                    (*attributes.offset(i as isize)).type_0);
            return 0 as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    st_logf(b"attribute matches\n\x00" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "272:1"]
unsafe extern "C" fn print_attributes(mut attributes: *const CK_ATTRIBUTE,
                                      mut num_attributes: CK_ULONG) {
    let mut i: CK_ULONG = 0;
    st_logf(b"find objects: attrs: %lu\n\x00" as *const u8 as
                *const libc::c_char, num_attributes);
    i = 0 as libc::c_int as CK_ULONG;
    while i < num_attributes {
        st_logf(b"  type: \x00" as *const u8 as *const libc::c_char);
        match (*attributes.offset(i as isize)).type_0 {
            1 => {
                let mut ck_true: *mut CK_BBOOL = 0 as *mut CK_BBOOL;
                if (*attributes.offset(i as isize)).ulValueLen !=
                       ::std::mem::size_of::<CK_BBOOL>() as libc::c_ulong {
                    application_error(b"token attribute wrong length\n\x00" as
                                          *const u8 as *const libc::c_char);
                } else {
                    ck_true =
                        (*attributes.offset(i as isize)).pValue as
                            *mut CK_BBOOL;
                    st_logf(b"token: %s\x00" as *const u8 as
                                *const libc::c_char,
                            if *ck_true as libc::c_int != 0 {
                                b"TRUE\x00" as *const u8 as
                                    *const libc::c_char
                            } else {
                                b"FALSE\x00" as *const u8 as
                                    *const libc::c_char
                            });
                }
            }
            0 => {
                let mut class: *mut CK_OBJECT_CLASS =
                    0 as *mut CK_OBJECT_CLASS;
                if (*attributes.offset(i as isize)).ulValueLen !=
                       ::std::mem::size_of::<CK_ULONG>() as libc::c_ulong {
                    application_error(b"class attribute wrong length\n\x00" as
                                          *const u8 as *const libc::c_char);
                } else {
                    class =
                        (*attributes.offset(i as isize)).pValue as
                            *mut CK_OBJECT_CLASS;
                    st_logf(b"class \x00" as *const u8 as
                                *const libc::c_char);
                    match *class {
                        1 => {
                            st_logf(b"certificate\x00" as *const u8 as
                                        *const libc::c_char);
                        }
                        2 => {
                            st_logf(b"public key\x00" as *const u8 as
                                        *const libc::c_char);
                        }
                        3 => {
                            st_logf(b"private key\x00" as *const u8 as
                                        *const libc::c_char);
                        }
                        4 => {
                            st_logf(b"secret key\x00" as *const u8 as
                                        *const libc::c_char);
                        }
                        6 => {
                            st_logf(b"domain parameters\x00" as *const u8 as
                                        *const libc::c_char);
                        }
                        _ => {
                            st_logf(b"[class %lx]\x00" as *const u8 as
                                        *const libc::c_char, *class);
                        }
                    }
                }
            }
            2 => {
                st_logf(b"private\x00" as *const u8 as *const libc::c_char);
            }
            3 => {
                st_logf(b"label\x00" as *const u8 as *const libc::c_char);
            }
            16 => {
                st_logf(b"application\x00" as *const u8 as
                            *const libc::c_char);
            }
            17 => {
                st_logf(b"value\x00" as *const u8 as *const libc::c_char);
            }
            258 => { st_logf(b"id\x00" as *const u8 as *const libc::c_char); }
            _ => {
                st_logf(b"[unknown 0x%08lx]\x00" as *const u8 as
                            *const libc::c_char,
                        (*attributes.offset(i as isize)).type_0);
            }
        }
        st_logf(b"\n\x00" as *const u8 as *const libc::c_char);
        i = i.wrapping_add(1)
    };
}
#[c2rust::src_loc = "346:1"]
unsafe extern "C" fn free_st_object(mut o: *mut st_object) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*o).num_attributes {
        free((*(*o).attrs.offset(i as isize)).attribute.pValue);
        i += 1
    }
    free((*o).attrs as *mut libc::c_void);
    if (*o).type_0 as libc::c_uint ==
           STO_T_CERTIFICATE as libc::c_int as libc::c_uint {
        X509_free((*o).u.cert);
    } else if (*o).type_0 as libc::c_uint ==
                  STO_T_PRIVATE_KEY as libc::c_int as libc::c_uint {
        free((*o).u.private_key.file as *mut libc::c_void);
        EVP_PKEY_free((*o).u.private_key.key);
        X509_free((*o).u.private_key.cert);
    } else if (*o).type_0 as libc::c_uint ==
                  STO_T_PUBLIC_KEY as libc::c_int as libc::c_uint {
        EVP_PKEY_free((*o).u.public_key);
    }
    free(o as *mut libc::c_void);
}
#[c2rust::src_loc = "366:1"]
unsafe extern "C" fn add_st_object() -> *mut st_object {
    let mut o: *mut st_object = 0 as *mut st_object;
    let mut objs: *mut *mut st_object = 0 as *mut *mut st_object;
    objs =
        realloc(soft_token.object.objs as *mut libc::c_void,
                ((soft_token.object.num_objs + 1 as libc::c_int) as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut st_object>()
                                                     as libc::c_ulong)) as
            *mut *mut st_object;
    if objs.is_null() { return 0 as *mut st_object }
    soft_token.object.objs = objs;
    o =
        malloc(::std::mem::size_of::<st_object>() as libc::c_ulong) as
            *mut st_object;
    if o.is_null() { return 0 as *mut st_object }
    memset(o as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<st_object>() as libc::c_ulong);
    (*o).attrs = 0 as *mut st_attr;
    (*o).num_attributes = 0 as libc::c_int;
    (*o).object_handle = soft_token.object.num_objs as CK_OBJECT_HANDLE;
    let fresh1 = soft_token.object.num_objs;
    soft_token.object.num_objs = soft_token.object.num_objs + 1;
    let ref mut fresh2 = *soft_token.object.objs.offset(fresh1 as isize);
    *fresh2 = o;
    return o;
}
#[c2rust::src_loc = "390:1"]
unsafe extern "C" fn add_object_attribute(mut o: *mut st_object,
                                          mut secret: libc::c_int,
                                          mut type_0: CK_ATTRIBUTE_TYPE,
                                          mut pValue: CK_VOID_PTR,
                                          mut ulValueLen: CK_ULONG) -> CK_RV {
    let mut a: *mut st_attr = 0 as *mut st_attr;
    let mut i: libc::c_int = 0;
    i = (*o).num_attributes;
    a =
        realloc((*o).attrs as *mut libc::c_void,
                ((i + 1 as libc::c_int) as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<st_attr>()
                                                     as libc::c_ulong)) as
            *mut st_attr;
    if a.is_null() { return 0x31 as libc::c_int as CK_RV }
    (*o).attrs = a;
    (*(*o).attrs.offset(i as isize)).secret = secret;
    (*(*o).attrs.offset(i as isize)).attribute.type_0 = type_0;
    let ref mut fresh3 = (*(*o).attrs.offset(i as isize)).attribute.pValue;
    *fresh3 = malloc(ulValueLen);
    if (*(*o).attrs.offset(i as isize)).attribute.pValue.is_null() &&
           ulValueLen != 0 as libc::c_int as libc::c_ulong {
        return 0x31 as libc::c_int as CK_RV
    }
    memcpy((*(*o).attrs.offset(i as isize)).attribute.pValue,
           pValue as *const libc::c_void, ulValueLen);
    (*(*o).attrs.offset(i as isize)).attribute.ulValueLen = ulValueLen;
    (*o).num_attributes += 1;
    return 0 as libc::c_int as CK_RV;
}
#[c2rust::src_loc = "417:1"]
unsafe extern "C" fn add_pubkey_info(mut o: *mut st_object,
                                     mut key_type: CK_KEY_TYPE,
                                     mut key: *mut EVP_PKEY) -> CK_RV {
    match key_type {
        0 => {
            let mut modulus: *mut CK_BYTE = 0 as *mut CK_BYTE;
            let mut modulus_len: size_t = 0 as libc::c_int as size_t;
            let mut modulus_bits: CK_ULONG = 0 as libc::c_int as CK_ULONG;
            let mut exponent: *mut CK_BYTE = 0 as *mut CK_BYTE;
            let mut exponent_len: size_t = 0 as libc::c_int as size_t;
            let mut rsa: *mut RSA = 0 as *mut RSA;
            let mut n: *const BIGNUM = 0 as *const BIGNUM;
            let mut e: *const BIGNUM = 0 as *const BIGNUM;
            rsa = EVP_PKEY_get0_RSA(key);
            RSA_get0_key(rsa, &mut n, &mut e, 0 as *mut *const BIGNUM);
            modulus_bits = BN_num_bits(n) as CK_ULONG;
            modulus_len =
                ((BN_num_bits(n) + 7 as libc::c_int) / 8 as libc::c_int) as
                    size_t;
            modulus = malloc(modulus_len) as *mut CK_BYTE;
            BN_bn2bin(n, modulus);
            exponent_len =
                ((BN_num_bits(e) + 7 as libc::c_int) / 8 as libc::c_int) as
                    size_t;
            exponent = malloc(exponent_len) as *mut CK_BYTE;
            BN_bn2bin(e, exponent);
            add_object_attribute(o, 0 as libc::c_int,
                                 0x120 as libc::c_int as CK_ATTRIBUTE_TYPE,
                                 modulus as CK_VOID_PTR, modulus_len);
            add_object_attribute(o, 0 as libc::c_int,
                                 0x121 as libc::c_int as CK_ATTRIBUTE_TYPE,
                                 &mut modulus_bits as *mut CK_ULONG as
                                     CK_VOID_PTR,
                                 ::std::mem::size_of::<CK_ULONG>() as
                                     libc::c_ulong);
            add_object_attribute(o, 0 as libc::c_int,
                                 0x122 as libc::c_int as CK_ATTRIBUTE_TYPE,
                                 exponent as CK_VOID_PTR, exponent_len);
            RSA_set_method(rsa, RSA_PKCS1_OpenSSL());
            free(modulus as *mut libc::c_void);
            free(exponent as *mut libc::c_void);
        }
        _ => { }
    }
    return 0 as libc::c_int as CK_RV;
}
#[c2rust::src_loc = "461:1"]
unsafe extern "C" fn pem_callback(mut buf: *mut libc::c_char,
                                  mut num: libc::c_int, mut w: libc::c_int,
                                  mut key: *mut libc::c_void) -> libc::c_int {
    return -(1 as libc::c_int);
}
#[c2rust::src_loc = "468:1"]
unsafe extern "C" fn add_certificate(mut label: *mut libc::c_char,
                                     mut cert_file: *const libc::c_char,
                                     mut private_key_file:
                                         *const libc::c_char,
                                     mut id: *mut libc::c_char,
                                     mut anchor: libc::c_int) -> CK_RV {
    let mut current_block: u64;
    let mut o: *mut st_object = 0 as *mut st_object;
    let mut bool_true: CK_BBOOL = 1 as libc::c_int as CK_BBOOL;
    let mut bool_false: CK_BBOOL = 0 as libc::c_int as CK_BBOOL;
    let mut c: CK_OBJECT_CLASS = 0;
    let mut cert_type: CK_CERTIFICATE_TYPE =
        0 as libc::c_int as CK_CERTIFICATE_TYPE;
    let mut key_type: CK_KEY_TYPE = 0;
    let mut mech_type: CK_MECHANISM_TYPE = 0;
    let mut cert_data: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut cert_length: size_t = 0;
    let mut subject_data: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut subject_length: size_t = 0;
    let mut issuer_data: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut issuer_length: size_t = 0;
    let mut serial_data: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut serial_length: size_t = 0;
    let mut ret: CK_RV = 5 as libc::c_int as CK_RV;
    let mut cert: *mut X509 = 0 as *mut X509;
    let mut public_key: *mut EVP_PKEY = 0 as *mut EVP_PKEY;
    let mut id_len: size_t = strlen(id);
    let mut f: *mut FILE = 0 as *mut FILE;
    f = fopen(cert_file, b"r\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        st_logf(b"failed to open file %s\n\x00" as *const u8 as
                    *const libc::c_char, cert_file);
        return 5 as libc::c_int as CK_RV
    }
    cert =
        PEM_read_X509(f, 0 as *mut *mut X509, None, 0 as *mut libc::c_void);
    fclose(f);
    if cert.is_null() {
        st_logf(b"failed reading PEM cert\n\x00" as *const u8 as
                    *const libc::c_char);
        return 5 as libc::c_int as CK_RV
    }
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    cert_length = i2d_X509(cert, 0 as *mut *mut libc::c_uchar) as size_t;
    if cert_length <= 0 as libc::c_int as libc::c_ulong {
        ret = 22 as libc::c_int as CK_RV
    } else {
        cert_data = malloc(cert_length);
        if cert_data.is_null() {
            ret = 12 as libc::c_int as CK_RV
        } else {
            p = cert_data as *mut libc::c_uchar;
            ret = 0 as libc::c_int as CK_RV;
            cert_length = i2d_X509(cert, &mut p) as size_t;
            if cert_length <= 0 as libc::c_int as libc::c_ulong {
                free(cert_data);
                cert_data = 0 as *mut libc::c_void;
                ret = 22 as libc::c_int as CK_RV
            }
        }
    }
    if !(ret != 0) {
        let mut p_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        issuer_length =
            i2d_X509_NAME(X509_get_issuer_name(cert),
                          0 as *mut *mut libc::c_uchar) as size_t;
        if issuer_length <= 0 as libc::c_int as libc::c_ulong {
            ret = 22 as libc::c_int as CK_RV
        } else {
            issuer_data = malloc(issuer_length);
            if issuer_data.is_null() {
                ret = 12 as libc::c_int as CK_RV
            } else {
                p_0 = issuer_data as *mut libc::c_uchar;
                ret = 0 as libc::c_int as CK_RV;
                issuer_length =
                    i2d_X509_NAME(X509_get_issuer_name(cert), &mut p_0) as
                        size_t;
                if issuer_length <= 0 as libc::c_int as libc::c_ulong {
                    free(issuer_data);
                    issuer_data = 0 as *mut libc::c_void;
                    ret = 22 as libc::c_int as CK_RV
                }
            }
        }
        if !(ret != 0) {
            let mut p_1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            subject_length =
                i2d_X509_NAME(X509_get_subject_name(cert),
                              0 as *mut *mut libc::c_uchar) as size_t;
            if subject_length <= 0 as libc::c_int as libc::c_ulong {
                ret = 22 as libc::c_int as CK_RV
            } else {
                subject_data = malloc(subject_length);
                if subject_data.is_null() {
                    ret = 12 as libc::c_int as CK_RV
                } else {
                    p_1 = subject_data as *mut libc::c_uchar;
                    ret = 0 as libc::c_int as CK_RV;
                    subject_length =
                        i2d_X509_NAME(X509_get_subject_name(cert), &mut p_1)
                            as size_t;
                    if subject_length <= 0 as libc::c_int as libc::c_ulong {
                        free(subject_data);
                        subject_data = 0 as *mut libc::c_void;
                        ret = 22 as libc::c_int as CK_RV
                    }
                }
            }
            if !(ret != 0) {
                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                serial_length =
                    i2d_ASN1_INTEGER(X509_get_serialNumber(cert),
                                     0 as *mut *mut libc::c_uchar) as size_t;
                if serial_length <= 0 as libc::c_int as libc::c_ulong {
                    ret = 22 as libc::c_int as CK_RV
                } else {
                    serial_data = malloc(serial_length);
                    if serial_data.is_null() {
                        ret = 12 as libc::c_int as CK_RV
                    } else {
                        p_2 = serial_data as *mut libc::c_uchar;
                        ret = 0 as libc::c_int as CK_RV;
                        serial_length =
                            i2d_ASN1_INTEGER(X509_get_serialNumber(cert),
                                             &mut p_2) as size_t;
                        if serial_length <= 0 as libc::c_int as libc::c_ulong
                           {
                            free(serial_data);
                            serial_data = 0 as *mut libc::c_void;
                            ret = 22 as libc::c_int as CK_RV
                        }
                    }
                }
                if !(ret != 0) {
                    st_logf(b"done parsing, adding to internal structure\n\x00"
                                as *const u8 as *const libc::c_char);
                    o = add_st_object();
                    if o.is_null() {
                        ret = 0x31 as libc::c_int as CK_RV
                    } else {
                        (*o).type_0 = STO_T_CERTIFICATE;
                        (*o).u.cert = X509_dup(cert);
                        if (*o).u.cert.is_null() {
                            ret = 0x31 as libc::c_int as CK_RV
                        } else {
                            public_key = X509_get_pubkey((*o).u.cert);
                            match EVP_PKEY_base_id(public_key) {
                                6 => {
                                    key_type =
                                        0 as libc::c_int as CK_KEY_TYPE;
                                    current_block = 18002345992382212654;
                                }
                                116 => {
                                    key_type =
                                        1 as libc::c_int as CK_KEY_TYPE;
                                    current_block = 18002345992382212654;
                                }
                                _ => {
                                    st_logf(b"invalid key_type\n\x00" as
                                                *const u8 as
                                                *const libc::c_char);
                                    ret = 5 as libc::c_int as CK_RV;
                                    current_block = 13528689403053953748;
                                }
                            }
                            match current_block {
                                13528689403053953748 => { }
                                _ => {
                                    c = 1 as libc::c_int as CK_OBJECT_CLASS;
                                    add_object_attribute(o, 0 as libc::c_int,
                                                         0 as libc::c_int as
                                                             CK_ATTRIBUTE_TYPE,
                                                         &mut c as
                                                             *mut CK_OBJECT_CLASS
                                                             as CK_VOID_PTR,
                                                         ::std::mem::size_of::<CK_OBJECT_CLASS>()
                                                             as
                                                             libc::c_ulong);
                                    add_object_attribute(o, 0 as libc::c_int,
                                                         1 as libc::c_int as
                                                             CK_ATTRIBUTE_TYPE,
                                                         &mut bool_true as
                                                             *mut CK_BBOOL as
                                                             CK_VOID_PTR,
                                                         ::std::mem::size_of::<CK_BBOOL>()
                                                             as
                                                             libc::c_ulong);
                                    add_object_attribute(o, 0 as libc::c_int,
                                                         2 as libc::c_int as
                                                             CK_ATTRIBUTE_TYPE,
                                                         &mut bool_false as
                                                             *mut CK_BBOOL as
                                                             CK_VOID_PTR,
                                                         ::std::mem::size_of::<CK_BBOOL>()
                                                             as
                                                             libc::c_ulong);
                                    add_object_attribute(o, 0 as libc::c_int,
                                                         0x170 as libc::c_int
                                                             as
                                                             CK_ATTRIBUTE_TYPE,
                                                         &mut bool_false as
                                                             *mut CK_BBOOL as
                                                             CK_VOID_PTR,
                                                         ::std::mem::size_of::<CK_BBOOL>()
                                                             as
                                                             libc::c_ulong);
                                    add_object_attribute(o, 0 as libc::c_int,
                                                         3 as libc::c_int as
                                                             CK_ATTRIBUTE_TYPE,
                                                         label as CK_VOID_PTR,
                                                         strlen(label));
                                    add_object_attribute(o, 0 as libc::c_int,
                                                         0x80 as libc::c_int
                                                             as
                                                             CK_ATTRIBUTE_TYPE,
                                                         &mut cert_type as
                                                             *mut CK_CERTIFICATE_TYPE
                                                             as CK_VOID_PTR,
                                                         ::std::mem::size_of::<CK_CERTIFICATE_TYPE>()
                                                             as
                                                             libc::c_ulong);
                                    add_object_attribute(o, 0 as libc::c_int,
                                                         0x102 as libc::c_int
                                                             as
                                                             CK_ATTRIBUTE_TYPE,
                                                         id as CK_VOID_PTR,
                                                         id_len);
                                    add_object_attribute(o, 0 as libc::c_int,
                                                         0x101 as libc::c_int
                                                             as
                                                             CK_ATTRIBUTE_TYPE,
                                                         subject_data,
                                                         subject_length);
                                    add_object_attribute(o, 0 as libc::c_int,
                                                         0x81 as libc::c_int
                                                             as
                                                             CK_ATTRIBUTE_TYPE,
                                                         issuer_data,
                                                         issuer_length);
                                    add_object_attribute(o, 0 as libc::c_int,
                                                         0x82 as libc::c_int
                                                             as
                                                             CK_ATTRIBUTE_TYPE,
                                                         serial_data,
                                                         serial_length);
                                    add_object_attribute(o, 0 as libc::c_int,
                                                         0x11 as libc::c_int
                                                             as
                                                             CK_ATTRIBUTE_TYPE,
                                                         cert_data,
                                                         cert_length);
                                    if anchor != 0 {
                                        add_object_attribute(o,
                                                             0 as libc::c_int,
                                                             0x86 as
                                                                 libc::c_int
                                                                 as
                                                                 CK_ATTRIBUTE_TYPE,
                                                             &mut bool_true as
                                                                 *mut CK_BBOOL
                                                                 as
                                                                 CK_VOID_PTR,
                                                             ::std::mem::size_of::<CK_BBOOL>()
                                                                 as
                                                                 libc::c_ulong);
                                    } else {
                                        add_object_attribute(o,
                                                             0 as libc::c_int,
                                                             0x86 as
                                                                 libc::c_int
                                                                 as
                                                                 CK_ATTRIBUTE_TYPE,
                                                             &mut bool_false
                                                                 as
                                                                 *mut CK_BBOOL
                                                                 as
                                                                 CK_VOID_PTR,
                                                             ::std::mem::size_of::<CK_BBOOL>()
                                                                 as
                                                                 libc::c_ulong);
                                    }
                                    st_logf(b"add cert ok: %lx\n\x00" as
                                                *const u8 as
                                                *const libc::c_char,
                                            (*o).object_handle &
                                                0xfff as libc::c_int as
                                                    libc::c_ulong);
                                    o = add_st_object();
                                    if o.is_null() {
                                        ret = 0x31 as libc::c_int as CK_RV
                                    } else {
                                        (*o).type_0 = STO_T_PUBLIC_KEY;
                                        (*o).u.public_key = public_key;
                                        c =
                                            2 as libc::c_int as
                                                CK_OBJECT_CLASS;
                                        add_object_attribute(o,
                                                             0 as libc::c_int,
                                                             0 as libc::c_int
                                                                 as
                                                                 CK_ATTRIBUTE_TYPE,
                                                             &mut c as
                                                                 *mut CK_OBJECT_CLASS
                                                                 as
                                                                 CK_VOID_PTR,
                                                             ::std::mem::size_of::<CK_OBJECT_CLASS>()
                                                                 as
                                                                 libc::c_ulong);
                                        add_object_attribute(o,
                                                             0 as libc::c_int,
                                                             1 as libc::c_int
                                                                 as
                                                                 CK_ATTRIBUTE_TYPE,
                                                             &mut bool_true as
                                                                 *mut CK_BBOOL
                                                                 as
                                                                 CK_VOID_PTR,
                                                             ::std::mem::size_of::<CK_BBOOL>()
                                                                 as
                                                                 libc::c_ulong);
                                        add_object_attribute(o,
                                                             0 as libc::c_int,
                                                             2 as libc::c_int
                                                                 as
                                                                 CK_ATTRIBUTE_TYPE,
                                                             &mut bool_false
                                                                 as
                                                                 *mut CK_BBOOL
                                                                 as
                                                                 CK_VOID_PTR,
                                                             ::std::mem::size_of::<CK_BBOOL>()
                                                                 as
                                                                 libc::c_ulong);
                                        add_object_attribute(o,
                                                             0 as libc::c_int,
                                                             0x170 as
                                                                 libc::c_int
                                                                 as
                                                                 CK_ATTRIBUTE_TYPE,
                                                             &mut bool_false
                                                                 as
                                                                 *mut CK_BBOOL
                                                                 as
                                                                 CK_VOID_PTR,
                                                             ::std::mem::size_of::<CK_BBOOL>()
                                                                 as
                                                                 libc::c_ulong);
                                        add_object_attribute(o,
                                                             0 as libc::c_int,
                                                             3 as libc::c_int
                                                                 as
                                                                 CK_ATTRIBUTE_TYPE,
                                                             label as
                                                                 CK_VOID_PTR,
                                                             strlen(label));
                                        add_object_attribute(o,
                                                             0 as libc::c_int,
                                                             0x100 as
                                                                 libc::c_int
                                                                 as
                                                                 CK_ATTRIBUTE_TYPE,
                                                             &mut key_type as
                                                                 *mut CK_KEY_TYPE
                                                                 as
                                                                 CK_VOID_PTR,
                                                             ::std::mem::size_of::<CK_KEY_TYPE>()
                                                                 as
                                                                 libc::c_ulong);
                                        add_object_attribute(o,
                                                             0 as libc::c_int,
                                                             0x102 as
                                                                 libc::c_int
                                                                 as
                                                                 CK_ATTRIBUTE_TYPE,
                                                             id as
                                                                 CK_VOID_PTR,
                                                             id_len);
                                        add_object_attribute(o,
                                                             0 as libc::c_int,
                                                             0x110 as
                                                                 libc::c_int
                                                                 as
                                                                 CK_ATTRIBUTE_TYPE,
                                                             b"\x00" as
                                                                 *const u8 as
                                                                 *const libc::c_char
                                                                 as
                                                                 CK_VOID_PTR,
                                                             1 as libc::c_int
                                                                 as CK_ULONG);
                                        add_object_attribute(o,
                                                             0 as libc::c_int,
                                                             0x111 as
                                                                 libc::c_int
                                                                 as
                                                                 CK_ATTRIBUTE_TYPE,
                                                             b"\x00" as
                                                                 *const u8 as
                                                                 *const libc::c_char
                                                                 as
                                                                 CK_VOID_PTR,
                                                             1 as libc::c_int
                                                                 as CK_ULONG);
                                        add_object_attribute(o,
                                                             0 as libc::c_int,
                                                             0x10c as
                                                                 libc::c_int
                                                                 as
                                                                 CK_ATTRIBUTE_TYPE,
                                                             &mut bool_false
                                                                 as
                                                                 *mut CK_BBOOL
                                                                 as
                                                                 CK_VOID_PTR,
                                                             ::std::mem::size_of::<CK_BBOOL>()
                                                                 as
                                                                 libc::c_ulong);
                                        add_object_attribute(o,
                                                             0 as libc::c_int,
                                                             0x163 as
                                                                 libc::c_int
                                                                 as
                                                                 CK_ATTRIBUTE_TYPE,
                                                             &mut bool_false
                                                                 as
                                                                 *mut CK_BBOOL
                                                                 as
                                                                 CK_VOID_PTR,
                                                             ::std::mem::size_of::<CK_BBOOL>()
                                                                 as
                                                                 libc::c_ulong);
                                        mech_type =
                                            3 as libc::c_int as
                                                CK_MECHANISM_TYPE;
                                        add_object_attribute(o,
                                                             0 as libc::c_int,
                                                             0x166 as
                                                                 libc::c_int
                                                                 as
                                                                 CK_ATTRIBUTE_TYPE,
                                                             &mut mech_type as
                                                                 *mut CK_MECHANISM_TYPE
                                                                 as
                                                                 CK_VOID_PTR,
                                                             ::std::mem::size_of::<CK_MECHANISM_TYPE>()
                                                                 as
                                                                 libc::c_ulong);
                                        add_object_attribute(o,
                                                             0 as libc::c_int,
                                                             0x101 as
                                                                 libc::c_int
                                                                 as
                                                                 CK_ATTRIBUTE_TYPE,
                                                             subject_data,
                                                             subject_length);
                                        add_object_attribute(o,
                                                             0 as libc::c_int,
                                                             0x104 as
                                                                 libc::c_int
                                                                 as
                                                                 CK_ATTRIBUTE_TYPE,
                                                             &mut bool_true as
                                                                 *mut CK_BBOOL
                                                                 as
                                                                 CK_VOID_PTR,
                                                             ::std::mem::size_of::<CK_BBOOL>()
                                                                 as
                                                                 libc::c_ulong);
                                        add_object_attribute(o,
                                                             0 as libc::c_int,
                                                             0x10a as
                                                                 libc::c_int
                                                                 as
                                                                 CK_ATTRIBUTE_TYPE,
                                                             &mut bool_true as
                                                                 *mut CK_BBOOL
                                                                 as
                                                                 CK_VOID_PTR,
                                                             ::std::mem::size_of::<CK_BBOOL>()
                                                                 as
                                                                 libc::c_ulong);
                                        add_object_attribute(o,
                                                             0 as libc::c_int,
                                                             0x10b as
                                                                 libc::c_int
                                                                 as
                                                                 CK_ATTRIBUTE_TYPE,
                                                             &mut bool_false
                                                                 as
                                                                 *mut CK_BBOOL
                                                                 as
                                                                 CK_VOID_PTR,
                                                             ::std::mem::size_of::<CK_BBOOL>()
                                                                 as
                                                                 libc::c_ulong);
                                        add_object_attribute(o,
                                                             0 as libc::c_int,
                                                             0x106 as
                                                                 libc::c_int
                                                                 as
                                                                 CK_ATTRIBUTE_TYPE,
                                                             &mut bool_true as
                                                                 *mut CK_BBOOL
                                                                 as
                                                                 CK_VOID_PTR,
                                                             ::std::mem::size_of::<CK_BBOOL>()
                                                                 as
                                                                 libc::c_ulong);
                                        add_object_attribute(o,
                                                             0 as libc::c_int,
                                                             0x86 as
                                                                 libc::c_int
                                                                 as
                                                                 CK_ATTRIBUTE_TYPE,
                                                             &mut bool_true as
                                                                 *mut CK_BBOOL
                                                                 as
                                                                 CK_VOID_PTR,
                                                             ::std::mem::size_of::<CK_BBOOL>()
                                                                 as
                                                                 libc::c_ulong);
                                        add_pubkey_info(o, key_type,
                                                        public_key);
                                        st_logf(b"add key ok: %lx\n\x00" as
                                                    *const u8 as
                                                    *const libc::c_char,
                                                (*o).object_handle &
                                                    0xfff as libc::c_int as
                                                        libc::c_ulong);
                                        if !private_key_file.is_null() {
                                            let mut flags: CK_FLAGS = 0;
                                            let mut f_0: *mut FILE =
                                                0 as *mut FILE;
                                            o = add_st_object();
                                            if o.is_null() {
                                                ret =
                                                    0x31 as libc::c_int as
                                                        CK_RV;
                                                current_block =
                                                    13528689403053953748;
                                            } else {
                                                (*o).type_0 =
                                                    STO_T_PRIVATE_KEY;
                                                (*o).u.private_key.file =
                                                    strdup(private_key_file);
                                                (*o).u.private_key.key =
                                                    0 as *mut EVP_PKEY;
                                                (*o).u.private_key.cert =
                                                    X509_dup(cert);
                                                if (*o).u.private_key.cert.is_null()
                                                   {
                                                    ret =
                                                        0x31 as libc::c_int as
                                                            CK_RV;
                                                    current_block =
                                                        13528689403053953748;
                                                } else {
                                                    c =
                                                        3 as libc::c_int as
                                                            CK_OBJECT_CLASS;
                                                    add_object_attribute(o,
                                                                         0 as
                                                                             libc::c_int,
                                                                         0 as
                                                                             libc::c_int
                                                                             as
                                                                             CK_ATTRIBUTE_TYPE,
                                                                         &mut c
                                                                             as
                                                                             *mut CK_OBJECT_CLASS
                                                                             as
                                                                             CK_VOID_PTR,
                                                                         ::std::mem::size_of::<CK_OBJECT_CLASS>()
                                                                             as
                                                                             libc::c_ulong);
                                                    add_object_attribute(o,
                                                                         0 as
                                                                             libc::c_int,
                                                                         1 as
                                                                             libc::c_int
                                                                             as
                                                                             CK_ATTRIBUTE_TYPE,
                                                                         &mut bool_true
                                                                             as
                                                                             *mut CK_BBOOL
                                                                             as
                                                                             CK_VOID_PTR,
                                                                         ::std::mem::size_of::<CK_BBOOL>()
                                                                             as
                                                                             libc::c_ulong);
                                                    add_object_attribute(o,
                                                                         0 as
                                                                             libc::c_int,
                                                                         2 as
                                                                             libc::c_int
                                                                             as
                                                                             CK_ATTRIBUTE_TYPE,
                                                                         &mut bool_true
                                                                             as
                                                                             *mut CK_BBOOL
                                                                             as
                                                                             CK_VOID_PTR,
                                                                         ::std::mem::size_of::<CK_BBOOL>()
                                                                             as
                                                                             libc::c_ulong);
                                                    add_object_attribute(o,
                                                                         0 as
                                                                             libc::c_int,
                                                                         0x170
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             CK_ATTRIBUTE_TYPE,
                                                                         &mut bool_false
                                                                             as
                                                                             *mut CK_BBOOL
                                                                             as
                                                                             CK_VOID_PTR,
                                                                         ::std::mem::size_of::<CK_BBOOL>()
                                                                             as
                                                                             libc::c_ulong);
                                                    add_object_attribute(o,
                                                                         0 as
                                                                             libc::c_int,
                                                                         3 as
                                                                             libc::c_int
                                                                             as
                                                                             CK_ATTRIBUTE_TYPE,
                                                                         label
                                                                             as
                                                                             CK_VOID_PTR,
                                                                         strlen(label));
                                                    add_object_attribute(o,
                                                                         0 as
                                                                             libc::c_int,
                                                                         0x100
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             CK_ATTRIBUTE_TYPE,
                                                                         &mut key_type
                                                                             as
                                                                             *mut CK_KEY_TYPE
                                                                             as
                                                                             CK_VOID_PTR,
                                                                         ::std::mem::size_of::<CK_KEY_TYPE>()
                                                                             as
                                                                             libc::c_ulong);
                                                    add_object_attribute(o,
                                                                         0 as
                                                                             libc::c_int,
                                                                         0x102
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             CK_ATTRIBUTE_TYPE,
                                                                         id as
                                                                             CK_VOID_PTR,
                                                                         id_len);
                                                    add_object_attribute(o,
                                                                         0 as
                                                                             libc::c_int,
                                                                         0x110
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             CK_ATTRIBUTE_TYPE,
                                                                         b"\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char
                                                                             as
                                                                             CK_VOID_PTR,
                                                                         1 as
                                                                             libc::c_int
                                                                             as
                                                                             CK_ULONG);
                                                    add_object_attribute(o,
                                                                         0 as
                                                                             libc::c_int,
                                                                         0x111
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             CK_ATTRIBUTE_TYPE,
                                                                         b"\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char
                                                                             as
                                                                             CK_VOID_PTR,
                                                                         1 as
                                                                             libc::c_int
                                                                             as
                                                                             CK_ULONG);
                                                    add_object_attribute(o,
                                                                         0 as
                                                                             libc::c_int,
                                                                         0x10c
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             CK_ATTRIBUTE_TYPE,
                                                                         &mut bool_false
                                                                             as
                                                                             *mut CK_BBOOL
                                                                             as
                                                                             CK_VOID_PTR,
                                                                         ::std::mem::size_of::<CK_BBOOL>()
                                                                             as
                                                                             libc::c_ulong);
                                                    add_object_attribute(o,
                                                                         0 as
                                                                             libc::c_int,
                                                                         0x163
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             CK_ATTRIBUTE_TYPE,
                                                                         &mut bool_false
                                                                             as
                                                                             *mut CK_BBOOL
                                                                             as
                                                                             CK_VOID_PTR,
                                                                         ::std::mem::size_of::<CK_BBOOL>()
                                                                             as
                                                                             libc::c_ulong);
                                                    mech_type =
                                                        3 as libc::c_int as
                                                            CK_MECHANISM_TYPE;
                                                    add_object_attribute(o,
                                                                         0 as
                                                                             libc::c_int,
                                                                         0x166
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             CK_ATTRIBUTE_TYPE,
                                                                         &mut mech_type
                                                                             as
                                                                             *mut CK_MECHANISM_TYPE
                                                                             as
                                                                             CK_VOID_PTR,
                                                                         ::std::mem::size_of::<CK_MECHANISM_TYPE>()
                                                                             as
                                                                             libc::c_ulong);
                                                    add_object_attribute(o,
                                                                         0 as
                                                                             libc::c_int,
                                                                         0x101
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             CK_ATTRIBUTE_TYPE,
                                                                         subject_data,
                                                                         subject_length);
                                                    add_object_attribute(o,
                                                                         0 as
                                                                             libc::c_int,
                                                                         0x103
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             CK_ATTRIBUTE_TYPE,
                                                                         &mut bool_true
                                                                             as
                                                                             *mut CK_BBOOL
                                                                             as
                                                                             CK_VOID_PTR,
                                                                         ::std::mem::size_of::<CK_BBOOL>()
                                                                             as
                                                                             libc::c_ulong);
                                                    add_object_attribute(o,
                                                                         0 as
                                                                             libc::c_int,
                                                                         0x200
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             CK_ATTRIBUTE_TYPE,
                                                                         &mut bool_false
                                                                             as
                                                                             *mut CK_BBOOL
                                                                             as
                                                                             CK_VOID_PTR,
                                                                         ::std::mem::size_of::<CK_BBOOL>()
                                                                             as
                                                                             libc::c_ulong);
                                                    flags =
                                                        0 as libc::c_int as
                                                            CK_FLAGS;
                                                    add_object_attribute(o,
                                                                         0 as
                                                                             libc::c_int,
                                                                         0x201
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             CK_ATTRIBUTE_TYPE,
                                                                         &mut flags
                                                                             as
                                                                             *mut CK_FLAGS
                                                                             as
                                                                             CK_VOID_PTR,
                                                                         ::std::mem::size_of::<CK_FLAGS>()
                                                                             as
                                                                             libc::c_ulong);
                                                    add_object_attribute(o,
                                                                         0 as
                                                                             libc::c_int,
                                                                         0x105
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             CK_ATTRIBUTE_TYPE,
                                                                         &mut bool_true
                                                                             as
                                                                             *mut CK_BBOOL
                                                                             as
                                                                             CK_VOID_PTR,
                                                                         ::std::mem::size_of::<CK_BBOOL>()
                                                                             as
                                                                             libc::c_ulong);
                                                    add_object_attribute(o,
                                                                         0 as
                                                                             libc::c_int,
                                                                         0x108
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             CK_ATTRIBUTE_TYPE,
                                                                         &mut bool_true
                                                                             as
                                                                             *mut CK_BBOOL
                                                                             as
                                                                             CK_VOID_PTR,
                                                                         ::std::mem::size_of::<CK_BBOOL>()
                                                                             as
                                                                             libc::c_ulong);
                                                    add_object_attribute(o,
                                                                         0 as
                                                                             libc::c_int,
                                                                         0x109
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             CK_ATTRIBUTE_TYPE,
                                                                         &mut bool_false
                                                                             as
                                                                             *mut CK_BBOOL
                                                                             as
                                                                             CK_VOID_PTR,
                                                                         ::std::mem::size_of::<CK_BBOOL>()
                                                                             as
                                                                             libc::c_ulong);
                                                    add_object_attribute(o,
                                                                         0 as
                                                                             libc::c_int,
                                                                         0x107
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             CK_ATTRIBUTE_TYPE,
                                                                         &mut bool_true
                                                                             as
                                                                             *mut CK_BBOOL
                                                                             as
                                                                             CK_VOID_PTR,
                                                                         ::std::mem::size_of::<CK_BBOOL>()
                                                                             as
                                                                             libc::c_ulong);
                                                    add_object_attribute(o,
                                                                         0 as
                                                                             libc::c_int,
                                                                         0x162
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             CK_ATTRIBUTE_TYPE,
                                                                         &mut bool_true
                                                                             as
                                                                             *mut CK_BBOOL
                                                                             as
                                                                             CK_VOID_PTR,
                                                                         ::std::mem::size_of::<CK_BBOOL>()
                                                                             as
                                                                             libc::c_ulong);
                                                    add_object_attribute(o,
                                                                         0 as
                                                                             libc::c_int,
                                                                         0x164
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             CK_ATTRIBUTE_TYPE,
                                                                         &mut bool_false
                                                                             as
                                                                             *mut CK_BBOOL
                                                                             as
                                                                             CK_VOID_PTR,
                                                                         ::std::mem::size_of::<CK_BBOOL>()
                                                                             as
                                                                             libc::c_ulong);
                                                    add_pubkey_info(o,
                                                                    key_type,
                                                                    public_key);
                                                    f_0 =
                                                        fopen(private_key_file,
                                                              b"r\x00" as
                                                                  *const u8 as
                                                                  *const libc::c_char);
                                                    if f_0.is_null() {
                                                        st_logf(b"failed to open private key\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char);
                                                        return 5 as
                                                                   libc::c_int
                                                                   as CK_RV
                                                    }
                                                    (*o).u.private_key.key =
                                                        PEM_read_PrivateKey(f_0,
                                                                            0
                                                                                as
                                                                                *mut *mut EVP_PKEY,
                                                                            Some(pem_callback
                                                                                     as
                                                                                     unsafe extern "C" fn(_:
                                                                                                              *mut libc::c_char,
                                                                                                          _:
                                                                                                              libc::c_int,
                                                                                                          _:
                                                                                                              libc::c_int,
                                                                                                          _:
                                                                                                              *mut libc::c_void)
                                                                                         ->
                                                                                             libc::c_int),
                                                                            0
                                                                                as
                                                                                *mut libc::c_void);
                                                    fclose(f_0);
                                                    if (*o).u.private_key.key.is_null()
                                                       {
                                                        st_logf(b"failed to read private key a startup\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char);
                                                    } else {
                                                        /* XXX verify keytype */
                                                        if key_type ==
                                                               0 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong
                                                           {
                                                            RSA_set_method(EVP_PKEY_get0_RSA((*o).u.private_key.key),
                                                                           RSA_PKCS1_OpenSSL());
                                                        }
                                                        if X509_check_private_key(cert,
                                                                                  (*o).u.private_key.key)
                                                               !=
                                                               1 as
                                                                   libc::c_int
                                                           {
                                                            EVP_PKEY_free((*o).u.private_key.key);
                                                            (*o).u.private_key.key
                                                                =
                                                                0 as
                                                                    *mut EVP_PKEY;
                                                            st_logf(b"private key doesn\'t verify\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
                                                        } else {
                                                            st_logf(b"private key usable\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
                                                            soft_token.flags.login_done
                                                                =
                                                                1 as
                                                                    libc::c_int
                                                        }
                                                    }
                                                    current_block =
                                                        13526015532137226550;
                                                }
                                            }
                                        } else {
                                            current_block =
                                                13526015532137226550;
                                        }
                                        match current_block {
                                            13528689403053953748 => { }
                                            _ => {
                                                ret =
                                                    0 as libc::c_int as CK_RV
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if ret != 0 as libc::c_int as libc::c_ulong {
        st_logf(b"something went wrong when adding cert!\n\x00" as *const u8
                    as *const libc::c_char);
    }
    free(cert_data);
    free(serial_data);
    free(issuer_data);
    free(subject_data);
    X509_free(cert);
    return ret;
}
#[c2rust::src_loc = "713:1"]
unsafe extern "C" fn find_object_final(mut state: *mut session_state) {
    if !(*state).find.attributes.is_null() {
        let mut i: CK_ULONG = 0;
        i = 0 as libc::c_int as CK_ULONG;
        while i < (*state).find.num_attributes {
            if !(*(*state).find.attributes.offset(i as
                                                      isize)).pValue.is_null()
               {
                free((*(*state).find.attributes.offset(i as isize)).pValue);
            }
            i = i.wrapping_add(1)
        }
        free((*state).find.attributes as *mut libc::c_void);
        (*state).find.attributes = 0 as *mut CK_ATTRIBUTE;
        (*state).find.num_attributes = 0 as libc::c_int as CK_ULONG;
        (*state).find.next_object = -(1 as libc::c_int)
    };
}
#[c2rust::src_loc = "730:1"]
unsafe extern "C" fn reset_crypto_state(mut state: *mut session_state) {
    (*state).encrypt_object = -(1 as libc::c_int);
    if !(*state).encrypt_mechanism.is_null() {
        free((*state).encrypt_mechanism as *mut libc::c_void);
    }
    (*state).encrypt_mechanism = 0 as CK_MECHANISM_PTR;
    (*state).decrypt_object = -(1 as libc::c_int);
    if !(*state).decrypt_mechanism.is_null() {
        free((*state).decrypt_mechanism as *mut libc::c_void);
    }
    (*state).decrypt_mechanism = 0 as CK_MECHANISM_PTR;
    (*state).sign_object = -(1 as libc::c_int);
    if !(*state).sign_mechanism.is_null() {
        free((*state).sign_mechanism as *mut libc::c_void);
    }
    (*state).sign_mechanism = 0 as CK_MECHANISM_PTR;
    (*state).verify_object = -(1 as libc::c_int);
    if !(*state).verify_mechanism.is_null() {
        free((*state).verify_mechanism as *mut libc::c_void);
    }
    (*state).verify_mechanism = 0 as CK_MECHANISM_PTR;
    (*state).digest_object = -(1 as libc::c_int);
}
#[c2rust::src_loc = "752:1"]
unsafe extern "C" fn close_session(mut state: *mut session_state) {
    if !(*state).find.attributes.is_null() {
        application_error(b"application didn\'t do C_FindObjectsFinal\n\x00"
                              as *const u8 as *const libc::c_char);
        find_object_final(state);
    }
    (*state).session_handle = 0 as libc::c_int as CK_SESSION_HANDLE;
    soft_token.application = 0 as *mut libc::c_void;
    soft_token.notify = None;
    reset_crypto_state(state);
}
#[c2rust::src_loc = "766:1"]
unsafe extern "C" fn has_session() -> *const libc::c_char {
    return if soft_token.open_sessions > 0 as libc::c_int {
               b"yes\x00" as *const u8 as *const libc::c_char
           } else { b"no\x00" as *const u8 as *const libc::c_char };
}
#[c2rust::src_loc = "772:1"]
unsafe extern "C" fn read_conf_file(mut fn_0: *const libc::c_char) {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut cert: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut id: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut label: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut anchor: libc::c_int = 0;
    let mut f: *mut FILE = 0 as *mut FILE;
    f = fopen(fn_0, b"r\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        st_logf(b"can\'t open configuration file %s\n\x00" as *const u8 as
                    *const libc::c_char, fn_0);
        return
    }
    while !fgets(buf.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 1024]>() as
                     libc::c_ulong as libc::c_int, f).is_null() {
        buf[strcspn(buf.as_mut_ptr(),
                    b"\n\x00" as *const u8 as *const libc::c_char) as usize] =
            '\u{0}' as i32 as libc::c_char;
        anchor = 0 as libc::c_int;
        st_logf(b"line: %s\n\x00" as *const u8 as *const libc::c_char,
                buf.as_mut_ptr());
        p = buf.as_mut_ptr();
        while *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as
                  libc::c_int &
                  _ISspace as libc::c_int as libc::c_ushort as libc::c_int !=
                  0 {
            p = p.offset(1)
        }
        if *p as libc::c_int == '#' as i32 { continue ; }
        while *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as
                  libc::c_int &
                  _ISspace as libc::c_int as libc::c_ushort as libc::c_int !=
                  0 {
            p = p.offset(1)
        }
        s = 0 as *mut libc::c_char;
        id =
            strtok_r(p, b"\t\x00" as *const u8 as *const libc::c_char,
                     &mut s);
        if id.is_null() { continue ; }
        label =
            strtok_r(0 as *mut libc::c_char,
                     b"\t\x00" as *const u8 as *const libc::c_char, &mut s);
        if label.is_null() { continue ; }
        cert =
            strtok_r(0 as *mut libc::c_char,
                     b"\t\x00" as *const u8 as *const libc::c_char, &mut s);
        if cert.is_null() { continue ; }
        key =
            strtok_r(0 as *mut libc::c_char,
                     b"\t\x00" as *const u8 as *const libc::c_char, &mut s);
        /* XXX */
        if strcmp(id, b"anchor\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
            id =
                b"\x00\x00\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char;
            anchor = 1 as libc::c_int
        }
        st_logf(b"adding: %s\n\x00" as *const u8 as *const libc::c_char,
                label);
        add_certificate(label, cert, key, id, anchor);
    }
    fclose(f);
}
#[c2rust::src_loc = "826:1"]
unsafe extern "C" fn func_not_supported() -> CK_RV {
    st_logf(b"function not supported\n\x00" as *const u8 as
                *const libc::c_char);
    return 0x54 as libc::c_int as CK_RV;
}
#[c2rust::src_loc = "833:1"]
unsafe extern "C" fn get_rcfilename() -> *mut libc::c_char {
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut home: *const libc::c_char = 0 as *const libc::c_char;
    let mut fn_0: *mut libc::c_char = 0 as *mut libc::c_char;
    if getuid() == geteuid() {
        fn_0 =
            getenv(b"SOFTPKCS11RC\x00" as *const u8 as *const libc::c_char);
        if !fn_0.is_null() { return strdup(fn_0) }
        home = getenv(b"HOME\x00" as *const u8 as *const libc::c_char)
    }
    if home.is_null() {
        pw = getpwuid(getuid());
        if !pw.is_null() { home = (*pw).pw_dir }
    }
    if home.is_null() {
        return strdup(b"/etc/soft-token.rc\x00" as *const u8 as
                          *const libc::c_char)
    }
    if asprintf(&mut fn_0 as *mut *mut libc::c_char,
                b"%s/.soft-token.rc\x00" as *const u8 as *const libc::c_char,
                home) < 0 as libc::c_int {
        return 0 as *mut libc::c_char
    }
    return fn_0;
}
/* Forward reference.  */
#[no_mangle]
#[c2rust::src_loc = "862:1"]
pub unsafe extern "C" fn C_Initialize(mut a: CK_VOID_PTR) -> CK_RV {
    let mut args: CK_C_INITIALIZE_ARGS_PTR = a as CK_C_INITIALIZE_ARGS_PTR;
    let mut i: size_t = 0;
    let mut fn_0: *mut libc::c_char = 0 as *mut libc::c_char;
    st_logf(b"Initialize\n\x00" as *const u8 as *const libc::c_char);
    OPENSSL_init_crypto((0x4 as libc::c_long | 0x8 as libc::c_long) as
                            uint64_t, 0 as *const OPENSSL_INIT_SETTINGS);
    OPENSSL_init_crypto(0x2 as libc::c_long as uint64_t,
                        0 as *const OPENSSL_INIT_SETTINGS);
    i = 0 as libc::c_int as size_t;
    while i <
              (::std::mem::size_of::<[session_state; 10]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<session_state>()
                                                   as libc::c_ulong) {
        soft_token.state[i as usize].session_handle =
            0 as libc::c_int as CK_SESSION_HANDLE;
        soft_token.state[i as usize].find.attributes = 0 as *mut CK_ATTRIBUTE;
        soft_token.state[i as usize].find.num_attributes =
            0 as libc::c_int as CK_ULONG;
        soft_token.state[i as usize].find.next_object = -(1 as libc::c_int);
        reset_crypto_state(&mut *soft_token.state.as_mut_ptr().offset(i as
                                                                          isize));
        i = i.wrapping_add(1)
    }
    soft_token.flags.hardware_slot = 1 as libc::c_int;
    soft_token.flags.app_error_fatal = 0 as libc::c_int;
    soft_token.flags.login_done = 0 as libc::c_int;
    soft_token.object.objs = 0 as *mut *mut st_object;
    soft_token.object.num_objs = 0 as libc::c_int;
    soft_token.logfile = 0 as *mut FILE;
    if !a.is_null() {
        st_logf(b"\tCreateMutex:\t%p\n\x00" as *const u8 as
                    *const libc::c_char, (*args).CreateMutex);
        st_logf(b"\tDestroyMutext\t%p\n\x00" as *const u8 as
                    *const libc::c_char, (*args).DestroyMutex);
        st_logf(b"\tLockMutext\t%p\n\x00" as *const u8 as *const libc::c_char,
                (*args).LockMutex);
        st_logf(b"\tUnlockMutext\t%p\n\x00" as *const u8 as
                    *const libc::c_char, (*args).UnlockMutex);
        st_logf(b"\tFlags\t%04x\n\x00" as *const u8 as *const libc::c_char,
                (*args).flags as libc::c_uint);
    }
    soft_token.next_session_handle = 1 as libc::c_int as CK_SESSION_HANDLE;
    fn_0 = get_rcfilename();
    if fn_0.is_null() { return 0x31 as libc::c_int as CK_RV }
    read_conf_file(fn_0);
    free(fn_0 as *mut libc::c_void);
    return 0 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "915:1"]
pub unsafe extern "C" fn C_Finalize(mut args: CK_VOID_PTR) -> CK_RV {
    let mut i: size_t = 0;
    let mut j: libc::c_int = 0;
    st_logf(b"Finalize\n\x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as size_t;
    while i <
              (::std::mem::size_of::<[session_state; 10]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<session_state>()
                                                   as libc::c_ulong) {
        if soft_token.state[i as usize].session_handle !=
               0 as libc::c_int as libc::c_ulong {
            application_error(b"application finalized without closing session\n\x00"
                                  as *const u8 as *const libc::c_char);
            close_session(&mut *soft_token.state.as_mut_ptr().offset(i as
                                                                         isize));
        }
        i = i.wrapping_add(1)
    }
    j = 0 as libc::c_int;
    while j < soft_token.object.num_objs {
        free_st_object(*soft_token.object.objs.offset(j as isize));
        j += 1
    }
    free(soft_token.object.objs as *mut libc::c_void);
    soft_token.object.objs = 0 as *mut *mut st_object;
    soft_token.object.num_objs = 0 as libc::c_int;
    return 0 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "940:1"]
pub unsafe extern "C" fn C_GetInfo(mut args: CK_INFO_PTR) -> CK_RV {
    st_logf(b"GetInfo\n\x00" as *const u8 as *const libc::c_char);
    memset(args as *mut libc::c_void, 17 as libc::c_int,
           ::std::mem::size_of::<_CK_INFO>() as libc::c_ulong);
    (*args).cryptokiVersion.major = 2 as libc::c_int as libc::c_uchar;
    (*args).cryptokiVersion.minor = 10 as libc::c_int as libc::c_uchar;
    snprintf_fill((*args).manufacturerID.as_mut_ptr() as *mut libc::c_char,
                  ::std::mem::size_of::<[libc::c_uchar; 32]>() as
                      libc::c_ulong as libc::c_int,
                  ' ' as i32 as libc::c_char,
                  b"SoftToken\x00" as *const u8 as *const libc::c_char);
    snprintf_fill((*args).libraryDescription.as_mut_ptr() as
                      *mut libc::c_char,
                  ::std::mem::size_of::<[libc::c_uchar; 32]>() as
                      libc::c_ulong as libc::c_int,
                  ' ' as i32 as libc::c_char,
                  b"SoftToken\x00" as *const u8 as *const libc::c_char);
    (*args).libraryVersion.major = 1 as libc::c_int as libc::c_uchar;
    (*args).libraryVersion.minor = 8 as libc::c_int as libc::c_uchar;
    return 0 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "963:1"]
pub unsafe extern "C" fn C_GetFunctionList(mut ppFunctionList:
                                               CK_FUNCTION_LIST_PTR_PTR)
 -> CK_RV {
    *ppFunctionList = &mut funcs;
    return 0 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "970:1"]
pub unsafe extern "C" fn C_GetSlotList(mut tokenPresent: CK_BBOOL,
                                       mut pSlotList: CK_SLOT_ID_PTR,
                                       mut pulCount: CK_ULONG_PTR) -> CK_RV {
    st_logf(b"GetSlotList: %s\n\x00" as *const u8 as *const libc::c_char,
            if tokenPresent as libc::c_int != 0 {
                b"tokenPresent\x00" as *const u8 as *const libc::c_char
            } else {
                b"token not Present\x00" as *const u8 as *const libc::c_char
            });
    if !pSlotList.is_null() {
        *pSlotList.offset(0 as libc::c_int as isize) =
            1 as libc::c_int as CK_SLOT_ID
    }
    *pulCount = 1 as libc::c_int as CK_ULONG;
    return 0 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "983:1"]
pub unsafe extern "C" fn C_GetSlotInfo(mut slotID: CK_SLOT_ID,
                                       mut pInfo: CK_SLOT_INFO_PTR) -> CK_RV {
    st_logf(b"GetSlotInfo: slot: %d : %s\n\x00" as *const u8 as
                *const libc::c_char, slotID as libc::c_int, has_session());
    memset(pInfo as *mut libc::c_void, 18 as libc::c_int,
           ::std::mem::size_of::<_CK_SLOT_INFO>() as libc::c_ulong);
    if slotID != 1 as libc::c_int as libc::c_ulong {
        return 7 as libc::c_int as CK_RV
    }
    snprintf_fill((*pInfo).slotDescription.as_mut_ptr() as *mut libc::c_char,
                  ::std::mem::size_of::<[libc::c_uchar; 64]>() as
                      libc::c_ulong as libc::c_int,
                  ' ' as i32 as libc::c_char,
                  b"SoftToken (slot)\x00" as *const u8 as
                      *const libc::c_char);
    snprintf_fill((*pInfo).manufacturerID.as_mut_ptr() as *mut libc::c_char,
                  ::std::mem::size_of::<[libc::c_uchar; 32]>() as
                      libc::c_ulong as libc::c_int,
                  ' ' as i32 as libc::c_char,
                  b"SoftToken (slot)\x00" as *const u8 as
                      *const libc::c_char);
    (*pInfo).flags = ((1 as libc::c_int) << 0 as libc::c_int) as CK_FLAGS;
    if soft_token.flags.hardware_slot != 0 {
        (*pInfo).flags |=
            ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong
    }
    (*pInfo).hardwareVersion.major = 1 as libc::c_int as libc::c_uchar;
    (*pInfo).hardwareVersion.minor = 0 as libc::c_int as libc::c_uchar;
    (*pInfo).firmwareVersion.major = 1 as libc::c_int as libc::c_uchar;
    (*pInfo).firmwareVersion.minor = 0 as libc::c_int as libc::c_uchar;
    return 0 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "1013:1"]
pub unsafe extern "C" fn C_GetTokenInfo(mut slotID: CK_SLOT_ID,
                                        mut pInfo: CK_TOKEN_INFO_PTR)
 -> CK_RV {
    st_logf(b"GetTokenInfo: %s\n\x00" as *const u8 as *const libc::c_char,
            has_session());
    memset(pInfo as *mut libc::c_void, 19 as libc::c_int,
           ::std::mem::size_of::<_CK_TOKEN_INFO>() as libc::c_ulong);
    snprintf_fill((*pInfo).label.as_mut_ptr() as *mut libc::c_char,
                  ::std::mem::size_of::<[libc::c_uchar; 32]>() as
                      libc::c_ulong as libc::c_int,
                  ' ' as i32 as libc::c_char,
                  b"SoftToken (token)\x00" as *const u8 as
                      *const libc::c_char);
    snprintf_fill((*pInfo).manufacturerID.as_mut_ptr() as *mut libc::c_char,
                  ::std::mem::size_of::<[libc::c_uchar; 32]>() as
                      libc::c_ulong as libc::c_int,
                  ' ' as i32 as libc::c_char,
                  b"SoftToken (token)\x00" as *const u8 as
                      *const libc::c_char);
    snprintf_fill((*pInfo).model.as_mut_ptr() as *mut libc::c_char,
                  ::std::mem::size_of::<[libc::c_uchar; 16]>() as
                      libc::c_ulong as libc::c_int,
                  ' ' as i32 as libc::c_char,
                  b"SoftToken (token)\x00" as *const u8 as
                      *const libc::c_char);
    snprintf_fill((*pInfo).serialNumber.as_mut_ptr() as *mut libc::c_char,
                  ::std::mem::size_of::<[libc::c_uchar; 16]>() as
                      libc::c_ulong as libc::c_int,
                  ' ' as i32 as libc::c_char,
                  b"4711\x00" as *const u8 as *const libc::c_char);
    (*pInfo).flags =
        ((1 as libc::c_int) << 10 as libc::c_int |
             (1 as libc::c_int) << 3 as libc::c_int) as CK_FLAGS;
    if soft_token.flags.login_done == 0 as libc::c_int {
        (*pInfo).flags |=
            ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong
    }
    /* CFK_RNG |
       CKF_RESTORE_KEY_NOT_NEEDED |
    */
    (*pInfo).ulMaxSessionCount =
        (::std::mem::size_of::<[session_state; 10]>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<session_state>()
                                             as libc::c_ulong);
    (*pInfo).ulSessionCount = soft_token.open_sessions as libc::c_ulong;
    (*pInfo).ulMaxRwSessionCount =
        (::std::mem::size_of::<[session_state; 10]>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<session_state>()
                                             as libc::c_ulong);
    (*pInfo).ulRwSessionCount = soft_token.open_sessions as libc::c_ulong;
    (*pInfo).ulMaxPinLen = 1024 as libc::c_int as libc::c_ulong;
    (*pInfo).ulMinPinLen = 0 as libc::c_int as libc::c_ulong;
    (*pInfo).ulTotalPublicMemory = 4711 as libc::c_int as libc::c_ulong;
    (*pInfo).ulFreePublicMemory = 4712 as libc::c_int as libc::c_ulong;
    (*pInfo).ulTotalPrivateMemory = 4713 as libc::c_int as libc::c_ulong;
    (*pInfo).ulFreePrivateMemory = 4714 as libc::c_int as libc::c_ulong;
    (*pInfo).hardwareVersion.major = 2 as libc::c_int as libc::c_uchar;
    (*pInfo).hardwareVersion.minor = 0 as libc::c_int as libc::c_uchar;
    (*pInfo).firmwareVersion.major = 2 as libc::c_int as libc::c_uchar;
    (*pInfo).firmwareVersion.minor = 0 as libc::c_int as libc::c_uchar;
    return 0 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "1065:1"]
pub unsafe extern "C" fn C_GetMechanismList(mut slotID: CK_SLOT_ID,
                                            mut pMechanismList:
                                                CK_MECHANISM_TYPE_PTR,
                                            mut pulCount: CK_ULONG_PTR)
 -> CK_RV {
    st_logf(b"GetMechanismList\n\x00" as *const u8 as *const libc::c_char);
    *pulCount = 2 as libc::c_int as CK_ULONG;
    if pMechanismList.is_null() { return 0 as libc::c_int as CK_RV }
    *pMechanismList.offset(0 as libc::c_int as isize) =
        3 as libc::c_int as CK_MECHANISM_TYPE;
    *pMechanismList.offset(1 as libc::c_int as isize) =
        1 as libc::c_int as CK_MECHANISM_TYPE;
    return 0 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "1081:1"]
pub unsafe extern "C" fn C_GetMechanismInfo(mut slotID: CK_SLOT_ID,
                                            mut type_0: CK_MECHANISM_TYPE,
                                            mut pInfo: CK_MECHANISM_INFO_PTR)
 -> CK_RV {
    st_logf(b"GetMechanismInfo: slot %d type: %d\n\x00" as *const u8 as
                *const libc::c_char, slotID as libc::c_int,
            type_0 as libc::c_int);
    return 0x54 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "1091:1"]
pub unsafe extern "C" fn C_InitToken(mut slotID: CK_SLOT_ID,
                                     mut pPin: CK_UTF8CHAR_PTR,
                                     mut ulPinLen: CK_ULONG,
                                     mut pLabel: CK_UTF8CHAR_PTR) -> CK_RV {
    st_logf(b"InitToken: slot %d\n\x00" as *const u8 as *const libc::c_char,
            slotID as libc::c_int);
    return 0x54 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "1101:1"]
pub unsafe extern "C" fn C_OpenSession(mut slotID: CK_SLOT_ID,
                                       mut flags: CK_FLAGS,
                                       mut pApplication: CK_VOID_PTR,
                                       mut Notify: CK_NOTIFY,
                                       mut phSession: CK_SESSION_HANDLE_PTR)
 -> CK_RV {
    let mut i: size_t = 0;
    st_logf(b"OpenSession: slot: %d\n\x00" as *const u8 as
                *const libc::c_char, slotID as libc::c_int);
    if soft_token.open_sessions as libc::c_ulong ==
           (::std::mem::size_of::<[session_state; 10]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<session_state>()
                                                as libc::c_ulong) {
        return 0xb1 as libc::c_int as CK_RV
    }
    soft_token.application = pApplication;
    soft_token.notify = Notify;
    i = 0 as libc::c_int as size_t;
    while i <
              (::std::mem::size_of::<[session_state; 10]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<session_state>()
                                                   as libc::c_ulong) {
        if soft_token.state[i as usize].session_handle ==
               0 as libc::c_int as libc::c_ulong {
            break ;
        }
        i = i.wrapping_add(1)
    }
    if i ==
           (::std::mem::size_of::<[session_state; 10]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<session_state>()
                                                as libc::c_ulong) {
        abort();
    }
    soft_token.open_sessions += 1;
    let fresh4 = soft_token.next_session_handle;
    soft_token.next_session_handle =
        soft_token.next_session_handle.wrapping_add(1);
    soft_token.state[i as usize].session_handle = fresh4;
    *phSession = soft_token.state[i as usize].session_handle;
    return 0 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "1132:1"]
pub unsafe extern "C" fn C_CloseSession(mut hSession: CK_SESSION_HANDLE)
 -> CK_RV {
    let mut state: *mut session_state = 0 as *mut session_state;
    st_logf(b"CloseSession\n\x00" as *const u8 as *const libc::c_char);
    if verify_session_handle(hSession, &mut state) !=
           0 as libc::c_int as libc::c_ulong {
        application_error(b"closed session not open\x00" as *const u8 as
                              *const libc::c_char);
    } else { close_session(state); }
    return 0 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "1146:1"]
pub unsafe extern "C" fn C_CloseAllSessions(mut slotID: CK_SLOT_ID) -> CK_RV {
    let mut i: size_t = 0;
    st_logf(b"CloseAllSessions\n\x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as size_t;
    while i <
              (::std::mem::size_of::<[session_state; 10]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<session_state>()
                                                   as libc::c_ulong) {
        if soft_token.state[i as usize].session_handle !=
               0 as libc::c_int as libc::c_ulong {
            close_session(&mut *soft_token.state.as_mut_ptr().offset(i as
                                                                         isize));
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "1160:1"]
pub unsafe extern "C" fn C_GetSessionInfo(mut hSession: CK_SESSION_HANDLE,
                                          mut pInfo: CK_SESSION_INFO_PTR)
 -> CK_RV {
    st_logf(b"GetSessionInfo\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, 0 as *mut *mut session_state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    memset(pInfo as *mut libc::c_void, 20 as libc::c_int,
           ::std::mem::size_of::<_CK_SESSION_INFO>() as libc::c_ulong);
    (*pInfo).slotID = 1 as libc::c_int as CK_SLOT_ID;
    if soft_token.flags.login_done != 0 {
        (*pInfo).state = 1 as libc::c_int as CK_STATE
    } else { (*pInfo).state = 0 as libc::c_int as CK_STATE }
    (*pInfo).flags = ((1 as libc::c_int) << 2 as libc::c_int) as CK_FLAGS;
    (*pInfo).ulDeviceError = 0 as libc::c_int as libc::c_ulong;
    return 0 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "1181:1"]
pub unsafe extern "C" fn C_Login(mut hSession: CK_SESSION_HANDLE,
                                 mut userType: CK_USER_TYPE,
                                 mut pPin: CK_UTF8CHAR_PTR,
                                 mut ulPinLen: CK_ULONG) -> CK_RV {
    let mut pin: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    st_logf(b"Login\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, 0 as *mut *mut session_state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    if !pPin.is_null() {
        if asprintf(&mut pin as *mut *mut libc::c_char,
                    b"%.*s\x00" as *const u8 as *const libc::c_char,
                    ulPinLen as libc::c_int, pPin) < 0 as libc::c_int {
            return 0x31 as libc::c_int as CK_RV
        }
        st_logf(b"type: %d password: %s\n\x00" as *const u8 as
                    *const libc::c_char, userType as libc::c_int, pin);
    }
    i = 0 as libc::c_int;
    while i < soft_token.object.num_objs {
        let mut o: *mut st_object =
            *soft_token.object.objs.offset(i as isize);
        let mut f: *mut FILE = 0 as *mut FILE;
        if !((*o).type_0 as libc::c_uint !=
                 STO_T_PRIVATE_KEY as libc::c_int as libc::c_uint) {
            if (*o).u.private_key.key.is_null() {
                f =
                    fopen((*o).u.private_key.file,
                          b"r\x00" as *const u8 as *const libc::c_char);
                if f.is_null() {
                    st_logf(b"can\'t open private file: %s\n\x00" as *const u8
                                as *const libc::c_char,
                            (*o).u.private_key.file);
                } else {
                    (*o).u.private_key.key =
                        PEM_read_PrivateKey(f, 0 as *mut *mut EVP_PKEY, None,
                                            pin as *mut libc::c_void);
                    fclose(f);
                    if (*o).u.private_key.key.is_null() {
                        st_logf(b"failed to read key: %s error: %s\n\x00" as
                                    *const u8 as *const libc::c_char,
                                (*o).u.private_key.file,
                                ERR_error_string(ERR_get_error(),
                                                 0 as *mut libc::c_char));
                    } else {
                        /* XXX check keytype */
                        RSA_set_method(EVP_PKEY_get0_RSA((*o).u.private_key.key),
                                       RSA_PKCS1_OpenSSL());
                        if X509_check_private_key((*o).u.private_key.cert,
                                                  (*o).u.private_key.key) !=
                               1 as libc::c_int {
                            EVP_PKEY_free((*o).u.private_key.key);
                            (*o).u.private_key.key = 0 as *mut EVP_PKEY;
                            st_logf(b"private key %s doesn\'t verify\n\x00" as
                                        *const u8 as *const libc::c_char,
                                    (*o).u.private_key.file);
                        } else {
                            soft_token.flags.login_done = 1 as libc::c_int
                        }
                    }
                }
            }
        }
        /* just ignore failure */
        i += 1
    } /* XXX RAND is broken while running in mozilla ? */
    free(pin as *mut libc::c_void);
    return if soft_token.flags.login_done != 0 {
               0 as libc::c_int
           } else { 0xa0 as libc::c_int } as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "1244:1"]
pub unsafe extern "C" fn C_Logout(mut hSession: CK_SESSION_HANDLE) -> CK_RV {
    st_logf(b"Logout\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, 0 as *mut *mut session_state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    return 0x54 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "1252:1"]
pub unsafe extern "C" fn C_GetObjectSize(mut hSession: CK_SESSION_HANDLE,
                                         mut hObject: CK_OBJECT_HANDLE,
                                         mut pulSize: CK_ULONG_PTR) -> CK_RV {
    st_logf(b"GetObjectSize\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, 0 as *mut *mut session_state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    return 0x54 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "1262:1"]
pub unsafe extern "C" fn C_GetAttributeValue(mut hSession: CK_SESSION_HANDLE,
                                             mut hObject: CK_OBJECT_HANDLE,
                                             mut pTemplate: CK_ATTRIBUTE_PTR,
                                             mut ulCount: CK_ULONG) -> CK_RV {
    let mut state: *mut session_state = 0 as *mut session_state;
    let mut obj: *mut st_object = 0 as *mut st_object;
    let mut i: CK_ULONG = 0;
    let mut ret: CK_RV = 0;
    let mut j: libc::c_int = 0;
    st_logf(b"GetAttributeValue: %lx\n\x00" as *const u8 as
                *const libc::c_char,
            hObject & 0xfff as libc::c_int as libc::c_ulong);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, &mut state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    ret = object_handle_to_object(hObject, &mut obj);
    if ret != 0 as libc::c_int as libc::c_ulong {
        st_logf(b"object not found: %lx\n\x00" as *const u8 as
                    *const libc::c_char,
                hObject & 0xfff as libc::c_int as libc::c_ulong);
        return ret
    }
    i = 0 as libc::c_int as CK_ULONG;
    while i < ulCount {
        st_logf(b"       getting 0x%08lx\n\x00" as *const u8 as
                    *const libc::c_char,
                (*pTemplate.offset(i as isize)).type_0);
        j = 0 as libc::c_int;
        while j < (*obj).num_attributes {
            if (*(*obj).attrs.offset(j as isize)).secret != 0 {
                (*pTemplate.offset(i as isize)).ulValueLen =
                    -(1 as libc::c_int) as CK_ULONG;
                break ;
            } else if (*pTemplate.offset(i as isize)).type_0 ==
                          (*(*obj).attrs.offset(j as isize)).attribute.type_0
             {
                if !(*pTemplate.offset(i as isize)).pValue.is_null() &&
                       (*(*obj).attrs.offset(j as isize)).secret ==
                           0 as libc::c_int {
                    if (*pTemplate.offset(i as isize)).ulValueLen >=
                           (*(*obj).attrs.offset(j as
                                                     isize)).attribute.ulValueLen
                       {
                        memcpy((*pTemplate.offset(i as isize)).pValue,
                               (*(*obj).attrs.offset(j as
                                                         isize)).attribute.pValue,
                               (*(*obj).attrs.offset(j as
                                                         isize)).attribute.ulValueLen);
                    }
                }
                (*pTemplate.offset(i as isize)).ulValueLen =
                    (*(*obj).attrs.offset(j as isize)).attribute.ulValueLen;
                break ;
            } else { j += 1 }
        }
        if j == (*obj).num_attributes {
            st_logf(b"key type: 0x%08lx not found\n\x00" as *const u8 as
                        *const libc::c_char,
                    (*pTemplate.offset(i as isize)).type_0);
            (*pTemplate.offset(i as isize)).ulValueLen =
                -(1 as libc::c_int) as CK_ULONG
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "1310:1"]
pub unsafe extern "C" fn C_FindObjectsInit(mut hSession: CK_SESSION_HANDLE,
                                           mut pTemplate: CK_ATTRIBUTE_PTR,
                                           mut ulCount: CK_ULONG) -> CK_RV {
    let mut state: *mut session_state = 0 as *mut session_state;
    st_logf(b"FindObjectsInit\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, &mut state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    if (*state).find.next_object != -(1 as libc::c_int) {
        application_error(b"application didn\'t do C_FindObjectsFinal\n\x00"
                              as *const u8 as *const libc::c_char);
        find_object_final(state);
    }
    if ulCount != 0 {
        let mut i: CK_ULONG = 0;
        print_attributes(pTemplate as *const CK_ATTRIBUTE, ulCount);
        (*state).find.attributes =
            calloc(1 as libc::c_int as libc::c_ulong,
                   ulCount.wrapping_mul(::std::mem::size_of::<CK_ATTRIBUTE>()
                                            as libc::c_ulong)) as
                *mut CK_ATTRIBUTE;
        if (*state).find.attributes.is_null() {
            return 0x31 as libc::c_int as CK_RV
        }
        i = 0 as libc::c_int as CK_ULONG;
        while i < ulCount {
            let ref mut fresh5 =
                (*(*state).find.attributes.offset(i as isize)).pValue;
            *fresh5 = malloc((*pTemplate.offset(i as isize)).ulValueLen);
            if (*(*state).find.attributes.offset(i as isize)).pValue.is_null()
               {
                find_object_final(state);
                return 0x31 as libc::c_int as CK_RV
            }
            memcpy((*(*state).find.attributes.offset(i as isize)).pValue,
                   (*pTemplate.offset(i as isize)).pValue,
                   (*pTemplate.offset(i as isize)).ulValueLen);
            (*(*state).find.attributes.offset(i as isize)).type_0 =
                (*pTemplate.offset(i as isize)).type_0;
            (*(*state).find.attributes.offset(i as isize)).ulValueLen =
                (*pTemplate.offset(i as isize)).ulValueLen;
            i = i.wrapping_add(1)
        }
        (*state).find.num_attributes = ulCount;
        (*state).find.next_object = 0 as libc::c_int
    } else {
        st_logf(b"find all objects\n\x00" as *const u8 as
                    *const libc::c_char);
        (*state).find.attributes = 0 as *mut CK_ATTRIBUTE;
        (*state).find.num_attributes = 0 as libc::c_int as CK_ULONG;
        (*state).find.next_object = 0 as libc::c_int
    }
    return 0 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "1358:1"]
pub unsafe extern "C" fn C_FindObjects(mut hSession: CK_SESSION_HANDLE,
                                       mut phObject: CK_OBJECT_HANDLE_PTR,
                                       mut ulMaxObjectCount: CK_ULONG,
                                       mut pulObjectCount: CK_ULONG_PTR)
 -> CK_RV {
    let mut state: *mut session_state = 0 as *mut session_state;
    let mut i: libc::c_int = 0;
    st_logf(b"FindObjects\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, &mut state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    if (*state).find.next_object == -(1 as libc::c_int) {
        application_error(b"application didn\'t do C_FindObjectsInit\n\x00" as
                              *const u8 as *const libc::c_char);
        return 7 as libc::c_int as CK_RV
    }
    if ulMaxObjectCount == 0 as libc::c_int as libc::c_ulong {
        application_error(b"application asked for 0 objects\n\x00" as
                              *const u8 as *const libc::c_char);
        return 7 as libc::c_int as CK_RV
    }
    *pulObjectCount = 0 as libc::c_int as CK_ULONG;
    i = (*state).find.next_object;
    while i < soft_token.object.num_objs {
        st_logf(b"FindObjects: %d\n\x00" as *const u8 as *const libc::c_char,
                i);
        (*state).find.next_object = i + 1 as libc::c_int;
        if attributes_match(*soft_token.object.objs.offset(i as isize),
                            (*state).find.attributes,
                            (*state).find.num_attributes) != 0 {
            let fresh6 = phObject;
            phObject = phObject.offset(1);
            *fresh6 =
                (**soft_token.object.objs.offset(i as isize)).object_handle;
            ulMaxObjectCount = ulMaxObjectCount.wrapping_sub(1);
            *pulObjectCount = (*pulObjectCount).wrapping_add(1);
            if ulMaxObjectCount == 0 as libc::c_int as libc::c_ulong {
                break ;
            }
        }
        i += 1
    }
    return 0 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "1396:1"]
pub unsafe extern "C" fn C_FindObjectsFinal(mut hSession: CK_SESSION_HANDLE)
 -> CK_RV {
    let mut state: *mut session_state = 0 as *mut session_state;
    st_logf(b"FindObjectsFinal\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, &mut state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    find_object_final(state);
    return 0 as libc::c_int as CK_RV;
}
#[c2rust::src_loc = "1407:1"]
unsafe extern "C" fn commonInit(mut attr_match: *mut CK_ATTRIBUTE,
                                mut attr_match_len: libc::c_int,
                                mut mechs: *const CK_MECHANISM_TYPE,
                                mut mechs_len: libc::c_int,
                                pMechanism: CK_MECHANISM_PTR,
                                mut hKey: CK_OBJECT_HANDLE,
                                mut o: *mut *mut st_object) -> CK_RV {
    let mut ret: CK_RV = 0;
    let mut i: libc::c_int = 0;
    *o = 0 as *mut st_object;
    ret = object_handle_to_object(hKey, o);
    if ret != 0 as libc::c_int as libc::c_ulong { return ret }
    ret =
        attributes_match(*o, attr_match, attr_match_len as CK_ULONG) as CK_RV;
    if ret == 0 {
        application_error(b"called commonInit on key that doesn\'t support required attr\x00"
                              as *const u8 as *const libc::c_char);
        return 7 as libc::c_int as CK_RV
    }
    i = 0 as libc::c_int;
    while i < mechs_len {
        if *mechs.offset(i as isize) == (*pMechanism).mechanism { break ; }
        i += 1
    }
    if i == mechs_len {
        application_error(b"called mech (%08lx) not supported\n\x00" as
                              *const u8 as *const libc::c_char,
                          (*pMechanism).mechanism);
        return 7 as libc::c_int as CK_RV
    }
    return 0 as libc::c_int as CK_RV;
}
#[c2rust::src_loc = "1439:1"]
unsafe extern "C" fn dup_mechanism(mut dup: *mut CK_MECHANISM_PTR,
                                   pMechanism: CK_MECHANISM_PTR) -> CK_RV {
    let mut p: CK_MECHANISM_PTR = 0 as *mut _CK_MECHANISM;
    p =
        malloc(::std::mem::size_of::<_CK_MECHANISM>() as libc::c_ulong) as
            CK_MECHANISM_PTR;
    if p.is_null() { return 0x31 as libc::c_int as CK_RV }
    if !(*dup).is_null() { free(*dup as *mut libc::c_void); }
    *dup = p;
    memcpy(p as *mut libc::c_void, pMechanism as *const libc::c_void,
           ::std::mem::size_of::<_CK_MECHANISM>() as libc::c_ulong);
    return 0 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "1457:1"]
pub unsafe extern "C" fn C_EncryptInit(mut hSession: CK_SESSION_HANDLE,
                                       mut pMechanism: CK_MECHANISM_PTR,
                                       mut hKey: CK_OBJECT_HANDLE) -> CK_RV {
    let mut state: *mut session_state = 0 as *mut session_state;
    let mut mechs: [CK_MECHANISM_TYPE; 2] =
        [1 as libc::c_int as CK_MECHANISM_TYPE,
         3 as libc::c_int as CK_MECHANISM_TYPE];
    let mut bool_true: CK_BBOOL = 1 as libc::c_int as CK_BBOOL;
    let mut attr: [CK_ATTRIBUTE; 1] =
        [{
             let mut init =
                 _CK_ATTRIBUTE{type_0:
                                   0x104 as libc::c_int as CK_ATTRIBUTE_TYPE,
                               pValue:
                                   &mut bool_true as *mut CK_BBOOL as
                                       *mut libc::c_void,
                               ulValueLen:
                                   ::std::mem::size_of::<CK_BBOOL>() as
                                       libc::c_ulong,};
             init
         }];
    let mut o: *mut st_object = 0 as *mut st_object;
    let mut ret: CK_RV = 0;
    st_logf(b"EncryptInit\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, &mut state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    ret =
        commonInit(attr.as_mut_ptr(),
                   (::std::mem::size_of::<[CK_ATTRIBUTE; 1]>() as
                        libc::c_ulong).wrapping_div(::std::mem::size_of::<CK_ATTRIBUTE>()
                                                        as libc::c_ulong) as
                       libc::c_int, mechs.as_mut_ptr(),
                   (::std::mem::size_of::<[CK_MECHANISM_TYPE; 2]>() as
                        libc::c_ulong).wrapping_div(::std::mem::size_of::<CK_MECHANISM_TYPE>()
                                                        as libc::c_ulong) as
                       libc::c_int, pMechanism, hKey, &mut o);
    if ret != 0 { return ret }
    ret = dup_mechanism(&mut (*state).encrypt_mechanism, pMechanism);
    if ret == 0 as libc::c_int as libc::c_ulong {
        (*state).encrypt_object =
            ((*o).object_handle & 0xfff as libc::c_int as libc::c_ulong) as
                libc::c_int
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "1487:1"]
pub unsafe extern "C" fn C_Encrypt(mut hSession: CK_SESSION_HANDLE,
                                   mut pData: CK_BYTE_PTR,
                                   mut ulDataLen: CK_ULONG,
                                   mut pEncryptedData: CK_BYTE_PTR,
                                   mut pulEncryptedDataLen: CK_ULONG_PTR)
 -> CK_RV {
    let mut current_block: u64;
    let mut state: *mut session_state = 0 as *mut session_state;
    let mut o: *mut st_object = 0 as *mut st_object;
    let mut buffer: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ret: CK_RV = 0;
    let mut rsa: *mut RSA = 0 as *mut RSA;
    let mut padding: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut buffer_len: libc::c_int = 0;
    let mut padding_len: libc::c_int = 0;
    st_logf(b"Encrypt\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, &mut state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    if (*state).encrypt_object == -(1 as libc::c_int) {
        return 7 as libc::c_int as CK_RV
    }
    o = *soft_token.object.objs.offset((*state).encrypt_object as isize);
    if (*o).u.public_key.is_null() {
        st_logf(b"public key NULL\n\x00" as *const u8 as *const libc::c_char);
        return 7 as libc::c_int as CK_RV
    }
    rsa = EVP_PKEY_get0_RSA((*o).u.public_key);
    if rsa.is_null() { return 7 as libc::c_int as CK_RV }
    RSA_blinding_off(rsa);
    buffer_len = RSA_size(rsa);
    buffer = malloc(buffer_len as libc::c_ulong);
    if buffer.is_null() {
        ret = 0x31 as libc::c_int as CK_RV
    } else {
        ret = 0 as libc::c_int as CK_RV;
        match (*(*state).encrypt_mechanism).mechanism {
            1 => {
                padding = 1 as libc::c_int;
                padding_len = 11 as libc::c_int;
                current_block = 5494826135382683477;
            }
            3 => {
                padding = 3 as libc::c_int;
                padding_len = 0 as libc::c_int;
                current_block = 5494826135382683477;
            }
            _ => {
                ret = 0x54 as libc::c_int as CK_RV;
                current_block = 15865388192082990972;
            }
        }
        match current_block {
            15865388192082990972 => { }
            _ => {
                if (buffer_len as
                        CK_ULONG).wrapping_add(padding_len as libc::c_ulong) <
                       ulDataLen {
                    ret = 7 as libc::c_int as CK_RV
                } else if pulEncryptedDataLen.is_null() {
                    st_logf(b"pulEncryptedDataLen NULL\n\x00" as *const u8 as
                                *const libc::c_char);
                    ret = 7 as libc::c_int as CK_RV
                } else if pData.is_null() {
                    st_logf(b"data NULL\n\x00" as *const u8 as
                                *const libc::c_char);
                    ret = 7 as libc::c_int as CK_RV
                } else {
                    len =
                        RSA_public_encrypt(ulDataLen as libc::c_int,
                                           pData as *const libc::c_uchar,
                                           buffer as *mut libc::c_uchar, rsa,
                                           padding);
                    if len <= 0 as libc::c_int {
                        ret = 0x30 as libc::c_int as CK_RV
                    } else {
                        if len > buffer_len { abort(); }
                        if !pEncryptedData.is_null() {
                            memcpy(pEncryptedData as *mut libc::c_void,
                                   buffer, len as libc::c_ulong);
                        }
                        *pulEncryptedDataLen = len as CK_ULONG
                    }
                }
            }
        }
    }
    if !buffer.is_null() {
        memset(buffer, 0 as libc::c_int, buffer_len as libc::c_ulong);
        free(buffer);
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "1582:1"]
pub unsafe extern "C" fn C_EncryptUpdate(mut hSession: CK_SESSION_HANDLE,
                                         mut pPart: CK_BYTE_PTR,
                                         mut ulPartLen: CK_ULONG,
                                         mut pEncryptedPart: CK_BYTE_PTR,
                                         mut pulEncryptedPartLen:
                                             CK_ULONG_PTR) -> CK_RV {
    st_logf(b"EncryptUpdate\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, 0 as *mut *mut session_state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    return 0x54 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "1595:1"]
pub unsafe extern "C" fn C_EncryptFinal(mut hSession: CK_SESSION_HANDLE,
                                        mut pLastEncryptedPart: CK_BYTE_PTR,
                                        mut pulLastEncryptedPartLen:
                                            CK_ULONG_PTR) -> CK_RV {
    st_logf(b"EncryptFinal\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, 0 as *mut *mut session_state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    return 0x54 as libc::c_int as CK_RV;
}
/* C_DecryptInit initializes a decryption operation. */
#[no_mangle]
#[c2rust::src_loc = "1607:1"]
pub unsafe extern "C" fn C_DecryptInit(mut hSession: CK_SESSION_HANDLE,
                                       mut pMechanism: CK_MECHANISM_PTR,
                                       mut hKey: CK_OBJECT_HANDLE) -> CK_RV {
    let mut state: *mut session_state =
        0 as
            *mut session_state; /* XXX RAND is broken while running in mozilla ? */
    let mut mechs: [CK_MECHANISM_TYPE; 2] =
        [1 as libc::c_int as CK_MECHANISM_TYPE,
         3 as libc::c_int as
             CK_MECHANISM_TYPE]; /* XXX RAND is broken while running in mozilla ? */
    let mut bool_true: CK_BBOOL =
        1 as libc::c_int as
            CK_BBOOL; /* XXX RAND is broken while running in mozilla ? */
    let mut attr: [CK_ATTRIBUTE; 1] =
        [{
             let mut init =
                 _CK_ATTRIBUTE{type_0:
                                   0x105 as libc::c_int as CK_ATTRIBUTE_TYPE,
                               pValue:
                                   &mut bool_true as *mut CK_BBOOL as
                                       *mut libc::c_void,
                               ulValueLen:
                                   ::std::mem::size_of::<CK_BBOOL>() as
                                       libc::c_ulong,};
             init
         }];
    let mut o: *mut st_object = 0 as *mut st_object;
    let mut ret: CK_RV = 0;
    st_logf(b"DecryptInit\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, &mut state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    ret =
        commonInit(attr.as_mut_ptr(),
                   (::std::mem::size_of::<[CK_ATTRIBUTE; 1]>() as
                        libc::c_ulong).wrapping_div(::std::mem::size_of::<CK_ATTRIBUTE>()
                                                        as libc::c_ulong) as
                       libc::c_int, mechs.as_mut_ptr(),
                   (::std::mem::size_of::<[CK_MECHANISM_TYPE; 2]>() as
                        libc::c_ulong).wrapping_div(::std::mem::size_of::<CK_MECHANISM_TYPE>()
                                                        as libc::c_ulong) as
                       libc::c_int, pMechanism, hKey, &mut o);
    if ret != 0 { return ret }
    ret = dup_mechanism(&mut (*state).decrypt_mechanism, pMechanism);
    if ret == 0 as libc::c_int as libc::c_ulong {
        (*state).decrypt_object =
            ((*o).object_handle & 0xfff as libc::c_int as libc::c_ulong) as
                libc::c_int
    }
    return 0 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "1638:1"]
pub unsafe extern "C" fn C_Decrypt(mut hSession: CK_SESSION_HANDLE,
                                   mut pEncryptedData: CK_BYTE_PTR,
                                   mut ulEncryptedDataLen: CK_ULONG,
                                   mut pData: CK_BYTE_PTR,
                                   mut pulDataLen: CK_ULONG_PTR) -> CK_RV {
    let mut current_block: u64;
    let mut state: *mut session_state = 0 as *mut session_state;
    let mut o: *mut st_object = 0 as *mut st_object;
    let mut buffer: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ret: CK_RV = 0;
    let mut rsa: *mut RSA = 0 as *mut RSA;
    let mut padding: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut buffer_len: libc::c_int = 0;
    let mut padding_len: libc::c_int = 0;
    st_logf(b"Decrypt\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, &mut state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    if (*state).decrypt_object == -(1 as libc::c_int) {
        return 7 as libc::c_int as CK_RV
    }
    o = *soft_token.object.objs.offset((*state).decrypt_object as isize);
    if (*o).u.private_key.key.is_null() {
        st_logf(b"private key NULL\n\x00" as *const u8 as
                    *const libc::c_char);
        return 7 as libc::c_int as CK_RV
    }
    rsa = EVP_PKEY_get0_RSA((*o).u.private_key.key);
    if rsa.is_null() { return 7 as libc::c_int as CK_RV }
    RSA_blinding_off(rsa);
    buffer_len = RSA_size(rsa);
    buffer = malloc(buffer_len as libc::c_ulong);
    if buffer.is_null() {
        ret = 0x31 as libc::c_int as CK_RV
    } else {
        ret = 0 as libc::c_int as CK_RV;
        match (*(*state).decrypt_mechanism).mechanism {
            1 => {
                padding = 1 as libc::c_int;
                padding_len = 11 as libc::c_int;
                current_block = 5494826135382683477;
            }
            3 => {
                padding = 3 as libc::c_int;
                padding_len = 0 as libc::c_int;
                current_block = 5494826135382683477;
            }
            _ => {
                ret = 0x54 as libc::c_int as CK_RV;
                current_block = 8137419950338444964;
            }
        }
        match current_block {
            8137419950338444964 => { }
            _ => {
                if (buffer_len as
                        CK_ULONG).wrapping_add(padding_len as libc::c_ulong) <
                       ulEncryptedDataLen {
                    ret = 7 as libc::c_int as CK_RV
                } else if pulDataLen.is_null() {
                    st_logf(b"pulDataLen NULL\n\x00" as *const u8 as
                                *const libc::c_char);
                    ret = 7 as libc::c_int as CK_RV
                } else if pEncryptedData.is_null() {
                    st_logf(b"data NULL\n\x00" as *const u8 as
                                *const libc::c_char);
                    ret = 7 as libc::c_int as CK_RV
                } else {
                    len =
                        RSA_private_decrypt(ulEncryptedDataLen as libc::c_int,
                                            pEncryptedData as
                                                *const libc::c_uchar,
                                            buffer as *mut libc::c_uchar, rsa,
                                            padding);
                    if len <= 0 as libc::c_int {
                        ret = 0x30 as libc::c_int as CK_RV
                    } else {
                        if len > buffer_len { abort(); }
                        if !pData.is_null() {
                            memcpy(pData as *mut libc::c_void, buffer,
                                   len as libc::c_ulong);
                        }
                        *pulDataLen = len as CK_ULONG
                    }
                }
            }
        }
    }
    if !buffer.is_null() {
        memset(buffer, 0 as libc::c_int, buffer_len as libc::c_ulong);
        free(buffer);
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "1735:1"]
pub unsafe extern "C" fn C_DecryptUpdate(mut hSession: CK_SESSION_HANDLE,
                                         mut pEncryptedPart: CK_BYTE_PTR,
                                         mut ulEncryptedPartLen: CK_ULONG,
                                         mut pPart: CK_BYTE_PTR,
                                         mut pulPartLen: CK_ULONG_PTR)
 -> CK_RV {
    st_logf(b"DecryptUpdate\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, 0 as *mut *mut session_state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    return 0x54 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "1749:1"]
pub unsafe extern "C" fn C_DecryptFinal(mut hSession: CK_SESSION_HANDLE,
                                        mut pLastPart: CK_BYTE_PTR,
                                        mut pulLastPartLen: CK_ULONG_PTR)
 -> CK_RV {
    st_logf(b"DecryptFinal\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, 0 as *mut *mut session_state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    return 0x54 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "1759:1"]
pub unsafe extern "C" fn C_DigestInit(mut hSession: CK_SESSION_HANDLE,
                                      mut pMechanism: CK_MECHANISM_PTR)
 -> CK_RV {
    st_logf(b"DigestInit\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, 0 as *mut *mut session_state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    return 0x54 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "1768:1"]
pub unsafe extern "C" fn C_SignInit(mut hSession: CK_SESSION_HANDLE,
                                    mut pMechanism: CK_MECHANISM_PTR,
                                    mut hKey: CK_OBJECT_HANDLE) -> CK_RV {
    let mut state: *mut session_state = 0 as *mut session_state;
    let mut mechs: [CK_MECHANISM_TYPE; 2] =
        [1 as libc::c_int as CK_MECHANISM_TYPE,
         3 as libc::c_int as CK_MECHANISM_TYPE];
    let mut bool_true: CK_BBOOL = 1 as libc::c_int as CK_BBOOL;
    let mut attr: [CK_ATTRIBUTE; 1] =
        [{
             let mut init =
                 _CK_ATTRIBUTE{type_0:
                                   0x108 as libc::c_int as CK_ATTRIBUTE_TYPE,
                               pValue:
                                   &mut bool_true as *mut CK_BBOOL as
                                       *mut libc::c_void,
                               ulValueLen:
                                   ::std::mem::size_of::<CK_BBOOL>() as
                                       libc::c_ulong,};
             init
         }];
    let mut o: *mut st_object = 0 as *mut st_object;
    let mut ret: CK_RV = 0;
    st_logf(b"SignInit\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, &mut state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    ret =
        commonInit(attr.as_mut_ptr(),
                   (::std::mem::size_of::<[CK_ATTRIBUTE; 1]>() as
                        libc::c_ulong).wrapping_div(::std::mem::size_of::<CK_ATTRIBUTE>()
                                                        as libc::c_ulong) as
                       libc::c_int, mechs.as_mut_ptr(),
                   (::std::mem::size_of::<[CK_MECHANISM_TYPE; 2]>() as
                        libc::c_ulong).wrapping_div(::std::mem::size_of::<CK_MECHANISM_TYPE>()
                                                        as libc::c_ulong) as
                       libc::c_int, pMechanism, hKey, &mut o);
    if ret != 0 { return ret }
    ret = dup_mechanism(&mut (*state).sign_mechanism, pMechanism);
    if ret == 0 as libc::c_int as libc::c_ulong {
        (*state).sign_object =
            ((*o).object_handle & 0xfff as libc::c_int as libc::c_ulong) as
                libc::c_int
    }
    return 0 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "1798:1"]
pub unsafe extern "C" fn C_Sign(mut hSession: CK_SESSION_HANDLE,
                                mut pData: CK_BYTE_PTR,
                                mut ulDataLen: CK_ULONG,
                                mut pSignature: CK_BYTE_PTR,
                                mut pulSignatureLen: CK_ULONG_PTR) -> CK_RV {
    let mut current_block: u64;
    let mut state: *mut session_state = 0 as *mut session_state;
    let mut o: *mut st_object = 0 as *mut st_object;
    let mut buffer: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ret: CK_RV = 0;
    let mut rsa: *mut RSA = 0 as *mut RSA;
    let mut padding: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut buffer_len: libc::c_int = 0;
    let mut padding_len: libc::c_int = 0;
    st_logf(b"Sign\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, &mut state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    if (*state).sign_object == -(1 as libc::c_int) {
        return 7 as libc::c_int as CK_RV
    }
    o = *soft_token.object.objs.offset((*state).sign_object as isize);
    if (*o).u.private_key.key.is_null() {
        st_logf(b"private key NULL\n\x00" as *const u8 as
                    *const libc::c_char);
        return 7 as libc::c_int as CK_RV
    }
    rsa = EVP_PKEY_get0_RSA((*o).u.private_key.key);
    if rsa.is_null() { return 7 as libc::c_int as CK_RV }
    RSA_blinding_off(rsa);
    buffer_len = RSA_size(rsa);
    buffer = malloc(buffer_len as libc::c_ulong);
    if buffer.is_null() {
        ret = 0x31 as libc::c_int as CK_RV
    } else {
        match (*(*state).sign_mechanism).mechanism {
            1 => {
                padding = 1 as libc::c_int;
                padding_len = 11 as libc::c_int;
                current_block = 11636175345244025579;
            }
            3 => {
                padding = 3 as libc::c_int;
                padding_len = 0 as libc::c_int;
                current_block = 11636175345244025579;
            }
            _ => {
                ret = 0x54 as libc::c_int as CK_RV;
                current_block = 17454931238048573484;
            }
        }
        match current_block {
            17454931238048573484 => { }
            _ => {
                if (buffer_len as CK_ULONG) <
                       ulDataLen.wrapping_add(padding_len as libc::c_ulong) {
                    ret = 7 as libc::c_int as CK_RV
                } else if pulSignatureLen.is_null() {
                    st_logf(b"signature len NULL\n\x00" as *const u8 as
                                *const libc::c_char);
                    ret = 7 as libc::c_int as CK_RV
                } else if pData.is_null() {
                    st_logf(b"data NULL\n\x00" as *const u8 as
                                *const libc::c_char);
                    ret = 7 as libc::c_int as CK_RV
                } else {
                    len =
                        RSA_private_encrypt(ulDataLen as libc::c_int,
                                            pData as *const libc::c_uchar,
                                            buffer as *mut libc::c_uchar, rsa,
                                            padding);
                    st_logf(b"private encrypt done\n\x00" as *const u8 as
                                *const libc::c_char);
                    if len <= 0 as libc::c_int {
                        ret = 0x30 as libc::c_int as CK_RV
                    } else {
                        if len > buffer_len { abort(); }
                        if !pSignature.is_null() {
                            memcpy(pSignature as *mut libc::c_void, buffer,
                                   len as libc::c_ulong);
                        }
                        *pulSignatureLen = len as CK_ULONG;
                        ret = 0 as libc::c_int as CK_RV
                    }
                }
            }
        }
    }
    if !buffer.is_null() {
        memset(buffer, 0 as libc::c_int, buffer_len as libc::c_ulong);
        free(buffer);
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "1894:1"]
pub unsafe extern "C" fn C_SignUpdate(mut hSession: CK_SESSION_HANDLE,
                                      mut pPart: CK_BYTE_PTR,
                                      mut ulPartLen: CK_ULONG) -> CK_RV {
    st_logf(b"SignUpdate\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, 0 as *mut *mut session_state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    return 0x54 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "1905:1"]
pub unsafe extern "C" fn C_SignFinal(mut hSession: CK_SESSION_HANDLE,
                                     mut pSignature: CK_BYTE_PTR,
                                     mut pulSignatureLen: CK_ULONG_PTR)
 -> CK_RV {
    st_logf(b"SignUpdate\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, 0 as *mut *mut session_state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    return 0x54 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "1915:1"]
pub unsafe extern "C" fn C_VerifyInit(mut hSession: CK_SESSION_HANDLE,
                                      mut pMechanism: CK_MECHANISM_PTR,
                                      mut hKey: CK_OBJECT_HANDLE) -> CK_RV {
    let mut state: *mut session_state = 0 as *mut session_state;
    let mut mechs: [CK_MECHANISM_TYPE; 2] =
        [1 as libc::c_int as CK_MECHANISM_TYPE,
         3 as libc::c_int as CK_MECHANISM_TYPE];
    let mut bool_true: CK_BBOOL = 1 as libc::c_int as CK_BBOOL;
    let mut attr: [CK_ATTRIBUTE; 1] =
        [{
             let mut init =
                 _CK_ATTRIBUTE{type_0:
                                   0x10a as libc::c_int as CK_ATTRIBUTE_TYPE,
                               pValue:
                                   &mut bool_true as *mut CK_BBOOL as
                                       *mut libc::c_void,
                               ulValueLen:
                                   ::std::mem::size_of::<CK_BBOOL>() as
                                       libc::c_ulong,};
             init
         }];
    let mut o: *mut st_object = 0 as *mut st_object;
    let mut ret: CK_RV = 0;
    st_logf(b"VerifyInit\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, &mut state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    ret =
        commonInit(attr.as_mut_ptr(),
                   (::std::mem::size_of::<[CK_ATTRIBUTE; 1]>() as
                        libc::c_ulong).wrapping_div(::std::mem::size_of::<CK_ATTRIBUTE>()
                                                        as libc::c_ulong) as
                       libc::c_int, mechs.as_mut_ptr(),
                   (::std::mem::size_of::<[CK_MECHANISM_TYPE; 2]>() as
                        libc::c_ulong).wrapping_div(::std::mem::size_of::<CK_MECHANISM_TYPE>()
                                                        as libc::c_ulong) as
                       libc::c_int, pMechanism, hKey, &mut o);
    if ret != 0 { return ret }
    ret = dup_mechanism(&mut (*state).verify_mechanism, pMechanism);
    if ret == 0 as libc::c_int as libc::c_ulong {
        (*state).verify_object =
            ((*o).object_handle & 0xfff as libc::c_int as libc::c_ulong) as
                libc::c_int
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "1945:1"]
pub unsafe extern "C" fn C_Verify(mut hSession: CK_SESSION_HANDLE,
                                  mut pData: CK_BYTE_PTR,
                                  mut ulDataLen: CK_ULONG,
                                  mut pSignature: CK_BYTE_PTR,
                                  mut ulSignatureLen: CK_ULONG) -> CK_RV {
    let mut current_block: u64;
    let mut state: *mut session_state = 0 as *mut session_state;
    let mut o: *mut st_object = 0 as *mut st_object;
    let mut buffer: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ret: CK_RV = 0;
    let mut rsa: *mut RSA = 0 as *mut RSA;
    let mut padding: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut buffer_len: libc::c_int = 0;
    st_logf(b"Verify\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, &mut state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    if (*state).verify_object == -(1 as libc::c_int) {
        return 7 as libc::c_int as CK_RV
    }
    o = *soft_token.object.objs.offset((*state).verify_object as isize);
    if (*o).u.public_key.is_null() {
        st_logf(b"public key NULL\n\x00" as *const u8 as *const libc::c_char);
        return 7 as libc::c_int as CK_RV
    }
    rsa = EVP_PKEY_get0_RSA((*o).u.public_key);
    if rsa.is_null() { return 7 as libc::c_int as CK_RV }
    RSA_blinding_off(rsa);
    buffer_len = RSA_size(rsa);
    buffer = malloc(buffer_len as libc::c_ulong);
    if buffer.is_null() {
        ret = 0x31 as libc::c_int as CK_RV
    } else {
        ret = 0 as libc::c_int as CK_RV;
        match (*(*state).verify_mechanism).mechanism {
            1 => {
                padding = 1 as libc::c_int;
                current_block = 11932355480408055363;
            }
            3 => {
                padding = 3 as libc::c_int;
                current_block = 11932355480408055363;
            }
            _ => {
                ret = 0x54 as libc::c_int as CK_RV;
                current_block = 10576395752482358930;
            }
        }
        match current_block {
            10576395752482358930 => { }
            _ => {
                if (buffer_len as CK_ULONG) < ulDataLen {
                    ret = 7 as libc::c_int as CK_RV
                } else if pSignature.is_null() {
                    st_logf(b"signature NULL\n\x00" as *const u8 as
                                *const libc::c_char);
                    ret = 7 as libc::c_int as CK_RV
                } else if pData.is_null() {
                    st_logf(b"data NULL\n\x00" as *const u8 as
                                *const libc::c_char);
                    ret = 7 as libc::c_int as CK_RV
                } else {
                    len =
                        RSA_public_decrypt(ulDataLen as libc::c_int,
                                           pData as *const libc::c_uchar,
                                           buffer as *mut libc::c_uchar, rsa,
                                           padding);
                    st_logf(b"private encrypt done\n\x00" as *const u8 as
                                *const libc::c_char);
                    if len <= 0 as libc::c_int {
                        ret = 0x30 as libc::c_int as CK_RV
                    } else {
                        if len > buffer_len { abort(); }
                        if len as CK_ULONG != ulSignatureLen {
                            ret = 5 as libc::c_int as CK_RV
                        } else if memcmp(pSignature as *const libc::c_void,
                                         buffer, len as libc::c_ulong) !=
                                      0 as libc::c_int {
                            ret = 5 as libc::c_int as CK_RV
                        }
                    }
                }
            }
        }
    }
    if !buffer.is_null() {
        memset(buffer, 0 as libc::c_int, buffer_len as libc::c_ulong);
        free(buffer);
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2045:1"]
pub unsafe extern "C" fn C_VerifyUpdate(mut hSession: CK_SESSION_HANDLE,
                                        mut pPart: CK_BYTE_PTR,
                                        mut ulPartLen: CK_ULONG) -> CK_RV {
    st_logf(b"VerifyUpdate\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, 0 as *mut *mut session_state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    return 0x54 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "2055:1"]
pub unsafe extern "C" fn C_VerifyFinal(mut hSession: CK_SESSION_HANDLE,
                                       mut pSignature: CK_BYTE_PTR,
                                       mut ulSignatureLen: CK_ULONG)
 -> CK_RV {
    st_logf(b"VerifyFinal\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, 0 as *mut *mut session_state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    return 0x54 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "2065:1"]
pub unsafe extern "C" fn C_GenerateRandom(mut hSession: CK_SESSION_HANDLE,
                                          mut RandomData: CK_BYTE_PTR,
                                          mut ulRandomLen: CK_ULONG)
 -> CK_RV {
    st_logf(b"GenerateRandom\n\x00" as *const u8 as *const libc::c_char);
    let mut vshret: CK_RV = 0;
    vshret = verify_session_handle(hSession, 0 as *mut *mut session_state);
    (vshret) != 0 as libc::c_int as libc::c_ulong;
    return 0x54 as libc::c_int as CK_RV;
}
#[no_mangle]
#[c2rust::src_loc = "2076:18"]
pub static mut funcs: CK_FUNCTION_LIST =
    unsafe {
        {
            let mut init =
                _CK_FUNCTION_LIST{version:
                                      {
                                          let mut init =
                                              _CK_VERSION{major:
                                                              2 as libc::c_int
                                                                  as
                                                                  libc::c_uchar,
                                                          minor:
                                                              11 as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uchar,};
                                          init
                                      },
                                  C_Initialize:
                                      Some(C_Initialize as
                                               unsafe extern "C" fn(_:
                                                                        *mut libc::c_void)
                                                   -> CK_RV),
                                  C_Finalize:
                                      Some(C_Finalize as
                                               unsafe extern "C" fn(_:
                                                                        *mut libc::c_void)
                                                   -> CK_RV),
                                  C_GetInfo:
                                      Some(C_GetInfo as
                                               unsafe extern "C" fn(_:
                                                                        *mut _CK_INFO)
                                                   -> CK_RV),
                                  C_GetFunctionList:
                                      Some(C_GetFunctionList as
                                               unsafe extern "C" fn(_:
                                                                        *mut *mut _CK_FUNCTION_LIST)
                                                   -> CK_RV),
                                  C_GetSlotList:
                                      Some(C_GetSlotList as
                                               unsafe extern "C" fn(_:
                                                                        libc::c_uchar,
                                                                    _:
                                                                        *mut CK_SLOT_ID,
                                                                    _:
                                                                        *mut libc::c_ulong)
                                                   -> CK_RV),
                                  C_GetSlotInfo:
                                      Some(C_GetSlotInfo as
                                               unsafe extern "C" fn(_:
                                                                        CK_SLOT_ID,
                                                                    _:
                                                                        *mut _CK_SLOT_INFO)
                                                   -> CK_RV),
                                  C_GetTokenInfo:
                                      Some(C_GetTokenInfo as
                                               unsafe extern "C" fn(_:
                                                                        CK_SLOT_ID,
                                                                    _:
                                                                        *mut _CK_TOKEN_INFO)
                                                   -> CK_RV),
                                  C_GetMechanismList:
                                      Some(C_GetMechanismList as
                                               unsafe extern "C" fn(_:
                                                                        CK_SLOT_ID,
                                                                    _:
                                                                        *mut CK_MECHANISM_TYPE,
                                                                    _:
                                                                        *mut libc::c_ulong)
                                                   -> CK_RV),
                                  C_GetMechanismInfo:
                                      Some(C_GetMechanismInfo as
                                               unsafe extern "C" fn(_:
                                                                        CK_SLOT_ID,
                                                                    _:
                                                                        CK_MECHANISM_TYPE,
                                                                    _:
                                                                        *mut _CK_MECHANISM_INFO)
                                                   -> CK_RV),
                                  C_InitToken:
                                      Some(C_InitToken as
                                               unsafe extern "C" fn(_:
                                                                        CK_SLOT_ID,
                                                                    _:
                                                                        *mut libc::c_uchar,
                                                                    _:
                                                                        libc::c_ulong,
                                                                    _:
                                                                        *mut libc::c_uchar)
                                                   -> CK_RV),
                                  C_InitPIN:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_InitPIN>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                               ->
                                                                                                                   CK_RV>,
                                                                                                    *mut libc::c_void>(Some(func_not_supported
                                                                                                                                as
                                                                                                                                unsafe extern "C" fn()
                                                                                                                                    ->
                                                                                                                                        CK_RV))),
                                  C_SetPIN:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_SetPIN>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                              ->
                                                                                                                  CK_RV>,
                                                                                                   *mut libc::c_void>(Some(func_not_supported
                                                                                                                               as
                                                                                                                               unsafe extern "C" fn()
                                                                                                                                   ->
                                                                                                                                       CK_RV))),
                                  C_OpenSession:
                                      Some(C_OpenSession as
                                               unsafe extern "C" fn(_:
                                                                        CK_SLOT_ID,
                                                                    _:
                                                                        CK_FLAGS,
                                                                    _:
                                                                        *mut libc::c_void,
                                                                    _:
                                                                        CK_NOTIFY,
                                                                    _:
                                                                        *mut CK_SESSION_HANDLE)
                                                   -> CK_RV),
                                  C_CloseSession:
                                      Some(C_CloseSession as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE)
                                                   -> CK_RV),
                                  C_CloseAllSessions:
                                      Some(C_CloseAllSessions as
                                               unsafe extern "C" fn(_:
                                                                        CK_SLOT_ID)
                                                   -> CK_RV),
                                  C_GetSessionInfo:
                                      Some(C_GetSessionInfo as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE,
                                                                    _:
                                                                        *mut _CK_SESSION_INFO)
                                                   -> CK_RV),
                                  C_GetOperationState:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_GetOperationState>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                         ->
                                                                                                                             CK_RV>,
                                                                                                              *mut libc::c_void>(Some(func_not_supported
                                                                                                                                          as
                                                                                                                                          unsafe extern "C" fn()
                                                                                                                                              ->
                                                                                                                                                  CK_RV))),
                                  C_SetOperationState:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_SetOperationState>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                         ->
                                                                                                                             CK_RV>,
                                                                                                              *mut libc::c_void>(Some(func_not_supported
                                                                                                                                          as
                                                                                                                                          unsafe extern "C" fn()
                                                                                                                                              ->
                                                                                                                                                  CK_RV))),
                                  C_Login:
                                      Some(C_Login as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE,
                                                                    _:
                                                                        CK_USER_TYPE,
                                                                    _:
                                                                        *mut libc::c_uchar,
                                                                    _:
                                                                        libc::c_ulong)
                                                   -> CK_RV),
                                  C_Logout:
                                      Some(C_Logout as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE)
                                                   -> CK_RV),
                                  C_CreateObject:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_CreateObject>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                    ->
                                                                                                                        CK_RV>,
                                                                                                         *mut libc::c_void>(Some(func_not_supported
                                                                                                                                     as
                                                                                                                                     unsafe extern "C" fn()
                                                                                                                                         ->
                                                                                                                                             CK_RV))),
                                  C_CopyObject:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_CopyObject>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                  ->
                                                                                                                      CK_RV>,
                                                                                                       *mut libc::c_void>(Some(func_not_supported
                                                                                                                                   as
                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                       ->
                                                                                                                                           CK_RV))),
                                  C_DestroyObject:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_DestroyObject>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                     ->
                                                                                                                         CK_RV>,
                                                                                                          *mut libc::c_void>(Some(func_not_supported
                                                                                                                                      as
                                                                                                                                      unsafe extern "C" fn()
                                                                                                                                          ->
                                                                                                                                              CK_RV))),
                                  C_GetObjectSize:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_GetObjectSize>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                     ->
                                                                                                                         CK_RV>,
                                                                                                          *mut libc::c_void>(Some(func_not_supported
                                                                                                                                      as
                                                                                                                                      unsafe extern "C" fn()
                                                                                                                                          ->
                                                                                                                                              CK_RV))),
                                  C_GetAttributeValue:
                                      Some(C_GetAttributeValue as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE,
                                                                    _:
                                                                        CK_OBJECT_HANDLE,
                                                                    _:
                                                                        *mut _CK_ATTRIBUTE,
                                                                    _:
                                                                        libc::c_ulong)
                                                   -> CK_RV),
                                  C_SetAttributeValue:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_SetAttributeValue>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                         ->
                                                                                                                             CK_RV>,
                                                                                                              *mut libc::c_void>(Some(func_not_supported
                                                                                                                                          as
                                                                                                                                          unsafe extern "C" fn()
                                                                                                                                              ->
                                                                                                                                                  CK_RV))),
                                  C_FindObjectsInit:
                                      Some(C_FindObjectsInit as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE,
                                                                    _:
                                                                        *mut _CK_ATTRIBUTE,
                                                                    _:
                                                                        libc::c_ulong)
                                                   -> CK_RV),
                                  C_FindObjects:
                                      Some(C_FindObjects as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE,
                                                                    _:
                                                                        *mut CK_OBJECT_HANDLE,
                                                                    _:
                                                                        libc::c_ulong,
                                                                    _:
                                                                        *mut libc::c_ulong)
                                                   -> CK_RV),
                                  C_FindObjectsFinal:
                                      Some(C_FindObjectsFinal as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE)
                                                   -> CK_RV),
                                  C_EncryptInit:
                                      Some(C_EncryptInit as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE,
                                                                    _:
                                                                        *mut _CK_MECHANISM,
                                                                    _:
                                                                        CK_OBJECT_HANDLE)
                                                   -> CK_RV),
                                  C_Encrypt:
                                      Some(C_Encrypt as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE,
                                                                    _:
                                                                        *mut libc::c_uchar,
                                                                    _:
                                                                        libc::c_ulong,
                                                                    _:
                                                                        *mut libc::c_uchar,
                                                                    _:
                                                                        *mut libc::c_ulong)
                                                   -> CK_RV),
                                  C_EncryptUpdate:
                                      Some(C_EncryptUpdate as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE,
                                                                    _:
                                                                        *mut libc::c_uchar,
                                                                    _:
                                                                        libc::c_ulong,
                                                                    _:
                                                                        *mut libc::c_uchar,
                                                                    _:
                                                                        *mut libc::c_ulong)
                                                   -> CK_RV),
                                  C_EncryptFinal:
                                      Some(C_EncryptFinal as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE,
                                                                    _:
                                                                        *mut libc::c_uchar,
                                                                    _:
                                                                        *mut libc::c_ulong)
                                                   -> CK_RV),
                                  C_DecryptInit:
                                      Some(C_DecryptInit as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE,
                                                                    _:
                                                                        *mut _CK_MECHANISM,
                                                                    _:
                                                                        CK_OBJECT_HANDLE)
                                                   -> CK_RV),
                                  C_Decrypt:
                                      Some(C_Decrypt as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE,
                                                                    _:
                                                                        *mut libc::c_uchar,
                                                                    _:
                                                                        libc::c_ulong,
                                                                    _:
                                                                        *mut libc::c_uchar,
                                                                    _:
                                                                        *mut libc::c_ulong)
                                                   -> CK_RV),
                                  C_DecryptUpdate:
                                      Some(C_DecryptUpdate as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE,
                                                                    _:
                                                                        *mut libc::c_uchar,
                                                                    _:
                                                                        libc::c_ulong,
                                                                    _:
                                                                        *mut libc::c_uchar,
                                                                    _:
                                                                        *mut libc::c_ulong)
                                                   -> CK_RV),
                                  C_DecryptFinal:
                                      Some(C_DecryptFinal as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE,
                                                                    _:
                                                                        *mut libc::c_uchar,
                                                                    _:
                                                                        *mut libc::c_ulong)
                                                   -> CK_RV),
                                  C_DigestInit:
                                      Some(C_DigestInit as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE,
                                                                    _:
                                                                        *mut _CK_MECHANISM)
                                                   -> CK_RV),
                                  C_Digest:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_Digest>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                              ->
                                                                                                                  CK_RV>,
                                                                                                   *mut libc::c_void>(Some(func_not_supported
                                                                                                                               as
                                                                                                                               unsafe extern "C" fn()
                                                                                                                                   ->
                                                                                                                                       CK_RV))),
                                  C_DigestUpdate:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_DigestUpdate>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                    ->
                                                                                                                        CK_RV>,
                                                                                                         *mut libc::c_void>(Some(func_not_supported
                                                                                                                                     as
                                                                                                                                     unsafe extern "C" fn()
                                                                                                                                         ->
                                                                                                                                             CK_RV))),
                                  C_DigestKey:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_DigestKey>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                 ->
                                                                                                                     CK_RV>,
                                                                                                      *mut libc::c_void>(Some(func_not_supported
                                                                                                                                  as
                                                                                                                                  unsafe extern "C" fn()
                                                                                                                                      ->
                                                                                                                                          CK_RV))),
                                  C_DigestFinal:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_DigestFinal>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                   ->
                                                                                                                       CK_RV>,
                                                                                                        *mut libc::c_void>(Some(func_not_supported
                                                                                                                                    as
                                                                                                                                    unsafe extern "C" fn()
                                                                                                                                        ->
                                                                                                                                            CK_RV))),
                                  C_SignInit:
                                      Some(C_SignInit as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE,
                                                                    _:
                                                                        *mut _CK_MECHANISM,
                                                                    _:
                                                                        CK_OBJECT_HANDLE)
                                                   -> CK_RV),
                                  C_Sign:
                                      Some(C_Sign as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE,
                                                                    _:
                                                                        *mut libc::c_uchar,
                                                                    _:
                                                                        libc::c_ulong,
                                                                    _:
                                                                        *mut libc::c_uchar,
                                                                    _:
                                                                        *mut libc::c_ulong)
                                                   -> CK_RV),
                                  C_SignUpdate:
                                      Some(C_SignUpdate as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE,
                                                                    _:
                                                                        *mut libc::c_uchar,
                                                                    _:
                                                                        libc::c_ulong)
                                                   -> CK_RV),
                                  C_SignFinal:
                                      Some(C_SignFinal as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE,
                                                                    _:
                                                                        *mut libc::c_uchar,
                                                                    _:
                                                                        *mut libc::c_ulong)
                                                   -> CK_RV),
                                  C_SignRecoverInit:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_SignRecoverInit>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                       ->
                                                                                                                           CK_RV>,
                                                                                                            *mut libc::c_void>(Some(func_not_supported
                                                                                                                                        as
                                                                                                                                        unsafe extern "C" fn()
                                                                                                                                            ->
                                                                                                                                                CK_RV))),
                                  C_SignRecover:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_SignRecover>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                   ->
                                                                                                                       CK_RV>,
                                                                                                        *mut libc::c_void>(Some(func_not_supported
                                                                                                                                    as
                                                                                                                                    unsafe extern "C" fn()
                                                                                                                                        ->
                                                                                                                                            CK_RV))),
                                  C_VerifyInit:
                                      Some(C_VerifyInit as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE,
                                                                    _:
                                                                        *mut _CK_MECHANISM,
                                                                    _:
                                                                        CK_OBJECT_HANDLE)
                                                   -> CK_RV),
                                  C_Verify:
                                      Some(C_Verify as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE,
                                                                    _:
                                                                        *mut libc::c_uchar,
                                                                    _:
                                                                        libc::c_ulong,
                                                                    _:
                                                                        *mut libc::c_uchar,
                                                                    _:
                                                                        libc::c_ulong)
                                                   -> CK_RV),
                                  C_VerifyUpdate:
                                      Some(C_VerifyUpdate as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE,
                                                                    _:
                                                                        *mut libc::c_uchar,
                                                                    _:
                                                                        libc::c_ulong)
                                                   -> CK_RV),
                                  C_VerifyFinal:
                                      Some(C_VerifyFinal as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE,
                                                                    _:
                                                                        *mut libc::c_uchar,
                                                                    _:
                                                                        libc::c_ulong)
                                                   -> CK_RV),
                                  C_VerifyRecoverInit:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_VerifyRecoverInit>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                         ->
                                                                                                                             CK_RV>,
                                                                                                              *mut libc::c_void>(Some(func_not_supported
                                                                                                                                          as
                                                                                                                                          unsafe extern "C" fn()
                                                                                                                                              ->
                                                                                                                                                  CK_RV))),
                                  C_VerifyRecover:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_VerifyRecover>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                     ->
                                                                                                                         CK_RV>,
                                                                                                          *mut libc::c_void>(Some(func_not_supported
                                                                                                                                      as
                                                                                                                                      unsafe extern "C" fn()
                                                                                                                                          ->
                                                                                                                                              CK_RV))),
                                  C_DigestEncryptUpdate:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_DigestEncryptUpdate>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                           ->
                                                                                                                               CK_RV>,
                                                                                                                *mut libc::c_void>(Some(func_not_supported
                                                                                                                                            as
                                                                                                                                            unsafe extern "C" fn()
                                                                                                                                                ->
                                                                                                                                                    CK_RV))),
                                  C_DecryptDigestUpdate:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_DecryptDigestUpdate>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                           ->
                                                                                                                               CK_RV>,
                                                                                                                *mut libc::c_void>(Some(func_not_supported
                                                                                                                                            as
                                                                                                                                            unsafe extern "C" fn()
                                                                                                                                                ->
                                                                                                                                                    CK_RV))),
                                  C_SignEncryptUpdate:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_SignEncryptUpdate>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                         ->
                                                                                                                             CK_RV>,
                                                                                                              *mut libc::c_void>(Some(func_not_supported
                                                                                                                                          as
                                                                                                                                          unsafe extern "C" fn()
                                                                                                                                              ->
                                                                                                                                                  CK_RV))),
                                  C_DecryptVerifyUpdate:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_DecryptVerifyUpdate>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                           ->
                                                                                                                               CK_RV>,
                                                                                                                *mut libc::c_void>(Some(func_not_supported
                                                                                                                                            as
                                                                                                                                            unsafe extern "C" fn()
                                                                                                                                                ->
                                                                                                                                                    CK_RV))),
                                  C_GenerateKey:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_GenerateKey>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                   ->
                                                                                                                       CK_RV>,
                                                                                                        *mut libc::c_void>(Some(func_not_supported
                                                                                                                                    as
                                                                                                                                    unsafe extern "C" fn()
                                                                                                                                        ->
                                                                                                                                            CK_RV))),
                                  C_GenerateKeyPair:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_GenerateKeyPair>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                       ->
                                                                                                                           CK_RV>,
                                                                                                            *mut libc::c_void>(Some(func_not_supported
                                                                                                                                        as
                                                                                                                                        unsafe extern "C" fn()
                                                                                                                                            ->
                                                                                                                                                CK_RV))),
                                  C_WrapKey:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_WrapKey>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                               ->
                                                                                                                   CK_RV>,
                                                                                                    *mut libc::c_void>(Some(func_not_supported
                                                                                                                                as
                                                                                                                                unsafe extern "C" fn()
                                                                                                                                    ->
                                                                                                                                        CK_RV))),
                                  C_UnwrapKey:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_UnwrapKey>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                 ->
                                                                                                                     CK_RV>,
                                                                                                      *mut libc::c_void>(Some(func_not_supported
                                                                                                                                  as
                                                                                                                                  unsafe extern "C" fn()
                                                                                                                                      ->
                                                                                                                                          CK_RV))),
                                  C_DeriveKey:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_DeriveKey>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                 ->
                                                                                                                     CK_RV>,
                                                                                                      *mut libc::c_void>(Some(func_not_supported
                                                                                                                                  as
                                                                                                                                  unsafe extern "C" fn()
                                                                                                                                      ->
                                                                                                                                          CK_RV))),
                                  C_SeedRandom:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_SeedRandom>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                  ->
                                                                                                                      CK_RV>,
                                                                                                       *mut libc::c_void>(Some(func_not_supported
                                                                                                                                   as
                                                                                                                                   unsafe extern "C" fn()
                                                                                                                                       ->
                                                                                                                                           CK_RV))),
                                  C_GenerateRandom:
                                      Some(C_GenerateRandom as
                                               unsafe extern "C" fn(_:
                                                                        CK_SESSION_HANDLE,
                                                                    _:
                                                                        *mut libc::c_uchar,
                                                                    _:
                                                                        libc::c_ulong)
                                                   -> CK_RV),
                                  C_GetFunctionStatus:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_GetFunctionStatus>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                         ->
                                                                                                                             CK_RV>,
                                                                                                              *mut libc::c_void>(Some(func_not_supported
                                                                                                                                          as
                                                                                                                                          unsafe extern "C" fn()
                                                                                                                                              ->
                                                                                                                                                  CK_RV))),
                                  C_CancelFunction:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_CancelFunction>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                      ->
                                                                                                                          CK_RV>,
                                                                                                           *mut libc::c_void>(Some(func_not_supported
                                                                                                                                       as
                                                                                                                                       unsafe extern "C" fn()
                                                                                                                                           ->
                                                                                                                                               CK_RV))),
                                  C_WaitForSlotEvent:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              CK_C_WaitForSlotEvent>(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                        ->
                                                                                                                            CK_RV>,
                                                                                                             *mut libc::c_void>(Some(func_not_supported
                                                                                                                                         as
                                                                                                                                         unsafe extern "C" fn()
                                                                                                                                             ->
                                                                                                                                                 CK_RV))),};
            init
        }
    };
