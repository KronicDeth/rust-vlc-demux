#![feature(lang_items)]
#![feature(start)]
#![no_std]
#![allow(non_camel_case_types, dead_code)]

mod vlc;
mod traits;
mod types;

extern crate libc;
use libc::{size_t, c_int, c_char, c_void, uint8_t};

use vlc::{VLCModuleProperties, stream_Peek, vlc_Log};
use core::mem::transmute;

pub use traits::*;
pub use types::*;

//static int Open ( vlc_object_t * );
//static void Close( vlc_object_t * );

// int vlc_entry__MODULE_NAME (vlc_set_cb, void *); int vlc_entry__MODULE_NAME (vlc_set_cb vlc_set, void *opaque) { module_t *module; module_config_t *config = ((void*)0); if (vlc_set (opaque, ((void*)0), VLC_MODULE_CREATE, &module)) goto error; if (vlc_set (opaque, module, VLC_MODULE_NAME, ("modules/demux/wav.c"))) goto error; 
//    if (vlc_set (opaque, module, VLC_MODULE_DESCRIPTION, (const char *)(N_("WAV demuxer")))) goto error;
//    vlc_set (opaque, ((void*)0), VLC_CONFIG_CREATE, (0x06), &config); vlc_set (opaque, config, VLC_CONFIG_VALUE, (int64_t)(4));
//    vlc_set (opaque, ((void*)0), VLC_CONFIG_CREATE, (0x07), &config); vlc_set (opaque, config, VLC_CONFIG_VALUE, (int64_t)(403));
//    if (vlc_set (opaque, module, VLC_MODULE_CAPABILITY, (const char *)("demux")) || vlc_set (opaque, module, VLC_MODULE_SCORE, (int)(142))) goto error;
//    if (vlc_set (opaque, module, VLC_MODULE_CB_OPEN, Open) || vlc_set (opaque, module, VLC_MODULE_CB_CLOSE, Close)) goto error;
//(void) config; return 0; error: return -1; } 

//EXTERN_SYMBOL typedef int (*vlc_set_cb) (void *, void *, int, ...);

//type vlc_set_cb = FnOnce(*mut c_void, *mut c_void, c_int, ...) -> c_int;

#[no_mangle]
pub extern fn vlc_entry__3_0_0a(vlc_set: unsafe extern fn(*mut c_void, *mut c_void, c_int, ...) -> c_int,
  opaque: *mut c_void) -> c_int {
  let module: *mut c_void = 0 as *mut c_void;
  let config: *mut c_void = 0 as *mut c_void;

  unsafe {
    if vlc_set(opaque, 0 as *mut c_void, VLCModuleProperties::VLC_MODULE_CREATE as i32, &module) != 0 {
      panic!("cannot create module");
      return -1;
    }
  }

  let name = b"inrustwetrust\0";
  unsafe {
    if vlc_set(opaque, module, VLCModuleProperties::VLC_MODULE_NAME as i32, name.as_ptr()) != 0 {
      panic!("cannot set module name");
      return -1;
    }
  }

  let desc = b"FLV demuxer written in Rust\0";
  unsafe {
    if vlc_set(opaque, module, VLCModuleProperties::VLC_MODULE_DESCRIPTION as i32, desc.as_ptr()) != 0 {
      panic!("cannot set module description");
      return -1;
    }
  }

  let capability = b"demux\0";
  unsafe {
    if vlc_set(opaque, module, VLCModuleProperties::VLC_MODULE_CAPABILITY as i32, capability.as_ptr()) != 0 {
      panic!("cannot set module capability");
      return -1;
    }
  }

  unsafe {
    if vlc_set(opaque, module, VLCModuleProperties::VLC_MODULE_SCORE as i32, 999) != 0 {
      panic!("cannot set module score");
      return -1;
    }
  }

  unsafe {
    let p_open: extern "C" fn(*mut c_void) -> c_int = transmute(open as extern "C" fn(_) -> c_int);
    if vlc_set(opaque, module, VLCModuleProperties::VLC_MODULE_CB_OPEN as i32, p_open) != 0 {
      panic!("cannot set module open callback");
      return -1;
    }
  }

  unsafe {
    let p_close: extern "C" fn(*mut c_void) = transmute(close as extern "C" fn(_));
    if vlc_set(opaque, module, VLCModuleProperties::VLC_MODULE_CB_CLOSE as i32, p_close) != 0 {
      panic!("cannot set module close callback");
      return -1;
    }
  }
  //panic!("POUETPOUETPOUET");
 0
}

extern "C" fn open(obj: *mut c_void) -> c_int {
  let mut buf: *const uint8_t = 0 as *const uint8_t;
  //unsafe { let sz = stream_Peek(obj, buf as *mut *const u8, 2); }
  unsafe { vlc_Log(obj, 0, b"inrustwetrust\0".as_ptr(), b"inrustwetrust.rs".as_ptr(),
    0, b"<>\0".as_ptr(), "in rust function OPEN %d\n".as_ptr(), 42); }
  //unsafe { let sz = stream_Peek(obj, &mut buf, 2); }
  //panic!("IN OPEN");
  -1
}

extern "C" fn close(obj: *mut c_void) {
  panic!("IN CLOSE");
}
