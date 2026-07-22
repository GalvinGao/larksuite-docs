---
document_id: '7073692582770278406'
directory_id: '7073450228347305989'
title: Animation.translate3d
full_path: /uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_translate3d
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Animation
- Animation
- Animation.translate3d
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:26Z
source_url: https://open.larksuite.com/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_translate3d
---

# Animation.translate3d(number tx, number ty, number tz)
对 xyz 坐标进行平移变换
  
    
  ## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/animation/animation" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |

  
    
  ## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| tx | number | 是 | / | 在 X 轴平移的距离，单位为 px |
| ty | number | 否 | / | 在 Y 轴平移的距离，单位为 px |
| tz | number | 否 | / | 在 Z 轴平移的距离，单位为 px |

  
    
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

  animation.translate3d(10, 10, 10).step();
  ```
