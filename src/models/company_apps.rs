/*
 * Company apps API
 *
 * A consumer/client app associated with a company entity. For more information,     see <a href=\"https://docs.apigee.com/api-platform/publish/creating-apps-surface-your-api\">Register apps and manage API keys</a>.
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CompanyApps : List of company app details.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompanyApps {
    #[serde(rename = "app", skip_serializing_if = "Option::is_none")]
    pub app: Option<Vec<crate::models::CompanyApp>>,
}

impl CompanyApps {
    /// List of company app details.
    pub fn new() -> CompanyApps {
        CompanyApps {
            app: None,
        }
    }
}


