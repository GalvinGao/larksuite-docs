---
document_id: '7163183770995408901'
directory_id: '7137946436762976261'
title: CameraContext.startRecord
full_path: /uYjL24iN/ukDOukDOukDO/camera/cameracontext/startrecord
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Camera
- CameraContext
- CameraContext.startRecord
document_type: GuideDocumentType
updated_at: 2022-11-11T03:58:31Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukDOukDOukDO/camera/cameracontext/startrecord
---

# CameraContext.startRecord

开始录像

:::html
<md-alert type="tip">

- 调用前需要用户授权 `scope.record`。了解如何授权，可查看[API 权限](/document/uYjL24iN/uITMuITMuITM)。

</md-alert>
:::

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V5.21.0+</md-version> | <md-version>V5.21.0+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/component/pages/camera/camera" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14" disable=true>预览</md-preview-app> |


## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| timeout | number | 否 | 30 | 录制时长上限，最长 300s |



## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| tempThumbPath | string | 封面图片文件的临时路径 (本地路径) |
| tempVideoPath | string | 视频的文件的临时路径 (本地路径) |




