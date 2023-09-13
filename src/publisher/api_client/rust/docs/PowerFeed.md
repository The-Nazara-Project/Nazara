# PowerFeed

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**power_panel** | [**crate::models::NestedPowerPanel**](NestedPowerPanel.md) |  | 
**rack** | Option<[**crate::models::NestedRack**](NestedRack.md)> |  | [optional]
**name** | **String** |  | 
**status** | Option<[**crate::models::Status4**](Status_4.md)> |  | [optional]
**r#type** | Option<[**crate::models::Type3**](Type_3.md)> |  | [optional]
**supply** | Option<[**crate::models::Supply**](Supply.md)> |  | [optional]
**phase** | Option<[**crate::models::Phase**](Phase.md)> |  | [optional]
**voltage** | Option<**i32**> |  | [optional]
**amperage** | Option<**i32**> |  | [optional]
**max_utilization** | Option<**i32**> | Maximum permissible draw (percentage) | [optional]
**comments** | Option<**String**> |  | [optional]
**mark_connected** | Option<**bool**> | Treat as if a cable is connected | [optional]
**cable** | Option<[**crate::models::NestedCable**](NestedCable.md)> |  | [optional]
**cable_end** | Option<**String**> |  | [optional][readonly]
**link_peers** | Option<**Vec<String>**> |  Return the appropriate serializer for the link termination model.  | [optional][readonly]
**link_peers_type** | Option<**String**> |  | [optional][readonly]
**connected_endpoints** | Option<**Vec<String>**> |  Return the appropriate serializer for the type of connected object.  | [optional][readonly]
**connected_endpoints_type** | Option<**String**> |  | [optional][readonly]
**connected_endpoints_reachable** | Option<**bool**> |  | [optional][readonly]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**_occupied** | Option<**bool**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


