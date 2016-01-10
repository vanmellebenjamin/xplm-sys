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
pub const xplm_Menu_NoCheck: ::libc::c_uint = 0;
pub const xplm_Menu_Unchecked: ::libc::c_uint = 1;
pub const xplm_Menu_Checked: ::libc::c_uint = 2;
pub type XPLMMenuCheck = ::libc::c_int;
pub type XPLMMenuID = *mut ::libc::c_void;
pub type XPLMMenuHandler_f =
    ::std::option::Option<unsafe extern "C" fn(inMenuRef: *mut ::libc::c_void,
                                               inItemRef: *mut ::libc::c_void)
                              -> ()>;
extern "C" {
    pub fn XPLMFindPluginsMenu() -> XPLMMenuID;
    pub fn XPLMCreateMenu(inName: *const ::libc::c_char,
                          inParentMenu: XPLMMenuID,
                          inParentItem: ::libc::c_int,
                          inHandler: XPLMMenuHandler_f,
                          inMenuRef: *mut ::libc::c_void) -> XPLMMenuID;
    pub fn XPLMDestroyMenu(inMenuID: XPLMMenuID) -> ();
    pub fn XPLMClearAllMenuItems(inMenuID: XPLMMenuID) -> ();
    pub fn XPLMAppendMenuItem(inMenu: XPLMMenuID,
                              inItemName: *const ::libc::c_char,
                              inItemRef: *mut ::libc::c_void,
                              inForceEnglish: ::libc::c_int) -> ::libc::c_int;
    pub fn XPLMAppendMenuSeparator(inMenu: XPLMMenuID) -> ();
    pub fn XPLMSetMenuItemName(inMenu: XPLMMenuID, inIndex: ::libc::c_int,
                               inItemName: *const ::libc::c_char,
                               inForceEnglish: ::libc::c_int) -> ();
    pub fn XPLMCheckMenuItem(inMenu: XPLMMenuID, index: ::libc::c_int,
                             inCheck: XPLMMenuCheck) -> ();
    pub fn XPLMCheckMenuItemState(inMenu: XPLMMenuID, index: ::libc::c_int,
                                  outCheck: *mut XPLMMenuCheck) -> ();
    pub fn XPLMEnableMenuItem(inMenu: XPLMMenuID, index: ::libc::c_int,
                              enabled: ::libc::c_int) -> ();
    pub fn XPLMRemoveMenuItem(inMenu: XPLMMenuID, inIndex: ::libc::c_int)
     -> ();
}
