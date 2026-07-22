---
document_id: '7073691561008234501'
directory_id: '7073450228347305989'
title: Animation.step
full_path: /uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_step
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Animation
- Animation
- Animation.step
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:26Z
source_url: https://open.larksuite.com/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_step
---

# Animation.step(Object object)
表示一组动画完成。可以在一组动画中调用任意多个动画方法，一组动画中的所有动画会同时开始，一组动画完成后才会进行下一组动画。
  
    
  ## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/animation/animation" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |

  
    
## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| duration | number | 否 | 400 | 动画持续时间，单位 ms |
| timingFunction | string | 否 | linear | 动画的效果<br>**可选值**：<br>- `linear`：动画从头到尾的速度是相同的<br>- `ease`：动画以低速开始，然后加快，在结束前变慢<br>- `ease-in`：动画以低速开始<br>- `ease-in-out`：动画以低速开始和结束<br>- `ease-out`：动画以低速结束<br>- `step-start`：动画第一帧就跳至结束状态直到结束<br>- `step-end`：动画一直保持开始状态，最后一帧跳到结束状态 |
| delay | number | 否 | 0 | 动画延迟时间，单位 ms |
| transformOrigin | string | 否 | 50% 50% 0 | 元素变形的原点 |

  
    
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

  animation.background('#FFFFFF').step({ duration: 300, timingFunction: 'linear', delay: 0 });
  ```
