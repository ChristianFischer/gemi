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

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use crate::ppu::{LcdBuffer, SCREEN_H, SCREEN_PIXELS, SCREEN_W};


pub enum State {
    Open,
    Closed,
}


/// A window to present the Gameboy's output.
pub struct Window {
    event_pump:     sdl2::EventPump,
    canvas:         sdl2::render::WindowCanvas,
    texture:        sdl2::render::Texture,
    color_buffer:   Box<[u8]>,
    state:          State,
}


impl Window {
    /// Creates a new window with a given size and title.
    pub fn create(title: &str, width: u32, height: u32) -> Result<Window, String> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;
        let event_pump = sdl.event_pump()?;

        let window = video
            .window(title, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())
            ?
        ;

        let canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())
            ?
        ;

        let texture = canvas
            .texture_creator()
            .create_texture_streaming(None, width, height)
            .map_err(|e| e.to_string())
            ?
        ;

        Ok(Window {
            event_pump,
            canvas,
            texture,
            color_buffer: vec![0x00; SCREEN_PIXELS * 4].into_boxed_slice(),
            state: State::Open,
        })
    }


    /// Checks whether the window is open.
    pub fn is_opened(&self) -> bool {
        match self.state {
            State::Open   => true,
            State::Closed => false,
        }
    }


    /// Close the window.
    pub fn close(&mut self) {
        self.state = State::Closed;
    }

    /// Polls and handles events of this window.
    pub fn poll_events(&mut self) {
        while let Some(event) = self.event_pump.poll_event() {
            match event {
                Event::Quit { .. } => {
                    self.close();
                }

                Event::KeyDown { keycode: Some(keycode), .. } => {
                    self.handle_key_down(keycode);
                }

                _ => { }
            }
        }
    }

    fn handle_key_down(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::Escape => { self.close(); }
            _ => { }
        }
    }

    /// Presents the content of a LCD buffer on the window.
    pub fn present(&mut self, lcd: &LcdBuffer) {
        // convert palette based image data into RGBA
        for i in 0..SCREEN_PIXELS {
            let color_index = lcd.get_pixels()[i];
            let color = match color_index {
                3 => 0xffffffffu32,
                2 => 0x808080ffu32,
                1 => 0x404040ffu32,
                _ => 0x000000ffu32,
            };

            self.color_buffer[i * 4 + 0] = ((color >> 24) & 0xff) as u8;
            self.color_buffer[i * 4 + 1] = ((color >> 16) & 0xff) as u8;
            self.color_buffer[i * 4 + 2] = ((color >>  8) & 0xff) as u8;
            self.color_buffer[i * 4 + 3] = ((color >>  0) & 0xff) as u8;
        }

        // update texture
        self.texture
            .update(
                None,
                &self.color_buffer,
                (SCREEN_W * 4) as usize
            )
            .map_err(|e| e.to_string())
            .unwrap()
        ;

        // copy texture into framebuffer
        self.canvas
            .copy(
                &self.texture,
                None,
                Rect::new(0, 0, SCREEN_W, SCREEN_H)
            )
            .unwrap()
        ;

        // present the framebuffer
        self.canvas.present();
    }
}
