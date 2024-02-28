use std::path::{Path, PathBuf};

use rfd::FileDialog;

use crate::prelude::*;

/// Plugin to load and reload files from the file system.
pub(crate) struct FileWatcherPlugin;

impl Plugin for FileWatcherPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedFile>()
            .add_event::<FileLoadedEvent>()
            .add_systems(Update, load.run_if(resource_changed::<SelectedFile>));
    }
}

/// The source path of the currently loaded [`Breadboard`].
///
/// The `load` system is triggered when this resource changes, which means the current breadboard
/// needs to be unloaded, and the new one loaded.
#[derive(Resource, Deref, DerefMut)]
struct SelectedFile(PathBuf);

impl Default for SelectedFile {
    fn default() -> Self {
        Self(dirs::home_dir().unwrap_or(PathBuf::new()))
    }
}

impl AsRef<Path> for SelectedFile {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

/// The watcher resource.
///
/// This stores a receiver for a channel on which activity happens if the watched file is modified.
///
/// The `watch` system checks for any events on this resource, and updates the loaded
/// [`Breadboard`] immediately upon any changes.
///
/// This allows for breadboard source files to be modified while Butter.app is running.
#[derive(Resource, Deref, DerefMut)]
struct Watcher(());

/// Event triggered when a file was loaded.
#[derive(Event)]
pub(crate) struct FileLoadedEvent {
    pub name: String,
    pub contents: String,
}

fn load(source: Res<SelectedFile>, mut event: EventWriter<FileLoadedEvent>) {
    if !source.is_file() {
        // TODO: Trigger `alert` widget.
        return;
    }

    let Some(name) = source.file_name().map(|v| v.to_string_lossy().into_owned()) else {
        // TODO: Trigger `alert` widget.
        return;
    };

    let Ok(contents) = std::fs::read_to_string(&*source) else {
        // TODO: Trigger `alert` widget.
        return;
    };

    event.send(FileLoadedEvent { name, contents });
}

#[derive(SystemParam)]
pub(crate) struct LoadButton<'w> {
    load_path: ResMut<'w, SelectedFile>,
    redraw: ResMut<'w, ForceRedraw>,
}

impl WidgetSystem for LoadButton<'_> {
    type Args = ();
    type Output = ();

    fn system(
        world: &mut World,
        state: &mut SystemState<Self>,
        ui: &mut egui::Ui,
        _: Self::Args,
    ) -> Self::Output {
        let LoadButton {
            mut load_path,
            mut redraw,
        } = state.get_mut(world);

        if ui.button("Load Breadboard…").clicked() {
            if let Some(file) = FileDialog::new()
                .set_title("Open Breadboard File")
                .add_filter("breadboard", &["bnb"])
                .set_directory(&*load_path)
                .pick_file()
            {
                **load_path = file;
            }

            redraw.set();
        }
    }
}
