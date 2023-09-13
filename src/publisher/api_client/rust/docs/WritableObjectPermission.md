# WritableObjectPermission

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**object_types** | **Vec<String>** |  | 
**groups** | Option<**Vec<i32>**> |  | [optional]
**users** | Option<**Vec<i32>**> |  | [optional]
**actions** | **Vec<String>** | The list of actions granted by this permission | 
**constraints** | Option<[**serde_json::Value**](.md)> | Queryset filter matching the applicable objects of the selected type(s) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


