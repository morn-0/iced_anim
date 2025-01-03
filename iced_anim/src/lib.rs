//! A library for easily creating animations in Iced. Animations are designed around spring physics
//! to make animations feel natural and interactive.
//!
//! # Overview
//!
//! The primary widgets are `Animation` and `AnimationBuilder`, which are widgets that help you
//! animates a value when it changes. The `Animation` widget allows you to store the animated value
//! in your app state, while the `AnimationBuilder` widget maintains the animated value so your app
//! state is always up-to-date with the latest value. Refer to those widget modules for documentation
//! and the `examples` directory for examples on how to use them.
//!
//! ## Animated widgets
//!
//! A subset of the standard `iced` widgets are exported under a `widgets` feature
//! flag. You can use these as drop-in replacements for the existing widgets:
//!
//! ```rust
//! # #[derive(Clone)] enum Message { DoSomething }
//! use iced::{Element, widget::text};
//! use iced_anim::widget::button;
//!
//! fn fancy_button<'a>() -> Element<'a, Message> {
//!     button(text("Animated button"))
//!         .on_press(Message::DoSomething)
//!         .into()
//! }
//! ```
//!
//! ## Animating types
//!
//! You can animate anything that implements [`Animate`]. A handful of common types already
//! implement [`Animate`] like [`f32`], [`iced::Color`], and [`iced::Theme`], with more to come
//! in the future. You can also derive [`Animate`] on your own types to animate them as well
//! by enabling the `derive` feature flag and adding `#[derive(Animate)]` to your type:
//!
//! ```rust
//! use iced_anim::Animate;
//!
//! // Note that animate also requires `Clone` and `PartialEq` impls.
//! #[derive(Animate, Clone, PartialEq)]
//! struct MyType {
//!     size: f32,
//!     color: iced::Color,
//! }
//! ```
//!
//! ## Controlling the spring motion
//!
//! The spring motion of an [`AnimationBuilder`] can be customized. There are a few
//! defaults like [`SpringMotion::Smooth`] and [`SpringMotion::Bouncy`], but you can
//! provide a custom response and damping fraction with [`SpringMotion::Custom`].
//!
//! ## Supported Iced versions
//!
//! This crate supports Iced 0.13 and newer.
pub mod animate;
pub mod animation;
pub mod animation_builder;
pub mod spring;
pub mod spring_event;
pub mod spring_motion;

#[cfg(feature = "widgets")]
pub mod widget;

pub use animate::Animate;
pub use animation::Animation;
pub use animation_builder::*;
pub use spring::Spring;
pub use spring_event::SpringEvent;
pub use spring_motion::SpringMotion;

#[cfg(feature = "derive")]
pub use iced_anim_derive::Animate;
