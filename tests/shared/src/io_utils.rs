/*
 * Copyright (C) 2022-2023 by Christian Fischer
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

use std::borrow::Cow;
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
#[derive(Clone)]
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


impl IteratorState {
    /// Creates an IteratorState for a submodule of the current one.
    pub fn create_sub_state(&self, sub_state_name: &str) -> IteratorState {
        let mut sub_stack = self.stack.clone();
        sub_stack.push(sub_state_name.to_string());

        IteratorState {
            indent: self.indent.clone().add("    "),
            path:   sub_stack.join("/"),
            stack:  sub_stack,
        }
    }

    /// Get the current module name.
    pub fn get_top_module(&self) -> Option<&str> {
        self.stack.last().map(|s| s as &str)
    }
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
    let mut subdir_list : Vec<PathBuf> = Vec::new();
    let mut files_list  : Vec<PathBuf> = Vec::new();
    let mut content                    = String::new();
    let mut had_submodules             = false;

    // iterate through all elements in the current path
    for paths in root_path.read_dir().unwrap() {
        if let Ok(entry) = paths {
            let path      = entry.path();
            let file_type = entry.file_type().unwrap();

            if file_type.is_file() {
                files_list.push(path);
            }
            else if file_type.is_dir() {
                subdir_list.push(path);
            }
        }
    }

    // sort by name
    files_list.sort();
    subdir_list.sort();

    // for each directory..
    for path in subdir_list {
        // get a suitable module name
        let module = filename_to_symbol(path.to_str().unwrap());

        // check how to handle the directory
        let handle_dir : HandleDirectory = (callbacks.on_handle_dir)(&path, state);

        // recurse into subdirectory, if not skipped
        match handle_dir {
            HandleDirectory::Ignore => { }

            HandleDirectory::Enter => {
                // recurse into the subdirectory
                let subdir_content = recursive_visit_directory_step(
                    &path,
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
                    &path,
                    &state.create_sub_state(&module),
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

                    had_submodules = true;
                }
            }
        };
    }

    // for each file..
    if files_list.len() > 0 {
        let mut module_open = false;

        // determine a module name for files, if there were already submodules present
        let files_module = if had_submodules {
            Some(
                state.get_top_module()
                .map(|n| format!("{n}_other"))
                .unwrap_or("other".to_string())
            )
        }
        else {
            None
        };

        // create sub state if required
        let files_state = if let Some(module_name) = &files_module {
            Cow::Owned(state.create_sub_state(module_name))
        }
        else {
            Cow::Borrowed(state)
        };

        // iterate through all files
        for path in files_list {
            // call the on_file_found callback
            let str = (callbacks.on_file_found)(&path, &files_state);

            // add the result to the content, if any code was generated
            if !str.is_empty() {
                // module begin, if mandatory and not yet done
                if !module_open {
                    if let Some(module_name) = &files_module {
                        content.push_str("\n");
                        content.push_str(&format!("{}mod {} {{\n", state.indent, module_name));
                        content.push_str(&format!("{}    use super::*;\n", state.indent));

                        module_open = true;
                    }
                }

                content.push('\n');
                content.push('\n');
                content.push_str(&str);
            }
        }

        // module end
        if module_open {
            content.push_str(&format!("{}}}\n", state.indent));
        }
    }

    content
}


/// Get the name of a file from a path object.
/// Path and file extensions will be stripped from the filename.
pub fn get_plain_filename(f: &PathBuf) -> String {
    let mut filename = f.to_str().unwrap().to_string();

    // replace \\ by /
    filename = filename.replace('\\', "/");

    // cut file extension
    if let Some(pos) = filename.rfind('.') {
        filename = filename[0 .. pos].to_string();
    }

    // cut path
    if let Some(pos) = filename.rfind('/') {
        filename = filename[pos+1 ..].to_string();
    }

    filename
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

    // replace multiple occurrences of _ characters
    loop {
        let sym2 = sym.replace("__", "_");
        if sym2 != sym {
            sym = sym2;
        }
        else {
            break;
        }
    }

    // remove leading _ characters
    while sym.starts_with('_') {
        sym = sym[1 ..].to_string();
    }

    // remove trailing _ characters
    while sym.ends_with('_') {
        sym = sym[.. sym.len() - 1].to_string();
    }

    // convert into lowercase
    sym = sym.to_lowercase();

    sym
}


/// Checks if a string starts with a numeric character.
pub fn starts_with_number(s: &str) -> bool {
    if s.len() > 0 {
        match s.chars().next().unwrap() {
            '0' ..= '9' => true,
            _ => false
        }
    }
    else {
        false
    }
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
