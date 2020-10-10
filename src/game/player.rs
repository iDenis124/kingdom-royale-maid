use super::item::Items;
use super::roles::{Role, RoleName};
use super::Game;
use serenity::model::id::UserId;

pub struct Player {
    id: UserId,
    role: Box<(dyn Role + Send + Sync)>,
    alive: bool,
    items: Items, // count + item
}

impl Player {
    pub fn new(id: UserId, role: Box<(dyn Role + Send + Sync)>, watch_colour: String) -> Self {
        // PONDER: We may want to allow disabling certain items
        //         If we do, how should that be handled? Should we just pass a reference to the Game and ask it for enabled items?
        Self {
            id: id,
            role: role,
            alive: true,
            items: Items::new(),
        }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn set_dead(&mut self) {
        self.alive = false;

        // TODO: Discord related stuffs
    }

    pub fn give_item_to(&mut self, other: &mut Player, name: &str) {
        let my_item = self.items.get_item_mut(name);
        let their_item = other.items.get_item_mut(name);

        my_item.0 -= 1;
        their_item.0 += 1;
    }

    pub fn can_do_special_action(&self, game: &Game) -> bool {
        self.role.can_do_special_action(game)
    }

    pub fn act(&self, target: &mut Player, game: &mut Game) {
        self.role.act(target, game)
    }

    pub fn role_name(&self) -> RoleName {
        self.role.name()
    }
}
