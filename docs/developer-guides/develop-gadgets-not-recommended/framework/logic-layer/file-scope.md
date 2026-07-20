---
document_id: '6965379567070511109'
directory_id: '6907567266536996865'
title: 模块化
full_path: /uYjL24iN/uYDNuYDNuYDN
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Framework
- Logic Layer
- File scope
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:28Z
source_url: https://open.larksuite.com/document/uYjL24iN/uYDNuYDNuYDN
---

# 模块化

## 模块化

在开发过程中，可以将一些公共的代码抽离成为一个单独的 js 文件作为一个模块。模块只有通过 **`module.exports`** 或者 **`exports`** 才能对外暴露接口。

**需要注意的是：**

* `exports` 是 `module.exports` 的一个引用，因此在模块里边随意更改 `exports` 的指向会造成未知的错误。所以更推荐开发者采用 `module.exports` 来暴露模块接口，除非你已经清晰知道这两者的关系。

```js
// common.js
function hello(name) {
  console.log(`Hello ${name}!`);
}
function goodbye(name) {
  console.log(`Goodbye ${name}!`);
}

module.exports.hello = hello;
exports.goodbye = goodbye;
```

在需要引用这些模块的文件中，使用 `require(path)` 将公共代码引入：
- 如果该文件是自己本地项目下的模块，那么 path 需要为相对路径。
```js
// index.js 
const common = require('./common.js') // common.js 在与 index.js 同级 的目录下
Page({
  helloWorld: function() {
    common.hello('world')
  },
  goodbyeWorld: function() {
    common.goodbye('world')
  }
})
```
- 如果引用了 node_modules 里的包，那么 path 为包的名称即可，无需写相对路径。使用前需要开启 npm 能力，详细见 [npm 支持](/document/uYjL24iN/uEzMzUjLxMzM14SMzMTN/npm-support)。
```js
const sm2 = require('miniprogram-sm-crypto').sm2 // miniprogram-sm-crypto 为三方包
const { publicKey, privateKey } = sm2.generateKeyPairHex();
Page({
  data: {
    publicKey: publicKey,
    privateKey: privateKey
  }
})
```


## 文件作用域
在 JavaScript 文件中声明的变量和函数只在该文件中有效；不同的文件中可以声明相同名字的变量和函数，不会互相影响。<br>

如果需要全局访问某些信息，那么可以将这些信息在 `App()` 中进行设置，然后
在需要的时候通过函数 `getApp()` 获取全局的应用实例以获得挂载在它上面的信息，如：
```javascript
// app.js
App({
  globalData: 1
})
```
```javascript
// a.js
const app = getApp();
app.globalData++;
```

```javascript
// b.js 
// 如果 a.js 已经在 b.js 前加载过，那么此时 globalData 应该为 2
console.log(getApp().globalData);
```









