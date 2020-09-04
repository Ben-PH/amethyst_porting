use crate::ametheed::assets::SerializableFormat;
use crate::ametheed::assets::Format;
use crate::ametheed::assets::loader::Loader;
use crate::ametheed::assets::storage::AssetStorage;
use specs::prelude::*;
use crate::ametheed:: {
    error::Error,

    assets::progress::ProgressCounter,
    assets::storage::Handle,
    assets::Asset,

};

/// Prefab data container for a single entity
///
/// ### Type parameters:
///
/// - `T`: `PrefabData`
#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct PrefabEntity<T> {
    parent: Option<usize>,
    data: Option<T>,
}

impl<T> Default for PrefabEntity<T> {
    fn default() -> Self {
        PrefabEntity::new(None, None)
    }
}

impl<T> PrefabEntity<T> {
    /// New prefab entity
    pub fn new(parent: Option<usize>, data: Option<T>) -> Self {
        PrefabEntity { parent, data }
    }

    /// Set parent index
    pub fn set_parent(&mut self, parent: usize) {
        self.parent = Some(parent);
    }

    /// Set data
    pub fn set_data(&mut self, data: T) {
        self.data = Some(data);
    }

    /// Get immutable access to the data
    pub fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }

    /// Get mutable access to the data
    pub fn data_mut(&mut self) -> Option<&mut T> {
        self.data.as_mut()
    }

    /// Get mutable access to the data
    ///
    /// If Option is `None`, insert a default entry
    pub fn data_or_default(&mut self) -> &mut T
    where
        T: Default,
    {
        self.data.get_or_insert_with(T::default)
    }

    /// Get mutable access to the data
    ///
    /// If Option is `None`, insert an entry computed from a closure
    pub fn data_or_insert_with(&mut self, func: impl FnOnce() -> T) -> &mut T {
        self.data.get_or_insert_with(func)
    }

    /// Trigger sub asset loading for the prefab entity
    pub fn load_sub_assets<'a>(
        &mut self,
        progress: &mut ProgressCounter,
        system_data: &mut <T as PrefabData<'a>>::SystemData,
    ) -> Result<bool, Error>
    where
        T: PrefabData<'a>,
    {
        if let Some(ref mut data) = self.data {
            data.load_sub_assets(progress, system_data)
        } else {
            Ok(false)
        }
    }
}

use std::marker::PhantomData;

/// Main `Prefab` structure, containing all data loaded in a single prefab.
///
/// Contains a list prefab data for the entities affected by the prefab. The first entry in the
/// `entities` list will be applied to the main `Entity` the `Handle` is placed on, and all other
/// entries will trigger creation of a new entity. Note that the parent index is ignored for the
/// first entry in the list.
///
/// The recommended way of loading resources is to place them on the main `Entity`.
///
/// ### Example:
///
/// If we want to give the existing Baker entity a Knife and a Plate with a
/// Cake on it.  The prefab contains 3 new entities `Knife`, `Plate`,
/// and `Cake`, and the main `Entity` that the `Handle` is placed on is the
/// `Baker`.  We want the graph to be `Knife on Baker`, `Plate on Baker`,
/// `Cake on Plate` using parent links. The data will be as follows:
///
/// ```rust,ignore
/// Prefab {
///     entities: vec![
///         PrefabEntity { parent: None, .. },    /* #0 Baker, parent is not used */
///         PrefabEntity { parent: Some(0), .. }, /* #1 Knife parented to Baker #0 */
///         PrefabEntity { parent: Some(0), .. }, /* #2 Plate parented to Baker #0 */
///         PrefabEntity { parent: Some(2), .. }, /* #3 Cake parented to Plate #2 */
///     ],
/// }
/// ```
///
/// ### Type parameters:
///
/// - `T`: `PrefabData`
#[derive(Default, Deserialize, Serialize)]
pub struct Prefab<T> {
    #[serde(skip)]
    tag: Option<u64>,
    entities: Vec<PrefabEntity<T>>,
    #[serde(skip)]
    counter: Option<ProgressCounter>,
}


/// Trait for loading a prefabs data for a single entity
pub trait PrefabData<'a> {
    /// `SystemData` needed to perform the load
    type SystemData: SystemData<'a>;

    /// The result type returned by the load operation
    type Result;

    /// Add the data for this prefab onto the given `Entity`
    ///
    /// This can also be used to load resources, the recommended way of doing so is to put the
    /// resources on the main `Entity` of the `Prefab`
    ///
    /// ### Parameters:
    ///
    /// - `entity`: `Entity` to load components on, or the root `Entity` for the resource scenario
    /// - `system_data`: `SystemData` needed to do the loading
    /// - `entities`: Some components need access to the entities that was created as part of the
    ///               full prefab, for linking purposes, so this contains all those `Entity`s.
    /// - `children`: Entities that need access to the `Hierarchy`  in this function won't be able
    ///               to access it yet, since it is only updated after this function runs. As a work-
    ///               around, this slice includes all hierarchical children of the entity being passed.
    fn add_to_entity(
        &self,
        entity: Entity,
        system_data: &mut Self::SystemData,
        entities: &[Entity],
        children: &[Entity],
    ) -> Result<Self::Result, Error>;

    /// Trigger asset loading for any sub assets.
    ///
    /// ### Parameters:
    ///
    /// - `progress`: Progress structure that needs to be used for tracking progress of sub loads
    /// - `system_data`: `SystemData` for the prefab
    ///
    /// ### Returns
    ///
    /// - `Err(error)` - if an `Error` occurs
    /// - `Ok(false)` - if no sub assets need loading
    /// - `Ok(true)` - if sub assets need loading, in this case the sub asset load must be added to
    ///                the given progress tracker
    ///
    /// ### Type parameters:
    ///
    /// - `P`: Progress tracker
    fn load_sub_assets(
        &mut self,
        _progress: &mut ProgressCounter,
        _system_data: &mut Self::SystemData,
    ) -> Result<bool, Error> {
        Ok(false)
    }
}


/// Tag placed on entities created by the prefab system.
///
/// The tag value match the tag value of the `Prefab` the `Entity` was created from.
pub struct PrefabTag<T> {
    tag: u64,
    _m: PhantomData<T>,
}

impl<T> PrefabTag<T> {
    /// Create a new tag
    pub fn new(tag: u64) -> Self {
        PrefabTag {
            tag,
            _m: PhantomData,
        }
    }

    /// Get the tag
    pub fn tag(&self) -> u64 {
        self.tag
    }
}

/// Convenience `PrefabData` for loading assets of type `A` using `Format` `F`.
///
/// Will add a `Handle<A>` to the `Entity`
///
/// ### Type parameters:
///
/// - `A`: `Asset`,
/// - `F`: `Format` for loading `A`
// TODO: Add a debug impl for this that uses Filename correctly by default
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AssetPrefab<A, F = Box<dyn SerializableFormat<<A as Asset>::Data>>>
where
    A: Asset,
    // A::Data: FormatRegisteredData,
    F: Format<A::Data>,
{
    /// From existing handle
    #[serde(skip)]
    Handle(Handle<A>),
    /// From file, (name, format)
    File(String, F),
    /// Placeholder during loading
    #[serde(skip)]
    Placeholder,
}

impl<'a, A, F> PrefabData<'a> for AssetPrefab<A, F>
where
    A: Asset,
    F: Format<A::Data>,
{
    type SystemData = (
        ReadExpect<'a, Loader>,
        WriteStorage<'a, Handle<A>>,
        Read<'a, AssetStorage<A>>,
    );

    type Result = Handle<A>;

    fn add_to_entity(
        &self,
        entity: Entity,
        system_data: &mut Self::SystemData,
        _: &[Entity],
        _: &[Entity],
    ) -> Result<Handle<A>, Error> {
        let handle = match *self {
            AssetPrefab::Handle(ref handle) => handle.clone(),
            _ => unreachable!(),
        };
        Ok(system_data
            .1
            .insert(entity, handle.clone())
            .map(|_| handle)?)
    }

    fn load_sub_assets(
        &mut self,
        progress: &mut ProgressCounter,
        (loader, _, storage): &mut Self::SystemData,
    ) -> Result<bool, Error> {
        let (ret, next) = match std::mem::replace(self, AssetPrefab::Placeholder) {
            AssetPrefab::File(name, format) => {
                let handle = loader.load(name, format, progress, storage);
                (true, AssetPrefab::Handle(handle))
            }
            slot => (false, slot),
        };
        *self = next;
        Ok(ret)
    }
}

impl<T> Component for PrefabTag<T>
where
    T: Send + Sync + 'static,
{
    type Storage = DenseVecStorage<Self>;
}

impl<T> Asset for Prefab<T>
where
    T: Send + Sync + 'static,
{
    const NAME: &'static str = "PREFAB";
    type Data = Self;
    type HandleStorage = FlaggedStorage<Handle<Self>, DenseVecStorage<Handle<Self>>>;
}
