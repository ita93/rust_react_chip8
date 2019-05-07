#![cfg(target_os="android")]
#![allow(non_snake_case)]

use std::ffi::{CString, CStr};
use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::{jstring};

use std::os::raw::c_void;
use std::os::raw::c_float;
use std::os::raw::c_double;
use std::os::raw::c_char;
use std::os::raw::c_schar;
use std::os::raw::c_uchar;
use std::os::raw::c_int;
use std::os::raw::c_short;
use std::os::raw::c_ushort;
use std::os::raw::c_longlong;

extern { pub fn __android_log_write(prio: c_int, tag: *const c_char, text: *const c_char) -> c_int; }

pub fn write_log(message: &str) {
    let message = CString::new(message).unwrap();
    let message = message.as_ptr();
    let tag = CString::new("RustAndroidGlueStdouterr").unwrap();
    let tag = tag.as_ptr();
    unsafe { __android_log_write(2 as i32, tag, message) };
}