//! Mockie - Mock HTTP Server
//!
//! Una herramienta para crear servidores HTTP mock configurables desde línea de comandos.
//!
//! # Ejemplos
//!
//! ```bash
//! # Levantar el servidor
//! mockie serve --port 3000
//!
//! # Agregar un endpoint
//! mockie add --method GET --path /api/users --response '{"users":[]}'
//!
//! # Listar endpoints
//! mockie list
//! ```

pub mod cli;
pub mod config;
pub mod error;
pub mod handlers;
pub mod models;
pub mod server;
pub mod storage;

pub use error::{AppError, Result};
