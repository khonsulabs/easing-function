use std::fmt::Debug;

use easings::Linear;

pub mod easings;

/// An easing function for customizing animations.
#[derive(Debug, Clone, PartialEq)]
pub struct EasingFunction(EasingKind);

impl EasingFunction {
    pub fn from_fn(func: fn(f32) -> f32) -> Self {
        Self(EasingKind::Fn(func))
    }

    pub fn new<Easing>(easing: Easing) -> Self
    where
        Easing: crate::Easing + Debug + Clone,
    {
        Self(EasingKind::Custom(Box::new(easing)))
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

#[derive(Debug)]
enum EasingKind {
    /// A function pointer to use as an easing function.
    Fn(fn(f32) -> f32),
    /// A custom easing implementation.
    Custom(Box<dyn EasingClone>),
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

impl Clone for EasingKind {
    fn clone(&self) -> Self {
        match self {
            Self::Fn(func) => Self::Fn(*func),
            Self::Custom(func) => Self::Custom(func.clone_boxed()),
        }
    }
}

trait EasingClone: Debug + Easing {
    fn clone_boxed(&self) -> Box<dyn EasingClone>;
}

impl<T> EasingClone for T
where
    T: Debug + Clone + Easing,
{
    fn clone_boxed(&self) -> Box<dyn EasingClone> {
        Box::new(self.clone())
    }
}

/// Performs easing for value interpolation.
pub trait Easing: Debug + Send + Sync + 'static {
    /// Eases a value ranging between zero and one. The resulting value does not
    /// need to be bounded between zero and one.
    fn ease(&self, progress: f32) -> f32;
}
