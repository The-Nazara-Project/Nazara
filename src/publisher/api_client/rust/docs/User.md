# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**username** | **String** | Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only. | 
**password** | **String** |  | 
**first_name** | Option<**String**> |  | [optional]
**last_name** | Option<**String**> |  | [optional]
**email** | Option<**String**> |  | [optional]
**is_staff** | Option<**bool**> | Designates whether the user can log into this admin site. | [optional]
**is_active** | Option<**bool**> | Designates whether this user should be treated as active. Unselect this instead of deleting accounts. | [optional]
**date_joined** | Option<**String**> |  | [optional]
**groups** | Option<[**Vec<crate::models::NestedGroup>**](NestedGroup.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


