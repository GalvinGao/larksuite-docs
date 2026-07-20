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
                duration
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                400
            </md-td>
            <md-td>
                动画持续时间，单位 ms

            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                timingFunction
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                linear
            </md-td>
            <md-td>
                动画的效果

**可选值**：
- `linear`：动画从头到尾的速度是相同的
- `ease`：动画以低速开始，然后加快，在结束前变慢
- `ease-in`：动画以低速开始
- `ease-in-out`：动画以低速开始和结束
- `ease-out`：动画以低速结束
- `step-start`：动画第一帧就跳至结束状态直到结束
- `step-end`：动画一直保持开始状态，最后一帧跳到结束状态
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                delay
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>0</md-td>
            <md-td>
                动画延迟时间，单位 ms
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                transformOrigin
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                50% 50% 0
            </md-td>
            <md-td>
                元素变形的原点
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

  animation.background('#FFFFFF').step({ duration: 300, timingFunction: 'linear', delay: 0 });
  ```
