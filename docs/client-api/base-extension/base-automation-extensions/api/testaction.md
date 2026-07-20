---
document_id: '7260082411119017989'
directory_id: '7258197168736616454'
title: testAction
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/api/testaction
breadcrumb:
- Client API
- Base Extension
- Base Automation Extensions
- API
- testAction
document_type: GuideDocumentType
updated_at: 2023-07-26T11:07:51Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/api/testaction
---

# testAction
用于在本地模拟调用 addAction 的execute函数的方法。在test/index中，模板已经写好此函数示范用例。使用时在终端使用`npm run test`即可使用test/index中的模拟输入。



## 输入
```js
testAction(args, context)
```

:::html
<md-table>
  <colgroup>
    <col style="width: auto">
    <col style="width: auto">
    <col style="width: auto">
    <col style="width: auto">
  </colgroup>
	<md-thead> 
      <md-tr>
      	<md-th>名称</md-th>
        <md-th>数据类型</md-th>
        <md-th>是否必填</md-th>
        <md-th>描述</md-th>
      </md-tr>
  </md-thead> 
  	<md-tbody>
      <md-tr>
      	<md-td>args</md-td>
        <md-td>{
    [key: string]: any;
}</md-td>
        <md-td>是</md-td>
        <md-td>模拟运行用户的入参</md-td>
      </md-tr>
      <md-tr style="">
      	<md-td>context</md-td>
        <md-td>Context</md-td>
        <md-td>否</md-td>
        <md-td>模拟运行时上下文入参</md-td>
      </md-tr>
    </md-tbody>
</md-table>
:::


## 输出
一个对象。本地运行`execute`函数的出参。
## 示例代码
### 调用示例
```js
import { testAction, createActionContext } from '@lark-opdev/block-basekit-server-api';

async function test() {
    const actionContext = await createActionContext({
      tenantAccessToken: '',
    });
    
    testAction({
        text: 'hello world',
        transformType: 'toUpperCase',
    },
    actionContext);
}

test();

basekit.addAction({
  // 定义运行逻辑
  execute: async function (args, context) {
    // npm run test的时候，args的值:{ text: 'hello world', transformType: 'toUpperCase', }
    const { text = '', transformType } = args;
    // 根据转换类型将源文本做大小写转换
    const outputText = transformType === 'toUpperCase'
      ? text.toUpperCase()
      : text.toLowerCase();
    // 返回转换后的数据
    return {
      text: outputText,
    };
  },
  //....
      
```
### 返回示例
将在终端看到如下模拟输入的运行结果：
```js
{
    text: 'HELLO WORLD'
}
