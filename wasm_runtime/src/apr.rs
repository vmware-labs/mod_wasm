use std::collections::HashMap;
use std::sync::RwLock;

use once_cell::sync::Lazy;

pub trait AprInterface: Sync + Send {
    fn get_header(&self, headers_ptr: u64, key: &str) -> Option<String>;
    fn set_header(&self, headers_ptr: u64, key: &str, value: &str);
    fn delete_header(&self, headers_ptr: u64, key: &str);
}

impl dyn AprInterface {}

pub static APR_INTERFACES: Lazy<RwLock<HashMap<String, Box<dyn AprInterface + 'static>>>> =
    Lazy::new(|| {
        let data: HashMap<String, Box<dyn AprInterface + 'static>> = HashMap::new();
        RwLock::new(data)
    });

pub mod c_api {
    use crate::ffi_utils::*;
    use std::ffi::{c_char, c_void};

    #[repr(C)]
    pub struct Headers {
        _data: [u8; 0],
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    type GetHeaderCb =
        unsafe extern "C" fn(headers: *mut Headers, key: *const c_char) -> *const c_char;
    type SetHeaderCb = unsafe extern "C" fn(
        headers: *mut Headers,
        key: *const c_char,
        value: *const c_char,
    ) -> c_void;
    type DeleteHeaderCb = unsafe extern "C" fn(headers: *mut Headers, key: *const c_char) -> c_void;

    pub struct Apr {
        get_header_cb: Option<GetHeaderCb>,
        set_header_cb: Option<SetHeaderCb>,
        delete_header_cb: Option<DeleteHeaderCb>,
    }

    impl crate::apr::AprInterface for Apr {
        fn get_header(&self, headers_ptr: u64, key: &str) -> Option<String> {
            unsafe {
                match self.get_header_cb {
                    Some(cb) => {
                        let headers = headers_ptr as *mut Headers;
                        let key_ptr = str_to_c_char(key);
                        let result = cb(headers, key_ptr);
                        match result.as_ref() {
                            Some(_) => Some(const_c_char_to_str(result).to_string()),
                            None => None,
                        }
                    }
                    None => None,
                }
            }
        }
        fn set_header(&self, headers_ptr: u64, key: &str, value: &str) {
            unsafe {
                if let Some(cb) = self.set_header_cb {
                    cb(
                        headers_ptr as *mut Headers,
                        str_to_c_char(key),
                        str_to_c_char(value),
                    );
                }
            }
            ()
        }

        fn delete_header(&self, headers_ptr: u64, key: &str) {
            unsafe {
                if let Some(cb) = self.delete_header_cb {
                    cb(headers_ptr as *mut Headers, str_to_c_char(key));
                }
            }
            ()
        }
    }

    pub fn set_apr_callbacks(
        config_id: &str,
        get_header_cb: Option<GetHeaderCb>,
        set_header_cb: Option<SetHeaderCb>,
        delete_header_cb: Option<DeleteHeaderCb>,
    ) {
        if let Some(_) = get_header_cb {
            let mut apr_interfaces = crate::apr::APR_INTERFACES
                .write()
                .expect("ERROR! Poisoned RwLock APR_INTERFACES on write()");
            if !apr_interfaces.contains_key(config_id) {
                let apr: Box<Apr> = Box::new(Apr {
                    get_header_cb,
                    set_header_cb,
                    delete_header_cb,
                });

                apr_interfaces.insert(config_id.to_string(), apr);
            }
        }
    }
}

pub mod wasm_host {
    use std::ffi::CStr;
    use wasi_common::WasiCtx;
    use wasmtime::{Caller, Linker};

    fn get_header_impl(config_id: String, header_ptr: u64, key: *const u8) -> Option<String> {
        let apr_interfaces = crate::apr::APR_INTERFACES
            .read()
            .expect("ERROR! Poisoned RwLock APR_INTERFACES on read()");

        let apr = match apr_interfaces.get(&config_id) {
            Some(a) => a,
            None => return None,
        };

        unsafe {
            let key_str = CStr::from_ptr(key as *const i8);

            apr.get_header(header_ptr, key_str.to_str().unwrap())
        }
    }

    fn set_header_impl(config_id: String, header_ptr: u64, key: *const u8, value: *const u8) {
        let apr_interfaces = crate::apr::APR_INTERFACES
            .read()
            .expect("ERROR! Poisoned RwLock APR_INTERFACES on read()");

        let apr = match apr_interfaces.get(&config_id) {
            Some(a) => a,
            None => return (),
        };

        unsafe {
            let key_str = CStr::from_ptr(key as *const i8);
            let value_ptr = CStr::from_ptr(value as *const i8);

            apr.set_header(
                header_ptr,
                key_str.to_str().unwrap(),
                value_ptr.to_str().unwrap(),
            );
        }
    }

    fn delete_header_impl(config_id: String, header_ptr: u64, key: *const u8) {
        let apr_interfaces = crate::apr::APR_INTERFACES
            .read()
            .expect("ERROR! Poisoned RwLock APR_INTERFACES on read()");

        let apr = match apr_interfaces.get(&config_id) {
            Some(a) => a,
            None => return (),
        };

        unsafe {
            let key_str = CStr::from_ptr(key as *const i8);

            apr.delete_header(header_ptr, key_str.to_str().unwrap());
        }
    }

    pub fn register_host_functions(linker: &mut Linker<WasiCtx>, config_id: String) {
        let config_id_get_header: String = config_id.clone();
        linker.func_wrap(
            "apr",
            "get_header",
            move |mut caller: Caller<'_, WasiCtx>, headers: u64, key: i32| -> i32 {
                let memory = caller.get_export("memory").unwrap().into_memory().unwrap();

                let ptr_native = unsafe { memory.data_ptr(&caller).offset(key as isize) };

                let value = get_header_impl(config_id_get_header.clone(), headers, ptr_native);

                match value {
                    Some(v) => {
                        let alloc = caller
                            .get_export("allocate")
                            .unwrap()
                            .into_func()
                            .unwrap()
                            .typed::<i32, i32>(&caller)
                            .unwrap();
                        // let free = caller.get_export("deallocate").unwrap().into_func().unwrap().typed::<(i32, i32), ()>(&caller).unwrap();

                        let ptr = alloc.call(&mut caller, v.len() as i32).unwrap();

                        memory.write(&mut caller, ptr as usize, v.as_bytes()).ok();
                        ptr as i32
                        // free.call(&mut caller, (ptr, v.len() as i32)).unwrap();
                    }
                    None => 0,
                }
            },
        ).ok();

        let config_id_set_header: String = config_id.clone();
        linker.func_wrap(
            "apr",
            "set_header",
            move |mut caller: Caller<'_, WasiCtx>, headers: u64, key: i32, value: i32| {
                let memory = caller.get_export("memory").unwrap().into_memory().unwrap();

                let key_native = unsafe { memory.data_ptr(&caller).offset(key as isize) };
                let value_native = unsafe { memory.data_ptr(caller).offset(value as isize) };

                set_header_impl(
                    config_id_set_header.clone(),
                    headers,
                    key_native,
                    value_native,
                );
            },
        ).ok();

        let config_id_delete_header: String = config_id.clone();
        linker.func_wrap(
            "apr",
            "delete_header",
            move |mut caller: Caller<'_, WasiCtx>, headers: u64, key: i32| {
                let memory = caller.get_export("memory").unwrap().into_memory().unwrap();

                let key_native = unsafe { memory.data_ptr(&caller).offset(key as isize) };
                delete_header_impl(config_id_delete_header.clone(), headers, key_native);
            },
        ).ok();
    }
}
