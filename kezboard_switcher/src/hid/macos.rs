use std::{
    ffi::{c_int, c_void, CStr, CString},
    ptr,
    sync::mpsc::Sender,
};

use crate::{hid::HIDManager, DeviceInformation};
use native_types::macos::*;

pub struct MacOSHIDManager;

impl HIDManager for MacOSHIDManager {
    fn initialize(sender: Sender<DeviceInformation>) -> Result<(), String> {
        println!("Initializing macOS HID Manager");
        unsafe {
            let hid_manager = IOHIDManagerCreate(kCFAllocatorDefault, K_IOHIDMANAGER_OPTION_NONE);
            if hid_manager.is_null() {
                return Err(String::from("Failed to create HID Manger"));
            }

            // Create a matching dictionary for keyboards
            let matching_dict =
                CFDictionaryCreateMutable(kCFAllocatorDefault, 0, ptr::null(), ptr::null());
            let usage_page_key = CFStringCreateWithCString(
                kCFAllocatorDefault,
                CString::new("UsagePage").unwrap().as_ptr(),
                K_CFSTRING_ENCODING_UTF8,
            );
            let usage_key = CFStringCreateWithCString(
                kCFAllocatorDefault,
                CString::new("Usage").unwrap().as_ptr(),
                K_CFSTRING_ENCODING_UTF8,
            );
            let usage_page_value = CFNumberCreate(
                kCFAllocatorDefault,
                K_CF_NUMBER_SINT32_TYPE,
                &K_HID_USAGE_PAGE_GENERIC as *const _ as *const c_void,
            );
            let usage_value = CFNumberCreate(
                kCFAllocatorDefault,
                K_CF_NUMBER_SINT32_TYPE,
                &K_HID_USAGE_KEYBOARD as *const _ as *const c_void,
            );

            if usage_page_key.is_null()
                || usage_key.is_null()
                || usage_page_value.is_null()
                || usage_value.is_null()
            {
                return Err(String::from("Failed to create matching dictionary values"));
            }

            CFDictionarySetValue(
                matching_dict,
                usage_page_key as CFTypeRef,
                usage_page_value as CFTypeRef,
            );
            CFDictionarySetValue(
                matching_dict,
                usage_key as CFTypeRef,
                usage_value as CFTypeRef,
            );

            let sender_box = Box::new(sender);
            let context = Box::into_raw(sender_box) as *mut c_void;

            IOHIDManagerSetDeviceMatching(hid_manager, matching_dict);
            IOHIDManagerRegisterInputValueCallback(
                hid_manager,
                handle_input_value_callback,
                context,
            );

            let run_loop = CFRunLoopGetCurrent();
            IOHIDManagerScheduleWithRunLoop(hid_manager, run_loop, kCFRunLoopDefaultMode);

            IOHIDManagerOpen(hid_manager, K_IOHIDMANAGER_OPTION_NONE);

            // Release the matching dictionary and keys
            CFRelease(matching_dict as CFTypeRef);
            CFRelease(usage_page_key as CFTypeRef);
            CFRelease(usage_key as CFTypeRef);
            CFRelease(usage_page_value as CFTypeRef);
            CFRelease(usage_value as CFTypeRef);

            CFRunLoopRun();
        }
        Ok(())
    }
}

extern "C" fn handle_input_value_callback(
    context: *mut c_void,
    _result: IOReturn,
    _sender: *mut c_void,
    value: IOHIDValueRef,
) {
    unsafe {
        let sender: &Sender<DeviceInformation> = &*(context as *const Sender<DeviceInformation>);

        let element = IOHIDValueGetElement(value);
        let device = IOHIDElementGetDevice(element);

        let vendor_id_key = CString::new("VendorID").unwrap();
        let product_id_key = CString::new("ProductID").unwrap();
        let manufacturer_key = CString::new("Manufacturer").unwrap();
        let product_key = CString::new("Product").unwrap();

        let vendor_id_key_ref = CFStringCreateWithCString(
            kCFAllocatorDefault,
            vendor_id_key.as_ptr(),
            K_CFSTRING_ENCODING_UTF8,
        );
        let product_id_key_ref = CFStringCreateWithCString(
            kCFAllocatorDefault,
            product_id_key.as_ptr(),
            K_CFSTRING_ENCODING_UTF8,
        );
        let manufacturer_key_ref = CFStringCreateWithCString(
            kCFAllocatorDefault,
            manufacturer_key.as_ptr(),
            K_CFSTRING_ENCODING_UTF8,
        );
        let product_key_ref = CFStringCreateWithCString(
            kCFAllocatorDefault,
            product_key.as_ptr(),
            K_CFSTRING_ENCODING_UTF8,
        );

        if vendor_id_key_ref.is_null()
            || product_id_key_ref.is_null()
            || manufacturer_key_ref.is_null()
            || product_key_ref.is_null()
        {
            eprintln!("Failed to create CFString keys");
            return;
        }

        let vendor_id_ref = IOHIDDeviceGetProperty(device, vendor_id_key_ref);
        let product_id_ref = IOHIDDeviceGetProperty(device, product_id_key_ref);
        let manufacturer_ref = IOHIDDeviceGetProperty(device, manufacturer_key_ref);
        let product_ref = IOHIDDeviceGetProperty(device, product_key_ref);

        let mut vendor_id = 0;
        let mut product_id = 0;
        let mut manufacturer = String::from("Unknown Manufacturer");
        let mut product = String::from("Unknown Product");

        if !vendor_id_ref.is_null() {
            CFNumberGetValue(
                vendor_id_ref as CFNumberRef,
                K_CF_NUMBER_SINT32_TYPE as *const c_void,
                &mut vendor_id as *mut _ as *mut c_void,
            );
        }

        if !product_id_ref.is_null() {
            CFNumberGetValue(
                product_id_ref as CFNumberRef,
                K_CF_NUMBER_SINT32_TYPE as *const c_void,
                &mut product_id as *mut _ as *mut c_void,
            );
        }

        if !manufacturer_ref.is_null() {
            let mut buffer = [0i8; 128];
            if CFStringGetCString(
                manufacturer_ref as CFStringRef,
                buffer.as_mut_ptr(),
                buffer.len() as c_int,
                K_CFSTRING_ENCODING_UTF8,
            ) {
                manufacturer = CStr::from_ptr(buffer.as_ptr())
                    .to_string_lossy()
                    .into_owned();
            }
        }

        if !product_ref.is_null() {
            let mut buffer = [0i8; 128];
            if CFStringGetCString(
                product_ref as CFStringRef,
                buffer.as_mut_ptr(),
                buffer.len() as c_int,
                K_CFSTRING_ENCODING_UTF8,
            ) {
                product = CStr::from_ptr(buffer.as_ptr())
                    .to_string_lossy()
                    .into_owned();
            }
        }

        let device_info = DeviceInformation {
            vendor_id,
            product_id,
            manufacturer,
            product,
        };

        if let Err(e) = sender.send(device_info) {
            eprintln!("Error sending device information: {}", e);
        }

        // Release the keys created with CFStringCreateWithCString
        CFRelease(vendor_id_key_ref as CFTypeRef);
        CFRelease(product_id_key_ref as CFTypeRef);
        CFRelease(manufacturer_key_ref as CFTypeRef);
        CFRelease(product_key_ref as CFTypeRef);
    }
}
