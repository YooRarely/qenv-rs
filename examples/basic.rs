use qenv;

qenv::define! {
    PORT: "8080",      // 有默认值
    DATABASE_URL :"postgres://example.com/db",      // 必填（假设 .env 里有）
    IS_DEBUG: "false", // 布尔转换测试
}


fn main() {
    // 2. 初始化（会自动尝试读取 .env）
    // 我们用 .expect 因为如果初始化失败，通常是重复调用了
    qenv::init().expect("Failed to initialize qenv");
    println!("--- 🛠️  QENV 示例演示 ---");

    // 3. 使用 .get() 获取原始引用 (零克隆)
    println!("📡 Database: {}", DATABASE_URL);
	
    // 4. 使用 .take() 自动转换类型
    let port: u16 =  PORT.take();
    let is_debug: bool =IS_DEBUG.take();

    println!("🔌 Port: {}", port);
    println!("🐞 Debug mode: {}", is_debug);

    // 5. 安全处理测试 (try_get)
    qenv::define!(OPTIONAL_VAR);
    match OPTIONAL_VAR.try_get() {
        Ok(val) => println!("Found optional: {}", val),
        Err(_) => println!("💡 OPTIONAL_VAR is missing as expected"),
    }

    println!("--- 🎉 演示完毕，非常 Ok ---");
}
