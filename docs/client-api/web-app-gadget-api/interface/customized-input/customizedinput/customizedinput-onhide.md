---
document_id: '7073692582769082374'
directory_id: '7073450228347289605'
title: CustomizedInput.onHide
full_path: /uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/onhide
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Customized Input
- CustomizedInput
- CustomizedInput.onHide
document_type: GuideDocumentType
updated_at: 2023-12-07T03:50:10Z
source_url: https://open.larksuite.com/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/onhide
---

# customizedInput.onHide(function callback)

隐藏输入框之后触发的事件

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **X** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |




## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| callback | function | 是 |  | 该事件的回调函数 |


## 输出
回调函数返回对象的属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| content | string | 文本内容 |
| userModelSelect | object | 头像右侧pickerView，为空时不显示<br>**示例值**：{items:['real name', 'anonymous'],data:'real name'} |
| at | object[] | @选择联系人列表 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>id<br></md-text> | string | 标识联系人的openID |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>name<br></md-text> | string | 联系人名字 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>offset<br></md-text> | number | @联系人所占文本在文本内容中的位置 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>length<br></md-text> | number | @联系人 所占文本长度 |
| picture | string[] | 图片地址列表，目前只支持传入一个图片 |



## 示例代码
:::html
<md-alert type="tip">
显示输入框参考：[customizedInput.show](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/show)

</md-alert>
:::

```js
//customizedInput通过tt.getCustomizedInput()获取，并且和show方法公用一个对象实例

customizedInput.onHide(function(res) {
    console.log(JSON.stringify(res));
});

```

回调函数返回对象示例：

```json
{"content":"@Jiasheng Wu @Zhiyou Hou"}
``` 


