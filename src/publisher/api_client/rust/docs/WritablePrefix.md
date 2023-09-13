# WritablePrefix

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**family** | Option<**String**> |  | [optional][readonly]
**prefix** | **String** | IPv4 or IPv6 network with mask | 
**site** | Option<**i32**> |  | [optional]
**vrf** | Option<**i32**> |  | [optional]
**tenant** | Option<**i32**> |  | [optional]
**vlan** | Option<**i32**> |  | [optional]
**status** | Option<**String**> | Operational status of this prefix | [optional]
**role** | Option<**i32**> | The primary function of this prefix | [optional]
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


