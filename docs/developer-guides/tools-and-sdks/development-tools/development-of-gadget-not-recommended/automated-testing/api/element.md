---
document_id: '6967331158355460102'
directory_id: '6907567269107826690'
title: Element
full_path: /uYjL24iN/uITM4YjLyEDO24iMxgjN
breadcrumb:
- Developer Guides
- Tools and SDKs
- Development Tools
- Development of Gadget (Not Recommended)
- Automated Testing
- API
- Element
document_type: GuideDocumentType
updated_at: 2022-11-17T05:56:58Z
source_url: https://open.larksuite.com/document/uYjL24iN/uITM4YjLyEDO24iMxgjN
---

# Element
Element 模块提供了控制小程序页面元素的方法。
### 属性
#### element.elementOptions
```
interface ElementOptions {
    elementId: string
    tagName: string
    nodeId?: string
    pageId: string
    videoId?: string
}
```
### 方法
#### element.$
在元素范围内获取元素。
```
element.$(selector: string): Promise<Element>
```
**参数说明**
|字段|	类型|	必填	|默认值|	说明|
|---|---|---|---|---|
|selector|	string	|是|	-	|选择器|
> 同 ```TTML -> createSelectorQuery -> select```，仅支持部分 CSS 选择器， [参考文档](/document/uYjL24iN/uYjN24iN2YjL2YjN) 
  
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  const page = await miniProgram.currentPage()
  let element = await page.$('.index-hd')
  element = await element.$('.index-desc')
  console.log(await element.text())
})
```
#### element.$$
在元素范围内获取元素数组。
```
element.$$(selector: string): Promise<Element[]>
```
**参数说明**
|字段|	类型|	必填	|默认值|	说明|
|---|---|---|---|---|
|selector|	string	|是|	-	|选择器|
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  const page = await miniProgram.currentPage()
  const element = await page.$('.index-bd')
  const elements = await element.$$('.kind-list-text')
  console.log(await elements[0].text())
})
```
#### element.size
获取元素大小。
```
element.size(): Promise<Object>
```
**返回值说明**
|字段|	类型|	说明	|
|---|---|---|
|width|	number	|元素宽度|
|height|	number	|元素高度|
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  const page = await miniProgram.currentPage()
  const element = await page.$('.index-bd')
  const { width, height } = await element.size()
  console.log(width, height)
})
```
#### element.offset
获取元素绝对位置。
```
element.offset(): Promise<Object>
```
**返回值说明**
|字段|	类型|	说明	|
|---|---|---|
|left	|number|	左上角 x 坐标，单位：px|
|top|	number	|左上角 y 坐标，单位：px|
  
坐标信息以页面左上角为原点。
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  const page = await miniProgram.currentPage()
  const element = await page.$('.index-bd')
  const { left top } = await element.offset()
  console.log(left, top)
})
```
#### element.text
获取元素文本。
```
element.text(): Promise<string>
```
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  const page = await miniProgram.currentPage()
  const element = await page.$('.index-desc')
  console.log(await element.text())
})
```
#### element.attribute
获取元素特性。
```
element.attribute(name: string): Promise<string>
```
**参数说明**
|字段|	类型|	必填	|默认值|	说明|
|---|---|---|---|---|
|name	|string	|是	|-	|特性名|
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  const page = await miniProgram.currentPage()
  const element = await page.$('.index-logo')
  console.log(await element.attribute('src')) // -> 'resources/kind/logo.png'
})
```
#### element.property
获取元素属性。
```
element.property(name: string): Promise<any>
```
**参数说明**
|字段|	类型|	必填	|默认值|	说明|
|---|---|---|---|---|
|name	|string	|是	|-	|属性名|
element.property 与 element.attribute 主要区别如下：
* element.attribute 获取的是标签上的值，因此它的返回类型一定是字符串，element.property 则不一定。
* element.attribute 可以获取到 class 和 id 之类的值，element.property 不行。
* element.property 可以获取到文档里对应组件列举的大部分属性值，比如表单 input 等组件的 value 值。
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  const page = await miniProgram.currentPage()
  const element = await page.$('input')
  console.log(await element.property('value'))
})
```
#### element.value
获取元素值。
```
element.value(): Promise<string>
```
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  const page = await miniProgram.currentPage()
  const element = await page.$('.weui-input')
  console.log(await element.value())
})
```
#### element.style
> sdk 1.0.2，基础库 1.9.37.1 开始支持。

获取元素样式值。
```
element.style(name: string): Promise<string>
```
**参数说明**
|字段|	类型|	必填	|默认值|	说明|
|---|---|---|---|---|
|name	|string	|是	|-	|样式名|
  
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  const page = await miniProgram.currentPage()
  const element = await page.$('.index-desc')
  console.log(await element.style('color')) // -> 'rgb(136, 136, 136)'
})
```
#### element.tap
点击元素。
```
element.tap(): Promise<void>
```
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  const page = await miniProgram.currentPage()
  const element = await page.$('.kind-list-item-hd')
  await element.tap()
 })
```
#### element.longpress
长按元素。
```
element.longpress(): Promise<void>
```
#### element.touchstart
手指开始触摸元素。
```
element.touchstart(options: Object): Promise<void>
```
options 字段定义如下：
|字段|	类型|	必填	|默认值|	说明|
|---|---|---|---|---|
|touches	|array<Touch>	|是	|-	|触摸事件，当前停留在屏幕中的触摸点信息的数组|
|changedTouches	|array<Touch>	|是	|-	|触摸事件，当前变化的触摸点信息的数组|
  
#### element.touchmove
手指触摸元素后移动。
```
element.touchmove(options: Object): Promise<void>
```
options 字段同 touchstart。
#### element.touchend
手指结束触摸元素。
```
element.touchend(options: Object): Promise<void>
```
options 字段同 touchstart。
```
automator.launch(launchOptions).then(async miniProgram => {
  const page = await miniProgram.currentPage()
  const element = await page.$('.touch')
  await element.touchstart({
    touches: [
      {
        identifier: 1,
        pageX: 500,
        pageY: 500
      }
    ],
    changedTouches: [
      {
        identifier: 1,
        pageX: 500,
        pageY: 500
      }
    ]
  })
  await element.touchend({
    touches: [],
    changedTouches: [
      {
        identifier: 1,
        pageX: 500,
        pageY: 500
      }
    ]
  })
})
```
#### element.trigger
触发元素事件。
```
element.trigger(type: string, detail?: Object): Promise<void>
```
**参数说明**
|字段|	类型|	必填	|默认值|	说明|
|---|---|---|---|---|
|type|	string	|是|	-|	触发事件类型|
|detail	|Object	|否	|-	|触发事件时传递的 detail 值|
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  const page = await miniProgram.currentPage()
  const element = await page.$('picker')
  await element.trigger('change', { value: 1 })
})
```
> 该方法无法改变组件状态，仅触发响应方法，也无法触发用户操作事件，即 tap，longpress 等事件，请使用对应的其它方法调用。
#### element.input
输入文本，仅 input、textarea 组件可以使用。
```
element.input(value: string): Promise<void>
```
**参数说明**
|字段|	类型|	必填	|默认值|	说明|
|---|---|---|---|---|
|value	|string|	是|	-	|需要输入的文本|
  
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  const page = await miniProgram.currentPage()
  const element = await page.$('input')
  await element.input('test')
})
```
#### element.callMethod
调用组件实例指定方法，仅自定义组件可以使用。
```
element.callMethod(method: string, ...args: any[]): Promise<any>
```
**参数说明**
|字段|	类型|	必填	|默认值|	说明|
|---|---|---|---|---|
|method|	string|	是|	-	|需要调用的方法名|
|...args	|array<any>	|否	|-	|方法参数|
  
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  const page = await miniProgram.currentPage()
  const element = await page.$('set-tab-bar')
  await element.callMethod('navigateBack')
})
```
#### element.data
获取组件实例渲染数据，仅自定义组件可以使用。
```
element.data(path?: string): Promise<Object>
```
**参数说明**
|字段|	类型|	必填	|默认值|	说明|
|---|---|---|---|---|
|path	|string	|否|-	|数据路径|
  
  
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  const page = await miniProgram.currentPage()
  const element = await page.$('set-tab-bar')
  console.log(await element.data('hasSetTabBarBadge'))
})
```
#### element.setData
设置组件实例渲染数据，仅自定义组件可以使用。
```
element.setData(data: Object): Promise<void>
```
**参数说明**
|字段|	类型|	必填	|默认值|	说明|
|---|---|---|---|---|
|data|	Object	|是	|-	|要改变的数据|
  
  
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  const page = await miniProgram.currentPage()
  const element = await page.$('set-tab-bar')
  await page.setData({
    hasSetTabBarBadge: true
  })
})
```
#### element.contentSize
获取滚动宽高，仅 scroll-view 组件可以使用。
```
element.scrollWidth(): Promise<Size>
```
Size
|字段|	类型|	说明	|
|---|---|---|
|width|	number	|宽度|
|height|	number	|高度|
#### element.scrollTo
滚动到指定位置，仅 scroll-view 组件可以使用。
```
element.scrollTo(x: number, y: number): Promise<void>
```
**参数说明**
|字段|	类型|	必填	|默认值|	说明|
|---|---|---|---|---|
x|	number|	是	|-|	横向滚动位置
y	|number	|是|	-|	纵向滚动位置
  
  
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  const page = await miniProgram.currentPage()
  const element = await page.$('scroll-view')
  const y = (await element.scrollHeight()) - 50
  await element.scrollTo(0, y)
})
```
#### element.swipeTo
滑动到指定滑块，仅 swiper 组件可以使用。
```
element.swipeTo(index: number): Promise<void>
```
**参数说明**
|字段|	类型|	必填	|默认值|	说明|
|---|---|---|---|---|
index|	number|	是|	-	|目标滑块的 index
  
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  const page = await miniProgram.currentPage()
  const element = await page.$('swiper')
  await element.swipeTo(2)
})
```
#### element.slideTo
滑动到指定数值，仅 slider 组件可以使用。
```
element.slideTo(value: number): Promise<void>
```
**参数说明**
|字段|	类型|	必填	|默认值|	说明|
|---|---|---|---|---|
value|	number|	是|	-	|
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  const page = await miniProgram.currentPage()
  const element = await page.$('slider')
  await element.slideTo(10)
})
```
