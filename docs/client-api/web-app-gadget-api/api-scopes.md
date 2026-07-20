---
document_id: '6965379543683383302'
directory_id: '6907567266540847106'
title: API 权限
full_path: /uYjL24iN/uITMuITMuITM
breadcrumb:
- Client API
- Web app/Gadget API
- API Scopes
document_type: GuideDocumentType
updated_at: 2024-01-31T08:43:24Z
source_url: https://open.larksuite.com/document/uYjL24iN/uITMuITMuITM
---

# API 权限

## 概述
为了对用户信息进行严格保护，客户端部分接口需要经过用户授权同意才能调用，如果用户拒绝授权，**请开发者兼容用户拒绝授权的场景**。本指南详细介绍如何处理授权请求并正确调用这些接口。

这些接口按使用范围分成多个 scope ，用户选择对 scope 来进行授权，当授权给一个 scope 之后，其对应的所有接口都可以直接使用。

## 直接调用接口的默认行为
- 如果未授权过此权限，会弹窗询问用户，用户同意授权后会继续执行API后续逻辑；
- 如果用户已授权，会直接继续执行；
- 如果用户已拒绝授权，会进入接口 fail 回调，并且弹窗询问用户，引导用户前往权限设置界面。

![图片名称](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/08e2f27cad9047a8bf988e84cf4ab4f1_UIb05Z2xz8.png?height=1334&lazyload=true&width=1580)

## 权限（Scope）列表

| scope         | API           | 权限        | 描述         | 需要授权给客户端        |
| --------- | ------ | -----------   | ----------- | --------- |
|`scope.userInfo` | [getUserInfo](/document/uYjL24iN/ucjMx4yNyEjL3ITM) | 用户信息 | 获得你的基本信息（昵称、头像、性别及地区等）。 | ✓(**Lark 5.2及以上版本生效**) |
|`scope.userLocation` | [openLocation](/document/uYjL24iN/uQTOz4CN5MjL0kzM) | 地理位置 | 获取你的地理位置信息 | ✓ |
|`scope.record` | [getRecorderManager](/document/uYjL24iN/uQDOx4CN4EjL0gTM)| 麦克风 | 访问你的麦克风功能 | ✓ |
|`scope.writePhotosAlbum` | [saveImageToPhotosAlbum](/document/uYjL24iN/uUTMx4SNxEjL1ETM) | 保存到相册 | 保存图片或视频到你的相册 | ✓ |
|`scope.clipboard` | [getClipboardData](/document/uYjL24iN/uczNx4yN3EjL3cTM) | 剪切板 | 访问剪切板 | ✓ |
|`scope.bluetooth` | [openBluetoothAdapter](/document/uYjL24iN/ugzNxYjL4cTM24CO3EjN) | 蓝牙 | 访问蓝牙 | ✓ |


## 授权有效期
一旦用户明确同意或拒绝过授权，其授权信息会被记录。自3.5及其以后版本，授权信息会同步到服务器，更换设备或者卸载重装后不再需要用户重新授权。 

## 系统接口权限
操作系统对于一些系统接口进行了权限控制，当调用这些接口时，需要用户对Lark进行授权。
各个操作系统对于这些权限的处理方式大体一致，但也会有所不同，开发者应有所了解。

::: note
调用一个同时需要小程序权限和系统权限的API时，可能会先出现小程序API授权弹窗，再出现系统接口授权弹窗。
:::

