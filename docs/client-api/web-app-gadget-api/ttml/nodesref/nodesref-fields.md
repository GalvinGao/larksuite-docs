---
document_id: '7073691561008578565'
directory_id: '7073451436034146309'
title: NodesRef.fields
full_path: /uYjL24iN/uUjN24SN2YjL1YjN/nodesref/fields
breadcrumb:
- Client API
- Web app/Gadget API
- TTML
- NodesRef
- NodesRef.fields
document_type: GuideDocumentType
updated_at: 2023-02-10T09:17:21Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUjN24SN2YjL1YjN/nodesref/fields
---

# NodesRef.fields(Object object, function callback)

获取节点的相关信息。需要获取的字段在`fields`中指定。返回值是`nodesRef`对应的 `selectorQuery`。


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
      <md-td><md-version>V2.2.0+</md-version></md-td>
      <md-td><md-version>V2.2.0+</md-version></md-td>
      <md-td><md-version>V2.2.0+</md-version></md-td>
     <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/createSelectorQuery/createSelectorQuery" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
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
                fields
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                fields 配置
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
                    id
                </md-text>
            </md-td>
            <md-td>
                boolean
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                是否返回节点 id
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
                    dataset
                </md-text>
            </md-td>
            <md-td>
                boolean
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                是否返回节点 dataset
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
                    rect
                </md-text>
            </md-td>
            <md-td>
                boolean
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                是否返回节点布局位置（left right top bottom）
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
                    size
                </md-text>
            </md-td>
            <md-td>
                boolean
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                是否返回节点尺寸（width height）
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                callback
            </md-td>
            <md-td>
                function
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                callback function
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::



## 输出
回调函数返回对象的属性：
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
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                id
            </md-td>
            <md-td>
                string
            </md-td>

            <md-td>
                节点 id
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                dataset
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                节点的 dataset
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                left
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                节点的左边界坐标
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                right
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                节点的右边界坐标
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                top
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                节点的上边界坐标
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                bottom
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                节点的下边界坐标
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                width
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                节点的宽度
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                height
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                节点的高度
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/createSelectorQuery/createSelectorQuery" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
Page({
  getFields: function(){
    tt.createSelectorQuery().select('#the-id').fields({
      id: true,
      dataset: true,
      size: true,
    }, (res) => {
      res.id         //节点的ID
      res.dataset    // 节点的dataset
      res.width      // 节点的宽度
      res.height     // 节点的高度
    }).exec()
  }
});
```
