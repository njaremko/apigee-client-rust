# \CompanyDeveloperApi

All URIs are relative to *https://api.enterprise.apigee.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_update_company_developers**](CompanyDeveloperApi.md#add_update_company_developers) | **post** /organizations/{org_name}/companies/{company_name}/developers | Add or update company developers
[**list_developers_company**](CompanyDeveloperApi.md#list_developers_company) | **get** /organizations/{org_name}/companies/{company_name}/developers | List developers in company
[**remove_developer_company**](CompanyDeveloperApi.md#remove_developer_company) | **delete** /organizations/{org_name}/companies/{company_name}/developers/{developer_email} | Remove developer from company



## add_update_company_developers

> crate::models::CompanyDevelopers add_update_company_developers(org_name, company_name, company_developers)
Add or update company developers

Adds a developer to a company, or updates an existing developer in the company.  Optionally, you can assign a role to the developer, though it is not required. You can create roles for your partners, such as administrator or application developer. Those roles can then be assigned to specific partner employees.  When updating an existing developer, specify both the developer's email and role (if applicable).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Organization name. | [required] |
**company_name** | **String** | Company name. | [required] |
**company_developers** | Option<[**CompanyDevelopers**](CompanyDevelopers.md)> | Add or update a company developer. |  |

### Return type

[**crate::models::CompanyDevelopers**](companyDevelopers.md)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_developers_company

> crate::models::CompanyDevelopers list_developers_company(org_name, company_name)
List developers in company

Lists developers in a company.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Organization name. | [required] |
**company_name** | **String** | Company name. | [required] |

### Return type

[**crate::models::CompanyDevelopers**](companyDevelopers.md)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_developer_company

> remove_developer_company(org_name, company_name, developer_email)
Remove developer from company

Removes a developer from a company.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Organization name. | [required] |
**company_name** | **String** | Company name. | [required] |
**developer_email** | **String** | Email address of the developer. | [required] |

### Return type

 (empty response body)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

