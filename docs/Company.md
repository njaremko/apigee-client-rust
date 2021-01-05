# Company

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**apps** | Option<**Vec<String>**> | Output only. Apps associated with the company. | [optional]
**name** | Option<**String**> | Name of the company. See <a href=\"https://docs.apigee.com/api-platform/reference/naming-guidelines\">naming restrictions</a>. Required when creating a company. | [optional]
**display_name** | Option<**String**> | Display name for the company. | [optional]
**organization** | Option<**String**> | Output only. Organization name. | [optional]
**status** | Option<**String**> | Output only. Status of the company. Valid values are `active` or `inactive`. Defaults to `active` when creating the company. To change the value, see Update a company. | [optional]
**attributes** | Option<[**Vec<crate::models::CompanyAttributes>**](Company_attributes.md)> | Name/value formatted attributes used to extend the default company profile. **Note**: With Apigee Edge for Public Cloud, the custom attribute limit is 18. | [optional]
**created_at** | Option<**i32**> | Output only. Time when the company was last modified in seconds since epoch. | [optional]
**last_modified_at** | Option<**i32**> | Output only. Time when the company was last modified in seconds since epoch. | [optional]
**last_modified_by** | Option<**String**> | Output only. Email address of developer that last modified the company. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


