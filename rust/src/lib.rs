#![cfg(target_os="android")]
#![allow(non_snake_case)]
use std::ffi::{CStr};
use jni::objects::{JObject};
use android_ffi::ffi::{jobject, jstring, jboolean, JNIEnv, jbooleanArray};
extern crate rand;
extern crate jni;
mod font;
mod keyboard;
mod display;
mod cpu;
//mod jni_glue;

//use jni_glue::write_log;
extern crate android_ffi;
use android_ffi::{write_log, load_asset};
use android_ffi::ffi::{AAssetManager_fromJava};

use cpu::CPU;

static mut GAME_CPU: Option<CPU> = None;

unsafe fn jstring_to_str<'a>(env: *mut JNIEnv, j_recipient: jstring) -> &'a str{
    let input = ((*(*env).functions).GetStringUTFChars)(env, j_recipient, std::ptr::null_mut());
    CStr::from_ptr(input).to_str().unwrap_or("0")
}

#[no_mangle]
pub unsafe extern fn Java_com_rust_1react_1chip8_MobileAppBridge_initCpu(_env: *const JNIEnv, _ : jobject) -> jboolean {
    GAME_CPU = Some(CPU::new());
    GAME_CPU.get_or_insert(CPU::new()).reset();
    1 //always return true
}

#[no_mangle]
pub unsafe extern fn Java_com_rust_1react_1chip8_MobileAppBridge_hello(env: *mut JNIEnv, _: jobject, j_recipient: jstring) -> jstring {
    let converted_string = jstring_to_str(env, j_recipient);
    write_log(&format!("The input value: {}", converted_string));
    j_recipient
}

//com.react_rust.MobileAppBridge
//com.rust_react_chip8.MobileAppBridge
#[no_mangle]
pub unsafe extern fn Java_com_rust_1react_1chip8_MobileAppBridge_loadROM(env: *mut JNIEnv, _: jobject, asset_manager : jobject, file_name: jstring) -> jboolean {
    if file_name.is_null() {
        return 0;
    }

    //get ROM name
    let rom_name = jstring_to_str(env, file_name);
    //Load ROM to memory
    let res = AAssetManager_fromJava(env, asset_manager);
    if res.is_null() {
        return 0;
    } else {
        return match load_asset(rom_name, res) {
            Ok(value) => {
                GAME_CPU.get_or_insert(CPU::new()).load_rom(value);
                1
            },
            _ => {
                0
            },
        }
    }
}

#[no_mangle]
pub unsafe extern fn Java_com_rust_1react_1chip8_MobileAppBridge_pressButton(env: *mut JNIEnv, _ : JObject, j_key: jstring, j_value: jboolean) {
    let converted_string = jstring_to_str(env, j_key);
    let key_idx = usize::from_str_radix(converted_string, 16).unwrap_or(0);
    if let Some(ref mut cpu ) = GAME_CPU{
        match j_value {
            1 => cpu.press_key(key_idx, true),
            _ => cpu.press_key(key_idx, false),
        }
    }
}

#[no_mangle]
pub unsafe extern fn Java_com_rust_1react_1chip8_MobileAppBridge_excuteCycle(_env: *mut JNIEnv, _ : JObject) -> jboolean{
    if let Some(ref mut value) = GAME_CPU {
        value.execute_cycle();
        if value.get_re_draw() {
            return 1;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern fn Java_com_rust_1react_1chip8_MobileAppBridge_getDisplayMem(env: *mut JNIEnv, _: JObject) -> jbooleanArray{
    let output_array = ((*(*env).functions).NewBooleanArray)(env, 2048i32);
    if let Some(ref mut cpu) = GAME_CPU {
        let return_ptr = cpu.get_display().get_mem().as_ptr() as *const jboolean;
        ((*(*env).functions).SetBooleanArrayRegion)(env, output_array, 0, 2048, return_ptr);
    }
    output_array
}

#[no_mangle]
pub unsafe extern fn Java_com_rust_1react_1chip8_MobileAppBridge_decreaseTimer(_env: *mut JNIEnv, _: JObject){
    if let Some(ref mut cpu) = GAME_CPU {
        cpu.delay_desc();
    }
}