---
document_id: '7180270043523383302'
directory_id: '7180165099248205829'
title: onThemeChange
full_path: /uAjLw4CM/uYjL24iN/block/api/darkmode/onthemechange
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- DarkMode
- onThemeChange
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/darkmode/onthemechange
---

# onThemeChange

监听系统主题变化。
:::html
<md-alert type="tip">
由于 Android 系统主题的刷新机制会导致整个页面重新创建，在 Android 设备上并不能通过此接口实时监听到系统主题变化。为达到最佳的适配效果，请在 Block 初次加载时，调用 [tt.getSystemInfo](/document/uAjLw4CM/uYjL24iN/block/api/device/getsysteminfo) 接口对 theme 进行处理。
</md-alert>
:::

## 输入
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
          <md-th style="width: 18%;">
                是否必填
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
            <md-td>
                当主题变化时的回调函数
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
                theme
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                表示当前系统的主题模式，开启 Dark Mode 之后才会返回该字段
  -  `dark`：当前系统处于暗色模式
  -  `light`：当前系统处于亮色模式                          
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


## 示例代码

```js
tt.onThemeChange(({ theme }) => {
  console.log('onThemeChange', theme);
});
```
回调函数返回对象示例：
```json
{
  "theme": "dark"
}
```
