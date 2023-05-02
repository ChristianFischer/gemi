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

use std::cell::RefCell;
use std::fs;
use std::ops::Add;
use std::path::PathBuf;
use std::rc::Rc;
use crate::test_config::EmulatorTestConfig;

pub type OnHandleDir<'a> = Box<dyn Fn(&PathBuf, &IteratorState) -> HandleDirectory + 'a>;
pub type OnFileFound<'a> = Box<dyn Fn(&PathBuf, &IteratorState) -> Vec<EmulatorTestConfig> + 'a>;


/// A set of callbacks to let the caller generate code for each
/// file or directory found.
pub struct FindRomCallbacks<'a> {
    /// Called on entering a directory.
    pub on_handle_dir: OnHandleDir<'a>,

    /// Called for each file.
    pub on_file_found: OnFileFound<'a>,
}


/// A visitor trait to be implemented by objects to receive test configurations.
pub trait TestConfigVisitor {
    /// Called when a new module is opened.
    fn on_open_module(&mut self, module_name: &str, state: &IteratorState);

    /// Called when a previously opened module is closed.
    fn on_close_module(&mut self, module_name: &str, state: &IteratorState);

    /// Called when a test configuration is found.
    fn on_visit_test(&mut self, test_cfg: &EmulatorTestConfig, state: &IteratorState);
}

/// Type alias for a reference to a test config visitor.
pub type TestConfigVisitorRef = Rc<RefCell<dyn TestConfigVisitor>>;


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


/// An entry of the module stack inside [VisitorStateTracker].
struct ModuleStackEntry {
    /// The name of the module.
    name: String,

    /// The state of the module.
    state: IteratorState,

    /// Whether the module has been opened.
    did_open: bool,
}


/// A tracker object storing the current state of the visitor.
/// It stores in which module the visitor is currently in and
/// delays the opening of modules until the first test is found.
struct VisitorStateTracker {
    /// The name of the top level module.
    root_name: String,

    /// The state of the top level module.
    root_state: IteratorState,

    /// The stack of all currently opened modules.
    state_stack: Vec<ModuleStackEntry>,

    /// The visitor to be notified for detected modules and tests.
    visitor: TestConfigVisitorRef,
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


impl VisitorStateTracker {
    pub fn new(root_name: String, visitor: TestConfigVisitorRef) -> Self {
        Self {
            root_name: root_name.clone(),
            root_state: IteratorState {
                indent: "".to_string(),
                path:   root_name.clone(),
                stack:  vec![root_name.clone()],
            },
            state_stack: vec![],
            visitor,
        }
    }


    /// Get the current module name.
    pub fn get_current_module_name(&self) -> &str {
        if let Some(entry) = self.state_stack.last() {
            &entry.name
        }
        else {
            &self.root_name
        }
    }


    /// Get the current state of the module we're in.
    pub fn get_current_state(&self) -> &IteratorState {
        if let Some(entry) = self.state_stack.last() {
            &entry.state
        }
        else {
            &self.root_state
        }
    }


    /// Open a new submodule. The call on the visitor will be delayed until a test is found.
    pub fn open_module(&mut self, module_name: &str) {
        let current_state = self.get_current_state();
        let sub_state     = current_state.create_sub_state(module_name);

        // push a new entry to the stack of opened modules
        self.state_stack.push(ModuleStackEntry {
            name: module_name.to_string(),
            state: sub_state,
            did_open: false,
        });
    }


    /// Close the current module. This will be ignored unless the module was forwarded to the visitor as well.
    pub fn close_module(&mut self, module_name: &str) {
        assert!(!self.state_stack.is_empty(), "Module stack is empty");

        // check whether there is a module to close and if it matches the given name
        let (can_pop, was_opened) = if let Some(entry) = self.state_stack.last() {
            assert_eq!(entry.name, module_name, "Module name mismatch");
            let matching_name = entry.name == module_name;

            (matching_name, entry.did_open)
        }
        else {
            (false, false)
        };

        if can_pop {
            // remove from the stack
            self.state_stack.pop();

            // when it was opened, send the closing event to the visitor
            if was_opened {
                self.visitor.borrow_mut().on_close_module(module_name, self.get_current_state());
            }
        }
    }


    /// Visit a test configuration.
    /// This will invoke the module opening on the visitor if it wasn't done yet.
    pub fn visit_test(&mut self, test_cfg: &EmulatorTestConfig) {
        self.assure_modules_opened();

        self.visitor.borrow_mut().on_visit_test(test_cfg, self.get_current_state());
    }


    /// Assure that all currently opened modules are opened on the visitor.
    fn assure_modules_opened(&mut self) {
        let mut parent_state: &IteratorState = &self.root_state;

        // iterate through all modules
        for entry in self.state_stack.iter_mut() {
            // if the module wasn't opened yet...
            if !entry.did_open {
                // ... send the opening event to the visitor
                self.visitor.borrow_mut().on_open_module(&entry.name, parent_state);
                entry.did_open = true;
            }

            parent_state = &entry.state;
        }
    }
}


/// Recursively iterate through subdirectories to find ROM files.
/// Invokes a callback for each directory and file found. Collects the generated code
/// from each callback and creates file content with all generated tests.
pub fn recursive_visit_directory(
        root_path: PathBuf,
        callbacks: &FindRomCallbacks,
        visitor:   TestConfigVisitorRef
) {
    let root_name = filename_to_symbol(root_path.file_name().unwrap().to_str().unwrap());
    let mut state = VisitorStateTracker::new(root_name.clone(), visitor);

    recursive_visit_directory_step(
        &root_path,
        callbacks,
        &mut state
    );
}


/// Internal part of recursive_find_roms which is called recursively.
fn recursive_visit_directory_step(
        current_path: &PathBuf,
        callbacks: &FindRomCallbacks,
        state: &mut VisitorStateTracker
) {
    let mut subdir_list : Vec<PathBuf> = Vec::new();
    let mut files_list  : Vec<PathBuf> = Vec::new();
    let mut had_submodules = false;

    // iterate through all elements in the current path
    for paths in current_path.read_dir().unwrap() {
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
        let handle_dir : HandleDirectory = (callbacks.on_handle_dir)(&path, state.get_current_state());

        // recurse into subdirectory, if not skipped
        match handle_dir {
            HandleDirectory::Ignore => { }

            HandleDirectory::Enter => {
                // recurse into the subdirectory
                recursive_visit_directory_step(
                    &path,
                    callbacks,
                    state
                );
            }

            HandleDirectory::CreateModule => {
                // open a new module
                state.open_module(&module);

                // recurse into the subdirectory
                recursive_visit_directory_step(
                    &path,
                    callbacks,
                    state
                );

                // close the module created
                state.close_module(&module);

                // mark that we had submodules
                had_submodules = true;
            }
        };
    }

    // if any files were found..
    if files_list.len() > 0 {
        // functor to visit all files, which can be run either in the current module
        // or in a separate module
        let visit_files = |files_list: &Vec<PathBuf>, state: &mut VisitorStateTracker| {
            // iterate through all files
            for path in files_list {
                // call the on_file_found callback
                let tests: Vec<EmulatorTestConfig> = (callbacks.on_file_found)(&path, state.get_current_state());

                // visit each test found
                for test_cfg in tests {
                    state.visit_test(&test_cfg);
                }
            }
        };

        // if we have files in a directory which also contained submodules, we create
        // a separate module for the files, so tests wont be mixed up with directories
        if had_submodules {
            // create a module for the files
            let files_module = format!("{}_other", state.get_current_module_name());
            state.open_module(&files_module);

            // visit all files in the submodule
            visit_files(&files_list, state);

            // close the module
            state.close_module(&files_module);
        }
        else {
            // visit all files in the current module
            visit_files(&files_list, state);
        }
    }
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
