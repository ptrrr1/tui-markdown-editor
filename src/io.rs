// IO
// Opening Saving and Creating Files

// Struct buffer
// Filepath
// Filename

#[derive(Debug)]
pub struct File {
	path: String,
	name: String,
	is_modified: bool
}

impl File {
	pub fn new(path: String) -> File {
		// check if there is a path
		// if yes, return title and path
		// else, open as normal but when saving, open file explorer
		unimplemented!()

		// Title -> without .md
		// Path
		// Has been modified
	}

	pub fn get_content() -> String {
		unimplemented!()
	}
}