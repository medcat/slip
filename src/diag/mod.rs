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

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};

mod emission;
mod level;
mod name;
mod output;
mod overrides;
mod source;
mod span;

pub use self::level::Level;
pub use self::name::Name;
pub use self::source::{Source, SourceId};
pub use self::span::{Position, SourceSpan, Span};

use self::emission::Emission;
use self::overrides::Overrides;

/// The diagnostic information related to a compilation.  This includes
/// information about file sources and errors.
#[derive(Debug)]
pub struct Diagnostics {
    /// A mapping from file source ids to the actual sources themselves.
    /// Note that this is wrapped in a rwlock - this allows us to mutate
    /// the source list with only an immutable reference.  This is not really
    /// great, but it allows us to pass out the contents of a file _and_
    /// import a file at the same time.
    sources: RwLock<HashMap<SourceId, Arc<Source>>>,

    /// This contains the next source id.  Since we need to make sure that
    /// this is unique across all files, we'll just use a counter.
    next: Arc<AtomicUsize>,

    /// The currently active level.  Anything emitted at or above this level
    /// will be reported.
    active: RwLock<Level>,

    /// This allows us to keep track of overrides that are set by the language
    /// while compiling.
    overrides: RwLock<Overrides>,

    /// All of the emissions occurring over the lifetime of the diagnostics
    /// information.  This can be compiled into a large list at the end and
    /// emitted then, or serialized.
    emissions: Mutex<Vec<Emission>>,
}

impl Diagnostics {
    /// Creates a new diagnostics session.  This just returns the default
    /// value of it.
    pub fn new() -> Diagnostics {
        Diagnostics::default()
    }

    /// Add a source to the diagnostic session.  This returns a source id in
    /// response, which can be used to retrieve the source itself, or its
    /// contents.
    pub fn add_source<N: Into<String>, C: Into<String>>(&self, name: N, content: C) -> SourceId {
        let mut sources = self.sources.write().unwrap();
        let id = SourceId(self.next.fetch_add(1, Ordering::Acquire));
        let source = Source {
            id,
            name: name.into(),
            content: content.into(),
        };
        sources.insert(id, Arc::new(source));
        id
    }

    pub fn source(&self, id: SourceId) -> Option<Arc<Source>> {
        self.sources.read().unwrap().get(&id).cloned()
    }

    /// Emits the given emission if, and only if, the given check name
    /// diagnostic is active.  More concisely, if check is active, then emit
    /// a diagnostic with name `name`, at location `span`, with message
    /// `message`, as if [`emit()`] was called with those parameters.
    pub fn emit_if(
        &self,
        check: Name,
        name: Name,
        span: Span,
        message: impl Into<Cow<'static, str>>,
    ) {
        if self.active(check) {
            self.emit(name, span, message)
        }
    }

    /// Emits a diagnostic.  Even if the given diagostic name would not be
    /// emitted, it is still appended to the emissions list.  If it is emitted,
    /// it is emitted with all of the information that can be gathered, and
    /// emitted with [`Emission::emit()`] - with terminal support by default,
    /// otherwise, just stderr.
    pub fn emit(&self, name: Name, span: Span, message: impl Into<Cow<'static, str>>) {
        let emission = Emission::new(
            name,
            self.overrides.read().unwrap().lookup(name),
            span,
            message,
        );
        if self.active(name) {
            let sources = self.sources.read().unwrap();
            let source = span
                .source()
                .and_then(|s| sources.get(&s))
                .map(AsRef::as_ref);
            if let Some(mut term) = ::term::stderr() {
                emission.emit(source, &mut *term).unwrap();
            } else {
                emission
                    .emit(source, &mut self::output::NonTerminal::stderr())
                    .unwrap();
            }
        }

        self.emissions.lock().unwrap().push(emission);
    }

    pub fn active(&self, name: Name) -> bool {
        self.overrides.read().unwrap().lookup(name) >= *self.active.read().unwrap()
    }
}

impl Default for Diagnostics {
    fn default() -> Diagnostics {
        Diagnostics {
            sources: RwLock::new(HashMap::new()),
            next: Arc::new(AtomicUsize::new(1)),
            active: RwLock::new(Level::default()),
            overrides: RwLock::new(Overrides::default()),
            emissions: Mutex::new(vec![]),
        }
    }
}
