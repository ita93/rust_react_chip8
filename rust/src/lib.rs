#![cfg(target_os="android")]
#![allow(non_snake_case)]
use std::ffi::{CString, CStr};
use jni::objects::{JObject, JString};
use android_ffi::ffi::{jobject, jstring, jboolean, JNIEnv};
use std::os::raw::{c_void, c_char};

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
use android_ffi::ffi::{AAssetManager_fromJava, AAssetManager, JNINativeInterface};

use cpu::CPU;
use display::Display;
use keyboard::Keyboard;

static mut GAME_CPU: Option<CPU> = None;

unsafe fn jstring_to_str<'a>(env: *mut JNIEnv, j_recipient: jstring) -> &'a str{
    let input = ((*(*env).functions).GetStringUTFChars)(env, j_recipient, std::ptr::null_mut());
    CStr::from_ptr(input).to_str().unwrap_or("0")
}

#[no_mangle]
pub unsafe extern fn Java_com_rust_1react_1chip8_MobileAppBridge_initCpu(env: *const JNIEnv, _ : jobject) -> jboolean {
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
    
    if j_value == 1 {
        GAME_CPU.get_or_insert(CPU::new()).press_key(key_idx, true);
    }else{
        GAME_CPU.get_or_insert(CPU::new()).press_key(key_idx, false);
    }

    write_log(&format!("Key: {} and value: {}", key_idx, j_value));
}

#[no_mangle]
pub unsafe extern fn Java_com_rust_1react_1chip8_MobileAppBridge_excuteCycle(env: *mut JNIEnv, _ : JObject, j_key: jstring) -> jboolean{
    GAME_CPU.get_or_insert(CPU::new()).execute_cycle();
    1 //FIX ME: return true when this is a Draw instruction, otherwise return 0.
}

/*
#[no_mangle]
pub fn reset() {
    unsafe {
        Cpu.reset();
    }
}

#[no_mangle]
pub fn get_memory() -> &'static [u8; 4096] {
    unsafe {
        &Cpu.memory
    }
}

#[no_mangle]
pub fn get_display() -> &'static [bool; 2048] {
    unsafe {
        &Cpu.display.memory
    }
}

#[no_mangle]
pub fn get_register_v() -> &'static [u8; 16] {
    unsafe {
        &Cpu.registers
    }
}

#[no_mangle]
pub fn get_register_i() -> u16 {
    unsafe {
        Cpu.ir
    }
}

#[no_mangle]
pub fn get_register_pc() -> u16 {
    unsafe {
        Cpu.pc
    }
}



#[no_mangle]
pub fn decrement_timers() {
    unsafe {
        Cpu.delay_desc();
    }
}
*/