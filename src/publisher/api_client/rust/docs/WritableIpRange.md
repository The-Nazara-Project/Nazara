# WritableIpRange

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**family** | Option<**String**> |  | [optional][readonly]
**start_address** | **String** | IPv4 or IPv6 address (with mask) | 
**end_address** | **String** | IPv4 or IPv6 address (with mask) | 
**size** | Option<**i32**> |  | [optional][readonly]
**vrf** | Option<**i32**> |  | [optional]
**tenant** | Option<**i32**> |  | [optional]
**status** | Option<**String**> | Operational status of this range | [optional]
**role** | Option<**i32**> | The primary function of this range | [optional]
**description** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**children** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


