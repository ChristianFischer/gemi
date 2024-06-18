/*
 * Copyright (C) 2022-2024 by Christian Fischer
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


use std::env;
use std::path::PathBuf;

use egui::{Link, ScrollArea, Ui};

use crate::state::EmulatorState;
use crate::views::View;

/// A file browser view to list ROM files from within the current working directory.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FileBrowserView {
    #[serde(skip)]
    rt: FileBrowserRuntime,
}


/// Runtime data of the file browser, which does not get serialized.
struct FileBrowserRuntime {
    root: Folder,
}


/// A single folder with the subfolders and files it's containing.
struct Folder {
    /// The folder's name.
    name: String,
    
    /// Subfolders of this folder.
    subdirectories: Vec<Folder>,
    
    /// ROM files within this folder.
    files: Vec<PathBuf>,
}


impl FileBrowserView {
    /// Creates a new `FileBrowserView` for the current working directory.
    pub fn new() -> Self {
        Self {
            rt: Default::default(),
        }
    }
}


impl View for FileBrowserView {
    fn title(&self, _state: &mut EmulatorState) -> &str {
        "File Browser"
    }

    fn ui(&mut self, state: &mut EmulatorState, ui: &mut Ui) {
        ScrollArea::new([false, true])
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    self.display_folder_node(state, ui, &self.rt.root);
                })
        ;
    }
}


impl FileBrowserView {
    /// Displays a node for a single folder and it's subfolders.
    fn display_folder_node(&self, state: &mut EmulatorState, ui: &mut Ui, folder: &Folder) {
        ui.collapsing(&folder.name, |ui| {
            ui.vertical(|ui| {
                // other folders
                for subdirectory in &folder.subdirectories {
                    self.display_folder_node(state, ui, subdirectory);
                }

                // files
                for file in &folder.files {
                    if let Some(s) = file.file_name().and_then(|s| s.to_str()) {
                        ui.indent("i", |ui|{
                            let response = ui.add(Link::new(s));

                            if response.clicked() {
                                self.on_clicked_rom(state, file);
                            }
                        });
                    }
                }
            })
        });
    }


    fn on_clicked_rom(&self, state: &mut EmulatorState, path: &PathBuf) {
        _ = state.open_rom(path);
    }
}


impl FileBrowserRuntime {
    pub fn from_path(path: PathBuf) -> Self {
        Self {
            root: Folder::parse(path, 5),
        }
    }
}


impl Folder {
    /// Parse a folder's content into a [Folder] object.
    /// This only reads `max_depth` levels into the folder structure to prevent 
    /// this function from loading too much data.
    pub fn parse(path: PathBuf, max_depth: u32) -> Self {
        match path.read_dir() {
            Ok(read_dir) => {
                let mut subdirectories: Vec<Folder>  = Vec::new();
                let mut files:          Vec<PathBuf> = Vec::new();

                for entry in read_dir {
                    if let Ok(entry) = entry {
                        let path = entry.path();

                        if path.is_dir() {
                            if max_depth > 0 {
                                let subfolder= Folder::parse(path, max_depth - 1);
                                if !subfolder.is_empty() {
                                    subdirectories.push(subfolder);
                                }
                            }
                        }
                        else {
                            if let Some(ext) = path.extension().and_then(|s| s.to_str()).map(|s| s.to_lowercase()) {
                                if ext == "gb" || ext == "gbc" {
                                    files.push(path);
                                }
                            }
                        }
                    }
                }

                Folder {
                    name: path.file_name().and_then(|s| s.to_str()).unwrap_or("??").to_string(),
                    subdirectories,
                    files,
                }
            },

            Err(_) => Folder::default(),
        }
    }


    /// Checks whether this folder contains no files or subdirectories.
    pub fn is_empty(&self) -> bool {
        self.subdirectories.is_empty() && self.files.is_empty()
    }
}


impl Default for FileBrowserRuntime {
    fn default() -> Self {
        match env::current_dir() {
            Ok(current_dir) => Self::from_path(current_dir),
            Err(_) => Self {
                root: Default::default(),
            }
        }
    }
}


impl Default for Folder {
    fn default() -> Self {
        Self {
            name: "<unknown>".to_string(),
            subdirectories: Vec::new(),
            files: Vec::new(),
        }
    }
}
