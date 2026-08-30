---
title: Markdown 语法
---

# Markdown 语法

euv-docs 在构建期渲染 VuePress 风格的 markdown 子集。

## 标题

`h2` 与 `h3` 会自动出现在右侧锚点目录。`h1`、`h4`、`h5`、`h6` 渲染为普通标题,不进目录。

### h3 子标题

#### h4 子子标题

##### h5 子子子标题

###### h6 最深一级标题

## 行内格式

支持 **加粗**、*斜体*、~~删除线~~ 与 `行内代码`。

加粗和斜体可以嵌套:***加粗斜体***、**带 `行内代码` 的加粗**,以及
*混合 `代码` 与强调*。

内联链接也能嵌套:**[快速上手](./getting-started.md)** 与
*[euv 仓库外链](https://github.com/euv-dev/euv)*。

脚注引用以 `[^example]` 形式内联,渲染为纯文本 `[^example]`(不自动编号,也不生成脚注列表)。

## 硬换行 vs 软换行

这段文字末尾有两个尾随空格,所以下一行  
虽然在同一个段落里,但会另起一行(硬换行,`<br>`)。

这段文字末尾没有尾随空格,所以下一行
会与这一段合并为同一段(软换行,只插入一个空格)。

## 列表

- 无序项
- 另一项
  - 嵌套无序项
  - 另一嵌套项
    1. 无序下的嵌套有序
    2. 第二条嵌套有序
  - 回到无序

1. 有序项
2. 第二项
3. 第三项带一个[链接](./getting-started.md)

- [x] 已完成任务
- [ ] 待办任务
- [x] 已完成任务带 `行内代码`

## 表格

| 特性 | VuePress | euv-docs | 备注 |
| :--- | :---: | ---: | --- |
| Frontmatter | ✅ | ✅ | 构建期解析 |
| 自定义容器 | ✅ | ✅ | `::: tip / warning / danger / note` |
| 锚点目录 | ✅ | ✅ | 仅 h2 + h3 |
| 表格对齐 | ✅ | ✅ | 左 / 中 / 右 |

## 代码块

带语言标签的围栏代码块会在外壳上挂语法类:

```rust
fn fib(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fib(n - 1) + fib(n - 2),
    }
}
```

```bash
euv build --release -- --target web --out-dir www/pkg
```

```toml
[site]
title = "euv-docs"

[[locales]]
prefix = "/"
lang = "en-US"
label = "English"
```

```text
纯文本围栏,无语法类
```

```markdown
markdown 围栏,无语法类
```

缩进代码块(4 空格缩进)也支持,但建议用围栏形式,因为语言标签会驱动外壳类。

    // 4 空格缩进块渲染为代码
    fn main() {}

## 自定义容器

::: tip
提示容器使用实线边框。
:::

::: warning
警告容器使用虚线边框。
:::

::: danger
危险容器使用双线边框。
:::

::: info
支持的完整类型集合为 `tip`、`warning`、`danger`、`info`、`note`、`details`(在 euv-ui 容器 CSS 中定义)。
:::

::: note 自定义标题
容器支持在类型后写自定义标题。
:::

::: details 点击展开
可展开块中渲染的隐藏内容。
:::

## 引用块

> 文档是写给未来自己的情书。

嵌套引用块:

> 第一层。
>
> > 第二层。
> >
> > 里面也能用 **加粗** 与 `代码`。
>
> 回到第一层。

## 分割线

三个或更多 `-`、`_`、`*` 各成一行:

---

***

___

## 图片

![euv logo](https://euv-dev.github.io/euv-docs/favicon.svg)

图片保留 alt 文本与源 URL 渲染。

## 链接

- [指南首页的内部链接](./README.md)
- [euv 仓库的外链](https://github.com/euv-dev/euv)
- [跳转到表格](#表格)
- [邮件链接](mailto:nobody@example.com)
- [带标题的链接](https://github.com/euv-dev/euv "GitHub 上的 euv 仓库")

## 转义字符

反斜杠转义下一个字符: \*字面星号\*、\[不是链接\]、\<不是标签\>、\`不是代码\`。反斜杠被消耗,字符以纯文本渲染。

## 内联与块级 HTML

段落内允许嵌入一段内联 HTML:

这段文字里有 <sup>上标</sup> 与 <sub>下标</sub>
通过原生 HTML 标签嵌入。

一段原生 HTML 块按字面渲染:

<div style="border: 1px dashed currentColor; padding: 8px 12px;">
  此块作为原生 HTML 渲染,不走 markdown 解析。
</div>
