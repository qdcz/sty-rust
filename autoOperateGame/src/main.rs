use winapi::um::{winuser, winbase}; // 导入 winapi 库中的相关模块


fn main() {
    println!("welcome, autoOperateGame!");

    unsafe {
        let x = 1000; // 设置鼠标点击位置坐标 x
        let y = 500; // 设置鼠标点击位置坐标 y
        let dx = 0;
        let dy = 0;
        let mut input: winuser::INPUT = std::mem::zeroed(); // 初始化一个 INPUT 结构体
        input.type_ = winuser::INPUT_MOUSE; // 设置为模拟鼠标事件
        input.u.mi().dx = (dx * (65536 / winuser::GetSystemMetrics(winuser::SM_CXSCREEN))) as i32;
        input.u.mi().dy = (dy * (65536 / winuser::GetSystemMetrics(winuser::SM_CYSCREEN))) as i32;
        input.u.mi().dwFlags = winuser::MOUSEEVENTF_MOVE | winuser::MOUSEEVENTF_ABSOLUTE; // 设置为鼠标移动事件
        winuser::SendInput(1, &mut input as *mut _, std::mem::size_of::<winuser::INPUT>() as i32); // 发送鼠标移动事件
        input.u.mi().dwFlags = winuser::MOUSEEVENTF_LEFTDOWN; // 设置为左键按下事件
        winuser::SendInput(1, &mut input as *mut _, std::mem::size_of::<winuser::INPUT>() as i32); // 发送左键按下事件
        input.u.mi().dwFlags = winuser::MOUSEEVENTF_LEFTUP; // 设置为左键释放事件
        winuser::SendInput(1, &mut input as *mut _, std::mem::size_of::<winuser::INPUT>() as i32); // 发送左键释放事件
    }
}
