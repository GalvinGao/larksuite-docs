---
document_id: '7073692582769475590'
directory_id: '7073450228347289605'
title: CustomizedInput.onPicSelect
full_path: /uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/onpicselect
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Customized Input
- CustomizedInput
- CustomizedInput.onPicSelect
document_type: GuideDocumentType
updated_at: 2023-12-07T03:49:58Z
source_url: https://open.larksuite.com/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/onpicselect
---

# customizedInput.onPicSelect(function callback)

监听连接成功的事件回调。

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
      <md-td>/
      </md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::



## 输入
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
                callback
            </md-td>
            <md-td>
                function
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                该事件的回调函数
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 输出
回调函数返回对象的属性：
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 30%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
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
                文本内容
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
                头像右侧pickerView，为空时不显示

**示例值**：{items:['real name', 'anonymous'],data:'real name'}
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
                @选择联系人列表
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
                标识联系人的openID
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
                联系人名字
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
                @联系人所占文本在文本内容中的位置
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
                @联系人 所占文本长度
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
                图片地址列表，目前只支持传入一个图片
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 示例代码


```js
const customizedInput = tt.getCustomizedInput();
customizedInput.onPicSelect(function(res) {
    console.log(JSON.stringify(res));
});
```




