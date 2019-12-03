extern crate gtk;
extern crate gio;
extern crate glib;

use std::thread;
//use std::cell::Cell;
use std::cell::RefCell;
//use std::cell::RefMut;
use std::rc::Rc;

use gtk::prelude::*;
use gio::prelude::*;
//use glib::prelude::*;

use std::env::args;

use gtk::{Application}; //, ApplicationWindow, Button, SpinButton, Stack, StackPage};

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
		let _pause_button: gtk::Button = ui.get_object("pause_button").unwrap();

		let window: gtk::ApplicationWindow = ui.get_object("window").unwrap();
		window.set_title("First GTK4 Program");
		window.set_default_size(350, 200);
		app.add_window(&window);

		enum Time {
			UpdateTime(f64),
		}
		let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
		{
			let sender_clone = sender.clone();

			let stack_clone = stack.clone();
			let hours_adjustment_clone = hours_adjustment.clone();
			let minutes_adjustment_clone = minutes_adjustment.clone();
			let seconds_adjustment_clone = seconds_adjustment.clone();

			let hours_spinbutton_clone = hours_spinbutton.clone();
			let minutes_spinbutton_clone = minutes_spinbutton.clone();
			let seconds_spinbutton_clone = seconds_spinbutton.clone();

			start_button.connect_clicked(move|_| {
				println!("Clicked!");
				stack_clone.set_visible_child_name("pause_stop");

				hours_spinbutton_clone.set_sensitive(false);
				minutes_spinbutton_clone.set_sensitive(false);
				seconds_spinbutton_clone.set_sensitive(false);

				let sender_clone2 = sender_clone.clone();
				let mut seconds =
					hours_adjustment_clone.get_value() * 3600.0 +
					minutes_adjustment_clone.get_value() * 60.0 +
					seconds_adjustment_clone.get_value();

				thread::spawn(move || {
					glib::timeout_add_seconds(1, move||  {
						if seconds > 0.0 {
							seconds = seconds - 1.0;
						}
						let _ = sender_clone2.send(Time::UpdateTime(seconds));

						if seconds > 0.0 {
							glib::Continue(true)
						} else {
							glib::Continue(false)
						}

					});
				});
			});
		}

		let stop = Rc::new(RefCell::new(false));

		{
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
				//let mut s = stop.borrow_mut();
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

		{
			let hours_adjustment_clone = hours_adjustment.clone();
			let minutes_adjustment_clone = minutes_adjustment.clone();
			let seconds_adjustment_clone = seconds_adjustment.clone();
			let stack_clone2 = stack.clone();

			let hours_spinbutton_clone = hours_spinbutton.clone();
			let minutes_spinbutton_clone = minutes_spinbutton.clone();
			let seconds_spinbutton_clone = seconds_spinbutton.clone();
			//let stop_clone = stop.clone();

			receiver.attach(None, move |msg| {
				match msg {
					Time::UpdateTime(secs) => {
						//println!("O valor de secs no receiver é: {}", secs);
						let hours = secs as i32 /3600;
						//println!("O valor de hours no receiver é: {}", hours);
						let minutes = (secs as i32 % 3600) / 60;
						//println!("O valor de minutes no receiver é: {}", minutes);
						let seconds = (secs as i32 % 3600) % 60;
						//println!("O valor de seconds no receiver é: {}", seconds);
						hours_adjustment_clone.set_value(hours as f64);
						minutes_adjustment_clone.set_value(minutes as f64);
						seconds_adjustment_clone.set_value(seconds as f64);
						if secs == 0.0 {
							stack_clone2.set_visible_child_name("start");
							hours_spinbutton_clone.set_sensitive(true);
							minutes_spinbutton_clone.set_sensitive(true);
							seconds_spinbutton_clone.set_sensitive(true);
						}

					}
				}

				let teste = RefCell::new(true);
				if *stop == teste {
					glib::Continue(false)
				} else {
					glib::Continue(true)
				}
			});
		}

		window.show();
	});

	application.run(&args().collect::<Vec<_>>());
}
