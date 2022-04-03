extern crate gtk;

use std::rc::Rc;
use gtk::prelude::*;

pub struct Timer {
	pub window:					gtk::ApplicationWindow,
	pub hours_spinbutton:		gtk::SpinButton,
	pub minutes_spinbutton:		gtk::SpinButton,
	pub seconds_spinbutton:		gtk::SpinButton,
	pub hours_adjustment:		gtk::Adjustment,
	pub minutes_adjustment:		gtk::Adjustment,
	pub seconds_adjustment:		gtk::Adjustment,
	pub start_button:			gtk::Button,
	pub stop_button:			gtk::Button,
	pub pause_button:			gtk::Button,
	pub continue_button:		gtk::Button,
	pub stack:					gtk::Stack,
}

impl Timer {
	pub fn new(ui: &gtk::Builder) -> Rc<Self> {

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

		window.set_title(Some("Temporizador"));
		window.set_default_size(350, 200);


		// window.quit_activate(clone!(@strong window => move |_|{
		// 	window.destroy();
		// }));

		let timer = Rc::new(Self {
			window,
			hours_spinbutton,
			minutes_spinbutton,
			seconds_spinbutton,
			hours_adjustment,
			minutes_adjustment,
			seconds_adjustment,
			start_button,
			stop_button,
			pause_button,
			continue_button,
			stack,
		});

		timer
	}
}
