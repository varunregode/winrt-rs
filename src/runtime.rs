use crate::{hstring, ErrorCode, Guid, RawPtr};

#[link(name = "kernel32")]
extern "system" {
    pub fn GetProcessHeap() -> RawPtr;
    pub fn HeapAlloc(heap: RawPtr, flags: u32, bytes: usize) -> RawPtr;
    pub fn HeapFree(heap: RawPtr, flags: u32, ptr: RawPtr) -> i32;

    pub fn CreateEventW(security: RawPtr, manual: i32, state: i32, name: RawPtr) -> RawPtr;
    pub fn SetEvent(handle: RawPtr) -> i32;
    pub fn WaitForSingleObject(handle: RawPtr, milliseconds: u32) -> u32;
    pub fn CloseHandle(handle: RawPtr) -> i32;

    pub fn FormatMessageW(
        flags: u32,
        source: RawPtr,
        code: u32,
        language: u32,
        buffer: *mut u16,
        size: u32,
        args: RawPtr,
    ) -> u32;
}

#[link(name = "onecore")]
extern "system" {
    pub fn CoIncrementMTAUsage(cookie: *mut RawPtr) -> ErrorCode;

    pub fn RoGetActivationFactory(
        hstring: *mut hstring::Header,
        interface: &Guid,
        result: *mut RawPtr,
    ) -> ErrorCode;
}

#[link(name = "oleaut32")]
extern "system" {
    pub fn SysStringLen(bstr: RawPtr) -> u32;
    pub fn SysFreeString(bstr: RawPtr);
    pub fn GetErrorInfo(reserved: u32, info: *mut RawPtr) -> ErrorCode;
    pub fn SetErrorInfo(reserved: u32, info: RawPtr) -> ErrorCode;
}
