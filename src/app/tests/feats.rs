use crate::app::App;
use crate::game::character::feats::apply_feat_effect;

#[test]
fn tough_feat_applies_correct_hp_bonus() {
    let mut app = App::new();

    // Verify initial HP
    let initial_max_hp = app.player.max_hp;

    // Grant Tough feat
    if let Some(tough_feat) = app.feat_defs.get("tough") {
        apply_feat_effect(&mut app.player, tough_feat);

        // Check that HP increased by 2 * level (level 1 = +2 HP)
        assert_eq!(app.player.max_hp, initial_max_hp + 2);
        assert_eq!(app.player.current_hp, initial_max_hp + 2);

        println!("Tough feat applied correctly: +2 HP");
    } else {
        panic!("Tough feat not found in asset definitions");
    }
}

#[test]
fn grant_feat_method_works() {
    let mut app = App::new();

    // Initially should have no feats
    assert!(app.player.feats.is_empty());

    // Grant Tough feat
    app.grant_feat("tough");

    // Check that feat is in player's feats list
    assert!(app.player.feats.contains(&"tough".to_string()));

    println!("grant_feat method works correctly");
}
