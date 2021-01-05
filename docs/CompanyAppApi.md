# \CompanyAppApi

All URIs are relative to *https://api.enterprise.apigee.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**approve_revoke_company_app**](CompanyAppApi.md#approve_revoke_company_app) | **post** /organizations/{org_name}/companies/{company_name}/apps/{app_name} | Approve or revoke a company app
[**create_company_app**](CompanyAppApi.md#create_company_app) | **post** /organizations/{org_name}/companies/{company_name}/apps | Create company app
[**delete_company_app**](CompanyAppApi.md#delete_company_app) | **delete** /organizations/{org_name}/companies/{company_name}/apps/{app_name} | Delete company app
[**get_company_app**](CompanyAppApi.md#get_company_app) | **get** /organizations/{org_name}/companies/{company_name}/apps/{app_name} | Get company app
[**list_company_apps**](CompanyAppApi.md#list_company_apps) | **get** /organizations/{org_name}/companies/{company_name}/apps | List company apps
[**organizations_org_name_companies_company_name_apps_app_name_put**](CompanyAppApi.md#organizations_org_name_companies_company_name_apps_app_name_put) | **put** /organizations/{org_name}/companies/{company_name}/apps/{app_name} | Update company app



## approve_revoke_company_app

> approve_revoke_company_app(org_name, company_name, app_name, action)
Approve or revoke a company app

Sets the API key status of a company app to `approved` or `revoked`. If a company app is revoked, none of its API keys are valid for API calls, even though the keys themselves still display an \"Approved\" status. The HTTP status code for success is: `204 No Content`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Name of the organization. | [required] |
**company_name** | **String** | Name of the company. | [required] |
**app_name** | **String** | Name of the application. | [required] |
**action** | **String** | Action to perform. Valid values include `approved` or `revoked`. | [required] |

### Return type

 (empty response body)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_company_app

> crate::models::CompanyApp create_company_app(org_name, company_name, company_app)
Create company app

Creates an app associated with a company, associates the app with an API product, and auto-generates an API key for the app to use in calls to API proxies inside the API product. You must create a profile for the company in your organization before you can register apps that are associated with the company.  **Ensure optimal API product and app security**  An organization-level property, `features.keymanagement.disable.unbounded.permissions`, strengthens the security of API products in verifying API calls. When the property is set to `true`, the following features are enforced.  * **App creation**: When creating a developer or company app, the Edge API requires that the app be associated with an API product. (The Edge UI already enforces this.)   * **API product configuration**: To create or update an API product, the API product must include at least one API proxy or a resource path in its definition.  * **Runtime security**: API calls are rejected by an API product in the following situations:    * An API product doesn't include at least one API proxy or resource path.     * If the `flow.resource.name` variable in the message doesn't include a resource path that the API product can evaluate.    * If the app making the API call isn't associated with an API product.   **Note:** Setting this organization property requires system administrator rights. Edge for Private Cloud system administrators can make add this property on their own with the Update organization properties API. If you are an Edge for Public Cloud user and the previously described restrictions are not in place, contact Apigee Support to set the organization property for you.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Name of the organization. | [required] |
**company_name** | **String** | Name of the company. | [required] |
**company_app** | Option<[**CompanyApp**](CompanyApp.md)> | Company app details. |  |

### Return type

[**crate::models::CompanyApp**](CompanyApp.md)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_company_app

> crate::models::CompanyApp delete_company_app(org_name, company_name, app_name)
Delete company app

Deletes a company app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Name of the organization. | [required] |
**company_name** | **String** | Name of the company. | [required] |
**app_name** | **String** | Name of the application. | [required] |

### Return type

[**crate::models::CompanyApp**](CompanyApp.md)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_company_app

> crate::models::CompanyApp get_company_app(org_name, company_name, app_name)
Get company app

Gets the profile for a company app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Name of the organization. | [required] |
**company_name** | **String** | Name of the company. | [required] |
**app_name** | **String** | Name of the application. | [required] |

### Return type

[**crate::models::CompanyApp**](CompanyApp.md)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_company_apps

> crate::models::OneOfarrayCompanyApps list_company_apps(org_name, company_name, expand, key_status, count, start_key)
List company apps

Lists company apps in an organization. Optionally, you can expand the response to include the profile for each app.  With Apigee Edge for Public Cloud: * A maximum of 100 company apps are returned per API call. * You can paginate the list of company apps returned using the `startKey` and `count` query parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Name of the organization. | [required] |
**company_name** | **String** | Name of the company. | [required] |
**expand** | Option<**bool**> | Flag that specifies whether to view expanded details for each app. Set to `true` to view expanded details. Defaults to `false`. Not applicable if you use the `count` or `startKey` query parameters. |  |
**key_status** | Option<**String**> | **Apigee Edge for Private Cloud only**. Filter list to apps with specific key status. Valid values include `approved`, `pending`, or `revoked`. |  |
**count** | Option<**i32**> | **Apigee Edge for Public Cloud only**. Number of company apps to return in the API call. The limit is 100. Required if you specify `startKey`. |  |
**start_key** | Option<**String**> | **Apigee Edge for Public Cloud only**. ID of the app from which to start displaying the list of apps.    For example, if the unfiltered list includes the following app names:  ``` \"companyApp1\", \"companyApp2\", \"companyApp3\" ```  If you set the `startKey` to `companyApp2`, the list will include:  ``` \"companyrApp2\", \"companyApp3\" ``` |  |

### Return type

[**crate::models::OneOfarrayCompanyApps**](oneOf<array,CompanyApps>.md)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_org_name_companies_company_name_apps_app_name_put

> crate::models::CompanyApp organizations_org_name_companies_company_name_apps_app_name_put(org_name, company_name, app_name, company_app)
Update company app

Updates an existing company app. **Note**: You must include all current attribute, API product, and callback values in the payload along with any changes you want to make; otherwise, the existing values are removed. To display the current values, see Get the company app API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Name of the organization. | [required] |
**company_name** | **String** | Name of the company. | [required] |
**app_name** | **String** | Name of the application. | [required] |
**company_app** | Option<[**CompanyApp**](CompanyApp.md)> | Company app details. |  |

### Return type

[**crate::models::CompanyApp**](CompanyApp.md)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

