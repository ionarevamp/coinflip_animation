#![allow(non_snake_case)]
#![allow(unused_parens)]

extern crate ctrlc;

use std::{
	env,
    io::{self, Read, Write, StdoutLock},
	hint,
	process::{Command, Stdio},
    time::Instant,
	sync::{Arc, atomic::{AtomicBool, Ordering}}
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
        "{}",  // TODO: Implement Result for output
        command.wait().expect("failed to execute\n")
    );
    let mut command = command.stdout.take().unwrap();
    let _ = command.read_to_string(&mut out);

    out
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
    let height = size;
    let width = size;
    let x_pos = (x_pos as f64);
    let y_pos = (y_pos as f64);
    let shade = (flip_state) / (2.0 * std::f64::consts::PI);
    let mut shade = (100.0 + ((shade % 0.5) * 280.0)) as i32;
    if shade > 200_i32 {
        shade = 200_i32;
    }

	let width = width as f64;
	let ww = width*width;

    outbuf.write_all(format!("\x1B[2;1H\x1B[38;2;255;255;255m{:8}\n{:8}",&flip_state,&sin_val).as_bytes()).unwrap();

    outbuf.write_all("\x1B[48;2;0;0;0m".as_bytes()).unwrap();

    for i in 0..height+1 {
		let mut y = 0_f64 - i as f64;
		let yy = y*y;
		let yyww = yy*ww;
		let height = ((size as f64 / 2.2_f64) * sin_val);
    	let hh = height*height;
		let hhww = hh*ww;

        for j in 0..=width as usize {
            let mut draw_char = '@';
            let mut x = 0_f64 - j as f64;

            if y_pos + y < 1_f64 {
                y -= y_pos + y;
            }
            if x_pos + x < 1_f64 {
                x -= x_pos + x;
            }

            let xxhh = x*x*hh;
            let xxhh_yyww = (xxhh) + (yyww);

            if xxhh_yyww <= (hhww) {

                if xxhh_yyww > (hhww * 0.9) {
                    draw_char =// if x == 0.0 {
                    //    '='
                    //} else
                    //if y == 0.0 {
                        if sin_val <= 0.4 {
                            '▥'
                        } else if sin_val <= 0.6 {
                            '◫'
                        } else if sin_val <= 0.8 {
                            '▯'
                        } else {
                            '○'
                        };
                    //} else {
                    //    '0'
                    //};
                    let _ = outbuf.write(
                        format!("\x1B[38;2;{0};{0};{0}m", if shade+20 > 255 {
                            255
                        } else {
                            shade + 20
                        })
                        .as_bytes()
                        );
                } else if xxhh_yyww <= (hhww * 0.9) && xxhh_yyww > (hhww * 0.8) {
                    draw_char = '$';
                    let _ = outbuf.write(
                        format!("\x1B[38;2;{0};{0};{0}m", if shade < 20 {
                            0
                        } else {
                            shade - 20
                        })
                        .as_bytes()
                        );
                } else {
                    if sin_val <= 0.35 {
                        draw_char = '©';
                    }
                    outbuf.write_all(format!("\x1B[38;2;{0};{0};{0}m",&shade).as_bytes()).unwrap();
                }
                
                if sin_val <= 0.1 {
                    draw_char = '▥';
                    outbuf.write_all(format!("\x1b[38;2;{0};{0};{0}m", &shade).as_bytes()).unwrap();
                }

				let mut outstr = format!(
                    "\x1B[{};{}H{}",
                    ((y_pos + y) as i32),
                    ((x_pos + x) as i32),
					draw_char
				);
                outbuf.write_all(outstr.as_bytes()).unwrap();
                outstr = format!(
                    "\x1B[{};{}H{}",
                    ((y_pos - y) as i32),
                    ((x_pos + x) as i32),
					draw_char
                );
				outbuf.write_all(outstr.as_bytes()).unwrap();
				outstr = format!(
                    "\x1B[{};{}H{}",
                    ((y_pos + y) as i32),
                    ((x_pos - x) as i32),
					draw_char
                );
				outbuf.write_all(outstr.as_bytes()).unwrap();
				outstr = format!(
                    "\x1B[{};{}H{}",
                    ((y_pos - y) as i32),
                    ((x_pos - x) as i32),
					draw_char
                );
				outbuf.write_all(outstr.as_bytes()).unwrap();
            } else {
                let mut outstr = format!(
                    "\x1B[{};{}H ",
                    ((y_pos + y) as i32),
                    ((x_pos + x) as i32),
                );
                outbuf.write_all(outstr.as_bytes()).unwrap();
                outstr = format!(
                    "\x1B[{};{}H ",
                    ((y_pos - y) as i32),
                    ((x_pos + x) as i32),
                );
				outbuf.write_all(outstr.as_bytes()).unwrap();
                outstr = format!(
                    "\x1B[{};{}H ",
                    ((y_pos + y) as i32),
                    ((x_pos - x) as i32),
                );
				outbuf.write_all(outstr.as_bytes()).unwrap();
                outstr = format!(
                    "\x1B[{};{}H ",
                    ((y_pos - y) as i32),
                    ((x_pos - x) as i32),
                );
				outbuf.write_all(outstr.as_bytes()).unwrap();
            }
        }
			
    }
    outbuf.write_all(b"\x1B[48;2;0;0;0m").unwrap();
}

fn main() {
	let outbuf: &mut StdoutLock = &mut io::stdout().lock();

    let args = env::args().collect::<Vec<String>>();
	
	let mut time = 10u128;
	if args.len() >= 2 {
    	time = args[1].trim().parse().unwrap_or(time);
    }
	time *= 1_000_000_000u128;
	

    let HEIGHT = spawn_read(&"tput".to_string(), &["lines".to_string()]);
    let WIDTH = spawn_read(&"tput".to_string(), &["cols".to_string()]);
    let HEIGHT: i32 = HEIGHT.trim().parse().unwrap_or( {
            println!("Choosing default terminal height.");
            24_i32
        }
    );
    let WIDTH: i32 = WIDTH.trim().parse().unwrap_or( {
            println!("Choosing default terminal width.");
            80_i32
        }
    );

    let CENTER = [(WIDTH / 2) as usize, (HEIGHT / 2) as usize];

    let speed_mod = 1.0;

    let mut flipstate: f64 = 0.0;
	let factor = 5_f64;
    let flipspeed: f64 = ((0.00360*factor/100_000.0)/60.0) * speed_mod;

    let mut coin_size = 
    if CENTER[0] >= CENTER[1] {
        ((CENTER[0] - (CENTER[0] % 2)) as f64 / 4.2f64) as usize
    } else {
        ((CENTER[1] - (CENTER[1] % 2)) as f64 / 4.2f64) as usize
    };
	if args.len() >= 3 {
		coin_size = args[2].trim().parse().unwrap_or(coin_size);
	}
	
	for _ in 0..2 {
		outbuf.write_all("\x1B[A\x1B[2K".as_bytes()).unwrap(); // clear `Exit status` lines
	}
	for _ in 0..HEIGHT-1 {
		outbuf.write_all("\n".as_bytes()).unwrap(); 
		// ^ make room for animation without affecting previous output
	}

    hide_cursor();
	
    let running = Arc::new(AtomicBool::new(true));
	let r = running.clone();	
	
	ctrlc::try_set_handler(move || {
		r.store(false, Ordering::SeqCst);
	} ).expect("Error setting ctrl-C handler.");

    let start = Instant::now();
    loop {
		
        draw_coin(CENTER[0], CENTER[1], coin_size, flipstate, outbuf);
        let _ = outbuf.flush();
		
        flipstate = (flipspeed*start.elapsed().as_nanos() as f64) % (std::f64::consts::PI);
		
        if start.elapsed().as_nanos() >= time || !running.load(Ordering::SeqCst) {
            break;
        } else {
			hint::spin_loop();
		}
        
    }

    show_cursor();
}
