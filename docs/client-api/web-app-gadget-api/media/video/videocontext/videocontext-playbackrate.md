---
document_id: '7235839783196491782'
directory_id: '7073451436033916933'
title: VideoContext.playbackRate
full_path: /uYjL24iN/uITMx4iMxEjLyETM/videocontext/playbackrate
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Video
- VideoContext
- VideoContext.playbackRate
document_type: GuideDocumentType
updated_at: 2023-05-22T07:36:15Z
source_url: https://open.larksuite.com/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/playbackrate
---

# VideoContext.playbackRate(number rate)

设置视频倍速播放

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
      <md-td><md-version>V6.1.0+</md-version></md-td>
      <md-td><md-version>V6.1.0+</md-version></md-td>
      <md-td><md-version>V6.1.0+</md-version></md-td>
      <md-td><md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/component/pages/video/video" fontSize="14">预览</md-preview-app></md-td>
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
      <md-td>rate</md-td>
      <md-td>number</md-td>
      <md-td>是</md-td>
      <md-td></md-td>
      <md-td>
        倍速值，范围[0.5, 2]。
若传入值小于0.5，则为0.5；若传入值大于2，则为2；向下取整保留1位小数

      </md-td>
    </md-tr>

  </md-tbody>
</md-table>
:::


## 输出
无


