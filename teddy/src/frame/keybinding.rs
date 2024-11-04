//use std::collections::HashMap;
//use std::fmt::Debug;
//
//use crossterm::event::KeyCode;
//use teddy_core::action::Action;
//
//use crate::prelude::*;
//
//use super::Frame;
//
#[derive(PartialEq, Eq, Hash, Clone, Debug, Default)]
pub struct Selection {
  from: usize,
  to: usize,

  buffer_size: usize,
}

impl Selection {
  pub fn with_buffer_len(buffer_size: usize) -> Self {
    Self { from: 0, to: 0, buffer_size }
  }
}
//
//  pub fn select(&mut self, from: usize, to: usize) {
//    if from > self.file_size {
//      panic!("from is greater than file size");
//    }
//
//    if to > self.file_size {
//      panic!("to is greater than file size");
//    }
//
//    if from > to {
//      panic!("from is greater than to");
//    }
//
//    self.from = from;
//    self.to = to;
//  }
//}

//#[derive(PartialEq, Eq, Hash, Clone)]
//enum BetweenChars {
//  Parantheses,
//
//  /// '[' and ']'
//  Brackets,
//  /// '{' and '}'
//  Brackets2,
//}

//pub trait BindAction: Debug {
//  fn act(&self, frame: &mut Frame) -> Result<Option<Action>>;
//}

//#[derive(Default)]
//pub struct RegisteredKeyBindings {
//  keybindings: HashMap<KeyBinding, Box<dyn BindAction>>,
//  input_match_state: KeyBinding,
//}
//
//impl Debug for RegisteredKeyBindings {
//  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//    f.debug_struct("RegisteredKeyBindings")
//      .field("keybindings", &self.keybindings)
//      .field("input_match_state", &self.input_match_state)
//      .finish()
//  }
//}
//
//#[derive(Hash, PartialEq, Eq, Default, Debug)]
//pub struct KeyBinding(Vec<KeyCode>);
//
//impl KeyBinding {
//  pub fn char(char: char) -> Self {
//    Self(vec![KeyCode::Char(char)])
//  }
//
//  pub fn keycode(key: KeyCode) -> Self {
//    Self(vec![key])
//  }
//}
//
//impl KeyBinding {
//  pub fn clear(&mut self) {
//    self.0.clear();
//  }
//}
//
//// impl KeyBinding {
////   #[must_use]
////   fn selection(mut self, selection: Selection) -> Self {
////     self.0 = Some(selection);
////     self
////   }
//// }
//
//// #[derive(Clone)]
//// pub enum KeyBindQueryInput {
////   Selection(Selection),
////   Char(KeyCode),
//// }
//
//impl RegisteredKeyBindings {
//  // TODO: 'static kommer bita mig i arslet
//  pub fn register<T>(&mut self, bind: KeyBinding, action: T)
//  where
//    T: BindAction + 'static,
//  {
//    self.keybindings.insert(bind, Box::new(action));
//  }
//
//  pub fn mutate_state(&self, input: KeyCode, to: &mut KeyBinding) {
//    to.0.push(input);
//    // match input {
//    //   KeyBindQueryInput::Selection(selection) => {
//    //     let input = KeyBinding::default();
//    //     *to = input.selection(selection);
//    //   }
//    //   KeyBindQueryInput::Char(char) => {
//    //     to.chars.push(char);
//    //   }
//    // }
//  }
//
//  /// This will accept characters thats coming in and decide if it matches a registered command.
//  pub fn query(&self, input: &KeyBinding) -> Option<&dyn BindAction> {
//    let value = self.keybindings.get(input)?;
//    Some(&**value)
//  }
//}
