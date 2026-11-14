//! OpenAPI 3.1 document generation.
//!
//! Baseline document is generated from `utoipa`. Path annotations are added
//! as real endpoints are implemented; `iaop-api-contracts` remains the
//! canonical contract source of truth.

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Sales API",
        version = "0.1.0",
        description = "IAOP Sales service API. Contracts are governed by iaop-api-contracts."
    ),
    servers((url = "/"))
)]
pub struct ApiDoc;
