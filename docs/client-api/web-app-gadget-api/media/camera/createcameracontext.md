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

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">应用能力</md-th>
      <md-th style="width: 20%;">Android</md-th>
      <md-th style="width: 20%;">iOS</md-th>
      <md-th style="width: 20%;">PC</md-th>
      <md-th style="width: 20%;">预览效果</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>小程序</md-td>
      <md-td><md-version>V5.21.0+</md-version></md-td>
      <md-td><md-version>V5.21.0+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td><md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/component/pages/camera/camera" fontSize="14">预览</md-preview-app></md-td>
    </md-tr>
    <md-tr>
      <md-td>网页应用</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14" disable=true>预览</md-preview-app></md-td>
    </md-tr>
  </md-tbody>
</md-table>
:::

## 输入

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">名称</md-th>
      <md-th style="width: 18%;">数据类型</md-th>
      <md-th style="width: 10%;">必填</md-th>
      <md-th style="width: 10%;">默认值</md-th>
      <md-th>描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>id</md-td>
      <md-td>string</md-td>
      <md-td>是</md-td>
      <md-td></md-td>
      <md-td>
        [`camera`](/document/uYjL24iN/uYTNuYTNuYTN/camera) 组件的 id
      </md-td>
    </md-tr>

  </md-tbody>
</md-table>
:::


## 输出
返回值：`CameraContext`，该对象的方法列表参见下表：

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 50%;">方法</md-th>
      <md-th style="width: 50%;">介绍</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
    <md-td>[CameraContext.setZoom](/document/uYjL24iN/ukDOukDOukDO/camera/cameracontext/setzoom)</md-td>
    <md-td>设置缩放级别</md-td>
  </md-tr>
<md-tr>
    <md-td>[CameraContext.takePhoto](/document/uYjL24iN/ukDOukDOukDO/camera/cameracontext/takephoto)</md-td>
    <md-td>拍摄照片</md-td>
  </md-tr>
<md-tr>
    <md-td>[CameraContext.startRecord](/document/uYjL24iN/ukDOukDOukDO/camera/cameracontext/startrecord)</md-td>
    <md-td>开始录像</md-td>
  </md-tr>
<md-tr>
    <md-td>[CameraContext.stopRecord](/document/uYjL24iN/ukDOukDOukDO/camera/cameracontext/stoprecord)</md-td>
    <md-td>结束录像</md-td>
  </md-tr>

</md-tbody>
</md-table>
:::


