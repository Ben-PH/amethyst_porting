use std::sync::Weak;
use crate::ametheed::assets::reload::Reload;
use crate::ametheed::error::Error;
use crate::ametheed::assets::progress::Tracker;
use crate::ametheed::assets::FormatValue;
use std::sync::atomic::{AtomicUsize, Ordering};
use crossbeam_queue::SegQueue;
use derivative::Derivative;
use std::sync::Arc;
use super::Asset;
use specs::Component;
use specs::prelude::*;
use std::marker::PhantomData;


/// An `Allocator`, holding a counter for producing unique IDs.
#[derive(Debug, Default)]
pub struct Allocator {
    store_count: AtomicUsize,
}

impl Allocator {
    /// Produces a new id.
    pub fn next_id(&self) -> usize {
        self.store_count.fetch_add(1, Ordering::Relaxed)
    }
}

pub(crate) enum Processed<A: Asset> {
    NewAsset {
        data: Result<FormatValue<A::Data>, Error>,
        handle: Handle<A>,
        name: String,
        tracker: Box<dyn Tracker>,
    },
    HotReload {
        data: Result<FormatValue<A::Data>, Error>,
        handle: Handle<A>,
        name: String,
        old_reload: Box<dyn Reload<A::Data>>,
    },
}

/// Completion status, returned by `ProgressCounter::complete`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Completion {
    /// Loading is complete
    Complete,
    /// Some asset loads have failed
    Failed,
    /// Still loading assets
    Loading,
}

/// Returned by processor systems, describes the loading state of the asset.
pub enum ProcessingState<A>
where
    A: Asset,
{
    /// Asset is not fully loaded yet, need to wait longer
    Loading(A::Data),
    /// Asset has finished loading, can now be inserted into storage and tracker notified
    Loaded(A),
}

/// An asset storage, storing the actual assets and allocating
/// handles to them.
pub struct AssetStorage<A: Asset> {
    assets: VecStorage<(A, u32)>,
    bitset: BitSet,
    handles: Vec<Handle<A>>,
    handle_alloc: Allocator,
    pub(crate) processed: Arc<SegQueue<Processed<A>>>,
    reloads: Vec<(WeakHandle<A>, Box<dyn Reload<A::Data>>)>,
    unused_handles: SegQueue<Handle<A>>,
}


/// A handle to an asset. This is usually what the
/// user deals with, the actual asset (`A`) is stored
/// in an `AssetStorage`.
#[derive(Derivative)]
#[derivative(
    Clone(bound = ""),
    Eq(bound = ""),
    Hash(bound = ""),
    PartialEq(bound = ""),
    Debug(bound = "")
)]
pub struct Handle<A: ?Sized> {
    id: Arc<u32>,
    #[derivative(Debug = "ignore")]
    marker: PhantomData<A>,
}

impl<A> Handle<A> {
    /// Return the 32 bit id of this handle.
    pub fn id(&self) -> u32 {
        *self.id.as_ref()
    }

    /// Downgrades the handle and creates a `WeakHandle`.
    pub fn downgrade(&self) -> WeakHandle<A> {
        let id = Arc::downgrade(&self.id);

        WeakHandle {
            id,
            marker: PhantomData,
        }
    }

    /// Returns `true` if this is the only handle to the asset its pointing at.
    fn is_unique(&self) -> bool {
        Arc::strong_count(&self.id) == 1
    }
}

impl<A> Component for Handle<A>
where
    A: Asset,
{
    type Storage = A::HandleStorage;
}

/// A weak handle, which is useful if you don't directly need the asset
/// like in caches. This way, the asset can still get dropped (if you want that).
#[derive(Derivative)]
#[derivative(Clone(bound = ""), Debug(bound = ""))]
pub struct WeakHandle<A> {
    id: Weak<u32>,
    #[derivative(Debug = "ignore")]
    marker: PhantomData<A>,
}

impl<A> WeakHandle<A> {
    /// Tries to upgrade to a `Handle`.
    #[inline]
    pub fn upgrade(&self) -> Option<Handle<A>> {
        self.id.upgrade().map(|id| Handle {
            id,
            marker: PhantomData,
        })
    }

    /// Returns `true` if the original handle is dead.
    #[inline]
    pub fn is_dead(&self) -> bool {
        self.id.upgrade().is_none()
    }
}
