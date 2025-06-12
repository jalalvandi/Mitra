//  ~/mitra-gui/src/main.rs
//  This version fixes build errors for Iced v0.12 and type conversion issues.

use iced::widget::{button, column, container, row, text, Column, Row};
use iced::{alignment, executor, font, Application, Command, Element, Length, Settings, Theme};
use mitra_core::{self, events, ParsiDate, ParsiDateTime}; // Import ParsiDateTime as well

// --- Font Loading ---
const FONT_VAZIRMATN: &[u8] = include_bytes!("../assets/Vazirmatn-Regular.ttf");

// --- Custom Styles for Buttons (Unchanged) ---
#[derive(Clone, Copy, Default)]
pub enum DayButtonStyle {
    #[default]
    Default,
    Today,
    Selected,
}
impl button::StyleSheet for DayButtonStyle {
    type Style = Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        let mut appearance = button::Appearance {
            background: Some(iced::Background::Color(iced::Color::from_rgb8(
                0x28, 0x2C, 0x34,
            ))),
            text_color: iced::Color::WHITE,
            ..Default::default()
        };
        appearance.border.radius = 4.0.into();
        match self {
            DayButtonStyle::Default => appearance,
            DayButtonStyle::Today => {
                appearance.border.width = 1.0;
                appearance.border.color = iced::Color::from_rgb8(0x61, 0xAF, 0xEF);
                appearance
            }
            DayButtonStyle::Selected => {
                appearance.background = Some(iced::Background::Color(iced::Color::from_rgb8(
                    0x61, 0xAF, 0xEF,
                )));
                appearance.text_color = iced::Color::BLACK;
                appearance
            }
        }
    }
}

// --- Helper Trait for ParsiDate (Unchanged) ---
trait WeekdayNumber {
    fn weekday_num_from_sat(&self) -> u32;
}
impl WeekdayNumber for ParsiDate {
    fn weekday_num_from_sat(&self) -> u32 {
        match self.weekday().unwrap_or_default().as_str() {
            "شنبه" => 0,
            "یکشنبه" => 1,
            "دوشنبه" => 2,
            "سه‌شنبه" => 3,
            "چهارشنبه" => 4,
            "پنجشنبه" => 5,
            "جمعه" => 6,
            _ => 7,
        }
    }
}

// --- Application State (Unchanged) ---
#[derive(Debug)]
struct MitraApp {
    today: ParsiDate,
    selected_date: ParsiDate,
    current_year: i32,
    current_month: u32,
}

// --- Application Messages (Unchanged) ---
#[derive(Debug, Clone)]
enum Message {
    PreviousMonth,
    NextMonth,
    DayClicked(ParsiDate),
    FontLoaded(Result<(), font::Error>), // A message to handle font loading result
}

// --- Application Implementation ---
impl Application for MitraApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let today = ParsiDate::today().expect("Failed to get today's date");
        let app = MitraApp {
            today: today.clone(),
            selected_date: today.clone(),
            current_year: today.year(),
            current_month: today.month(),
        };
        (
            app,
            // The `font::load` command now returns a message that we can handle.
            font::load(FONT_VAZIRMATN).map(Message::FontLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("Mitra - Persian Calendar")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PreviousMonth => {
                if self.current_month == 1 {
                    self.current_month = 12;
                    self.current_year -= 1;
                } else {
                    self.current_month -= 1;
                }
            }
            Message::NextMonth => {
                if self.current_month == 12 {
                    self.current_month = 1;
                    self.current_year += 1;
                } else {
                    self.current_month += 1;
                }
            }
            Message::DayClicked(date) => {
                self.selected_date = date;
            }
            Message::FontLoaded(Ok(())) => {
                println!("Vazirmatn font loaded successfully.");
            }
            Message::FontLoaded(Err(e)) => {
                eprintln!("Error loading font: {:?}", e);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let calendar_view = self.build_calendar_view();
        let details_panel = self.build_details_panel();

        let main_layout = row![calendar_view, details_panel].spacing(10);

        container(main_layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(10)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

// --- View-building helper methods ---
impl MitraApp {
    fn build_calendar_view(&self) -> Element<Message> {
        let month_name = ParsiDate::new(self.current_year, self.current_month, 1)
            .unwrap()
            .format("%B");

        let header = row![
            button(text("<").font(font::Font::DEFAULT)).on_press(Message::PreviousMonth),
            text(format!("{} {}", month_name, self.current_year))
                .width(Length::Fill)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .shaping(text::Shaping::Advanced)
                .size(24),
            button(text(">").font(font::Font::DEFAULT)).on_press(Message::NextMonth)
        ]
        .spacing(10)
        .align_items(iced::Alignment::Center);

        let weekdays_header = row![
            text("ش")
                .width(Length::Fill)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .shaping(text::Shaping::Advanced),
            text("ی")
                .width(Length::Fill)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .shaping(text::Shaping::Advanced),
            text("د")
                .width(Length::Fill)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .shaping(text::Shaping::Advanced),
            text("س")
                .width(Length::Fill)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .shaping(text::Shaping::Advanced),
            text("چ")
                .width(Length::Fill)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .shaping(text::Shaping::Advanced),
            text("پ")
                .width(Length::Fill)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .shaping(text::Shaping::Advanced),
            text("ج")
                .width(Length::Fill)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .shaping(text::Shaping::Advanced),
        ]
        .spacing(5);

        let mut days_grid = Column::new().spacing(5);
        let first_day_of_month = ParsiDate::new(self.current_year, self.current_month, 1).unwrap();
        let starting_weekday = first_day_of_month.weekday_num_from_sat() as usize;
        let days_in_month = ParsiDate::days_in_month(self.current_year, self.current_month);

        let mut day_counter = 1;
        for _week in 0..6 {
            let mut week_row = Row::new().spacing(5).align_items(iced::Alignment::Center);
            for wday in 0..7 {
                if (day_counter == 1 && wday < starting_weekday) || day_counter > days_in_month {
                    week_row = week_row.push(container(text("")).width(Length::Fill));
                } else {
                    let current_day_date =
                        ParsiDate::new(self.current_year, self.current_month, day_counter).unwrap();
                    let mut style = DayButtonStyle::Default;
                    if current_day_date == self.today {
                        style = DayButtonStyle::Today;
                    }
                    if current_day_date == self.selected_date {
                        style = DayButtonStyle::Selected;
                    }
                    let event_indicator = if events::get_event_indicator(
                        self.current_year,
                        self.current_month,
                        day_counter,
                    )
                    .is_some()
                    {
                        text("•")
                            .size(16)
                            .style(iced::Color::from_rgb8(0xE0, 0x6C, 0x75))
                    } else {
                        text("")
                    };
                    let button_content = row![text(day_counter.to_string()), event_indicator]
                        .width(Length::Fill)
                        .align_items(iced::Alignment::Center)
                        .spacing(2);
                    week_row = week_row.push(
                        button(button_content)
                            .on_press(Message::DayClicked(current_day_date.clone()))
                            .width(Length::Fill)
                            .style(iced::theme::Button::Custom(Box::new(style))),
                    );
                    day_counter += 1;
                }
            }
            days_grid = days_grid.push(week_row);
            if day_counter > days_in_month {
                break;
            }
        }

        let calendar_layout = column![header, weekdays_header, days_grid]
            .spacing(15)
            .max_width(400);
        container(calendar_layout).padding(10).into()
    }

    fn build_details_panel(&self) -> Element<Message> {
        let selected_datetime = ParsiDateTime::new(
            self.selected_date.year(),
            self.selected_date.month(),
            self.selected_date.day(),
            0,
            0,
            0,
        )
        .unwrap();

        let info_result = mitra_core::get_date_info(selected_datetime, false);

        let content = match info_result {
            Ok(info) => {
                // --- Two-Column Layout for Key-Value Pairs ---
                // Create a column for labels (right-aligned)
                let labels_column = column![
                    text("میلادی:").shaping(text::Shaping::Advanced),
                    text("روز هفته:").shaping(text::Shaping::Advanced),
                    text("روز سال:").shaping(text::Shaping::Advanced),
                ]
                .spacing(12)
                .align_items(alignment::Horizontal::Right.into());

                // Create a column for values (left-aligned)
                let values_column = column![
                    text(info.gregorian_equivalent).shaping(text::Shaping::Advanced),
                    text(info.weekday).shaping(text::Shaping::Advanced),
                    text(info.day_of_year.to_string()).shaping(text::Shaping::Advanced),
                ]
                .spacing(12)
                .align_items(alignment::Horizontal::Left.into());

                // Place the two columns side-by-side in a row
                let key_value_section = row![values_column, labels_column].spacing(10);

                // --- Events Section ---
                let mut events_section = column![text("رویدادها")
                    .size(20)
                    .shaping(text::Shaping::Advanced)
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center),]
                .spacing(8);

                if let Some(events_list) = events::get_events_for_date(
                    self.selected_date.year(),
                    self.selected_date.month(),
                    self.selected_date.day(),
                ) {
                    for event in events_list {
                        let holiday_marker = if event.holiday { "[تعطیل] " } else { "- " };
                        let event_text = format!("{}{}", holiday_marker, event.title);
                        events_section = events_section.push(
                            text(event_text)
                                .shaping(text::Shaping::Advanced)
                                .width(Length::Fill)
                                .horizontal_alignment(alignment::Horizontal::Right),
                        );
                    }
                } else {
                    events_section = events_section.push(
                        text("- رویدادی ثبت نشده است")
                            .shaping(text::Shaping::Advanced)
                            .width(Length::Fill)
                            .horizontal_alignment(alignment::Horizontal::Right),
                    );
                }

                // --- Final Assembly ---
                column![
                    text("اطلاعات روز")
                        .size(22)
                        .shaping(text::Shaping::Advanced)
                        .width(Length::Fill)
                        .horizontal_alignment(alignment::Horizontal::Center),
                    text(info.parsi_date_formatted.clone())
                        .size(18)
                        .width(Length::Fill)
                        .horizontal_alignment(alignment::Horizontal::Center),
                    iced::widget::horizontal_rule(10),
                    key_value_section, // Add the key-value section here
                    iced::widget::horizontal_rule(10),
                    events_section, // Add the events section here
                ]
                .spacing(12)
            }
            Err(e) => {
                column![
                    text("Error").size(22),
                    text(format!("Could not retrieve date details: {}", e))
                ]
            }
        };

        container(content.padding(15))
            .width(Length::Fixed(320.0))
            .height(Length::Fill)
            .style(iced::theme::Container::Box)
            .into()
    }
}

// --- Main function (Corrected for default font setting) ---
pub fn main() -> iced::Result {
    let mut settings = Settings::default();

    // In Iced 0.12, we set the default font by its name.
    // The font itself is loaded via a command in `Application::new`.
    // Iced will automatically use it once it's loaded.
    settings.default_font = font::Font::with_name("Vazirmatn");

    settings.window = iced::window::Settings {
        size: iced::Size::new(750.0, 500.0),
        min_size: Some(iced::Size::new(700.0, 450.0)),
        ..Default::default()
    };

    MitraApp::run(settings)
}
