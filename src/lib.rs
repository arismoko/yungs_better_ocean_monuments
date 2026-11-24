//! YUNG's Better Ocean Monuments for Sunrise. The structures, templates and loot are data; the
//! one piece of code keeps the monument's own tridents from despawning
//! (`PersistentTridentMixin` in the original).
//!
//! The monument's templates hold thrown tridents whose owner is YUNG's own UUID, which no player
//! has. A trident stuck in a block despawns once its `life` reaches 1200 ticks
//! (`AbstractArrow.tickDespawn`); the original skips that count for these tridents. A player who
//! picks one up and throws it becomes its owner, so a thrown trident despawns as usual.
//!
//! | Moment | Where | What it does |
//! |---|---|---|
//! | `entity_tick_listened` | this module | asks the game to tell the entity ticks |
//! | `entity_tick_pre` | [`Mod::entity_tick_pre`] | sets a template trident's `life` back to 0 before it can despawn |

use mod_api::{
    self, Compound, EntityTickListenedEvent, EntityTickPreEvent, Hooks, HostSide, ServerHost, Tag,
};

/// `UUID e624cdc1-c238-4dde-9f22-1f76b5123ce8`, the owner the templates give their tridents, as
/// the four ints of its saved `Owner` array.
const TEMPLATE_OWNER: [i32; 4] = [
    0xe624_cdc1_u32 as i32,
    0xc238_4dde_u32 as i32,
    0x9f22_1f76_u32 as i32,
    0xb512_3ce8_u32 as i32,
];

/// `life` at which a template trident is set back to 0: well before `AbstractArrow.tickDespawn`
/// removes it at 1200, so a trident never reaches it.
const RESET_AT: i16 = 1100;

#[derive(Default)]
struct Mod;

#[mod_api::sunrise_mod]
impl Hooks for Mod {
    fn entity_tick_listened(&self, _host: HostSide<'_>, event: &mut EntityTickListenedEvent) {
        event.listened = true;
    }

    /// `PersistentTridentMixin`: a template trident's despawn count never runs out.
    fn entity_tick_pre(&self, host: HostSide<'_>, event: &mut EntityTickPreEvent<'_>) {
        let HostSide::Server(host) = host else {
            return;
        };
        for (&id, &kind) in event.entities.iter().zip(event.kinds) {
            if kind == "minecraft:trident" {
                keep(host, id);
            }
        }
    }
}

fn keep(host: &mut dyn ServerHost, id: u32) {
    let Some(entity) = host.entity(id) else {
        return;
    };
    let Some(data) = entity.nbt(host) else {
        return;
    };
    let data = Tag::Compound(data);
    if !due(&data) {
        return;
    }
    let mut reset = Compound::new();
    reset.insert("life".into(), Tag::Short(0));
    entity.merge_nbt(host, reset);
}

/// Whether a trident's saved data is a template trident's whose `life` needs setting back.
fn due(data: &Tag) -> bool {
    let owner =
        matches!(data.get("Owner"), Some(Tag::IntArray(owner)) if owner[..] == TEMPLATE_OWNER);
    let life = match data.get("life") {
        Some(Tag::Short(life)) => *life,
        _ => 0,
    };
    owner && life >= RESET_AT
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trident(owner: Vec<i32>, life: i16) -> Tag {
        Tag::compound([("Owner", Tag::IntArray(owner)), ("life", Tag::Short(life))])
    }

    #[test]
    fn a_template_trident_is_reset_near_its_despawn() {
        assert!(due(&trident(TEMPLATE_OWNER.to_vec(), 1100)));
        assert!(!due(&trident(TEMPLATE_OWNER.to_vec(), 1099)));
    }

    #[test]
    fn a_players_trident_despawns_as_usual() {
        assert!(!due(&trident(vec![1, 2, 3, 4], 1199)));
        assert!(!due(&Tag::compound([("life", Tag::Short(1199))])));
    }

    #[test]
    fn the_owner_is_yungs_uuid() {
        let uuid = 0xe624cdc1_c2384dde_9f221f76_b5123ce8_u128;
        let ints = [96, 64, 32, 0].map(|shift| (uuid >> shift) as u32 as i32);
        assert_eq!(ints, TEMPLATE_OWNER);
    }
}
