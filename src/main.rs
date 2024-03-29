use gpui::*;
use assets::Assets;
use settings::{default_settings, SettingsStore};
use theme::{ThemeRegistry, ThemeSettings, LoadThemes, ActiveTheme};
use settings::Settings;
use ui::prelude::*;
use ui::Button;

struct HelloWorld {
    text: SharedString,
    count: i32
}
 
impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let button = Button::new("test-id", "Click Me")
        .style(ui::ButtonStyle::Filled)
        .size(ui::ButtonSize::Large)
        .on_click(_cx.listener(move |this: &mut HelloWorld, selection: &ClickEvent, cx: &mut ViewContext<'_, HelloWorld>| {
            this.count += 1;
        }));

        div()
            .flex()
            .flex_col()
            .bg(_cx.theme().colors().background)
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(_cx.theme().colors().text)
            .child(format!("Hello, {}!", &self.count))
            .child(button)
    }
}
 
fn main() {
    App::new().run(|cx: &mut AppContext| {
        
        let theme_name = "One Dark";
        
        let mut store = SettingsStore::default();
        store.set_default_settings(default_settings().as_ref(), cx)
            .unwrap();
        cx.set_global(store);
        
        theme::init(LoadThemes::All(Box::new(Assets)), cx );

        let theme_registry  = ThemeRegistry::global(cx);
        let mut theme_settings = ThemeSettings::get_global(cx).clone();
        theme_settings.active_theme = theme_registry.get(&theme_name).unwrap();

        let theme_names: Vec<SharedString> = theme_registry.list_names(true);
        println!("theme_names: {:?}", theme_names);
        ThemeSettings::override_global(theme_settings, cx);
        

        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| HelloWorld {
                text: "World".into(),
                count: 0,
            })
        });
    });
}
