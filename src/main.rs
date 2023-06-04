use image::{GenericImageView};


fn print_char(br:u8) -> &'static str {

    let parse_br = br/32; 

    let index:[&str; 8] = [" ", ".", ",", "-", "~", "+", "=", "@"];
    index[parse_br as usize]  
}

fn create_ascii(dir: &str, scale: u32) {
    let img = image::open(dir).unwrap();
    let (width, height) = img.dimensions();

     
    for y in 0..height {
        for x in 0..width {
            if y % (scale * 2) == 0 && x % scale == 0 {
                let pix = img.get_pixel(x, y);
                let mut brightness = pix[0]/3 + pix[1]/3 + pix[2]/3;

                if pix[3] == 0 {
                    brightness = 0;
                }

                print!("{}", print_char(brightness));

            }
            
        }
        if y % scale == 0 { 
            println!("");
        }
    }
 


}


fn main() {
    create_ascii("cat_wtf.jpg", 5); 
}
