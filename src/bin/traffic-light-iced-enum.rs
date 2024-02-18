// cargo run --release --bin traffic-light-iced-enum

use std::vec;

use iced::widget::canvas::{Frame, Geometry, Path, Program};
use iced::widget::{button, column, container, row, Canvas};
use iced::{
    executor, mouse, Application, Color, Command, Element, Length, Point, Rectangle, Renderer,
    Settings, Size, Theme,
};

pub fn main() -> iced::Result {
    TrafficLight::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
enum Country {
    US,
    UK,
    GER,
}

#[derive(Debug, Clone, Copy)]
enum Light {
    Green,
    Yellow,
    Amber,
    AmberRed,
    Red,
    NA,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    USButton,
    UKButton,
    GERButton,
    NEXT,
}

#[derive(Debug, Clone, Copy)]
struct TrafficLight {
    country: Country,
    light: Light,
}

#[derive(Debug, Clone, Copy)]
struct ShowLights {
    red: Color,
    yellow: Color,
    green: Color,
}

impl TrafficLight {
    fn next(self) -> Self {
        match self.country {
            Country::US => {
                let next_color = match self.light {
                    Light::Green => Light::Yellow,
                    Light::Yellow => Light::Red,
                    Light::Red => Light::Green,
                    Light::Amber => Light::NA,
                    Light::AmberRed => Light::NA,
                    Light::NA => Light::NA,
                };

                TrafficLight {
                    country: self.country,
                    light: next_color,
                }
            }
            Country::UK => {
                let next_color = match self.light {
                    Light::Green => Light::Yellow,
                    Light::Yellow => Light::Red,
                    Light::Amber => Light::Red,
                    Light::Red => Light::AmberRed,
                    Light::AmberRed => Light::Green,
                    Light::NA => Light::NA,
                };

                TrafficLight {
                    country: self.country,
                    light: next_color,
                }
            }
            Country::GER => {
                let next_color = match self.light {
                    Light::Green => Light::Yellow,
                    Light::Yellow => Light::Red,
                    Light::Red => Light::Amber,
                    Light::Amber => Light::Green,
                    Light::AmberRed => Light::NA,
                    Light::NA => Light::NA,
                };

                TrafficLight {
                    country: self.country,
                    light: next_color,
                }
            }
        }
    }

    fn show(&self) -> ShowLights {
        match self.light {
            Light::Green => ShowLights {
                green: Color::from_rgb(0.0, 1.0, 0.0),
                yellow: Color::from_rgb(0.33, 0.30, 0.12),
                red: Color::from_rgb(0.33, 0.13, 0.12),
            },
            Light::Yellow => ShowLights {
                green: Color::from_rgb(0.12, 0.33, 0.17),
                yellow: Color::from_rgb(1.0, 1.0, 0.0),
                red: Color::from_rgb(0.33, 0.13, 0.12),
            },
            Light::Amber => ShowLights {
                green: Color::from_rgb(0.12, 0.33, 0.17),
                yellow: Color::from_rgb(1.0, 1.0, 0.0),
                red: Color::from_rgb(0.33, 0.13, 0.12),
            },
            Light::AmberRed => ShowLights {
                green: Color::from_rgb(0.12, 0.33, 0.17),
                yellow: Color::from_rgb(1.0, 1.0, 0.0),
                red: Color::from_rgb(1.0, 0.0, 0.0),
            },
            Light::Red => ShowLights {
                green: Color::from_rgb(0.12, 0.33, 0.17),
                yellow: Color::from_rgb(0.33, 0.30, 0.12),
                red: Color::from_rgb(1.0, 0.0, 0.0),
            },
            Light::NA => ShowLights {
                green: Color::from_rgb(0.0, 0.0, 0.0),
                yellow: Color::from_rgb(0.0, 0.0, 0.0),
                red: Color::from_rgb(0.0, 0.0, 0.0),
            },
        }
    }
}

impl Application for TrafficLight {
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;
    type Message = Message;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                country: Country::US,
                light: Light::Green,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Traffic Light")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::USButton => self.country = Country::US,
            Message::UKButton => self.country = Country::UK,
            Message::GERButton => self.country = Country::GER,
            Message::NEXT => {
                let current_state = self.clone();
                let next_state = current_state.next();
                self.country = next_state.country;
                self.light = next_state.light;
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let us_button = button("US").padding(10).on_press(Message::USButton);
        let uk_button = button("UK").padding(10).on_press(Message::UKButton);
        let ger_button = button("GER").padding(10).on_press(Message::GERButton);
        let next_button = button("NEXT STATE").padding(10).on_press(Message::NEXT);

        let canvas = Canvas::new(self);

        let content = row![
            column![us_button, uk_button, ger_button],
            column![next_button],
            column![canvas]
        ];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}

impl<Message> Program<Message> for TrafficLight {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, Size::new(500.0, 500.0));

        let lights_holder = Path::rectangle(Point::new(100.0, 10.0), Size::new(100.0, 200.0));
        frame.fill(&lights_holder, Color::from_rgb(0.0, 0.0, 0.0));

        let red_circle = Path::circle(Point::new(150.0, 50.0), 30.0);
        let yellow_circle = Path::circle(Point::new(150.0, 110.0), 30.0);
        let green_circle = Path::circle(Point::new(150.0, 170.0), 30.0);

        let lights = self.show();
        frame.fill(&red_circle, lights.red);
        frame.fill(&yellow_circle, lights.yellow);
        frame.fill(&green_circle, lights.green);

        vec![frame.into_geometry()]
    }
}
