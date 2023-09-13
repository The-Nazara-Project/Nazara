# WritableRack

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**name** | **String** |  | 
**facility_id** | Option<**String**> |  | [optional]
**site** | **i32** |  | 
**location** | Option<**i32**> |  | 
**tenant** | Option<**i32**> |  | [optional]
**status** | Option<**String**> |  | [optional]
**role** | Option<**i32**> | Functional role | [optional]
**serial** | Option<**String**> |  | [optional]
**asset_tag** | Option<**String**> | A unique tag used to identify this rack | [optional]
**r#type** | Option<**String**> |  | [optional]
**width** | Option<**i32**> | Rail-to-rail width | [optional]
**u_height** | Option<**i32**> | Height in rack units | [optional]
**desc_units** | Option<**bool**> | Units are numbered top-to-bottom | [optional]
**outer_width** | Option<**i32**> | Outer dimension of rack (width) | [optional]
**outer_depth** | Option<**i32**> | Outer dimension of rack (depth) | [optional]
**outer_unit** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**device_count** | Option<**i32**> |  | [optional][readonly]
**powerfeed_count** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


