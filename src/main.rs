struct US;
struct UK;

struct Green;
struct Yellow;
struct Amber;
struct AmberRed;
struct Red;

struct TrafficLight<Country, State> {
    country: Country,
    state: State,
}

trait HueState<Country, HueChange> {
    fn transition(self) -> TrafficLight<Country, HueChange>;
}

impl<Country, State> TrafficLight<Country, State> {
    fn apply<Region, Hue>(region: Region, hue: Hue) -> TrafficLight<Region, Hue> {
        TrafficLight {
            country: region,
            state: hue,
        }
    }
}



impl<Green> TrafficLight<US, Green> {
    fn green_rule(&self) -> () {
        println!("I am the color Green, which means GO!");
    }
}

impl<Yellow> TrafficLight<US, Yellow> {
    fn yellow_rule(&self) -> () {
        println!("I am the color Yellow, which means Use caution!");
    }
}

impl<Red> TrafficLight<US, Red> {
    fn red_rule(&self) -> () {
        println!("I am the color Red, which means STOP!");
    }
}

impl<Green> TrafficLight<UK, Green> {
    fn green_rule(&self) -> () {
        println!("I am the colour Green, which means GO!");
    }
}

impl<Amber> TrafficLight<UK, Amber> {
    fn amber_rule(&self) -> () {
        println!("I am the colour Amber, which means Use caution!");
    }
}

impl<AmberRed> TrafficLight<UK, AmberRed> {
    fn amber_red_rule(&self) -> () {
        println!("I am the colour Amber & Red, which means prepare for Green!");
    }
}

impl<Red> TrafficLight<UK, Red> {
    fn red_rule(&self) -> () {
        println!("I am the colour Red, which means STOP!");
    }
}



impl HueState<US, Yellow> for TrafficLight<US, Green> {
    fn transition(self) -> TrafficLight<US, Yellow> {
        println!("I was Green, now I am Yellow");
        TrafficLight {
            country: self.country,
            state: Yellow,
        }
    }
}

impl HueState<US, Red> for TrafficLight<US, Yellow> {
    fn transition(self) -> TrafficLight<US, Red> {
        println!("I was Yellow, now I am Red");
        TrafficLight {
            country: self.country,
            state: Red,
        }
    }
}

impl HueState<US, Green> for TrafficLight<US, Red> {
    fn transition(self) -> TrafficLight<US, Green> {
        println!("I was Red, now I am Green");
        TrafficLight {
            country: self.country,
            state: Green,
        }
    }
}

impl HueState<UK, Amber> for TrafficLight<UK, Green> {
    fn transition(self) -> TrafficLight<UK, Amber> {
        println!("I was Green, now I am Amber");
        TrafficLight {
            country: self.country,
            state: Amber,
        }
    }
}

impl HueState<UK, Red> for TrafficLight<UK, Amber> {
    fn transition(self) -> TrafficLight<UK, Red> {
        println!("I was Amber, now I am Red");
        TrafficLight {
            country: self.country,
            state: Red,
        }
    }
}

impl HueState<UK, AmberRed> for TrafficLight<UK, Red> {
    fn transition(self) -> TrafficLight<UK, AmberRed> {
        println!("I was Red, now I am Amber & Red");
        TrafficLight {
            country: self.country,
            state: AmberRed,
        }
    }
}

impl HueState<UK, Green> for TrafficLight<UK, AmberRed> {
    fn transition(self) -> TrafficLight<UK, Green> {
        println!("I was Amber & Red, now I am Green");
        TrafficLight {
            country: self.country,
            state: Green,
        }
    }
}



fn main() {
    let us_green = TrafficLight::<US, Green>::apply(US, Green);
    println!("US traffic Light");
    us_green.green_rule();
    let us_yellow = us_green.transition();
    us_yellow.yellow_rule();
    let us_red = us_yellow.transition();
    us_red.red_rule();

    println!();

    let uk_green = TrafficLight::<UK, Green>::apply(UK, Green);
    println!("UK traffic Light");
    uk_green.green_rule();
    let uk_amber = uk_green.transition();
    uk_amber.amber_rule();
    let uk_red = uk_amber.transition();
    uk_red.red_rule();
    let uk_amber_red = uk_red.transition();
    uk_amber_red.amber_red_rule();
}
