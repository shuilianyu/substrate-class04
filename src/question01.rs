enum TrafficLight {
    Red,
    Green,
    Yellow,
}


struct sTraffic {
    tl: TrafficLight,
}


pub trait Traffic {
    fn time(&self) ->u8;
}


impl Traffic for sTraffic{
    fn time(&self) ->u8 {
        match self.tl {
            TrafficLight::Red => return 60,
            TrafficLight::Green => return 20,
            TrafficLight::Yellow => return 3,
        }
    }
}


pub fn getTimes<T:Traffic>(item: &T) -> u8{
    return item.time();
}


fn main() {

    let trafficLightTime = sTraffic {
        tl: TrafficLight::Red,
    };

    let times = getTimes(&trafficLightTime);
    
    println!("light Red is: {}", times);

    let GreentrafficLight = sTraffic {
        tl: TrafficLight::Green,
    };

    let times = getTimes(&GreentrafficLight);
    
    println!("light Green is: {}", times);

    let YellowtrafficLight = sTraffic {
        tl: TrafficLight::Yellow,
    };

    let times = getTimes(&YellowtrafficLight);
    
    println!("light Yellow is: {}", times);
}
