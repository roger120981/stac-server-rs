//! STAC API server implementation using [axum](https://github.com/tokio-rs/axum).

#![deny(
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_debug_implementations,
    missing_docs,
    non_ascii_idents,
    noop_method_call,
    pointer_structural_match,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results
)]

mod config;
mod error;
mod router;

use tokio::net::TcpListener;
pub use {config::Config, error::Error, router::api};

/// Crate-specific result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Starts a server.
///
/// # Examples
///
/// ```no_run
/// use stac_api_backend::MemoryBackend;
/// use stac_server::Config;
///
/// # tokio_test::block_on(async {
/// // Runs forever
/// stac_server::serve(MemoryBackend::new(), Config::default()).await.unwrap();
/// # });
/// ```
pub async fn serve<B>(backend: B, config: Config) -> Result<()>
where
    B: stac_api_backend::Backend,
    stac_api_backend::Error: From<<B as stac_api_backend::Backend>::Error>,
{
    let listener = TcpListener::bind(&config.addr).await?;
    let api = api(backend, config)?;
    axum::serve(listener, api).await.map_err(Error::from)
}

// Needed for integration tests.
#[cfg(test)]
use {
    futures_util as _, geojson as _, stac_async as _, stac_validate as _, tokio_postgres as _,
    tokio_test as _,
};
