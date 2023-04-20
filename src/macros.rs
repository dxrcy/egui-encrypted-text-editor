/// Check keyboard input on UI
macro_rules! keys {
    ( $ui: ident : $key: ident ) => {
        $ui.input_mut(|i| i.consume_key(::eframe::egui::Modifiers::NONE, ::eframe::egui::Key::$key))
    };

    ( $ui: ident : $mod_1: ident + $key: ident ) => {
        $ui.input_mut(|i| {
            i.consume_key(::eframe::egui::Modifiers::$mod_1, ::eframe::egui::Key::$key)
        })
    };

    ( $ui: ident : $mod_1: ident + $mod_2: ident + $key: ident ) => {
        $ui.input_mut(|i| {
            i.consume_key(
                ::eframe::egui::Modifiers::$mod_1 | ::eframe::egui::Modifiers::$mod_2,
                ::eframe::egui::Key::$key,
            )
        })
    };
}
