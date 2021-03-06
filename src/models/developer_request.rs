/*
 * Developers API
 *
 * Developers must register with an organization on Apigee Edge. After they are registered, developers register their apps, choose the APIs they want to use, and receive the unique API credentials (consumer keys and secrets) needed to access your APIs.
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DeveloperRequest : Developer request.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeveloperRequest {
    /// List of attributes that can be used to extend the default developer profile. With Apigee Edge for Public Cloud, the custom attribute limit is 18.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<crate::models::Attribute>>,
    /// Email address of the developer. This value is used to uniquely identify the developer in Apigee Edge.
    #[serde(rename = "email")]
    pub email: String,
    /// First name of the developer.
    #[serde(rename = "firstName")]
    pub first_name: String,
    /// Last name of the developer.
    #[serde(rename = "lastName")]
    pub last_name: String,
    /// Username. Not used by Apigee.
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl DeveloperRequest {
    /// Developer request.
    pub fn new(email: String, first_name: String, last_name: String) -> DeveloperRequest {
        DeveloperRequest {
            attributes: None,
            email,
            first_name,
            last_name,
            username: None,
        }
    }
}


