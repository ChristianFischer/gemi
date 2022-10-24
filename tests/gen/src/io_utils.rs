/*
 * Copyright (C) 2022 by Christian Fischer
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use std::cmp::min;
use std::fs;
use std::ops::Add;
use std::path::PathBuf;

pub type OnHandleDir<'a> = Box<dyn Fn(&PathBuf, &IteratorState) -> HandleDirectory + 'a>;
pub type OnFileFound<'a> = Box<dyn Fn(&PathBuf, &IteratorState) -> String + 'a>;


/// A set of callbacks to let the caller generate code for each
/// file or directory found.
pub struct FindRomCallbacks<'a> {
    /// Called on entering a directory.
    pub on_handle_dir: OnHandleDir<'a>,

    /// Called for each file.
    pub on_file_found: OnFileFound<'a>,
}


/// The current state while iterating through directories.
pub struct IteratorState {
    /// The current indent expected when code is being generated.
    pub indent: String,

    /// The path to the current element with every element
    /// converted into symbols instead of the original file name.
    pub path: String,

    /// The stack of path elements leading to the current file to be tested.
    pub stack: Vec<String>,
}


/// A callback return value to define how to handle a single directory on iterating.
pub enum HandleDirectory {
    /// Just enters the directory without creating a submodule.
    Enter,

    /// Enters the directory and creates a module for any containing tests.
    CreateModule,

    /// Ignores this directory and all of it's subdirectories.
    Ignore,
}


/// Recursively iterate through subdirectories to find ROM files.
/// Invokes a callback for each directory and file found. Collects the generated code
/// from each callback and creates file content with all generated tests.
pub fn recursive_visit_directory(root_path: PathBuf, callbacks: &FindRomCallbacks) -> String {
    let root_name = filename_to_symbol(root_path.file_name().unwrap().to_str().unwrap());

    recursive_visit_directory_step(
        &root_path,
        &IteratorState {
            indent: "".to_string(),
            path:   root_name.clone(),
            stack:  vec![root_name],
        },
        callbacks
    )
}


/// Internal part of recursive_find_roms which is called recursively.
fn recursive_visit_directory_step(root_path: &PathBuf, state: &IteratorState, callbacks: &FindRomCallbacks) -> String {
    let mut content = String::new();

    // iterate through all elements in the current path
    for paths in root_path.read_dir().unwrap() {
        if let Ok(entry) = paths {
            let path      = entry.path();
            let file_type = entry.file_type().unwrap();

            // for each file..
            if file_type.is_file() {
                // call the on_file_found callback
                let str = (callbacks.on_file_found)(&path, state);

                // add the result to the content, if any code was generated
                if !str.is_empty() {
                    content.push('\n');
                    content.push('\n');
                    content.push_str(&str);
                }
            }

            // for each directory..
            if file_type.is_dir() {
                // get a suitable module name
                let module = filename_to_symbol(path.to_str().unwrap());

                // update the path stack to be used in the subdirectory
                let mut next_stack = state.stack.clone();
                next_stack.push(module);

                // check how to handle the directory
                let handle_dir : HandleDirectory = (callbacks.on_handle_dir)(&path, state);

                // recurse into subdirectory, if not skipped
                match handle_dir {
                    HandleDirectory::Ignore => { }

                    HandleDirectory::Enter => {
                        // recurse into the subdirectory
                        let subdir_content = recursive_visit_directory_step(
                            &entry.path(),
                            &state,
                            callbacks
                        );

                        // add the content, if the subdirectory contained any tests
                        if !subdir_content.is_empty() {
                            content.push_str("\n");
                            content.push_str(&subdir_content);
                        }
                    }

                    HandleDirectory::CreateModule => {
                        // recurse into the subdirectory
                        let subdir_content = recursive_visit_directory_step(
                            &entry.path(),
                            &IteratorState {
                                indent: state.indent.clone().add("    "),
                                path:   next_stack.join("/"),
                                stack:  next_stack,
                            },
                            callbacks
                        );

                        // add the content, if the subdirectory contained any tests
                        if !subdir_content.is_empty() {
                            content.push_str("\n");

                            content.push_str(&format!(
                                "{}mod {} {{\n{}    use super::*;\n",
                                state.indent,
                                filename_to_symbol(path.to_str().unwrap()),
                                state.indent,
                            ));

                            content.push_str(&subdir_content);

                            content.push_str(&format!("{}}}\n\n", state.indent));
                        }
                    }
                };
            }
        }
    }

    content
}


/// Takes a filename and creates a symbol identifier,
/// which may be used for function or module names.
pub fn filename_to_symbol(filename: &str) -> String {
    let mut sym = filename.to_string();

    // replace \\ by /
    sym = sym.replace('\\', "/");

    // cut file extension
    if let Some(pos) = sym.rfind('.') {
        sym = sym[0 .. pos].to_string();
    }

    // cut path
    if let Some(pos) = sym.rfind('/') {
        sym = sym[pos+1 ..].to_string();
    }

    // replace all characters invalid for symbols
    sym = sym.chars().map(|c| match c {
        'a' ..= 'z' => c,
        'A' ..= 'Z' => c,
        '0' ..= '9' => c,
        _ => '_',
    }).collect();

    // convert into lowercase
    sym = sym.to_lowercase();

    sym
}


/// Updates a file with a given content.
/// Compares the new content with the current one and only
/// updates the file when the content has been changed.
pub fn update_file(file_path: &PathBuf, content: &str) {
    // stop here if the content did not change
    if let Ok(old_content) = fs::read_to_string(file_path) {
        // normalize to unix line endings
        let old_content_normalized = old_content.replace("\r\n", "\n");

        if content == old_content_normalized {
            return;
        }
    }

    fs::write(file_path, content).unwrap();
}
