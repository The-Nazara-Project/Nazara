# WritableVmInterface

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**virtual_machine** | **i32** |  | 
**name** | **String** |  | 
**enabled** | Option<**bool**> |  | [optional]
**parent** | Option<**i32**> |  | [optional]
**bridge** | Option<**i32**> |  | [optional]
**mtu** | Option<**i32**> |  | [optional]
**mac_address** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**mode** | Option<**String**> |  | [optional]
**untagged_vlan** | Option<**i32**> |  | [optional]
**tagged_vlans** | Option<**Vec<i32>**> |  | [optional]
**vrf** | Option<**i32**> |  | [optional]
**l2vpn_termination** | Option<**String**> |  | [optional][readonly]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**count_ipaddresses** | Option<**i32**> |  | [optional][readonly]
**count_fhrp_groups** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


