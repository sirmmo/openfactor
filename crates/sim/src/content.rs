use crate::{ItemId, Pos};

/// Everything that defines "the game". The sim has no built-in items or recipes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Content {
    pub items: Vec<String>,
    pub recipes: Vec<Recipe>,
    /// Ticks to mine one item.
    pub miner_ticks: u32,
    /// Distance a belt item moves per tick, in 1/`TILE` of a tile.
    pub belt_speed: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Recipe {
    pub inputs: Vec<(ItemId, u32)>,
    pub outputs: Vec<(ItemId, u32)>,
    pub ticks: u32,
}

/// A rectangle of minable tiles.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Patch {
    pub origin: Pos,
    pub width: u32,
    pub height: u32,
    pub item: ItemId,
    pub amount_per_tile: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WorldGen {
    pub patches: Vec<Patch>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContentError {
    ZeroBeltSpeed,
    ZeroMinerTicks,
    ZeroRecipeTicks { recipe: usize },
    UnknownItem { item: ItemId },
    /// A recipe input must be positive and listed once.
    BadRecipeInput { recipe: usize },
    EmptyPatch { patch: usize },
}

impl Content {
    /// A minimal pack for tests and the demo: ore -> plate, 2 plates -> widget.
    pub fn example() -> Self {
        Content {
            items: vec!["ore".into(), "plate".into(), "widget".into()],
            recipes: vec![
                Recipe {
                    inputs: vec![(ItemId(0), 1)],
                    outputs: vec![(ItemId(1), 1)],
                    ticks: 30,
                },
                Recipe {
                    inputs: vec![(ItemId(1), 2)],
                    outputs: vec![(ItemId(2), 1)],
                    ticks: 60,
                },
            ],
            miner_ticks: 30,
            belt_speed: 16,
        }
    }

    pub fn validate(&self, worldgen: &WorldGen) -> Result<(), ContentError> {
        if self.belt_speed == 0 {
            return Err(ContentError::ZeroBeltSpeed);
        }
        if self.miner_ticks == 0 {
            return Err(ContentError::ZeroMinerTicks);
        }
        let check_item = |item: ItemId| {
            if usize::from(item.0) < self.items.len() {
                Ok(())
            } else {
                Err(ContentError::UnknownItem { item })
            }
        };
        for (index, recipe) in self.recipes.iter().enumerate() {
            if recipe.ticks == 0 {
                return Err(ContentError::ZeroRecipeTicks { recipe: index });
            }
            for (i, &(item, amount)) in recipe.inputs.iter().enumerate() {
                check_item(item)?;
                let repeated = recipe.inputs[..i].iter().any(|&(other, _)| other == item);
                if amount == 0 || repeated {
                    return Err(ContentError::BadRecipeInput { recipe: index });
                }
            }
            for &(item, _) in &recipe.outputs {
                check_item(item)?;
            }
        }
        for (index, patch) in worldgen.patches.iter().enumerate() {
            check_item(patch.item)?;
            if patch.amount_per_tile == 0 {
                return Err(ContentError::EmptyPatch { patch: index });
            }
        }
        Ok(())
    }
}
