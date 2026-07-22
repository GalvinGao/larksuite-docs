---
document_id: '7073693024736018437'
directory_id: '7073450228347289605'
title: CustomizedInput.update
full_path: /uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/update
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Customized Input
- CustomizedInput
- CustomizedInput.update
document_type: GuideDocumentType
updated_at: 2024-03-20T11:46:46Z
source_url: https://open.larksuite.com/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/update
---

# CustomizedInput.update(Object object)


更新输入框中显示的内容。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **X** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| content | string | 否 | \- | 文本内容<br>**示例值**：@tom @jim |
| placeholder | string | 否 | \- | 描述文本内容预期值的提示信息<br>**示例值**：reply： |
| userModelSelect | object | 否 | \- | 头像右侧pickerView，为空时不显示<br>**示例值**：{items:['real name', 'anonymous'],data:'real name'} |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>items<br></md-text> | string[] | 是 | \- | pickerView可选择的值<br>**示例值**：['real name', 'anonymous'] |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>data<br></md-text> | string | 否 | items[0]的内容 | pickerView选中的值<br>**示例值**：real name |
| avatar | string | 否 | \- | 头像图片地址，为空时不显示，当userModelSelect为空时也不显示<br>**示例值**：https://www.byte.test.png<br><md-alert type="tip" icon="none"><br>Lark[V6.4.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持 ttfile 本地图片<br></md-alert> |
| at | object[] | 否 | \- | @选择联系人列表，为空时不显示<br>**示例值**：[{id: 'xxx',name: 'tom',offset: 1,length: 12}]<br><md-alert type="tip" icon="none"><br>Lark[V3.8.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>id<br></md-text> | string | 是 | \- | 标识联系人的openID<br>**示例值**：ou_5aa316e5072c018ec1c14f0f6afeadxx |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>name<br></md-text> | string | 是 | \- | 联系人名字<br>**示例值**：tom |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>offset<br></md-text> | number | 是 | \- | @联系人所占文本在文本内容中的位置<br>**示例值**：2 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>length<br></md-text> | number | 是 | \- | @联系人 所占文本长度<br>**示例值**：2<br><md-alert type="tip" icon="none"><br>已知问题：<br>- 在 Android 设备上，length 最小值为 1<br>- 在 iOS 设备上，length 最小值为 0<br></md-alert> |
| picture | string[] | 否 | \- | 图片地址列表，需传入 ttfile 本地文件，目前只支持传入一个图片<br>**示例值**：[] |
| showEmoji | boolean | 否 | false | 是否显示表情面板<br>**示例值**：false |
| enablesReturnKey | boolean | 否 | false | 内容为空是否允许发送<br>**示例值**：false<br><md-alert type="tip" icon="none"><br>Lark[V3.7.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |


::: note
1. 输入参数仅支持全量更新，暂不支持差量更新，未指定的参数将被视作为默认值。
2. 如需指定 at 参数，请在调用 CusomizedInput 相关 API 之前，确保已经调用 [requestAccess](/document/uYjL24iN/uUzMuUzMuUzM/requestaccess) ( 如需兼容 LarkV6.9.0 以下版本，可使用 [login](/document/uYjL24iN/uYzMuYzMuYzM) )
:::

## 输出
无



## 示例代码

:::html
<md-alert type="tip">
显示输入框参考：[customizedInput.show](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/show)

</md-alert>
:::



```js
//customizedInput通过tt.getCustomizedInput()获取，并且和show方法公用一个对象实例
let prams = {
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
      customizedInput.update(params);
```





