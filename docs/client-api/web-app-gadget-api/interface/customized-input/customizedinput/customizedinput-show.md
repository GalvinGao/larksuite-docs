---
document_id: '7073691561007710213'
directory_id: '7073450228347289605'
title: CustomizedInput.show
full_path: /uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/show
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Customized Input
- CustomizedInput
- CustomizedInput.show
document_type: GuideDocumentType
updated_at: 2024-03-20T11:46:42Z
source_url: https://open.larksuite.com/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/show
---

# CustomizedInput.show(Object object)

显示输入框


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
      <md-td>**X**</md-td>
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
                content
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
              	\-
            </md-td>
            <md-td>
                文本内容

**示例值**：@tom @jim
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                placeholder
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                \-
            </md-td>
            <md-td>
                描述文本内容预期值的提示信息

**示例值**：reply：
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                userModelSelect
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                \-
            </md-td>
            <md-td>
                头像右侧pickerView，为空时不显示

**示例值**：{items:['real name', 'anonymous'],data:'real name'}
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
                    items
                </md-text>
            </md-td>
            <md-td>
                string[]
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td>
                \-
            </md-td>
            <md-td>
                pickerView可选择的值

**示例值**：['real name', 'anonymous']
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
                    data
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                items[0]的内容
            </md-td>
            <md-td>
                pickerView选中的值

**示例值**：real name
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                avatar
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                \-
            </md-td>
            <md-td>
                头像图片地址，为空时不显示，当userModelSelect为空时也不显示

**示例值**：https://www.byte.test.png
<md-alert type="tip" icon="none">
Lark[V6.4.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持 ttfile 本地图片
</md-alert>
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                at
            </md-td>
            <md-td>
                object[]
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                \-
            </md-td>
            <md-td>
                @选择联系人列表，为空时不显示

**示例值**：[{id: 'xxx',name: 'tom',offset: 1,length: 12}]
<md-alert type="tip" icon="none">
Lark[V3.8.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
</md-alert>
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
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td>
                \-
            </md-td>
            <md-td>
                标识联系人的openID

**示例值**：ou_5aa316e5072c018ec1c14f0f6afeadxx
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
                    name
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td>
                \-
            </md-td>
            <md-td>
                联系人名字
              
**示例值**：tom

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
                    offset
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td>
                \-
            </md-td>
            <md-td>
                @联系人所占文本在文本内容中的位置

**示例值**：2
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
                    length
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td>
                \-
            </md-td>
            <md-td>
                @联系人 所占文本长度

**示例值**：2
<md-alert type="tip" icon="none">
已知问题：
- 在 Android 设备上，length 最小值为 1
- 在 iOS 设备上，length 最小值为 0
</md-alert>
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                picture
            </md-td>
            <md-td>
                string[]
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                \-
            </md-td>
            <md-td>
                图片地址列表，需传入 ttfile 本地文件，目前只支持传入一个图片

**示例值**：[]
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                showEmoji
            </md-td>
            <md-td>
                boolean
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                false
            </md-td>
            <md-td>
                是否显示表情面板

**示例值**：false
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                enablesReturnKey
            </md-td>
            <md-td>
                boolean
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                false
            </md-td>
            <md-td>
                内容为空是否允许发送

**示例值**：false
<md-alert type="tip" icon="none">
Lark[V3.7.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
</md-alert> 
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

::: note
如需指定 at 参数，请在调用 CusomizedInput 相关 API 之前，确保已经调用 [requestAccess](/document/uYjL24iN/uUzMuUzMuUzM/requestaccess) ( 如需兼容 LarkV6.9.0 以下版本，可使用 [login](/document/uYjL24iN/uYzMuYzMuYzM) )
:::

## 输出
无


## 示例代码

```js
let params = {
          "picture": [],
          "at": [
              {
                  "id": "xxx",
                  "name": "Jiasheng Wu",
                  "offset": 1,
                  "length": 12
              },
              {
                  "id": "xxx",
                  "name": "Zhiyou Hou",
                  "offset": 14,
                  "length": 11
              }
          ],
          "userModelSelect": {
              "items": [
                  "real name",
                  "anonymous"
              ],
              "data": "real name"
          },
          "placeholder": "reply：",
          "content": " @Jiasheng Wu @Zhiyou Hou ",
          "avatar": "https://xxx.png",
          "showEmoji": true,
          "enablesReturnKey": true
      }
      let customizedInput = tt.getCustomizedInput();
      customizedInput.show(params);
```



