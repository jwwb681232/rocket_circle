错误级别由分为两级，分别为：
* AError ：从controller 入口 处抛出的错误，此错需要实现responder，返回json给用户`需要设定为 AResult`
* BError ：从service、model抛出的错误，此错误禁止实现responder，只是供A错误捕获，然后向外（用户）返回`需要设定为 BResult`


伪代码
```rust
/****************************** AError *****************************/
enum AError{
    BusinessError(Value),//Value is Json::Value
    BError(BError),
}

// todo



/****************************** BError *****************************/
enum BError{
    IoError,
    Database,
    Serialize,
    //...
}


impl From<std::io::Error> for BError{
    
}

impl From<diesel::result::Error> for BError{
    
}

impl From<serde_json::Error> for BError{
    
}

```
