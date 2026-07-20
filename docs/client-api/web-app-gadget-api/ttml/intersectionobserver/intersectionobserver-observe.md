---
document_id: '7073693024735707141'
directory_id: '7073451436033835013'
title: IntersectionObserver.observe
full_path: /uYjL24iN/uUjN24SN2YjL1YjN/intersectionobserver/observe
breadcrumb:
- Client API
- Web app/Gadget API
- TTML
- IntersectionObserver
- IntersectionObserver.observe
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:54Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUjN24SN2YjL1YjN/intersectionobserver/observe
---

# IntersectionObserver.observe(string targetSelector,function callback)

指定目标节点并开始监听相交状态变化情况


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
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/createIntersectionObserver/createIntersectionObserver" fontSize="14">预览</md-preview-app>
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
                targetSelector
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                选择器
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
                回调函数
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 输出
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 30%;">
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
                param
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                回调函数接受的参数
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
                    intersectionRatio
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                相交比例
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
                    intersectionRect
                </md-text>
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                相交区域的边界
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;
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
                左边界
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;
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
                右边界
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;
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
                上边界
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;
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
                下边界
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
                    boundingClientRect
                </md-text>
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                目标边界
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;
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
                左边界
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;
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
                右边界
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;
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
                上边界
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;
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
                下边界
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
                    relativeRect
                </md-text>
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                参照区域的边界
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;
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
                左边界
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;
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
                右边界
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;
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
                上边界
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;
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
                下边界
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
                    time
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                相交检测时的时间戳
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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/createIntersectionObserver/createIntersectionObserver" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
//也可以使用 this.createIntersectionObserver({selectAll}) 来创建
tt.createIntersectionObserver(this, {
    selectAll: true
})
.relativeTo('.container')
.observe('.ball', res => {
    res.intersectionRect  // 相交区域
    res.intersectionRect.left  // 相交区域的左边界坐标
    res.intersectionRect.top  // 相交区域的上边界坐标
    res.intersectionRect.width  // 相交区域的宽度
    res.intersectionRect.height  // 相交区域的高度
});
```

