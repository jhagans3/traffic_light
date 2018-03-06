// https://en.wikipedia.org/wiki/Traffic_light

fn main() {

 // https://simple.wikipedia.org/wiki/Traffic_light   

    let us_green = TrafficLight::<US, Green>::apply(US, Green);
    println!("US traffic Light");
    us_green.green_rule();
    let us_yellow = us_green.transition();
    us_yellow.yellow_rule();
    let us_red = us_yellow.transition();
    us_red.red_rule();

    println!();

 // http://www.101computing.net/traffic-lights-challenge/traffic-light-sequence/

    let uk_green = TrafficLight::<UK, Green>::apply(UK, Green);
    println!("UK traffic Light");
    uk_green.green_rule();
    let uk_amber = uk_green.transition();
    uk_amber.amber_rule();
    let uk_red = uk_amber.transition();
    uk_red.red_rule();
    let uk_amber_red = uk_red.transition();
    uk_amber_red.amber_red_rule();

// only red lights exists now us green is gone
// https://en.wikipedia.org/wiki/Substructural_type_system
    // us_green.green_rule(); // this will not compile

    // uk_amber_red cannot go to red

    let us_amber = TrafficLight::<US, Amber>::apply(US, Amber);
//  us amber has no defined behaviors
    // us_amber.transition(); // this will not compile
    // us_amber.state(); // this will not compile
    
}





// Country or Region
struct US;
struct UK;

// Traffic light's state
struct Green;
struct Yellow;
struct Amber;
struct AmberRed;
struct Red;

// state field is not used 
#[allow(dead_code)] 
struct TrafficLight<Country, State> {
    country: Country,
    state: State,
}

// How the trafic light's state changes, note the self
trait HueState<Country, HueChange> {
    fn transition(self) -> TrafficLight<Country, HueChange>;
}

// How a new Traffic light is made
impl<Country, State> TrafficLight<Country, State> {
    fn apply<Region, Hue>(region: Region, hue: Hue) -> TrafficLight<Region, Hue> {
        TrafficLight {
            country: region,
            state: hue,
        }
    }
}


// For Traffic lights of types, in the US with the state green, behaviors
impl TrafficLight<US, Green> {
    fn green_rule(&self) -> () {
        println!("I am the color Green, which means GO!");
    }
}

impl TrafficLight<US, Yellow> {
    fn yellow_rule(&self) -> () {
        println!("I am the color Yellow, which means Use caution!");
    }
}

impl TrafficLight<US, Red> {
    fn red_rule(&self) -> () {
        println!("I am the color Red, which means STOP!");
    }
}

// For Traffic lights of types, in the UK with the state green, behaviors
impl TrafficLight<UK, Green> {
    fn green_rule(&self) -> () {
        println!("I am the colour Green, which means GO!");
    }
}

impl TrafficLight<UK, Amber> {
    fn amber_rule(&self) -> () {
        println!("I am the colour Amber, which means Use caution!");
    }
}

impl TrafficLight<UK, AmberRed> {
    fn amber_red_rule(&self) -> () {
        println!("I am the colour Amber & Red, which means prepare for Green!");
    }
}

impl TrafficLight<UK, Red> {
    fn red_rule(&self) -> () {
        println!("I am the colour Red, which means STOP!");
    }
}


// Light changes from green ~> yellow in the us
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

// Light changes from green ~> amber in the uk
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