---
document_id: '6965379543684087814'
directory_id: '6907567266537734145'
title: navigateBack
full_path: /uYjL24iN/uADM04CMwQjLwADN
breadcrumb:
- Client API
- Web app/Gadget API
- Navigation
- navigateBack
document_type: GuideDocumentType
updated_at: 2022-03-11T04:19:42Z
source_url: https://open.larksuite.com/document/uYjL24iN/uADM04CMwQjLwADN
---

# navigateBack(Object object)

返回上一级页面（或上N级页面）。可通过 getCurrentPages 获取当前的页面栈，决定需要返回几层。

:::html
<md-alert type="tip">

 使用 [tt.redirectTo](/document/uYjL24iN/ucTOz4yN5MjL3kzM) 方法跳转新页面，会同时关闭原页面，因此不会增加页面栈的层级，使用 [tt.navigateBack](/document/uYjL24iN/uADM04CMwQjLwADN) 时需要要注意其页面层级关系
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
      <md-td>**✓**</md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/navigator/navigator" fontSize="14">预览</md-preview-app>
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

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：
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
                delta
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                1
            </md-td>
            <md-td>
                返回的层级数，如果 delta 大于现有层级数，则返回到首页。**PC只支持delta=1**

**示例值**：1
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::



## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性


## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/navigator/navigator" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
        
  </div>
</div> 
:::

```js
tt.navigateBack({
    delta: 1,
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`navigateBack fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "navigateBack:ok"
}
```
