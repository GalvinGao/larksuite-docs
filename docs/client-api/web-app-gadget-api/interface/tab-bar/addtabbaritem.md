---
document_id: '7073693024735952901'
directory_id: '6907567269107367938'
title: addTabBarItem
full_path: /uYjL24iN/uQjM04CNyQjL0IDN/addtabbaritem
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Tab Bar
- addTabBarItem
document_type: GuideDocumentType
updated_at: 2022-03-11T04:14:32Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQjM04CNyQjL0IDN/addtabbaritem
---

# 	addTabBarItem(Object object)


增加tab bar（小程序底部tab栏）的目标item


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
      <md-td><md-version>V5.1.0+</md-version></md-td>
      <md-td><md-version>V5.1.0+</md-version></md-td>
      <md-td><md-version>V5.1.0+</md-version></md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" disable="true" fontSize="14">预览</md-preview-app>
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
                tab索引

**示例值**：0，1，2
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                pagePath
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                tab路径

**示例值**：'pages/index/index'
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                text
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                tab展示文案，可以为“”但是不能为`null`
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                light
            </md-td>
            <md-td>
                object[]
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                日间模式数据
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
                    iconPath
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                日间模式tab非选中图片，图片cdn地址以及本地打包图片资源
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
                    selectedIconPath
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                日间模式tab选中图片，图片cdn地址以及本地打包图片资源
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                dark
            </md-td>
            <md-td>
                object[]
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                夜间模式数据
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
                    iconPath
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                夜间模式tab非选中图片，图片cdn地址以及本地打包图片资源
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
                    selectedIconPath
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                夜间模式tab选中图片，图片cdn地址以及本地打包图片资源
            </md-td>
        </md-tr>
        
    </md-tbody>
</md-table>
:::

## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性


## 示例代码


```js
tt.addTabBarItem({
    index: 1, 
    pagePath:"pages/index/index", 
    text: "text", 
    light: {
      iconPath: "xx", 
      selectedIconPath: "xx" 
    },
    dark: {
      iconPath: "xx", 
      selectedIconPath: "xx" 
    },
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`addTabBarItem fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
	"errMsg":"addTabBarItem:ok"
}
```

## 错误码
`fail`返回对象中会包含[errCode属性](/document/uYjL24iN/ukzNy4SO3IjL5cjM#a825f4c8)，代表错误码，具体错误码列表参见：

错误码 | 错误信息 |描述
--|--|--|--|--
|`-10001` | fatal error : no request  |api业务中的model为空|
| `-10002` | no page path |api业务中的添加tab的page path为空                |
| `-10003 `| no page text                                | api业务中的添加tab的page text为空                |
| `-10004 `| no page lightIcon                           |api业务中的添加tab的light icon model为空         |
| `-10005` | no page lightIcon iconPath                  |api业务中的添加tab的light icon path为空          |
| `-10006` | no page lightIcon selectedIconPath          |api业务中的添加tab的light icon selected path为空 |
| `-10007` | no page darkIcon                            |api业务中的添加tab的dark icon model为空          |
| `-10008` | no page darkIcon iconPath                   |api业务中的添加tab的dark icon path为空           |
| `-10009` | no page darkIcon selectedIconPath           |api业务中的添加tab的dark icon selected path为空  |
| `-10010` | at most 5 tabs should be remained           |已有最多5个tab，无法添加                          |
| `-10011` | index is invalid                            |添加index索引无效（为负数或者大于现有tab数量）             |
| `-10012` | this tab already exists                     |添加tab的pagePath已经存在                      |
| `-10013` | no tab                                      |本身无tab时，不能添加                            |
| `-20001` | no current tab controller                   |Tab fragment为空                          |
| `-20002` | no current app view proxy                   | Tab fragment的activity为空                 |
|` -20003` | only support in gadget                      |不是小程序形态 |
| `-20004` | no tab host                                 | Tab host初始化错误|                          
| `-20005` | add target tab fail, cannot find target tab| 添加tabwidget时发生错误|
