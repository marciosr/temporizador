extern crate gtk;
extern crate gio;
extern crate glib;

use std::thread;
use std::cell::RefCell;
use std::rc::Rc;
use gtk::prelude::*;
use gio::prelude::*;
use std::env::args;
use gtk::{Application};

enum Time {
	UpdateTime(f64),
}

fn main() {
	if gtk::init().is_err() {
    	println!("A inicialização do gtk falhou.");
    	return;
	}

	let application = Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
		.expect("failed to initialize GTK application");

	application.connect_activate(|app| {

		let ui_src = include_str!("window.ui");
		let ui = gtk::Builder::new_from_string(ui_src);

		let hours_spinbutton: gtk::SpinButton = ui.get_object("hours_spinbutton").unwrap();
		let hours_adjustment: gtk::Adjustment = ui.get_object("hours_adjustment").unwrap();
		let minutes_spinbutton: gtk::SpinButton = ui.get_object("minutes_spinbutton").unwrap();
		let minutes_adjustment: gtk::Adjustment = ui.get_object("minutes_adjustment").unwrap();
		let seconds_spinbutton: gtk::SpinButton = ui.get_object("seconds_spinbutton").unwrap();
		let seconds_adjustment: gtk::Adjustment = ui.get_object("seconds_adjustment").unwrap();
		let start_button: gtk::Button = ui.get_object("start_button").unwrap();
		let stack: gtk::Stack = ui.get_object("stack").unwrap();
		let stop_button: gtk::Button = ui.get_object("stop_button").unwrap();
		let pause_button: gtk::Button = ui.get_object("pause_button").unwrap();
		let continue_button: gtk::Button = ui.get_object("continue_button").unwrap();

		let window: gtk::ApplicationWindow = ui.get_object("window").unwrap();
		window.set_title("First GTK4 Program");
		window.set_default_size(350, 200);
		app.add_window(&window);

		// Variáveis utilizadas no controle do estado do aplicativo
		let stop = Rc::new(RefCell::new(false));
		let pause = Rc::new(RefCell::new(false));
		let pause_value = Rc::new(RefCell::new(0.0));

		{	// Bloco de iniciar o temporizador

			let hours_adjustment_clone = hours_adjustment.clone();
			let minutes_adjustment_clone = minutes_adjustment.clone();
			let seconds_adjustment_clone = seconds_adjustment.clone();

			let hours_spinbutton_clone = hours_spinbutton.clone();
			let minutes_spinbutton_clone = minutes_spinbutton.clone();
			let seconds_spinbutton_clone = seconds_spinbutton.clone();

			let stack_clone = stack.clone();
			let stop_clone = stop.clone();
			let pause_clone = pause.clone();

			start_button.connect_clicked(move|_| {
				*stop_clone.borrow_mut() = false;
				let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
				let sender_clone = sender.clone();

				let seconds =
					hours_adjustment_clone.get_value() * 3600.0 +
					minutes_adjustment_clone.get_value() * 60.0 +
					seconds_adjustment_clone.get_value();

				println!("***Valor do ajustamento: {}", seconds_adjustment_clone.get_value());
				do_timeout (	seconds,
								&hours_spinbutton_clone,
								&minutes_spinbutton_clone,
								&seconds_spinbutton_clone,
								&stack_clone,
								&sender_clone);

				let hours_adjustment_clone2 = hours_adjustment_clone.clone();
				let minutes_adjustment_clone2 = minutes_adjustment_clone.clone();
				let seconds_adjustment_clone2 = seconds_adjustment_clone.clone();
				let stack_clone2 = stack_clone.clone();
				let stop_clone2 = stop_clone.clone();
				let pause_clone2 = pause_clone.clone();
				let hours_spinbutton_clone2 = hours_spinbutton_clone.clone();
				let minutes_spinbutton_clone2 = minutes_spinbutton_clone.clone();
				let seconds_spinbutton_clone2 = seconds_spinbutton_clone.clone();

				receiver.attach(None, move |msg|{

					do_receiver(msg,
								&hours_adjustment_clone2,
								&minutes_adjustment_clone2,
								&seconds_adjustment_clone2,
								&hours_spinbutton_clone2,
								&minutes_spinbutton_clone2,
								&seconds_spinbutton_clone2,
								&stack_clone2,
								&stop_clone2,
								&pause_clone2)
				});
			});
		}

		{ // Bloco que implementa a ação de parar o temporizador

			let stack_clone2 = stack.clone();
			let hours_adjustment_clone = hours_adjustment.clone();
			let minutes_adjustment_clone = minutes_adjustment.clone();
			let seconds_adjustment_clone = seconds_adjustment.clone();
			let hours_spinbutton_clone = hours_spinbutton.clone();
			let minutes_spinbutton_clone = minutes_spinbutton.clone();
			let seconds_spinbutton_clone = seconds_spinbutton.clone();
			let stop_clone = stop.clone();

			stop_button.connect_clicked(move|_| {
				stack_clone2.set_visible_child_name("start");

				hours_adjustment_clone.set_value(0.0);
				minutes_adjustment_clone.set_value(0.0);
				seconds_adjustment_clone.set_value(0.0);

				hours_spinbutton_clone.set_sensitive(true);
				minutes_spinbutton_clone.set_sensitive(true);
				seconds_spinbutton_clone.set_sensitive(true);
				println!("STOP PLEASE!");
				*stop_clone.borrow_mut() = true;
			});
		}

		{ // Bloco de pausa

			let stack_clone = stack.clone();
			let hours_adjustment_clone = hours_adjustment.clone();
			let minutes_adjustment_clone = minutes_adjustment.clone();
			let seconds_adjustment_clone = seconds_adjustment.clone();
			let stop_clone = stop.clone();
			let pause_clone = pause.clone();
			let pause_value_clone = pause_value.clone();

			pause_button.connect_clicked(move|_| {

				// Alterna para a página de continue do gtk_stack
				stack_clone.set_visible_child_name("continue");

				// Recupera o valor atual do tempo
				let seconds =
					hours_adjustment_clone.get_value() * 3600.0 +
					minutes_adjustment_clone.get_value() * 60.0 +
					seconds_adjustment_clone.get_value();

				*pause_value_clone.borrow_mut() = seconds;
				*pause_clone.borrow_mut() = true;
				*stop_clone.borrow_mut() = true; // Altera o estado para parar o receiver
				println!("O valor do pause_clone dentro do callback do pause_button é: {:?}", pause_value_clone);
				println!("O valor do pause_clone dentro do callback do pause_button é: {}", *pause_value_clone.borrow());
			});
		}

		{ // Bloco que implementa a continuação

			let hours_adjustment_clone = hours_adjustment.clone();
			let minutes_adjustment_clone = minutes_adjustment.clone();
			let seconds_adjustment_clone = seconds_adjustment.clone();
			let hours_spinbutton_clone = hours_spinbutton.clone();
			let minutes_spinbutton_clone = minutes_spinbutton.clone();
			let seconds_spinbutton_clone = seconds_spinbutton.clone();
			let stack_clone = stack.clone();
			let stop_clone = stop.clone();
			let pause_clone = pause.clone();

			continue_button.connect_clicked(move|_| {

				*stop_clone.borrow_mut() = false;
				*pause_clone.borrow_mut() = false;
				let (sender_p, receiver_p) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
				let sender_clone = sender_p.clone();
				let seconds = *pause_value.borrow();

				do_timeout (seconds,
							&hours_spinbutton_clone,
							&minutes_spinbutton_clone,
							&seconds_spinbutton_clone,
							&stack_clone,
							&sender_clone);

				let hours_adjustment_clone2 = hours_adjustment_clone.clone();
				let minutes_adjustment_clone2 = minutes_adjustment_clone.clone();
				let seconds_adjustment_clone2 = seconds_adjustment_clone.clone();
				let stack_clone2 = stack_clone.clone();
				let stop_clone2 = stop_clone.clone();
				let pause_clone2 = pause_clone.clone();
				let hours_spinbutton_clone2 = hours_spinbutton_clone.clone();
				let minutes_spinbutton_clone2 = minutes_spinbutton_clone.clone();
				let seconds_spinbutton_clone2 = seconds_spinbutton_clone.clone();

				receiver_p.attach(None, move |msg|{
					do_receiver(msg,
								&hours_adjustment_clone2,
								&minutes_adjustment_clone2,
								&seconds_adjustment_clone2,
								&hours_spinbutton_clone2,
								&minutes_spinbutton_clone2,
								&seconds_spinbutton_clone2,
								&stack_clone2,
								&stop_clone2,
								&pause_clone2)
				});
			});
		}

		window.show();
	});
	application.run(&args().collect::<Vec<_>>());
}

fn do_timeout (	mut seconds: 			f64,
				hours_spinbutton:		&gtk::SpinButton,
				minutes_spinbutton:		&gtk::SpinButton,
				seconds_spinbutton:		&gtk::SpinButton,
				stack:					&gtk::Stack,
				sender:					&glib::Sender<Time>) {

	if seconds > 0.0 {

		stack.set_visible_child_name("pause_stop");
		hours_spinbutton.set_sensitive(false);
		minutes_spinbutton.set_sensitive(false);
		seconds_spinbutton.set_sensitive(false);

		let sender_clone = sender.clone();

		thread::spawn(move || {
			glib::timeout_add_seconds(1, move||  {
				if seconds > 0.0 {
					seconds = seconds - 1.0;
				}

				let sender_result = sender_clone.send(Time::UpdateTime(seconds));

				//match sender_clone.send(Time::UpdateTime(seconds)) {
				match sender_result {
					Ok(_) => {
						if seconds > 0.0 {
							glib::Continue(true)
						} else {
							glib::Continue(false)
						}
					},
					Err(_) => glib::Continue(false),
				}
			});
		});
	}
}

fn do_receiver (msg: Time,
				hours_adjustment:		&gtk::Adjustment,
				minutes_adjustment:		&gtk::Adjustment,
				seconds_adjustment:		&gtk::Adjustment,
				hours_spinbutton:		&gtk::SpinButton,
				minutes_spinbutton:		&gtk::SpinButton,
				seconds_spinbutton:		&gtk::SpinButton,
				stack:					&gtk::Stack,
				stop:					&Rc<RefCell<bool>>,
				pause:					&Rc<RefCell<bool>>,
				) -> glib::Continue {


	let padrao = Rc::new(RefCell::new(true));
	if *stop == padrao {

		if *pause != padrao {

			hours_adjustment.set_value(0.0);
			minutes_adjustment.set_value(0.0);
			seconds_adjustment.set_value(0.0);
			stack.set_visible_child_name("start");
			hours_spinbutton.set_sensitive(true);
			minutes_spinbutton.set_sensitive(true);
			seconds_spinbutton.set_sensitive(true);

		} else {
			stack.set_visible_child_name("continue");
		}
		glib::Continue(false)
	} else {
		match msg {
			Time::UpdateTime(secs) => {
				let hours = secs as i32 /3600;
				let minutes = (secs as i32 % 3600) / 60;
				let seconds = (secs as i32 % 3600) % 60;
				hours_adjustment.set_value(hours as f64);
				minutes_adjustment.set_value(minutes as f64);
				seconds_adjustment.set_value(seconds as f64);

				if secs == 0.0 {
					stack.set_visible_child_name("start");
					hours_spinbutton.set_sensitive(true);
					minutes_spinbutton.set_sensitive(true);
					seconds_spinbutton.set_sensitive(true);
				}
			}
		}
		glib::Continue(true)
	}
}
