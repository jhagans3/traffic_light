use nannou::prelude::*;
use nannou::ui::prelude::*;
use std::any::Any;

// cargo run --release --bin traffic-light-dyn
fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

trait Region {
    fn as_any(&self) -> &dyn Any;
}

struct WORLD;
impl WORLD {
    fn apply(self) -> Box<dyn Region> {
        Box::new(WORLD)
    }
}
impl Region for WORLD {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct US;
impl US {
    fn apply(self) -> Box<dyn Region> {
        Box::new(US)
    }
}
impl Region for US {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct UK;
impl UK {
    fn apply(self) -> Box<dyn Region> {
        Box::new(UK)
    }
}
impl Region for UK {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct GER;
impl GER {
    fn apply(self) -> Box<dyn Region> {
        Box::new(GER)
    }
}
impl Region for GER {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

trait Hue {
    fn as_any(&self) -> &dyn Any;
}

// Traffic light's state
struct Green;
impl Green {
    fn apply(self) -> Box<dyn Hue> {
        Box::new(Green)
    }
}
impl Hue for Green {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct Yellow;
impl Yellow {
    fn apply(self) -> Box<dyn Hue> {
        Box::new(Yellow)
    }
}
impl Hue for Yellow {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct Amber;
impl Amber {
    fn apply(self) -> Box<dyn Hue> {
        Box::new(Amber)
    }
}
impl Hue for Amber {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct AmberRed;
impl AmberRed {
    fn apply(self) -> Box<dyn Hue> {
        Box::new(AmberRed)
    }
}
impl Hue for AmberRed {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct Red;
impl Red {
    fn apply(self) -> Box<dyn Hue> {
        Box::new(Red)
    }
}
impl Hue for Red {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct TrafficLight<Region, Hue> {
    region: Region,
    hue: Hue,
}

struct ShowLights {
    green: nannou::color::rgb::Rgb<nannou::color::encoding::Srgb, u8>,
    yellow: nannou::color::rgb::Rgb<nannou::color::encoding::Srgb, u8>,
    red: nannou::color::rgb::Rgb<nannou::color::encoding::Srgb, u8>,
}

impl TrafficLight<Box<dyn Region>, Box<dyn Hue>> {
    fn transition(&self) -> TrafficLight<Box<dyn Region>, Box<dyn Hue>> {
        if (*self.region).as_any().is::<US>() {
            if (*self.hue).as_any().is::<Green>() {
                TrafficLight {
                    region: US.apply(),
                    hue: Yellow.apply(),
                }
            } else if (*self.hue).as_any().is::<Yellow>() {
                TrafficLight {
                    region: US.apply(),
                    hue: Red.apply(),
                }
            } else if (*self.hue).as_any().is::<Red>() {
                TrafficLight {
                    region: US.apply(),
                    hue: Green.apply(),
                }
            } else {
                unreachable!()
            }
        } else if (*self.region).as_any().is::<UK>() {
            if (*self.hue).as_any().is::<Green>() {
                TrafficLight {
                    region: UK.apply(),
                    hue: Amber.apply(),
                }
            } else if (*self.hue).as_any().is::<Amber>() {
                TrafficLight {
                    region: UK.apply(),
                    hue: Red.apply(),
                }
            } else if (*self.hue).as_any().is::<Red>() {
                TrafficLight {
                    region: UK.apply(),
                    hue: AmberRed.apply(),
                }
            } else if (*self.hue).as_any().is::<AmberRed>() {
                TrafficLight {
                    region: UK.apply(),
                    hue: Green.apply(),
                }
            } else {
                unreachable!()
            }
        } else if (*self.region).as_any().is::<GER>() {
            if (*self.hue).as_any().is::<Green>() {
                TrafficLight {
                    region: GER.apply(),
                    hue: Yellow.apply(),
                }
            } else if (*self.hue).as_any().is::<Yellow>() {
                TrafficLight {
                    region: GER.apply(),
                    hue: Red.apply(),
                }
            } else if (*self.hue).as_any().is::<Red>() {
                TrafficLight {
                    region: GER.apply(),
                    hue: Amber.apply(),
                }
            } else if (*self.hue).as_any().is::<Amber>() {
                TrafficLight {
                    region: GER.apply(),
                    hue: Green.apply(),
                }
            } else {
                unreachable!()
            }
        } else if (*self.region).as_any().is::<WORLD>() {
            println!("In world");
            TrafficLight {
                region: WORLD.apply(),
                hue: Green.apply(),
            }
        } else {
            unreachable!()
        }
    }

    fn rule(&self) -> &str {
        if let Some(_us) = (*self.region).as_any().downcast_ref::<US>() {
            if (*self.hue).as_any().is::<Green>() {
                "I am the color Green, which means GO!"
            } else if (*self.hue).as_any().is::<Yellow>() {
                "I am the color Yellow, which means Use caution!"
            } else if (*self.hue).as_any().is::<Red>() {
                "I am the color Red, which means STOP!"
            } else {
                unreachable!()
            }
        } else if (*self.region).as_any().is::<UK>() {
            if (*self.hue).as_any().is::<Green>() {
                "I am the color Green, which means GO!"
            } else if (*self.hue).as_any().is::<Amber>() {
                "I am the color amber, which means Use caution!"
            } else if (*self.hue).as_any().is::<AmberRed>() {
                "I am the colours Amber and Red, which means prepare for Green!"
            } else if (*self.hue).as_any().is::<Red>() {
                "I am the color Red, which means STOP!"
            } else {
                unreachable!()
            }
        } else if (*self.region).as_any().is::<GER>() {
            if (*self.hue).as_any().is::<Green>() {
                "I am the color Green, which means GO!"
            } else if (*self.hue).as_any().is::<Yellow>() {
                "I am the color Yellow, which means Use caution!"
            } else if (*self.hue).as_any().is::<Red>() {
                "I am the color Red, which means STOP!"
            } else if (*self.hue).as_any().is::<Amber>() {
                "I am the color Yellow, which means prepare for Green!"
            } else {
                unreachable!()
            }
        } else if (*self.region).as_any().is::<WORLD>() {
            "In world"
        } else {
            unreachable!()
        }
    }

    fn show(&self) -> ShowLights {
        if (*self.region).as_any().is::<US>() {
            if (*self.hue).as_any().is::<Green>() {
                ShowLights {
                    green: LIME,
                    yellow: DARKORANGE,
                    red: DARKRED,
                }
            } else if (*self.hue).as_any().is::<Yellow>() {
                ShowLights {
                    green: DARKGREEN,
                    yellow: YELLOW,
                    red: DARKRED,
                }
            } else if (*self.hue).as_any().is::<Red>() {
                ShowLights {
                    green: DARKGREEN,
                    yellow: DARKORANGE,
                    red: RED,
                }
            } else {
                unreachable!()
            }
        } else if (*self.region).as_any().is::<UK>() {
            if (*self.hue).as_any().is::<Green>() {
                ShowLights {
                    green: LIME,
                    yellow: DARKORANGE,
                    red: DARKRED,
                }
            } else if (*self.hue).as_any().is::<Amber>() {
                ShowLights {
                    green: DARKGREEN,
                    yellow: YELLOW,
                    red: DARKRED,
                }
            } else if (*self.hue).as_any().is::<Red>() {
                ShowLights {
                    green: DARKGREEN,
                    yellow: DARKORANGE,
                    red: RED,
                }
            } else if (*self.hue).as_any().is::<AmberRed>() {
                ShowLights {
                    green: DARKGREEN,
                    yellow: YELLOW,
                    red: RED,
                }
            } else {
                unreachable!()
            }
        } else if (*self.region).as_any().is::<GER>() {
            if (*self.hue).as_any().is::<Green>() {
                ShowLights {
                    green: LIME,
                    yellow: DARKORANGE,
                    red: DARKRED,
                }
            } else if (*self.hue).as_any().is::<Yellow>() {
                ShowLights {
                    green: DARKGREEN,
                    yellow: YELLOW,
                    red: DARKRED,
                }
            } else if (*self.hue).as_any().is::<Red>() {
                ShowLights {
                    green: DARKGREEN,
                    yellow: DARKORANGE,
                    red: RED,
                }
            } else if (*self.hue).as_any().is::<Amber>() {
                ShowLights {
                    green: DARKGREEN,
                    yellow: YELLOW,
                    red: DARKRED,
                }
            } else {
                unreachable!()
            }
        } else if (*self.region).as_any().is::<WORLD>() {
            ShowLights {
                green: DARKGREEN,
                yellow: DARKORANGE,
                red: DARKRED,
            }
        } else {
            unreachable!()
        }
    }
}

struct Model {
    ui: Ui,
    ids: Ids,
    title: String,
    traffic_light: TrafficLight<Box<dyn Region>, Box<dyn Hue>>,
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
    let title = "Affine Types".to_string();

    let traffic_light: TrafficLight<Box<dyn Region>, Box<dyn Hue>> = TrafficLight {
        region: WORLD.apply(),
        hue: Green.apply(),
    };

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
        model.traffic_light.region = US.apply();
        model.traffic_light.hue = Green.apply();
        model.title = model.traffic_light.rule().to_string();
    }

    for _click in new_button()
        .down(10.0)
        .label("UK traffic light")
        .set(model.ids.uk, ui)
    {
        model.traffic_light.region = UK.apply();
        model.traffic_light.hue = Green.apply();
        model.title = model.traffic_light.rule().to_string();
    }

    for _click in new_button()
        .down(10.0)
        .label("German traffic light")
        .set(model.ids.ger, ui)
    {
        model.traffic_light.region = GER.apply();
        model.traffic_light.hue = Green.apply();
        model.title = model.traffic_light.rule().to_string();
    }

    if let Some(_world) = (model.traffic_light.region)
        .as_any()
        .downcast_ref::<WORLD>()
    {
    } else {
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

    draw.text(&model.title)
        .align_text_top()
        .color(WHITE)
        .font_size(24);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // Draw the state of the `Ui` to the frame.
    model.ui.draw_to_frame(app, &frame).unwrap();
}
