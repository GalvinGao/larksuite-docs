---
document_id: '7163183770995392517'
directory_id: '7137946436762976261'
title: CameraContext.takePhoto
full_path: /uYjL24iN/ukDOukDOukDO/camera/cameracontext/takephoto
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Camera
- CameraContext
- CameraContext.takePhoto
document_type: GuideDocumentType
updated_at: 2022-11-11T03:58:28Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukDOukDOukDO/camera/cameracontext/takephoto
---

# CameraContext.takePhoto

拍摄照片

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V5.21.0+</md-version> | <md-version>V5.21.0+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/component/pages/camera/camera" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14" disable=true>预览</md-preview-app> |


## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| quality | Enum&lt;string&gt; | 否 | medium | 成像质量 |
| selfieMirror | boolean | 否 | true | 是否开启镜像 |


## 参数说明

### quality 合法值
|值|说明|
|--|----|
|low|低|
|medium|中|
|high|高|

## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| tempImagePath | string | 照片文件的临时路径 (本地路径)，安卓是 jpg 图片格式，ios是jpeg |




