---
document_id: '7073692582769278982'
directory_id: '7073450228347289605'
title: CustomizedInput.onModelSelect
full_path: /uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/onmodelselect
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Customized Input
- CustomizedInput
- CustomizedInput.onModelSelect
document_type: GuideDocumentType
updated_at: 2023-12-07T03:50:02Z
source_url: https://open.larksuite.com/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/onmodelselect
---

# customizedInput.onModelSelect(function callback)

选择pickerView之后触发的事件


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
      <md-th style="width: 20%;">名称</md-th>
      <md-th style="width: 18%;">数据类型</md-th>
       <md-th style="width: 10%;">必填</md-th>
      <md-th style="width: 10%;">默认值</md-th>
      <md-th>描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>

    
   <md-tr>
      <md-td>callback</md-td>
      <md-td>function</md-td>
      <md-td>是</md-td>
      <md-td></md-td>
      <md-td>该事件的回调函数</md-td>

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
:::html
<md-alert type="tip">
显示输入框参考：[customizedInput.show](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/show)

</md-alert>
:::

```js
//customizedInput通过tt.getCustomizedInput()获取，并且和show方法公用一个对象实例
customizedInput.onModelSelect({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`onModelSelect fail: ${JSON.stringify(res)}`);
    }
});
```
回调函数返回对象示例：

```json
{
    "userModelSelect": "anonymous",
    "at": [
        {
            "id": "xxx",
            "name": "@Jiasheng Wu",
            "offset": 1,
            "length": 12
        },
        {
            "id": "xxx",
            "name": "@Zhiyou Hou",
            "offset": 14,
            "length": 11
        }
    ],
    "content": " @Jiasheng Wu @Zhiyou Hou "
}
``` 



