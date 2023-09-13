# Rack

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**name** | **String** |  | 
**facility_id** | Option<**String**> |  | [optional]
**site** | Option<[**crate::models::NestedSite**](NestedSite.md)> |  | 
**location** | Option<[**crate::models::NestedLocation**](NestedLocation.md)> |  | [optional]
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**status** | Option<[**crate::models::Status5**](Status_5.md)> |  | [optional]
**role** | Option<[**crate::models::NestedRackRole**](NestedRackRole.md)> |  | [optional]
**serial** | Option<**String**> |  | [optional]
**asset_tag** | Option<**String**> | A unique tag used to identify this rack | [optional]
**r#type** | Option<[**crate::models::Type6**](Type_6.md)> |  | [optional]
**width** | Option<[**crate::models::Width**](Width.md)> |  | [optional]
**u_height** | Option<**i32**> | Height in rack units | [optional]
**desc_units** | Option<**bool**> | Units are numbered top-to-bottom | [optional]
**outer_width** | Option<**i32**> | Outer dimension of rack (width) | [optional]
**outer_depth** | Option<**i32**> | Outer dimension of rack (depth) | [optional]
**outer_unit** | Option<[**crate::models::OuterUnit**](Outer_unit.md)> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**device_count** | Option<**i32**> |  | [optional][readonly]
**powerfeed_count** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


