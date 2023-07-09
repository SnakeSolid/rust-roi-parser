mod data;

pub use self::data::InitiativeData;
pub use self::data::VoteData;

use sqlite::Connection;
use sqlite::State;
use std::error::Error;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Database {
    inner: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new<P>(path: P) -> Result<Self, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let connection = sqlite::open(path)?;
        connection.execute(include_str!("create_database.sql"))?;

        Ok(Self {
            inner: Arc::new(Mutex::new(connection)),
        })
    }

    pub async fn all_initiatives(&self) -> Result<Vec<InitiativeData>, Box<dyn Error>> {
        debug!("All initiatives");

        let lock = self.inner.lock().await;
        let mut query = lock.prepare(
            "SELECT initiative_id, name, archived, enabled FROM initiatives ORDER BY initiative_id",
        )?;
        let mut result = Vec::new();

        while let State::Row = query.next()? {
            let id: i64 = query.read(0)?;
            let name = query.read(1)?;
            let archived: i64 = query.read(2)?;
            let enabled: i64 = query.read(3)?;
            let item = InitiativeData::new(id as u32, name, archived != 0, enabled != 0);

            result.push(item);
        }

        Ok(result)
    }

    pub async fn active_initiatives(&self) -> Result<Vec<InitiativeData>, Box<dyn Error>> {
        debug!("Active initiatives");

        let lock = self.inner.lock().await;
        let mut query = lock.prepare(
            "SELECT initiative_id, name, archived, enabled FROM initiatives WHERE archived == 0 AND enabled != 0",
        )?;
        let mut result = Vec::new();

        while let State::Row = query.next()? {
            let id: i64 = query.read(0)?;
            let name = query.read(1)?;
            let archived: i64 = query.read(2)?;
            let enabled: i64 = query.read(3)?;
            let item = InitiativeData::new(id as u32, name, archived != 0, enabled != 0);

            result.push(item);
        }

        Ok(result)
    }

    pub async fn create_initiative(
        &self,
        id: u32,
        name: &str,
        is_archived: bool,
        is_enabled: bool,
    ) -> Result<(), Box<dyn Error>> {
        debug!(
            "Create initiative: id = {}, name = {}, is_archived = {}, is_enabled = {}",
            id, name, is_archived, is_enabled
        );

        let lock = self.inner.lock().await;
        let mut query =
            lock.prepare("INSERT OR REPLACE INTO initiatives (initiative_id, name, archived, enabled) VALUES (:id, :name, :archived, :enabled)")?;
        query.bind((":id", id as i64))?;
        query.bind((":name", name))?;
        query.bind((":archived", is_archived as i64))?;
        query.bind((":enabled", is_enabled as i64))?;
        query.next()?;

        Ok(())
    }

    pub async fn enable_initiative(&self, id: u32) -> Result<(), Box<dyn Error>> {
        debug!("Enable initiative: id = {}", id);

        let lock = self.inner.lock().await;
        let mut query =
            lock.prepare("UPDATE initiatives SET enabled = 1 WHERE initiative_id = :id")?;
        query.bind((":id", id as i64))?;
        query.next()?;

        Ok(())
    }

    pub async fn disable_initiative(&self, id: u32) -> Result<(), Box<dyn Error>> {
        debug!("Disable initiative: id = {}", id);

        let lock = self.inner.lock().await;
        let mut query =
            lock.prepare("UPDATE initiatives SET enabled = 0 WHERE initiative_id = :id")?;
        query.bind((":id", id as i64))?;
        query.next()?;

        Ok(())
    }

    pub async fn archive_initiative(&self, id: u32) -> Result<(), Box<dyn Error>> {
        debug!("Archive initiative: id = {}", id);

        let lock = self.inner.lock().await;
        let mut query =
            lock.prepare("UPDATE initiatives SET archived = 1 WHERE initiative_id = :id")?;
        query.bind((":id", id as i64))?;
        query.next()?;

        Ok(())
    }

    pub async fn remove_initiative(&self, id: u32) -> Result<(), Box<dyn Error>> {
        debug!("Remove initiative: id = {}", id);

        let lock = self.inner.lock().await;
        let mut query = lock.prepare("DELETE FROM votes WHERE initiative_id = :id")?;
        query.bind((":id", id as i64))?;
        query.next()?;

        let mut query = lock.prepare("DELETE FROM initiatives WHERE initiative_id = :id")?;
        query.bind((":id", id as i64))?;
        query.next()?;

        Ok(())
    }

    pub async fn initiative_votes(&self, id: u32) -> Result<Vec<VoteData>, Box<dyn Error>> {
        debug!("Initiative votes");

        let lock = self.inner.lock().await;
        let mut query = lock
            .prepare("SELECT timestamp, positive, negative FROM votes WHERE initiative_id = :id")?;
        query.bind((":id", id as i64))?;

        let mut result = Vec::new();

        while let State::Row = query.next()? {
            let timestamp = query.read(0)?;
            let positive: i64 = query.read(1)?;
            let negative: i64 = query.read(2)?;
            let item = VoteData::new(id, timestamp, positive as u32, negative as u32);

            result.push(item);
        }

        Ok(result)
    }

    pub async fn save_vote(
        &self,
        id: u32,
        timestamp: i64,
        positive: u32,
        negative: u32,
    ) -> Result<(), Box<dyn Error>> {
        debug!(
            "Save vote: id = {}, timestamp = {}, positive = {}, negative = {}",
            id, timestamp, positive, negative
        );

        let lock = self.inner.lock().await;
        let mut query =
            lock.prepare("INSERT OR REPLACE INTO votes (initiative_id, timestamp, positive, negative) VALUES (:id, :timestamp, :positive, :negative)")?;
        query.bind((":id", id as i64))?;
        query.bind((":timestamp", timestamp))?;
        query.bind((":positive", positive as i64))?;
        query.bind((":negative", negative as i64))?;
        query.next()?;

        Ok(())
    }
}
