enum Type {
    Normal,
    Fire,
    Water,
    Grass,
    Electric,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

enum Category {
    Physical,
    Special,
    Status,
}

enum Status {
    BRN,
    FRZ,
    PAR,
    PSN,
    TOX,
    SLP,
}

// This is in the order they appear on https://bulbapedia.bulbagarden.net/wiki/Category:Moves_by_targeting.
// I have not implemented the necessary enum variants for triple battles / horde battles.
enum Targeting {
    AnyAdjacentFoe(AnyAdjacentFoe),
    AnyAdjacent(AnyAdjacent),
    Any(Any),
    UserOrAdjacentAlly(UserOrAdjacentAlly),
    AllAdjacentFoes(AllAdjacentFoes),
    AllAdjacent(AllAdjacent),
    AllAllies(AllAllies),
    AllFoes(AllFoes),
    All(All),
    AnyAdjacentAlly(AnyAdjacentAlly),
    User(User),
    UserAndAllAllies(UserAndAllAllies),
}

enum AnyAdjacentFoe {
    Foe1,
    Foe2,
}

enum AnyAdjacent {
    Foe1,
    Foe2,
    Ally,
}

enum Any {
    Foe1,
    Foe2,
    User,
    Ally,
}

enum UserOrAdjacentAlly {
    User,
    Ally,
}

enum AllAdjacentFoes {
    Foes,
}

enum AllAdjacent {
    FoesAndAlly,
}

enum AllAllies {
    Ally,
}

enum AllFoes {
    Foes,
}

enum All {
    FoesAndSelfAndAlly, // verbose on purpose. Gives a checklist of things to implement, if necessary.
}

enum AnyAdjacentAlly {
    Ally,
}

enum User {
    User,
}

enum UserAndAllAllies {
    UserAndAlly,
}

impl From<AnyAdjacentFoe> for Targeting {
    fn from(value: AnyAdjacentFoe) -> Self {
        Targeting::AnyAdjacentFoe(value)
    }
}
impl From<AnyAdjacent> for Targeting {
    fn from(value: AnyAdjacent) -> Self {
        Targeting::AnyAdjacent(value)
    }
}
impl From<Any> for Targeting {
    fn from(value: Any) -> Self {
        Targeting::Any(value)
    }
}
impl From<UserOrAdjacentAlly> for Targeting {
    fn from(value: UserOrAdjacentAlly) -> Self {
        Targeting::UserOrAdjacentAlly(value)
    }
}
impl From<AllAdjacentFoes> for Targeting {
    fn from(value: AllAdjacentFoes) -> Self {
        Targeting::AllAdjacentFoes(value)
    }
}
impl From<AllAdjacent> for Targeting {
    fn from(value: AllAdjacent) -> Self {
        Targeting::AllAdjacent(value)
    }
}
impl From<AllAllies> for Targeting {
    fn from(value: AllAllies) -> Self {
        Targeting::AllAllies(value)
    }
}
impl From<AllFoes> for Targeting {
    fn from(value: AllFoes) -> Self {
        Targeting::AllFoes(value)
    }
}
impl From<All> for Targeting {
    fn from(value: All) -> Self {
        Targeting::All(value)
    }
}
impl From<AnyAdjacentAlly> for Targeting {
    fn from(value: AnyAdjacentAlly) -> Self {
        Targeting::AnyAdjacentAlly(value)
    }
}
impl From<User> for Targeting {
    fn from(value: User) -> Self {
        Targeting::User(value)
    }
}
impl From<UserAndAllAllies> for Targeting {
    fn from(value: UserAndAllAllies) -> Self {
        Targeting::UserAndAllAllies(value)
    }
}

struct Stats {
    hp: u16,
    atk: u16,
    def: u16,
    spa: u16,
    spd: u16,
    spe: u16,
}

trait Move {
    fn apply(&self, state: &State, user: usize, target: Targeting) -> Vec<(f32, State)>;
    fn typ(&self) -> Type;
    fn category(&self) -> Category;
    fn power(&self) -> Option<u16>;
    fn accuracy(&self) -> Option<u8> {
        Some(100)
    }
}

fn damage_calc(
    level: u8,
    power: u16,
    a: u16,
    d: u16,
    stab: f32,
    typ: f32,
    critrate: u8,
) -> std::collections::HashMap<u32, f32> {
    todo!()
}

struct Pokemon {
    stats: &'static Stats,
    typ: (Type, Option<Type>),
    tera_type: Type,
    moves: (
        Box<dyn Move>,
        Option<Box<dyn Move>>,
        Option<Box<dyn Move>>,
        Option<Box<dyn Move>>,
    ),
    condition: Option<Status>,
    current_hp: [f32; 512],
}

type State = [Option<Pokemon>; 12];

fn main() {
    println!(
        "{}\t{}",
        std::mem::align_of::<(f32, u32)>(),
        std::mem::align_of::<(f32, u16)>()
    );
}
