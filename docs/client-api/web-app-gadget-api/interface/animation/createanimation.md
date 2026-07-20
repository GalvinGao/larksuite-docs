---
document_id: '6965379543683088390'
directory_id: '6907567266537406465'
title: createAnimation
full_path: /uYjL24iN/uETNy4SM1IjLxUjM
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Animation
- createAnimation
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:26Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETNy4SM1IjLxUjM
---

# `createAnimation`

创建一个动画实例`animation`。可以通过**链式调用**实例方法来描述动画，最后通过`step`和`export`方法导出动画数据，传递给组件的`animation`属性。

::: note
实现的是一个CSS动画，具体效果依赖浏览器的实现。
:::

## 输入

名称 | 数据类型 | 属性 | 默认值 | 描述
--|--|--|--|--
`duration` | `number` | optional | `400` | 指定一个动画周期的时长，单位 `ms`
`timingFunction` | `string` | optional | `linear` | 定义动画在每一动画周期中执行的节奏，[参考文档](https://developer.mozilla.org/en-US/docs/Web/CSS/animation-timing-function)
`delay` | `number` | optional | `0` | 定义动画于何时开始
`transformOrigin` | `string` | optional | `50% 50% 0` | 指定元素变形的原点

## 输出

返回一个**animation**对象，该对象具有如下方法：

### 样式
可以通过以下方法控制 CSS 属性，参数值与CSS定义相同，可以参考对应文档。

- `opacity`
- `backgroundColor`
- `width`
- `height`
- `top`
- `right`
- `bottom`
- `left`

### 变换

变换方法名和参数与 CSS 标准定义值相同，可以参考[对应文档](https://developer.mozilla.org/zh-CN/docs/Web/CSS/transform-function)。

#### 旋转

- `rotate`
- `rotateX`
- `rotateY`
- `rotateZ`
- `rotate3d`

#### 缩放

- `scale`
- `scaleX`
- `scaleY`
- `scaleZ`
- `scale3d`

#### 偏移

- `translate`
- `translateX`
- `translateY`
- `translateZ`
- `translate3d`

#### 倾斜

- `skew`
- `skewX`
- `skewY`

#### 矩阵变形

- `matrix`
- `matrix3d`

### `step`

调用`step`方法来标示一个「动画组」。一组动画内的所有视觉变换会同时发生，同时结束。
可以给动画组添加和创建动画时相同类型的属性来实现自定义动画组。如不指定则沿用创建动画时指定的属性值。

### `export`

调用`export`方法会把「动画组」导出为可以传递给 animation 属性应用的数据结构。

::: note
每次 `export` 只会导出「尚未被导出」的动画组，若某动画组已经被导出过，则会被清除。
如果在调用 `export` 时存在尚未完成的「动画组」，则未进入「动画组」的视觉变换不会生效（但也不会被删除，下次调用 `step` 方法后会继续生效）。
:::


## 代码示例

``` html
<view animation="{{animationData}}"></view>
```

``` js
Page({
    animate () {  
        if (!this.animation) {
            console.log('createNewAnimation');
            // 创建一个默认动画组执行时间为1秒的动画
            var animation = tt.createAnimation({
                duration: 1000,
                timingFunction: 'cubic-bezier(0.1, 0.7, 1.0, 0.1)'
            });
            this.animation = animation
        }

        // 创建一个动画组，使用默认设置
        this.animation
            .backgroundColor('blue')
            .step();

        // 应用第1个动画组
        this.setData({
            animationData: this.animation.export()
        });

        // 创建第2个动画组，修改动画执行时间为5秒
        // 可以分步骤创建动画组
        this.animation
            .backgroundColor('green');
        this.animation
            .skewX(30)
            .step({
                duration: 5000
            });

        // 没有被动画组包裹的视觉修改不会被导出
        this.animation
            .rotateY(60)
            .opacity(0.3)
            .width(200);

        // 此时应用已经创建好的第2个动画组
        // 注意：这时第1个动画可能还没有执行完毕，导出应用可能出现不如预期的效果
        //      建议确保分别导出的前序动画执行完毕再应用下一个导出
        //      在同一个导出里的动画组会确保顺序执行
        this.setData({
            animationData: this.animation.export()
        });

        // 此时才创建了第3个动画组
        // 如果整个方法再次被调用时，第一次导出会将其应用
        this.animation.step({
            timingFunction: 'step-start'
        });
    }
});
```
