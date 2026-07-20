---
document_id: '6965379541104574469'
directory_id: '6907567266536308737'
title: 本地数据缓存
full_path: /uYjL24iN/uMTOz4yM5MjLzkzM
breadcrumb:
- Client API
- Web app/Gadget API
- Cache
- Local Data Cache
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:21Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMTOz4yM5MjLzkzM
---

# 本地数据缓存

小程序可以通过下面列出的API读取、写入、删除、清理本地缓存数据，与 [LocalStorage](https://developer.mozilla.org/zh-CN/docs/Web/API/Window/localStorage) 类似。

本地缓存数据以**用户+小程序**维度隔离，同一台设备上，不同用户之间、不同小程序之间，本地缓存数据不可互访。

::: note
单个 key 允许存储的最大数据长度为 **1MB**，所有数据存储上限为 **10MB**，同时也受到用户设备存储空间、缓存清理等机制的限制，可能会导致信息丢失，因此请不要将重要数据存放在本地数据缓存。
:::

## API Checklist

- [getStorage](/document/uYjL24iN/ukDOx4SO4EjL5gTM)
- [getStorageSync](/document/uYjL24iN/uATOx4CM5EjLwkTM)
- [setStorage](/document/uYjL24iN/uETOx4SM5EjLxkTM)
- [setStorageSync](/document/uYjL24iN/uITOx4iM5EjLykTM)
- [removeStorage](/document/uYjL24iN/uMTOx4yM5EjLzkTM)
- [removeStorageSync](/document/uYjL24iN/uQTOx4CN5EjL0kTM)
- [clearStorage](/document/uYjL24iN/uUTOx4SN5EjL1kTM)
- [clearStorageSync](/document/uYjL24iN/uYTOx4iN5EjL2kTM)
- [getStorageInfo](/document/uYjL24iN/ucTOx4yN5EjL3kTM)
- [getStorageInfoSync](/document/uYjL24iN/ugTOx4CO5EjL4kTM)

## 本地缓存数据类型

小程序默认支持以下数据类型，存储和获取时无需类型转换。其它数据类型可根据需要在处理时转换为字符串存储，**强行操作会导致数据丢失**。

- `string`
- `number`
- `boolean`
- `object`
- `object[]`
- `string[]`
- `number[]`
- `boolean[]`
- `undefined`
- `null`
