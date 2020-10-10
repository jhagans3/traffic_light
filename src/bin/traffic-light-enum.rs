use nannou::prelude::*;
use nannou::ui::prelude::*;

// cargo run --release --bin traffic-light-enum
fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

#[derive(Copy, Clone)]
enum CountryEnum {
    US,
    UK,
    GER,
    Unknown,
}

impl CountryEnum {
    fn show(&self) -> &str {
        match self {
            CountryEnum::US => "US",
            CountryEnum::UK => "UK",
            CountryEnum::GER => "GER",
            CountryEnum::Unknown => "Select a country",
        }
    }
}

#[derive(Copy, Clone)]
enum LightEnum {
    Green,
    Yellow,
    Amber,
    AmberRed,
    Red,
    NA,
}

struct ShowLights {
    green: nannou::color::rgb::Rgb<nannou::color::encoding::Srgb, u8>,
    yellow: nannou::color::rgb::Rgb<nannou::color::encoding::Srgb, u8>,
    red: nannou::color::rgb::Rgb<nannou::color::encoding::Srgb, u8>,
}

#[derive(Copy, Clone)]
struct TrafficLightForTheModel {
    country: CountryEnum,
    light: LightEnum,
}

impl TrafficLightForTheModel {
    fn new(country: CountryEnum, light: LightEnum) -> Self {
        TrafficLightForTheModel { country, light }
    }

    fn transition(self) -> Self {
        match self.country {
            CountryEnum::US => {
                let new_light = match self.light {
                    LightEnum::Green => LightEnum::Yellow,
                    LightEnum::Yellow => LightEnum::Red,
                    LightEnum::Red => LightEnum::Green,
                    LightEnum::Amber => LightEnum::NA,
                    LightEnum::AmberRed => LightEnum::NA,
                    LightEnum::NA => LightEnum::NA,
                };

                TrafficLightForTheModel {
                    country: self.country,
                    light: new_light,
                }
            }
            CountryEnum::UK => {
                let new_light = match self.light {
                    LightEnum::Green => LightEnum::Yellow,
                    LightEnum::Yellow => LightEnum::Red,
                    LightEnum::Amber => LightEnum::Red,
                    LightEnum::Red => LightEnum::AmberRed,
                    LightEnum::AmberRed => LightEnum::Green,
                    LightEnum::NA => LightEnum::NA,
                };

                TrafficLightForTheModel {
                    country: self.country,
                    light: new_light,
                }
            }
            CountryEnum::GER => {
                let new_light = match self.light {
                    LightEnum::Green => LightEnum::Yellow,
                    LightEnum::Yellow => LightEnum::Red,
                    LightEnum::Red => LightEnum::Amber,
                    LightEnum::Amber => LightEnum::Green,
                    LightEnum::AmberRed => LightEnum::NA,
                    LightEnum::NA => LightEnum::NA,
                };

                TrafficLightForTheModel {
                    country: self.country,
                    light: new_light,
                }
            }
            CountryEnum::Unknown => self,
        }
    }

    fn show(&self) -> ShowLights {
        match self.light {
            LightEnum::Green => ShowLights {
                green: LIME,
                yellow: DARKORANGE,
                red: DARKRED,
            },
            LightEnum::Yellow => ShowLights {
                green: DARKGREEN,
                yellow: YELLOW,
                red: DARKRED,
            },
            LightEnum::Amber => ShowLights {
                green: DARKGREEN,
                yellow: YELLOW,
                red: DARKRED,
            },
            LightEnum::AmberRed => ShowLights {
                green: DARKGREEN,
                yellow: YELLOW,
                red: RED,
            },
            LightEnum::Red => ShowLights {
                green: DARKGREEN,
                yellow: DARKORANGE,
                red: RED,
            },
            LightEnum::NA => ShowLights {
                green: BLACK,
                yellow: BLACK,
                red: BLACK,
            },
        }
    }

    fn rule(&self) -> &str {
        match self.light {
            LightEnum::Green => "I am the color Green, which means GO!",
            LightEnum::Yellow => "I am the color Yellow, which means Use caution!",
            LightEnum::Amber => "I am the colour Amber, which means Use caution!",
            LightEnum::AmberRed => "I am the colours Amber and Red, which means prepare for Green!",
            LightEnum::Red => "I am the color Red, which means STOP!",
            LightEnum::NA => "I am not in a valid state!",
        }
    }
}

struct Model {
    ui: Ui,
    ids: Ids,
    title: String,
    traffic_light: TrafficLightForTheModel,
}

widget_ids! {
    struct Ids {
        next_state,
        us,
        uk,
        ger,
    }
}

fn model(app: &App) -> Model {
    // Set the loop mode to wait for events, an energy-efficient option for pure-GUI apps.
    app.set_loop_mode(LoopMode::Wait);

    // Create the UI.
    let mut ui = app.new_ui().build().unwrap();

    // Generate some ids for our widgets.
    let ids = Ids::new(ui.widget_id_generator());

    // Init our variables
    let title = "Enums for state".to_string();
    let traffic_light = TrafficLightForTheModel::new(CountryEnum::Unknown, LightEnum::Green);

    Model {
        ui,
        ids,
        title,
        traffic_light,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // Calling `set_widgets` allows us to instantiate some widgets.
    let ui = &mut model.ui.set_widgets();

    fn new_button<'a>() -> widget::Button<'a, widget::button::Flat> {
        widget::Button::new()
            .w_h(200.0, 60.0)
            .label_font_size(15)
            .rgb(0.3, 0.3, 0.3)
            .label_rgb(1.0, 1.0, 1.0)
            .border(0.0)
    }

    for _click in new_button()
        .top_left_with_margin(20.0)
        .label("US traffic light")
        .set(model.ids.us, ui)
    {
        model.traffic_light.country = CountryEnum::US;
        model.traffic_light.light = LightEnum::Green;
        model.title = model.traffic_light.rule().to_string();
    }

    for _click in new_button()
        .down(10.0)
        .label("Uk traffic light")
        .set(model.ids.uk, ui)
    {
        model.traffic_light.country = CountryEnum::UK;
        model.traffic_light.light = LightEnum::Green;
        model.title = model.traffic_light.rule().to_string();
    }

    for _click in new_button()
        .down(10.0)
        .label("GER traffic light")
        .set(model.ids.ger, ui)
    {
        model.traffic_light.country = CountryEnum::GER;
        model.traffic_light.light = LightEnum::Green;
        model.title = model.traffic_light.rule().to_string();
    }

    match model.traffic_light.country {
        CountryEnum::Unknown => {}
        _ => {
            for _click in new_button()
                .down(10.0)
                .label("Next State")
                .set(model.ids.next_state, ui)
            {
                model.traffic_light = model.traffic_light.transition();
                model.title = model.traffic_light.rule().to_string();
            }
        }
    }
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    draw.background().rgb(0.02, 0.02, 0.02);

    let win = app.window_rect();

    let base = nannou::geom::rect::Rect::from_w_h(110.0, 310.0).top_right_of(win.pad(20.0));
    draw.rect().xy(base.xy()).wh(base.wh()).color(GOLDENROD);

    let green = nannou::geom::rect::Rect::from_w_h(100.0, 100.0).top_right_of(win.pad(25.0));
    draw.ellipse()
        .xy(green.xy())
        .wh(green.wh())
        .color(model.traffic_light.show().green);

    let yellow = green.below(green);
    draw.ellipse()
        .xy(yellow.xy())
        .wh(yellow.wh())
        .color(model.traffic_light.show().yellow);

    let red = yellow.below(yellow);
    draw.ellipse()
        .xy(red.xy())
        .wh(red.wh())
        .color(model.traffic_light.show().red);

    draw.text(&model.traffic_light.country.show())
        .x_y(0.0, 200.0)
        .color(WHITE)
        .font_size(24);
    draw.text(&model.title)
        .align_text_top()
        .color(WHITE)
        .font_size(24);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // Draw the state of the `Ui` to the frame.
    model.ui.draw_to_frame(app, &frame).unwrap();
}
