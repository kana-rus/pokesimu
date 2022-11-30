use crate::pokemon::Pokemon;


pub(crate) struct SingleBattle {
    environment:    Environment,
    my_pokemons:    [Pokemon; 3],
    enemy_pokemons: [Pokemon; 3],
    battlers_idxs:  (usize, usize),

    effectors: todo!(/* WhileSomeTurns な Effector のカウントをどこで管理するか .... */),
}
// regular_effectors:
// - regular_ailments of Pokemons
// - environment of SingleBattle

pub(crate) struct Environment {
    weather: Weather,
    field:   Field,
}
    enum Weather {
        Normal,
    }
    enum Field { // グラスフィールド とか まきびし とか
        Normal,
    }
