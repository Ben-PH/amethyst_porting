use crate::ametheed::assets::timing::Time;
use crate::ametheed::assets::system_desc::SystemDesc;
use crate::ametheed::assets::loader::Loader;
use specs::prelude::*;
use crate::ametheed::assets::FormatValue;
use crate::ametheed::error::Error;
use std::{sync::Arc, time::Instant};

use derive_new::new;

/// The `Reload` trait provides a method which checks if an asset needs to be reloaded.
pub trait Reload<D>: ReloadClone<D> + Send + Sync + 'static {
    /// Checks if a reload is necessary.
    fn needs_reload(&self) -> bool;
    /// Returns the asset name.
    fn name(&self) -> String;
    /// Returns the format name.
    fn format(&self) -> &'static str;
    /// Reloads the asset.
    fn reload(self: Box<Self>) -> Result<FormatValue<D>, Error>;
}

pub trait ReloadClone<D> {
    fn cloned(&self) -> Box<dyn Reload<D>>;
}

impl<D: 'static, T> ReloadClone<D> for T
where
    T: Clone + Reload<D>,
{
    fn cloned(&self) -> Box<dyn Reload<D>> {
        Box::new(self.clone())
    }
}

impl<D: 'static> Clone for Box<dyn Reload<D>> {
    fn clone(&self) -> Self {
        self.cloned()
    }
}


/// An ECS resource which allows to configure hot reloading.
///
/// ## Examples
///
/// ```
/// # use amethyst_assets::HotReloadStrategy;
/// # use amethyst_core::ecs::{World, WorldExt};
/// #
/// let mut world = World::new();
/// // Assets will be reloaded every two seconds (in case they changed)
/// world.insert(HotReloadStrategy::every(2));
/// ```
#[derive(Clone, Debug)]
pub struct HotReloadStrategy {
    inner: HotReloadStrategyInner,
}

impl HotReloadStrategy {
    /// Causes hot reloads every `n` seconds.
    pub fn every(n: u8) -> Self {
        use std::u64::MAX;

        HotReloadStrategy {
            inner: HotReloadStrategyInner::Every {
                interval: n,
                last: Instant::now(),
                frame_number: MAX,
            },
        }
    }

    /// This allows to use `trigger` for hot reloading.
    pub fn when_triggered() -> Self {
        use std::u64::MAX;

        HotReloadStrategy {
            inner: HotReloadStrategyInner::Trigger {
                triggered: false,
                frame_number: MAX,
            },
        }
    }

    /// Never do any hot-reloading.
    pub fn never() -> Self {
        HotReloadStrategy {
            inner: HotReloadStrategyInner::Never,
        }
    }

    /// The frame after calling this, all changed assets will be reloaded.
    /// Doesn't do anything if the strategy wasn't created with `when_triggered`.
    pub fn trigger(&mut self) {
        if let HotReloadStrategyInner::Trigger {
            ref mut triggered, ..
        } = self.inner
        {
            *triggered = true;
        }
    }

    /// Crate-internal method to check if reload is necessary.
    /// `reload_counter` is a per-storage value which is only used
    /// for and by this method.
    pub(crate) fn needs_reload(&self, current_frame: u64) -> bool {
        match self.inner {
            HotReloadStrategyInner::Every { frame_number, .. } => frame_number == current_frame,
            HotReloadStrategyInner::Trigger { frame_number, .. } => frame_number == current_frame,
            HotReloadStrategyInner::Never => false,
        }
    }
}

impl Default for HotReloadStrategy {
    fn default() -> Self {
        HotReloadStrategy::every(1)
    }
}

#[derive(Clone, Debug)]
enum HotReloadStrategyInner {
    Every {
        interval: u8,
        last: Instant,
        frame_number: u64,
    },
    Trigger {
        triggered: bool,
        frame_number: u64,
    },
    Never,
}

/// Builds a `HotReloadSystem`.
#[derive(Debug, new)]
pub struct HotReloadSystemDesc {
    /// The `HotReloadStrategy`.
    pub strategy: HotReloadStrategy,
}

impl<'a, 'b> SystemDesc<'a, 'b, HotReloadSystem> for HotReloadSystemDesc {
    fn build(self, world: &mut World) -> HotReloadSystem {
        <HotReloadSystem as System<'_>>::SystemData::setup(world);

        world.insert(self.strategy);
        world.fetch_mut::<Loader>().set_hot_reload(true);

        HotReloadSystem::new()
    }
}

/// System for updating `HotReloadStrategy`.
#[derive(Debug, new)]
pub struct HotReloadSystem;

impl<'a> System<'a> for HotReloadSystem {
    type SystemData = (Read<'a, Time>, Write<'a, HotReloadStrategy>);

    fn run(&mut self, (time, mut strategy): Self::SystemData) {
        #[cfg(feature = "profiler")]
        profile_scope!("hot_reload_system");

        match strategy.inner {
            HotReloadStrategyInner::Trigger {
                ref mut triggered,
                ref mut frame_number,
            } => {
                if *triggered {
                    *frame_number = time.frame_number() + 1;
                }
                *triggered = false;
            }
            HotReloadStrategyInner::Every {
                interval,
                ref mut last,
                ref mut frame_number,
            } => {
                if last.elapsed().as_secs() > u64::from(interval) {
                    *frame_number = time.frame_number() + 1;
                    *last = Instant::now();
                }
            }
            HotReloadStrategyInner::Never => {}
        }
    }
}
