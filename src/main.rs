#![allow(non_snake_case)]
#![allow(unused_parens)]
use std::env;
use std::{
    io::{self, Read, Write, StdoutLock},
	hint,
	process::{Command, Stdio},
    time::Instant,
};

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
fn clr() {
    print!("\x1B[2J\x1B[1;1H"); // escape code, clear line, escape code, move to top
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

    let _ = outbuf.write(format!("\x1B[2;1H\x1B[38;2;255;255;255m{:8}\n{:8}",&flip_state,&sin_val).as_bytes());

    let _ = outbuf.write(format!("\x1B[38;2;{0};{0};{0}m",&shade).as_bytes());
    let _ = outbuf.write("\x1B[48;2;0;0;0m".as_bytes());

    for i in 0..=height {
		let mut y = 0_f64 - i as f64;
		let yy = y*y;
		let yyww = yy*ww;
		let height = ((size as f64 / 2.2_f64) * (sin_val as f64)) as f64;
    	let hh = height*height;
		let hhww = hh*ww;

        for j in 0..=width as usize {
            let mut x = 0_f64 - j as f64;

            if y_pos + y < 1_f64 {
                y -= y_pos + y;
            }
            if x_pos + x < 1_f64 {
                x -= x_pos + x;
            }
            if (x*x*hh) + (yyww) <= (hhww) {
                
				let mut outstr = format!(
                    "\x1B[{};{}H@",
                    ((y_pos + y) as i32),
                    ((x_pos + x) as i32)
				);
                let _ = outbuf.write(outstr.as_bytes());
                outstr = format!(
                    "\x1B[{};{}H@",
                    ((y_pos - y) as i32).to_string(),
                    ((x_pos + x) as i32).to_string()
                );
				let _ = outbuf.write(outstr.as_bytes());
				outstr = format!(
                    "\x1B[{};{}H@",
                    ((y_pos + y) as i32).to_string(),
                    ((x_pos - x) as i32).to_string()
                );
				let _ = outbuf.write(outstr.as_bytes());
				outstr = format!(
                    "\x1B[{};{}H@",
                    ((y_pos - y) as i32).to_string(),
                    ((x_pos - x) as i32).to_string()
                );
				let _ = outbuf.write(outstr.as_bytes());
            } else {
                let mut outstr = format!(
                    "\x1B[{};{}H ",
                    ((y_pos + y) as i32).to_string(),
                    ((x_pos + x) as i32).to_string()
                );
                let _ = outbuf.write(outstr.as_bytes());
                outstr = format!(
                    "\x1B[{};{}H ",
                    ((y_pos - y) as i32).to_string(),
                    ((x_pos + x) as i32).to_string()
                );
				let _ = outbuf.write(outstr.as_bytes());
                outstr = format!(
                    "\x1B[{};{}H ",
                    ((y_pos + y) as i32).to_string(),
                    ((x_pos - x) as i32).to_string()
                );
				let _ = outbuf.write(outstr.as_bytes());
                outstr = format!(
                    "\x1B[{};{}H ",
                    ((y_pos - y) as i32).to_string(),
                    ((x_pos - x) as i32).to_string()
                );
				let _ = outbuf.write(outstr.as_bytes());
            }
        }
			
    }
    print!("\x1B[48;2;0;0;0m");
}

fn main() {
	let outbuf: &mut StdoutLock = &mut io::stdout().lock();

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

    let CENTER = [(WIDTH / 2) as usize, (HEIGHT / 2) as usize];


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

        draw_coin(CENTER[0], CENTER[1], coin_size, flipstate, outbuf);
        let _ = outbuf.flush();
		
        flipstate = (flipspeed*start.elapsed().as_nanos() as f64) % (std::f64::consts::PI);
		
        if start.elapsed().as_nanos() >= time {
            break;
        } else {
			hint::spin_loop();
		}
        
    }

    show_cursor();
}
