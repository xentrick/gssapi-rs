#[test]
fn context {
    // Create mutable variable to capture major status of function call.
    let mut major: u32 = 0;
    // Create mutable variable to capture minor status
    let mut minor: u32 = 0;
    // Service name and protocol we want to get a `Name` object from.
    let host: Vec<u8> = "http@fmd-anacon-01stg.vrt.sourcefire.com".as_bytes().to_vec();
    let mut hostbuf = gss_buffer_desc_struct::new( host );
    // Create mutable `gss_name_t` that will receive output from gss_import_name
    let mut name_output = gss_name_struct::new();
    // what is happening
    let mut hostbased = GSS_C_NT_HOSTBASED_SERVICE.clone();

    // Call gss_import_name() to get a `Name` object for Analyst Console
    major = gss_import_name(
        &mut minor,
        &mut hostbuf,
        &mut hostbased, // This is used because we have a name type of "service@host"
        &mut name_output,
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
    // Return base64 version of token.
    String::from("placeholder")
}
