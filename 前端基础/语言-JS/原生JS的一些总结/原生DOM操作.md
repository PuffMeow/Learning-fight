### 查找元素

#### element.getElementById(idName: string)

通过id查找元素，返回元素DOM

#### element.getElementsByClassName(className: string)

通过class查找元素，返回一个类数组。页面上可能会有多个元素有同名class。如果要遍历返回的类数组就需要转换[...result] 或者 Array.from[result]，就能将类数组转换为数组

#### element.getElementsByTagName(tagName: string)

通过标签获取元素，类似于getElementsByClassName，返回一个类数组。比如获取页面上所有的div标签。

#### element.querySelector(selectors) | element.querySelectorAll(selectors)

使用选择器来查找元素api，比如查找某个有example类名的元素，element.querySelector(".example")

querySelector 获取单个元素，querySelectorAll 获取多个元素返回类数组结构

### DOM样式

#### 给元素增加样式

```js
element.style.width = xxx
```

#### 给元素增加class

```js
element.className='aaa'  // 设置元素的class为aaa ，如果元素上原本有class则会覆盖

element.classList.add("aaa") // 给element新增aaa

element.className += " aaa"  // 给element新增aaa
```

#### 判断元素上是否有某个属性

```js
element.classList.contains("aaa")  // 如果element上面的class类名是aaa返回true，否则返回false
```

### 操作DOM上的属性

#### 新增属性

```js
element.setAttribute("data-id", 1);
```

#### 删除属性

```js
element.removeAttribute("data-id");
```

### DOM元素的增删改查

#### 创建DOM元素

```js
const div = document.createElement("div");
```

#### 删除DOM元素

```
 element.remove(); // 删除element
 
 element.removeChild(clildEle) // 删除element中的子元素 childEle
```

#### 复制DOM

`element.cloneNode( true | false ) `，参数为是否进行深层复制(深克隆)

```js
const box = document.getElementsByClassName("box")[0];
const p = document.createElement("p");
p.innerText = "嘤嘤嘤";
const p2 = p.cloneNode(true); // 复制一个p  参数true标识深度复制，如果p里面有子节点也复制过来
box.appendChild(p);
box.appendChild(p2);
```

#### 插入DOM

```js
element.appendChild(element) 在某个element的最后插入要插入的element元素

element.insertBefore(newele,element) // 在element元素中的 element元素前插入 newele
```

#### 替换DOM

`parentEle.replaceChild(newEle,oldEle)`

在某个父元素下将其某个子元素用一个新的元素替换掉

#### 删除DOM

```
parentEle.removeChild(element) // 删除父元素parentEle中的element元素
```

#### 遍历DOM

- parentNode：查找指定节点的父节点

- nextSibling：查找指定节点的下一个节点

- previousSibling：查找指定节点的上一个节点

- firstChild：查找指定节点的第一个字节点

- lastChild：查找指定节点的最后一个字节点

- childElementCount：返回子元素的个数，不包括文本节点和注释

- firstElementChild：返回第一个子元素

- lastElementChild：返回最后一个子元素

- previousElementSibling：返回前一个相邻兄弟元素

- nextElementSibling：返回后一个相邻兄弟元素

### NodeType(12种)

文档、元素、属性以及 HTML 或 XML 文档的其他方面拥有不同的节点类型。

存在 12 种不同的节点类型，其中可能会有不同节点类型的子节点：

| 节点类型 | 描述                  | 子节点                                              |
| :------- | :-------------------- | :-------------------------------------------------- |
| 1        | Element               | 代表元素                                            |
| 2        | Attr                  | 代表属性                                            |
| 3        | Text                  | 代表元素或属性中的文本内容。                        |
| 4        | CDATASection          | 代表文档中的 CDATA 部分（不会由解析器解析的文本）。 |
| 5        | EntityReference       | 代表实体引用。                                      |
| 6        | Entity                | 代表实体。                                          |
| 7        | ProcessingInstruction | 代表处理指令。                                      |
| 8        | Comment               | 代表注释。                                          |
| 9        | Document              | 代表整个文档（DOM 树的根节点）。                    |
| 10       | DocumentType          | 向为文档定义的实体提供接口                          |
| 11       | DocumentFragment      | 代表轻量级的 Document 对象，能够容纳文档的某个部分  |
| 12       | Notation              | 代表 DTD 中声明的符号。                             |