use std::path::Path;
use crate::filesystem::events::WatchEvent;

use notify::{
    Config,
    Event,
    RecommendedWatcher,
    RecursiveMode,
    Result,
    Watcher,
    EventKind
};

pub fn watch<F>(path: &Path, mut emit: F,) -> notify::Result<()> where F: FnMut(WatchEvent) + Send + 'static,
 {
    let mut watcher = RecommendedWatcher::new(
        move |result: notify::Result<Event>| {
            match result {
                Ok(event) => {
                    use notify::event::{CreateKind, ModifyKind, RemoveKind, RenameMode};
                    match event.kind {
                        EventKind::Create(CreateKind::File)
                        | EventKind::Create(CreateKind::Folder)
                        | EventKind::Create(CreateKind::Any) => {
                            if let Some(path) = event.paths.first() {
                                emit(WatchEvent::Created(path.clone()));
                            }
                        }
                    
                        EventKind::Modify(ModifyKind::Any)
                        | EventKind::Modify(ModifyKind::Data(_))
                        | EventKind::Modify(ModifyKind::Metadata(_)) => {
                            if let Some(path) = event.paths.first() {
                                emit(
                                    WatchEvent::Modified(path.clone())
                                );
                            }
                        }
                    
                        EventKind::Remove(RemoveKind::File)
                        | EventKind::Remove(RemoveKind::Folder)
                        | EventKind::Remove(RemoveKind::Any) => {
                            if let Some(path) = event.paths.first() {
                                emit(
                                    WatchEvent::Deleted(path.clone())
                                );
                            }
                        }
                    
                        EventKind::Modify(ModifyKind::Name(RenameMode::Both)) => {
                            if event.paths.len() >= 2 {
                                emit(
                                    WatchEvent::Renamed {
                                        from: event.paths[0].clone(),
                                        to: event.paths[1].clone(),
                                    }
                                );
                            }
                        }
                    
                        _ => {}
                    }
                }
    
                Err(err) => {
                    eprintln!("Watch error: {}", err);
                }
            }
        },
        Config::default(),
    )?;
    watcher.watch(path, RecursiveMode::Recursive)?;
     loop {
           std::thread::park();
       }
}