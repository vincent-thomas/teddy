use crate::{buffer::buffer::Buffer, prelude::Result};
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{layout::Rect, Frame};
use teddy_events::Event;
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;

//use crate::{action::Action, config::Config, tui::Event};

/// `Component` is a trait that represents a visual and interactive element of the user interface.
///
/// Implementors of this trait can be registered with the main application loop and will be able to
/// receive events, update state, and be rendered on the screen.
pub trait Component: Buffer {
  /// Register an action handler that can send actions for processing if necessary.
  ///
  /// # Arguments
  ///
  /// * `tx` - An unbounded sender that can send actions.
  ///
  /// # Returns
  ///
  /// * `Result<()>` - An Ok result or an error.
  fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
    let _ = tx; // to appease clippy
    Ok(())
  }
  // /// Register a configuration handler that provides configuration settings if necessary.
  // ///
  // /// # Arguments
  // ///
  // /// * `config` - Configuration settings.
  // ///
  // /// # Returns
  // ///
  // /// * `Result<()>` - An Ok result or an error.
  // TODO:
  // fn register_config_handler(&mut self, config: Config) -> Result<()> {
  //   let _ = config; // to appease clippy
  //   Ok(())
  // }
  /// Initialize the component with a specified area if necessary.
  ///
  /// # Arguments
  ///
  /// * `area` - Rectangular area to initialize the component within.
  ///
  /// # Returns
  ///
  /// * `Result<()>` - An Ok result or an error.
  fn init(&mut self) -> Result<()> {
    Ok(())
  }
  /// Handle incoming events and produce actions if necessary.
  ///
  /// # Arguments
  ///
  /// * `event` - An optional event to be processed.
  ///
  /// # Returns
  ///
  /// * `Result<Option<Action>>` - An action to be processed or none.
  fn handle_events(&mut self, event: Option<Event>) -> Result<Option<Action>> {
    use crossterm::event::Event as CrosstermEvent;
    let action = match event {
      Some(Event::Crossterm(CrosstermEvent::Key(key_event))) => self.handle_key_event(key_event)?,
      Some(Event::Crossterm(CrosstermEvent::Mouse(mouse_event))) => {
        self.handle_mouse_event(mouse_event)?
      }
      _ => None,
    };
    Ok(action)
  }
  /// Handle key events and produce actions if necessary.
  ///
  /// # Arguments
  ///
  /// * `key` - A key event to be processed.
  ///
  /// # Returns
  ///
  /// * `Result<Option<Action>>` - An action to be processed or none.
  fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>>;
  /// Handle mouse events and produce actions if necessary.
  ///
  /// # Arguments
  ///
  /// * `mouse` - A mouse event to be processed.
  ///
  /// # Returns
  ///
  /// * `Result<Option<Action>>` - An action to be processed or none.
  fn handle_mouse_event(&mut self, mouse: MouseEvent) -> Result<Option<Action>>;
  // /// Update the state of the component based on a received action. (REQUIRED)
  // ///
  // /// # Arguments
  // ///
  // /// * `action` - An action that may modify the state of the component.
  // ///
  // /// # Returns
  // ///
  // /// * `Result<Option<Action>>` - An action to be processed or none.
  // fn update(&mut self, action: Action) -> Result<Option<Action>>;
  /// Render the component on the screen. (REQUIRED)
  ///
  /// # Arguments
  ///
  /// * `f` - A frame used for rendering.
  /// * `area` - The area in which the component should be drawn.
  ///
  /// # Returns
  ///
  /// * `Result<()>` - An Ok result or an error.
  fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()>;
}