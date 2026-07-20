---
document_id: '6965379543684317190'
directory_id: '6907567269107367938'
title: hideTabBar
full_path: /uYjL24iN/ukDN04SO0QjL5QDN
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Tab Bar
- hideTabBar
document_type: GuideDocumentType
updated_at: 2022-03-11T04:14:26Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukDN04SO0QjL5QDN
---

# hideTabBar(Object object)


隐藏 tabBar（小程序底部tab栏）


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
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/index?showTabBarPage=true" fontSize="14">预览</md-preview-app>
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
      <md-th style="width: 20%;">名称</md-th>
      <md-th style="width: 18%;">数据类型</md-th>
       <md-th style="width: 10%;">必填</md-th>
      <md-th style="width: 10%;">默认值</md-th>
      <md-th>描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
 <md-tr>
      <md-td>animation</md-td>
      <md-td>boolean</md-td>
      <md-td>否</md-td>
      <md-td>false</md-td>
       <md-td>是否需要动画效果。
   <md-alert type="tip" icon="none">
- PC 端：暂不支持
</md-alert></md-td>
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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/index?showTabBarPage=true" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
tt.hideTabBar({
    animation: true,
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`hideTabBar fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "hideTabBar:ok"
}
```
