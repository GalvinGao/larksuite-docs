---
document_id: '7073692582769164294'
directory_id: '7073451436033835013'
title: IntersectionObserver.relativeToViewport
full_path: /uYjL24iN/uUjN24SN2YjL1YjN/intersectionobserver/relativetoviewport
breadcrumb:
- Client API
- Web app/Gadget API
- TTML
- IntersectionObserver
- IntersectionObserver.relativeToViewport
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:54Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUjN24SN2YjL1YjN/intersectionobserver/relativetoviewport
---

# IntersectionObserver.relativeToViewport(object margins)

指定页面显示区域作为参照区域之一


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
<md-td><md-version>V2.3.0+</md-version></md-td>
      <md-td><md-version>V2.3.0+</md-version></md-td>
      <md-td><md-version>V2.3.0+</md-version></md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="这里一定要修改成具体的路径" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td>**✕**</md-td>
      <md-td>**✕**</md-td>
      <md-td>**✕**</md-td>
      <md-td>/</md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::



## 输入
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 20%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th style="width: 10%;">
                必填
            </md-th>
            <md-th style="width: 10%;">
                默认值
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                margins
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                用来扩展（或收缩）参照节点布局区域的边界
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    left
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>0</md-td>
            <md-td>
                节点布局区域的左边界
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    right
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>0</md-td>
            <md-td>
                节点布局区域的右边界
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    top
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>0</md-td>
            <md-td>
                节点布局区域的上边界
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    bottom
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>0</md-td>
            <md-td>
                节点布局区域的下边界
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 输出
无


## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/createIntersectionObserver/createIntersectionObserver" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
tt.createIntersectionObserver(this, {
    selectAll: true
})
.relativeToViewport({top:10})
```

