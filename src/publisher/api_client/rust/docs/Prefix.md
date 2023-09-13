# Prefix

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**family** | Option<[**crate::models::Family**](Family.md)> |  | [optional]
**prefix** | **String** | IPv4 or IPv6 network with mask | 
**site** | Option<[**crate::models::NestedSite**](NestedSite.md)> |  | [optional]
**vrf** | Option<[**crate::models::NestedVrf**](NestedVRF.md)> |  | [optional]
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**vlan** | Option<[**crate::models::NestedVlan**](NestedVLAN.md)> |  | [optional]
**status** | Option<[**crate::models::Status9**](Status_9.md)> |  | [optional]
**role** | Option<[**crate::models::NestedRole**](NestedRole.md)> |  | [optional]
**is_pool** | Option<**bool**> | All IP addresses within this prefix are considered usable | [optional]
**mark_utilized** | Option<**bool**> | Treat as 100% utilized | [optional]
**description** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**children** | Option<**i32**> |  | [optional][readonly]
**_depth** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


