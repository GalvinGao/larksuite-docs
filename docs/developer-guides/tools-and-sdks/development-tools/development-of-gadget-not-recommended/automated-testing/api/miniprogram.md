---
document_id: '6967331158354935814'
directory_id: '6907567269107826690'
title: MiniProgram
full_path: /uYjL24iN/uATM4YjLwEDO24CMxgjN
breadcrumb:
- Developer Guides
- Tools and SDKs
- Development Tools
- Development of Gadget (Not Recommended)
- Automated Testing
- API
- MiniProgram
document_type: GuideDocumentType
updated_at: 2022-11-17T05:56:52Z
source_url: https://open.larksuite.com/document/uYjL24iN/uATM4YjLwEDO24CMxgjN
---

# MiniProgram
MiniProgram 模块提供了控制小程序的方法。
### 方法
#### miniProgram.pageStack
获取小程序页面堆栈。
```
miniProgram.pageStack(): Promise<Page[]> 
```
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  const pageStack = await miniProgram.pageStack()
  console.log(pageStack.length) // 当前页面栈数量
  })
```
#### miniProgram.navigateTo
跳转到指定页面。跳转后原页面保留，同 [tt.navigateTo](/document/uYjL24iN/uYTOz4iN5MjL2kzM)。
```
miniProgram.navigateTo(path: string): Promise<Page>
```
**参数说明**

|字段|	类型|	必填	|默认值|	说明|
|---|---|---|---|---|
|path|	string|	是|	-	|需要跳转的应用内非 tabBar 的页面的路径|


示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  const page = await miniProgram.navigateTo('/page/component/index')
  console.log(page.pageOptions.path) // -> 'page/component/index'
  })
```
#### miniProgram.redirectTo
关闭当前页面，跳转到指定页面，同 [tt.redirectTo](/document/uYjL24iN/ucTOz4yN5MjL3kzM)。
```
miniProgram.redirectTo(path: string): Promise<Page>
```
**参数说明**
|字段|	类型|	必填	|默认值|	说明|
|---|---|---|---|---|
|path|	string|	是	|-|	需要跳转的应用内非 tabBar 的页面的路径|

#### miniProgram.navigateBack
返回上一级页面（或上N级页面），同 [tt.navigateBack](/document/uYjL24iN/uADM04CMwQjLwADN)。
```
miniProgram.navigateBack(delta: number): Promise<Page>
```
|字段|	类型|	必填	|默认值|	说明|
|---|---|---|---|---|
|delta	|number|	否|	1	|需要跳转的应用内页面路径|

#### miniProgram.relaunch
关闭所有当前页面，打开指定页面，同 [tt.reLaunch](/document/uYjL24iN/uEDM04SMwQjLxADN)。
```
miniProgram.relaunch(url: string): Promise<Page>
```
**参数说明**
|字段|	类型|	必填	|默认值|	说明|
|---|---|---|---|---|
|url	|string	|是	|-	|需要跳转的应用内页面路径|

#### miniProgram.switchTab
跳转到指定 TabBar 页面，并关闭其他所有非 TabBar 页面，同 [tt.switchTab](/document/uYjL24iN/ukTOz4SO5MjL5kzM)。
```
miniProgram.switchTab(url: string): Promise<Page>
```
**参数说明**
|字段|	类型|	必填	|默认值|	说明|
|---|---|---|---|---|
|url|	string	|是|	-	|需要跳转的 tabBar 页面的路径|


#### miniProgram.currentPage
获取当前页面。
```
miniProgram.currentPage(): Promise<Page>
```
#### miniProgram.systemInfo
获取系统信息，同 [tt.getSystemInfo](/document/uYjL24iN/uQjNx4CN2EjL0YTM)。
```
miniProgram.systemInfo(): Promise<Object>
```
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  const systemInfo = await miniProgram.systemInfo()
  if (systemInfo.screenWidth === 640) {
    // Do something
  }})
```
#### miniProgram.callTTMethod
> 基础库版本 1.9.37.1 及以上版本

调用 tt 对象上的指定方法。
```
miniProgram.callTTMethod(method: string, ...args: any[]): Promise<any>
```
**参数说明**

|字段|	类型|	必填	|默认值|	说明|
|---|---|---|---|---|
|method|	string	|是	|-	|需要调用的方法名|
|...args	|array<any>|	否|	-	|方法参数|


调用异步方法时无需传入 success 及 fail 回调函数。
  
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  await miniProgram.callTTMethod('setStorage', {
    key: 'test',
    data: 'test'
  })
  const { data } = await miniProgram.callTTMethod('getStorageSync', 'test')
  console.log(data) // -> 'test'
  })
```
#### miniProgram.mockTTMethod
> 开发者工具版本  1.9.0 及以上，基础库版本 1.9.37.1 及以上
 
覆盖 tt 对象上指定方法的调用结果。
利用该接口，你可以很方便地直接指定  ` tt.chooseLocation ` 等调用系统组件的返回结果。
```
mockTTMethod(method: string, result: string | Function | any, ...args: any[]): Promise<void>
```
**参数说明**
|字段|	类型|	必填	|默认值|	说明|
|---|---|---|---|---|
|method	|string|	是|	-	|需要覆盖的方法名|
|result	|any|	是|	-	|指定调用结果或一个处理返回方法|
|...args|array<any>|	否|	-	|作为 result （ 处理返回方法）参数 |
  
  
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  await miniProgram.mockTTMethod('showModal', {
    confirm: true,
    cancel: false
  })
  
  // 1. 单参数（异步方法） mock
    const mockedMethod = function(param: any, platform: string) {
        return new Promise(resolve => {
            this.origin({
                success(res: any) {
                    res.platform = platform
                    resolve(res)
                }
            })
        })
    }
    minigram.mockTTMethod('getSystemInfo', mockedMethod, 'test')
  // 2. 多参数（同步方法） mock
    const mockedMethods = function(test0: string, test1: string, test3: string, test4: string) {
        return this.origin(test3, test4)
    }
    minigram.mockTTMethod('setStorageSync', mockedMethods, 'test-key', 'test-value' )
```
#### miniProgram.restoreTTMethod
重置 tt 指定方法，消除 mockTTMethod 调用的影响。
```
miniProgram.restoreTTMethod(method: string): Promise<void>
```
**参数说明**
|字段|	类型|	必填	|默认值|	说明|
|---|---|---|---|---|
|method	|string|	是|	-	|需要覆盖的方法名|
  
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  console.log(await miniProgram.callTTMethod('getStorageSync', 'test')) // -> ''
  await miniProgram.mockTTMethod('getStorageSync', 'mockValue')
  console.log(await miniProgram.callTTMethod('getStorageSync', 'test')) // -> 'mockValue'
  await miniProgram.restoreTTMethod('getStorageSync')
  console.log(await miniProgram.callTTMethod('getStorageSync', 'test')) // -> ''
  })
```
#### miniProgram.pageScrollTo
滚动页面到目标位置，同 [tt.pageScrollTo](/document/uYjL24iN/uITNy4iM1IjLyUjM)。
```
miniProgram.pageScrollTo(scrollTop: number): Promise<void>
```
**参数说明**
|字段|	类型|	必填	|默认值|	说明|
|---|---|---|---|---|
|scrollTop	|number	|是|	-	|滚动到页面的目标位置，单位 px|
  
  
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  await miniProgram.pageScrollTo(50)})
```
#### miniProgram.disconnect
断开与小程序运行时的连接。
```
miniProgram.disconnect(): void
```
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  miniProgram.disconnect()})
```
#### miniProgram.close
断开与小程序运行时的连接并关闭项目窗口。
```
miniProgram.close(): Promise<void>
```
示例代码：
```
automator.launch(launchOptions).then(async miniProgram => {
  await miniProgram.close()})
```

