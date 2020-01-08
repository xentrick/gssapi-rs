use std::os::raw::c_void;
use std::mem::zeroed;

use gssapi::gssapi_h::{
    gss_buffer_desc_struct,
    OM_uint32,
    gss_name_t,
};
use gssapi::mglueP_h::{
    gss_name_struct,
};
use gssapi::src::generic::gssapi_generic::{
    GSS_C_NT_HOSTBASED_SERVICE,
};
use gssapi::src::mechglue::g_imp_name::{
    gss_import_name,
};

#[test]
fn get_token() {
    // Create mutable variable to capture major status of function call.
    let mut major: OM_uint32 = 0;

    // Create mutable variable to capture minor status
    let mut minor: OM_uint32 = 0;


    // Service name and protocol we want to get a `Name` object from.
    let mut host = String::from("http@fmd-anacon-01stg.vrt.sourcefire.com");
    let mut hostvec: Vec<u8> = "http@fmd-anacon-01stg.vrt.sourcefire.com".as_bytes().to_vec();

    unsafe {
        let mut hostbuf = &mut gss_buffer_desc_struct {
            length: host.len(),
            value: host.as_mut_ptr() as *mut c_void,
        };

        let mut host_ptr = &mut *hostbuf;

        // Create mutable `gss_name_t` that will receive output from gss_import_name
        let mut name_output: *mut gss_name_struct = zeroed();
        let mut name_ptr: *mut gss_name_t = name_output as *mut *mut gss_name_struct;

        // what is happening
        let mut hostbased = GSS_C_NT_HOSTBASED_SERVICE.clone();

        // Call gss_import_name() to get a `Name` object for Analyst Console
        major = gss_import_name(
            &mut minor,
            host_ptr,
            hostbased, // This is used because we have a name type of "service@host"
            name_ptr,
        );
        // Verify import via `minor` status
        match major {
            0 => println!("gss_name_import() successful!"),
            1 => println!("input_name_type was GSS_C_NT_EXPORT_NAME, but the mechanism contained within the input_name is not supported"),
            2 => println!("input_name paremeter could not be interpreted as a name of the specified type"),
            3 => println!("input_name_type was unrecognized"),
            13 => println!("The underlying mechanism detected an error for which no specific GSS status code is defined. Details in minor variable."),
            _ => (),
        }
    }
    // Return base64 version of token.
}
