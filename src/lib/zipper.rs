// Copyright (C) 2016 Élisabeth HENRY.
//
// This file is part of Crowbook.
//
// Crowbook is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published
// by the Free Software Foundation, either version 2.1 of the License, or
// (at your option) any later version.
//
// Caribon is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received ba copy of the GNU Lesser General Public License
// along with Crowbook.  If not, see <http://www.gnu.org/licenses/>.

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
    pub fn write(&mut self, file: &str, content: &[u8], add_args: bool) -> Result<()> {
        let path = Path::new(file);
        if path.starts_with("..") || path.is_absolute() {
            return Err(Error::Zipper(format!("file {} refers to an absolute or a parent path.
This is forbidden because we are supposed to create a temporary file in a temporary dir.", file)));
        }
        let dest_file = self.path.join(path);
        let dest_dir = dest_file.parent().unwrap();
        if !fs::metadata(dest_dir).is_ok() { // dir does not exist, create it
            try!(DirBuilder::new()
                 .recursive(true)
                 .create(&dest_dir)
                 .map_err(|_| Error::Zipper(format!("could not create temporary directory in {}", dest_dir.display()))));
        }
        
        
        if let Ok(mut f) = File::create(&dest_file) {
            if f.write_all(content).is_ok() {
                if add_args {
                    self.args.push(String::from(file));
                }
                Ok(())
            } else {
                Err(Error::Zipper(format!("could not write to temporary file {}", file)))
            }
        } else {
            Err(Error::Zipper(format!("could not create temporary file {}", file)))
        }
    }

    /// Unzip a file and deletes it afterwards
    pub fn unzip(&mut self, file: &str) -> Result<()> {
        // change to dest directory to unzip file
        let dir = env::current_dir().unwrap();
        try!(env::set_current_dir(&self.path).map_err(|_| Error::Zipper("could not change current directory".to_owned())));
        let output = Command::new("unzip")
                      .arg(file)
                      .output()
                      .map_err(|e| Error::Zipper(format!("failed to execute unzip  on {}: {}", file, e)));

        // change back to original current directory before try! ing anything
        env::set_current_dir(dir).unwrap();
        try!(output);

        fs::remove_file(self.path.join(file))
            .map_err(|_| Error::Zipper(format!("failed to remove file {}", file)))
    }

    /// run command and copy file name (supposed to result from the command) to current dir
    pub fn run_command(&mut self, mut command: Command, in_file: &str, out_file: &str) -> Result<String> {
        let res_output = command.args(&self.args)
            .current_dir(&self.path)
            .output()
            .map_err(|e| Error::Zipper(format!("failed to execute process: {}", e)));
        let output = try!(res_output);
        try!(fs::copy(self.path.join(in_file), out_file).map_err(|_| {
            println!("{}", &String::from_utf8_lossy(&output.stdout));
            Error::Zipper(format!("could not copy file {} to {}", in_file, out_file))
        }));
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    /// zip all files in zipper's tmp dir to a given file name and return odt file
    pub fn generate_odt(&mut self, command: &str, odt_file: &str) -> Result<String> {
        let mut command = Command::new(command);
        command.arg("-r");
        command.arg("result.odt");
        command.arg(".");
        self.run_command(command, "result.odt", odt_file)
    }
    

    /// generate a pdf file into given file name
    pub fn generate_pdf(&mut self, command: &str, tex_file: &str, pdf_file: &str) -> Result<String> {
        // first pass
        let _ = Command::new(command)
            .current_dir(&self.path)
            .arg(tex_file)
            .output();

        // second pass
        let mut command = Command::new(command);
        command.arg(tex_file);
        self.run_command(command, "result.pdf", pdf_file)
    }
    
    /// generate an epub into given file name
    pub fn generate_epub(&mut self, command: &str, file: &str) -> Result<String> {
        let mut command = Command::new(command);
        command.arg("-X");
        command.arg("result.epub");
        self.run_command(command, "result.epub", file)
    }
}

impl Drop for Zipper {
    fn drop(&mut self) {
        if let Err(err) = fs::remove_dir_all(&self.path) {
            println!("Error in zipper: could not delete temporary directory {}, error: {}", self.path.to_string_lossy(), err);
        }
    }
}
