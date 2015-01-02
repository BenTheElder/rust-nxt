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
extern crate nxt;

static C : u16 = 523;
static D : u16 = 587;
static E : u16 = 659;
static G : u16 = 784;
static P : u16 = 0; 

fn play_song(n: &mut nxt::Nxt, s: &[u16]) -> Result<(),String> {
	let d = std::time::duration::Duration::milliseconds(500);
	for f in s.iter() {
		if *f != 0 {
			match n.play_tone(*f, 500) {
				Ok(_) => {},
				Err(err) => {
					return Err(format!("Error: {}", err));
				}
			}
		}
		std::io::timer::sleep(d);
	}
	Ok(())
}

fn main() {
	let mut brick = match nxt::Nxt::get_nxt() {
		Ok(brick) => brick,
		Err(err) => panic!(format!("Error getting nxt: {}",err))
	};
	brick.set_timeout(0);
	
	let song = vec!(E, D, C, D, E, E, E, P, D, D, D, P, E, G, G, P, E, D, C, D, E, E, E, E, D, D, E, D, C);
	match play_song(& mut brick, song.as_slice()) {
		Ok(_) => {},
		Err(err) => {
			println!("{}", err);
		}
	};
}