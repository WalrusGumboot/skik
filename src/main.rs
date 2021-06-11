extern crate termion;

use termion::{color, cursor, clear};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;

use std::{fmt, thread, time};
use std::io::{self, Read};

// ------------------------------
// |   STRUCTS AND FUNCTIONS    |
// ------------------------------

#[derive(PartialEq)]
enum Mode {
	Edit,
	Normal
}

#[derive(Debug, Copy, Clone)]
struct EncChar {
	orig: char, //what the character was in ciphertext
	new:  char  //what it turns into in plaintext
}

struct Mapping {
	cipher: char, //what character is being mapped
	plain:  char  //what it turns into in plaintext
}

impl EncChar {
	fn apply_mpt_to_char(&mut self, mpt: &Vec<Mapping>) {
		for m in mpt {
			if m.cipher == self.orig && m.plain != '_' {
				self.new = m.plain;
				return;
			}
		}
	}
}

impl fmt::Display for EncChar {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.orig == self.new {
        	write!(f, "{}", self.new)
		} else {
			write!(f, "{}{}{}", color::Fg(color::LightCyan), self.new, color::Fg(color::Reset))
		}
    }
}

impl fmt::Display for Mapping {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.plain == '_' {
        	write!(f, "[ {} > {} ]", self.cipher, self.plain)
		} else {
			write!(f, "[ {} > {}{}{} ]", self.cipher, color::Fg(color::LightCyan), self.plain, color::Fg(color::Reset))
		}		
    }
}

fn apply_mpt_to_enc_chars(encs: &mut Vec<EncChar>, mpt: &Vec<Mapping>) {
	for e in encs.iter_mut() {
		e.apply_mpt_to_char(mpt);
	}
}

fn move_cursor(c: &mut char, inc: i8, encs: &Vec<EncChar>, mpt: &Vec<Mapping>, status: &String, mode: &Mode) {
	if (*c == 'a' && inc == -1) || (*c == 'z' && inc == 1) { //the literal edge cases
		return;
	}
	*c = ((*c as i8) + inc) as u8 as char; //jesus h
	draw(encs, mpt, status, c, mode);	//TODO: Make this more efficient by only redrawing the mpt
}

fn draw(encs: &Vec<EncChar>, mpt: &Vec<Mapping>, status: &String, cursor: &char, mode: &Mode) {
	let mut last_line = 6; //TODO: dynamically change this
	let (term_x, term_y) = termion::terminal_size().unwrap();

	println!("{}{}{}skik v. 0.1.0a{}, running in a {}{}{} by {}{}{} terminal.", clear::All, cursor::Goto(2, 2), color::Fg(color::LightGreen), color::Fg(color::Reset), color::Fg(color::LightMagenta), term_x, color::Fg(color::Reset), color::Fg(color::LightMagenta), term_y, color::Fg(color::Reset)); //god jesus this looks awful
	println!("{}{col}Ctrl+C{res} to quit, {col}Insert{res} to modify character, {col}Escape{res} to exit modification", cursor::Goto(2, 3), col = color::Fg(color::LightCyan), res = color::Fg(color::Reset));

	{
		//displaying the enc_chars
		println!("{}{}-- ciphertext --{}", cursor::Goto(2, 5), color::Fg(color::LightMagenta), color::Fg(color::Reset));

		let mut x = 2;
		let mut y = 6;

		for &c in encs.iter() {
			if x > term_x - 4 {
				x = 2;
				y = y + 1;
			}

			println!("{}{}", cursor::Goto(x, y), c);
			
			x = x + 1;			
		}

		last_line = y + 2;
	}

	{
		//displaying the mpt
		println!("{}{}-- mapping table --{}", cursor::Goto(2, last_line), color::Fg(color::LightMagenta), color::Fg(color::Reset));

		let mut x = 2;
		let mut y = last_line + 1;
		for m in mpt {
			if x > term_x - 14 {
				x = 2;
				y = y + 1;
			}
			if m.cipher == *cursor {
				if *mode == Mode::Edit {
					let c = color::Bg(color::Cyan);
					println!("{}{}{}{}", cursor::Goto(x, y), c, m, color::Bg(color::Reset));
				} else {
					let c = color::Bg(color::Red);
					println!("{}{}{}{}", cursor::Goto(x, y), c, m, color::Bg(color::Reset));

				};
				
			} else {
				println!("{}{}", cursor::Goto(x, y), m);
			}

			x = x + 10;
		}

		last_line = y + 2;
	}

	//status line
	println!("{}{}{}{}", cursor::Goto(2, last_line), color::Fg(color::LightGreen), &status, color::Fg(color::Reset));
}

fn main() -> io::Result<()> {
	// Basic setup
    //let input = String::from("This is some sample ciphertext. The quick brown fox jumps over the lazy dog, and he does so in such a manner that it doesn't wake up said lazy dog. Quite impressive!");

	//Reading in text from stdin buffer
	let mut input = String::new();
	let cmd_stdin = io::stdin();
	let mut handle = cmd_stdin.lock();
	handle.read_to_string(&mut input)?;
    //Ok(());
    



	//there is most definitely a better way to do this
	//TODO: don't suck
	let mut enc_chars = Vec::new();
	for c in input.chars() {
		let mut l = c;
		l.make_ascii_lowercase();
		let enc_char = EncChar{
			orig: l,
			new:  l
		};
		enc_chars.push(enc_char);
	}

	let mut mpt = Vec::new(); //mapping table
	for i in 97..123 { //97 ascii is lowercase a
		mpt.push(Mapping {cipher: (i as u8) as char, plain: '_'});
	}

	//testing
	mpt[0].plain = 'b';
	mpt[1].plain = 'a';
	apply_mpt_to_enc_chars(&mut enc_chars, &mpt);

	let mut status = String::from("ready.");

	let stdout = io::stdout().into_raw_mode();
	let stdin = termion::async_stdin();
	let mut it = stdin.keys(); //iterator object

	let mut cursor_char: char = 'a';
	let mut mode = Mode::Normal;
	draw(&enc_chars, &mpt, &status, &cursor_char, &mode);

	loop {
		//copied straight from GitLab: https://gitlab.redox-os.org/redox-os/termion/-/issues/168
		let b = it.next(); 
		match b {
			Some(x) => match x {
				Ok(k) => {
					if mode == Mode::Edit {
						match k {
							Key::Esc => {mode = Mode::Normal},
							Key::Char(c) => {
								for m in &mut mpt {
									if m.cipher == cursor_char {
										m.plain = c;
									}
								}
								mode = Mode::Normal;
								apply_mpt_to_enc_chars(&mut enc_chars, &mpt);
								draw(&enc_chars, &mpt, &status, &cursor_char, &mode);
							},
							_ => {}
						}
						//Handle keypresses when editing
						
					}
					match k {
						Key::Left => move_cursor(&mut cursor_char, -1, &enc_chars, &mpt, &status, &mode),
						Key::Right => move_cursor(&mut cursor_char, 1, &enc_chars, &mpt, &status, &mode),
						Key::Insert => {
							if mode == Mode::Normal {
								mode = Mode::Edit;
							}
							continue;
						}
						Key::Ctrl('c') => break Ok(()), //quit the program
						_ => {} //any other key: do fuck-all
					}
				},  
				_ => {}
				},
			None => {
				thread::sleep(time::Duration::from_millis(40)); //poor man's buffer
				//no time to handle keypresses? Just do fuck-all for 40 ms lmao
			} 
		}
		//this loop might do nothing if no recognized key was pressed.
	}
	
}