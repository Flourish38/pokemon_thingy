use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy)]
enum Type {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
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

fn half_down(x: f32) -> f32 {
    if x.fract() <= 0.5 {
        x.floor()
    } else {
        x.ceil()
    }
}

/*
// This is the same thing but didn't get changed for some reason by the formatting... -_-

const TYPE_EFFECTIVENESS: [f32; 324] = [
//  NOR  FIR  WAT  ELE  GRA  ICE  FIG  POI  GRO  FLY  PSY  BUG  ROC  GHO  DRA  DAR  STE  FAI
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 0.0, 1.0, 0.5, 1.0,
    1.0, 0.5, 0.5, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 0.5, 1.0, 2.0, 1.0,
    1.0, 2.0, 0.5, 1.0, 0.5, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 1.0, 1.0,
    1.0, 1.0, 2.0, 0.5, 0.5, 1.0, 1.0, 1.0, 0.0, 2.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0,
    1.0, 0.5, 2.0, 1.0, 0.5, 1.0, 1.0, 0.5, 2.0, 0.5, 1.0, 0.5, 2.0, 1.0, 0.5, 1.0, 0.5, 1.0,
    1.0, 0.5, 0.5, 1.0, 2.0, 0.5, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0,
    2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 0.5, 0.5, 0.5, 2.0, 0.0, 1.0, 2.0, 2.0, 0.5,
    1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 0.5, 0.5, 1.0, 1.0, 1.0, 0.5, 0.5, 1.0, 1.0, 0.0, 2.0,
    1.0, 2.0, 1.0, 2.0, 0.5, 1.0, 1.0, 2.0, 1.0, 0.0, 1.0, 0.5, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0,
    1.0, 1.0, 1.0, 0.5, 2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 1.0, 1.0, 0.5, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 0.0, 0.5, 1.0,
    1.0, 0.5, 1.0, 1.0, 2.0, 1.0, 0.5, 0.5, 1.0, 0.5, 2.0, 1.0, 1.0, 0.5, 1.0, 2.0, 0.5, 0.5,
    1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 0.5, 2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0,
    0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 0.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 0.5,
    1.0, 0.5, 0.5, 0.5, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 0.5, 2.0,
    1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 0.5, 1.0,
];
*/

const TYPE_EFFECTIVENESS: [f32; 324] = [
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 0.0, 1.0, 0.5, 1.0, 1.0,
    0.5, 0.5, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 0.5, 1.0, 2.0, 1.0, 1.0, 2.0,
    0.5, 1.0, 0.5, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0,
    0.5, 0.5, 1.0, 1.0, 1.0, 0.0, 2.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 0.5, 2.0, 1.0,
    0.5, 1.0, 1.0, 0.5, 2.0, 0.5, 1.0, 0.5, 2.0, 1.0, 0.5, 1.0, 0.5, 1.0, 1.0, 0.5, 0.5, 1.0, 2.0,
    0.5, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0,
    1.0, 0.5, 1.0, 0.5, 0.5, 0.5, 2.0, 0.0, 1.0, 2.0, 2.0, 0.5, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0,
    0.5, 0.5, 1.0, 1.0, 1.0, 0.5, 0.5, 1.0, 1.0, 0.0, 2.0, 1.0, 2.0, 1.0, 2.0, 0.5, 1.0, 1.0, 2.0,
    1.0, 0.0, 1.0, 0.5, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 0.5, 2.0, 1.0, 2.0, 1.0, 1.0,
    1.0, 1.0, 2.0, 0.5, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0,
    0.5, 1.0, 1.0, 1.0, 1.0, 0.0, 0.5, 1.0, 1.0, 0.5, 1.0, 1.0, 2.0, 1.0, 0.5, 0.5, 1.0, 0.5, 2.0,
    1.0, 1.0, 0.5, 1.0, 2.0, 0.5, 0.5, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 0.5, 2.0, 1.0, 2.0,
    1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0,
    2.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    2.0, 1.0, 0.5, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0,
    0.5, 1.0, 0.5, 1.0, 0.5, 0.5, 0.5, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0,
    0.5, 2.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 0.5,
    1.0,
];

fn type_effectiveness(a: Type, d: Type) -> f32 {
    TYPE_EFFECTIVENESS[(a as usize) * 18 + d as usize]
}

fn damage_calc(
    state: &State,
    user: usize,
    target: usize,
    typ: Type,
    category: Category,
    power: u16,
) -> Vec<f32> {
    let user = state[user].as_ref().unwrap();
    let target = state[target].as_ref().unwrap();
    let stab = match user.typ {
        (t, _) | (_, Some(t)) if t == typ => 1.5,
        (_, _) => 1.,
    };
    let typ = type_effectiveness(typ, target.typ.0)
        * match target.typ.1 {
            None => 1.,
            Some(t) => type_effectiveness(typ, t),
        };
    let (a, d) = match category {
        Category::Physical => (user.stats.atk as f32, target.stats.def as f32),
        Category::Special => (user.stats.spa as f32, target.stats.spd as f32),
        Category::Status => panic!(),
    };
    let base_damage: f32 =
        half_down(((((2. * (user.level as f32)) / 5. + 2.) * (power as f32) * a / d) / 50. + 2.));
    //let crit_damage = (base_damage * 1.5).floor();
    let mut output = Vec::new();
    for random in 85..101 {
        let after_random = ((base_damage * random as f32) / 100.).floor();
        let after_stab = half_down(after_random * stab);
        let after_type = (after_stab * typ).floor();
        output.push(after_type);
    }
    output
}

struct Pokemon {
    stats: &'static Stats,
    typ: (Type, Option<Type>),
    /*tera_type: Type,
    moves: (
        Box<dyn Move>,
        Option<Box<dyn Move>>,
        Option<Box<dyn Move>>,
        Option<Box<dyn Move>>,
    ),
    condition: Option<Status>,
    current_hp: [f32; 512],*/
    level: u8,
}

type State = [Option<Pokemon>; 12];

const ABOMASNOW_STATS: Stats = Stats {
    hp: 363,
    atk: 198,
    def: 186,
    spa: 311,
    spd: 206,
    spe: 178,
};

fn main() {
    let state: State = [
        Some(Pokemon {
            stats: &ABOMASNOW_STATS,
            typ: (Type::Grass, Some(Type::Ice)),
            level: 100,
        }),
        Some(Pokemon {
            stats: &ABOMASNOW_STATS,
            typ: (Type::Grass, Some(Type::Ice)),
            level: 100,
        }),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ];
    for x in damage_calc(&state, 0, 1, Type::Ice, Category::Special, 110) {
        println!("{}", x);
    }
}
