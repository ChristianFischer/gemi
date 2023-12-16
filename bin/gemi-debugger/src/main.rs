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

use std::env;
use std::path::PathBuf;
use eframe::{CreationContext, NativeOptions, run_native};
use egui::ViewportBuilder;
use crate::app::EmulatorApplication;

mod views;
mod app;
mod behaviour;
mod event;
mod state;
mod strings;
mod ui;
mod view_response;


/// Load the application. If possible, try to load the application state
/// from the storage. If this fails, create a new one by default values.
/// After creating the application, handle any arguments passed to the
/// application.
fn load_application(cc: &CreationContext) -> EmulatorApplication {
    // try to load the application state from the storage or create a new one
    let mut app =
        EmulatorApplication::from_creation_context(cc)
        .unwrap_or_else(|| EmulatorApplication::default())
    ;

    // if there are any arguments, take them as file reference
    // and try to open the ROM.
    env::args().into_iter().skip(1).for_each(|arg| {
        let path = PathBuf::from(arg);

        // open the ROM, fail on error
        app.open_rom(&path).unwrap();
    });

    app
}


fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
                .with_app_id("gemi-debugger"),
        .. NativeOptions::default()
    };

    run_native(
        "GameBoy Debugger",
        options,
        Box::new(|cc| {
            let app = load_application(cc);
            Box::new(app)
        })
    )
}
