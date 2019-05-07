#![cfg(target_os="android")]
#![allow(non_snake_case)]
use std::ffi::{CString, CStr};
use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::{jstring, jboolean};

extern crate android_glue;
extern crate rand;
extern crate jni;
mod font;
mod keyboard;
mod display;
mod cpu;
mod jni_glue;

use jni_glue::write_log;

#[no_mangle]
pub unsafe extern fn Java_com_rust_1react_1chip8_MobileAppBridge_hello(env: JNIEnv, _ : JObject, j_recipient: JString) -> jstring {
    let recipient = CString::from(
        CStr::from_ptr(env.get_string(j_recipient).unwrap().as_ptr())
    );
    let output = env.new_string(recipient.to_str().unwrap()).unwrap();
    write_log("I AM HERE");
    output.into_inner()
}

use cpu::CPU;
use display::Display;
use keyboard::Keyboard;

//cannot call function here
static mut Cpu: CPU = CPU{
            memory: [0; 4096],
            registers: [0; 16],
            keyboard: Keyboard{
                keys: [false; 16],
            },
            display: Display{
                memory: [false; 2048],
            },
            pc: 0,
            ir: 0, //index register,
            stacks: [0; 16],
            sp: 0, //stack pointer
            delay_timer: 0,
        };

//com.react_rust.MobileAppBridge
//com.rust_react_chip8.MobileAppBridge
#[no_mangle]
pub unsafe extern fn Java_com_rust_1react_1chip8_MobileAppBridge_loadROM(env: JNIEnv, _ : JObject, j_recipient: JString) -> jboolean {
    let recipient = CString::from(
        CStr::from_ptr(env.get_string(j_recipient).unwrap().as_ptr())
    );
    match Cpu.load_rom(recipient.to_str().unwrap_or("none")){
        Ok(_) => 1,
        _ => 0
    }
}

/*
#[no_mangle]
pub fn Java_com_rust_1react_1chip8_MobileAppBridge_pressDown(env: JNIEnv, _ : JObject, j_recipient: JString) {
    unsafe {
        Cpu.keyboard.press_down(i as usize);
    }
}

#[no_mangle]
pub fn Java_com_rust_1react_1chip8_MobileAppBridge_pressUp(env: JNIEnv, _ : JObject, j_recipient: JString) {
    unsafe {
        Cpu.keyboard.press_up(i as usize);
    }
}
*/
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
pub fn execute_cycle() {
    unsafe {
        Cpu.execute_cycle();
    }
}

#[no_mangle]
pub fn decrement_timers() {
    unsafe {
        Cpu.delay_desc();
    }
}