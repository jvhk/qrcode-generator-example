use qrcode::QrCode;
use image::Luma;

fn main(){
    // generate a encode in bits format 
    let code = QrCode::new(b"https://jvhk.github.io").unwrap();

    // render image using Lumav0.18 check: https://docs.rs/image/0.18.0/image/struct.Luma.html to more details
    let image = code.render::<Luma<u8>>().build();

    //saves to file in root folder
    image.save("qr.png").unwrap();

    /*if you want to see the bits of image use the line bellow*/
    //println!("{:?}", image);
}