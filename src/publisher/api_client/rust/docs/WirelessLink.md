# WirelessLink

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**interface_a** | Option<[**crate::models::NestedInterface**](NestedInterface.md)> |  | 
**interface_b** | Option<[**crate::models::NestedInterface**](NestedInterface.md)> |  | 
**ssid** | Option<**String**> |  | [optional]
**status** | Option<[**crate::models::Status1**](Status_1.md)> |  | [optional]
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**auth_type** | Option<[**crate::models::AuthType**](Auth_type.md)> |  | [optional]
**auth_cipher** | Option<[**crate::models::AuthCipher**](Auth_cipher.md)> |  | [optional]
**auth_psk** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


