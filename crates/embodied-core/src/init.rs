use heapless::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum InitStage {
    PreCore,
    PostCore,
    PreEnv,
    Env,
    PostEnv,
    PreDevice,
    Device,
    PostDevice,
    Late,
}

impl InitStage {
    pub const ALL: [Self; 9] = [
        Self::PreCore,
        Self::PostCore,
        Self::PreEnv,
        Self::Env,
        Self::PostEnv,
        Self::PreDevice,
        Self::Device,
        Self::PostDevice,
        Self::Late,
    ];
    pub const fn name(self) -> &'static str {
        match self {
            Self::PreCore => "PreCore",
            Self::PostCore => "PostCore",
            Self::PreEnv => "PreEnv",
            Self::Env => "Env",
            Self::PostEnv => "PostEnv",
            Self::PreDevice => "PreDevice",
            Self::Device => "Device",
            Self::PostDevice => "PostDevice",
            Self::Late => "Late",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegisterError {
    Full,
    AlreadyStarted,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InitError<E> {
    AlreadyStarted,
    Failed {
        stage: InitStage,
        hook: Option<&'static str>,
        error: E,
    },
}

struct Hook<C, E> {
    stage: InitStage,
    name: &'static str,
    function: fn(&mut C) -> Result<(), E>,
}

/// Explicit registration, no linker sections, constructors or heap allocation.
/// A registry can run only once, including after a failure or unwinding panic.
pub struct InitRegistry<C, E, const N: usize> {
    hooks: Vec<Hook<C, E>, N>,
    started: bool,
}

impl<C, E, const N: usize> Default for InitRegistry<C, E, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C, E, const N: usize> InitRegistry<C, E, N> {
    pub const fn new() -> Self {
        Self {
            hooks: Vec::new(),
            started: false,
        }
    }

    pub fn register(
        &mut self,
        stage: InitStage,
        name: &'static str,
        function: fn(&mut C) -> Result<(), E>,
    ) -> Result<(), RegisterError> {
        if self.started {
            return Err(RegisterError::AlreadyStarted);
        }
        self.hooks
            .push(Hook {
                stage,
                name,
                function,
            })
            .map_err(|_| RegisterError::Full)
    }

    /// Execute each stage's platform action before its hooks. In particular,
    /// the Env action can initialize a HAL and save its peripherals in `context`.
    pub fn run(
        &mut self,
        context: &mut C,
        mut before_stage: impl FnMut(InitStage, &mut C) -> Result<(), E>,
    ) -> Result<Ready, InitError<E>> {
        if self.started {
            return Err(InitError::AlreadyStarted);
        }
        self.started = true;
        for stage in InitStage::ALL {
            before_stage(stage, context).map_err(|error| InitError::Failed {
                stage,
                hook: None,
                error,
            })?;
            for hook in &self.hooks {
                if hook.stage == stage {
                    (hook.function)(context).map_err(|error| InitError::Failed {
                        stage,
                        hook: Some(hook.name),
                        error,
                    })?;
                }
            }
        }
        Ok(Ready { _private: () })
    }
}

/// Proof that all nine stages completed. Application startup should require this token.
/// A raw executor remains usable directly; integration must route application spawning here.
#[derive(Debug)]
#[must_use = "use this token to start application tasks after initialization"]
pub struct Ready {
    _private: (),
}

impl Ready {
    pub fn start<T>(self, start_application: impl FnOnce() -> T) -> T {
        start_application()
    }
}
