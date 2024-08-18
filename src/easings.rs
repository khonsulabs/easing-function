//! Built-in [`Easing`] implementations.

use core::f32::consts::PI;

use crate::{Easing, EasingFunction, EasingKind};

macro_rules! declare_easing_function {
    ($name:ident, $($anchor_name:ident)?, $description:literal, $closure:expr) => {
        /// An [`Easing`] function that eases
        #[doc = $description]
        $(#[doc = concat!("\n\nSee <https://easings.net/#", stringify!($anchor_name), "> for a visualization and more information.")])?
        #[derive(Clone, Copy, Debug)]
        pub struct $name;

        impl $name {
            /// Eases
            #[doc = $description]
            $(#[doc = concat!("\n\nSee <https://easings.net/#", stringify!($anchor_name), "> for a visualization and more information.")])?
            #[must_use]
            pub fn ease(progress: f32) -> f32 {
                let closure = force_closure_type($closure);
                closure(progress)
            }
        }

        impl Easing for $name {
            fn ease(&self, progress: f32) -> f32 {
                Self::ease(progress)
            }
        }

        impl From<$name> for EasingFunction {
            fn from(_function: $name) -> Self {
                Self::from_fn($name::ease)
            }
        }
    };
}

macro_rules! declare_easing_functions {
    ($(($name:ident, $([$anchor_name:ident],)? $name_no_ease:ident, $description:literal, $closure:expr)),+) => {
        /// An enumeration of all strandard easings provided.
        #[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum StandardEasing {
            $(
                /// Eases
                #[doc = $description]
                $(#[doc = concat!("\n\nSee <https://easings.net/#", stringify!($anchor_name), "> for a visualization and more information.")])?
                $name_no_ease,
            )+
        }

        impl StandardEasing {
            /// Returns a collection of every variant of this enum.
            #[must_use]
            pub fn all() -> &'static [StandardEasing] {
                static ALL: [StandardEasing; 31] = [
                    $(StandardEasing::$name_no_ease),+
                ];
                &ALL
            }
        }

        impl From<StandardEasing> for EasingFunction {
            fn from(easing: StandardEasing) -> Self {
                match easing {
                    $(StandardEasing::$name_no_ease => Self::from($name)),+
                }
            }
        }

        impl TryFrom<EasingFunction> for StandardEasing {
            type Error = NonStandardEasing;

            fn try_from(func: EasingFunction) -> Result<Self, Self::Error> {
                let EasingKind::Fn(easing_fn) = &func.0 else {
                    return Err(NonStandardEasing(func))
                };
                let easing_fn = *easing_fn as *const fn(f32) -> f32;

                if false {
                    unreachable!()
                } $(else
                if easing_fn == $name::ease as *const fn(f32) -> f32 {
                     Ok(Self::$name_no_ease)
                })+ else  {
                    Err(NonStandardEasing(func))
                }
            }
        }

        impl Easing for StandardEasing {
            fn ease(&self, percent: f32) -> f32 {
                match self {
                    $(Self::$name_no_ease => $name::ease(percent)),+
                }
            }
        }

        $(
            declare_easing_function!($name, $($anchor_name)?, $description, $closure);
        )+
    };
}

/// An error returned from [`StandardEasing::try_from`] indicating the
/// [`EasingFunction`] is not a standard easing function.
#[derive(Debug, Clone, PartialEq)]
pub struct NonStandardEasing(pub EasingFunction);

// This prevents the closures from requiring the parameter to be type annotated.
fn force_closure_type(f: impl Fn(f32) -> f32) -> impl Fn(f32) -> f32 {
    f
}

declare_easing_functions!(
    (
        EaseInSine,
        [easeInSine],
        InSine,
        "in using a sine wave",
        |percent| 1. - (percent * PI / 2.).cos()
    ),
    (
        EaseOutSine,
        [easeOutSine],
        OutSine,
        "out using a sine wave",
        |percent| (percent * PI / 2.).sin()
    ),
    (
        EaseInOutSine,
        [easeInOutSine],
        InOutSine,
        "in and out using a sine wave",
        |percent| -((percent * PI).cos() - 1.) / 2.
    ),
    (
        EaseInQuadradic,
        [easeInQuad],
        InQuadradic,
        "in using a quadradic (x^2) curve",
        squared
    ),
    (
        EaseOutQuadradic,
        [easeOutQuad],
        OutQuadradic,
        "out using a quadradic (x^2) curve",
        |percent| 1. - squared(1. - percent)
    ),
    (
        EaseInOutQuadradic,
        [easeInOutQuad],
        InOutQuadradic,
        "in and out using a quadradic (x^2) curve",
        |percent| {
            if percent < 0.5 {
                2. * percent * percent
            } else {
                1. - squared(-2. * percent + 2.) / 2.
            }
        }
    ),
    (
        EaseInCubic,
        [easeInCubic],
        InCubic,
        "in using a cubic (x^3) curve",
        cubed
    ),
    (
        EaseOutCubic,
        [easeOutCubic],
        OutCubic,
        "out using a cubic (x^3) curve",
        |percent| 1. - cubed(1. - percent)
    ),
    (
        EaseInOutCubic,
        [easeInOutCubic],
        InOutCubic,
        "in and out using a cubic (x^3) curve",
        |percent| {
            if percent < 0.5 {
                4. * cubed(percent)
            } else {
                1. - cubed(-2. * percent + 2.) / 2.
            }
        }
    ),
    (
        EaseInQuartic,
        [easeInQuart],
        InQuartic,
        "in using a quartic (x^4) curve",
        quarted
    ),
    (
        EaseOutQuartic,
        [easeOutQuart],
        OutQuartic,
        "out using a quartic (x^4) curve",
        |percent| 1. - quarted(1. - percent)
    ),
    (
        EaseInOutQuartic,
        [easeInOutQuart],
        InOutQuartic,
        "in and out using a quartic (x^4) curve",
        |percent| {
            if percent < 0.5 {
                8. * quarted(percent)
            } else {
                1. - quarted(-2. * percent + 2.) / 2.
            }
        }
    ),
    (
        EaseInQuintic,
        [easeInQuint],
        InQuintic,
        "in using a quintic (x^5) curve",
        quinted
    ),
    (
        EaseOutQuintic,
        [easeOutQuint],
        OutQuintic,
        "out using a quintic (x^5) curve",
        |percent| 1. - quinted(1. - percent)
    ),
    (
        EaseInOutQuintic,
        [easeInOutQuint],
        InOutQuintic,
        "in and out using a quintic (x^5) curve",
        |percent| {
            if percent < 0.5 {
                16. * quinted(percent)
            } else {
                1. - quinted(-2. * percent + 2.) / 2.
            }
        }
    ),
    (
        EaseInExponential,
        [easeInExpo],
        InExponential,
        "in using an expenential curve",
        |percent| { 2f32.powf(10. * percent - 10.) }
    ),
    (
        EaseOutExponential,
        [easeOutExpo],
        OutExponential,
        "out using an expenential curve",
        |percent| { 1. - 2f32.powf(-10. * percent) }
    ),
    (
        EaseInOutExponential,
        [easeInOutExpo],
        InOutExponential,
        "in and out using an expenential curve",
        |percent| if percent < 0.5 {
            2f32.powf(20. * percent - 10.) / 2.
        } else {
            (2. - 2f32.powf(-20. * percent + 10.)) / 2.
        }
    ),
    (
        EaseInCircular,
        [easeInCirc],
        InCircular,
        "in using a curve resembling the top-left arc of a circle",
        |percent| 1. - (1. - squared(percent)).sqrt()
    ),
    (
        EaseOutCircular,
        [easeOutCirc],
        OutCircular,
        "out using a curve resembling the top-left arc of a circle",
        |percent| (1. - squared(percent - 1.)).sqrt()
    ),
    (
        EaseInOutCircular,
        [easeInOutCirc],
        InOutCircular,
        "in and out using a curve resembling the top-left arc of a circle",
        |percent| {
            if percent < 0.5 {
                (1. - (1. - squared(2. * percent)).sqrt()) / 2.
            } else {
                ((1. - squared(-2. * percent + 2.)).sqrt() + 1.) / 2.
            }
        }
    ),
    (
        EaseInBack,
        [easeInBack],
        InBack,
        "in using a curve that backs away initially",
        |percent| {
            let squared = squared(percent);
            let cubed = squared * percent;
            C3 * cubed - C1 * squared
        }
    ),
    (
        EaseOutBack,
        [easeOutBack],
        OutBack,
        "out using a curve that backs away initially",
        |percent| {
            let percent_minus_one = percent - 1.;
            let squared = squared(percent_minus_one);
            let cubed = squared * percent_minus_one;
            1. + C3 * cubed + C1 * squared
        }
    ),
    (
        EaseInOutBack,
        [easeInOutBack],
        InOutBack,
        "in and out using a curve that backs away initially",
        |percent| {
            if percent < 0.5 {
                (squared(2. * percent) * ((C2 + 1.) * 2. * percent - C2)) / 2.
            } else {
                (squared(2. * percent - 2.) * ((C2 + 1.) * (percent * 2. - 2.) + C2) + 2.) / 2.
            }
        }
    ),
    (
        EaseInElastic,
        [easeInElastic],
        InElastic,
        "in using a curve that bounces around the start initially then quickly accelerates",
        |percent| { -(2f32.powf(10. * percent - 10.)) * ((percent * 10. - 10.75) * C4).sin() }
    ),
    (
        EaseOutElastic,
        [easeOutElastic],
        OutElastic,
        "out using a curve that bounces around the start initially then quickly accelerates",
        |percent| { 2f32.powf(-10. * percent) * ((percent * 10. - 0.75) * C4).sin() + 1. }
    ),
    (
        EaseInOutElastic,
        [easeInOutElastic],
        InOutElastic,
        "in and out using a curve that bounces around the start initially then quickly accelerates",
        |percent| if percent < 0.5 {
            -(2f32.powf(20. * percent - 10.)) * ((percent * 20. - 11.125) * C5).sin() / 2.
        } else {
            2f32.powf(-20. * percent + 10.) * ((percent * 20. - 11.125) * C5).sin() / 2. + 1.
        }
    ),
    (
        EaseInBounce,
        [easeInBounce],
        InBounce,
        "in using a curve that bounces progressively closer as it progresses",
        |percent| 1. - EaseOutBounce.ease(1. - percent)
    ),
    (
        EaseOutBounce,
        [easeOutBounce],
        OutBounce,
        "out using a curve that bounces progressively closer as it progresses",
        |percent| {
            const N1: f32 = 7.5625;
            const D1: f32 = 2.75;

            if percent < 1. / D1 {
                N1 * squared(percent)
            } else if percent < 2. / D1 {
                let percent = percent - 1.5 / D1;
                N1 * squared(percent) + 0.75
            } else if percent < 2.5 / D1 {
                let percent = percent - 2.25 / D1;
                N1 * squared(percent) + 0.9375
            } else {
                let percent = percent - 2.625 / D1;
                N1 * squared(percent) + 0.984_375
            }
        }
    ),
    (
        EaseInOutBounce,
        [easeInOutBounce],
        InOutBounce,
        "in and out using a curve that bounces progressively closer as it progresses",
        |percent| {
            if percent < 0.5 {
                (1. - EaseOutBounce::ease(1. - 2. * percent)) / 2.
            } else {
                (1. + EaseOutBounce::ease(2. * percent - 1.)) / 2.
            }
        }
    ),
    (Linear, Linear, "linearly", |percent| percent)
);

fn squared(value: f32) -> f32 {
    value * value
}

fn cubed(value: f32) -> f32 {
    value * value * value
}

fn quarted(value: f32) -> f32 {
    let sq = squared(value);
    squared(sq)
}

fn quinted(value: f32) -> f32 {
    let squared = squared(value);
    let cubed = squared * value;
    squared * cubed
}

const C1: f32 = 1.70158;
const C2: f32 = C1 * 1.525;
const C3: f32 = C1 + 1.;
const C4: f32 = (2. * PI) / 3.;
const C5: f32 = (2. * PI) / 4.5;

#[test]
fn roundtrip() {
    for &easing in StandardEasing::all() {
        let f = EasingFunction::from(dbg!(easing));
        let rt = StandardEasing::try_from(f);
        assert_eq!(rt, Ok(easing));
    }
}
