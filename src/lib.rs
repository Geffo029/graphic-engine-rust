use crossterm::{cursor, event, terminal, ExecutableCommand, QueueableCommand};
use std::io::{Write, stdout};


pub fn add(left: usize, right: usize) -> usize {
	left + right
}

pub fn init() -> std::io::Result<()> {
	let mut stdout = stdout();
	
	terminal::enable_raw_mode()?;
	stdout.execute(event::EnableMouseCapture)?;
	stdout.execute(cursor::Hide)?;
	stdout.queue(terminal::Clear(terminal::ClearType::All))?;
	Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
