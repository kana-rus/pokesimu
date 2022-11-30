use crate::battle::Battle;


pub(crate) struct Pokemon {
    name:      String,
    types:     (Type, Option<Type>),
    tera_type: Type,
    stats:     Stats,
    regular_ailments: Vec<RegularAilment>,
    special_ailemtnts: Vec<Effector>,
        // (ひるみ とか アンコール とか みやぶる とか やどりぎ とか ...)
        // ref: https://blog.mono0x.net/2011/12/19/
    moves:     [Option<Move>; 4],
    ability:   Effector,
    item:      Option<Effector>,
}

enum RegularAilment {
    Burn,
    Freeze,
    Paralysis,
    Poison,
    // BadPoison, // もうどく。SV であるんだっけ？
    Sleep,
    Confusion,
}

enum Type {
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
}

struct Stats {
    hp: u16,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    s: u8,
    evasion: u8,  // かいひ
    accuracy: u8, // めいちゅう
}

struct Effector {
    name:   String,
    timing: EffectTime,
    effect: fn(&mut Battle),
}
    enum EffectTime {
        OnAppear,
        OnBeforeAttacked,
        OnAfterAttacked,
        OnAttacking,
        OnTurnEnd,
        OnHPbeQuad,
        // Always,
        WhileSomeTurns(u8),
        // ...
    }
    // enum Effect {
    //     ToOnePokemon(fn(&mut Pokemon)),
    //     /**/ToAllEnemies(fn(&mut [Pokemon; 2])), // for double battle
    //     ToEnvironment(fn(&mut Environment)),
    // }

struct Move {
    name:   String,
    pp:     u8,
    target: MoveTarget,
    power:  Option<u8>,
    side_effect: Option<fn(&mut Battle)>,
    // power も side_effect も None なのは「はねる」くらいか
}
    enum MoveTarget {
        ToOnePokemon,
        /**/ToAllPokemons/*except me*/, // for double battle
        ToEnvironment,
    }