use crate::{
    ai::{apply_energy_cost, MyTurn},
    attack_system, camera, go_down_stairs,
    gui::{mqui::ItemWindowMode, TargettingMode},
    menus::MenuType,
    ranged_combat::ranged_aim::select_nearest_target_pos,
    screen_manager::{self, MANAGER},
    statistics::Pools,
    BlocksTiles, BlocksVisibility, Door, EquipmentSlot, Equippable, Equipped, HasMoved,
    InContainer, Item, RangedTargetting, RangedWeapon, Renderable, TileType, WantsToPickupItem,
    WantsToRest,
};
use bracket_lib::prelude::*;
use macroquad::input::{get_keys_down, get_keys_pressed, KeyCode};

use super::{AttackSystem, Entity, FoV, Map, Name, Position, ProgramState, State};
use std::{
    cmp::{max, min},
    sync::LazyLock,
};

pub struct Player {}
