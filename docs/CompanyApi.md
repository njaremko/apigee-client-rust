# \CompanyApi

All URIs are relative to *https://api.enterprise.apigee.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_company**](CompanyApi.md#create_company) | **post** /organizations/{org_name}/companies | Create company
[**delete_company**](CompanyApi.md#delete_company) | **delete** /organizations/{org_name}/companies/{company_name} | Delete company
[**get_company**](CompanyApi.md#get_company) | **get** /organizations/{org_name}/companies/{company_name} | Get company details
[**list_companies**](CompanyApi.md#list_companies) | **get** /organizations/{org_name}/companies | List companies
[**update_company**](CompanyApi.md#update_company) | **put** /organizations/{org_name}/companies/{company_name} | Update a company.



## create_company

> crate::models::Company create_company(org_name, company_request)
Create company

Creates a company in an organization.  The company is always created with a status of `active`. To set the status explicitly, see <a href=\"/docs/companies/1/routes/organizations/%7Borg_name%7D/companies/%7Bcompany_name%7D/put\">Update a company</a>.  **Note**: Do not set the company name to `0` (number or string). This value is not supported and will cause unexpected results.  To set the company's billing type (`PREPAID` or `POSTPAID`), make sure the organization profile is configured to support your setting (for example, set to accept `BOTH`). See <a href=\"https://docs.apigee.com/monetization/content/edit-organization-profile\">Edit the organization profile</a>.  For a list of attributes that are supported for monetization, see <a href=\"https://docs.apigee.com/api-platform/publish/adding-developers-your-api-product#monetization-attributes\">Configuring monetization attibutes</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Organization name. | [required] |
**company_request** | Option<[**CompanyRequest**](CompanyRequest.md)> | Company configuration. |  |

### Return type

[**crate::models::Company**](Company.md)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_company

> crate::models::Company delete_company(org_name, company_name)
Delete company

Deletes a company.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Organization name. | [required] |
**company_name** | **String** | Company name. | [required] |

### Return type

[**crate::models::Company**](Company.md)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_company

> crate::models::Company get_company(org_name, company_name)
Get company details

Gets company details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Organization name. | [required] |
**company_name** | **String** | Company name. | [required] |

### Return type

[**crate::models::Company**](Company.md)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_companies

> crate::models::OneOfarrayCompanies list_companies(org_name, expand, start_key, count)
List companies

Lists all companies in an organization. Optionally, you can return an expanded list of companies, displaying a full profile for each company in the organization.  **With Apigee Edge for Public Cloud**: * The maximum number of companies returned is **1000**. * You can paginate the list of companies returned using the `startKey` and `count` query parameters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Organization name. | [required] |
**expand** | Option<**bool**> | Flag that specifies whether to expand details. Set to `true` to expand details. Defaults to `false`. |  |
**start_key** | Option<**String**> | **Apigee Edge for Public Cloud only**. Name of the company from which to start displaying the list of companies.  For example, if the an unfiltered list returns:  ``` company1 company2 company3 ```  If your `startKey` is `company1`, the returned list will be:  ``` company2 company3 ``` |  |
**count** | Option<**String**> | **Apigee Edge for Public Cloud only**: Number of companies to return in the list. The maximum limit is **1000**. Use with the `startkey` to provide more targeted filtering. |  |

### Return type

[**crate::models::OneOfarrayCompanies**](oneOf<array,Companies>.md)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_company

> crate::models::Company update_company(org_name, company_name, action, company_request)
Update a company.

Updates an existing company.  Send the complete company record as a payload with any changes you want to make. For a list of attributes that are supported for monetization, see <a href=\"https://docs.apigee.com/api-platform/publish/adding-developers-your-api-product#monetization-attributes\">Configuring monetization attibutes</a>.  To update the status of a company, set the `action` query parameter to `active` or `inactive` and `Content-type` to `application/octet-stream`. If you set the status to `inactive`, you cannot access the developers and apps associated with the company. In this case, the API returns a `204: No Content` status.  **Notes**: * You cannot update the status of a company by passing the value in the request body. * Currently, updating the status of a company cannot be executed using the **Try this API panel**.  If you want to change the company's billing type (`PREPAID` or `POSTPAID`), make sure the organization profile is configured to support your setting (for example, set to accept `BOTH`). See <a href=\"https://docs.apigee.com/monetization/content/edit-organization-profile\">Edit the organization profile</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Organization name. | [required] |
**company_name** | **String** | Company name. | [required] |
**action** | Option<**String**> | Set to `active` or `inactive`. |  |
**company_request** | Option<[**CompanyRequest**](CompanyRequest.md)> | Updated company configuration. |  |

### Return type

[**crate::models::Company**](Company.md)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

