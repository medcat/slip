//! Maintains the source information and diagnostic information about the
//! modules that are being compiled.  The two are heavily intertwined - the
//! diagnostic information contains source information, and vice versa.
//!
//! The concept is as thus: a single "program" consists of multiple [`File`]s.
//! Each file has a name (the path to the file), and a content.  The name of
//! the file is important for tracking down bugs and issues alike, and so we
//! need to keep track of the file.  However, when we serialize, we want to
//! keep this information in a memory-efficient manner - we do not want to
//! clone the name of the file all over the place.  Instead, we associate every
//! file with a [`Source`], which is just a single integer that describes that
//! file.  Then, the serialized code can use this [`Source`] to point to the
//! correct file, taking up the space of only 8 bytes.  So far so good.

use std::collections::HashMap;
use std::borrow::Cow;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};

mod name;
mod level;
mod span;
mod source;
mod overrides;
mod emission;
pub use self::name::Name;
pub use self::level::Level;
pub use self::span::{Position, Span, SourceSpan};
pub use self::source::{SourceId, Source};

use self::overrides::Overrides;

/// The diagnostic information related to a compilation.  This includes 
/// information about file sources and errors.
pub struct Diagnostics {
    /// A mapping from file source ids to the actual sources themselves.
    /// Note that this is wrapped in a rwlock - this allows us to mutate
    /// the source list with only an immutable reference.  This is not really
    /// great, but it allows us to pass out the contents of a file _and_
    /// import a file at the same time.
    sources: RwLock<HashMap<SourceId, RwLock<Source>>>,

    /// This contains the next source id.  Since we need to make sure that 
    /// this is unique across all files, we'll just use a counter.
    next: Arc<AtomicUsize>,

    /// This allows us to keep track of overrides that are set by the language
    /// while compiling.
    overrides: RwLock<Overrides>,

    /// All of the emissions occurring over the lifetime of the diagnostics
    /// information.  This can be compiled into a large list at the end and
    /// emitted then, or serialized.
    emissions: Mutex<Vec<Emission>>,
}