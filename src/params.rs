use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub enum Strategy {
    Always,
    Complete,
}

impl FromStr for Strategy {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "always" => Ok(Strategy::Always),
            "complete" => Ok(Strategy::Complete),
            _ => Err(()),
        }
    }
}

// Parameter to accelerate or slowdown the relocation rate.
// Base formula is: Hash(Event) % 2^(age - init_age + 1)  == 0. This parameter
// modifies it to Hash(Event) % 2^(age - init_age + 1 - relocation_rate) == 0
// where relocation_rate = 0 for standard rate and 1 for aggressive rate.
// - Standard rate keeps base behaviour with a relocation triggered every other event
//   and with the following probabilities of relocation:
//   50%: no relocation, 25%: init_age, 12.5%: init_age+1, 6.25%; init_age+2, ...
// - Aggressive rate triggers a relocation every event and each probability
//   is shifted one step to the right:
//   50%: init_age, 25%: init_age+1, 12.5%: init_age+2, 6.25%; init_age+3, ...
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RelocationRate {
    Standard,
    Aggressive,
}

impl FromStr for RelocationRate {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "standard" =>  Ok(RelocationRate::Standard),
            "aggressive" => Ok(RelocationRate::Aggressive),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DropDist {
    Exponential,
    RevProp,
}

impl FromStr for DropDist {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "exp" | "exponential" => Ok(DropDist::Exponential),
            "rev" |
            "reverse-proportional" => Ok(DropDist::RevProp),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Params {
    pub init_age: u8,
    pub split_strategy: Strategy,
    pub max_young: usize,
    pub iterations: usize,
    pub summary_intervals: usize,
    pub growth: (u8, u8),
    pub structure_output_file: Option<String>,
    pub drop_dist: DropDist,
    pub relocation_rate: RelocationRate,
    // A number between 0 and 1 indicating probability of distant relocation:
    // - 0 => purely local, meaning the weakest neighbour of current section is chosen
    // - 1 => purely distant, meaning a random section is chosen in the whole network
    //   and then the weakest neighbour of this section is chosen
    // - between the 0 and 1 a mix of the two methods
    pub distant_relocation_probability: f64,
}
