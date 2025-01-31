mod api;
mod repository;

use std::thread;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use api::auth::login::login;
use api::auth::signup::signup;
use api::auth::verify::confirm::confirm;
use api::auth::verify::send::send;
use image::{GenericImageView, ImageError};
use minifb::{Window, WindowOptions};
use rand::prelude::IndexedRandom;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv::dotenv().ok();

	std::env::set_var("RUST_LOG", "debug");
	std::env::set_var("RUST_BACKTRACE", "1");
	env_logger::init();

	repository::database::create_user_db().await.unwrap();

	let washee_washee = false;
	if washee_washee {
		thread::spawn(move || {
			if let Err(e) = motivation() {
				eprintln!("Window error: {:?}", e);
			}
		});
	}

	HttpServer::new(move || {
		App::new().wrap(Logger::default()).service(
			web::scope("/auth")
				.service(login)
				.service(signup)
				.service(web::scope("/verify").service(send).service(confirm)),
		)
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}

const PHRASES: &[&str] = &[
	"You become doctor NOW!",
	"No game until doctor",
	"You no type fast enough",
	"You no fix link.exe, you failure",
	"For each time you use ai I slap you 3 time",
	"Each error you pay 2 cent",
	"You finger no bleed, you no try hard enough",
];

fn motivation() -> Result<(), ImageError> {
	let args: Vec<String> = std::env::args().collect();
	let image_path = args
		.get(1)
		.map(|s| s.as_str())
		.unwrap_or("Mr._Washee_Washee.jpg");

	let mut img = image::open(image_path).map_err(|e| {
		eprintln!("Failed to load image: {}", image_path);
		e
	})?;

	let (mut width, mut height) = img.dimensions();

	if width < 100 {
		let scale_factor = 100.0 / width as f32;
		width = 100;
		height = (height as f32 * scale_factor).round() as u32;
		img = img.resize(width, height, image::imageops::FilterType::Lanczos3);
	}

	let img = img.into_rgba8();
	let (width, height) = img.dimensions();

	let mut buffer = Vec::with_capacity((width * height) as usize);
	for pixel in img.pixels() {
		let r = pixel[0] as u32;
		let g = pixel[1] as u32;
		let b = pixel[2] as u32;
		buffer.push((r << 16) | (g << 8) | b);
	}

	let opts = WindowOptions {
		topmost: true,
		borderless: false,
		..WindowOptions::default()
	};

	let mut rng = rand::rng();

	loop {
		let title = PHRASES.choose(&mut rng).unwrap();

		let mut window = match Window::new(title, width as usize * 2usize, height as usize, opts) {
			| Ok(win) => win,
			| Err(e) => {
				eprintln!("Window creation failed: {:?}", e);
				continue;
			},
		};

		window.set_target_fps(10);

		while window.is_open() {
			if let Err(e) = window.update_with_buffer(&buffer, width as usize, height as usize) {
				eprintln!("Window update error: {:?}", e);
			}
		}
	}
}
