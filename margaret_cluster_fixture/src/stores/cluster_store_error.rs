use thiserror::Error;

use margaret::framework::database::database_error::DatabaseError;
use margaret::framework::tokio_postgres;

#[derive(Debug, Error)]
pub enum ClusterStoreError {
    #[error("failed to delete the note: {0}")]
    DeleteNote(#[source] tokio_postgres::Error),

    #[error("failed to end the session: {0}")]
    EndSession(#[source] tokio_postgres::Error),

    #[error("failed to find the note: {0}")]
    FindNote(#[source] tokio_postgres::Error),

    #[error("failed to find the upload: {0}")]
    FindUpload(#[source] tokio_postgres::Error),

    #[error("failed to find the user of a session: {0}")]
    FindSessionUser(#[source] tokio_postgres::Error),

    #[error("failed to find the name of a user: {0}")]
    FindUserName(#[source] tokio_postgres::Error),

    #[error("failed to insert the note: {0}")]
    InsertNote(#[source] tokio_postgres::Error),

    #[error("failed to insert the upload: {0}")]
    InsertUpload(#[source] tokio_postgres::Error),

    #[error("failed to list the messages: {0}")]
    ListMessages(#[source] tokio_postgres::Error),

    #[error("failed to list the notes: {0}")]
    ListNotes(#[source] tokio_postgres::Error),

    #[error("a message row does not match the message model: {0}")]
    MalformedMessageRow(#[source] tokio_postgres::Error),

    #[error("a note row does not match the note model: {0}")]
    MalformedNoteRow(#[source] tokio_postgres::Error),

    #[error("a session row does not match the user model: {0}")]
    MalformedSessionRow(#[source] tokio_postgres::Error),

    #[error("an upload row does not match the upload model: {0}")]
    MalformedUploadRow(#[source] tokio_postgres::Error),

    #[error("a user row does not match the user model: {0}")]
    MalformedUserRow(#[source] tokio_postgres::Error),

    #[error("failed to post the message: {0}")]
    PostMessage(#[source] tokio_postgres::Error),

    #[error("failed to record the token acquisition: {0}")]
    RecordTokenAcquisition(#[source] tokio_postgres::Error),

    #[error("failed to seed the cluster: {0}")]
    Seed(#[source] tokio_postgres::Error),

    #[error("failed to start a session: {0}")]
    StartSession(#[source] tokio_postgres::Error),

    #[error("failed to sweep the expired notes: {0}")]
    SweepNotes(#[source] tokio_postgres::Error),

    #[error("the cluster database is unavailable: {0}")]
    Unavailable(#[source] DatabaseError),

    #[error("failed to update the note: {0}")]
    UpdateNote(#[source] tokio_postgres::Error),
}
