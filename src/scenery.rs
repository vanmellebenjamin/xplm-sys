// Copyright (c) 2015 xplm-sys developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

/* automatically generated by rust-bindgen */

pub type Enum_Unnamed1 = ::libc::c_uint;
pub const xplm_ProbeY: ::libc::c_uint = 0;
pub type XPLMProbeType = ::libc::c_int;
pub type Enum_Unnamed2 = ::libc::c_uint;
pub const xplm_ProbeHitTerrain: ::libc::c_uint = 0;
pub const xplm_ProbeError: ::libc::c_uint = 1;
pub const xplm_ProbeMissed: ::libc::c_uint = 2;
pub type XPLMProbeResult = ::libc::c_int;
pub type XPLMProbeRef = *mut ::libc::c_void;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed3 {
    pub structSize: ::libc::c_int,
    pub locationX: ::libc::c_float,
    pub locationY: ::libc::c_float,
    pub locationZ: ::libc::c_float,
    pub normalX: ::libc::c_float,
    pub normalY: ::libc::c_float,
    pub normalZ: ::libc::c_float,
    pub velocityX: ::libc::c_float,
    pub velocityY: ::libc::c_float,
    pub velocityZ: ::libc::c_float,
    pub is_wet: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_Unnamed3 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed3 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XPLMProbeInfo_t = Struct_Unnamed3;
pub type XPLMObjectRef = *mut ::libc::c_void;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed4 {
    pub structSize: ::libc::c_int,
    pub x: ::libc::c_float,
    pub y: ::libc::c_float,
    pub z: ::libc::c_float,
    pub pitch: ::libc::c_float,
    pub heading: ::libc::c_float,
    pub roll: ::libc::c_float,
}
impl ::std::clone::Clone for Struct_Unnamed4 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed4 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XPLMDrawInfo_t = Struct_Unnamed4;
pub type XPLMObjectLoaded_f =
    ::std::option::Option<unsafe extern "C" fn(inObject: XPLMObjectRef,
                                               inRefcon: *mut ::libc::c_void)
                              -> ()>;
pub type XPLMLibraryEnumerator_f =
    ::std::option::Option<unsafe extern "C" fn(inFilePath:
                                                   *const ::libc::c_char,
                                               inRef: *mut ::libc::c_void)
                              -> ()>;
extern "C" {
    pub fn XPLMCreateProbe(inProbeType: XPLMProbeType) -> XPLMProbeRef;
    pub fn XPLMDestroyProbe(inProbe: XPLMProbeRef) -> ();
    pub fn XPLMProbeTerrainXYZ(inProbe: XPLMProbeRef, inX: ::libc::c_float,
                               inY: ::libc::c_float, inZ: ::libc::c_float,
                               outInfo: *mut XPLMProbeInfo_t)
     -> XPLMProbeResult;
    pub fn XPLMLoadObject(inPath: *const ::libc::c_char) -> XPLMObjectRef;
    pub fn XPLMLoadObjectAsync(inPath: *const ::libc::c_char,
                               inCallback: XPLMObjectLoaded_f,
                               inRefcon: *mut ::libc::c_void) -> ();
    pub fn XPLMDrawObjects(inObject: XPLMObjectRef, inCount: ::libc::c_int,
                           inLocations: *mut XPLMDrawInfo_t,
                           lighting: ::libc::c_int,
                           earth_relative: ::libc::c_int) -> ();
    pub fn XPLMUnloadObject(inObject: XPLMObjectRef) -> ();
    pub fn XPLMLookupObjects(inPath: *const ::libc::c_char,
                             inLatitude: ::libc::c_float,
                             inLongitude: ::libc::c_float,
                             enumerator: XPLMLibraryEnumerator_f,
                             _ref: *mut ::libc::c_void) -> ::libc::c_int;
}
