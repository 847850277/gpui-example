use std::fmt::format;

use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, SharedString, TextAlign, Window, WindowBounds, WindowOptions
};

#[derive(Debug)]
#[allow(dead_code)]
struct HelloWorld {
    text: SharedString,
}

pub struct DbPannel {}

impl Render for DbPannel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        div()
        .bg(rgb(0xffffff))
        .size(px(800.0))
        //.text_color(rgb(0x0000ff))
        .child(
            div().h_2()
            .text_align(TextAlign::Center)
            .text_size(px(30.0))
            .child("Database Connection".to_string())
            )
            
        .child(
            div()
            .child(
                div()
                .text_align(TextAlign::Left)
                .text_size(px(20.0))
                .child("System".to_string())
                )
        )
        .child(
            div()
            .child(
                div()
                .text_align(TextAlign::Left)
                .text_size(px(20.0))
                .child("Server".to_string())
                )
        )
        .child(
            div()
            .child(
                div()
                .text_align(TextAlign::Left)
                .text_size(px(20.0))
                .child("Username".to_string())
                )
        )
        .child(
            div()
            .child(
                div()
                .text_align(TextAlign::Left)
                .text_size(px(20.0))
                .child("Password".to_string())
                )
        )
        .child(
            div()
            .child(
                div()
                .text_align(TextAlign::Left)
                .text_size(px(20.0))
                .child("Database".to_string())
                )
        )
        .child(
            div()
            .child(
                div()
                .text_align(TextAlign::Center)
                .text_size(px(20.0))
                .child("Login".to_string())
                )
        )
    }
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x505050))
            .size(px(800.0))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .border_color(rgb(0x0000ff))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| DbPannel {
                    //text: "World".into(),
                })
            },
        )
        .unwrap();
    });
}
