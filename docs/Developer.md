# Developer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**apps** | Option<**Vec<String>**> | Output only. List of apps associated with the developer. | [optional]
**attributes** | Option<[**Vec<crate::models::Attribute>**](Attribute.md)> | List of attributes that can be used to extend the default developer profile. With Apigee Edge for Public Cloud, the custom attribute limit is 18. | [optional]
**companies** | Option<**Vec<String>**> | Output only. List of companies associated with the developer. | [optional]
**created_at** | Option<**i32**> | Output only. Time the developer was created in milliseconds since epoch. | [optional]
**created_by** | Option<**String**> | Output only. Email address of the developer that created the developer. | [optional]
**developer_id** | Option<**String**> | ID of the developer. Generated internally by Apigee and not guaranteed to stay consistent over time.  | [optional]
**email** | Option<**String**> | Email address of the developer. This value is used to uniquely identify the developer in Apigee Edge. | [optional]
**first_name** | Option<**String**> | First name of the developer. | [optional]
**last_name** | Option<**String**> | Last name of the developer. | [optional]
**last_modified_at** | Option<**i32**> | Output only. Last modified time as milliseconds since epoch. | [optional]
**last_modified_by** | Option<**String**> | Output only. Email of developer that last modified the app. | [optional]
**organization_name** | Option<**String**> | Output only. Name of the organization associated with the developer. | [optional]
**status** | Option<[**serde_json::Value**](.md)> | Status of the developer. Valid values are `active` and `inactive`. | [optional]
**username** | Option<**String**> | Username. Not used by Apigee. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


