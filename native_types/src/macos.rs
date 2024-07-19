use libc::{c_char, c_void};

pub type KernReturnT = i32;

pub type CFAllocatorRef = *const c_void;
pub type CFDictionaryRef = *const c_void;
pub type CFRunLoopRef = *const c_void;
pub type CFStringRef = *const c_void;
pub type CFTypeRef = *const c_void;
pub type CFStringEncoding = u32;
pub type CFNumberRef = *const c_void;
pub type CFMutableDictionaryRef = *const c_void;
pub type CFNumberType = CFIndex;
pub type CFIndex = i32;
pub type CFDictionaryKeyCallBacks = *const c_void;
pub type CFDictionaryValueCallBacks = *const c_void;

pub type IOOptionBits = u32;
pub type IOHIDManagerRef = *mut c_void;
pub type IOReturn = KernReturnT;
pub type IOHIDValueRef = *mut c_void;
pub type IOHIDElementRef = *mut c_void;
pub type IOHIDDeviceRef = *mut c_void;

pub const K_IOHIDMANAGER_OPTION_NONE: u32 = 0;
pub const K_CF_NUMBER_SINT32_TYPE: i32 = 3;
pub const K_HID_USAGE_PAGE_GENERIC: i32 = 0x01;
pub const K_HID_USAGE_KEYBOARD: i32 = 0x06;
pub const K_CFSTRING_ENCODING_UTF8: u32 = 0x08000100;
pub const kTISPropertyLocalizedName: *const c_void =
    b"TSMInputSourcePropertyLocalizedName\0".as_ptr() as *const c_void;
pub const kTISPropertyInputSourceID: *const c_void =
    b"TSMInputSourceID\0".as_ptr() as *const c_void;

extern "C" {
    pub static kCFAllocatorDefault: CFAllocatorRef;
    pub static kCFRunLoopDefaultMode: CFStringRef;
    /** Creates an IOHIDManager object.
     *
     * @param allocator Allocator to be used during creation.
     * @param options Use K_IOHIDMANAGER_OPTION_NONE or 0.
     * @result Returns a new IOHIDManagerRef.
     */
    pub fn IOHIDManagerCreate(allocator: CFAllocatorRef, options: IOOptionBits) -> IOHIDManagerRef;

    /** Opens the IOHIDManager.
     *
     * @param manager IOHIDManager to open.
     * @param options Use K_IOHIDMANAGER_OPTION_NONE or 0.
     */
    pub fn IOHIDManagerOpen(manager: IOHIDManagerRef, options: IOOptionBits);

    /** Sets matching criteria for device enumeration.
     *
     * @param manager Reference to an IOHIDManager.
     * @param matching CFDictionaryRef containing device matching criteria.
     */
    pub fn IOHIDManagerSetDeviceMatching(manager: IOHIDManagerRef, matching: CFDictionaryRef);

    /** Registers a callback to be used when an input value is issued by any enumerated device.
     * @param manager Reference to an IOHIDManagerRef.
     * @param callback Pointer to a callback method of type IOHIDValueCallback.
     * @param context Pointer to data to be passed to the callback.
     */
    pub fn IOHIDManagerRegisterInputValueCallback(
        manager: IOHIDManagerRef,
        callback: extern "C" fn(*mut c_void, IOReturn, *mut c_void, IOHIDValueRef),
        context: *mut c_void,
    );

    /** Schedules the HID Manager with the run loop.
     *
     * @param manager Reference to an IOHIDManagerRef.
     * @param run_loop RunLoop to be used when scheduling any asynchronous activity.
     * @param run_loop_mode Run loop mode to be used when scheduling any asynchronous activity.
     */
    pub fn IOHIDManagerScheduleWithRunLoop(
        manager: IOHIDManagerRef,
        run_loop: CFRunLoopRef,
        run_loop_mode: CFStringRef,
    );

    /** Returns the element value associated with this IOHIDValueRef.
     * @param value: The value to be queried. If this parameter is not a valid IOHIDValueRef, the behavior is undefined.
     * @result: Returns a IOHIDElementRef referenced by this value.
     */
    pub fn IOHIDValueGetElement(value: IOHIDValueRef) -> IOHIDElementRef;

    /** Obtain the device associated with the element.
     * @param element: IOHIDElement to be queried.
     * @result: Returns a reference to the device.
     */
    pub fn IOHIDElementGetDevice(element: IOHIDElementRef) -> IOHIDDeviceRef;

    /** Obtains a property from an IOHIDDevice.
     * @param device: Reference to an IOHIDDevice.
     * @param key: CFStringRef containing key to be used when querying the device.
     * @result: Returns CFTypeRef containing the property.
     */
    pub fn IOHIDDeviceGetProperty(device: IOHIDDeviceRef, key: CFStringRef) -> CFTypeRef;

    /** Returns the CFRunLoop object for the current thread.
     * @result: Returns the current thread’s run loop.
     */
    pub fn CFRunLoopGetCurrent() -> CFRunLoopRef;

    /** Runs the current thread’s CFRunLoop object in its default mode indefinitely.
     */
    pub fn CFRunLoopRun();

    /** Creates an immutable string from a C string.
     * @param allocator: The allocator to use to allocate memory for the new string. Pass NULL or kCFAllocatorDefault to use the current default allocator.
     * @param cStr: The NULL-terminated C string to be used to create the CFString object. The string must use an 8-bit encoding.
     * @param encoding: The encoding of the characters in the C string. The encoding must specify an 8-bit encoding.
     */
    pub fn CFStringCreateWithCString(
        allocator: CFAllocatorRef,
        cStr: *const c_char,
        encoding: CFStringEncoding,
    ) -> CFStringRef;

    /** Creates a CFNumber object using a specified value.
     * @param allocator: The allocator to use to allocate memory for the new object. Pass NULL or kCFAllocatorDefault to use the default allocator.
     * @param theType: A constant that specifies the data type of the value to convert.
     * @param valuePtr: A pointer to the value for the returned number object.
     * @result: A new number with the value specified by valuePtr.
     */
    pub fn CFNumberCreate(
        allocator: CFAllocatorRef,
        theType: CFNumberType,
        valuePtr: *const c_void,
    ) -> CFNumberRef;

    /** Creates a new mutable dictionary.
     * @param allocator: The allocator to use to allocate memory for the new dictionary and its storage for key-value pairs. Pass NULL or kCFAllocatorDefault to use the current default allocator.
     * @param capacity: The maximum number of key-value pairs that can be contained by the new dictionary. The dictionary starts empty and can grow to this number of key-value pairs (and it can have less). Pass 0 to specify that the maximum capacity is not limited. The value must not be negative.
     * @param keyCallBacks: A pointer to a CFDictionaryKeyCallBacks structure initialized with the callbacks to use to retain, release, describe, and compare keys in the dictionary.
     * @param valueCallBacks: A pointer to a CFDictionaryValueCallBacks structure initialized with the callbacks to use to retain, release, describe, and compare values in the dictionary.
     * @result: A new dictionary, or NULL if there was a problem creating the object.
     */
    pub fn CFDictionaryCreateMutable(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        keyCallBacks: CFDictionaryKeyCallBacks,
        valueCallBacks: CFDictionaryValueCallBacks,
    ) -> CFMutableDictionaryRef;

    /** Sets the value corresponding to a given key.
     * @param theDict: The dictionary to modify.
     * @param key: The key of the value to set in theDict.
     * @param value: The value to add to or replace in theDict.
     */
    pub fn CFDictionarySetValue(
        theDict: CFMutableDictionaryRef,
        key: *const c_void,
        value: *const c_void,
    );

    /** Releases a Core Foundation object.
     * @param cf: A CFType object to release. This value must not be NULL.
     */
    pub fn CFRelease(cf: CFTypeRef);

    /** Copies the character contents of a string to a local C string buffer after converting the characters to a given encoding.
     * @param theString: The string whose contents you wish to access.
     * @param buffer: The C string buffer into which to copy the string.
     * @param bufferSize: The length of buffer in bytes.
     * @param encoding: The string encoding to which the character contents of theString should be converted.
     * @result: true if the conversion is successful, otherwise false.
     */
    pub fn CFStringGetCString(
        theString: CFStringRef,
        buffer: *mut c_char,
        bufferSize: CFIndex,
        encoding: CFStringEncoding,
    ) -> bool;

    /** Obtains the value of a CFNumber object cast to a specified type.
     * @param number: The CFNumber object to examine.
     * @param theType: A constant that specifies the data type to return.
     * @param valuePtr: On return, contains the value of number.
     * @result: true if the value was obtained, otherwise false.
     */
    pub fn CFNumberGetValue(
        number: CFNumberRef,
        theType: CFNumberRef,
        valuePtr: *mut c_void,
    ) -> bool;

    //TODO: Add documentation
    fn TISCopyCurrentKeyboardInputSource() -> *mut c_void;
    fn TISGetInputSourceProperty(
        inputSource: *mut c_void,
        propertyKey: *const c_void,
    ) -> *const c_void;
    fn TISCreateInputSourceList(
        properties: *const c_void,
        includeAllInstalled: bool,
    ) -> *mut c_void;
    fn TISSelectInputSource(inputSource: *mut c_void);
}
