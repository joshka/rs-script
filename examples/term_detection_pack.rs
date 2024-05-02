/*[toml]
[dependencies]
gethostname = "0.4.3"
supports-color= "3.0.0"
termbg = "0.5.0"
terminal-light = "1.4.0"
*/

use supports_color::Stream;
use terminal_light::r255;

use gethostname::gethostname;

println!("Hostname: {:?}", gethostname());

let timeout = std::time::Duration::from_millis(100);

println!(r"Crate termbg:
    (Note that crate termbg seems to ");

let term = termbg::terminal();
let rgb = termbg::rgb(timeout);
let theme = termbg::theme(timeout);

println!("  Term : {:?}", term);

match rgb {
    Ok(rgb) => {
        println!("  Color: R={}, G={}, B={}", r255(rgb.r.into()), rgb.g / 257, rgb.b / 257);
        println!("  Color={rgb:#?}");
    }
    Err(e) => {
        println!("  Color: detection failed {:?}", e);
    }
}

match theme {
    Ok(theme) => {
        println!("  Theme: {:?}", theme);
    }
    Err(e) => {
        println!("  Theme: detection failed {:?}", e);
    }
}

println!("\nCrate terminal_light:");

let luma = terminal_light::luma();
println!("luma={luma:#?}");
match luma {
    Ok(luma) if luma > 0.5 => {
        // Use a "light mode" skin.
        println!("Light mode");
    }
    Ok(luma) if luma < 0.5 => {
        // Use a "dark mode" skin.
        println!("Dark mode");
    }
    _ => {
        // Either we couldn't determine the mode or it's kind of medium.
        // We should use an intermediate skin, or one defining the background.
        println!("Intermediate mode");
    }
}

match terminal_light::background_color()
    .map(|c| c.rgb()) {
        Ok(bg_rgb) =>
 {
let luma_255 = 0.2126 * (bg_rgb.r as f32) + 0.7152 * (bg_rgb.g as f32) + 0.0722 * (bg_rgb.b as f32);
let luma_0_to_1 = luma_255 / 255.0;
println!("\nTerminal-light: Background color is {bg_rgb:#?}, luma_255={luma_255}, luma_0_to_1={luma_0_to_1}");
}
Err(_) => println!("terminal_light::background_color() not supported"),    }

println!("\nCrate supports-color:");

if let Some(support) = supports_color::on(Stream::Stdout) {
    if support.has_16m {
        println!("16 million (RGB) colors are supported");
    } else if support.has_256 {
        println!("256 colors are supported.");
    } else if support.has_basic {
        println!("Only basic ANSI colors are supported.");
    }
} else {
    println!("No color support.");
}
