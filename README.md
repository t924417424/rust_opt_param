# 在Rust中用更好的方式设置私有字段的参数

## 项目基于Rust闭包实现，让你可以通过如下形式链式调用(因为Rust没有默认的可变参数实现，如有需求可将相关setter方法放入vec中，也可实现功能)`configure`方法来设置私有字段参数

```rust
fn main() {
    let mut app = Application::default();
    println!("{:?}", app);
    // output: Application { name: "", version: V1 }
    app.configure(
        Application::set_name("set by opt")
    )
        .configure(
            Application::set_version(version::Version::V2)
        );
    println!("{:?}", app);
    // output: Application { name: "set by opt", version: V2 }
}

```

## 优点
- 可有效保护（限制）参数值
- 相比于直接通过`.field`设置字段值更优雅