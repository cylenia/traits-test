trait GreatestTraitOfAllTime {
    fn is_gay(&self) -> bool;
    fn is_stoned(&self) -> bool;
    fn is_twelve(&self) -> bool;
}

impl GreatestTraitOfAllTime for u32 {
    fn is_gay(&self) -> bool {
        *self == 69
    }
    fn is_stoned(&self) -> bool {
        *self == 420
    }
    fn is_twelve(&self) -> bool {
        *self == 67
    }
}

fn main() {
    println!(
        "is_gay    => 67:{} | 420:{} | 69:{}",
        67.is_gay(),
        420.is_gay(),
        69.is_gay()
    );
    println!(
        "is_stoned => 67:{} | 420:{} | 69:{}",
        67.is_stoned(),
        420.is_stoned(),
        69.is_stoned()
    );
    println!(
        "is_twelve => 67:{} | 420:{} | 69:{}",
        67.is_twelve(),
        420.is_twelve(),
        69.is_twelve()
    );
}
