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
use std::sync::{Arc, Mutex};

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
pub struct Diagnostics<'c> {
    /// A mapping from file source ids to the actual sources themselves.
    sources: HashMap<SourceId, Source<'c>>,

    /// This contains the next source id.  Since we need to make sure
    /// that this is unique across all files, we'll just use a counter.
    next: usize,

    /// The currently active level.  Anything emitted at or above this
    /// level will be reported.
    active: Level,

    /// This allows us to keep track of overrides that are set by the
    /// language while compiling.
    overrides: Overrides,

    /// All of the emissions occurring over the lifetime of the
    /// diagnostics information.  This can be compiled into a large
    /// list at the end and emitted then, or serialized.
    emissions: Vec<Emission>,
}

impl<'c> Diagnostics<'c> {
    /// Creates a new diagnostics session.  This just returns the default
    /// value of it.
    pub fn new() -> Diagnostics<'c> {
        Diagnostics::default()
    }

    /// Creates a new source, and returns the proper reference to that
    /// source.  This allows us to create information about the source
    /// while using a copyable id to refer to it later.
    pub fn push(
        &mut self,
        name: impl Into<Cow<'c, str>>,
        content: Option<impl Into<Cow<'c, str>>>,
    ) -> SourceId {
        let id = self.next;
        self.next += 1;
        let id = SourceId(id);
        self.sources.insert(
            id,
            Source {
                id,
                name: name.into(),
                content: content.map(Into::into),
            },
        );
        id
    }

    /// Emits the given emission if, and only if, the given check name
    /// diagnostic is active.  More concisely, if check is active, then emit
    /// a diagnostic with name `name`, at location `span`, with message
    /// `message`, as if [`emit()`] was called with those parameters.
    pub fn emit_if(
        &mut self,
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
    pub fn emit(&mut self, name: Name, span: Span, message: impl Into<Cow<'static, str>>) {
        let emission = Emission::new(name, self.overrides.lookup(name), span, message);
        if self.active(name) {
            let source = span.source().and_then(|s| self.sources.get(&s));
            if let Some(mut term) = ::term::stderr() {
                emission.emit(source, &mut *term).unwrap();
            } else {
                emission
                    .emit(source, &mut self::output::NonTerminal::stderr())
                    .unwrap();
            }
        }

        self.emissions.push(emission);
    }

    pub fn active(&self, name: Name) -> bool {
        self.overrides.lookup(name) >= self.active
    }
}

impl<'c> Default for Diagnostics<'c> {
    fn default() -> Diagnostics<'c> {
        Diagnostics {
            sources: Default::default(),
            next: 0,
            active: Level::default(),
            overrides: Overrides::new(),
            emissions: vec![],
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DiagnosticSync<'c>(Arc<Mutex<Diagnostics<'c>>>);

impl<'c> DiagnosticSync<'c> {
    /// Creates a new source, and returns the proper reference to that
    /// source.  This allows us to create information about the source
    /// while using a copyable id to refer to it later.
    pub fn push(
        &self,
        name: impl Into<Cow<'c, str>>,
        content: Option<impl Into<Cow<'c, str>>>,
    ) -> SourceId {
        self.0.lock().unwrap().push(name, content)
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
        self.0.lock().unwrap().emit_if(check, name, span, message)
    }

    /// Emits a diagnostic.  Even if the given diagostic name would not be
    /// emitted, it is still appended to the emissions list.  If it is emitted,
    /// it is emitted with all of the information that can be gathered, and
    /// emitted with [`Emission::emit()`] - with terminal support by default,
    /// otherwise, just stderr.
    pub fn emit(&self, name: Name, span: Span, message: impl Into<Cow<'static, str>>) {
        self.0.lock().unwrap().emit(name, span, message)
    }

    pub fn active(&self, name: Name) -> bool {
        self.0.lock().unwrap().active(name)
    }
}

impl<'c> From<Diagnostics<'c>> for DiagnosticSync<'c> {
    fn from(diag: Diagnostics<'c>) -> DiagnosticSync<'c> {
        DiagnosticSync(Arc::new(Mutex::new(diag)))
    }
}
