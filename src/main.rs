use ofi_libfabric_sys::bindgen as ffi;
use std::ffi::CString;
use std::ptr;

fn test_get_info() {
    unsafe {
        // Configure hints.
        let hints = ffi::fi_allocinfo();
        assert_eq!(hints.is_null(), false);

        (*hints).caps = ffi::FI_MSG as u64;
        (*hints).mode = ffi::FI_CONTEXT;
        (*(*hints).ep_attr).type_ = ffi::fi_ep_type_FI_EP_RDM;
        (*(*hints).domain_attr).mr_mode = ffi::FI_MR_LOCAL as i32;
        let prov_name = CString::new("tcp").unwrap();
        (*(*hints).fabric_attr).prov_name = prov_name.into_raw() as *mut i8;

        // Get Fabric info based on the hints.
        let mut info_ptr = ptr::null_mut();
        let version = ffi::fi_version();
        let ret = ffi::fi_getinfo(
            version,
            ptr::null_mut(),
            ptr::null_mut(),
            0,
            hints,
            &mut info_ptr,
        );

        assert_eq!(ret, 0);

        // Free the info structure returned by fi_getinfo.
        if !info_ptr.is_null() {
            ffi::fi_freeinfo(info_ptr);
        }

        // Free the hints structure we allocated.
        ffi::fi_freeinfo(hints);
    }
}

fn main() {
    test_get_info();
    println!("Hello, world!");
}
