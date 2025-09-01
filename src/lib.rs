#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use crate::{
        xmlCleanupParser, xmlErrorPtr, xmlInitParser, xmlSchemaCleanupTypes, xmlSchemaFree,
        xmlSchemaFreeParserCtxt, xmlSchemaNewParserCtxt, xmlSchemaParse, xmlSchemaParserCtxtPtr,
        xmlSchemaPtr, xmlSchemaSetParserStructuredErrors,
    };
    use ctor::{ctor, dtor};
    use serial_test::serial;
    use std::ffi::{CStr, CString, c_void};
    use std::path::Path;
    use workspace_root::get_workspace_root;

    struct ValidationErrors {
        errors: Vec<String>,
    }

    extern "C" fn structured_error_handler(user_data: *mut c_void, error: xmlErrorPtr) {
        if error.is_null() {
            return;
        }

        let line = unsafe { (*error).line };
        let column = unsafe { (*error).int2 };
        let message = unsafe { CStr::from_ptr((*error).message).to_str().unwrap() };

        let context = unsafe { &mut *(user_data as *mut ValidationErrors) };

        let error = format!("Error: Line {}, column {}: {}", line, column, message);

        context.errors.push(error);
    }

    struct SchemaParserContext(xmlSchemaParserCtxtPtr);

    impl SchemaParserContext {
        fn new(path: &Path) -> Result<Self, Vec<String>> {
            let path_str = match path.to_str() {
                Some(str) => str,
                None => return Err(vec![format!("Invalid path: {}", path.to_str().unwrap())]),
            };

            let c_path = match CString::new(path_str) {
                Ok(str) => str,
                Err(e) => return Err(vec![format!("Error: {}", e.to_string())]),
            };

            let context_ptr = unsafe { xmlSchemaNewParserCtxt(c_path.as_ptr()) };

            if context_ptr.is_null() {
                let err: Vec<String> =
                    Vec::from(["Failed to create schema parser context".to_string()]);

                Err(err)
            } else {
                Ok(Self(context_ptr))
            }
        }
    }

    impl Drop for SchemaParserContext {
        fn drop(&mut self) {
            unsafe { xmlSchemaFreeParserCtxt(self.0) };
        }
    }

    struct Schema(xmlSchemaPtr);

    impl Drop for Schema {
        fn drop(&mut self) {
            unsafe { xmlSchemaFree(self.0) };
        }
    }

    #[ctor]
    fn init() {
        println!("Setting up xml schema environment");
        unsafe { xmlInitParser() };
    }

    #[dtor]
    fn clean_up() {
        println!("Cleaning up xml schema");
        unsafe {
            xmlSchemaCleanupTypes();
            xmlCleanupParser();
        }
    }

    fn validate_xsd_schema(schema_file: &Path) -> Result<(), Vec<String>> {
        let mut error_context = ValidationErrors { errors: Vec::new() };

        let result: Result<(), Vec<String>> = (|| {
            // Create the parser context using our safe wrapper.
            let parser_context = SchemaParserContext::new(schema_file)?;

            // Set the structured error handler. This is the modern, safe way.
            unsafe {
                xmlSchemaSetParserStructuredErrors(
                    parser_context.0,
                    Some(structured_error_handler),
                    &mut error_context as *mut _ as *mut c_void,
                );
            }

            // Parse the schema. The result will be managed by our `Schema` wrapper.
            let schema_ptr = unsafe { xmlSchemaParse(parser_context.0) };
            if schema_ptr.is_null() {
                // If parsing fails, the error handler should have been called.
                // We return an empty error if none were captured for some reason.
                return Err(vec![
                    "Schema parsing failed: file is not a valid XSD schema.".to_string(),
                ]);
            }

            // Wrap the raw pointer in our RAII guard to ensure it's freed.
            let _schema = Schema(schema_ptr);

            // If the schema was parsed but we collected errors, it's still a failure.
            if !error_context.errors.is_empty() {
                return Err(Vec::new()); // Errors are already in error_context
            }

            Ok(())
        })();

        match result {
            Ok(()) => Ok(()),
            Err(mut e) => {
                e.extend(error_context.errors);

                Err(e)
            }
        }
    }

    #[serial]
    #[test]
    fn test_xsd() {
        let root = get_workspace_root();
        let path = root.join("examples").join("valid.xsd");
        println!("Path: {}", path.display());

        let result = validate_xsd_schema(path.as_path());
        assert!(result.is_ok());
    }

    #[serial]
    #[test]
    fn test_invalid_xsd() {
        let root = get_workspace_root();
        let path = root.join("examples").join("invalid.xsd");
        println!("Path: {}", path.display());

        let result = validate_xsd_schema(path.as_path());
        assert!(result.is_err());
    }
}
