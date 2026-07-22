---
document_id: '7163183770995376133'
directory_id: '7137946436762976261'
title: CameraContext.setZoom
full_path: /uYjL24iN/ukDOukDOukDO/camera/cameracontext/setzoom
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Camera
- CameraContext
- CameraContext.setZoom
document_type: GuideDocumentType
updated_at: 2022-11-11T03:58:25Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukDOukDOukDO/camera/cameracontext/setzoom
---

# CameraContext.setZoom

设置缩放级别

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V5.21.0+</md-version> | <md-version>V5.21.0+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/component/pages/camera/camera" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14" disable=true>预览</md-preview-app> |


## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| zoom | number | 是 |  | - 缩放级别，范围[1, maxZoom]。zoom 可取小数。maxZoom 可在 bindinitdone 返回值中获取。<br>- 如果超出范围取边界值。 |



## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| zoom | number | 实际设置的缩放级别。由于系统限制，某些机型可能无法设置成指定值，会改用最接近的可设值。 |




