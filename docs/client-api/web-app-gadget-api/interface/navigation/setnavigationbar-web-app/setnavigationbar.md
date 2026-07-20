---
document_id: '7073692582769262598'
directory_id: '7073450228347322373'
title: setNavigationBar
full_path: /uYjL24iN/uYjMy4iNyIjL2IjM/setnavigationbar
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Navigation
- setNavigationBar-Web App
- setNavigationBar
document_type: GuideDocumentType
updated_at: 2022-03-11T04:14:53Z
source_url: https://open.larksuite.com/document/uYjL24iN/uYjMy4iNyIjL2IjM/setnavigationbar
---

# setNavigationBar(Object object)

自定义导航栏左侧和右侧的按钮（如“返回”、“确定”），并可以监听这些按钮的点击事件，从而处理业务需要的逻辑（如“返回前的用户确认”或“重定向到指定的页面”）。
:::html
<md-alert type="tip">
注意事项：
- 设置左侧按钮后，需要主动监听 [onLeftNavigationBarClick](/document/uYjL24iN/uYjMy4iNyIjL2IjM/onleftnavigationbarclick) 处理点击事件
- 设置右侧按钮后，需要主动监听 [onRightNavigationBarClick](/document/uYjL24iN/uYjMy4iNyIjL2IjM/onrightnavigationbarclick) 处理点击事件
- PC 只支持设置图片按钮

</md-alert>
:::


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
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>/</md-td>
    </md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V5.3.0+</md-version></md-td>
      <md-td><md-version>V5.3.0+</md-version></md-td>
      <md-td><md-version>V5.3.0+</md-version></md-td>
      <md-td><md-preview-app type="gadget" disable="true" fontSize="14">预览				 </md-preview-app>
	  </md-td>
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
<md-th style="width: 25%;">
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
left
</md-td>
<md-td>
object
</md-td>
<md-td>
否
</md-td>
<md-td>

</md-td>
<md-td>
导航栏左侧设置，最多只支持两个按钮设置，即 items.length <= 2
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
object[]
</md-td>
<md-td>
 是
</md-td>
<md-td>
 
</md-td>
<md-td>
  item 数组来控制导航栏左侧的显示，数组长度为 0 则清空对应方位导航栏设置
</md-td>

</md-tr>
<md-tr>
<md-td>
&emsp;&emsp;
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
 
</md-td>
<md-td>
  item 的标志，当 item 被点击后，将触发 tt.onLeftNavigationBarClick({id:string} => {}) 监听
</md-td>

</md-tr>
<md-tr>
<md-td>
&emsp;&emsp;
<span style="color: #8F959E">
∟
</span>
&nbsp;
<md-text type="field-name">
text
</md-text>
</md-td>
<md-td>
string
</md-td>
<md-td>
 否
</md-td>
<md-td>
 
</md-td>
<md-td>
控制显示文本，空字符串表示不显示文本( PC 端不支持此字段)
  </md-td>

</md-tr>	
<md-tr>
<md-td>
&emsp;&emsp;
<span style="color: #8F959E">
∟
</span>
&nbsp;
<md-text type="field-name">
imageBase64
</md-text>
</md-td>
<md-td>
string
</md-td>
<md-td>
 否
</md-td>
<md-td>
 
</md-td>
<md-td>
控制是否显示 icon，字符串长度不能超过 10240
</md-td>

</md-tr>	

<md-tr>
<md-td>
right
</md-td>
<md-td>
object
</md-td>
<md-td>
否
</md-td>
<md-td>

</md-td>
<md-td>
导航栏右侧设置，最多只支持两个按钮设置，即 items.length <= 2
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
object[]
</md-td>
<md-td>
 是
</md-td>
<md-td>
 
</md-td>
<md-td>
  item 数组来控制导航栏右侧的显示，数组长度为 0 则清空对应方位导航栏设置
</md-td>

</md-tr>
<md-tr>
<md-td>
&emsp;&emsp;
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
 
</md-td>
<md-td>
  item 的标志，当 item 被点击后，将触发 tt.onRightNavigationBarClick({id:string} => {}) 监听
</md-td>

</md-tr>
<md-tr>
<md-td>
&emsp;&emsp;
<span style="color: #8F959E">
∟
</span>
&nbsp;
<md-text type="field-name">
text
</md-text>
</md-td>
<md-td>
string
</md-td>
<md-td>
 否
</md-td>
<md-td>
 
</md-td>
<md-td>
控制显示文本，空字符串表示不显示文本( PC 端不支持此字段)
  </md-td>

</md-tr>	
<md-tr>
<md-td>
&emsp;&emsp;
<span style="color: #8F959E">
∟
</span>
&nbsp;
<md-text type="field-name">
imageBase64
</md-text>
</md-td>
<md-td>
string
</md-td>
<md-td>
 否
</md-td>
<md-td>
 
</md-td>
<md-td>
控制是否显示 icon，字符串长度不能超过 10240
</md-td>

</md-tr>	

</md-tbody>
</md-table>
:::



## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性
    

## 示例代码

```js
tt.setNavigationBar({
    left:{
        items: [
            // 显示图片
            {id: "left_one", text: "left_xxx", imageBase64:"iVBORw0KGgoAAAANSUhEUgAAADAAAAAwCAYAAABXAvmHAAAABmJLR0QA/wD/AP+gvaeTAAADn0lEQVRoge3ZS2hcVRzH8U/SakxSjSLUViWtqLFIqWgtYmPdRBGsiAsXLoSKBR9YofhYaFGsIijiwroSX9CFFFwoCuIDRFARg4IUS7W+KqKt+KzYqk3HcXGScOfcO3PvzL0zCdgvHMjJnPM7v3Pvef4vRznK/5u+CnXOw+W4ECtwOhahhl/xC3bhQ7yPj1GvqP2OGcS9+Fow007agy1Y0nPXCfrwgfbNJ9NBPIKRHnufZY0wTMp0oo4fMNFj77M8I4zz7diIdRjDsDDMThM6ugk7cEh2J2rYqqK5uQTj2ICH8AImhcl4XFR2GMe2oX0CbsF3sjvyNPo7Nb4K+5sIz6SlnYpHDApPfCqjjeeUeBOrsS9DdCadW8Z1Bpfg+4x27iojugxfZYjWsbaMcBPOxN6onSlc2qngycLEzOrA+pJmm3EGfo7a2qO9+TXLE5oPoesrMNuMCRyJ2rtz+rc1eBz35YmcisORyCf4d/rvTVW7jngyavsAvkjkV+cJbIkEfhOG1A1Cx3KfQEkukL0y1fFRXuU+6bPNA4nfJ3B/tX4RDn93CHtNqyX8pjyhlVGFGkajMsdU5TrBOcLaHw/dZPoDx+cJ3R5VeqcLZlsxim3CIS/uwFNFBHZElbZ2xWY+i/Ewfk94yZ28hAtHsgNXdslgUUZwD14rWiE+Qqzsjq/u8ZfGDiyeWzutyTqyHilQZt6wMON/U1F+qBdGmjAsBAhmOIjP8ip9qnEIXdYVa8VYH3l5MS6QNTx2R/lV1fsqzFiU3xkXyOrArig/l29gPMq/V6TSxRpf2z84sVpfhRjSuIkdkr6LZ9IvfScudbXrkI1yxn8rtkWV9+vtajSIbyMPV7UjMCp9KnywWo8teSxq+3MsaFdku/RcKHSYKsk66UjfdZ0ILRPO30mhvbobjD0LP0ZtvqlEfOjmSKwubHSnFKzfTjRhuXQY54D0ZaptnpfuxDdCHKcV5+PVgm2MSz/5Gq7twG+KAbwRif+Ek5qUXyhc+g/j5RztITwq+wJ/W1njSQbwSkK82cV6hcYLUbPb3Ajulh3crenSvrNAiFJPSh9B+rFZOmy+QQjHjAmfnzbjdfydYXzm0n51N8wniTe05Xi7iaF20ls4u9vms1gr7BGdGt+tw3W+Sm7Vnuk/8RKuUPJrTFWfWeFZ3JjI7xO+exHmxpfTaRLvCm9tXjGgcQXKDQHOR5YKT72Oi+bYS8eMC8vkork2UoZretXQf4MZqV9JCxyjAAAAAElFTkSuQmCC" },
            // 显示文本
            {id: "left_two", text: "left_xxx"}
        ]            
    } ,
    right:{
        items: [
            // 显示图片
            {id: "right_one", text: "right_xxx", imageBase64:"iVBORw0KGgoAAAANSUhEUgAAADAAAAAwCAYAAABXAvmHAAAABmJLR0QA/wD/AP+gvaeTAAADn0lEQVRoge3ZS2hcVRzH8U/SakxSjSLUViWtqLFIqWgtYmPdRBGsiAsXLoSKBR9YofhYaFGsIijiwroSX9CFFFwoCuIDRFARg4IUS7W+KqKt+KzYqk3HcXGScOfcO3PvzL0zCdgvHMjJnPM7v3Pvef4vRznK/5u+CnXOw+W4ECtwOhahhl/xC3bhQ7yPj1GvqP2OGcS9+Fow007agy1Y0nPXCfrwgfbNJ9NBPIKRHnufZY0wTMp0oo4fMNFj77M8I4zz7diIdRjDsDDMThM6ugk7cEh2J2rYqqK5uQTj2ICH8AImhcl4XFR2GMe2oX0CbsF3sjvyNPo7Nb4K+5sIz6SlnYpHDApPfCqjjeeUeBOrsS9DdCadW8Z1Bpfg+4x27iojugxfZYjWsbaMcBPOxN6onSlc2qngycLEzOrA+pJmm3EGfo7a2qO9+TXLE5oPoesrMNuMCRyJ2rtz+rc1eBz35YmcisORyCf4d/rvTVW7jngyavsAvkjkV+cJbIkEfhOG1A1Cx3KfQEkukL0y1fFRXuU+6bPNA4nfJ3B/tX4RDn93CHtNqyX8pjyhlVGFGkajMsdU5TrBOcLaHw/dZPoDx+cJ3R5VeqcLZlsxim3CIS/uwFNFBHZElbZ2xWY+i/Ewfk94yZ28hAtHsgNXdslgUUZwD14rWiE+Qqzsjq/u8ZfGDiyeWzutyTqyHilQZt6wMON/U1F+qBdGmjAsBAhmOIjP8ip9qnEIXdYVa8VYH3l5MS6QNTx2R/lV1fsqzFiU3xkXyOrArig/l29gPMq/V6TSxRpf2z84sVpfhRjSuIkdkr6LZ9IvfScudbXrkI1yxn8rtkWV9+vtajSIbyMPV7UjMCp9KnywWo8teSxq+3MsaFdku/RcKHSYKsk66UjfdZ0ILRPO30mhvbobjD0LP0ZtvqlEfOjmSKwubHSnFKzfTjRhuXQY54D0ZaptnpfuxDdCHKcV5+PVgm2MSz/5Gq7twG+KAbwRif+Ek5qUXyhc+g/j5RztITwq+wJ/W1njSQbwSkK82cV6hcYLUbPb3Ajulh3crenSvrNAiFJPSh9B+rFZOmy+QQjHjAmfnzbjdfydYXzm0n51N8wniTe05Xi7iaF20ls4u9vms1gr7BGdGt+tw3W+Sm7Vnuk/8RKuUPJrTFWfWeFZ3JjI7xO+exHmxpfTaRLvCm9tXjGgcQXKDQHOR5YKT72Oi+bYS8eMC8vkork2UoZretXQf4MZqV9JCxyjAAAAAElFTkSuQmCC" },
            // 显示文本
            {id: "right_two", text: "right_xxx"}
        ],
    },    
    success(res) {
        console.log("success");
    },
    fail(err) {
        console.log(`${JSON.stringify(err)}`);
    }                 
    
});

tt.onLeftNavigationBarClick(ev => {
    alert(`你点击了左上角id 为 ${ev.id} 的按钮`)
})
tt.onRightNavigationBarClick(ev =>{
    alert(`你点击了右上角 id 为 ${ev.id} 的按钮`)
})
```

`success`返回对象示例：

```js
{
    "errMsg": "setNavigationBar:ok"
}
``` 

