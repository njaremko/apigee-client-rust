/*
 * Companies API
 *
 * Manage companies in an organization.  A company is a collection of developers managed as a single entity. A company can be any grouping that is appropriate to your organization, for example, business unit, product line, or division. Grouping developers into companies is useful when your goal is to work with multiple developers associated under a single corporate entity for billing purposes, for example.  Companies are optional. It not required that the developers in your organization are associated with a company. Note that a developer is always a single entity, uniquely identified by the email element.
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `create_company`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCompanyError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_company`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCompanyError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_company`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCompanyError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_companies`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCompaniesError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `update_company`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCompanyError {
    Status400(),
    UnknownValue(serde_json::Value),
}


/// Creates a company in an organization.  The company is always created with a status of `active`. To set the status explicitly, see <a href=\"/docs/companies/1/routes/organizations/%7Borg_name%7D/companies/%7Bcompany_name%7D/put\">Update a company</a>.  **Note**: Do not set the company name to `0` (number or string). This value is not supported and will cause unexpected results.  To set the company's billing type (`PREPAID` or `POSTPAID`), make sure the organization profile is configured to support your setting (for example, set to accept `BOTH`). See <a href=\"https://docs.apigee.com/monetization/content/edit-organization-profile\">Edit the organization profile</a>.  For a list of attributes that are supported for monetization, see <a href=\"https://docs.apigee.com/api-platform/publish/adding-developers-your-api-product#monetization-attributes\">Configuring monetization attibutes</a>.
pub async fn create_company(configuration: &configuration::Configuration, org_name: &str, company_request: Option<crate::models::CompanyRequest>) -> Result<crate::models::Company, Error<CreateCompanyError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/organizations/{org_name}/companies", configuration.base_path, org_name=crate::apis::urlencode(org_name));
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&company_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateCompanyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a company.
pub async fn delete_company(configuration: &configuration::Configuration, org_name: &str, company_name: &str) -> Result<crate::models::Company, Error<DeleteCompanyError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/organizations/{org_name}/companies/{company_name}", configuration.base_path, org_name=crate::apis::urlencode(org_name), company_name=crate::apis::urlencode(company_name));
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteCompanyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets company details.
pub async fn get_company(configuration: &configuration::Configuration, org_name: &str, company_name: &str) -> Result<crate::models::Company, Error<GetCompanyError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/organizations/{org_name}/companies/{company_name}", configuration.base_path, org_name=crate::apis::urlencode(org_name), company_name=crate::apis::urlencode(company_name));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetCompanyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists all companies in an organization. Optionally, you can return an expanded list of companies, displaying a full profile for each company in the organization.  **With Apigee Edge for Public Cloud**: * The maximum number of companies returned is **1000**. * You can paginate the list of companies returned using the `startKey` and `count` query parameters
pub async fn list_companies(configuration: &configuration::Configuration, org_name: &str, expand: Option<bool>, start_key: Option<&str>, count: Option<&str>) -> Result<crate::models::Companies, Error<ListCompaniesError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/organizations/{org_name}/companies", configuration.base_path, org_name=crate::apis::urlencode(org_name));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = expand {
        local_var_req_builder = local_var_req_builder.query(&[("expand", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_key {
        local_var_req_builder = local_var_req_builder.query(&[("startKey", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = count {
        local_var_req_builder = local_var_req_builder.query(&[("count", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListCompaniesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates an existing company.  Send the complete company record as a payload with any changes you want to make. For a list of attributes that are supported for monetization, see <a href=\"https://docs.apigee.com/api-platform/publish/adding-developers-your-api-product#monetization-attributes\">Configuring monetization attibutes</a>.  To update the status of a company, set the `action` query parameter to `active` or `inactive` and `Content-type` to `application/octet-stream`. If you set the status to `inactive`, you cannot access the developers and apps associated with the company. In this case, the API returns a `204: No Content` status.  **Notes**: * You cannot update the status of a company by passing the value in the request body. * Currently, updating the status of a company cannot be executed using the **Try this API panel**.  If you want to change the company's billing type (`PREPAID` or `POSTPAID`), make sure the organization profile is configured to support your setting (for example, set to accept `BOTH`). See <a href=\"https://docs.apigee.com/monetization/content/edit-organization-profile\">Edit the organization profile</a>.
pub async fn update_company(configuration: &configuration::Configuration, org_name: &str, company_name: &str, action: Option<&str>, company_request: Option<crate::models::CompanyRequest>) -> Result<crate::models::Company, Error<UpdateCompanyError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/organizations/{org_name}/companies/{company_name}", configuration.base_path, org_name=crate::apis::urlencode(org_name), company_name=crate::apis::urlencode(company_name));
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = action {
        local_var_req_builder = local_var_req_builder.query(&[("action", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&company_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateCompanyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

