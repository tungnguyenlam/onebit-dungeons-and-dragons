/// Dialog tree evaluator.
use crate::{
    data::types::{DialogChoice, DialogEffect, DialogNode, DialogTree},
    game::story::world_state::WorldState,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedChoice {
    pub text: String,
    pub next: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedNode {
    pub id: String,
    pub text: String,
    pub choices: Vec<ResolvedChoice>,
}

/// Resolve current node for rendering and apply node-enter effects.
pub fn resolve(tree: &DialogTree, node_id: &str, world: &mut WorldState) -> Option<ResolvedNode> {
    let node = find_node(tree, node_id)?;
    apply_effects(&node.effect, world);

    let choices = node
        .choices
        .iter()
        .filter(|c| c.condition.trim().is_empty() || world.evaluate(&c.condition))
        .map(|c| ResolvedChoice {
            text: c.text.clone(),
            next: c.next.clone(),
        })
        .collect();

    Some(ResolvedNode {
        id: node.id.clone(),
        text: node.text.clone(),
        choices,
    })
}

/// Apply selected choice and return next node id (or "END").
pub fn choose(
    tree: &DialogTree,
    node_id: &str,
    choice_index: usize,
    world: &mut WorldState,
) -> Option<String> {
    let node = find_node(tree, node_id)?;

    if node.text == "__SKILL_CHECK__" {
        let skill = node.skill.as_deref().unwrap_or("unknown");
        let dc = node.dc.unwrap_or(10) as i32;
        let check_val = world.counter(&format!("skill:{skill}"));
        return if check_val >= dc {
            node.on_pass.clone()
        } else {
            node.on_fail.clone()
        };
    }

    let visible: Vec<&DialogChoice> = node
        .choices
        .iter()
        .filter(|c| c.condition.trim().is_empty() || world.evaluate(&c.condition))
        .collect();
    let choice = visible.get(choice_index)?;
    apply_effects(&choice.effect, world);
    Some(choice.next.clone())
}

fn find_node<'a>(tree: &'a DialogTree, node_id: &str) -> Option<&'a DialogNode> {
    tree.nodes.iter().find(|n| n.id == node_id)
}

fn apply_effects(effects: &[DialogEffect], world: &mut WorldState) {
    for effect in effects {
        match effect {
            DialogEffect::SetFlag { set_flag } => world.set_flag(set_flag.clone()),
            DialogEffect::ClearFlag { clear_flag } => world.clear_flag(clear_flag.clone()),
            DialogEffect::DeltaCounter { delta_counter } => {
                world.delta_counter(delta_counter.key.clone(), delta_counter.delta);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::types::{CounterDelta, DialogChoice, DialogEffect, DialogNode, DialogTree};

    fn tree() -> DialogTree {
        DialogTree {
            npc_id: "npc".into(),
            nodes: vec![
                DialogNode {
                    id: "root".into(),
                    text: "Hello".into(),
                    effect: vec![],
                    choices: vec![
                        DialogChoice {
                            text: "A".into(),
                            condition: "".into(),
                            effect: vec![DialogEffect::SetFlag {
                                set_flag: "talked".into(),
                            }],
                            next: "next".into(),
                        },
                        DialogChoice {
                            text: "B".into(),
                            condition: "flag:hidden".into(),
                            effect: vec![],
                            next: "next".into(),
                        },
                    ],
                    skill: None,
                    dc: None,
                    on_pass: None,
                    on_fail: None,
                },
                DialogNode {
                    id: "check".into(),
                    text: "__SKILL_CHECK__".into(),
                    effect: vec![],
                    choices: vec![],
                    skill: Some("intimidation".into()),
                    dc: Some(12),
                    on_pass: Some("pass".into()),
                    on_fail: Some("fail".into()),
                },
                DialogNode {
                    id: "next".into(),
                    text: "Next".into(),
                    effect: vec![DialogEffect::DeltaCounter {
                        delta_counter: CounterDelta {
                            key: "rep".into(),
                            delta: 1,
                        },
                    }],
                    choices: vec![],
                    skill: None,
                    dc: None,
                    on_pass: None,
                    on_fail: None,
                },
            ],
        }
    }

    #[test]
    fn resolve_filters_choices() {
        let mut ws = WorldState::new();
        let r = resolve(&tree(), "root", &mut ws).unwrap();
        assert_eq!(r.choices.len(), 1);
        assert_eq!(r.choices[0].text, "A");
    }

    #[test]
    fn choose_applies_effects() {
        let mut ws = WorldState::new();
        let next = choose(&tree(), "root", 0, &mut ws).unwrap();
        assert_eq!(next, "next");
        assert!(ws.flag("talked"));
    }

    #[test]
    fn skill_check_branch_uses_world_counter() {
        let mut ws = WorldState::new();
        ws.set_counter("skill:intimidation", 13);
        let next = choose(&tree(), "check", 0, &mut ws).unwrap();
        assert_eq!(next, "pass");
    }
}
