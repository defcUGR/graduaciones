use serde::{Deserialize, Serialize};
use tauri::State;
use cuid2::CuidConstructor;

const cuid_creator: CuidConstructor = CuidConstructor::new();

#[derive(Deserialize, Serialize, Clone)]
pub struct SessionData {
  id: String, // CUID2
  name: String,
  date: String
}

pub type SessionStoreType = Vec<SessionData>;

pub type SessionStore = crate::store::JsonStore::<SessionStoreType>;

#[tauri::command]
pub fn get_sessions(store: State<SessionStore>) -> Result<SessionStoreType, String> {
  store.lock().map(|g| g.clone()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_session_data(id: &str, store: State<SessionStore>) -> Result<Option<SessionData>, String> {
  let lock = store.lock().map_err(|e| e.to_string())?;
  let found = (*lock).iter().find(|s| &s.id == id).map(|bs| bs.clone());

  Ok(found)
}

#[tauri::command]
pub fn create_session(name: String, date: String, store: State<SessionStore>) -> Result<SessionData, String> {
  let session = SessionData {
    id: cuid_creator.with_length(10).create_id(),
    name,
    date,
  };

  let mut lock = store.lock().map_err(|e| e.to_string())?;
  (*lock).push(session.clone());

  Ok(session)
}