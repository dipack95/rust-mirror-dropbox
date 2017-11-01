use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    latitude: f32,
    longitude: f32
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let latitude_direction = if self.latitude >= 0.0 { "North" } else { "South" };
        let longitude_direction = if self.longitude >= 0.0 { "East" } else { "West" };

        write!(f, 
            "{}: {:.3}deg{} {:.3}deg{}", 
            self.name, self.latitude.abs(), latitude_direction,
            self.longitude.abs(), longitude_direction
        )
    }
}

fn main() {
    for city in [
        City {name: "Dublin", latitude: 44.67, longitude: -7.56},
        City {name: "Mumbai", latitude: 12.34, longitude: 12.74}
    ].iter() {
        println!("{}", *city);
    }
}