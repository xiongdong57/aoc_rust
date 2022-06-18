use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap},
};

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct Effect {
    turns: i64,
    dmg: i64,
    heal: i64,
    armor: i64,
    mana: i64,
}

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct Spell {
    name: &'static str,
    mana: i64,
    delay: i64,
    effect: Effect,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone)]
struct State {
    effects: BTreeMap<&'static str, Effect>,
    hp: i64,
    mana: i64,
    boss_hp: i64,
}

impl State {
    fn ef_dmg(&self) -> i64 {
        self.effects.values().map(|e| e.dmg).sum()
    }

    fn ef_heal(&self) -> i64 {
        self.effects.values().map(|e| e.heal).sum()
    }
    fn ef_armor(&self) -> i64 {
        self.effects.values().map(|e| e.armor).sum()
    }
    fn ef_mana(&self) -> i64 {
        self.effects.values().map(|e| e.mana).sum()
    }
    fn timer(&mut self) {
        let mut to_remove = Vec::new();
        for (k, v) in self.effects.iter_mut() {
            v.turns -= 1;
            if v.turns == 0 {
                to_remove.push(*k);
            }
        }
        for k in &to_remove {
            self.effects.remove(*k);
        }
    }
}

pub fn solve(hard: i64) -> i64 {
    let boss_hp = 58;
    let boss_dmg = 9;
    let mut spells = Vec::new();
    spells.push(Spell {
        name: "Magic Missile",
        mana: 53,
        delay: 0,
        effect: Effect {
            turns: 1,
            dmg: 4,
            heal: 0,
            armor: 0,
            mana: 0,
        },
    });
    spells.push(Spell {
        name: "Drain",
        mana: 73,
        delay: 0,
        effect: Effect {
            turns: 1,
            dmg: 2,
            heal: 2,
            armor: 0,
            mana: 0,
        },
    });
    spells.push(Spell {
        name: "Shield",
        mana: 113,
        delay: 1,
        effect: Effect {
            turns: 6,
            dmg: 0,
            heal: 0,
            armor: 7,
            mana: 0,
        },
    });
    spells.push(Spell {
        name: "Poison",
        mana: 173,
        delay: 1,
        effect: Effect {
            turns: 6,
            dmg: 3,
            heal: 0,
            armor: 0,
            mana: 0,
        },
    });
    spells.push(Spell {
        name: "Recharge",
        mana: 229,
        delay: 1,
        effect: Effect {
            turns: 5,
            dmg: 0,
            heal: 0,
            armor: 0,
            mana: 101,
        },
    });
    let mut states = BinaryHeap::new();
    let mut visited = BTreeSet::new();
    let initial = State {
        effects: BTreeMap::new(),
        hp: 50,
        mana: 500,
        boss_hp,
    };
    visited.insert(initial.clone());
    states.push((Reverse(0), initial));
    while let Some((Reverse(mana), state)) = states.pop() {
        for spell in &spells {
            // player's turn
            let mut state = state.clone();
            state.hp -= hard;
            if state.hp <= 0 {
                continue;
            }

            state.hp += state.ef_heal();
            state.boss_hp -= state.ef_dmg();
            state.mana += state.ef_mana();
            state.timer();

            if state.boss_hp <= 0 {
                return mana;
            }

            if state.effects.contains_key(spell.name) {
                continue;
            }

            // use new spell
            state.mana -= spell.mana;
            if state.mana < 0 {
                continue;
            }
            if spell.delay == 0 {
                state.hp += spell.effect.heal;
                state.boss_hp -= spell.effect.dmg;
                state.mana += spell.effect.mana;
            } else {
                state.effects.insert(spell.name, spell.effect);
            }

            if state.boss_hp <= 0 {
                return mana + spell.mana;
            }

            // boss's turn
            state.hp += state.ef_heal();
            state.boss_hp -= state.ef_dmg();
            state.mana += state.ef_mana();
            state.timer();

            if state.boss_hp <= 0 {
                return mana + spell.mana;
            }

            state.hp -= 1.max(boss_dmg - state.ef_armor());
            if state.hp <= 0 {
                continue;
            }

            if !visited.contains(&state) {
                visited.insert(state.clone());
                states.push((Reverse(mana + spell.mana), state));
            }
        }
    }
    panic!("no solution");
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_result() {
        assert_eq!(solve(0), 1269);
    }


    #[test]
    fn part_b_result() {
        assert_eq!(solve(1), 1309);
    }
}
