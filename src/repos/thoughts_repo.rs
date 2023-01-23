use crate::{domain::model::thought::Thought, web_api::handlers::CreateThoughtInput};
use axum::extract::FromRef;
use nanoid::nanoid;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

/// A basic in-memory repository.
#[derive(FromRef)]
pub struct ThoughtsRepo {
    store: Arc<Mutex<HashMap<String, Thought>>>,
}

impl ThoughtsRepo {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add(&self, item: CreateThoughtInput) -> Thought {
        let new_item = Thought {
            id: nanoid!(3),
            idea: item.idea,
            tags: item.tags,
        };
        self.store
            .lock()
            .unwrap()
            .insert(new_item.id.clone(), new_item.clone());
        new_item
    }

    pub fn get_all(&self) -> Vec<Thought> {
        let mut res = Vec::with_capacity(self.store.lock().unwrap().len());
        self.store
            .lock()
            .unwrap()
            .values()
            .for_each(|t| res.push(t.clone()));
        res
    }
}
