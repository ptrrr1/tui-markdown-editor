use tui_textarea::TextArea;
use std::{
    path::{
        Path, 
        PathBuf
    }, 
    io::Write
};

#[derive(Debug, Clone)]
pub struct File {
    pub path: PathBuf,
    pub name: String,
    pub textarea: TextArea<'static>,
}

impl File {
	pub fn open(path: PathBuf) -> File {
		// TODO: Check if file exists, create if not
		// Create textarea
		let mut textarea = TextArea::default();

		// Add content if any
        if let Ok(content) = std::fs::read_to_string(&path) {
            textarea = TextArea::new(content.lines().map(String::from).collect());
        }

        // Extract name from path
        let name = Path::new(&path)
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_string();

        // Return Struct
        File {
            path: path,
            name: name,
            textarea: textarea,
        }
	}

	// TODO: Rework [Was copy pasted from old code]
	#[allow(dead_code)]
	pub fn save(&self) -> std::io::Result<()> {
        let path = std::path::Path::new(&self.path);
        let file = std::fs::File::create(&path)?;
        let mut buf_writer = std::io::BufWriter::new(file);

        for line in self.textarea.lines() {
            buf_writer.write_all(line.as_bytes())?;
            buf_writer.write_all(b"\n")?; // Adding a newline after each line
        }

        buf_writer.flush()?;
        Ok(())
	}
}