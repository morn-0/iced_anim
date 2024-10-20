//! Checkboxes can be used to let users make binary choices.
use iced::{
    advanced::{
        layout::{self, Layout},
        mouse, renderer, text,
        widget::{
            self,
            tree::{self, Tree},
        },
        Clipboard, Shell, Widget,
    },
    alignment,
    event::{self, Event},
    mouse::Cursor,
    touch, window, Element, Length, Pixels, Rectangle, Size,
};

use crate::{Animate, SpringMotion};

use super::AnimatedState;
pub use iced::widget::checkbox::{
    danger, primary, secondary, success, Catalog, Icon, Status, Style, StyleFn,
};

/// An animatedbox that can be checked.
#[allow(missing_debug_implementations)]
pub struct Checkbox<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Renderer: text::Renderer,
    Theme: Catalog,
{
    is_checked: bool,
    on_toggle: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    label: String,
    width: Length,
    size: f32,
    spacing: f32,
    text_size: Option<Pixels>,
    text_line_height: text::LineHeight,
    text_shaping: text::Shaping,
    text_wrapping: text::Wrapping,
    font: Option<Renderer::Font>,
    icon: Icon<Renderer::Font>,
    class: Theme::Class<'a>,
    motion: SpringMotion,
}

impl<'a, Message, Theme, Renderer> Checkbox<'a, Message, Theme, Renderer>
where
    Renderer: text::Renderer,
    Theme: Catalog,
{
    /// The default size of a [`Checkbox`].
    const DEFAULT_SIZE: f32 = 16.0;

    /// The default spacing of a [`Checkbox`].
    const DEFAULT_SPACING: f32 = 8.0;

    /// Creates a new [`Checkbox`].
    ///
    /// It expects:
    ///   * the label of the [`Checkbox`]
    ///   * a boolean describing whether the [`Checkbox`] is checked or not
    pub fn new(label: impl Into<String>, is_checked: bool) -> Self {
        Checkbox {
            is_checked,
            on_toggle: None,
            label: label.into(),
            width: Length::Shrink,
            size: Self::DEFAULT_SIZE,
            spacing: Self::DEFAULT_SPACING,
            text_size: None,
            text_line_height: text::LineHeight::default(),
            text_shaping: text::Shaping::default(),
            text_wrapping: text::Wrapping::default(),
            font: None,
            icon: Icon {
                font: Renderer::ICON_FONT,
                code_point: Renderer::CHECKMARK_ICON,
                size: None,
                line_height: text::LineHeight::default(),
                shaping: text::Shaping::Basic,
            },
            class: Theme::default(),
            motion: SpringMotion::default(),
        }
    }

    /// Sets the function that will be called when the [`Checkbox`] is toggled.
    /// It will receive the new state of the [`Checkbox`] and must produce a
    /// `Message`.
    ///
    /// Unless `on_toggle` is called, the [`Checkbox`] will be disabled.
    pub fn on_toggle<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(bool) -> Message,
    {
        self.on_toggle = Some(Box::new(f));
        self
    }

    /// Sets the function that will be called when the [`Checkbox`] is toggled,
    /// if `Some`.
    ///
    /// If `None`, the checkbox will be disabled.
    pub fn on_toggle_maybe<F>(mut self, f: Option<F>) -> Self
    where
        F: Fn(bool) -> Message + 'a,
    {
        self.on_toggle = f.map(|f| Box::new(f) as _);
        self
    }

    /// Sets the size of the [`Checkbox`].
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = size.into().0;
        self
    }

    /// Sets the width of the [`Checkbox`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the spacing between the [`Checkbox`] and the text.
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = spacing.into().0;
        self
    }

    /// Sets the text size of the [`Checkbox`].
    pub fn text_size(mut self, text_size: impl Into<Pixels>) -> Self {
        self.text_size = Some(text_size.into());
        self
    }

    /// Sets the text [`text::LineHeight`] of the [`Checkbox`].
    pub fn text_line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.text_line_height = line_height.into();
        self
    }

    /// Sets the [`text::Shaping`] strategy of the [`Checkbox`].
    pub fn text_shaping(mut self, shaping: text::Shaping) -> Self {
        self.text_shaping = shaping;
        self
    }

    /// Sets the [`text::Wrapping`] strategy of the [`Checkbox`].
    pub fn text_wrapping(mut self, wrapping: text::Wrapping) -> Self {
        self.text_wrapping = wrapping;
        self
    }

    /// Sets the [`Renderer::Font`] of the text of the [`Checkbox`].
    ///
    /// [`Renderer::Font`]: iced::advanced::text::Renderer
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Sets the [`Icon`] of the [`Checkbox`].
    pub fn icon(mut self, icon: Icon<Renderer::Font>) -> Self {
        self.icon = icon;
        self
    }

    /// Sets the style of the [`Checkbox`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style class of the [`Checkbox`].
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    /// Sets the motion that will be used by animations.
    pub fn motion(mut self, motion: SpringMotion) -> Self {
        self.motion = motion;
        self
    }

    /// The initial status that this widget will have based on its properties.
    ///
    /// This will be used as the initial state value.
    fn get_initial_status(&self) -> Status {
        if self.on_toggle.is_some() {
            Status::Active {
                is_checked: self.is_checked,
            }
        } else {
            Status::Disabled {
                is_checked: self.is_checked,
            }
        }
    }

    /// Gets the status of the [`Button`] based on the current [`State`].
    fn get_status(&self, cursor: Cursor, layout: Layout<'_>) -> Status {
        let is_mouse_over = cursor.is_over(layout.bounds());
        let is_disabled = self.on_toggle.is_none();
        let is_checked = self.is_checked;

        if is_disabled {
            Status::Disabled { is_checked }
        } else if is_mouse_over {
            Status::Hovered { is_checked }
        } else {
            Status::Active { is_checked }
        }
    }
}

pub struct State<Paragraph>
where
    Paragraph: text::Paragraph,
{
    text_state: widget::text::State<Paragraph>,
    animated_state: AnimatedState<Status, Style>,
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Checkbox<'a, Message, Theme, Renderer>
where
    Renderer: text::Renderer,
    Theme: Catalog,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State<Renderer::Paragraph>>()
    }

    fn state(&self) -> tree::State {
        let status = self.get_initial_status();
        // Initialize the state with the current style.
        let state = State::<Renderer::Paragraph> {
            text_state: Default::default(),
            animated_state: AnimatedState::new(status, self.motion),
        };
        tree::State::new(state)
    }

    fn diff(&self, tree: &mut Tree) {
        let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();
        state.animated_state.diff(self.motion);
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::next_to_each_other(
            &limits.width(self.width),
            self.spacing,
            |_| layout::Node::new(Size::new(self.size, self.size)),
            |limits| {
                let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();

                widget::text::layout(
                    &mut state.text_state,
                    renderer,
                    limits,
                    self.width,
                    Length::Shrink,
                    &self.label,
                    self.text_line_height,
                    self.text_size,
                    self.font,
                    alignment::Horizontal::Left,
                    alignment::Vertical::Top,
                    self.text_shaping,
                    self.text_wrapping,
                )
            },
        )
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        // Redraw anytime the status changes and would trigger a style change.
        let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();
        let status = self.get_status(cursor, layout);
        let needs_redraw = state.animated_state.needs_redraw(status);

        if needs_redraw {
            shell.request_redraw(window::RedrawRequest::NextFrame);
        }

        match event {
            Event::Window(window::Event::RedrawRequested(now)) => {
                state.animated_state.tick(now);
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let mouse_over = cursor.is_over(layout.bounds());

                if mouse_over {
                    if let Some(on_toggle) = &self.on_toggle {
                        shell.publish((on_toggle)(!self.is_checked));
                        return event::Status::Captured;
                    }
                }
            }
            _ => {}
        }

        event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) && self.on_toggle.is_some() {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        defaults: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let mut children = layout.children();
        let state = tree.state.downcast_ref::<State<Renderer::Paragraph>>();
        let style = state
            .animated_state
            .current_style(|status| theme.style(&self.class, *status));

        {
            let layout = children.next().unwrap();
            let bounds = layout.bounds();

            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: style.border,
                    ..renderer::Quad::default()
                },
                style.background,
            );

            let Icon {
                font,
                code_point,
                size,
                line_height,
                shaping,
            } = &self.icon;
            let size = size.unwrap_or(Pixels(bounds.height * 0.7));

            if self.is_checked {
                renderer.fill_text(
                    text::Text {
                        content: code_point.to_string(),
                        font: *font,
                        size,
                        line_height: *line_height,
                        bounds: bounds.size(),
                        horizontal_alignment: alignment::Horizontal::Center,
                        vertical_alignment: alignment::Vertical::Center,
                        shaping: *shaping,
                        wrapping: text::Wrapping::default(),
                    },
                    bounds.center(),
                    style.icon_color,
                    *viewport,
                );
            }
        }

        {
            let label_layout = children.next().unwrap();
            let state: &State<Renderer::Paragraph> = tree.state.downcast_ref();

            widget::text::draw(
                renderer,
                defaults,
                label_layout,
                state.text_state.0.raw(),
                widget::text::Style {
                    color: style.text_color,
                },
                viewport,
            );
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Checkbox<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + Catalog,
    Renderer: 'a + text::Renderer,
{
    fn from(
        checkbox: Checkbox<'a, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(checkbox)
    }
}

/// Creates a new animated [`Checkbox`].
pub fn checkbox<'a, Message, Theme, Renderer>(
    label: impl Into<String>,
    is_checked: bool,
) -> Checkbox<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: text::Renderer,
{
    Checkbox::new(label, is_checked)
}

impl Animate for iced::widget::checkbox::Style {
    fn components() -> usize {
        iced::Background::components()
            + iced::Color::components()
            + iced::Border::components()
            + Option::<iced::Color>::components()
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.background.distance_to(&end.background),
            self.icon_color.distance_to(&end.icon_color),
            self.border.distance_to(&end.border),
            self.text_color.distance_to(&end.text_color),
        ]
        .concat()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.background.update(components);
        self.icon_color.update(components);
        self.border.update(components);
        self.text_color.update(components);
    }
}
