// Copyright 2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

mod auth;
mod error;
mod u2f;
mod u2f_crate;

pub use error::Error;
use tauri::{
	Runtime,
	plugin::{Builder as PluginBuilder, TauriPlugin},
};
type Result<T> = std::result::Result<T, Error>;

#[tauri::command]
fn init_auth() { auth::init_usb(); }

#[tauri::command]
fn register(timeout:u64, challenge:String, application:String) -> crate::Result<String> {
	auth::register(application, timeout, challenge)
}

#[tauri::command]
fn verify_registration(
	challenge:String,
	application:String,
	register_data:String,
	client_data:String,
) -> crate::Result<String> {
	u2f::verify_registration(application, challenge, register_data, client_data)
}

#[tauri::command]
fn sign(
	timeout:u64,
	challenge:String,
	application:String,
	key_handle:String,
) -> crate::Result<String> {
	auth::sign(application, timeout, challenge, key_handle)
}

#[tauri::command]
fn verify_signature(
	challenge:String,
	application:String,
	sign_data:String,
	client_data:String,
	key_handle:String,
	pubkey:String,
) -> crate::Result<u32> {
	u2f::verify_signature(application, challenge, sign_data, client_data, key_handle, pubkey)
}

pub fn init<R:Runtime>() -> TauriPlugin<R> {
	PluginBuilder::new("authenticator")
		.invoke_handler(tauri::generate_handler![
			init_auth,
			register,
			verify_registration,
			sign,
			verify_signature
		])
		.build()
}
