// use mac_address::MacAddress;
// use neon::prelude::*;

// // 定义一个名为 `get_mac_address` 的函数，该函数接收一个 `FunctionContext` 参数，并返回一个 `JsResult<JsString>` 类型的结果
// fn get_mac_address(mut cx: FunctionContext) -> JsResult<JsString> {
//     // 使用 `MacAddress::new()` 创建一个 `MacAddress` 类型的实例，并将其赋值给 `mac` 变量
//     let mac = MacAddress::new().unwrap();
//     // 将 `mac` 转换为字符串，并将其赋值给 `mac_string` 变量
//     let mac_string = mac.to_string();
//     // 通过调用 `cx.string` 方法，将 `mac_string` 转换为 `JsString` 类型的实例，并作为结果返回
//     Ok(cx.string(mac_string))
// }

// // 使用 `neon::main` 宏定义一个名为 `main` 的函数，该函数接收一个 `ModuleContext` 参数，并返回 `NeonResult<()>` 类型的结果
// #[neon::main]
// fn main(mut cx: ModuleContext) -> NeonResult<()> {
//     // 通过调用 `cx.export_function` 方法，将 `get_mac_address` 函数导出为名为 "getMacAddress" 的 JavaScript 函数
//     cx.export_function("getMacAddress", get_mac_address)?;
//     // 成功导出函数后，返回一个 `Ok(())` 的结果
//     Ok(())
// }

extern crate mac_address;
use mac_address::get_mac_address;
fn main() {
    match get_mac_address() {
        Ok(Some(ma)) => {
            println!("MAC addr = {}", ma);
            println!("bytes = {:?}", ma.bytes());
        }
        Ok(None) => println!("No MAC address found."),
        Err(e) => println!("{:?}", e),
    }
}