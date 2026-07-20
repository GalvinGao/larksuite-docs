---
document_id: '6965379567070560261'
directory_id: '6907567266540847106'
title: 标准对象
full_path: /uYjL24iN/ukzNy4SO3IjL5cjM
breadcrumb:
- Client API
- Web app/Gadget API
- Standard Object Input
document_type: GuideDocumentType
updated_at: 2023-02-14T08:43:21Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukzNy4SO3IjL5cjM
---

# 标准对象

定义 API 的标准输入、输出内容。
绝大多数 API 为异步调用，调用的结果通过预先定义好的三个回调方法返回

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
      <md-td>success</md-td>
      <md-td>function</md-td>
      <md-td>否</md-td>
      <md-td></md-td>
      <md-td>接口调用成功的回调函数</md-td>

   </md-tr>  
    
	<md-tr>
      <md-td>fail</md-td>
      <md-td>function</md-td>
      <md-td>否</md-td>
      <md-td></md-td>
      <md-td>接口调用失败的回调函数</md-td>

   </md-tr>  
    
    <md-tr>
      <md-td>complete</md-td>
      <md-td>function</md-td>
      <md-td>否</md-td>
      <md-td></md-td>
      <md-td>接口调用完成的回调函数（成功或失败都会执行），该回调发生在 `success` 和 `fail` 之后       
      </md-td>

   </md-tr>  
    
</md-tbody>
</md-table>
:::


## 输出

`success`返回对象`res`的基础属性：
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
                errMsg
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                错误信息，返回消息格式为`${API_NAME}:ok`
              
**示例值**：chooseContact:ok
            </md-td>
        </md-tr>
   
    </md-tbody>
</md-table>
:::
<BR>

`fail`返回对象`res`的基础属性：
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
                errCode
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                错误码
              
**示例值**：1716201
<md-alert type="tip" icon="none">
目前仅部分API存在错误码，可参考API文档中的"错误码"说明，如[addTabBarItem](/uYjL24iN/uQjM04CNyQjL0IDN/addtabbaritem)。错误码的覆盖范围正在逐步扩展中。
</md-alert>    
            </md-td>
        </md-tr>
      <md-tr>
            <md-td>
                errMsg
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                错误信息，返回消息格式为`${API_NAME}:fail`
              
**示例值**：chooseContact:fail
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::
<BR>

`complete`返回对象`res`的基础属性：
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
                errMsg
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                错误信息，与 `success` 或 `fail` 返回对象的`errMsg`值相同
            </md-td>
        </md-tr>
   
    </md-tbody>
</md-table>
:::

:::note
API 执行的返回结果会扩展`success`、`fail`、`complete`返回对象的属性，具体参见各 API 详细说明。如[chooseChat](/document/uYjL24iN/uMTN3QjLzUzN04yM1cDN)
:::

## 示例代码
``` js
var someSwitch = 'on';
Page({
    oneFunc () {
        if (someSwitch === 'on') {
            someSwitch = 'off';
            
            tt.someAPI({
                someattr: somevalue,
                success (res) {
                    console.log(res.someattr);
                },
                fail (res) {
                    console.log(`run fail`);
                },
                complete (res) {
                    console.log(`run done`);
                    someSwitch = 'on';
                }
            });
        }
    }
});
```
