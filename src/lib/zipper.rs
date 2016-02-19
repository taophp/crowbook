use error::{Error,Result};

use std::env;
use std::path::{Path,PathBuf};
use std::io::Write;
use std::process::Command;
use std::fs::{self, File,DirBuilder};
use uuid;
use std::ops::Drop;

/// Struct used to create zip (using filesystem and zip command)
pub struct Zipper {
    args: Vec<String>,
    path: PathBuf,
}

impl Zipper {
    /// creates new zipper
    ///
    /// path: the path to a temporary directory (zipper will create a random dir in it and clean it later)
    /// inner_dirs: a vec of inner directory to create in this directory
    pub fn new(path: &str) -> Result<Zipper> {
        let uuid = uuid::Uuid::new_v4();
        let zipper_path = Path::new(path).join(uuid.to_simple_string());

        try!(DirBuilder::new()
             .recursive(true)
             .create(&zipper_path)
             .map_err(|_| Error::Zipper(format!("could not create temporary directory in {}", path))));

        Ok(Zipper {
            args: vec!(),
            path: zipper_path,
        })
    }

    /// writes a content to a temporary file
    pub fn write(&mut self, file: &str, content: &[u8]) -> Result<()> {
        let dest_file = self.path.join(file);
        let dest_dir = dest_file.parent().expect("This file should have a parent, it has just been joined to a directory!");
        if !fs::metadata(dest_dir).is_ok() { // dir does not exist, create it
            try!(DirBuilder::new()
                 .recursive(true)
                 .create(&dest_dir)
                 .map_err(|_| Error::Zipper(format!("could not create temporary directory in {}", dest_dir.display()))));
        }
        
        
        if let Ok(mut f) = File::create(&dest_file) {
            if f.write_all(content).is_ok() {
                self.args.push(String::from(file));
                Ok(())
            } else {
                Err(Error::Zipper(format!("could not write to temporary file {}", file)))
            }
        } else {
            Err(Error::Zipper(format!("could not create temporary file {}", file)))
        }
    }

    /// run command and copy file name to current dir
    pub fn run_command(&mut self, mut command: Command, file: &str) -> Result<String> {
        let dir = try!(env::current_dir().map_err(|_| Error::Zipper("could not get current directory".to_owned())));
        try!(env::set_current_dir(&self.path).map_err(|_| Error::Zipper("could not change current directory".to_owned())));

        let res_output = command.args(&self.args)
            .output()
            .map_err(|e| Error::Zipper(format!("failed to execute process: {}", e)));
        try!(env::set_current_dir(dir).map_err(|_| Error::Zipper("could not change back to old directory".to_owned())));
        let output = try!(res_output);
        try!(fs::copy(self.path.join(file), file).map_err(|_| {
            println!("{}", &String::from_utf8_lossy(&output.stdout));
            Error::Zipper(format!("could not copy file {}", file))
        }));
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    /// generate a pdf file into given file name
    pub fn generate_pdf(&mut self, command: &str, tex_file: &str, pdf_file: &str) -> Result<String> {
        let mut command = Command::new(command);
        command.arg(tex_file);
        self.run_command(command, pdf_file)
    }
    
    /// generate an epub into given file name
    pub fn generate_epub(&mut self, file: &str) -> Result<String> {
        let mut command = Command::new("zip");
        command.arg("-X");
        command.arg(file);
        self.run_command(command, file)
    }
}

impl Drop for Zipper {
    fn drop(&mut self) {
        if let Err(err) = fs::remove_dir_all(&self.path) {
            println!("Error in zipper: could not delete temporary directory {}, error: {}", self.path.to_string_lossy(), err);
        }
    }
}
