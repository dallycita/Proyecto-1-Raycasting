use raylib::prelude::*;
use std::collections::HashMap;

pub struct TextureManager {
    textures: HashMap<char, Image>,
}

impl TextureManager {
    pub fn new() -> Self {
        let mut textures = HashMap::new();

        textures.insert('+', create_brick_texture());
        textures.insert('-', create_blue_texture());
        textures.insert('|', create_green_texture());
        textures.insert('g', create_purple_texture());
        textures.insert('E', create_exit_texture());

        Self { textures }
    }

    pub fn get_pixel_color(
        &mut self,
        wall_type: char,
        tx: i32,
        ty: i32,
    ) -> Color {
        if let Some(image) =
            self.textures.get_mut(&wall_type)
        {
            let width = image.width;
            let height = image.height;

            if tx >= 0
                && tx < width
                && ty >= 0
                && ty < height
            {
                return image.get_color(tx, ty);
            }
        }

        Color::WHITE
    }

    pub fn get_size(
        &self,
        wall_type: char,
    ) -> (i32, i32) {
        if let Some(image) =
            self.textures.get(&wall_type)
        {
            return (
                image.width,
                image.height
            );
        }

        (64, 64)
    }
}

fn create_brick_texture() -> Image {
    let mut image =
        Image::gen_image_color(
            64,
            64,
            Color::new(130, 35, 35, 255),
        );

    for y in (0..64).step_by(16) {
        for x in 0..64 {
            image.draw_pixel(
                x,
                y,
                Color::new(70, 20, 20, 255),
            );
        }
    }

    for y in 0..64 {
        let offset =
            if (y / 16) % 2 == 0 {
                0
            } else {
                8
            };

        for x in (offset..64).step_by(16) {
            image.draw_pixel(
                x,
                y,
                Color::new(70, 20, 20, 255),
            );
        }
    }

    image
}

fn create_blue_texture() -> Image {
    let mut image =
        Image::gen_image_color(
            64,
            64,
            Color::new(30, 80, 170, 255),
        );

    for x in (0..64).step_by(8) {
        for y in 0..64 {
            image.draw_pixel(
                x,
                y,
                Color::new(15, 40, 100, 255),
            );
        }
    }

    image
}

fn create_green_texture() -> Image {
    let mut image =
        Image::gen_image_color(
            64,
            64,
            Color::new(40, 130, 70, 255),
        );

    for y in (0..64).step_by(8) {
        for x in 0..64 {
            image.draw_pixel(
                x,
                y,
                Color::new(20, 80, 40, 255),
            );
        }
    }

    image
}

fn create_purple_texture() -> Image {
    let mut image =
        Image::gen_image_color(
            64,
            64,
            Color::new(110, 50, 150, 255),
        );

    for y in 0..64 {
        for x in 0..64 {
            if (x + y) % 16 < 4 {
                image.draw_pixel(
                    x,
                    y,
                    Color::new(
                        180,
                        90,
                        210,
                        255,
                    ),
                );
            }
        }
    }

    image
}

fn create_exit_texture() -> Image {
    let mut image =
        Image::gen_image_color(
            64,
            64,
            Color::GOLD,
        );

    for x in 0..64 {
        image.draw_pixel(
            x,
            0,
            Color::YELLOW,
        );

        image.draw_pixel(
            x,
            63,
            Color::YELLOW,
        );
    }

    for y in 0..64 {
        image.draw_pixel(
            0,
            y,
            Color::YELLOW,
        );

        image.draw_pixel(
            63,
            y,
            Color::YELLOW,
        );
    }

    image
}
