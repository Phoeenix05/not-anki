mod event;
mod state;

use std::sync::Arc;

use druid::widget::{Button, Flex, Label, List, Scroll};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};
use event::LoggedEvent;
use state::AppState;

fn main() -> Result<(), PlatformError> {
    let window = WindowDesc::new(ui_builder());
    let data = AppState {
        counter: 0,
        events: Arc::new(Vec::new()),
    };
    AppLauncher::with_window(window)
        .log_to_console()
        .launch(data)
}

fn ui_builder() -> impl Widget<AppState> {
    let text = LocalizedString::new("hello-counter")
        .with_arg("count", |data: &AppState, _env| (*data).counter.into());
    let label = Label::new(text).padding(5.0).center();
    let button = Button::new("increment")
        .on_click(|_ctx, data: &mut AppState, _env| (*data).counter += 1)
        .padding(5.0);

    Flex::column()
        .with_child(label)
        .with_child(button)
        .with_flex_child(event_list_builder(), 1.0)
    // .with_flex_child(
    //     Scroll::new(List::new(event_list_builder).lens(AppState::events)).vertical(),
    //     1.0,
    // )
}

fn event_list_builder() -> impl Widget<AppState> {
    Scroll::new(Flex::column().with_flex_child(
        Scroll::new(List::new(make_list_item).lens(AppState::events)).vertical(),
        1.0,
    ))
}

fn make_list_item() -> impl Widget<LoggedEvent> {
    Flex::row().with_child(Label::dynamic(|d: &LoggedEvent, _| d.key()).fix_width(60.0))
}
