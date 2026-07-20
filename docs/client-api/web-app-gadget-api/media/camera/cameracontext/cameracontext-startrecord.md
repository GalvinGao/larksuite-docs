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
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

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
      <md-td>timeout</md-td>
      <md-td>number</md-td>
      <md-td>否</md-td>
      <md-td>30</md-td>
      <md-td>
        录制时长上限，最长 300s
      </md-td>
    </md-tr>

  </md-tbody>
</md-table>
:::


## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，`success`返回对象的扩展属性：

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 30%;">名称</md-th>
      <md-th style="width: 18%;">数据类型</md-th>
      <md-th>描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>tempThumbPath</md-td>
      <md-td>string</md-td>
      <md-td>
        封面图片文件的临时路径 (本地路径)
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>tempVideoPath</md-td>
      <md-td>string</md-td>
      <md-td>
        视频的文件的临时路径 (本地路径)
      </md-td>
    </md-tr>

  </md-tbody>
</md-table>
:::



