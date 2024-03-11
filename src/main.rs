use std::env;
#[allow(unused_imports)]
use std::{
    io::{self, BufRead, BufReader, Read, Write, StdoutLock},
    process::{Command, Stdio},
	hint,
    thread,
    thread::sleep,
    time::{Duration, Instant},
};

fn iwrite_string(string: &mut String) {
    print!("{}", string);
    let _ = io::stdout().flush();
}
fn iwrite_str(string: &str) {
    print!("{}", string);
    let _ = io::stdout().flush();
    // initial thoughts: superior as it can receive either a String or str reference
}
#[allow(dead_code)]
fn concat_vec(string_list: Vec<&str>) -> String {
	if string_list.len() == 0 {
		return String::new();
	}
    let mut concatenated = String::from(string_list[0]);
    for i in 1..string_list.len() {
        concatenated.push_str(string_list[i]);
    }
    return concatenated;
}
fn spawn_read(cmd: &String, args: &[String]) -> String {
    let mut out = String::new();
    let mut command = Command::new(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    println!(
        "{}",
        command.wait().expect("failed to execute\n").to_string()
    );
    let mut command = command.stdout.take().unwrap();
    let _ = command.read_to_string(&mut out);

    return out;
}

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>()) // prints std::io::Stdout
}
fn clr() {
    print!("\x1B[2J\x1B[1;1H"); // escape code, clear line, escape code, move to top
}
fn clr_line() {
    print!("\x1B[2J"); // clear line
}
fn clr_line_at(y: usize) {
	let y = y.to_string();
    print!("\x1B[{};1H\x1B[2J",&y); // clear line
}
fn hide_cursor() {
    print!("\x1b[3 q"); // 3 or 4 means underline cursor
    print!("\x1b[?25l"); //low means off
    flush_out();
}
fn show_cursor() {
    print!("\x1b[2 q"); // 2 means block cursor
    print!("\x1b[?25h"); //high means on
    flush_out();
}
fn flush_out() {
    let _ = io::stdout().flush();
}
fn draw_coin(x_pos: usize, y_pos: usize, radius: usize, flip_state: f64, outbuf: &mut StdoutLock) {
    let sin_val = flip_state.sin();
    let size = (2 * radius) + 1;
    let height = size as usize;
    let width = size as usize;
    let x_pos = (x_pos as f64);
    let y_pos = (y_pos as f64);
    let shade = (flip_state) / (2.0 * std::f64::consts::PI);
    let mut shade = (100.0 + ((shade % 0.5) * 280.0)) as i32;
    if shade > 200_i32 {
        shade = 200_i32;
    }

	let width = width as f64;
	let ww = width*width;

    outbuf.write(format!("\x1B[2;1H\x1B[38;2;255;255;255m{:8}\n{:8}",&flip_state,&sin_val).as_bytes());

    outbuf.write(format!("\x1B[38;2;{0};{0};{0}m",&shade).as_bytes());
    outbuf.write("\x1B[48;2;0;0;0m".as_bytes());

    for i in 0..=height {
		let mut y = 0_f64 - i as f64;
		let yy = y*y;
		let mut yyww = yy*ww;
		let height = ((size as f64 / 2.2_f64) * (sin_val as f64)) as f64;
    	let hh = height*height;
		let hhww = hh*ww;;

		let mut prefix = String::new();

        for j in 0..=width as usize{
            //		print!("\x1B[{0};{0}H",&pos.to_string());
            let mut x = 0_f64 - j as f64;

            if y_pos + y < 1_f64 {
                y -= y_pos + y;
            }
            if x_pos + x < 1_f64 {
                x -= x_pos + x;
            }
            if (x*x*hh) + (yyww) <= (hhww)
            //&& (x*x*height*height)/5.0+(y*y*width*width)/5.0 >= (height*height*width*width)/5.0
            {
                /*
                match (x_bg*x_bg*height_bg*height_bg)+(y_bg*y_bg*width_bg*width_bg) <= (height_bg*height_bg*width_bg*width_bg) {
                    true => /*prefix = concat_vec(vec!["\x1B[48;2;",
                        shade.to_string().as_str(),";",
                        shade.to_string().as_str(),";",
                        shade.to_string().as_str(),
                        "m"])*/  outbuf.write("\x1B[38;2;{0};{0};{0}m",(shade - 50_i32)),
                    false => prefix = String::from("\x1B[48;2;0;0;0m"),
                }
                */
                
				let mut outstr = format!(
                    "{}\x1B[{};{}H@",
                    prefix,
                    ((y_pos + y) as i32),
                    ((x_pos + x) as i32)
				);
                outbuf.write(outstr.as_bytes());
                outstr = format!(
                    "{}\x1B[{};{}H@",
                    prefix,
                    ((y_pos - y) as i32).to_string(),
                    ((x_pos + x) as i32).to_string()
                );
				outbuf.write(outstr.as_bytes());
				let mut outstr = format!(
                    "{}\x1B[{};{}H@",
                    prefix,
                    ((y_pos + y) as i32).to_string(),
                    ((x_pos - x) as i32).to_string()
                );
				outbuf.write(outstr.as_bytes());
				let mut outstr = format!(
                    "{}\x1B[{};{}H@",
                    prefix,
                    ((y_pos - y) as i32).to_string(),
                    ((x_pos - x) as i32).to_string()
                );
				outbuf.write(outstr.as_bytes());
            } else {
                let mut outstr = format!(
                    "{}\x1B[{};{}H ",
                    prefix,
                    ((y_pos + y) as i32).to_string(),
                    ((x_pos + x) as i32).to_string()
                );
                outbuf.write(outstr.as_bytes());
                outstr = format!(
                    "{}\x1B[{};{}H ",
                    prefix,
                    ((y_pos - y) as i32).to_string(),
                    ((x_pos + x) as i32).to_string()
                );
				outbuf.write(outstr.as_bytes());
                let mut outstr = format!(
                    "{}\x1B[{};{}H ",
                    prefix,
                    ((y_pos + y) as i32).to_string(),
                    ((x_pos - x) as i32).to_string()
                );
				outbuf.write(outstr.as_bytes());
                let mut outstr = format!(
                    "{}\x1B[{};{}H ",
                    prefix,
                    ((y_pos - y) as i32).to_string(),
                    ((x_pos - x) as i32).to_string()
                );
				outbuf.write(outstr.as_bytes());
            }
        }

//	clr_line_at( (y_pos as usize - ( ( radius-1 ) / 2 ) + 1 ) as usize);
//	clr_line_at( (y_pos as usize + ( ( radius-1 ) / 2 ) + 1 ) as usize);
			
    }
    print!("\x1B[48;2;0;0;0m");
}

fn main() {
	let mut outbuf: &mut StdoutLock = &mut io::stdout().lock();
    let mut average_time = Box::new([(1000f64 / 60f64) as u128; 2000]);

    let mut args = env::args().collect::<Vec<String>>();
    let time = (match args[1].trim().parse() {
        Ok(num) => num,
        Err(_) => 10u128,
    } * 1_000_000_000u128) as u128;
	
    let HEIGHT = spawn_read(&"tput".to_string(), &["lines".to_string()]);
    let WIDTH = spawn_read(&"tput".to_string(), &["cols".to_string()]);
    let HEIGHT: i32 = match HEIGHT.trim().parse() {
        Ok(num) => num,
        _ => {
            println!("Choosing default terminal height.");
            24_i32
        }
    };
    let WIDTH: i32 = match WIDTH.trim().parse() {
        Ok(num) => num,
        _ => {
            println!("Choosing default terminal width.");
            80_i32
        }
    };

    dbg!(HEIGHT, WIDTH);
    let CENTER = [(WIDTH / 2) as usize, (HEIGHT / 2) as usize];

//    sleep(Duration::from_secs(2u64));

    let mut time_index = 0usize;
    let mut flipstate: f64 = 0.0;
	let factor = 5 as f64;
    let flipspeed: f64 = (0.00360f64*factor/(1_00_000) as f64)/60.0;
    clr();

    let mut coin_size = 0;
    match CENTER[0] >= CENTER[1] {
        true => coin_size += ((CENTER[0] - (CENTER[0] % 2)) as f64 / 4.2f64) as usize,
        
		false => coin_size += ((CENTER[1] - (CENTER[1] % 2)) as f64 / 4.2f64) as usize,
    }
	if args.len() <= 3 {
		args.push(coin_size.to_string());
	}
	let coin_size = match args[2].trim().parse() {
		Ok(num) => num,
		Err(_) => coin_size,
	};


	for _ in 0..HEIGHT {
		println!();
	}
	
    hide_cursor();
    let start = Instant::now();
    loop {
        
        let loop_start = Instant::now();

        draw_coin(CENTER[0], CENTER[1], coin_size, flipstate, outbuf);
        let _ = outbuf.flush();
	
        flipstate += flipspeed;
        flipstate = (flipspeed*start.elapsed().as_nanos() as f64) % (std::f64::consts::PI);

        let time_diff = loop_start.elapsed().as_nanos();
        let mut sleep_time = ((1_000_000_000f64 / (60f64)) as i128 - time_diff as i128);
        sleep_time =
            0_i128 * ((sleep_time < 0_i128) as i128) + sleep_time * ((sleep_time >= 0_i128) as i128);
		
        if start.elapsed().as_nanos() >= time {
            break;
        } else {
			hint::spin_loop(); //test
		}
        
    }

    show_cursor();
}
