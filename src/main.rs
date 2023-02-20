extern crate gtk;
use gtk::prelude::*;
use gtk::Application;

#[macro_use]
mod utils;
mod core;
mod timer;

use crate::timer::Timer;
use crate::core::*;
use crate::gtk::glib::{MainContext, PRIORITY_DEFAULT, Continue};
use gtk::glib;
use glib::clone;

fn main() -> glib::ExitCode {

	let application = Application::new(Some("com.github.marciosr.temporizador"),
		Default::default());

	application.connect_startup(move|app| {
		let ui_src = include_str!("window.ui");
		let ui = gtk::Builder::new();
		ui.add_from_string(&ui_src)
			.expect("Erro ao abrir arquivo da interface");

		let timer = Timer::new(&ui);

		app.add_window(&timer.window);

		let core = Core::new();

		{
			let core_clone = core.clone();

			timer.start_button.connect_clicked(clone!(@strong timer => move|_| {
				println!("Teste!");
				*core_clone.stop.borrow_mut() = false;
				let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);
				let sender_clone = sender.clone();

				do_timeout (&timer, &sender_clone);

				receiver.attach(None, clone!(
					@weak timer,
					@strong core_clone
					=> @default-return Continue(true), move |msg| {
						do_receiver(msg, &timer, &core_clone)

				}));
			}));
		}

		{
			let core_clone = core.clone();

			timer.continue_button.connect_clicked(clone!(
				@strong timer => move|_| {
					println!("Teste!");


					let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);
					let sender_clone = sender.clone();

					do_timeout (&timer, &sender_clone);
					*core_clone.stop.borrow_mut() = false;
					*core_clone.pause.borrow_mut() = false;
					receiver.attach(None, clone!(
						@weak timer,
						@strong core_clone
						=> @default-return Continue(true), move |msg| {
							do_receiver(msg, &timer, &core_clone)

					}));
			}));
		}

		{ // Bloco que implementa a ação de parar o temporizador
			let core_clone = core.clone();
			timer.stop_button.connect_clicked(clone!( @strong timer => move|_| {
				*core_clone.stop.borrow_mut() = true;
				timer.stack.set_visible_child_name("start");

				timer.hours_adjustment.set_value(0.0);
				timer.minutes_adjustment.set_value(0.0);
				timer.seconds_adjustment.set_value(0.0);

				timer.hours_spinbutton.set_sensitive(true);
				timer.minutes_spinbutton.set_sensitive(true);
				timer.seconds_spinbutton.set_sensitive(true);
				println!("STOP PLEASE!");

			}));
		}

		{ // Bloco de pausa
			let core_clone = core.clone();
			timer.pause_button.connect_clicked(clone!(@weak timer => move|_| {

				// Alterna para a página de continue do gtk_stack
				timer.stack.set_visible_child_name("continue");

				// Recupera o valor atual do tempo
				let seconds =
					timer.hours_adjustment.value() * 3600.0 +
					timer.minutes_adjustment.value() * 60.0 +
					timer.seconds_adjustment.value();

				*core_clone.pause_value.borrow_mut() = seconds;
				*core_clone.pause.borrow_mut() = true;
				*core_clone.stop.borrow_mut() = true; // Altera o estado para parar o receiver
				println!(
					"O valor do pause_clone dentro do callback do pause_button é: {:?}",
					core_clone.pause_value
				);
				println!(
					"O valor do pause_clone dentro do callback do pause_button é: {}",
					*core_clone.pause_value.borrow()
				);
			}));
		}

		timer.window.show();
	});
	application.run()
}
