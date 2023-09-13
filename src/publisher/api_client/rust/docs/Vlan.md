# Vlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**site** | Option<[**crate::models::NestedSite**](NestedSite.md)> |  | [optional]
**group** | Option<[**crate::models::NestedVlanGroup**](NestedVLANGroup.md)> |  | [optional]
**vid** | **i32** |  | 
**name** | **String** |  | 
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**status** | Option<[**crate::models::Status8**](Status_8.md)> |  | [optional]
**role** | Option<[**crate::models::NestedRole**](NestedRole.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**l2vpn_termination** | Option<[**crate::models::NestedL2VpnTermination**](NestedL2VPNTermination.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**prefix_count** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


