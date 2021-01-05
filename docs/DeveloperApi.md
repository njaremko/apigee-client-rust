# \DeveloperApi

All URIs are relative to *https://api.enterprise.apigee.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_developer**](DeveloperApi.md#create_developer) | **post** /organizations/{org_name}/developers | Create developer
[**delete_developer**](DeveloperApi.md#delete_developer) | **delete** /organizations/{org_name}/developers/{developer_email} | Delete developer
[**delete_developer_attribute**](DeveloperApi.md#delete_developer_attribute) | **delete** /organizations/{org_name}/developers/{developer_email}/attributes/{attribute_name} | Delete developer attribute
[**get_developer**](DeveloperApi.md#get_developer) | **get** /organizations/{org_name}/developers/{developer_email} | Get developer
[**get_developer_attribute**](DeveloperApi.md#get_developer_attribute) | **get** /organizations/{org_name}/developers/{developer_email}/attributes/{attribute_name} | Get developer attribute
[**get_developer_attributes**](DeveloperApi.md#get_developer_attributes) | **get** /organizations/{org_name}/developers/{developer_email}/attributes | Get developer attributes
[**list_developers**](DeveloperApi.md#list_developers) | **get** /organizations/{org_name}/developers | List developers
[**set_developer_status**](DeveloperApi.md#set_developer_status) | **post** /organizations/{org_name}/developers/{developer_email} | Set developer status
[**update_developer**](DeveloperApi.md#update_developer) | **put** /organizations/{org_name}/developers/{developer_email} | Update developer
[**update_developer_attribute**](DeveloperApi.md#update_developer_attribute) | **post** /organizations/{org_name}/developers/{developer_email}/attributes/{attribute_name} | Update developer attribute
[**update_developer_attributes**](DeveloperApi.md#update_developer_attributes) | **post** /organizations/{org_name}/developers/{developer_email}/attributes | Update developer attributes



## create_developer

> crate::models::Developer create_developer(org_name, developer_request)
Create developer

Creates a profile for a developer in an organization. After the developer is created, they can register an app and receive an API key.  The developer is always created with a status of `active`. To set the status explicitly, use the <a href=\"/docs/developers/1/routes/organizations/%7Borg_name%7D/developers/%7Bdeveloper_email%7D/post\">Set developer status API</a>.  When creating a developer, you can define one or more monetization attributes as custom attributes. For more information about the monetization attributes, see <a href=\"https://docs.apigee.com/api-platform/publish/adding-developers-your-api-product#monetization-attributes\">Configuring monetization attributes</a>. For example:   ``` {   \"email\" : \"developer_email\",   \"firstName\" : \"first_name\",   \"lastName\" : \"last_name\",   \"userName\" : \"user_name\",   \"attributes\" : [{      \"name\": \"MINT_BILLING_TYPE\",      \"value\": \"one of PREPAID | POSTPAID\"   }] } ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Organization name. | [required] |
**developer_request** | Option<[**DeveloperRequest**](DeveloperRequest.md)> | Developer details. |  |

### Return type

[**crate::models::Developer**](Developer.md)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_developer

> crate::models::Developer delete_developer(org_name, developer_email)
Delete developer

Deletes a developer from an organization. All apps and API keys associated with the developer are also removed from the organization.  **Note**: To avoid permanently deleting developers and their artifacts, consider deactivating developers instead using the <a href=\"/docs/developers/1/routes/organizations/%7Borg_name%7D/developers/%7Bdeveloper_email%7D/post\">Set developer status API</a>.  With Apigee Edge for Public Cloud, deletion of the developer and associated artifacts happens asynchronously. The developer is deleted immediately, but the resources associated with that developer, such as apps, may take anywhere from a few seconds to a few minutes to be deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Organization name. | [required] |
**developer_email** | **String** | Email address for the developer. | [required] |

### Return type

[**crate::models::Developer**](Developer.md)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_developer_attribute

> crate::models::Attribute delete_developer_attribute(org_name, developer_email, attribute_name)
Delete developer attribute

Deletes a developer attribute.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Organization name. | [required] |
**developer_email** | **String** | Email address for the developer. | [required] |
**attribute_name** | **String** | Name of the attribute. | [required] |

### Return type

[**crate::models::Attribute**](Attribute.md)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_developer

> crate::models::Developer get_developer(org_name, developer_email)
Get developer

Gets the profile for a developer by email address. With Apigee Edge for Public Cloud, the response includes only the first 100 apps. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Organization name. | [required] |
**developer_email** | **String** | Email address for the developer. | [required] |

### Return type

[**crate::models::Developer**](Developer.md)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_developer_attribute

> crate::models::Attribute get_developer_attribute(org_name, developer_email, attribute_name)
Get developer attribute

Gets the value of a developer attribute.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Organization name. | [required] |
**developer_email** | **String** | Email address for the developer. | [required] |
**attribute_name** | **String** | Name of the attribute. | [required] |

### Return type

[**crate::models::Attribute**](Attribute.md)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_developer_attributes

> crate::models::Attributes get_developer_attributes(org_name, developer_email)
Get developer attributes

Gets developer attributes and their values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Organization name. | [required] |
**developer_email** | **String** | Email address for the developer. | [required] |

### Return type

[**crate::models::Attributes**](Attributes.md)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_developers

> crate::models::OneOfarrayDevelopers list_developers(org_name, expand, count, start_key, app)
List developers

Lists all developers in an organization by email address. This call does not list any company developers who are a part of the organization.  To get the developers associated with a specific app, specify the name of the app using the `app` query parameter.  With Apigee Edge for Public Cloud:  * The limit on the number of developers returned is 1000. * Paginate the list of developers returned using the `startKey` and `count` query parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Organization name. | [required] |
**expand** | Option<**bool**> | Flag that specifies whether to view expanded details for each developer. Set to `true` to view expanded details. Defaults to `false`. |  |
**count** | Option<**i32**> | **Apigee Edge for Public Cloud only**. Number of developers to return in the API call. The limit is 1000. Use with the `startKey` parameter to provide more targeted filtering. |  |
**start_key** | Option<**String**> | **Apigee Edge for Public Cloud only**. Email of a developer from which to start displaying the list of developers.  For example, if the an unfiltered list returns:  ``` westley@example.com fezzik@example.com   buttercup@example.com    ```  If your `startKey` is `fezzik@example.com`, the returned list will be:  ``` fezzik@example.com   buttercup@example.com  ``` |  |
**app** | Option<**String**> | Name of the app for which you want to return associated developers. |  |

### Return type

[**crate::models::OneOfarrayDevelopers**](oneOf<array,Developers>.md)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_developer_status

> set_developer_status(org_name, developer_email, action)
Set developer status

Sets a developer's status to `active` or `inactive` for a specific organization. Run this API for each organization where you want to change the developer's status.   By default, the status of a developer is set to `active`. If you set a developer's status to `inactive`, the API keys assigned to the developer's apps are no longer valid even though keys continue to show a status of \"Approved\". Inactive developers can still sign in to the developer portal and create apps; however, the new keys that get created won't be valid until the developer is set to `active`.  The HTTP status code for success is: `204 No Content`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Organization name. | [required] |
**developer_email** | **String** | Email address for the developer. | [required] |
**action** | **String** | Status of developer. Set to `active` or `inactive`. | [required] |

### Return type

 (empty response body)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_developer

> crate::models::Developer update_developer(org_name, developer_email, developer_request)
Update developer

Update an existing developer profile.  To add new values or update existing values, submit the new or updated portion of the developer profile along with the rest of the existing developer profile, even if no values are changing.  To delete attributes from a developer profile, submit the entire profile without the attributes that you want to delete.  **Apigee Edge for Public Cloud only**: OAuth access tokens and Key Management Service (KMS) entities (apps, developers, and API products) are cached for 180 seconds (current default). Any custom attributes associated with these entities also get cached for at least 180 seconds after the entity is accessed at runtime. Therefore, an ExpiresIn element on the OAuthV2 policy won't be able to expire an access token in less than 180 seconds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Organization name. | [required] |
**developer_email** | **String** | Email address for the developer. | [required] |
**developer_request** | Option<[**DeveloperRequest**](DeveloperRequest.md)> | Developer details. |  |

### Return type

[**crate::models::Developer**](Developer.md)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_developer_attribute

> crate::models::Attribute update_developer_attribute(org_name, developer_email, attribute_name, inline_object)
Update developer attribute

Update the value of a developer attribute.  **Apigee Edge for Public Cloud only**: OAuth access tokens and Key Management Service (KMS) entities (apps, developers, and API products) are cached for 180 seconds (current default). Any custom attributes associated with these entities also get cached for at least 180 seconds after the entity is accessed at runtime. Therefore, an ExpiresIn element on the OAuthV2 policy won't be able to expire an access token in less than 180 seconds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Organization name. | [required] |
**developer_email** | **String** | Email address for the developer. | [required] |
**attribute_name** | **String** | Name of the attribute. | [required] |
**inline_object** | Option<[**InlineObject**](InlineObject.md)> |  |  |

### Return type

[**crate::models::Attribute**](Attribute.md)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_developer_attributes

> crate::models::Attributes update_developer_attributes(org_name, developer_email, attributes)
Update developer attributes

Updates or creates developer attributes.  This API replaces the current list of attributes with the attributes specified in the request body. This lets you update existing attributes, add new attributes, or delete existing attributes by omitting them from the request body.  **Apigee Edge for Public Cloud only**: OAuth access tokens and Key Management Service (KMS) entities (apps, developers, and API products) are cached for 180 seconds (current default). Any custom attributes associated with these entities also get cached for at least 180 seconds after the entity is accessed at runtime. Therefore, an ExpiresIn element on the OAuthV2 policy won't be able to expire an access token in less than 180 seconds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** | Organization name. | [required] |
**developer_email** | **String** | Email address for the developer. | [required] |
**attributes** | Option<[**Attributes**](Attributes.md)> | Developer attributes. |  |

### Return type

[**crate::models::Attributes**](Attributes.md)

### Authorization

[Basic](../README.md#Basic), [OAuth](../README.md#OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

