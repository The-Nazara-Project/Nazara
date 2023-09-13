# VlanGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**name** | **String** |  | 
**slug** | **String** |  | 
**scope_type** | Option<**String**> |  | [optional]
**scope_id** | Option<**i32**> |  | [optional]
**scope** | Option<[**serde_json::Value**](.md)> |  | [optional][readonly]
**min_vid** | Option<**i32**> | Lowest permissible ID of a child VLAN | [optional]
**max_vid** | Option<**i32**> | Highest permissible ID of a child VLAN | [optional]
**description** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**vlan_count** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


