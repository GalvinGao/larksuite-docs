---
document_id: '6965379541104721925'
directory_id: '6907567266541879298'
title: onWindowResize
full_path: /uYjL24iN/uADO3UjLwgzN14CM4cTN
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Window
- onWindowResize
document_type: GuideDocumentType
updated_at: 2022-03-11T04:14:59Z
source_url: https://open.larksuite.com/document/uYjL24iN/uADO3UjLwgzN14CM4cTN
---

# onWindowResize(function callback)

监听窗口尺寸变化事件

:::html
<md-alert type="tip">
使用同一回调函数多次调用，会注册多次该事件，回调会被执行多次。
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
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td><md-version>V3.13.0+</md-version></md-td>
      <md-td><md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app></md-td> 
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
      <md-th style="width: 20%;">名称</md-th>
      <md-th style="width: 18%;">数据类型</md-th>
       <md-th style="width: 10%;">必填</md-th>
      <md-th style="width: 10%;">默认值</md-th>
      <md-th>描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>

    
   <md-tr>
      <md-td>callback</md-td>
      <md-td>function</md-td>
      <md-td>是</md-td>
      <md-td></md-td>
      <md-td>该事件的回调函数</md-td>

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
                size
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                窗口大小
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
                    windowWidth
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                变化后的窗口宽度，单位px
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
                    windowHeight
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                变化后的窗口高度，单位px
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 示例代码


```js
tt.onWindowResize(function(res) {
    console.log(JSON.stringify(res));
});
```

回调函数返回对象示例：
```json
{
    "size": {
        "windowHeight": 768,
        "windowWidth": 507
    }
}
```
