#![doc = include_str!(".crate-docs.md")]
use std::fmt::Debug;
use std::sync::Arc;

use easings::Linear;

pub mod easings;

/// An easing function for customizing animations.
#[derive(Debug, Clone, PartialEq)]
pub struct EasingFunction(EasingKind);

impl EasingFunction {
    /// Returns a new easing function using `func`.
    pub fn from_fn(func: fn(f32) -> f32) -> Self {
        Self(EasingKind::Fn(func))
    }

    /// Returns a new easing function using `easing`.
    pub fn new<Easing>(easing: Easing) -> Self
    where
        Easing: crate::Easing + Debug + Clone,
    {
        Self(EasingKind::Custom(Arc::new(easing)))
    }
}

impl Easing for EasingFunction {
    fn ease(&self, progress: f32) -> f32 {
        self.0.ease(progress)
    }
}

impl Default for EasingFunction {
    fn default() -> Self {
        Self::from(Linear)
    }
}

#[derive(Debug, Clone)]
enum EasingKind {
    /// A function pointer to use as an easing function.
    Fn(fn(f32) -> f32),
    /// A custom easing implementation.
    Custom(Arc<dyn Easing>),
}

impl Easing for EasingKind {
    fn ease(&self, progress: f32) -> f32 {
        match self {
            Self::Fn(func) => func(progress),
            Self::Custom(func) => func.ease(progress),
        }
    }
}

impl PartialEq for EasingKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Fn(l0), Self::Fn(r0)) => l0 == r0,
            (Self::Custom(l0), Self::Custom(r0)) => std::ptr::eq(&**l0, &**r0),
            _ => false,
        }
    }
}

/// Performs easing for value interpolation.
pub trait Easing: Debug + Send + Sync + 'static {
    /// Eases a value ranging between zero and one. The resulting value does not
    /// need to be bounded between zero and one.
    fn ease(&self, progress: f32) -> f32;
}

impl<T> Easing for T
where
    T: Fn(f32) -> f32 + Debug + Send + Sync + 'static,
{
    fn ease(&self, progress: f32) -> f32 {
        self(progress)
    }
}
