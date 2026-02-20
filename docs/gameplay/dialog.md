# Dialog Trees

## Dialog Tree TOML Schema

One file per NPC per region: `assets/regions/<slug>/dialog/<npc-id>.toml`

```toml
npc_id = "guard_kael"

[[nodes]]
id   = "root"
text = "Halt! State your business, traveller."

[[nodes.choices]]
text      = "I seek passage through the gate."
condition = ""                            # empty = always visible
effect    = []
next      = "passage_asked"

[[nodes.choices]]
text      = "I'm here to speak with the Captain."
condition = "flag:has_captains_letter"
effect    = []
next      = "captain_intro"

[[nodes.choices]]
text      = "[Intimidate DC 14] Stand aside."
condition = ""
effect    = []
next      = "intimidate_check"           # resolved in code via skill check


[[nodes]]
id   = "passage_asked"
text = "Aye, the gate's open. Don't cause trouble."
effect = [{ set_flag = "met_kael" }]     # effects fire when this node is reached
[[nodes.choices]]
text = "Thank you."
next = "END"


[[nodes]]
id   = "intimidate_check"
text = "__SKILL_CHECK__"   # special sentinel: game resolves Intimidation vs DC 14
                           # on success → next = "intimidate_success"
                           # on failure → next = "intimidate_fail"
skill   = "intimidation"
dc      = 14
on_pass = "intimidate_success"
on_fail = "intimidate_fail"

[[nodes]]
id   = "intimidate_success"
text = "He steps aside, clearly unnerved."
effect = [{ delta_counter = { key = "faction_city_guard_rep", delta = -2 } }]
[[nodes.choices]]
text = "Good."
next = "END"

[[nodes]]
id   = "intimidate_fail"
text = "The guard grabs his halberd. 'That's it, you're under arrest!'"
effect = [{ set_flag = "combat_kael_triggered" }]
[[nodes.choices]]
text = "(Prepare for combat)"
next = "END"
```

---

## Dialog Evaluator (`src/game/story/dialog.rs`)

`fn advance(tree: &DialogTree, node_id: &str, world: &WorldState) -> ResolvedNode`

1. Look up node by `node_id`.
2. Filter choices: only include choices where `condition` evaluates true against
   `WorldState`. Empty condition = always shown.
3. Handle `__SKILL_CHECK__` sentinels: resolve the skill check and return the
   appropriate `on_pass` / `on_fail` branch.
4. Apply `effect` list for the node that was *entered*.
5. Return `ResolvedNode { text, choices: Vec<(label, next_node_id)> }` to the UI.

`END` as `next` closes the dialog screen and returns to `AppState::WorldMap`.

---

## Dialog UI

`src/ui/screens/dialog.rs`:
- Centered popup, 60 % terminal width
- Top: NPC name
- Middle: current node text (word-wrapped)
- Bottom: numbered choice list `[1] ...`, `[2] ...`
- Press `1`–`9` to select; dialog evaluator advances to next node

See → [architecture/ui-layer.md](../architecture/ui-layer.md)
