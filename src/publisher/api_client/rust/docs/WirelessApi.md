# \WirelessApi

All URIs are relative to *https://netbox.suse.de/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**wireless_wireless_lan_groups_bulk_delete**](WirelessApi.md#wireless_wireless_lan_groups_bulk_delete) | **DELETE** /wireless/wireless-lan-groups/ | 
[**wireless_wireless_lan_groups_bulk_partial_update**](WirelessApi.md#wireless_wireless_lan_groups_bulk_partial_update) | **PATCH** /wireless/wireless-lan-groups/ | 
[**wireless_wireless_lan_groups_bulk_update**](WirelessApi.md#wireless_wireless_lan_groups_bulk_update) | **PUT** /wireless/wireless-lan-groups/ | 
[**wireless_wireless_lan_groups_create**](WirelessApi.md#wireless_wireless_lan_groups_create) | **POST** /wireless/wireless-lan-groups/ | 
[**wireless_wireless_lan_groups_delete**](WirelessApi.md#wireless_wireless_lan_groups_delete) | **DELETE** /wireless/wireless-lan-groups/{id}/ | 
[**wireless_wireless_lan_groups_list**](WirelessApi.md#wireless_wireless_lan_groups_list) | **GET** /wireless/wireless-lan-groups/ | 
[**wireless_wireless_lan_groups_partial_update**](WirelessApi.md#wireless_wireless_lan_groups_partial_update) | **PATCH** /wireless/wireless-lan-groups/{id}/ | 
[**wireless_wireless_lan_groups_read**](WirelessApi.md#wireless_wireless_lan_groups_read) | **GET** /wireless/wireless-lan-groups/{id}/ | 
[**wireless_wireless_lan_groups_update**](WirelessApi.md#wireless_wireless_lan_groups_update) | **PUT** /wireless/wireless-lan-groups/{id}/ | 
[**wireless_wireless_lans_bulk_delete**](WirelessApi.md#wireless_wireless_lans_bulk_delete) | **DELETE** /wireless/wireless-lans/ | 
[**wireless_wireless_lans_bulk_partial_update**](WirelessApi.md#wireless_wireless_lans_bulk_partial_update) | **PATCH** /wireless/wireless-lans/ | 
[**wireless_wireless_lans_bulk_update**](WirelessApi.md#wireless_wireless_lans_bulk_update) | **PUT** /wireless/wireless-lans/ | 
[**wireless_wireless_lans_create**](WirelessApi.md#wireless_wireless_lans_create) | **POST** /wireless/wireless-lans/ | 
[**wireless_wireless_lans_delete**](WirelessApi.md#wireless_wireless_lans_delete) | **DELETE** /wireless/wireless-lans/{id}/ | 
[**wireless_wireless_lans_list**](WirelessApi.md#wireless_wireless_lans_list) | **GET** /wireless/wireless-lans/ | 
[**wireless_wireless_lans_partial_update**](WirelessApi.md#wireless_wireless_lans_partial_update) | **PATCH** /wireless/wireless-lans/{id}/ | 
[**wireless_wireless_lans_read**](WirelessApi.md#wireless_wireless_lans_read) | **GET** /wireless/wireless-lans/{id}/ | 
[**wireless_wireless_lans_update**](WirelessApi.md#wireless_wireless_lans_update) | **PUT** /wireless/wireless-lans/{id}/ | 
[**wireless_wireless_links_bulk_delete**](WirelessApi.md#wireless_wireless_links_bulk_delete) | **DELETE** /wireless/wireless-links/ | 
[**wireless_wireless_links_bulk_partial_update**](WirelessApi.md#wireless_wireless_links_bulk_partial_update) | **PATCH** /wireless/wireless-links/ | 
[**wireless_wireless_links_bulk_update**](WirelessApi.md#wireless_wireless_links_bulk_update) | **PUT** /wireless/wireless-links/ | 
[**wireless_wireless_links_create**](WirelessApi.md#wireless_wireless_links_create) | **POST** /wireless/wireless-links/ | 
[**wireless_wireless_links_delete**](WirelessApi.md#wireless_wireless_links_delete) | **DELETE** /wireless/wireless-links/{id}/ | 
[**wireless_wireless_links_list**](WirelessApi.md#wireless_wireless_links_list) | **GET** /wireless/wireless-links/ | 
[**wireless_wireless_links_partial_update**](WirelessApi.md#wireless_wireless_links_partial_update) | **PATCH** /wireless/wireless-links/{id}/ | 
[**wireless_wireless_links_read**](WirelessApi.md#wireless_wireless_links_read) | **GET** /wireless/wireless-links/{id}/ | 
[**wireless_wireless_links_update**](WirelessApi.md#wireless_wireless_links_update) | **PUT** /wireless/wireless-links/{id}/ | 



## wireless_wireless_lan_groups_bulk_delete

> wireless_wireless_lan_groups_bulk_delete()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lan_groups_bulk_partial_update

> crate::models::WirelessLanGroup wireless_wireless_lan_groups_bulk_partial_update(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableWirelessLanGroup**](WritableWirelessLanGroup.md) |  | [required] |

### Return type

[**crate::models::WirelessLanGroup**](WirelessLANGroup.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lan_groups_bulk_update

> crate::models::WirelessLanGroup wireless_wireless_lan_groups_bulk_update(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableWirelessLanGroup**](WritableWirelessLanGroup.md) |  | [required] |

### Return type

[**crate::models::WirelessLanGroup**](WirelessLANGroup.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lan_groups_create

> crate::models::WirelessLanGroup wireless_wireless_lan_groups_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableWirelessLanGroup**](WritableWirelessLanGroup.md) |  | [required] |

### Return type

[**crate::models::WirelessLanGroup**](WirelessLANGroup.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lan_groups_delete

> wireless_wireless_lan_groups_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Wireless LAN Group. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lan_groups_list

> crate::models::WirelessWirelessLanGroupsList200Response wireless_wireless_lan_groups_list(id, name, slug, description, created, last_updated, q, tag, parent_id, parent, id__n, id__lte, id__lt, id__gte, id__gt, name__n, name__ic, name__nic, name__iew, name__niew, name__isw, name__nisw, name__ie, name__nie, slug__n, slug__ic, slug__nic, slug__iew, slug__niew, slug__isw, slug__nisw, slug__ie, slug__nie, description__n, description__ic, description__nic, description__iew, description__niew, description__isw, description__nisw, description__ie, description__nie, created__n, created__lte, created__lt, created__gte, created__gt, last_updated__n, last_updated__lte, last_updated__lt, last_updated__gte, last_updated__gt, tag__n, parent_id__n, parent__n, ordering, limit, offset)


Overrides ListModelMixin to allow processing ExportTemplates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**slug** | Option<**String**> |  |  |
**description** | Option<**String**> |  |  |
**created** | Option<**String**> |  |  |
**last_updated** | Option<**String**> |  |  |
**q** | Option<**String**> |  |  |
**tag** | Option<**String**> |  |  |
**parent_id** | Option<**String**> |  |  |
**parent** | Option<**String**> |  |  |
**id__n** | Option<**String**> |  |  |
**id__lte** | Option<**String**> |  |  |
**id__lt** | Option<**String**> |  |  |
**id__gte** | Option<**String**> |  |  |
**id__gt** | Option<**String**> |  |  |
**name__n** | Option<**String**> |  |  |
**name__ic** | Option<**String**> |  |  |
**name__nic** | Option<**String**> |  |  |
**name__iew** | Option<**String**> |  |  |
**name__niew** | Option<**String**> |  |  |
**name__isw** | Option<**String**> |  |  |
**name__nisw** | Option<**String**> |  |  |
**name__ie** | Option<**String**> |  |  |
**name__nie** | Option<**String**> |  |  |
**slug__n** | Option<**String**> |  |  |
**slug__ic** | Option<**String**> |  |  |
**slug__nic** | Option<**String**> |  |  |
**slug__iew** | Option<**String**> |  |  |
**slug__niew** | Option<**String**> |  |  |
**slug__isw** | Option<**String**> |  |  |
**slug__nisw** | Option<**String**> |  |  |
**slug__ie** | Option<**String**> |  |  |
**slug__nie** | Option<**String**> |  |  |
**description__n** | Option<**String**> |  |  |
**description__ic** | Option<**String**> |  |  |
**description__nic** | Option<**String**> |  |  |
**description__iew** | Option<**String**> |  |  |
**description__niew** | Option<**String**> |  |  |
**description__isw** | Option<**String**> |  |  |
**description__nisw** | Option<**String**> |  |  |
**description__ie** | Option<**String**> |  |  |
**description__nie** | Option<**String**> |  |  |
**created__n** | Option<**String**> |  |  |
**created__lte** | Option<**String**> |  |  |
**created__lt** | Option<**String**> |  |  |
**created__gte** | Option<**String**> |  |  |
**created__gt** | Option<**String**> |  |  |
**last_updated__n** | Option<**String**> |  |  |
**last_updated__lte** | Option<**String**> |  |  |
**last_updated__lt** | Option<**String**> |  |  |
**last_updated__gte** | Option<**String**> |  |  |
**last_updated__gt** | Option<**String**> |  |  |
**tag__n** | Option<**String**> |  |  |
**parent_id__n** | Option<**String**> |  |  |
**parent__n** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::WirelessWirelessLanGroupsList200Response**](wireless_wireless_lan_groups_list_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lan_groups_partial_update

> crate::models::WirelessLanGroup wireless_wireless_lan_groups_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Wireless LAN Group. | [required] |
**data** | [**WritableWirelessLanGroup**](WritableWirelessLanGroup.md) |  | [required] |

### Return type

[**crate::models::WirelessLanGroup**](WirelessLANGroup.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lan_groups_read

> crate::models::WirelessLanGroup wireless_wireless_lan_groups_read(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Wireless LAN Group. | [required] |

### Return type

[**crate::models::WirelessLanGroup**](WirelessLANGroup.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lan_groups_update

> crate::models::WirelessLanGroup wireless_wireless_lan_groups_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Wireless LAN Group. | [required] |
**data** | [**WritableWirelessLanGroup**](WritableWirelessLanGroup.md) |  | [required] |

### Return type

[**crate::models::WirelessLanGroup**](WirelessLANGroup.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lans_bulk_delete

> wireless_wireless_lans_bulk_delete()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lans_bulk_partial_update

> crate::models::WirelessLan wireless_wireless_lans_bulk_partial_update(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableWirelessLan**](WritableWirelessLan.md) |  | [required] |

### Return type

[**crate::models::WirelessLan**](WirelessLAN.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lans_bulk_update

> crate::models::WirelessLan wireless_wireless_lans_bulk_update(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableWirelessLan**](WritableWirelessLan.md) |  | [required] |

### Return type

[**crate::models::WirelessLan**](WirelessLAN.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lans_create

> crate::models::WirelessLan wireless_wireless_lans_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableWirelessLan**](WritableWirelessLan.md) |  | [required] |

### Return type

[**crate::models::WirelessLan**](WirelessLAN.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lans_delete

> wireless_wireless_lans_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Wireless LAN. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lans_list

> crate::models::WirelessWirelessLansList200Response wireless_wireless_lans_list(id, ssid, auth_psk, description, created, last_updated, q, tag, tenant_group_id, tenant_group, tenant_id, tenant, group_id, group, vlan_id, auth_type, auth_cipher, id__n, id__lte, id__lt, id__gte, id__gt, ssid__n, ssid__ic, ssid__nic, ssid__iew, ssid__niew, ssid__isw, ssid__nisw, ssid__ie, ssid__nie, auth_psk__n, auth_psk__ic, auth_psk__nic, auth_psk__iew, auth_psk__niew, auth_psk__isw, auth_psk__nisw, auth_psk__ie, auth_psk__nie, description__n, description__ic, description__nic, description__iew, description__niew, description__isw, description__nisw, description__ie, description__nie, created__n, created__lte, created__lt, created__gte, created__gt, last_updated__n, last_updated__lte, last_updated__lt, last_updated__gte, last_updated__gt, tag__n, tenant_group_id__n, tenant_group__n, tenant_id__n, tenant__n, group_id__n, group__n, vlan_id__n, auth_type__n, auth_cipher__n, ordering, limit, offset)


Overrides ListModelMixin to allow processing ExportTemplates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> |  |  |
**ssid** | Option<**String**> |  |  |
**auth_psk** | Option<**String**> |  |  |
**description** | Option<**String**> |  |  |
**created** | Option<**String**> |  |  |
**last_updated** | Option<**String**> |  |  |
**q** | Option<**String**> |  |  |
**tag** | Option<**String**> |  |  |
**tenant_group_id** | Option<**String**> |  |  |
**tenant_group** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**tenant** | Option<**String**> |  |  |
**group_id** | Option<**String**> |  |  |
**group** | Option<**String**> |  |  |
**vlan_id** | Option<**String**> |  |  |
**auth_type** | Option<**String**> |  |  |
**auth_cipher** | Option<**String**> |  |  |
**id__n** | Option<**String**> |  |  |
**id__lte** | Option<**String**> |  |  |
**id__lt** | Option<**String**> |  |  |
**id__gte** | Option<**String**> |  |  |
**id__gt** | Option<**String**> |  |  |
**ssid__n** | Option<**String**> |  |  |
**ssid__ic** | Option<**String**> |  |  |
**ssid__nic** | Option<**String**> |  |  |
**ssid__iew** | Option<**String**> |  |  |
**ssid__niew** | Option<**String**> |  |  |
**ssid__isw** | Option<**String**> |  |  |
**ssid__nisw** | Option<**String**> |  |  |
**ssid__ie** | Option<**String**> |  |  |
**ssid__nie** | Option<**String**> |  |  |
**auth_psk__n** | Option<**String**> |  |  |
**auth_psk__ic** | Option<**String**> |  |  |
**auth_psk__nic** | Option<**String**> |  |  |
**auth_psk__iew** | Option<**String**> |  |  |
**auth_psk__niew** | Option<**String**> |  |  |
**auth_psk__isw** | Option<**String**> |  |  |
**auth_psk__nisw** | Option<**String**> |  |  |
**auth_psk__ie** | Option<**String**> |  |  |
**auth_psk__nie** | Option<**String**> |  |  |
**description__n** | Option<**String**> |  |  |
**description__ic** | Option<**String**> |  |  |
**description__nic** | Option<**String**> |  |  |
**description__iew** | Option<**String**> |  |  |
**description__niew** | Option<**String**> |  |  |
**description__isw** | Option<**String**> |  |  |
**description__nisw** | Option<**String**> |  |  |
**description__ie** | Option<**String**> |  |  |
**description__nie** | Option<**String**> |  |  |
**created__n** | Option<**String**> |  |  |
**created__lte** | Option<**String**> |  |  |
**created__lt** | Option<**String**> |  |  |
**created__gte** | Option<**String**> |  |  |
**created__gt** | Option<**String**> |  |  |
**last_updated__n** | Option<**String**> |  |  |
**last_updated__lte** | Option<**String**> |  |  |
**last_updated__lt** | Option<**String**> |  |  |
**last_updated__gte** | Option<**String**> |  |  |
**last_updated__gt** | Option<**String**> |  |  |
**tag__n** | Option<**String**> |  |  |
**tenant_group_id__n** | Option<**String**> |  |  |
**tenant_group__n** | Option<**String**> |  |  |
**tenant_id__n** | Option<**String**> |  |  |
**tenant__n** | Option<**String**> |  |  |
**group_id__n** | Option<**String**> |  |  |
**group__n** | Option<**String**> |  |  |
**vlan_id__n** | Option<**String**> |  |  |
**auth_type__n** | Option<**String**> |  |  |
**auth_cipher__n** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::WirelessWirelessLansList200Response**](wireless_wireless_lans_list_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lans_partial_update

> crate::models::WirelessLan wireless_wireless_lans_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Wireless LAN. | [required] |
**data** | [**WritableWirelessLan**](WritableWirelessLan.md) |  | [required] |

### Return type

[**crate::models::WirelessLan**](WirelessLAN.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lans_read

> crate::models::WirelessLan wireless_wireless_lans_read(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Wireless LAN. | [required] |

### Return type

[**crate::models::WirelessLan**](WirelessLAN.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_lans_update

> crate::models::WirelessLan wireless_wireless_lans_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this Wireless LAN. | [required] |
**data** | [**WritableWirelessLan**](WritableWirelessLan.md) |  | [required] |

### Return type

[**crate::models::WirelessLan**](WirelessLAN.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_links_bulk_delete

> wireless_wireless_links_bulk_delete()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_links_bulk_partial_update

> crate::models::WirelessLink wireless_wireless_links_bulk_partial_update(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableWirelessLink**](WritableWirelessLink.md) |  | [required] |

### Return type

[**crate::models::WirelessLink**](WirelessLink.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_links_bulk_update

> crate::models::WirelessLink wireless_wireless_links_bulk_update(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableWirelessLink**](WritableWirelessLink.md) |  | [required] |

### Return type

[**crate::models::WirelessLink**](WirelessLink.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_links_create

> crate::models::WirelessLink wireless_wireless_links_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableWirelessLink**](WritableWirelessLink.md) |  | [required] |

### Return type

[**crate::models::WirelessLink**](WirelessLink.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_links_delete

> wireless_wireless_links_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this wireless link. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_links_list

> crate::models::WirelessWirelessLinksList200Response wireless_wireless_links_list(id, ssid, auth_psk, description, created, last_updated, q, tag, tenant_group_id, tenant_group, tenant_id, tenant, interface_a_id, interface_b_id, status, auth_type, auth_cipher, id__n, id__lte, id__lt, id__gte, id__gt, ssid__n, ssid__ic, ssid__nic, ssid__iew, ssid__niew, ssid__isw, ssid__nisw, ssid__ie, ssid__nie, auth_psk__n, auth_psk__ic, auth_psk__nic, auth_psk__iew, auth_psk__niew, auth_psk__isw, auth_psk__nisw, auth_psk__ie, auth_psk__nie, description__n, description__ic, description__nic, description__iew, description__niew, description__isw, description__nisw, description__ie, description__nie, created__n, created__lte, created__lt, created__gte, created__gt, last_updated__n, last_updated__lte, last_updated__lt, last_updated__gte, last_updated__gt, tag__n, tenant_group_id__n, tenant_group__n, tenant_id__n, tenant__n, interface_a_id__n, interface_a_id__lte, interface_a_id__lt, interface_a_id__gte, interface_a_id__gt, interface_b_id__n, interface_b_id__lte, interface_b_id__lt, interface_b_id__gte, interface_b_id__gt, status__n, auth_type__n, auth_cipher__n, ordering, limit, offset)


Overrides ListModelMixin to allow processing ExportTemplates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> |  |  |
**ssid** | Option<**String**> |  |  |
**auth_psk** | Option<**String**> |  |  |
**description** | Option<**String**> |  |  |
**created** | Option<**String**> |  |  |
**last_updated** | Option<**String**> |  |  |
**q** | Option<**String**> |  |  |
**tag** | Option<**String**> |  |  |
**tenant_group_id** | Option<**String**> |  |  |
**tenant_group** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**tenant** | Option<**String**> |  |  |
**interface_a_id** | Option<**String**> |  |  |
**interface_b_id** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |
**auth_type** | Option<**String**> |  |  |
**auth_cipher** | Option<**String**> |  |  |
**id__n** | Option<**String**> |  |  |
**id__lte** | Option<**String**> |  |  |
**id__lt** | Option<**String**> |  |  |
**id__gte** | Option<**String**> |  |  |
**id__gt** | Option<**String**> |  |  |
**ssid__n** | Option<**String**> |  |  |
**ssid__ic** | Option<**String**> |  |  |
**ssid__nic** | Option<**String**> |  |  |
**ssid__iew** | Option<**String**> |  |  |
**ssid__niew** | Option<**String**> |  |  |
**ssid__isw** | Option<**String**> |  |  |
**ssid__nisw** | Option<**String**> |  |  |
**ssid__ie** | Option<**String**> |  |  |
**ssid__nie** | Option<**String**> |  |  |
**auth_psk__n** | Option<**String**> |  |  |
**auth_psk__ic** | Option<**String**> |  |  |
**auth_psk__nic** | Option<**String**> |  |  |
**auth_psk__iew** | Option<**String**> |  |  |
**auth_psk__niew** | Option<**String**> |  |  |
**auth_psk__isw** | Option<**String**> |  |  |
**auth_psk__nisw** | Option<**String**> |  |  |
**auth_psk__ie** | Option<**String**> |  |  |
**auth_psk__nie** | Option<**String**> |  |  |
**description__n** | Option<**String**> |  |  |
**description__ic** | Option<**String**> |  |  |
**description__nic** | Option<**String**> |  |  |
**description__iew** | Option<**String**> |  |  |
**description__niew** | Option<**String**> |  |  |
**description__isw** | Option<**String**> |  |  |
**description__nisw** | Option<**String**> |  |  |
**description__ie** | Option<**String**> |  |  |
**description__nie** | Option<**String**> |  |  |
**created__n** | Option<**String**> |  |  |
**created__lte** | Option<**String**> |  |  |
**created__lt** | Option<**String**> |  |  |
**created__gte** | Option<**String**> |  |  |
**created__gt** | Option<**String**> |  |  |
**last_updated__n** | Option<**String**> |  |  |
**last_updated__lte** | Option<**String**> |  |  |
**last_updated__lt** | Option<**String**> |  |  |
**last_updated__gte** | Option<**String**> |  |  |
**last_updated__gt** | Option<**String**> |  |  |
**tag__n** | Option<**String**> |  |  |
**tenant_group_id__n** | Option<**String**> |  |  |
**tenant_group__n** | Option<**String**> |  |  |
**tenant_id__n** | Option<**String**> |  |  |
**tenant__n** | Option<**String**> |  |  |
**interface_a_id__n** | Option<**String**> |  |  |
**interface_a_id__lte** | Option<**String**> |  |  |
**interface_a_id__lt** | Option<**String**> |  |  |
**interface_a_id__gte** | Option<**String**> |  |  |
**interface_a_id__gt** | Option<**String**> |  |  |
**interface_b_id__n** | Option<**String**> |  |  |
**interface_b_id__lte** | Option<**String**> |  |  |
**interface_b_id__lt** | Option<**String**> |  |  |
**interface_b_id__gte** | Option<**String**> |  |  |
**interface_b_id__gt** | Option<**String**> |  |  |
**status__n** | Option<**String**> |  |  |
**auth_type__n** | Option<**String**> |  |  |
**auth_cipher__n** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::WirelessWirelessLinksList200Response**](wireless_wireless_links_list_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_links_partial_update

> crate::models::WirelessLink wireless_wireless_links_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this wireless link. | [required] |
**data** | [**WritableWirelessLink**](WritableWirelessLink.md) |  | [required] |

### Return type

[**crate::models::WirelessLink**](WirelessLink.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_links_read

> crate::models::WirelessLink wireless_wireless_links_read(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this wireless link. | [required] |

### Return type

[**crate::models::WirelessLink**](WirelessLink.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wireless_wireless_links_update

> crate::models::WirelessLink wireless_wireless_links_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this wireless link. | [required] |
**data** | [**WritableWirelessLink**](WritableWirelessLink.md) |  | [required] |

### Return type

[**crate::models::WirelessLink**](WirelessLink.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

