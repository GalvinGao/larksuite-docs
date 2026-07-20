---
document_id: '6965379543683448838'
directory_id: '6907567269107367938'
title: showTabBarRedDot
full_path: /uYjL24iN/uYjM04iNyQjL2IDN
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Tab Bar
- showTabBarRedDot
document_type: GuideDocumentType
updated_at: 2022-03-11T04:14:47Z
source_url: https://open.larksuite.com/document/uYjL24iN/uYjM04iNyQjL2IDN
---

# 	showTabBarRedDot(Object object)


显示 tabBar（小程序底部tab栏） 某一项的右上角的红点，可以使用底部标签栏红点给予用户提示。相关 API： [hideTabBarRedDot](/document/uYjL24iN/ugjM04COyQjL4IDN)


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
                index
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                tabBar 的哪一项，从左边算起
              
**最小值**：`0`
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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/index?showTabBarPage=true" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
tt.showTabBarRedDot({
    index: 0,
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`showTabBarRedDot fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "showTabBarRedDot:ok"
}
```
