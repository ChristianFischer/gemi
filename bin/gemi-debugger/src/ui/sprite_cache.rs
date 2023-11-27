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

use std::collections::HashMap;
use std::sync::Mutex;
use egui::{ColorImage, TextureOptions, Ui};
use lazy_static::lazy_static;
use gemi_core::ppu::sprite_image::SpriteImage;


/// The maximum number of sprites to cache.
/// If this number is exceeded, the oldest entries will be removed.
pub const MAX_SPRITE_CACHE_SIZE : usize = 4096;


lazy_static! {
    static ref INSTANCE: Mutex<SpriteCache> = Mutex::new(SpriteCache::new());
}


/// The data struct holding all cached data.
pub struct SpriteCache {
    map: HashMap<SpriteImage, SpriteCacheEntry>,
}


/// A single entry in the sprite cache.
struct SpriteCacheEntry {
    /// The texture being cached.
    texture: egui::TextureHandle,

    /// This counts the time since the last usage of this entry.
    /// The value is incremented each frame and reset to zero on each usage.
    time_since_used: usize,
}


/// Returns a texture for the given sprite image.
/// The texture will be created once and cached, so any subsequent calls
/// will return the same texture.
pub fn get_texture_for(ui: &mut Ui, sprite_image: &SpriteImage) -> egui::TextureHandle {
    let mut instance = INSTANCE.lock().unwrap();

    if let Some(entry) = instance.map.get_mut(sprite_image) {
        // reset the age of the entry, once it is used
        entry.time_since_used = 0;

        entry.texture.clone()
    }
    else {
        let id      = sprite_image.to_hex_string();
        let pixels  = sprite_image.to_rgba();
        let image   = ColorImage::from_rgba_unmultiplied([8, 8], &pixels);
        let texture = ui.ctx().load_texture(id, image, TextureOptions::NEAREST);

        instance.map.insert(
            sprite_image.clone(),
            SpriteCacheEntry {
                texture: texture.clone(),
                time_since_used: 0,
            }
        );

        texture
    }
}


/// To be called once per frame to update the age of each entry
/// and to remove old entries, if the cache size exceeds the maximum.
pub fn on_frame() {
    let mut instance = INSTANCE.lock().unwrap();

    // if we did exceed the maximum cache size, remove all entries, which
    // have not been used since some time.
    if instance.map.len() >= MAX_SPRITE_CACHE_SIZE {
        instance.map.retain(|_, entry| entry.time_since_used == 0);
    }

    // increase the age of all entries
    for entry in instance.map.values_mut() {
        entry.time_since_used += 1;
    }
}


impl SpriteCache {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}
