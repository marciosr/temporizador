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
use glib::{clone, Sender, Receiver};

macro_rules! get_widget {
    ($builder:expr, $wtype:ty, $name:ident) => {
        let $name: $wtype = $builder.get_object(stringify!($name)).expect(&format!("Could not find widget \"{}\"", stringify!($name)));
    };
}

enum Time {
	UpdateTime(f64),
}

fn main() {
	if gtk::init().is_err() {
    	println!("A inicialização do gtk falhou.");
    	return;
	}

	let application = Application::new(Some("com.github.marciosr.temporizador"), Default::default())
		.expect("failed to initialize GTK application");

	application.connect_activate(|app| {

		let ui_src = include_str!("window.ui");
		let ui = gtk::Builder::new_from_string(ui_src);
		get_widget!(ui, gtk::SpinButton, hours_spinbutton);
		get_widget!(ui, gtk::SpinButton, minutes_spinbutton);
		get_widget!(ui, gtk::SpinButton, seconds_spinbutton);
		get_widget!(ui, gtk::Adjustment, hours_adjustment);
		get_widget!(ui, gtk::Adjustment, minutes_adjustment);
		get_widget!(ui, gtk::Adjustment, seconds_adjustment);
		get_widget!(ui, gtk::Button, start_button);
		get_widget!(ui, gtk::Button, stop_button);
		get_widget!(ui, gtk::Button, pause_button);
		get_widget!(ui, gtk::Button, continue_button);
		get_widget!(ui, gtk::Stack, stack);
		get_widget!(ui, gtk::ApplicationWindow, window);

		window.set_title("Temporizador");
		window.set_default_size(350, 200);
		app.add_window(&window);

		// Variáveis utilizadas no controle do estado do aplicativo
		let stop = Rc::new(RefCell::new(false));
		let pause = Rc::new(RefCell::new(false));
		let pause_value = Rc::new(RefCell::new(0.0));

		{	// Bloco de iniciar o temporizador

			start_button.connect_clicked(clone!(@weak hours_adjustment as hours_adjustment_clone,
												@weak minutes_adjustment as minutes_adjustment_clone,
												@weak seconds_adjustment as seconds_adjustment_clone,
												@weak stack as stack_clone,
												@weak hours_spinbutton as hours_spinbutton_clone,
												@weak minutes_spinbutton as minutes_spinbutton_clone,
												@weak seconds_spinbutton as seconds_spinbutton_clone,
												@strong stop as stop_clone,
												@strong pause as pause_clone => move|_| {

				*stop_clone.borrow_mut() = false;
				let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
				let sender_clone = sender.clone();

				let seconds =
					hours_adjustment_clone.get_value() * 3600.0 +
					minutes_adjustment_clone.get_value() * 60.0 +
					seconds_adjustment_clone.get_value();

				do_timeout (seconds,
							&hours_spinbutton_clone,
							&minutes_spinbutton_clone,
							&seconds_spinbutton_clone,
							&stack_clone,
							&sender_clone);

				receiver.attach(None,clone!(@weak hours_adjustment_clone,
											@weak minutes_adjustment_clone,
			                                @weak seconds_adjustment_clone,
			                                @weak stack_clone,
											@weak hours_spinbutton_clone,
											@weak minutes_spinbutton_clone,
											@weak seconds_spinbutton_clone,
											@strong stop_clone,
											@strong pause_clone => @default-return glib::Continue(true),  move |msg|{

					do_receiver(msg,
								&hours_adjustment_clone,
								&minutes_adjustment_clone,
								&seconds_adjustment_clone,
								&hours_spinbutton_clone,
								&minutes_spinbutton_clone,
								&seconds_spinbutton_clone,
								&stack_clone,
								&stop_clone,
								&pause_clone)
				}));
			}));
		}

		{ // Bloco que implementa a ação de parar o temporizador

			stop_button.connect_clicked(clone!( @weak hours_adjustment,
												@weak minutes_adjustment,
			                                    @weak stop,
			                                    @weak seconds_adjustment,
			                                    @weak stack,
			                                    @weak hours_spinbutton,
			                                    @weak minutes_spinbutton,
			                                    @weak seconds_spinbutton => move|_| {

				stack.set_visible_child_name("start");

				hours_adjustment.set_value(0.0);
				minutes_adjustment.set_value(0.0);
				seconds_adjustment.set_value(0.0);

				hours_spinbutton.set_sensitive(true);
				minutes_spinbutton.set_sensitive(true);
				seconds_spinbutton.set_sensitive(true);
				println!("STOP PLEASE!");
				*stop.borrow_mut() = true;
			}));
		}

		{ // Bloco de pausa

			pause_button.connect_clicked(clone!(@weak hours_adjustment,
												@weak minutes_adjustment,
			                                    @weak stop,
			                                    @weak seconds_adjustment,
			                                    @weak stack,
			                                    @weak pause_value,
			                                    @weak pause => move|_| {

				// Alterna para a página de continue do gtk_stack
				stack.set_visible_child_name("continue");

				// Recupera o valor atual do tempo
				let seconds =
					hours_adjustment.get_value() * 3600.0 +
					minutes_adjustment.get_value() * 60.0 +
					seconds_adjustment.get_value();

				*pause_value.borrow_mut() = seconds;
				*pause.borrow_mut() = true;
				*stop.borrow_mut() = true; // Altera o estado para parar o receiver
				println!("O valor do pause_clone dentro do callback do pause_button é: {:?}", pause_value);
				println!("O valor do pause_clone dentro do callback do pause_button é: {}", *pause_value.borrow());
			}));
		}

		{ // Bloco que implementa a continuação

			continue_button.connect_clicked(clone!( @weak hours_adjustment, @weak minutes_adjustment,
													@weak stop, @weak seconds_adjustment, @weak stack,
													@weak hours_spinbutton, @weak minutes_spinbutton,
													@weak seconds_spinbutton,@weak pause => move|_| {

				*stop.borrow_mut() = false;
				*pause.borrow_mut() = false;
				let (sender_p, receiver_p) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
				let sender_clone = sender_p.clone();
				let seconds = *pause_value.borrow();

				do_timeout (seconds,
							&hours_spinbutton,
							&minutes_spinbutton,
							&seconds_spinbutton,
							&stack,
							&sender_clone);

				receiver_p.attach(None,clone!( @weak hours_adjustment, @weak minutes_adjustment,
			                                    @weak stop, @weak seconds_adjustment, @weak stack,
			                                    @weak hours_spinbutton, @weak minutes_spinbutton,
			                                    @weak seconds_spinbutton,@weak pause => @default-return glib::Continue(false), move |msg|{
					do_receiver(msg,
								&hours_adjustment,
								&minutes_adjustment,
								&seconds_adjustment,
								&hours_spinbutton,
								&minutes_spinbutton,
								&seconds_spinbutton,
								&stack,
								&stop,
								&pause)
				}));
			}));
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

