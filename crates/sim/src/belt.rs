//! Belt lines: a chain of belt tiles stored as the gaps between items.
//!
//! Moving a line only changes a gap or two, so a tick costs about the same
//! however many items are on it. Items only ever enter at the tail: anything
//! feeding a belt from the side (a junction or a machine) splits the line there.

use crate::{EntityId, ItemId, Pos};
use std::collections::VecDeque;

/// Length of one belt tile in position units.
pub const TILE: u32 = 256;
/// Minimum distance between two items on a belt.
pub const ITEM_SPACING: u32 = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Target {
    Line(u32),
    Entity(EntityId),
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct BeltItem {
    pub item: ItemId,
    /// Distance to the next item downstream, or to the line's end for the head.
    pub gap: u32,
}

#[derive(Clone, Debug)]
pub(crate) struct Line {
    /// Tiles from upstream to downstream.
    pub tiles: Vec<Pos>,
    /// Entity of the most downstream tile; a stable identity for arbitration.
    pub head_entity: EntityId,
    /// Index 0 is the most downstream item.
    pub items: VecDeque<BeltItem>,
    /// Distance from the last item to the start of the line.
    pub tail_free: u32,
    /// Number of leading items that cannot move.
    pub compressed: usize,
    pub target: Option<Target>,
    /// Round-robin state for junctions feeding this line.
    pub last_feeder: Option<EntityId>,
}

impl Line {
    pub fn length(&self) -> u32 {
        self.tiles.len() as u32 * TILE
    }

    /// Moves items `speed` units, closing gaps from the head backwards.
    pub fn advance(&mut self, speed: u32) {
        let mut remaining = speed;
        while remaining > 0 && self.compressed < self.items.len() {
            let min_gap = if self.compressed == 0 { 0 } else { ITEM_SPACING };
            let item = &mut self.items[self.compressed];
            let take = remaining.min(item.gap.saturating_sub(min_gap));
            item.gap -= take;
            remaining -= take;
            if item.gap <= min_gap {
                self.compressed += 1;
            }
        }
        if !self.items.is_empty() {
            self.tail_free += speed - remaining;
        }
    }

    /// The item waiting at the end of the line, if any.
    pub fn head_item(&self) -> Option<ItemId> {
        self.items.front().filter(|i| i.gap == 0).map(|i| i.item)
    }

    pub fn can_accept(&self) -> bool {
        self.tail_free >= ITEM_SPACING
    }

    pub fn pop_head(&mut self) {
        self.items.pop_front();
        self.compressed = 0;
    }

    pub fn push_tail(&mut self, item: ItemId) {
        self.items.push_back(BeltItem {
            item,
            gap: self.tail_free,
        });
        self.tail_free = 0;
    }
}
