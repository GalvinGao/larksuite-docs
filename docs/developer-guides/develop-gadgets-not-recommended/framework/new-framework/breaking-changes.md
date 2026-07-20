---
document_id: '7175034130278596614'
directory_id: '7174665909235073030'
title: 不兼容改动
full_path: /uYjL24iN/uEjMuEjMuEjM/new-framework/breaking-changes
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Framework
- new-framework
- Breaking changes
document_type: GuideDocumentType
updated_at: 2022-12-12T05:27:04Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEjMuEjMuEjM/new-framework/breaking-changes
---

# 不兼容改动
在以下场景中，业务代码可能会因新框架的不兼容改动而出现异常。如果在开启新框架后进行预览测试时发现异常，可以按照以下说明进行排查和兼容。  

## selectComponent
selectComponent 是自定义组件的方法，但是基于 uni-app 开发的小程序可能为了方便地在 Vue 实例中用 this.selectComponent 的方式调用自定义组件的 selectComponent 方法，而将自定义组件实例上的 .selectComponent 方法赋到了 Vue 实例上。

```javascript 
var _default = _vue.default.extend({
  created: function created() {
	// ...
    if (this.$scope) {
      this.selectComponent = this.$scope.selectComponent;
    }
  },
  // ...
});
``` 
如上代码中 this 是 Vue 实例，而 this.$scope 是和这个 Vue 实例关联的、用 Component 方法注册后被创建的自定义组件实例。  

但这是一个明显的 this 误用的方式，因为自定义组件实例的 .selectComponent 里的逻辑是按照 this 是自定义组件实例来编写的，而此时将其赋给 Vue 实例，导致其被调用时 this 指向 Vue 实例，那么就会导致方法访问 this 上的某些属性不存在的情况发生，进而导致报错。

```shell 
TypeError: Cannot read property 'element' of undefined
``` 
旧框架上不会报错的原因是，旧框架的 selectComponent 只访问 this.\_\_webviewId\_\_ 和 this.\_\_nodeId\_\_，而刚好这两个属性在 Vue 实例上也有（而 Vue 实例上的这两个属性也是从自定义组件实例上取过来赋上去的），所以不会出现问题。   

而新框架的 selectComponent 的参数和返回值度没有变，但是内部实现改变了，会访问 this 实例上的其他属性，而这些属性在 Vue 实例上是不存在的，所以会报错。

### 兼容方式
如果一定要将 selectComponent 挂在 Vue 实例上，那么建议使用这样的封装方式让 this 正确指向自定义组件实例。
```javascript 
var _default = _vue.default.extend({
  created: function created() {
	// ...
     if (this.$scope) {
        this.selectComponent = function () {
            // this 是 uni-app 封装的 Vue 实例，而 this.$scope 是和 uni-app 封装的 Vue 实例关联的自定义组件实例
            // 而自定义组件实例上，this === this.$scope.$vm
            this.$scope.selectComponent.apply(this.$scope, arguments);
        }
    }
  },
  // ...
});
``` 
或者也可以改为直接调用 this.$scope.selectComponent。

```javascript 
this.$scope.selectComponent('#user-avatar', (component) => {
	console.log('获取到 user-avatar 自定义组件', component);
});
``` 

## dataset
TTML 上的 dataset 属性写作 data-instanceId 这样的驼峰式写法时，该节点的 dataset 会被转成 currentTarget.dataset.instanceId 而不是 currentTarget.dataset.instanceid

```html
<view data-instanceId="12345" bindtap="onTap"></view>
```

```javascript
Page({
    onTap(e){
        // 旧框架下是 .instanceid
        console.log(e.target.dataset.instanceid)
        console.log(e.currentTarget.dataset.instanceid)
        // 新框架下是 .instanceId
        console.log(e.target.dataset.instanceId)
        console.log(e.currentTarget.dataset.instanceId)
    }
})
```

### 兼容方式
建议业务侧在使用 data- 属性时，维持良好的规范：
- 要么是在 TTML 上用 instance-id 这样的短线连接的形式，然后在事件回调函数中以 dataset.instanceId 的方式读取。
- 要么是在 TTML 上直接就以全部小写无短线连接的形式如 instanceid，然后在事件回调函数中以 dataset.instanceid 的方式读取。
以上两种写法都是新旧框架都兼容的写法。  

## 自定义组件作为 slot 插入
自定义组件可以在自己的 TTML 中声明 slot。
```html
<!-- user-card -->
<view class="title"></view>
<view class="content">
    <slot></slot>
</view>
```
自定义组件的直接子节点会作为插入到 slot 的节点插入到 TTML 中 slot 声明的位置。
```html
<user-card>
    <view class="wrapper">
        <view>卡片内容</view>
    </view>
</user-card>

<!-- user-card -->
<view class="title"></view>
<view class="content">
    <!-- user-card -->
    <view class="wrapper">
        <view>卡片内容</view>
    </view>
</view>
```
插入到 slot 的节点可以是一个自定义组件。
```html
<user-card>
    <card-content></card-content>
</user-card>
```
如果 slot 用 tt:if 控制存在与否，那么当 showSlot 从 true 变为 false 时，插入到 slot 的组件的 detached 会触发；当 showSlot 从 false 变为 true 时，插入到 slot 的组件的 attached 会触发。
```html
<!-- user-card -->
<view class="title"></view>
<view class="content">
    <slot tt:if="{{showSlot}}"></slot>
</view>
```

```javascript
// card-content
const set = new Set();
Component({
    attached() {
        if (set.has(this)) {
            console.log('attahced, already exist')
        } else {
            console.log('attahced, not exist before')
            set.add(this);
        }
    }
    detached() {
        console.log('detached')
    }
})
```
假设 user-card 的 showSlot 的初始值是 true，然后在 attached 中先置为 false 再置为 true。即将插入到 slot 的节点先移除再插入。那么在旧框架下，认为创建和销毁的只是 slot 节点，而 slot 节点是一个自定义组件 TTML 里对外部插入到 slot 节点的引用。
```javascript
// user-card
Component({
    data: {
        showSlot: true
    },
    attached() {
        setTimeout(() => {
            this.setData({
                showSlot: false
            });
            setTimeout(() => {
                this.setData({
                    showSlot: true
                });
            }, 100);
        }, 100);
    }
    detached() {
        console.log('detached')
    }
})
```
所以在旧框架下 Component 实例会复用之前的实例，在上面的例子中会输出如下日志。
```shell
attahced, not exist before
detached
attahced, already exist
```
在新框架下每次都会创建新的 Component 实例，在上面的例子中会输出如下日志。

```shell 
attahced, not exist before
detached
attahced, not exist before
``` 
但是如果 tt:if 控制的不是 slot 节点而是插入到 slot 的节点，如下所示。
```html
<user-card>
    <card-content tt:if="{{showSlot}}"></card-content>
</user-card>
```

```javascript
Page({
    data: {
        showSlot: true
    },
    onLoad() {
        setTimeout(() => {
            this.setData({
                showSlot: false
            });
            setTimeout(() => {
                this.setData({
                    showSlot: true
                });
            }, 100);
        }, 100);
    }
});
```

那么在旧框架的设计里这就是销毁节点本身，而在新框架里也同样如此，在上面的例子中会输出如下日志。
```shell 
attahced, not exist before
detached
attahced, not exist before
``` 

### 兼容方式
一般情况下不需要兼容，因为此处的 breaking chanege 一般不会引起异常。  
在这里仅作为变更而陈列出来。

## observer
observer 是自定义组件属性变化时触发的回调函数。
```javascript
Component({
    properties: {
        name: {
            type: String,
            value: 'anonymous',
            observer: function (newValue, oldValue) {
                console.log('name changed', newValue, oldValue);
            }
        }
    },
    attached() {
        console.log('component attached');
    }
})
```
组件的 name 属性和页面的 name 数据在 TTML 模板上声明了绑定关系，因此页面的 name 数据的值会赋给 name 属性。
```html
<user-card name="{{name}}"></user-card>
```
因为 name 数据的初始值 'kate' 和组件的 name 属性的初始值 'anonymous' 不同，所以会在用页面数据进行首次渲染时触发一次 observer。
而 onLoad 生命周期触发的 next event loop（因为 setTimeout 第二个参数传入 0）后 setData 给 name 数据赋值的 'robin' 又和 'kate' 不同，所以会再次触发一次 observer。
```javascript
Page({
    data: {
        name: 'kate'
    },
    onLoad() {
        setTimeout(() => {
            this.setData({
                name: 'robin'
            });
        }, 0);
    }
})
```

在旧框架下只有第一次 observer 是在 attached 前触发的，后续 observer 的触发都在 attached 之后。在上面的例子中，旧框架下会输出如下日志。
```shell
name changed kate anonymous
attached
name changed robin kate
```
而在新框架下会有多次 observer 在 attached 前触发。在上面的例子中，新框架下会输出如下日志。
```shell
name changed kate anonymous
name changed robin kate
attached
```
并不是 observer 回调函数就一定在 attached 回调函数之前执行。
attached 是自定义组件插入到页面 DOM 节点树时触发的生命周期，如果有一次 setData 触发 observer 是在自定义组件已经渲染完的时候才触发的，那么它依然是在 attached 之后才执行。如果在上面例子的基础上增加一个 2000ms 后的 setData
```javascript
Page({
    data: {
        name: 'kate'
    },
    onLoad() {
        setTimeout(() => {
            this.setData({
                name: 'robin'
            });
        }, 0);
        setTimeout(() => {
            this.setData({
                name: 'celine'
            });
        }, 2000);        
    }
})
```

那么在新框架下会输出如下日志。
```shell
name changed kate anonymous
name changed robin kate
attached
name changed celine robin
```

observer 提前于 attached 执行，可能导致 observer 的逻辑中由于缺少 attached 逻辑中设置的一些变量值而报错或者不执行，如
```javscript
Component({
    properties: {
        name: {
            type: String,
            value: 'anonymous',
            observer: function (newValue, oldValue) {
                if (this._inited) {
                    this.updateName(newValue);
                }
            }
        }
    },
    methods: {
        updateName() {
        }
    },
    attached() {
        this._inited = true;
    }
})
```
在这样的例子下，虽然在旧框架也会错过第一次 observer 里的 updateName，但是由于组件首次渲染时被赋值的往往是一些无效的默认值，如 ''。所以即使错过对于实际业务表现一般也没有影响。
但是在新框架下会有多次有效值的赋值触发的 observer 里的 updateName 被错过，最终影响实际业务表现。

### 兼容方式
应该把 attached 和 observer 当成时序顺序没有关联的两个独立的回调函数：
- observer 就是自定义组件属性被赋值时才触发的回调函数。
- attached 就是自定义组件插入到页面 DOM 节点树时才触发的回调函数。
因此如果 observer 的实际逻辑执行依赖 attached 设置的变量，就应该在 attached 里对 observer 的逻辑进行补偿执行，如
```javascript
Component({
    properties: {
        name: {
            type: String,
            value: 'anonymous',
            observer: function (newValue, oldValue) {
                if (this._inited) {
                    this._nameToUpdate = undefined;
                    this.updateName(newValue);
                } else {
                    // 如果因为 this._inited 不为 true 而不能 update 就把值保存起来
                    this._nameToUpdate = newValue;
                }
            }
        }
    },
    methods: {
        updateName() {
        }
    },
    attached() {
        this._inited = true;
        // 如果有需要 update 的 name 就进行补偿更新
        if (typeof this._nameToUpdate === 'string') {
            this.updateName(this._nameToUpdate);
        }
    }
})
```
在 uni-app 中，应在调用 watch 方法监听 组件属性/props 变化时，传入 { immediate: true }，表示当监听建立时立刻执行一次 watch 注册的回调函数。
```javascript
export default defineComponent({
    name: 'container',
    // ...
    setup(props) {
        watch(() => props.create, (v) => {
            if (v === 'true') {
                // ... 更新逻辑
            }
        }, { immediate: true });
    }
})
```
如果不是用 watch 方法，而是用 watch 配置来声明对组件属性的监听，那么在配置中增加 { immediate: true }。
```javascript
export default Vue.extends({
    props: {
        unionId: String,
    },
    watch: {
        unionId: {
            handler() {
                // ...
            },
            immediate: true
        }
    }
})
```
