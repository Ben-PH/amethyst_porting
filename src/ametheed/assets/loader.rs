
use crate::ametheed::assets::Error;
use crate::ametheed::assets::FormatValue;
use std::borrow::Borrow;
use std::hash::Hash;
use crate::ametheed::assets::progress::Progress;
use crate::ametheed::assets::Format;
use crate::ametheed::assets::Asset;
use crate::ametheed::Handle;
use crate::ametheed::assets::storage::AssetStorage;
use std::sync::Arc;

/// The asset loader, holding the sources and a reference to the `ThreadPool`.
pub struct Loader {
    hot_reload: bool,
    pool: Arc<ThreadPool>,
    sources: FnvHashMap<String, Arc<dyn Source>>,
}

impl Loader {

    /// If set to `true`, this `Loader` will ask formats to
    /// generate "reload instructions" which *allow* reloading.
    /// Calling `set_hot_reload(true)` does not actually enable
    /// hot reloading; this is controlled by the `HotReloadStrategy`
    /// resource.
    pub fn set_hot_reload(&mut self, value: bool) {
        self.hot_reload = value;
    }

    /// Loads an asset with a given format from the default (directory) source.
    /// If you want to load from a custom source instead, use `load_from`.
    ///
    /// See `load_from` for more information.
    pub fn load<A, F, N, P>(
        &self,
        name: N,
        format: F,
        progress: P,
        storage: &AssetStorage<A>,
    ) -> Handle<A>
    where
        A: Asset,
        F: Format<A::Data>,
        N: Into<String>,
        P: Progress,
    {
        #[cfg(feature = "profiler")]
        profile_scope!("initialise_loading_assets");
        self.load_from::<A, F, _, _, _>(name, format, "", progress, storage)
    }

    /// Loads an asset with a given id and format from a custom source.
    /// The actual work is done in a worker thread, thus this method immediately returns a handle.
    ///
    /// ## Parameters
    ///
    /// * `name`: this is just an identifier for the asset, most likely a file name e.g.
    ///   `"meshes/a.obj"`
    /// * `format`: A format struct which loads bytes from a `source` and produces `Asset::Data`
    ///   with them
    /// * `source`: An identifier for a source which has previously been added using `with_source`
    /// * `progress`: A tracker which will be notified of assets which have been imported
    /// * `storage`: The asset storage which can be fetched from the ECS `World` using
    ///   `read_resource`.
    pub fn load_from<A, F, N, P, S>(
        &self,
        name: N,
        format: F,
        source: &S,
        mut progress: P,
        storage: &AssetStorage<A>,
    ) -> Handle<A>
    where
        A: Asset,
        F: Format<A::Data>,
        N: Into<String>,
        P: Progress,
        S: AsRef<str> + Eq + Hash + ?Sized,
        String: Borrow<S>,
    {
        #[cfg(feature = "profiler")]
        profile_scope!("load_asset_from");
        use crate::ametheed::assets::progress::Tracker;

        let name = name.into();
        let source = source.as_ref();

        let format_name = format.name();
        let source_name = match source {
            "" => "[default source]",
            other => other,
        };

        let handle = storage.allocate();

        debug!(
            "{:?}: Loading asset {:?} with format {:?} from source {:?} (handle id: {:?})",
            A::NAME,
            name,
            format_name,
            source_name,
            handle,
        );

        progress.add_assets(1);
        let tracker = progress.create_tracker();

        let source = self.source(source);
        let handle_clone = handle.clone();
        let processed = storage.processed.clone();

        let hot_reload = if self.hot_reload {
            Some(dyn_clone::clone_box(&format) as Box<dyn Format<A::Data>>)
        } else {
            None
        };

        let cl = move || {
            #[cfg(feature = "profiler")]
            profile_scope!("load_asset_from_worker");
            let data = format
                .import(name.clone(), source, hot_reload)
                .with_context(|_| Error::Format(format_name));
            let tracker = Box::new(tracker) as Box<dyn Tracker>;

            processed.push(Processed::NewAsset {
                data,
                handle,
                name,
                tracker,
            });
        };
        self.pool.spawn(cl);

        handle_clone
    }

    /// Load an asset from data and return a handle.
    pub fn load_from_data<A, P>(
        &self,
        data: A::Data,
        mut progress: P,
        storage: &AssetStorage<A>,
    ) -> Handle<A>
    where
        A: Asset,
        P: Progress,
    {
        progress.add_assets(1);
        let tracker = progress.create_tracker();
        let tracker = Box::new(tracker);
        let handle = storage.allocate();
        storage.processed.push(Processed::NewAsset {
            data: Ok(FormatValue::data(data)),
            handle: handle.clone(),
            name: "<Data>".into(),
            tracker,
        });

        handle
    }

    /// Asynchronously load an asset from data and return a handle.
    pub fn load_from_data_async<A, P, F>(
        &self,
        data: F,
        mut progress: P,
        storage: &AssetStorage<A>,
    ) -> Handle<A>
    where
        A: Asset,
        P: Progress,
        F: FnOnce() -> A::Data + Send + Sync + 'static,
    {
        progress.add_assets(1);
        let tracker = progress.create_tracker();
        let tracker = Box::new(tracker);
        let handle = storage.allocate();
        let processed = storage.processed.clone();

        self.pool.spawn({
            let handle = handle.clone();
            move || {
                processed.push(Processed::NewAsset {
                    data: Ok(FormatValue::data(data())),
                    handle: handle.clone(),
                    name: "<Data>".into(),
                    tracker,
                });
            }
        });

        handle
    }

}
