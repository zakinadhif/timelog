use chrono::{Local, TimeZone};
use gpui::{Size, *};
use gpui_component::{
    button::Button,
    input::{Input, InputEvent, InputState}, 
    *
};
use rfd::FileDialog;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const BG_COLOR: u32 = 0x1f1e21;
const DEFAULT_TIMELOG_FILE: &str = "timelog.txt";

enum AppScreen {
    Home,
    Timelog,
}

pub struct App {
    screen: AppScreen,
    timelog_view: Option<Entity<TimelogView>>,
    file_path: Option<PathBuf>,
}

impl App {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            screen: AppScreen::Home,
            timelog_view: None,
            file_path: None,
        }
    }

    fn open_session(&mut self, window: &mut Window, cx: &mut Context<Self>, file_path: &str) {
        self.file_path = Some(PathBuf::from(file_path));
        self.timelog_view = Some(cx.new(|cx| TimelogView::new(window, cx, file_path)));
        self.screen = AppScreen::Timelog;
        cx.notify();
    }

    fn start_new_session(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.open_session(window, cx, DEFAULT_TIMELOG_FILE);
    }

    fn load_session(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Open a file picker dialog to allow users to select an existing timelog file
        if let Some(file_path) = FileDialog::new()
            .add_filter("Text files", &["txt"])
            .set_title("Select a timelog file")
            .pick_file()
        {
            // Use to_string_lossy to handle paths with non-UTF-8 characters
            let path_str = file_path.to_string_lossy();
            self.open_session(window, cx, &path_str);
        }
    }
}

impl Render for App {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match self.screen {
            AppScreen::Home => {
                self.render_home(window, cx)
            }
            AppScreen::Timelog => {
                if let Some(timelog_view) = &self.timelog_view {
                    div()
                        .size_full()
                        .child(timelog_view.clone())
                } else {
                    self.render_home(window, cx)
                }
            }
        }
    }
}

impl App {
    fn render_home(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> Div {
        div()
            .v_flex()
            .items_center()
            .justify_center()
            .size_full()
            .font_family("Consolas")
            .bg(rgb(BG_COLOR))
            .gap_4()
            .child(
                div()
                    .text_size(px(24.0))
                    .text_color(rgb(0xffffff))
                    .child("Timelog")
            )
            .child(
                Button::new("new-session")
                    .label("New Session")
                    .on_click(cx.listener(|this, _: &ClickEvent, window, cx| {
                        this.start_new_session(window, cx);
                    }))
            )
            .child(
                Button::new("load-session")
                    .label("Load Session")
                    .on_click(cx.listener(|this, _: &ClickEvent, window, cx| {
                        this.load_session(window, cx);
                    }))
            )
    }
}

pub struct TimelogView {
    logs: Entity<Vec<(u128, SharedString)>>,
    input: Entity<InputState>,
    _input_subscription: Subscription,
    _file_path: PathBuf,
}

impl TimelogView {
    fn new(window: &mut Window, cx: &mut Context<Self>, file_path: &str) -> Self {
        let input = cx.new(|cx| InputState::new(window, cx));
        let mut loaded_logs = Vec::new();

        if let Ok(file) = File::open(file_path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.starts_with('[') {
                        if let Some(end_bracket) = line.find(']') {
                            let date_str = &line[1..end_bracket];
                            if line.len() > end_bracket + 2 {
                                let msg = &line[end_bracket + 2..];
                                if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(
                                    date_str,
                                    "%Y-%m-%d %H:%M:%S",
                                ) {
                                    if let Some(local_dt) = Local.from_local_datetime(&dt).single()
                                    {
                                        let millis = local_dt.timestamp_millis() as u128;
                                        loaded_logs
                                            .push((millis, SharedString::from(msg.to_string())));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        let logs = cx.new(|_| loaded_logs);
        let file_path_buf = PathBuf::from(file_path);

        let logs_handle = logs.clone();
        let file_path_clone = file_path_buf.clone();
        let input_subscription = cx.subscribe_in(
            &input,
            window,
            move |_view, input, event, window, cx| match event {
                InputEvent::PressEnter { .. } => {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis();

                    let text = input.read(cx).value();

                    if let Ok(mut file) = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&file_path_clone)
                    {
                        let time_str = Local
                            .timestamp_millis_opt(now as i64)
                            .unwrap()
                            .format("%Y-%m-%d %H:%M:%S")
                            .to_string();

                        if let Err(e) = writeln!(file, "[{}] {}", time_str, text) {
                            eprintln!("Failed to write to file: {}", e);
                        }
                    }

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
            _file_path: file_path_buf,
        }
    }
}

impl Render for TimelogView {
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
                .child(div().child(msg.clone()).pl_1())
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

        let size: Size<Pixels> = Size { width: Pixels::from(600.0), height: Pixels::from(800.0) };
        let bounds = WindowBounds::centered(size, cx);
        cx.spawn(async move |cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(bounds),
                    titlebar: Some(TitlebarOptions {
                        title: Some("Timelog".into()),
                        appears_transparent: Default::default(),
                        traffic_light_position: Default::default(),
                    }),
                    focus: true,
                    show: true,
                    kind: WindowKind::Normal,
                    is_movable: true,
                    is_resizable: true,
                    is_minimizable: true,
                    display_id: None,
                    window_background: WindowBackgroundAppearance::default(),
                    app_id: None,
                    window_min_size: None,
                    window_decorations: None,
                    tabbing_identifier: None,

                },
                |window, cx| {
                    let view = cx.new(|cx| App::new(window, cx));
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
