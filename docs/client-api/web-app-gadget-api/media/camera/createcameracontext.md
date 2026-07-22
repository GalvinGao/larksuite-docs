---
document_id: '7163183770995343365'
directory_id: '7137232833810382853'
title: createCameraContext
full_path: /uYjL24iN/ukDOukDOukDO/camera/createcameracontext
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Camera
- createCameraContext
document_type: GuideDocumentType
updated_at: 2022-11-11T03:58:19Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukDOukDOukDO/camera/createcameracontext
---

# createCameraContext

创建 [`camera`](/document/uYjL24iN/uYTNuYTNuYTN/camera) 上下文 `CameraContext` 对象。

`CameraContext` 与页面内唯一的 [`camera`](/document/uYjL24iN/uYTNuYTNuYTN/camera) 组件绑定，操作对应的 [`camera`](/document/uYjL24iN/uYTNuYTNuYTN/camera) 组件。


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V5.21.0+</md-version> | <md-version>V5.21.0+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/component/pages/camera/camera" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14" disable=true>预览</md-preview-app> |


## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| id | string | 是 |  | [`camera`](/document/uYjL24iN/uYTNuYTNuYTN/camera) 组件的 id |



## 输出
返回值：`CameraContext`，该对象的方法列表参见下表：

| 方法 | 介绍 |
| --- | --- |
| [CameraContext.setZoom](/document/uYjL24iN/ukDOukDOukDO/camera/cameracontext/setzoom) | 设置缩放级别 |
| [CameraContext.takePhoto](/document/uYjL24iN/ukDOukDOukDO/camera/cameracontext/takephoto) | 拍摄照片 |
| [CameraContext.startRecord](/document/uYjL24iN/ukDOukDOukDO/camera/cameracontext/startrecord) | 开始录像 |
| [CameraContext.stopRecord](/document/uYjL24iN/ukDOukDOukDO/camera/cameracontext/stoprecord) | 结束录像 |



