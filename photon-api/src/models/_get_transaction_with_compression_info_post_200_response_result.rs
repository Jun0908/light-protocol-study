/*
 * photon-indexer
 *
 * Solana indexer for general compression
 *
 * The version of the OpenAPI document: 0.28.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// GetTransactionWithCompressionInfoPost200ResponseResult : A Solana transaction with additional compression information
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetTransactionWithCompressionInfoPost200ResponseResult {
    #[serde(rename = "compression_info", skip_serializing_if = "Option::is_none")]
    pub compression_info:
        Option<Box<models::GetTransactionWithCompressionInfoPost200ResponseResultCompressionInfo>>,
    /// An encoded confirmed transaction with status meta
    #[serde(rename = "transaction", skip_serializing_if = "Option::is_none")]
    pub transaction: Option<serde_json::Value>,
}

impl GetTransactionWithCompressionInfoPost200ResponseResult {
    /// A Solana transaction with additional compression information
    pub fn new() -> GetTransactionWithCompressionInfoPost200ResponseResult {
        GetTransactionWithCompressionInfoPost200ResponseResult {
            compression_info: None,
            transaction: None,
        }
    }
}
