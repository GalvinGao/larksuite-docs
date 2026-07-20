---
document_id: '7073692582770376710'
directory_id: '7073450228347305989'
title: Animation.skewX
full_path: /uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_skewx
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Animation
- Animation
- Animation.skewX
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:26Z
source_url: https://open.larksuite.com/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_skewx
---

# Animation.skewX(number angle)
对 X 轴坐标进行倾斜
  
    
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
        <md-td>**✓**</md-td>
        <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/animation/animation" fontSize="14">预览</md-preview-app>
  </md-td>
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
            angle
        </md-td>
        <md-td>
            number
        </md-td>
        <md-td>
            是
        </md-td>
        <md-td>/</md-td>
        <md-td>
            倾斜的角度，范围 [-180, 180]
        </md-td>
    </md-tr>
  
      </md-tbody>
  </md-table>
  :::
  
    
## 输出

返回值：  

`Animation` 实例

## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/animation/animation" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::
  
    
  ```js
  const animation = tt.createAnimation();

  animation.skewX(90).step();
  ```
