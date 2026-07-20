---
document_id: '7073692582769147910'
directory_id: '7072701762536538117'
title: SJS 运算符
full_path: /uYjL24iN/uEjMuEjMuEjM/sjs/sjs-operator
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Framework
- UI Layer
- SJS
- SJS Operator
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:31Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEjMuEjMuEjM/sjs/sjs-operator
---

# SJS 运算符

## 算术运算符

```javascript
var a = 1;
var b = 1;

a + b; // 加法运算符
a - b; // 减法运算符
a * b; // 乘法运算符
a / b; // 除法运算符
a % b; // 取模运算符
```

## 比较运算符

```javascript
var a = 1;
var b = 2;

a > b;
a < b;
a >= b;
a <= b;
a != b;
a == b;
a !== b;
a === b;
```

## 二元逻辑运算符

```javascript
var a = 1;
var b = 0;

a && b;
a || b;
!b;
```

## 位运算符

```javascript
var a = 2;
var b = 4;

a & b;
a | b;
a ^ b;

a >> 1;
b << 1;
b >>> 1;
```

## 赋值运算符

```javascript
var a = 2;

a += 10;
a -= 10;
a *= 1;
a /= 1;
a %= 2;

a <<= 1;
a >>= 1;
a >>>= 1;

a &= 1;
a |= 1;
a ^= 1;
```

## 一元运算符

```javascript
var a = 1;

a++;
++a;

a--;
--a;

+a;
-a;

~a;

delete a; // delete 运算
void 0; // void 运算
typeof a; // typeof 运算
```

## 三元运算符

```javascript
var a = 2;
var b = a > 1 ? "hello" : "bye";
```

## 逗号运算符

```javascript
var a = (1, "hello");
```

## tips

- 以上运算符的优先级同 JavaScript 一致
