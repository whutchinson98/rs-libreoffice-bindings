use std::ffi::{CStr, CString};
use std::path::Path;
use std::sync::Mutex;

include!(concat!("./generated", "/bindings.rs"));

pub struct Office {
    handle: *mut LibreOfficeKit,
    mutex: Mutex<()>,
}

impl Office {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let c_path = CString::new(path.as_ref().to_string_lossy().as_bytes())
            .map_err(|e| format!("Failed to create CString: {}", e))?;

        let handle = unsafe { initialize(c_path.as_ptr()) };

        if handle.is_null() {
            return Err(format!(
                "Failed to initialize LibreOfficeKit with path: '{}'",
                path.as_ref().display()
            ));
        }

        Ok(Office {
            handle,
            mutex: Mutex::new(()),
        })
    }

    pub fn get_error(&self) -> String {
        unsafe {
            let message = get_error(self.handle);
            if message.is_null() {
                return String::from("No error message available");
            }

            let c_str = CStr::from_ptr(message);
            let result = c_str.to_string_lossy().into_owned();

            // Free the error message if necessary
            free_error(self.handle, message);

            result
        }
    }

    pub fn load_document<P: AsRef<Path>>(&self, path: P) -> Result<Document, String> {
        let _lock = self.mutex.lock().unwrap();

        let c_path = CString::new(path.as_ref().to_string_lossy().as_bytes())
            .map_err(|e| format!("Failed to create CString: {}", e))?;

        let handle = unsafe { document_load(self.handle, c_path.as_ptr()) };

        if handle.is_null() {
            return Err(format!("Failed to load document: {}", self.get_error()));
        }

        Ok(Document { handle })
    }
}

impl Drop for Office {
    fn drop(&mut self) {
        unsafe {
            destroy_office(self.handle);
        }
    }
}

pub struct Document {
    handle: *mut LibreOfficeKitDocument,
}

impl Document {
    pub fn save_as<P: AsRef<Path>>(
        &self,
        path: P,
        format: &str,
        filter: &str,
    ) -> Result<(), String> {
        let c_path = CString::new(path.as_ref().to_string_lossy().as_bytes())
            .map_err(|e| format!("Failed to create CString for path: {}", e))?;

        let c_format = CString::new(format)
            .map_err(|e| format!("Failed to create CString for format: {}", e))?;

        let c_filter = CString::new(filter)
            .map_err(|e| format!("Failed to create CString for filter: {}", e))?;

        let status = unsafe {
            document_save(
                self.handle,
                c_path.as_ptr(),
                c_format.as_ptr(),
                c_filter.as_ptr(),
            )
        };

        if status != 1 {
            return Err(String::from("Failed to save document"));
        }

        Ok(())
    }
}

impl Drop for Document {
    fn drop(&mut self) {
        unsafe {
            destroy_document(self.handle);
        }
    }
}
