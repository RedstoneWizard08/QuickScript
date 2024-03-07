use parking_lot::RwLock;
use std::{collections::HashMap, ffi::CString, sync::Arc};

#[cfg(not(windows))]
fn lookup_with_dlsym(name: &str) -> Option<*const u8> {
    let c_str = CString::new(name).unwrap();
    let c_str_ptr = c_str.as_ptr();
    let sym = unsafe { libc::dlsym(libc::RTLD_DEFAULT, c_str_ptr) };

    if sym.is_null() {
        None
    } else {
        debug!("Found symbol {} at address {:?}", name, sym as *const u8);

        Some(sym as *const u8)
    }
}

#[cfg(windows)]
fn lookup_with_dlsym(name: &str) -> Option<*const u8> {
    use std::os::windows::io::RawHandle;
    use windows_sys::Win32::Foundation::HMODULE;
    use windows_sys::Win32::System::LibraryLoader;

    const UCRTBASE: &[u8] = b"ucrtbase.dll\0";

    let c_str = CString::new(name).unwrap();
    let c_str_ptr = c_str.as_ptr();

    unsafe {
        let handles = [
            // try to find the searched symbol in the currently running executable
            ptr::null_mut(),
            // try to find the searched symbol in local c runtime
            LibraryLoader::GetModuleHandleA(UCRTBASE.as_ptr()) as RawHandle,
        ];

        for handle in &handles {
            let addr = LibraryLoader::GetProcAddress(*handle as HMODULE, c_str_ptr.cast());

            match addr {
                None => continue,
                Some(addr) => {
                    debug!("Found symbol {} at address {:?}", name, addr as *const u8);

                    return Some(addr as *const u8);
                }
            }
        }

        None
    }
}

pub fn lookup_symbol<'a>(
    map: Arc<RwLock<HashMap<String, (String, *const u8, usize)>>>,
) -> Box<dyn Fn(&str) -> Option<*const u8> + 'a> {
    Box::new(move |name| {
        debug!("Looking for symbol: {}", name);

        if let Some((_, ptr, size)) = map.read().get(name) {
            debug!(
                "Found symbol {} at address {:?} ({} bytes)",
                name, *ptr, size
            );

            Some(*ptr)
        } else {
            lookup_with_dlsym(name)
        }
    })
}
