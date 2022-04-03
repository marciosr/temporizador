macro_rules! get_widget {
    ($builder:expr, $wtype:ty, $name:ident) => {
        let $name: $wtype = $builder.object(stringify!($name)).expect(&format!("Could not find widget \"{}\"", stringify!($name)));
    };
}
