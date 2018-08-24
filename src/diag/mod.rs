use error::*;

mod diagnostic;
mod instance;
mod level;
mod overrides;
mod source;
mod span;

pub use self::diagnostic::*;
use self::instance::Instance;
pub use self::level::*;
use self::overrides::Overrides;
pub use self::source::*;
pub use self::span::*;

#[derive(Debug, Clone)]
pub struct DiagnosticSet<'s> {
    source: &'s str,
    overrides: Overrides,
    instances: Vec<Instance>,
    report: Level,
}

impl<'s> DiagnosticSet<'s> {
    pub fn new(source: &'s str) -> DiagnosticSet<'s> {
        DiagnosticSet {
            source,
            overrides: Overrides::new(),
            instances: vec![],
            report: Level::default(),
        }
    }

    pub fn push_diagnostic(&mut self) {
        self.overrides.push();
    }

    pub fn pop_diagnostic(&mut self) {
        self.overrides.pop();
    }

    pub fn override_diagnostic(&mut self, diag: Diagnostic, level: Level) {
        self.overrides.insert(diag, level);
    }

    pub fn emit(&mut self, diag: Diagnostic, span: Span, message: String) -> Result<()> {
        let level = self.overrides.lookup(&diag);
        let instance = Instance::new(diag, level, span, message);
        if level >= self.report {
            if let Some(mut term) = ::term::stderr() {
                instance.term_emit(self.source, term.as_mut())?;
            } else {
                instance.file_emit(self.source, &mut ::std::io::stderr())?;
            }
        }

        self.instances.push(instance);

        Ok(())
    }
}
