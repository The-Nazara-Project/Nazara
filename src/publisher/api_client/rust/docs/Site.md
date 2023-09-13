# Site

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**name** | **String** |  | 
**slug** | **String** |  | 
**status** | Option<[**crate::models::Status3**](Status_3.md)> |  | [optional]
**region** | Option<[**crate::models::NestedRegion**](NestedRegion.md)> |  | [optional]
**group** | Option<[**crate::models::NestedSiteGroup**](NestedSiteGroup.md)> |  | [optional]
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**facility** | Option<**String**> | Local facility ID or description | [optional]
**time_zone** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**physical_address** | Option<**String**> |  | [optional]
**shipping_address** | Option<**String**> |  | [optional]
**latitude** | Option<**f32**> | GPS coordinate (latitude) | [optional]
**longitude** | Option<**f32**> | GPS coordinate (longitude) | [optional]
**comments** | Option<**String**> |  | [optional]
**asns** | Option<[**Vec<crate::models::NestedAsn>**](NestedASN.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**circuit_count** | Option<**i32**> |  | [optional][readonly]
**device_count** | Option<**i32**> |  | [optional][readonly]
**prefix_count** | Option<**i32**> |  | [optional][readonly]
**rack_count** | Option<**i32**> |  | [optional][readonly]
**virtualmachine_count** | Option<**i32**> |  | [optional][readonly]
**vlan_count** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


