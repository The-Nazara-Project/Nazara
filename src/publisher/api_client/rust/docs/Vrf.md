# Vrf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**name** | **String** |  | 
**rd** | Option<**String**> | Unique route distinguisher (as defined in RFC 4364) | [optional]
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**enforce_unique** | Option<**bool**> | Prevent duplicate prefixes/IP addresses within this VRF | [optional]
**description** | Option<**String**> |  | [optional]
**import_targets** | Option<[**Vec<crate::models::NestedRouteTarget>**](NestedRouteTarget.md)> |  | [optional]
**export_targets** | Option<[**Vec<crate::models::NestedRouteTarget>**](NestedRouteTarget.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**ipaddress_count** | Option<**i32**> |  | [optional][readonly]
**prefix_count** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


