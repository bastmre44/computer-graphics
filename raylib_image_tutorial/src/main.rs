use raylib::prelude::*;

fn main() {
    let image_width = 500;
    let image_height = 500;

    let raw_image = unsafe {
        raylib::ffi::GenImageColor(
            image_width,
            image_height,
            Color::BLACK,
        )
    };

    let mut new_image = unsafe {
        Image::from_raw(raw_image)
    };

   
    let cx1 = 150;
    let cy1 = 150;

    for i in -40..=40 {
       
        new_image.draw_pixel(cx1 + i, cy1, Color::YELLOW);

       
        new_image.draw_pixel(cx1, cy1 + i, Color::YELLOW);

        
        new_image.draw_pixel(cx1 + i, cy1 + i, Color::YELLOW);

        
        new_image.draw_pixel(cx1 + i, cy1 - i, Color::YELLOW);
    }

    
    let cx2 = 330;
    let cy2 = 300;

    for i in -50..=50 {
        
        new_image.draw_pixel(cx2 + i, cy2, Color::PURPLE);

        
        new_image.draw_pixel(cx2, cy2 + i, Color::PURPLE);

        new_image.draw_pixel(cx2 + i, cy2 + i, Color::PURPLE);

        new_image.draw_pixel(cx2 + i, cy2 - i, Color::PURPLE);
    }

    let output_file_name = "two_stars.png";

    new_image.export_image(output_file_name);

    println!(
        "Estrellas guardadas como '{}'!",
        output_file_name
    );
}