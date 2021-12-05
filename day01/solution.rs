use std::io;

fn main() -> io::Result<()> {
	let mut lines = io::stdin().lock().lines();
	while let Some(line) = lines.next() {
		println!(line);
	}
}
