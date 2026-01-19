use chrono::{Local, TimeZone};
use gpui::*;
use gpui_component::{
    input::{Input, InputEvent, InputState},
    *,
};
use std::time::{SystemTime, UNIX_EPOCH};

const BG_COLOR: u32 = 0x1f1e21;

pub struct HelloWorld {
    logs: Entity<Vec<(u128, SharedString)>>,
    input: Entity<InputState>,
    _input_subscription: Subscription,
}

impl HelloWorld {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| InputState::new(window, cx));
        let logs = cx.new(|_| Vec::new());

        let logs_handle = logs.clone();
        let input_subscription = cx.subscribe_in(
            &input,
            window,
            move |_view, input, event, window, cx| match event {
                InputEvent::PressEnter { secondary } => {
                    println!("Enter pressed, secondary: {}", secondary);
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis();

                    let text = input.read(cx).value();

                    logs_handle.update(cx, |logs, _| {
                        logs.push((now, text.clone()));
                    });

                    input.update(cx, |input, cx| {
                        input.set_value(String::new(), window, cx);
                    });
                }
                _ => {}
            },
        );

        Self {
            logs,
            input,
            _input_subscription: input_subscription,
        }
    }
}

impl Render for HelloWorld {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let logs = self.logs.read(cx);

        let log_view = logs.iter().map(|(ts, msg)| {
            let time_str = Local
                .timestamp_millis_opt(*ts as i64)
                .unwrap()
                .format("%H:%M:%S")
                .to_string();
            div()
                .h_4()
                .h_flex()
                .gap_2()
                .child(time_str)
                .child(msg.clone())
        });

        div()
            .v_flex()
            .gap_2()
            .p_4()
            .size_full()
            .font_family("Consolas")
            .bg(rgb(BG_COLOR))
            .children(log_view)
            .child(
                div()
                    .h_flex()
                    .h_4()
                    .items_center()
                    .child(Local::now().format("%H:%M:%S").to_string())
                    .child(
                        div().w_full().child(
                            Input::new(&self.input)
                                .appearance(false)
                                .font_family("Consolas")
                                .bordered(false),
                        ),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx| {
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|cx| HelloWorld::new(window, cx));
                cx.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
