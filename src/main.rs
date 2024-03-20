use gpui::*;
use assets::Assets;
use settings::{default_settings, SettingsStore};
use theme::{ThemeRegistry, ThemeSettings};
use seetings::Settings;

struct HelloWorld {
    text: SharedString,
}
 
impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
    }
}
 
fn main() {
    App::new().run(|cx: &mut AppContext| {
        
        let theme_name = "One Dark";
        
        let mut store = SettingsStore::default();
        store.set_default_settings(default_settings().as_ref(), cx)
            .unwrap();
        cx.set_global(store);
        
        theme::init(themes_to_load: theme::LoadThemes::All(Box::new(Assets)), cx);

        let theme_registry :&ThemeRegistry = cx.global::<ThemeRegistry>();
        let mut theme_settings: ThemeSettings = ThemeSettings::get_global(cx).clone();
        theme_settings.active_theme = theme_registry.get(&theme_name).unwrap();
        ThemeSettings::override_global(theme_settings, cx);

        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| HelloWorld {
                text: "World".into(),
            })
        });
    });
}
