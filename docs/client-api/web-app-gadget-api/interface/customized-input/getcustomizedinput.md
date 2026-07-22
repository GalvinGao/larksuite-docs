---
document_id: '6965379541104279557'
directory_id: '6907567266536751105'
title: getCustomizedInput
full_path: /uYjL24iN/uEDN1EjLxQTNx4SM0UTM
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Customized Input
- getCustomizedInput
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:26Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEDN1EjLxQTNx4SM0UTM
---

# getCustomizedInput()

获取**全局唯一**的`CustomizedInput`实例。通过`CustomizedInput`显示一个 可定制化的富文本输入框，支持@联系人、插入图片、插入表情、显示用户头像、切换用户头像状态。




## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **X** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |




## 输入
无


## 输出

返回值：`CustomizedInput`，该对象的方法列表参见下表：

:::html
<md-alert type="tip">
点击下表中的方法名，查看对应API的支持说明、调用方法
</md-alert>
:::

| 方法 | 介绍 |
| --- | --- |
| [show(Object object)](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/show) | 显示输入框。 |
| [update(Object object)](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/update) | 更新输入框中显示的内容。params参数与show(params)一致 |
| [hide()](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/hide) | 隐藏输入框 |
| [onPicSelect(function callabck)](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/onpicselect) | 监听连接成功的事件回调 |
| [onModelSelect(function callback)](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/onmodelselect) | 选择pickerView之后触发的事件，res参数与`onPicSelect((res) => {})`一致 |
| [onPublish(function callback)](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/onpublish) | 点击发送按钮触发的事件，res参数与`onPicSelect((res) => {})`一致。 |
| [onHide(function callback)](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/onhide) | 隐藏输入框 |

## 示例代码


```js
try {
    let result = tt.getCustomizedInput();
    console.log(`getCustomizedInput success: ${JSON.stringify(result)}`);
} catch (error) {
    console.log(`getCustomizedInput fail: ${JSON.stringify(error)}`);
}
```

返回值示例：
```json
getCustomizedInput success: {}
```






