//extern crate gtk;
//extern crate gio;
//extern crate glib;

use std::thread;
use std::cell::RefCell;
use std::rc::Rc;
use gtk::prelude::*;

use crate::timer::Timer;
use crate::gtk::glib::{Sender, Continue, timeout_add_seconds};


pub enum Time {
	UpdateTime(f64),
}

pub struct Core {
	pub stop:			Rc<RefCell<bool>>,
	pub pause:			Rc<RefCell<bool>>,
	pub pause_value:	Rc<RefCell<f64>>,
}

impl Core {
	pub fn new() -> Rc<Self> {

		let stop = Rc::new(RefCell::new(false));
		let pause = Rc::new(RefCell::new(false));
		let pause_value = Rc::new(RefCell::new(0.0));

		let core = Rc::new(Self {
			stop,
			pause,
			pause_value,
		});

		core
	}
}

pub fn do_timeout (	timer:		&Timer,
					sender:		&Sender<Time>) {
	let mut seconds =
		timer.hours_adjustment.value() * 3600.0 +
		timer.minutes_adjustment.value() * 60.0 +
		timer.seconds_adjustment.value();

	if seconds > 0.0 {
		timer.stack.set_visible_child_name("pause_stop");
		timer.hours_spinbutton.set_sensitive(false);
		timer.minutes_spinbutton.set_sensitive(false);
		timer.seconds_spinbutton.set_sensitive(false);

		let sender_clone = sender.clone();

		thread::spawn(move || {
			timeout_add_seconds(1, move||  {
				if seconds > 0.0 {
					seconds = seconds - 1.0;
				}

				let sender_result = sender_clone.send(Time::UpdateTime(seconds));

				//match sender_clone.send(Time::UpdateTime(seconds)) {
				match sender_result {
					Ok(_) => {
						if seconds > 0.0 {
							Continue(true)
						} else {
							Continue(false)
						}
					},
					Err(_) => Continue(false),
				}
			});
		});
	}
}

pub fn do_receiver (msg: 		Time,
										timer:		&Timer,
										core:		&Rc<Core>
										) ->		Continue {


	let padrao = Rc::new(RefCell::new(true));
	if core.stop == padrao {

		if core.pause != padrao {

			timer.hours_adjustment.set_value(0.0);
			timer.minutes_adjustment.set_value(0.0);
			timer.seconds_adjustment.set_value(0.0);
			timer.stack.set_visible_child_name("start");
			timer.hours_spinbutton.set_sensitive(true);
			timer.minutes_spinbutton.set_sensitive(true);
			timer.seconds_spinbutton.set_sensitive(true);

		} else {
			timer.stack.set_visible_child_name("continue");
		}
		Continue(false)
	} else {
		match msg {
			Time::UpdateTime(secs) => {
				let hours = secs as i32 /3600;
				let minutes = (secs as i32 % 3600) / 60;
				let seconds = (secs as i32 % 3600) % 60;
				timer.hours_adjustment.set_value(hours as f64);
				timer.minutes_adjustment.set_value(minutes as f64);
				timer.seconds_adjustment.set_value(seconds as f64);

				if secs == 0.0 {
					timer.stack.set_visible_child_name("start");
					timer.hours_spinbutton.set_sensitive(true);
					timer.minutes_spinbutton.set_sensitive(true);
					timer.seconds_spinbutton.set_sensitive(true);
				}
			}
		}
		Continue(true)
	}
}
