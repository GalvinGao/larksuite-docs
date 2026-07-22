---
document_id: '7163183770995359749'
directory_id: '7137232833810382853'
title: CameraErrno
full_path: /uYjL24iN/ukDOukDOukDO/camera/cameraerrno
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Camera
- CameraErrno
document_type: GuideDocumentType
updated_at: 2022-12-23T07:02:06Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukDOukDOukDO/camera/cameraerrno
---

# CameraErrno

Errno规则可参见 [Errno 错误码](/document/uYjL24iN/uAjMuAjMuAjM/errno)

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V5.21.0+</md-version> | <md-version>V5.21.0+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14" disable=true>预览</md-preview-app> |


### CameraErrno错误码字典
:::html
</md-alert>
:::
| **errno** | **errString** | **含义**                 |**归属**| **最低版本**
| ------ | ------ | ---------------------- | ---------- |----------|
|9000099| Failed to get component id | createCameraContext指定的id，和dom指定的id不对应时，api调用抛错 | 组件 | 5.21.0
|9005400| internalError | 内部错误 | Camera| 5.21.0
|9005401| Camera init error | 相机初始化错误 | Camera| 5.21.0
|9005402| Not allow to invoke @{apiName} in 'scanCode' mode | scanCode 模式下@{apiName}不可用 | Camera| 5.27.0
|9005411| StartRecord record already started | 已在录制状态 | Camera| 5.21.0
|9005412| StartRecord camera error | 开启录制时camera内部错误 | Camera| 5.21.0
|9005421| StopRecord record not started | 未开启录制 | Camera| 5.21.0
|9005422| StopRecord camera error | 停止录制时camera内部错误 | Camera| 5.21.0
|9005423| StopRecord save file error | 录制保存文件错误 | Camera| 5.21.0
|9005431| SetZoom fail: time out | setZoom超时 | Camera| 5.21.0
|9005441| TakePhoto camera error | 拍照时camera内部错误 | Camera| 5.21.0
|9005442| TakePhoto save file error | 拍照保存文件错误 | Camera| 5.21.0
|9005443| TakePhoto last capture not finish | 上次拍照未结束 | Camera| 5.21.0
|9005501| Camera init error | Camera 初始化错误 | Camera| 5.21.0
|9005502| Cannot add more than one camera component | 同一页面添加多于一个camera组件 | Camera| 5.21.0
|9005503| Failed to get component id | 未给组件设置id | Camera| 5.21.0


