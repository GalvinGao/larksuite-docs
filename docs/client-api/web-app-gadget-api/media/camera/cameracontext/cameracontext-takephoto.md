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
      <md-td>quality</md-td>
      <md-td>Enum&lt;string&gt;</md-td>
      <md-td>否</md-td>
      <md-td>medium</md-td>
      <md-td>
        成像质量
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>selfieMirror</md-td>
      <md-td>boolean</md-td>
      <md-td>否</md-td>
      <md-td>true</md-td>
      <md-td>
        是否开启镜像
      </md-td>
    </md-tr>

  </md-tbody>
</md-table>
:::

## 参数说明

### quality 合法值
|值|说明|
|--|----|
|low|低|
|medium|中|
|high|高|

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
      <md-td>tempImagePath</md-td>
      <md-td>string</md-td>
      <md-td>
        照片文件的临时路径 (本地路径)，安卓是 jpg 图片格式，ios是jpeg
      </md-td>
    </md-tr>

  </md-tbody>
</md-table>
:::



