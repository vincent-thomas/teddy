use std::{
  future::Future,
  pin::Pin,
  sync::{Arc, Mutex, RwLock},
};

use chrono::{Date, DateTime, NaiveDate, NaiveDateTime, Utc};
use teddy_core::action::Notification;
use tokio::task;

#[derive(Default, Debug, Clone)]
pub struct NotificationManager {
  pub vec: Vec<NotificationMessage>,
}

impl NotificationManager {
  pub fn append(&mut self, notification: NotificationMessage) {
    self.vec.push(notification)
  }
  //pub fn testing<F, Fut>(&'static mut self, initial_not: Notification, fun: F)
  //where
  //  F: Fn(Arc<Mutex<NotificationMessage>>) -> Fut + Send + 'static,
  //  Fut: Future<Output = ()> + Send + 'static,
  //{
  //  let mut thing = Arc::new(Mutex::new(NotificationMessage::as_is(initial_not, 255)));
  //  self.vec.push(thing.clone());
  //
  //  let another = thing.clone();
  //
  //  task::spawn(async move {
  //    fun(another.clone()).await;
  //
  //    another.lock().unwrap().lasts_to = Utc::now().timestamp() + 2;
  //  });
  //}
}

#[derive(Debug, Clone)]
pub struct NotificationMessage {
  pub payload: Notification,
  pub created_at: i64,
  pub lasts_to: i64,
}

impl NotificationMessage {
  pub fn new(payload: Notification, lasting: i64) -> Self {
    Self { payload, lasts_to: lasting, created_at: Utc::now().timestamp() }
  }
  pub fn as_is(payload: Notification, lasting: i64) -> Self {
    Self { payload, lasts_to: lasting, created_at: Utc::now().timestamp() }
  }
}
