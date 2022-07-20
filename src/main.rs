use std::{
    env,
    io::{Read, Write},
};

use hdrs::Client;

// export LD_LIBRARY_PATH=${JAVA_HOME}/jre/lib/server:${LD_LIBRARY_PATH}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("JAVA_HOME: {:?}", env::var("JAVA_HOME"));
    println!("HADOOP_HOME: {:?}", env::var("HADOOP_HOME"));
    println!("CLASSPATH: {:?}", env::var("CLASSPATH"));
    println!();

    let fs = Client::connect("hdfs://localhost:9000")?;

    let mut f = fs
        .open_file()
        .write(true)
        .create(true)
        .open("/tmp/hello.txt")?;
    let _n = f.write("Hello, World!".as_bytes())?;

    let mut f = fs.open_file().read(true).open("/tmp/hello.txt")?;
    let mut buf = vec![0; 1024];
    let _n = f.read(&mut buf)?;
    println!("buf: {:?}", String::from_utf8_lossy(&buf));

    let _ = fs.remove_file("/tmp/hello.txt")?;

    Ok(())
}
