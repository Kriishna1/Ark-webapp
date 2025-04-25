# \ArkServiceApi

All URIs are relative to _http://localhost_

| Method                                                                                                          | HTTP request                             | Description |
| --------------------------------------------------------------------------------------------------------------- | ---------------------------------------- | ----------- |
| [**ark_service_delete_nostr_recipient**](ArkServiceApi.md#ark_service_delete_nostr_recipient)                   | **POST** /v1/vtxo/nostr/delete           |             |
| [**ark_service_get_boarding_address**](ArkServiceApi.md#ark_service_get_boarding_address)                       | **POST** /v1/boarding                    |             |
| [**ark_service_get_event_stream**](ArkServiceApi.md#ark_service_get_event_stream)                               | **GET** /v1/events                       |             |
| [**ark_service_get_info**](ArkServiceApi.md#ark_service_get_info)                                               | **GET** /v1/info                         |             |
| [**ark_service_get_round**](ArkServiceApi.md#ark_service_get_round)                                             | **GET** /v1/round/{txid}                 |             |
| [**ark_service_get_round_by_id**](ArkServiceApi.md#ark_service_get_round_by_id)                                 | **GET** /v1/round/id/{id}                |             |
| [**ark_service_get_transactions_stream**](ArkServiceApi.md#ark_service_get_transactions_stream)                 | **GET** /v1/transactions                 |             |
| [**ark_service_list_vtxos**](ArkServiceApi.md#ark_service_list_vtxos)                                           | **GET** /v1/vtxos/{address}              |             |
| [**ark_service_ping**](ArkServiceApi.md#ark_service_ping)                                                       | **GET** /v1/round/ping/{requestId}       |             |
| [**ark_service_register_inputs_for_next_round**](ArkServiceApi.md#ark_service_register_inputs_for_next_round)   | **POST** /v1/round/registerInputs        |             |
| [**ark_service_register_outputs_for_next_round**](ArkServiceApi.md#ark_service_register_outputs_for_next_round) | **POST** /v1/round/registerOutputs       |             |
| [**ark_service_set_nostr_recipient**](ArkServiceApi.md#ark_service_set_nostr_recipient)                         | **POST** /v1/vtxo/nostr                  |             |
| [**ark_service_submit_redeem_tx**](ArkServiceApi.md#ark_service_submit_redeem_tx)                               | **POST** /v1/redeem-tx                   |             |
| [**ark_service_submit_signed_forfeit_txs**](ArkServiceApi.md#ark_service_submit_signed_forfeit_txs)             | **POST** /v1/round/submitForfeitTxs      |             |
| [**ark_service_submit_tree_nonces**](ArkServiceApi.md#ark_service_submit_tree_nonces)                           | **POST** /v1/round/tree/submitNonces     |             |
| [**ark_service_submit_tree_signatures**](ArkServiceApi.md#ark_service_submit_tree_signatures)                   | **POST** /v1/round/tree/submitSignatures |             |

## ark_service_delete_nostr_recipient

> serde_json::Value ark_service_delete_nostr_recipient(body)

### Parameters

| Name     | Type                                                                  | Description | Required   | Notes |
| -------- | --------------------------------------------------------------------- | ----------- | ---------- | ----- |
| **body** | [**V1DeleteNostrRecipientRequest**](V1DeleteNostrRecipientRequest.md) |             | [required] |       |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ark_service_get_boarding_address

> models::V1GetBoardingAddressResponse ark_service_get_boarding_address(body)

### Parameters

| Name     | Type                                                              | Description | Required   | Notes |
| -------- | ----------------------------------------------------------------- | ----------- | ---------- | ----- |
| **body** | [**V1GetBoardingAddressRequest**](V1GetBoardingAddressRequest.md) |             | [required] |       |

### Return type

[**models::V1GetBoardingAddressResponse**](v1GetBoardingAddressResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ark_service_get_event_stream

> models::StreamResultOfV1GetEventStreamResponse ark_service_get_event_stream()

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::StreamResultOfV1GetEventStreamResponse**](Stream_result_of_v1GetEventStreamResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ark_service_get_info

> models::V1GetInfoResponse ark_service_get_info()

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::V1GetInfoResponse**](v1GetInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ark_service_get_round

> models::V1GetRoundResponse ark_service_get_round(txid)

### Parameters

| Name     | Type       | Description | Required   | Notes |
| -------- | ---------- | ----------- | ---------- | ----- |
| **txid** | **String** |             | [required] |       |

### Return type

[**models::V1GetRoundResponse**](v1GetRoundResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ark_service_get_round_by_id

> models::V1GetRoundByIdResponse ark_service_get_round_by_id(id)

### Parameters

| Name   | Type       | Description | Required   | Notes |
| ------ | ---------- | ----------- | ---------- | ----- |
| **id** | **String** |             | [required] |       |

### Return type

[**models::V1GetRoundByIdResponse**](v1GetRoundByIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ark_service_get_transactions_stream

> models::StreamResultOfV1GetTransactionsStreamResponse ark_service_get_transactions_stream()

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::StreamResultOfV1GetTransactionsStreamResponse**](Stream_result_of_v1GetTransactionsStreamResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ark_service_list_vtxos

> models::V1ListVtxosResponse ark_service_list_vtxos(address)

### Parameters

| Name        | Type       | Description | Required   | Notes |
| ----------- | ---------- | ----------- | ---------- | ----- |
| **address** | **String** |             | [required] |       |

### Return type

[**models::V1ListVtxosResponse**](v1ListVtxosResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ark_service_ping

> serde_json::Value ark_service_ping(request_id)

### Parameters

| Name           | Type       | Description                                 | Required   | Notes |
| -------------- | ---------- | ------------------------------------------- | ---------- | ----- |
| **request_id** | **String** | The id used to register inputs and ouptuts. | [required] |       |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ark_service_register_inputs_for_next_round

> models::V1RegisterInputsForNextRoundResponse ark_service_register_inputs_for_next_round(body)

### Parameters

| Name     | Type                                                                              | Description | Required   | Notes |
| -------- | --------------------------------------------------------------------------------- | ----------- | ---------- | ----- |
| **body** | [**V1RegisterInputsForNextRoundRequest**](V1RegisterInputsForNextRoundRequest.md) |             | [required] |       |

### Return type

[**models::V1RegisterInputsForNextRoundResponse**](v1RegisterInputsForNextRoundResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ark_service_register_outputs_for_next_round

> serde_json::Value ark_service_register_outputs_for_next_round(body)

### Parameters

| Name     | Type                                                                                | Description | Required   | Notes |
| -------- | ----------------------------------------------------------------------------------- | ----------- | ---------- | ----- |
| **body** | [**V1RegisterOutputsForNextRoundRequest**](V1RegisterOutputsForNextRoundRequest.md) |             | [required] |       |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ark_service_set_nostr_recipient

> serde_json::Value ark_service_set_nostr_recipient(body)

### Parameters

| Name     | Type                                                            | Description | Required   | Notes |
| -------- | --------------------------------------------------------------- | ----------- | ---------- | ----- |
| **body** | [**V1SetNostrRecipientRequest**](V1SetNostrRecipientRequest.md) |             | [required] |       |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ark_service_submit_redeem_tx

> models::V1SubmitRedeemTxResponse ark_service_submit_redeem_tx(body)

### Parameters

| Name     | Type                                                      | Description | Required   | Notes |
| -------- | --------------------------------------------------------- | ----------- | ---------- | ----- |
| **body** | [**V1SubmitRedeemTxRequest**](V1SubmitRedeemTxRequest.md) |             | [required] |       |

### Return type

[**models::V1SubmitRedeemTxResponse**](v1SubmitRedeemTxResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ark_service_submit_signed_forfeit_txs

> serde_json::Value ark_service_submit_signed_forfeit_txs(body)

### Parameters

| Name     | Type                                                                      | Description | Required   | Notes |
| -------- | ------------------------------------------------------------------------- | ----------- | ---------- | ----- |
| **body** | [**V1SubmitSignedForfeitTxsRequest**](V1SubmitSignedForfeitTxsRequest.md) |             | [required] |       |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ark_service_submit_tree_nonces

> serde_json::Value ark_service_submit_tree_nonces(body)

### Parameters

| Name     | Type                                                          | Description | Required   | Notes |
| -------- | ------------------------------------------------------------- | ----------- | ---------- | ----- |
| **body** | [**V1SubmitTreeNoncesRequest**](V1SubmitTreeNoncesRequest.md) |             | [required] |       |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ark_service_submit_tree_signatures

> serde_json::Value ark_service_submit_tree_signatures(body)

### Parameters

| Name     | Type                                                                  | Description | Required   | Notes |
| -------- | --------------------------------------------------------------------- | ----------- | ---------- | ----- |
| **body** | [**V1SubmitTreeSignaturesRequest**](V1SubmitTreeSignaturesRequest.md) |             | [required] |       |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
