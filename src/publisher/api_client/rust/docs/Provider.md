# Provider

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**name** | **String** |  | 
**slug** | **String** |  | 
**asn** | Option<**i32**> | 32-bit autonomous system number | [optional]
**account** | Option<**String**> |  | [optional]
**portal_url** | Option<**String**> |  | [optional]
**noc_contact** | Option<**String**> |  | [optional]
**admin_contact** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**asns** | Option<[**Vec<crate::models::NestedAsn>**](NestedASN.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**circuit_count** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


