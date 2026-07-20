---
document_id: '6965379543684251654'
directory_id: '6907567269107367938'
title: setTabBarStyle
full_path: /uYjL24iN/uITN04iM1QjLyUDN
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Tab Bar
- setTabBarStyle
document_type: GuideDocumentType
updated_at: 2022-03-11T04:14:38Z
source_url: https://open.larksuite.com/document/uYjL24iN/uITN04iM1QjLyUDN
---

# setTabBarStyle(Object object)


动态设置 tabBar（小程序底部tab栏） 的整体样式


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
                color
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                tab 上的文字默认颜色，6 位 HexColor 例如"#ff3377"

**示例值**：'#FF0000'
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                selectedColor
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                tab 上的文字选中时的颜色，6 位 HexColor 例如"#ff3377"

**示例值**：'#00FF00'
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                backgroundColor
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                tab 的背景色，6 位 HexColor 例如"#ff3377"

**示例值**：'#0000FF'
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                borderStyle
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td>white</md-td>
            <md-td>
                tabBar上边框的颜色， 仅支持 black/white

**示例值**：'white'

**可选值**：
- `white`：白
- `black`：黑
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                borderColor
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                tabBar上边框的颜色,优先级高于borderStyle。适用4.3及以后版本。

**示例值**：'#00FF00'
<md-alert type="tip" icon="none">
Lark[V4.3.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/index?showTabBarPage=true" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
tt.setTabBarStyle({
    color: "#FF0000",
    selectedColor: "#00FF00",
    backgroundColor: "#0000FF",
    borderStyle: "white",
    borderColor: "#00FF00",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`setTabBarStyle fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "setTabBarStyle:ok"
}
```
