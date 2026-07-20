---
document_id: '6965379543683465222'
directory_id: '6907567266537734145'
title: openSchema
full_path: /uYjL24iN/ukzN4IjL5cDOy4SO3gjM
breadcrumb:
- Client API
- Web app/Gadget API
- Navigation
- openSchema
document_type: GuideDocumentType
updated_at: 2023-06-30T09:25:56Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukzN4IjL5cDOy4SO3gjM
---

# openSchema(Object object)

跳转到小程序以外的应用。

:::html
<md-alert type="tip">
注意事项：
- 如果出现无法跳转的现象，请先检查 schema 是否在[白名单](/document/uYjL24iN/uEjMxYjLxITM24SMyEjN)里。
白名单可以在「开发者后台」对应 App 的「安全设置」里配置 ( 网页应用无需配置白名单 )。
- PC 端在Lark 3.41.0 及以上版本需要对内置独立窗口可以打开的网页配置白名单，你可以通过配置为`*:*`来允许打开任意链接。
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
      <md-td><md-version>V3.1.0+</md-version></md-td>
      <md-td><md-version>V3.1.0+</md-version></md-td>
      <md-td><md-version>V3.1.0+</md-version></md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/openschema/openschema" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V3.44+</md-version></md-td>
      <md-td><md-version>V3.44+</md-version></md-td>
      <md-td><md-version>V3.47+</md-version></md-td>
      <md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app></md-td>
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
            <md-td>schema</md-td>
            <md-td>string</md-td>
            <md-td>是</md-td>
            <md-td></md-td>
            <md-td>
                指定应用的 schema，schema需要满足 URI 协议。

**示例值**：https://open.larksuite.com
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>external</md-td>
            <md-td>boolean</md-td>
            <md-td>否</md-td>
            <md-td>false</md-td>
            <md-td>
                是否跳转到Lark以外的应用(浏览器或其他应用程序)。内部应用（如 Doc / 小程序等）不受此参数限制。
              
**示例值**：false
<md-alert type="tip" icon="none">
- PC 端：Lark[V3.38.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持，通过指定 external 为 false 使用内置独立窗口打开网页，同时支持通过 options 参数指定窗口尺寸
- Android/iOS 端：Lark[V3.1.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
</md-alert> 
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>options</md-td>
            <md-td>object</md-td>
            <md-td>否</md-td>
            <md-td></md-td>
            <md-td>
                用于指定额外参数的对象

**示例值**：{"width":1200,"height":700}
<md-alert type="tip" icon="none">
- PC 端：Lark[V3.38.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
- Android/iOS 端：暂不支持
</md-alert> 
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">∟</span>
                &nbsp;
                <md-text type="field-name">width</md-text>
            </md-td>
            <md-td>number</md-td>
            <md-td>否</md-td>
            <md-td>640</md-td>
            <md-td>
                用于指定打开的端内容器的宽度，仅当 external 为 false 时生效。
              
**最小值**：640</br>
**最大值**：屏幕的宽度</br>
**默认值**：Lark窗口的宽度
<md-alert type="tip" icon="none">
Lark[V5.12.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本 宽度最小值从1200调整为640
</md-alert> 

            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">∟</span>
                &nbsp;
                <md-text type="field-name">height</md-text>
            </md-td>
            <md-td>number</md-td>
            <md-td>否</md-td>
            <md-td>480</md-td>
            <md-td>
                用于指定打开的端内容器的高度，仅当 external 为 false 时生效。

**最小值**：480</br>
**最大值**：屏幕的高度</br>
**默认值**：Lark窗口的高度
<md-alert type="tip" icon="none">
Lark[V5.12.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本 高度最小值从700调整为480
</md-alert> 

            </md-td>
        </md-tr>
             <md-tr>
            <md-td>
                target
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                inline
            </md-td>
            <md-td>
                指定[sidebar模式](/document/uYjL24iN/uIjNzUjLyYzM14iM2MTN)小程序(A)调用openSchema接口打开另外一个[sidebar模式](/document/uYjL24iN/uIjNzUjLyYzM14iM2MTN)的小程序(B)时的模式



**可选值**：
- `inline`：打开新应用(B)时，当前应用(A)会保留。当前应用(A)被关闭时，新应用(B)会一起被关闭
- `replace`：打开新应用(B)时，当前应用(A)会被关闭
<md-alert type="tip" icon="none">
- PC 端：Lark[V5.13.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
- iOS/Android 端：暂不支持
</md-alert> 
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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/openschema/openschema" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.openSchema({
    schema: "https://larksuite.com/",
    external: true,
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`openSchema fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "openSchema:ok"
}
```
