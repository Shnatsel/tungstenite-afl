extern crate afl;
extern crate tungstenite;

use std::io::Cursor;
//use std::result::Result;

fn main() {
    afl::fuzz(|data| {
    	let vector: Vec<u8> = data.into();
        let mut cursor = Cursor::new(vector);

        tungstenite::protocol::frame::Frame::parse(&mut cursor);
    });
}