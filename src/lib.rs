/*
Copyright 2014 Benjamin Elder from https://github.com/BenTheElder/slack-rs

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

	http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/
extern crate usb;
extern crate libc;

use usb::libusb;
use usb::{Context,Device,DeviceHandle};
use std::c_str::CString;

#[no_mangle]
pub static PORT_A : u8 = 0;
#[no_mangle]
pub static PORT_B : u8 = 1;
#[no_mangle]
pub static PORT_C : u8 = 2;
#[no_mangle]
pub static PORT_ALL : u8 = 0xFF;


#[no_mangle]
pub static MOTOR_MODE_COAST : u8 = 0;
#[no_mangle]
pub static MOTOR_MODE_ON : u8 = 1;
#[no_mangle]
pub static MOTOR_MODE_BRAKE : u8 = 2;
#[no_mangle]
pub static MOTOR_MODE_REGULATED : u8 = 4;


///Attempts to return an Nxt object.<br />
///See: <a href="struct.Nxt.html#method.get_nxt" class="fnname">nxt::Nxt::get_nxt</a><br />
///```C: Nxt* get_brick();```
///<br /><br />
#[no_mangle]
pub extern "C" fn get_nxt() -> Option<Box<Nxt>> { 
	match Nxt::get_nxt() {
		Ok(n) => { Some(box n) },
		Err(_) => { None }
	}
}

///Calls drop() on the Nxt object.<br />
///```C: void free_nxt(Nxt* n);```
///<br /><br />
#[no_mangle]
pub extern "C" fn free_nxt(n: &mut Nxt) {
	drop(n);
}


///Plays a tone at _freq_ hz for _duration_ milliseconds;<br>
///Returns 1 on success, and -1 on error.<br />
///See: <a href="struct.Nxt.html#method.play_tone" class="fnname">nxt::Nxt.play_tone</a><br />
///```C: int play_tone(Nxt* n, unsigned int freq, unsigned int duration);```
///<br /><br />
#[no_mangle]
pub extern "C" fn play_tone(n: &mut Nxt, freq: u16, duration: u16) -> i32 {
	match n.play_tone(freq, duration) {
		Ok(_) => { 1 },
		Err(_) => { -1 }
	}
}


///Returns the USB read/write timeout associated with the Nxt object.<br />
///See: <a href="struct.Nxt.html#method.get_timeout" class="fnname">nxt::Nxt.get_timeout</a><br />
///```C: unsigned int get_timeout(Nxt* n);```
///<br /><br />
#[no_mangle]
pub extern "C" fn get_timeout(n: &mut Nxt) -> u32 {
	n.get_timeout()
}


///Sets the USB read/write timeout associated with the Nxt object.<br />
///See: <a href="struct.Nxt.html#method.set_timeout" class="fnname">nxt::Nxt.set_timeout</a><br />
///```C: void set_timeout(Nxt* n, unsigned int timeout);```
///<br /><br />
#[no_mangle]
pub extern "C" fn set_timeout(n: &mut Nxt, timeout: u32) {
	n.set_timeout(timeout);
}


///Turns on the motor attached to _n_ nxt port _port_ at power _power_.<br />
///See: <a href="struct.Nxt.html#method.run_motor" class="fnname">nxt::Nxt.run_motor</a><br />
///```C: int run_motor(Nxt* n, uint8_t port, uint8_t power);```
///<br /><br />
#[no_mangle]
pub extern "C" fn run_motor(n: &mut Nxt, port: u8, power: u8) -> i32 {
	match n.run_motor(port, power) {
		Ok(_) => { 1 }
		Err(_) => { -1 }
	}
}


///Stops the motor attached to _n_ nxt port _port_.<br />
///See: <a href="struct.Nxt.html#method.stop_motor" class="fnname">nxt::Nxt.stop_motor</a><br />
///```C: int stop_motor(Nxt* n, uint8_t port);```
///<br /><br />
#[no_mangle]
pub extern "C" fn stop_motor(n: &mut Nxt, port: u8) -> i32 {
	match n.stop_motor(port) {
		Ok(_) => { 1 }
		Err(_) => { -1 }
	}
}

///Sends a message _msg_ to inbox _inbox_.<br />
///See: <a href="struct.Nxt.html#method.send_message" class="fnname">nxt::Nxt.send_message</a><br />
///```C: int send_message(Nxt* n, uint8_t inbox, char* msg);```
///<br /><br />
#[no_mangle]
pub extern "C" fn send_message(n: &mut Nxt, inbox: u8, msg: *const libc::c_char) -> i32 {
	let mut s = String::new();
	unsafe {
		let cs = CString::new(msg, false);
		s = s + cs.as_str().unwrap();
	}
	match n.send_message(inbox, s) {
		Ok(_) => { 1 },
		Err(_) => { -1 }
	}
}

///Starts program _name_ on the Nxt.<br />
///Note: program names on the nxt internally end with .rxe
///See: <a href="struct.Nxt.html#method.start_program" class="fnname">nxt::Nxt.start_program</a><br />
///```C: int start_program(Nxt* n, char* name);```
///<br /><br />
#[no_mangle]
pub extern "C" fn start_program(n: &mut Nxt, name: *const libc::c_char) -> i32 {
	let mut s = String::new();
	unsafe {
		let cs = CString::new(name, false);
		s = s +cs.as_str().unwrap();
	}
	match n.start_program(s) {
		Ok(_) => { 1 },
		Err(_) => { -1 }
	}
}


///Starts stops the currently running program on the Nxt.<br />
///See: <a href="struct.Nxt.html#method.stop_program" class="fnname">nxt::Nxt.stop_program</a><br />
///```C: int start_program(Nxt* n, char* name);```
///<br /><br />
#[no_mangle]
pub extern "C" fn stop_program(n: &mut Nxt) -> i32 {
	match n.stop_program() {
		Ok(_) => { 1 },
		Err(_) => { -1 }
	}
}


///Used to interact with nxt over usb
#[repr(C)]
pub struct Nxt {
	context: usb::Context,
	device: usb::Device,
	handle: usb::DeviceHandle,
	timeout: u32
}


impl Nxt {
	///Returns an Nxt object from the first matching usb device or an error on failure.
	///Note that the connection is currently opened here, and that it will be closed
	///when out of scope or on explicit drop()
	pub fn get_nxt() -> Result<Nxt,String> {
		let ctx = usb::Context::new();
		//Find usb device matching lego vendor id and nxt product id
		match ctx.find_by_vid_pid(0x0694, 0x0002) {
			Some(dev) => {
				//if we find it, open the device
				match dev.open() {
					Ok(h) => {
						//if that works, claim it, and return
						h.claim_interface(0);
						let nxt = Nxt {
							context: ctx.clone(),
							device: dev,
							handle: h.clone(),
							timeout: 0
						};
						Ok(nxt)
					},
					Err(err) => {
						Err(format!("Error opening device: {}", err))
					}
				}
			},
			None => {
				Err("Failed to find device".to_string())
			}		
		}
	}

	///Gets the USB read/write timeout in milliseconds
	pub fn get_timeout(&self) -> u32 {
		return self.timeout
	}

	///Sets the USB read/write timeout in milliseconds
	pub fn set_timeout(&mut self, timeout: u32) {
		self.timeout = timeout;
	}

	fn do_command(&mut self, cmd: &[u8]) -> Result<(), libusb::libusb_transfer_status> {
		self.handle.write(0x01, libusb::LIBUSB_TRANSFER_TYPE_BULK, cmd, self.timeout)
	}

	///Play a tone for _duration_ milliseconds at _freq_ hz.
	pub fn play_tone(&mut self, freq: u16, duration: u16) -> Result<(),libusb::libusb_transfer_status> {
		let freq_lsb = (freq & 0xFF) as u8;
		let freq_msb = ((freq >> 8) & 0xFF) as u8;
		let dur_lsb = (duration & 0xFF) as u8;
		let dur_msb = ((duration >> 8) & 0xFF ) as u8;
		let cmd = [0x00, 0x03, freq_lsb, freq_msb, dur_lsb, dur_msb];
		self.do_command(cmd.as_slice())
	}

	/*
	pub fn set_output_state(&mut self, port: u8, power: u8, mode: u8, reg_mode: u8, ) {
		let cmd = [0x00, 0x04, port, power, mode, reg_mode, ]
	}
	*/

	///Starts the motor at port _port_ running at power level _power_.
	pub fn run_motor(&mut self, port: u8, power: u8) -> Result<(),libusb::libusb_transfer_status> {
		let cmd = [0, 4, port, power, MOTOR_MODE_ON, 0, 0, 0x20, 0];
		self.do_command(cmd.as_slice())
	}

	///Stops the motor at port _port_.
	pub fn stop_motor(&mut self, port: u8) -> Result<(),libusb::libusb_transfer_status> {
		let cmd = [0, 4, port, 0, MOTOR_MODE_COAST, 0, 0, 0];
		self.do_command(cmd.as_slice())
	}

	///Sends a message _msg_ to the inbox _inbox_.
	///_inbox_ should be 0-9.
	///_msg_ should be ascii and no greater than 58 characters in length.
	pub fn send_message(&mut self, inbox: u8, msg: String) -> Result<(), String> {
		if msg.len() > 58 {
			return Err("Message too long".to_string());
		}
		if inbox > 9 {
			return Err("Invalid mailbox".to_string());
		}
		let svec = msg.into_bytes();
		let msg_len = svec.len() as u8;
		let mut cmd = vec![0, 9, inbox, msg_len+1];
		cmd.push_all(svec.as_slice());
		cmd.push(0);
		match self.do_command(cmd.as_slice()) {
			Err(err) =>  Err(format!("{}", err)),
			Ok(_) => Ok(())
		}
	}

	///Starts the program with name _name_, which should be ascii encoded and end with .rxe
	pub fn start_program(&mut self, name: String) -> Result<(), String> {
		if name.len() > 18 {
			return Err("Name too long".to_string());
		}
		let mut cmd = vec![0, 0];
		let svec = name.into_bytes();
		cmd.push_all(svec.as_slice());
		cmd.push_all(Vec::from_elem(19-svec.len(), 0).as_slice());
		//cmd.push(0);
		match self.do_command(cmd.as_slice()) {
			Err(err) =>  Err(format!("{}", err)),
			Ok(_) => Ok(())
		}
	}

	///Stops the currently running program on the nxt.
	pub fn stop_program(&mut self) -> Result<(), libusb::libusb_transfer_status> {
		let cmd = [0, 1];
		self.do_command(cmd.as_slice())
	}
}